pub use std::{
    collections::HashSet,
    fs::{create_dir_all, read_dir, File},
    io::{BufRead, BufReader, Error, Write},
    path::Path,
    time::{Duration, Instant},
};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use csv::StringRecord;
use nmea_parser::ais::{
    AisClass, CargoType, NavigationStatus, ShipType, Station, VesselDynamicData, VesselStaticData,
};
use nmea_parser::ParsedMessage;

use crate::db::{get_postgresdb_conn, postgres_prepare_tx_dynamic, postgres_prepare_tx_static};
use crate::decode::VesselData;

const BATCHSIZE: usize = 50000;

/// Convert time string to epoch seconds
pub fn csvdt_2_epoch(dt: &str) -> Result<i64, String> {
    let utctime = NaiveDateTime::parse_from_str(dt, "%Y%m%d_%H%M%S")
        .or_else(|_| NaiveDateTime::parse_from_str(dt, "%Y%m%dT%H%M%SZ"));

    match utctime {
        Ok(parsed_time) => Ok(Utc.from_utc_datetime(&parsed_time).timestamp()),
        Err(e) => Err(format!("Failed to parse timestamp '{}': {}", dt, e)),
    }
}

/// filter everything but vessel data, sort vessel data into static and dynamic vectors
pub fn filter_vesseldata_csv(rowopt: Option<StringRecord>) -> Option<(StringRecord, i32, bool)> {
    rowopt.as_ref()?;

    let row = rowopt.unwrap();
    let clonedrow = row.clone();
    let msgtype = clonedrow.get(1).unwrap();
    match msgtype {
        "1" | "2" | "3" | "18" | "19" | "27" => Some((
            row,
            csvdt_2_epoch(clonedrow.get(3).as_ref().unwrap()).unwrap_or_else(|e| {
                eprintln!("Failed to parse timestamp: {}", e);
                0
            }) as i32,
            true,
        )),
        "5" | "24" => Some((
            row,
            csvdt_2_epoch(clonedrow.get(3).as_ref().unwrap()).unwrap_or_else(|e| {
                eprintln!("Failed to parse timestamp: {}", e);
                0
            }) as i32,
            false,
        )),
        _ => None,
    }
}

/// convert ISO8601 time format (NOAA in use) to epoch seconds
pub fn iso8601_2_epoch(dt: &str) -> Option<i64> {
    match NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S") {
        Ok(utctime) => Some(Utc.from_utc_datetime(&utctime).timestamp()),
        Err(_) => None, // Return None instead of panicking
    }
}

/// Encodes the ETA into a 20-bit integer format
fn parse_eta(row: &csv::StringRecord) -> Option<DateTime<Utc>> {
    let eta_month: Option<u32> = row
        .get(45)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|x| x as u32);
    let eta_day: Option<u32> = row
        .get(46)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|x| x as u32);
    let eta_hour: Option<u32> = row
        .get(47)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|x| x as u32);
    let eta_minute: Option<u32> = row
        .get(48)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|x| x as u32);

    match (eta_month, eta_day, eta_hour, eta_minute) {
        (Some(month), Some(day), Some(hour), Some(minute))
            if (1..=12).contains(&month)
                && (1..=31).contains(&day)
                && (0..=23).contains(&hour)
                && (0..=59).contains(&minute) =>
        {
            // Use a fixed pseudo year - 2000, year in ETA will be discarded during insertion
            let pseudo_year = 2000;

            // Create a DateTime<Utc> with the pseudo year
            Utc.with_ymd_and_hms(pseudo_year, month, day, hour, minute, 0)
                .single()
        }
        _ => None,
    }
}

/// perform database input from Spire data
pub fn postgres_decodemsgs_ee_csv(
    connect_str: &str,
    filename: &std::path::PathBuf,
    source: &str,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(&filename.extension().expect("getting file ext"), &"csv");

    let start = Instant::now();

    let mut reader = csv::Reader::from_reader(File::open(filename)?);
    let mut stat_msgs = <Vec<VesselData>>::new();
    let mut positions = <Vec<VesselData>>::new();
    let mut count = 0;

    let mut c = get_postgresdb_conn(connect_str)?;

    for (row, epoch, is_dynamic) in reader
        .records()
        .filter_map(|r| filter_vesseldata_csv(r.ok()))
    {
        count += 1;
        if is_dynamic {
            let payload = VesselDynamicData {
                own_vessel: true,
                station: Station::BaseStation,
                ais_type: AisClass::Unknown,
                mmsi: row.get(0).unwrap().parse::<u32>().unwrap_or(0), // make tolerant with invalid MMSIs
                nav_status: NavigationStatus::NotDefined,
                rot: row.get(25).unwrap().parse::<f64>().ok(),
                rot_direction: None,
                sog_knots: row.get(26).unwrap().parse::<f64>().ok(),
                high_position_accuracy: false,
                latitude: row.get(29).unwrap().parse().ok(),
                longitude: row.get(28).unwrap().parse().ok(),
                cog: row.get(30).unwrap().parse().ok(),
                heading_true: row.get(31).unwrap().parse().ok(),
                timestamp_seconds: row.get(42).unwrap().parse::<u8>().unwrap_or(0),
                positioning_system_meta: None,
                current_gnss_position: None,
                special_manoeuvre: None,
                raim_flag: false,
                class_b_unit_flag: None,
                class_b_display: None,
                class_b_dsc: None,
                class_b_band_flag: None,
                class_b_msg22_flag: None,
                class_b_mode_flag: None,
                class_b_css_flag: None,
                radio_status: None,
            };
            let message = VesselData {
                epoch: Some(epoch),
                payload: Some(ParsedMessage::VesselDynamicData(payload)),
            };
            positions.push(message);
        } else {
            let payload = VesselStaticData {
                own_vessel: true,
                ais_type: AisClass::Unknown,
                mmsi: row.get(0).unwrap().parse().unwrap(),
                ais_version_indicator: row.get(23).unwrap().parse().unwrap_or_default(),
                imo_number: row.get(15).unwrap().parse().ok(),
                call_sign: row.get(14).unwrap().parse().ok(),
                name: Some(row.get(13).unwrap_or("").to_string()),
                ship_type: ShipType::new(row.get(16).unwrap().parse().unwrap_or_default()),
                cargo_type: CargoType::Undefined,
                equipment_vendor_id: None,
                equipment_model: None,
                equipment_serial_number: None,
                dimension_to_bow: row.get(17).unwrap_or_default().parse().ok(),
                dimension_to_stern: row.get(18).unwrap_or_default().parse().ok(),
                dimension_to_port: row.get(19).unwrap_or_default().parse().ok(),
                dimension_to_starboard: row.get(20).unwrap_or_default().parse().ok(),
                position_fix_type: None,
                // eta: None,  // at cols 45-48 ETA month, day, hour, minute, format: float64
                eta: parse_eta(&row),
                draught10: row.get(21).unwrap_or_default().parse().ok(),
                destination: row.get(22).unwrap_or_default().parse().ok(),
                mothership_mmsi: row.get(131).unwrap_or_default().parse().ok(),
            };
            let message = VesselData {
                epoch: Some(epoch),
                payload: Some(ParsedMessage::VesselStaticData(payload)),
            };
            stat_msgs.push(message);
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

    if !positions.is_empty() {
        postgres_prepare_tx_dynamic(&mut c, source, positions)?;
    }

    if !stat_msgs.is_empty() {
        postgres_prepare_tx_static(&mut c, source, stat_msgs)?;
    }

    let elapsed = start.elapsed();
    let f3 = filename.to_str().unwrap();
    let f4 = Path::new(f3);
    let fname = f4.file_name().unwrap().to_str().unwrap();
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

    Ok(())
}

/// progress database input from NOAA
pub fn postgres_decodemsgs_noaa_csv(
    connect_str: &str,
    filename: &std::path::PathBuf,
    source: &str,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(&filename.extension().expect("getting file ext"), &"csv");

    let start = Instant::now();

    let mut reader = csv::Reader::from_reader(File::open(filename)?);
    let mut stat_msgs = <Vec<VesselData>>::new();
    let mut positions = <Vec<VesselData>>::new();
    let mut count = 0;
    let mut static_seen: HashSet<u32> = HashSet::new();

    let mut c = get_postgresdb_conn(connect_str)?;

    for row_option in reader.records() {
        count += 1;
        let row = match row_option {
            Ok(row) => row,
            Err(err) => {
                eprintln!("Skipping row due to CSV parsing error: {}", err);
                continue; // Skip this row and proceed with the next one
            }
        };
        let row_clone = row.clone();
        let epoch = match iso8601_2_epoch(row_clone.get(1).as_ref().unwrap()) {
            Some(epoch) => epoch as i32,
            None => {
                eprintln!(
                    "Skipping row due to invalid timestamp: {:?}",
                    row_clone.get(1)
                );
                return Ok(());
            }
        };
        let mmsi: u32 = match row.get(0).and_then(|m| m.parse::<u32>().ok()) {
            Some(mmsi) => mmsi,
            None => {
                eprintln!("Skipping row due to invalid MMSI: {:?}", row.get(0));
                continue; // Skip this row and move to the next one
            }
        };

        let payload_dynamic = VesselDynamicData {
            own_vessel: true,
            station: Station::BaseStation,
            //             ais_type: AisClass::new(row.get(16).unwrap().parse().unwrap_or_default()),
            ais_type: match row.get(16) {
                Some("A") => AisClass::ClassA,
                Some("B") => AisClass::ClassB,
                _ => AisClass::Unknown,
            },
            mmsi: mmsi,
            nav_status: NavigationStatus::new(row.get(11).unwrap().parse().unwrap_or_default()),
            rot: None,
            rot_direction: None,
            sog_knots: row.get(4).unwrap().parse::<f64>().ok(),
            high_position_accuracy: false,
            latitude: row.get(2).unwrap().parse().ok(),
            longitude: row.get(3).unwrap().parse().ok(),
            cog: row.get(5).unwrap().parse().ok(),
            heading_true: row.get(6).unwrap().parse().ok(),
            timestamp_seconds: 0, // enforced field
            positioning_system_meta: None,
            current_gnss_position: None,
            special_manoeuvre: None,
            raim_flag: false,
            class_b_unit_flag: None,
            class_b_display: None,
            class_b_dsc: None,
            class_b_band_flag: None,
            class_b_msg22_flag: None,
            class_b_mode_flag: None,
            class_b_css_flag: None,
            radio_status: None,
        };
        let message_dyn = VesselData {
            epoch: Some(epoch),
            payload: Some(ParsedMessage::VesselDynamicData(payload_dynamic)),
        };
        positions.push(message_dyn);

        if static_seen.insert(mmsi) {
            let payload_static = VesselStaticData {
                own_vessel: true,
                ais_type: match row.get(16) {
                    Some("A") => AisClass::ClassA,
                    Some("B") => AisClass::ClassB,
                    _ => AisClass::Unknown,
                },
                mmsi: mmsi,
                ais_version_indicator: 0,
                imo_number: row.get(8).unwrap().parse().ok(),
                call_sign: row.get(9).unwrap().parse().ok(),
                name: Some(row.get(7).unwrap_or("").to_string()),
                ship_type: ShipType::new(row.get(10).unwrap().parse().unwrap_or_default()),
                cargo_type: CargoType::new(row.get(15).unwrap().parse().unwrap_or_default()),
                equipment_vendor_id: None,
                equipment_model: None,
                equipment_serial_number: None,
                dimension_to_bow: None,
                dimension_to_stern: None,
                dimension_to_port: None,
                dimension_to_starboard: None,
                position_fix_type: None,
                eta: None,
                draught10: row.get(14).unwrap_or_default().parse().ok(),
                destination: None,
                mothership_mmsi: None,
            };
            let message_stat = VesselData {
                epoch: Some(epoch),
                payload: Some(ParsedMessage::VesselStaticData(payload_static)),
            };
            stat_msgs.push(message_stat);
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

    if !positions.is_empty() {
        postgres_prepare_tx_dynamic(&mut c, source, positions)?;
    }

    if !stat_msgs.is_empty() {
        postgres_prepare_tx_static(&mut c, source, stat_msgs)?;
    }

    let elapsed = start.elapsed();
    let f3 = filename.to_str().unwrap();
    let f4 = Path::new(f3);
    let fname = f4.file_name().unwrap().to_str().unwrap();
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

    Ok(())
}
