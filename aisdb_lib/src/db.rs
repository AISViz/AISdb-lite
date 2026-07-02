use crate::decode::VesselData;

use chrono::{DateTime, Utc};
use include_dir::{include_dir, Dir};

pub use postgres::{Client as PGClient, NoTls, Transaction as PGTransaction};

static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../aisdb/aisdb_sql");

/// embed SQL strings as literals
pub fn sql_from_file(fname: &str) -> &str {
    PROJECT_DIR
        .get_file(fname)
        .unwrap()
        .contents_utf8()
        .unwrap()
}

pub fn get_postgresdb_conn(connect_str: &str) -> Result<PGClient, Box<dyn std::error::Error>> {
    // TLS is handled by gateway router
    let client = PGClient::connect(connect_str, NoTls)?;
    #[cfg(debug_assertions)]
    println!("Connected to postgres server");
    Ok(client)
}

/// insert static reports into database
pub fn postgres_insert_static(
    tx: &mut PGTransaction,
    msgs: Vec<VesselData>,
    source: &str,
) -> Result<(), postgres::Error> {
    let sql = sql_from_file("new_insert_static.sql");

    let stmt = tx.prepare(sql)?;
    for msg in msgs {
        let (p, e) = msg.staticdata();

        let eta = p.eta.unwrap_or(DateTime::<Utc>::MIN_UTC);
        tx.execute(
            &stmt,
            &[
                &(p.mmsi as i64),
                &(e as i64),
                &p.name.unwrap_or_default(),
                &(p.ship_type as i32),
                &p.call_sign.unwrap_or_default(),
                &(p.imo_number.unwrap_or_default() as i64),
                &(p.dimension_to_bow.unwrap_or_default() as i32),
                &(p.dimension_to_stern.unwrap_or_default() as i32),
                &(p.dimension_to_port.unwrap_or_default() as i32),
                &(p.dimension_to_starboard.unwrap_or_default() as i32),
                &(p.draught10.unwrap_or_default() as i32),
                &p.destination.unwrap_or_default(),
                &(p.ais_version_indicator as i32),
                &p.equipment_vendor_id.unwrap_or_default(),
                &eta.format("%m")
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_default(),
                &eta.format("%d")
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_default(),
                &eta.format("%H")
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_default(),
                &eta.format("%M")
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_default(),
                &source,
            ],
        )?;
    }
    Ok(())
}

/// Detect which column set the ais_global_dynamic table exposes.
///
/// Queried per transaction (one cheap information_schema lookup per file
/// batch); results are deliberately NOT cached process-wide, since one
/// process may hold connections to databases with different schemas.
fn detect_table_schema(tx: &mut PGTransaction) -> Result<String, postgres::Error> {
    let query = "
        SELECT
            EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'ais_global_dynamic' AND column_name = 'rot') as has_rot,
            EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'ais_global_dynamic' AND column_name = 'maneuver') as has_maneuver,
            EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'ais_global_dynamic' AND column_name = 'utc_second') as has_utc,
            EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'ais_global_dynamic' AND column_name = 'source') as has_source
    ";

    let row = tx.query_one(query, &[])?;
    let has_rot: bool = row.get(0);
    let has_maneuver: bool = row.get(1);
    let has_utc: bool = row.get(2);
    let has_source: bool = row.get(3);

    let schema_version = if has_rot && has_maneuver && has_utc && has_source {
        "full" // complete schema with every column
    } else if !has_rot && has_maneuver && has_utc && has_source {
        "norot" // schema without the rot column
    } else {
        "minimal" // basic 7-column schema
    };

    Ok(schema_version.to_string())
}

/// insert position reports into database
pub fn postgres_insert_dynamic(
    tx: &mut PGTransaction,
    msgs: Vec<VesselData>,
    source: &str,
) -> Result<(), postgres::Error> {
    // Detect the table schema, then pick the matching INSERT statement
    let schema_version = detect_table_schema(tx)?;

    let sql = match schema_version.as_str() {
        "full" => sql_from_file("new_insert_dynamic_clusteredidx.sql"),
        "norot" => sql_from_file("new_insert_dynamic_norot.sql"),
        _ => sql_from_file("new_insert_dynamic_minimal.sql"),
    };

    let stmt = tx.prepare(sql)?;

    for msg in msgs {
        let (p, e) = msg.dynamicdata();

        match schema_version.as_str() {
            "full" => {
                // full INSERT with every column
                // mmsi/time are BIGINT (i64); coordinate columns are REAL (f32)
                let _ = tx.execute(
                    &stmt,
                    &[
                        &(p.mmsi as i64),
                        &(e as i64),
                        &(p.longitude.unwrap_or_default() as f32),
                        &(p.latitude.unwrap_or_default() as f32),
                        &(p.rot.unwrap_or_default() as f32),
                        &(p.sog_knots.unwrap_or_default() as f32),
                        &(p.cog.unwrap_or_default() as f32),
                        &(p.heading_true.unwrap_or_default() as f32),
                        &p.special_manoeuvre.unwrap_or_default(),
                        &(p.timestamp_seconds as i32),
                        &source,
                    ],
                )?;
            }
            "norot" => {
                // INSERT without the rot column
                let _ = tx.execute(
                    &stmt,
                    &[
                        &(p.mmsi as i64),
                        &(e as i64),
                        &(p.longitude.unwrap_or_default() as f32),
                        &(p.latitude.unwrap_or_default() as f32),
                        &(p.sog_knots.unwrap_or_default() as f32),
                        &(p.cog.unwrap_or_default() as f32),
                        &(p.heading_true.unwrap_or_default() as f32),
                        &p.special_manoeuvre.unwrap_or_default(),
                        &(p.timestamp_seconds as i32),
                        &source,
                    ],
                )?;
            }
            _ => {
                // minimal INSERT (basic columns only)
                let _ = tx.execute(
                    &stmt,
                    &[
                        &(p.mmsi as i64),
                        &(e as i64),
                        &(p.longitude.unwrap_or_default() as f32),
                        &(p.latitude.unwrap_or_default() as f32),
                        &(p.sog_knots.unwrap_or_default() as f32),
                        &(p.cog.unwrap_or_default() as f32),
                        &(p.heading_true.unwrap_or_default() as f32),
                    ],
                )?;
            }
        }
    }

    Ok(())
}

/// prepare a new transaction and insert dynamic messages into the global table
pub fn postgres_prepare_tx_dynamic(
    c: &mut PGClient,
    source: &str,
    positions: Vec<VesselData>,
) -> Result<(), postgres::Error> {
    let mut t = c.transaction()?;
    postgres_insert_dynamic(&mut t, positions, source)?;
    t.commit()
}

/// prepare a new transaction and insert static messages into the global table
pub fn postgres_prepare_tx_static(
    c: &mut PGClient,
    source: &str,
    stat_msgs: Vec<VesselData>,
) -> Result<(), postgres::Error> {
    let mut t = c.transaction()?;
    postgres_insert_static(&mut t, stat_msgs, source)?;
    t.commit()
}
