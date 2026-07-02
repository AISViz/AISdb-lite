"""
Simplified decoder - CSV ExactEarth only
Uses existing csvreader.rs directly, eliminating Python layers
"""

from pathlib import Path
from typing import List, Union

from aisdb.aisdb import decoder
from aisdb.database.dbconn import PostgresDBConn


def decode_csv_files(
    filepaths: List[Union[str, Path]],
    dbconn: PostgresDBConn,
    source: str,
    verbose: bool = True,
    workers: int = 4,
) -> List[str]:
    """
    Decode ExactEarth CSV files directly in Rust.

    This is a simplified interface that uses the existing csvreader.rs.
    All heavy processing happens in Rust:
    - CSV reading
    - Column parsing
    - PostgreSQL insertion

    Args:
        filepaths: List of CSV files to process
        dbconn: PostgreSQL connection
        source: Data source identifier
        verbose: Show progress
        workers: Number of parallel workers

    Returns:
        List of successfully processed files

    Example:
        >>> conn = PostgresDBConn(libpq_connstring=os.environ["AISDB_PG_DSN"])
        >>> files = ["/data/exactEarth_2020-10-01.csv"]
        >>> processed = decode_csv_files(files, conn, source="EXACTEARTH")
        >>> print(f"Processed {len(processed)} files")
    """
    # The Rust decoder expects plain string paths
    raw_files = [str(f) for f in filepaths]

    # Call existing Rust decoder
    # decoder() already identifies CSV and calls postgres_decodemsgs_ee_csv
    processed = decoder(
        psql_conn_string=dbconn.connection_string,
        files=raw_files,
        source=source,
        verbose=verbose,
        workers=workers,
        allow_swap=False,
        type_preference="csv",  # Force CSV processing
    )

    if raw_files and not processed:
        raise RuntimeError(
            f"decoder failed for all {len(raw_files)} CSV files; "
            "see stderr for per-file errors"
        )

    return processed


# Alias for compatibility
decode_exactearth_csv = decode_csv_files


if __name__ == "__main__":
    print("""
Simplified CSV Decoder
======================

USAGE:
------
from aisdb.database.decoder_csv import decode_csv_files
from aisdb.database.dbconn import PostgresDBConn

# Connect (credentials via environment, never hardcoded)
conn = PostgresDBConn(libpq_connstring=os.environ["AISDB_PG_DSN"])

# Process CSV
files = ["/data/exactEarth_historical_data_2020-10-01.csv"]
processed = decode_csv_files(files, conn, source="EXACTEARTH")

PROCESSING (100% in Rust):
--------------------------
1. Read CSV line by line
2. Parse columns: MMSI, Time, Lat, Lon, SOG, COG, Heading, etc.
3. Convert timestamp to Unix epoch
4. Filter message types (1,2,3,5,18,19,24,27)
5. Separate dynamic (position) and static (vessel info)
6. Batch insert (50k records) into PostgreSQL
7. Return list of processed files

ADVANTAGES:
-----------
- All processing in Rust (fast)
- Reuses the existing, tested csvreader.rs
- Parallel workers supported
- Low memory footprint
- Simple Python API
    """)
