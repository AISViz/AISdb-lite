pub use std::{
    ffi::OsStr,
    fs::{create_dir_all, metadata, read_dir, File},
    io::{BufRead, BufReader, Error, Write},
    path::Path,
    time::{Duration, Instant},
};

use nmea_parser::{
    ais::{VesselDynamicData, VesselStaticData},
    NmeaParser, ParsedMessage,
};

use crate::db::{get_postgresdb_conn, postgres_prepare_tx_dynamic, postgres_prepare_tx_static};

const BATCHSIZE: usize = 50000;

/// collect decoded messages and epoch timestamps
#[derive(Clone)]
pub struct VesselData {
    pub payload: Option<ParsedMessage>,
    pub epoch: Option<i32>,
}

/// explicit type conversion in decode_msgs fn
impl VesselData {
    pub fn dynamicdata(self) -> (VesselDynamicData, i32) {
        let p = self.payload.unwrap();
        if let ParsedMessage::VesselDynamicData(p) = p {
            (p, self.epoch.unwrap())
        } else {
            panic!("wrong msg type")
        }
    }
    pub fn staticdata(self) -> (VesselStaticData, i32) {
        let p = self.payload.unwrap();
        if let ParsedMessage::VesselStaticData(p) = p {
            (p, self.epoch.unwrap())
        } else {
            panic!("wrong msg type")
        }
    }
}

/// collect base station timestamp and NMEA payload
/// as derived from NMEA string with metadata header
///
/// input example:
/// ``` text
/// \s:43479,c:1635883083,t:1635883172*6C\!AIVDM,1,1,,,144fiV0P00WT:`8POChN4?v4281b,0*64
/// ```
///
/// returns:
/// ``` text
/// ("!AIVDM,1,1,,,144fiV0P00WT:`8POChN4?v4281b,0*64", 1635883083)
/// ```
pub fn parse_headers(line: Result<String, Error>) -> Option<(String, i32)> {
    // Extract meta and payload from the line
    let (meta, payload) = line.as_ref().ok()?.rsplit_once('\\')?;

    let mut final_timestamp: Option<u64> = None;

    // Loop through the tags in the `meta` part
    for tag_outer in meta.split(',') {
        for tag in tag_outer.split('*') {
            // If the tag length is too short or doesn't contain "c:", skip it
            if tag.len() <= 3 || !tag.contains("c:") {
                continue;
            }

            // Try parsing the timestamp `i` from different parts of the tag
            let mut i: Option<u64> = None;

            if let Ok(parsed_i) = tag[2..].parse::<u64>() {
                i = Some(parsed_i);
            } else if let Ok(parsed_i) = tag[3..].parse::<u64>() {
                i = Some(parsed_i);
            } else if let Ok(parsed_i) = tag.split_once(' ').unwrap_or(("", "")).0.parse::<u64>() {
                i = Some(parsed_i);
            }

            // If we successfully parsed a timestamp, process it
            if let Some(timestamp) = i {
                let epoch_length = timestamp.to_string().len();
                let adjusted_timestamp = if epoch_length == 13 {
                    // Convert milliseconds to seconds
                    timestamp / 1000
                } else if epoch_length > 13 {
                    // Adjust to seconds by scaling down
                    let scale = 10u64.pow((epoch_length - 10) as u32);
                    timestamp / scale
                } else if epoch_length == 10 {
                    // Already in seconds
                    timestamp
                } else {
                    // Invalid epoch length
                    continue;
                };

                // Store the adjusted timestamp if valid
                if is_valid_epoch(adjusted_timestamp) {
                    final_timestamp = Some(adjusted_timestamp);
                }
            }
        }
    }

    // If we have a valid final timestamp, return it along with the payload
    if let Some(final_ts) = final_timestamp {
        return Some((payload.to_string(), final_ts.try_into().unwrap()));
    }

    None // If no valid timestamp was found, return None
}

// Helper function to validate that the timestamp is within the valid epoch range
fn is_valid_epoch(epoch: u64) -> bool {
    let min_valid_epoch: u64 = 946731600; // Around year 2000
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    epoch >= min_valid_epoch && epoch <= current_time
}

fn extract_epoch_from_nmea_line(line: &str) -> i32 {
    // Attempt to extract Unix timestamp from additional fields after the checksum
    if let Some(checksum_index) = line.find('*') {
        let after_checksum = &line[checksum_index + 1..];
        // Split by comma in case there are additional fields
        for field in after_checksum.split(',') {
            if let Ok(epoch) = field.trim().parse::<i32>() {
                if epoch > 1_000_000_000 {
                    return epoch;
                }
            }
        }
    }
    // If no timestamp found, use a default value or handle accordingly
    0 // Return 0 or current time, but be cautious with defaults
}

fn parse_headers_nmea(line: Result<String, Error>) -> Option<(String, i32)> {
    match line {
        Ok(line) => {
            let line = line.trim();
            if line.starts_with('!') {
                // Attempt to extract the epoch time from the line
                let epoch = extract_epoch_from_nmea_line(&line);
                Some((line.to_string(), epoch))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// workaround for panic from nmea_parser library,
/// caused by malformed base station timestamps / binary application messages?
/// discards UTC date response and binary application payloads before
/// decoding them
pub fn skipmsg(msg: &str, epoch: &i32) -> Option<(String, i32)> {
    let cols: Vec<&str> = msg.split(',').collect();
    if cols.len() < 6 {
        return Some((msg.to_string(), *epoch));
    }
    //         #[cfg(debug_assertions)]
    //         println!(
    //             "{:?} {:?} {:?} {:?} {:?} {:?}",
    //             cols[0], cols[1], cols[2], cols[3], cols[4], cols[5]
    //         );
    let count = str::parse::<u8>(cols[1]).unwrap_or(1);
    let fragment_no = str::parse::<u8>(cols[2]).unwrap_or(1);
    match (cols[0], count, fragment_no, cols[3], cols[4], cols[5]) {
        (_prefix, c, f, _seq_id, _channel, tx)
            if (tx.chars().count() <= 2
                || ((c == 1)
                    && (f == 1)
                    && (&tx[0..1] == ";" || &tx[0..1] == "I" || &tx[0..1] == "J"))) =>
        {
            //             println!("skipped {:?}", msg);
            None
        }
        _ => Some((msg.to_string(), *epoch)),
    }
}

/// discard all other message types, sort filtered categories
pub fn filter_vesseldata(
    sentence: &str,
    epoch: &i32,
    parser: &mut NmeaParser,
) -> Option<(ParsedMessage, i32, bool)> {
    //     #[cfg(debug_assertions)]
    //     println!("{:?} {:?}", epoch, sentence);

    match parser.parse_sentence(sentence).ok()? {
        ParsedMessage::VesselDynamicData(vdd) => {
            Some((ParsedMessage::VesselDynamicData(vdd), *epoch, true))
        }
        ParsedMessage::VesselStaticData(vsd) => {
            Some((ParsedMessage::VesselStaticData(vsd), *epoch, false))
        }
        _ => None,
    }
}

fn validate_file_ext(filename: std::path::PathBuf) -> Result<(), String> {
    match filename.extension() {
        Some(ext_os_str) => match ext_os_str.to_str() {
            Some("nm4") | Some("NM4") | Some("nmea") | Some("NMEA") | Some("rx") | Some("txt")
            | Some("RX") | Some("TXT") => Ok(()),
            _ => Err(format!("unknown file type! {:?}", &filename)),
        },
        _ => Err(format!("unknown file type! {:?}", &filename)),
    }
}

fn decode_filter_pipe(
    reader: BufReader<File>,
    mut parser: &mut NmeaParser,
    file_extension: &str,
) -> Vec<(ParsedMessage, i32, bool)> {
    match file_extension {
        "nm4" => {
            // Processing for .nm4 files
            reader
                .lines()
                .filter_map(parse_headers)
                //                 .inspect(|parsed| println!("parse_headers output: {:?}", parsed))
                .filter_map(|(s, e)| skipmsg(&s, &e))
                .filter_map(|(s, e)| filter_vesseldata(&s, &e, &mut parser))
                .collect::<Vec<(ParsedMessage, i32, bool)>>()
        }
        "nmea" | "txt" | "rx" => reader
            .lines()
            .filter_map(parse_headers_nmea)
            .filter_map(|(s, e)| skipmsg(&s, &e))
            .filter_map(|(s, e)| filter_vesseldata(&s, &e, &mut parser))
            .collect::<Vec<(ParsedMessage, i32, bool)>>(),
        _ => {
            // In case of unsupported file types
            Vec::new()
        }
    }
}

fn print_status_info(
    filename: std::path::PathBuf,
    elapsed: std::time::Duration,
    count: usize,
    verbose: bool,
) {
    let f3 = filename.to_str().unwrap();
    let f4 = Path::new(f3);
    let fname = f4.file_name().unwrap().to_str().unwrap();
    //     let fname = filename
    //         .to_str()
    //         .unwrap()
    //         .rsplit_once(std::path::MAIN_SEPARATOR)
    //         .unwrap()
    //         .1;
    let fname1 = format!("{:<1$}", fname, 64);
    let elapsed1 = format!(
        "elapsed: {:>1$}s",
        format!("{:.2 }", elapsed.as_secs_f32()),
        7
    );
    let rate1 = format!(
        "rate: {:>1$} msgs/s",
        format!("{:.0}", count as f32 / elapsed.as_secs_f32()),
        8
    );

    if verbose {
        println!(
            "{} count:{: >8}    {}    {}",
            fname1, count, elapsed1, rate1,
        );
    }
}

/// open .nm4 file and decode each line, keeping only vessel data.
/// decoded vessel data will be inserted into the global tables of the
/// PostgreSQL database at connect_str
pub fn postgres_decode_insert_msgs(
    connect_str: &str,
    filename: std::path::PathBuf,
    source: &str,
    mut parser: NmeaParser,
    verbose: bool,
) -> Result<NmeaParser, Box<dyn std::error::Error>> {
    validate_file_ext(filename.clone())?;
    let mut c = get_postgresdb_conn(connect_str).expect("getting db conn");

    let start = Instant::now();
    let reader = BufReader::new(File::open(&filename)?);

    let mut stat_msgs = <Vec<VesselData>>::new();
    let mut positions = <Vec<VesselData>>::new();
    let mut count = 0;

    let file_ext = filename
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    // in 500k batches
    for (payload, epoch, is_dynamic) in decode_filter_pipe(reader, &mut parser, &file_ext) {
        let message = VesselData {
            epoch: Some(epoch),
            payload: Some(payload),
        };

        match (is_dynamic, &message.payload) {
            (_, None) => continue,
            (true, Some(_m)) => {
                positions.push(message);
                count += 1;
            }
            (false, Some(_m)) => {
                stat_msgs.push(message);
                count += 1;
            }
        }

        if positions.len() >= BATCHSIZE {
            postgres_prepare_tx_dynamic(&mut c, source, positions)?;
            positions = vec![];
        };
        if stat_msgs.len() >= BATCHSIZE {
            postgres_prepare_tx_static(&mut c, source, stat_msgs)?;
            stat_msgs = vec![];
        }
    }

    // insert remaining
    if !positions.is_empty() {
        postgres_prepare_tx_dynamic(&mut c, source, positions)?;
    }
    if !stat_msgs.is_empty() {
        postgres_prepare_tx_static(&mut c, source, stat_msgs)?;
    }

    let elapsed = start.elapsed();
    print_status_info(filename, elapsed, count, verbose);

    Ok(parser)
}

/* --------------------------------------------------------------------------------------------- */

#[cfg(test)]
pub mod tests {

    use super::parse_headers;

    #[test]
    pub fn test_parse_headers() {
        let input = r#"\s:43479,c:1635883083,t:1635883172*6C\!AIVDM,1,1,,,144fiV0P00WT:`8POChN4?v4281b,0*64"#;
        let result = parse_headers(Ok(input.to_string())).unwrap();
        let expected = (
            "!AIVDM,1,1,,,144fiV0P00WT:`8POChN4?v4281b,0*64".to_string(),
            1635883083,
        );
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_parse_headers_milliseconds_timestamp() {
        // Example of a valid NM4 line with a 13-digit epoch in milliseconds
        let input = r#"\c:1726841314345,s:my-station,T:2024-09-20 14.08.34*23\!AIVDM,1,1,,A,15NTES0P00J>tC4@@FOhMgvD0D0M,0*49"#;
        let result = parse_headers(Ok(input.to_string())).unwrap();
        let expected = (
            "!AIVDM,1,1,,A,15NTES0P00J>tC4@@FOhMgvD0D0M,0*49".to_string(),
            1726841314,
        );
        assert_eq!(expected, result);
    }
}
