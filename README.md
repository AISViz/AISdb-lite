# AISdb-lite

A lightweight version of [AISdb](https://github.com/MAPS-Lab/AISdb) featuring advanced spatio-temporal capabilities with PostGIS and TimescaleDB (TigerData). Where the full AISdb shards AIS messages into per-month tables, AISdb-lite ingests everything into two global tables backed by TimescaleDB hypertables with PostGIS geometry.

## What it does

- `ais_global_dynamic` holds position reports; a hypertable partitioned on `time` (Unix epoch seconds) and space-partitioned on `mmsi`, with a generated `geom GEOMETRY(POINT, 4326)` column, a BRIN index on `time`, and a GiST index on `geom`
- `ais_global_static` holds vessel metadata; a hypertable with the same partitioning

Decoding and insertion run in Rust (`aisdb_lib`); the Python package (`aisdb`) provides the query, track-generation, and analysis layers.

## Installation

The build requires the following.

- Python 3.10-3.12
- Rust toolchain via rustup
- PostgreSQL with the TimescaleDB and PostGIS extensions

```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop            # or: maturin build --release
```

The core install covers decode, ingest, and query. Optional feature sets are
packaged as extras and imported lazily, so their dependencies are only needed
when the feature is used.

| Extra | Enables | Pulls in |
|-------|---------|----------|
| `scraping` | `DBQuery.check_marinetraffic`, vessel metadata scraping | selenium, webdriver-manager, beautifulsoup4 |
| `weather` | `aisdb.WeatherDataStore` (ERA5 at track positions) | xarray, cfgrib, cdsapi |
| `discretize` | `aisdb.Discretizer` (H3 hex indexing) | h3, matplotlib, geopandas |
| `rasters` | `aisdb.Gebco`, `ShoreDist`, `PortDist`, `InlandDenoising` | pillow, py7zr, requests, tqdm |
| `ports` | `aisdb.WorldPortIndexClient` | pandas, requests |
| `all` | everything above | |

```bash
pip install 'aisdb[weather,discretize]'   # pick what you need
```

## Quick start

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

ExactEarth CSV exports can bypass the NMEA path entirely.

```python
from aisdb.database.decoder_csv import decode_csv_files

decode_csv_files(files, dbconn, source="EXACTEARTH")
```

## Development

Build with `maturin develop` as shown above. The PostgreSQL test suite reads the connection from environment variables and connects to a database named after the user.

```bash
export pguser=aisdb_test pgpass=... pghost=localhost
pytest aisdb/tests/test_001_postgres.py
```

The tests ingest sample data and assert rows read back and that the BRIN/GiST indexes exist.

Benchmark and migration write-ups live in `docs/`.

- `docs/BRIN_INDEX_MIGRATION.md` explains why BRIN on `time` replaces the btree for range scans
- `docs/PARALLEL_WORKERS_OPTIMIZATION.md` covers the parallel-worker tuning methodology
- `scripts/benchmarks/parallel_workers_2weeks.sh` provides a reproducible worker-scaling benchmark

Continuous integration builds wheels for Linux, macOS, and Windows and runs the full test suite, including the PostgreSQL and TimescaleDB paths, on every push and pull request.

## Documentation

[docs](https://aisviz.gitbook.io/documentation/) · [tutorials](https://aisviz.gitbook.io/tutorials/) · [API reference](https://aisviz.cs.dal.ca/AISdb/) · [website](https://aisviz.cs.dal.ca/)

## Related projects

- [AISdb](https://github.com/MAPS-Lab/AISdb) is the canonical general-purpose package on [PyPI](https://pypi.org/project/aisdb/); this repository tracks the `vishvesh/dev` lineage of AISViz/AISdb
- [NOAA-Integrator](https://github.com/MAPS-Lab/AISdb-NOAA-Integrator) acquires AIS data from NOAA Marine Cadastre and loads it into AISdb-aligned databases
- [Tutorials](https://github.com/MAPS-Lab/AISdb-Tutorials) collects notebooks with worked examples for AISdb workflows

## License

This project is distributed under the terms of the GNU Affero General Public License v3.0 (AGPL-3.0). See [LICENSE](LICENSE) for details.
