# AISdb-lite

AISdb-lite is a lightweight, PostGIS and TimescaleDB-first variant of [AISdb](https://github.com/MAPS-Lab/AISdb) for storing, ingesting, and querying Automatic Identification System (AIS) data at global scale. It pairs a Rust decoding and insertion core (`aisdb_lib`) with a Python query, track-generation, and analysis layer (`aisdb`), and is developed and maintained by the [MAPS Lab](https://mapslab.tech/) at Dalhousie University, continuing work that began under the [MERIDIAN](https://meridian.cs.dal.ca) initiative.

Where the full AISdb shards AIS messages into per-month tables, AISdb-lite ingests everything into two global tables backed by TimescaleDB hypertables with PostGIS geometry, trading fine-grained partitioning for a single, uniformly indexed spatio-temporal store.

## Features

- Global two-table schema, with `ais_global_dynamic` for position reports and `ais_global_static` for vessel metadata, each a TimescaleDB hypertable partitioned on `time` (Unix epoch seconds) and space-partitioned on `mmsi`
- PostGIS geometry throughout, including a generated `geom GEOMETRY(POINT, 4326)` column, a BRIN index on `time` for range scans, and a GiST index on `geom` for spatial queries
- A Rust core (`aisdb_lib`) for NMEA decoding and insertion, exposed through a Python API (`aisdb`) that covers querying, track generation, and analysis for a range of skill levels
- Two ingestion paths, the NMEA/`nm4` stream and direct ExactEarth CSV exports, both feeding the same global hypertables
- Optional, lazily imported feature sets for weather sampling, H3 discretization, raster distances, port lookups, and vessel-metadata scraping, so heavy dependencies are pulled in only when a feature is used

AIS data comprises digital messages that ships and shore stations broadcast to report identity, position, course, and speed for collision avoidance and traffic monitoring. AISdb-lite turns raw NMEA and CSV feeds into an indexed, queryable database suited to research and operational analysis of vessel movement.

## Installation

The build requires the following.

- Python 3.8 or newer
- Rust toolchain via rustup
- PostgreSQL with the TimescaleDB and PostGIS extensions

```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop            # or: maturin build --release
```

The core install covers decode, ingest, and query. Optional feature sets are packaged as extras and imported lazily, so their dependencies are only needed when the feature is used.

| Extra | Enables | Pulls in |
|-------|---------|----------|
| `scraping` | `DBQuery.check_marinetraffic`, vessel metadata scraping | selenium, webdriver-manager, beautifulsoup4 |
| `weather` | `aisdb.WeatherDataStore` (ERA5 at track positions) | xarray, cfgrib, cdsapi |
| `discretize` | `aisdb.Discretizer` (H3 hex indexing) | h3, matplotlib, geopandas |
| `rasters` | `aisdb.Gebco`, `ShoreDist`, `PortDist`, `InlandDenoising` | pillow, py7zr, requests, tqdm |
| `ports` | `aisdb.WorldPortIndexClient` | pandas, requests |
| `all` | everything above | |

```bash
pip install '.[weather,discretize]'   # from the AISdb-lite checkout, pick what you need
```

AISdb-lite is built from source rather than installed from PyPI, and its import name is `aisdb`. The PyPI `aisdb` distribution is the full [AISdb](https://github.com/MAPS-Lab/AISdb), not this variant.

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

`decode_msgs` creates the global tables from the canonical SQL in `aisdb/aisdb_sql/`, converts pre-existing plain tables into hypertables, and raises if every decode batch fails rather than reporting success over an empty insert.

ExactEarth CSV exports can bypass the NMEA path entirely.

```python
from aisdb.database.decoder_csv import decode_csv_files

decode_csv_files(files, dbconn, source="EXACTEARTH")
```

Runnable end-to-end scripts and notebooks live in `examples/` (`database_creation.py`, `query_db_API.py`, `load_data_fail_handle.py`, plus discretization, ports, and weather notebooks).

## Development

Build with `maturin develop` as shown above. The PostgreSQL test suite reads the connection from environment variables and connects to a database named after the user (`postgresql://$pguser:$pgpass@$pghost:5432/$pguser`).

```bash
export pguser=aisdb_test pgpass=... pghost=localhost
pytest aisdb/tests/test_001_postgres_global.py
```

The tests ingest sample data and assert rows read back and that the BRIN and GiST indexes exist. Raster and land-mask tests download multi-gigabyte archives and are skipped unless `AISDB_RASTER_TESTS=1` is set.

Benchmark and migration write-ups live in `docs/`.

- `docs/BRIN_INDEX_MIGRATION.md` explains why BRIN on `time` replaces the btree for range scans
- `docs/PARALLEL_WORKERS_OPTIMIZATION.md` covers the parallel-worker tuning methodology
- `scripts/benchmarks/parallel_workers_2weeks.sh` provides a reproducible worker-scaling benchmark

Continuous integration builds wheels for Linux, macOS, and Windows and runs the test suite on all three, exercising the PostgreSQL and TimescaleDB paths on Linux and macOS, on every push and pull request. Contribution guidelines are in [CONTRIBUTING.md](CONTRIBUTING.md) and the [Code of Conduct](CODE_OF_CONDUCT.md).

## Documentation

[docs](https://aisviz.gitbook.io/documentation/) · [tutorials](https://aisviz.gitbook.io/tutorials/) · [API reference](https://maps-lab.github.io/AISdb-lite/) · [website](https://aisviz.cs.dal.ca/)

## Related projects

- [AISdb](https://github.com/MAPS-Lab/AISdb) is the canonical general-purpose package on [PyPI](https://pypi.org/project/aisdb/), supporting SQLite as well as PostgreSQL/TimescaleDB
- [NOAA-Integrator](https://github.com/MAPS-Lab/NOAA-Integrator) acquires AIS data from NOAA Marine Cadastre and loads it into AISdb-aligned databases
- [AISdb-Tutorials](https://github.com/MAPS-Lab/AISdb-Tutorials) collects notebooks with worked examples for AISdb workflows

## Citation

If you use AISdb-lite in your work, please cite it. Citation metadata lives in [CITATION.cff](CITATION.cff), and the BibTeX entry follows.

```bibtex
@software{AISdbLite2026:GSpadon,
  author    = {Spadon, Gabriel},
  title     = {AISdb-lite},
  year      = {2026},
  version   = {1.8.0-alpha},
  publisher = {MAPS Lab, Dalhousie University},
  url       = {https://github.com/MAPS-Lab/AISdb-lite},
  license   = {AGPL-3.0}
}
```

## License

This project is distributed under the terms of the GNU Affero General Public License v3.0 (AGPL-3.0). See [LICENSE](LICENSE) for details.
