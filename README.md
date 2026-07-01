# AISdb-lite

A lightweight version of [AISdb](https://github.com/AISViz/AISdb) featuring advanced spatio-temporal capabilities with PostGIS and TimescaleDB (TigerData).

Where the full AISdb shards AIS messages into per-month tables, AISdb-lite ingests everything into two global tables backed by TimescaleDB hypertables with PostGIS geometry:

- `ais_global_dynamic` - position reports; hypertable partitioned on `time` (Unix epoch seconds) and space-partitioned on `mmsi`, with a generated `geom GEOMETRY(POINT, 4326)` column, a BRIN index on `time`, and a GiST index on `geom`
- `ais_global_static` - vessel metadata; hypertable with the same partitioning

Decoding and insertion run in Rust (`aisdb_lib`); the Python package (`aisdb`) provides the query, track-generation, and analysis layers.

## Requirements

- Python 3.10-3.12
- Rust toolchain via rustup, with the `wasm32-unknown-unknown` target and `wasm-pack` (the wheel build embeds the web assets)
- PostgreSQL with the TimescaleDB and PostGIS extensions

## Build and install

```bash
rustup target add wasm32-unknown-unknown
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop            # or: maturin build --release
```

## Usage

```python
import os
import aisdb

with aisdb.PostgresDBConn(libpq_connstring=os.environ["AISDB_PG_DSN"]) as dbconn:
    aisdb.decode_msgs(
        filepaths=["/data/ais/20240101.nm4"],
        dbconn=dbconn,
        source="MERIDIAN",
        timescaledb=True,
    )
```

`decode_msgs` creates the global tables from the canonical SQL in `aisdb/aisdb_sql/`, converts pre-existing plain tables into hypertables (`migrate_data => true`), and raises if every decode batch fails rather than reporting success over an empty insert.

ExactEarth CSV exports can bypass the NMEA path entirely:

```python
from aisdb.database.decoder_csv import decode_csv_files

decode_csv_files(files, dbconn, source="EXACTEARTH")
```

## Testing

The PostgreSQL test suite reads the connection from environment variables and connects to a database named after the user:

```bash
export pguser=aisdb_test pgpass=... pghost=localhost
pytest aisdb/tests/test_001_postgres.py
```

The tests ingest sample data and assert rows read back and that the BRIN/GiST indexes exist.

## Performance notes

Benchmark and migration write-ups live in `docs/`:

- `docs/BRIN_INDEX_MIGRATION.md` - why BRIN on `time` replaces the btree for range scans
- `docs/PARALLEL_WORKERS_OPTIMIZATION.md` - parallel-worker tuning methodology
- `scripts/benchmarks/parallel_workers_2weeks.sh` - reproducible worker-scaling benchmark

## Relationship to AISdb

This repository tracks the `vishvesh/dev` lineage of AISViz/AISdb. The full AISdb remains the canonical general-purpose package on [PyPI](https://pypi.org/project/aisdb/); AISdb-lite is the PostGIS/TimescaleDB-first variant. Documentation and tutorials: [aisviz.gitbook.io](https://aisviz.gitbook.io/documentation/).
