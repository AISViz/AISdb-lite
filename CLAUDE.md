# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

AISdb-lite is a maritime vessel tracking system for storing, retrieving, analyzing, and visualizing Automatic Identification System (AIS) data. It's a hybrid Python/Rust codebase using PyO3 for FFI bindings.

**Version:** 1.8.0-alpha
**License:** AGPLv3+

### Key Capabilities

| Capability | Implementation |
|------------|----------------|
| AIS Decoding | Rust `decoder()` via PyO3 (types 1,2,3,5,18,19,24,27) |
| Track Generation | Python `TrackGen` generator function |
| Spatial Queries | PostgreSQL + PostGIS with GIST indexes |
| Real-time Streaming | Rust tungstenite WebSocket server |
| Track Interpolation | 4 methods: linear, cubic spline, geodesic, spacing |
| Weather Integration | Copernicus CDS (ERA5) via `WeatherDataStore` |
| H3 Indexing | Uber H3 hexagonal spatial indexing |

## Technology Stack

- **Python 3.8+**: Main API, database queries, track processing, data analysis
- **Rust 2021 Edition**: Performance-critical operations (CSV parsing, AIS decoding, binary search)
- **Rust Nightly**: Required for database_server (generators feature)
- **PostgreSQL 17 + TimescaleDB 2.24 + PostGIS**: Primary database (SQLite deprecated)
- **PyO3/Maturin**: Python-Rust FFI bindings
- **WebAssembly**: Browser-based client (via wasm-pack)
- **JavaScript/TypeScript**: Web frontend with OpenLayers 7+ and Vite 4.3.3

## Build Commands

```bash
# Development build (requires Rust toolchain)
python -m venv venv && source venv/bin/activate
pip install maturin[patchelf]
maturin develop --release --extras=test,docs

# Run tests (requires PostgreSQL with TimescaleDB/PostGIS)
pytest aisdb/tests/

# Run specific test file
pytest aisdb/tests/test_006_gis.py -v

# Run tests excluding problematic ones (as CI does)
pytest aisdb/tests/ --ignore=aisdb/tests/test_014_marinetraffic.py \
    --ignore=aisdb/tests/test_001_postgres.py \
    --ignore=aisdb/tests/test_004_sqlfcn.py \
    --ignore=aisdb/tests/test_002_decode.py \
    --ignore=aisdb/tests/test_005_dbqry.py --maxfail=10

# Build Rust only
cargo check
cargo build --release

# Build WebAssembly client
cd client_webassembly && wasm-pack build --release

# Build web frontend
cd aisdb_web && npm install && npm run build
```

## Architecture

```
AISdb-lite/
├── aisdb/                    # Python package
│   ├── database/             # DB connections, queries, decoders
│   │   ├── dbconn.py         # PostgresDBConn class
│   │   ├── dbqry.py          # DBQuery class for building queries
│   │   ├── decoder.py        # AIS message decoding orchestration
│   │   └── sql_query_strings.py  # SQL generation functions
│   ├── aisdb_sql/            # SQL schema files
│   ├── webdata/              # External data sources (bathymetry, shore distance)
│   ├── weather/              # ERA5/weather data integration
│   ├── discretize/           # H3 hexagonal binning
│   ├── tests/                # pytest test suite
│   ├── gis.py                # Geographic utilities (projections, distances)
│   ├── interp.py             # Track interpolation
│   ├── track_gen.py          # Track generation from DB rows
│   └── proc_util.py          # Track processing utilities
├── aisdb_lib/                # Core Rust library
│   └── src/
│       ├── csvreader.rs      # CSV parsing (~51K lines capability)
│       ├── db.rs             # Database operations
│       ├── decode.rs         # NMEA/AIS message decoding
│       └── util.rs           # File utilities
├── database_server/          # Rust WebSocket server for queries
├── receiver/                 # Rust AIS receiver daemon
├── client_webassembly/       # WASM client for browser
├── aisdb_web/                # Web frontend (OpenLayers map)
├── src/lib.rs                # PyO3 bindings (main entry point)
└── audit/                    # Automated codebase analysis system
```

## Key Data Flow

1. **Ingestion**: Raw AIS (NMEA/CSV) → Rust decoder → PostgreSQL/TimescaleDB
2. **Query**: Python DBQuery → SQL generation → PostgresDBConn → Track dictionaries
3. **Processing**: Track dicts → interpolation/filtering → enrichment (weather, bathymetry)
4. **Export**: GeoJSON, CSV, or direct visualization

## Public API

### Classes (8 total)

| Class | Module | Purpose |
|-------|--------|---------|
| `PostgresDBConn` | database.dbconn | Database connection manager with context support |
| `DBQuery` | database.dbqry | Query builder (UserDict subclass) |
| `Domain` | gis | Geographic bounding box with polygon support |
| `Gebco` | webdata.bathymetry | GEBCO 2022 bathymetric data access |
| `ShoreDist` | webdata.shore_dist | Shore distance calculations |
| `PortDist` | webdata.shore_dist | Port distance calculations |
| `WeatherDataStore` | weather.data_store | Weather data management (GRIB files) |
| `Discretizer` | discretize.h3 | H3 hexagonal binning utilities |

**Note:** `TrackGen` is a **generator function**, not a class.

### Key Functions

| Function | Purpose |
|----------|---------|
| `decode_msgs()` | AIS file decoding (CSV, NM4, NMEA, GZIP, ZIP) |
| `TrackGen()` | Track generation from database rows |
| `split_timedelta()` | Track splitting by time gap |
| `interp_time()` | Time-based interpolation |
| `interp_spacing()` | Distance-based interpolation |
| `interp_cubic_spline()` | Cubic spline interpolation |
| `geo_interp_time()` | Geodesic interpolation |
| `encode_greatcircledistance()` | Track segmentation |
| `haversine()` | Great-circle distance calculation |

### Rust FFI Functions (6 PyFunctions via PyO3)

| Function | Purpose |
|----------|---------|
| `decoder` | NMEA message decoding to database |
| `haversine` | Great-circle distance calculation |
| `simplify_linestring_idx` | Track simplification (Douglas-Peucker) |
| `encoder_score_fcn` | Denoising anomaly scores |
| `binarysearch_vector` | Vectorized binary search |
| `receiver` | AIS receiver wrapper |

## Database Schema

Tables use monthly partitioning pattern: `ais_{YYYYMM}_dynamic`, `ais_{YYYYMM}_static`

**Dynamic table** (position reports):
- `mmsi INTEGER`, `time INTEGER` (Unix epoch), `longitude/latitude REAL`
- `sog REAL` (speed over ground), `cog REAL` (course), `heading REAL`
- `geom GEOMETRY(POINT, 4326)` - PostGIS auto-generated from lon/lat

**Static table** (vessel metadata):
- `mmsi INTEGER PRIMARY KEY`, `imo INTEGER`, `vessel_name TEXT`
- `ship_type INTEGER`, `dim_bow/stern/port/starboard INTEGER`

**TimescaleDB Configuration:**
- Chunk interval: 7 days (604800 seconds)
- Partitions: 4 (by mmsi)
- Compression: Disabled by default

**Indexes:**
- `GIST` on `geom` column for spatial queries
- B-tree on `time`, `mmsi`, `longitude`, `latitude`

## Environment Variables

```bash
# PostgreSQL connection (required for tests)
export PGHOST=127.0.0.1
export PGUSER=postgres
export PGPASSWORD=your_password
export PGDATABASE=aisdb

# Optional
export AISDBTESTDIR=/path/to/testdata
export LOGLEVEL=DEBUG
```

## Audit System

The `audit/` directory contains an automated multi-agent analysis system:

```bash
# Run full audit (generates reports 0-4)
./audit/run_audit.sh

# Run specific prompt
./audit/run_audit.sh 1  # Bug analysis only

# Reports generated:
# 0-REPORT.md - Architecture documentation
# 1-REPORT.md - Bug analysis (173 confirmed bugs)
# 2-REPORT.md - Bad business decisions (290+ issues)
# 3-REPORT.md - Cross-report contradiction analysis
# 4-REPORT.md - Engineering blueprint for refactoring
```

## Known Issues (from audit reports)

### Critical
- **RUST-001/003**: Early `return Ok(())` in CSV parsing terminates on first invalid row
- **PYDB-001**: SQL injection in `sql_query_strings.py` via f-string interpolation
- **SQL-001/002**: Wrong column in UPSERT (`summer_dwt = excluded.gross_tonnage`)
- **INT-001**: Year 2038 problem (32-bit timestamps throughout)
- **WEB-001**: JavaScript comma operator bug `coords[-1, 0]` evaluates to `coords[0]`

### High
- **PYDB-004**: Mutable default argument `args=[]` in `dbconn.py:execute()`
- **WEBDATA-001**: Latitude/longitude swap in `load_raster.py:61`
- **BUILD-001**: CI triggers on `master` but main branch is `main`

## Code Patterns

### Track Dictionary Structure
```python
track = {
    'mmsi': 123456789,
    'time': np.array([...], dtype=np.uint32),
    'lon': np.array([...], dtype=np.float32),
    'lat': np.array([...], dtype=np.float32),
    'sog': np.array([...]),
    'cog': np.array([...]),
    'static': ['mmsi', 'imo', 'vessel_name', ...],
    'dynamic': ['time', 'lon', 'lat', 'sog', 'cog', ...],
}
```

### Database Query Pattern
```python
from aisdb import PostgresDBConn, DBQuery, TrackGen

with PostgresDBConn(connection_string) as dbconn:
    qry = DBQuery(
        dbconn=dbconn,
        start=datetime(2021, 7, 1),
        end=datetime(2021, 7, 2),
        callback=aisdb.sqlfcn_callbacks.in_timerange,
    )
    for track in TrackGen(qry.gen_qry(), decimate=False):
        process(track)
```

### Rust FFI Functions (via PyO3)
```python
from aisdb import (
    decoder,           # NMEA message decoding
    binarysearch_vector,  # Fast binary search
    haversine,         # Distance calculation
)
```

## Testing Notes

- **19 test files**, **60 test functions** total
- Tests require running PostgreSQL with TimescaleDB and PostGIS extensions
- **ALL tests are PostgreSQL-only** (no SQLite tests exist)
- Many tests lack assertions (execute code but don't validate results)
- Test files use hardcoded paths relative to `aisdb/tests/testdata/`
- CI skips several test files due to environment dependencies
- Test data files: CSV (July 2021), NM4 (Nov 2021), NMEA (Dec 2012), NOAA format

## Contributing

When modifying code:
1. Run `maturin develop --release` after Rust changes
2. Run affected tests with `pytest -v`
3. Check for SQL injection if modifying query generation
4. Use parameterized queries, not f-strings for SQL
5. Prefer `Result<T, E>` over `.unwrap()` in Rust code

## Rust Crate Structure

| Crate | Purpose | Notes |
|-------|---------|-------|
| `aisdb` (root) | PyO3 bindings | BATCHSIZE = 50000 |
| `aisdb-lib` | Core library | csvreader, db, decode, util |
| `database_server` | WebSocket query server | Requires nightly Rust |
| `receiver` | AIS receiver daemon | TCP/UDP via mproxy |
| `client_webassembly` | Browser WASM client | Single `process_response` export |

## Web Frontend

- **Ports**: 9924 (DB WebSocket), 9922 (live stream), 9923 (HTTP/Flask)
- **Layers**: 6 vector layers (tracks, vessels, zones, selected, highlight, metadata)
- **Palette**: 39 colors, 30+ vessel type mappings
- **Storage**: IndexedDB for client-side caching

## SQL Files Reference

30 SQL template files in `aisdb/aisdb_sql/`:
- `createtable_*.sql` - Table creation
- `timescale_createtable_*.sql` - TimescaleDB hypertables
- `psql_createtable_*.sql` - PostgreSQL-specific tables
- `insert_*.sql` - Insert statement templates
- `select_*.sql` - Query templates
- `cte_*.sql` - Common Table Expressions
- `coarsetype.sql` - Ship type reference (81 rows)
