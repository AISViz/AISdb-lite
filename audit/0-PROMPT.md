# Multi-Agent Repository Analysis System

## Purpose

This prompt orchestrates a comprehensive analysis of the AISdb-lite repository using specialized agents. The system is designed to:

1. Generate a complete technical report (`0-REPORT.md`)
2. Track all changes across successive runs (`0-CHANGELOG.md`)
3. Avoid duplications by checking existing content
4. Correct misunderstandings and outdated information
5. Add missing content discovered during analysis

---

## Report Writing Guidelines

### No Page Limit
- **There is NO page limit** for the report
- Document everything necessary for complete understanding
- Prioritize completeness over brevity when accuracy requires detail

### Avoid Duplications
- Never repeat the same information in multiple sections
- Cross-reference related sections instead of duplicating content
- Before adding content, verify it doesn't exist elsewhere in the report
- Use "See Section X.X" references for related information

### Reduce Verbosity
- Be direct and concise - avoid filler words and unnecessary preambles
- Use bullet points and tables over prose where appropriate
- One fact per sentence; avoid compound explanations
- Remove hedging language ("it seems", "appears to be", "might")
- State findings definitively based on code evidence

### Traceability Requirements
Every claim MUST include traceback information:

```
REQUIRED FORMAT:
- File path: exact path from repository root
- Line numbers: specific lines, not ranges > 20 lines
- Code snippet: relevant excerpt (< 10 lines)
- Verification command: how to confirm the finding

EXAMPLE:
**Finding:** TrackGen is a generator function, not a class.
**Location:** `aisdb/track_gen.py:92-95`
**Evidence:**
```python
def TrackGen(rowgen: iter, decimate: False) -> dict:
    '''Generate track dictionaries from database rows.'''
    yield track_dict
```
**Verify:** `grep -n "def TrackGen" aisdb/track_gen.py`
```

### Writing Style
- Active voice: "The function returns X" not "X is returned by the function"
- Present tense for current state: "The file contains" not "The file contained"
- Technical precision: use exact names, types, and values from code
- No speculation: if uncertain, investigate; if unverifiable, omit

### ASCII Diagrams (Required for 0-REPORT)

The architecture report MUST include comprehensive ASCII diagrams following software engineering best practices. These diagrams are MANDATORY for complete system documentation.

---

#### CATEGORY 1: System Architecture Diagrams

**1.1 High-Level System Overview**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                        AISdb-Lite v1.8.0-alpha                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐         │
│  │   RUST LAYER    │  │  PYTHON LAYER   │  │    JS LAYER     │         │
│  │   (Performance) │  │ (Business Logic)│  │  (Presentation) │         │
│  │                 │  │                 │  │                 │         │
│  │ • aisdb-lib     │  │ • aisdb/        │  │ • aisdb_web/    │         │
│  │ • receiver      │◄─┤ • database/     │◄─┤ • map/          │         │
│  │ • db-server     │  │ • webdata/      │  │ • IndexedDB     │         │
│  │ • WASM client   │  │ • weather/      │  │ • OpenLayers    │         │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘         │
│           │                    │                    │                   │
│           └────────────────────┼────────────────────┘                   │
│                                │                                        │
│                       ┌────────▼────────┐                               │
│                       │   PostgreSQL    │                               │
│                       │ PostGIS+Timescale│                              │
│                       └─────────────────┘                               │
└─────────────────────────────────────────────────────────────────────────┘
```

**1.2 Deployment Architecture**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                         DEPLOYMENT VIEW                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   EXTERNAL                    SERVER                      DATABASE      │
│  ┌─────────┐              ┌─────────────┐              ┌─────────────┐ │
│  │AIS Feed │──TCP:9921───▶│  Receiver   │──INSERT─────▶│ PostgreSQL  │ │
│  │ (NMEA)  │              │   (Rust)    │              │   :5432     │ │
│  └─────────┘              └─────────────┘              └──────┬──────┘ │
│                                                               │        │
│  ┌─────────┐              ┌─────────────┐                     │        │
│  │ Browser │◀─WSS:9924───▶│  DB Server  │◀────SELECT─────────┘        │
│  │ Client  │              │   (Rust)    │                              │
│  └─────────┘              └─────────────┘                              │
│       │                                                                 │
│       │                   ┌─────────────┐                              │
│       └──HTTP:9923───────▶│Flask Server │                              │
│                           │  (Python)   │                              │
│                           └─────────────┘                              │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 2: Cross-Language Interaction Diagrams

**2.1 Rust ↔ Python FFI Boundary (PyO3)**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    RUST-PYTHON FFI BOUNDARY (PyO3)                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  PYTHON SIDE                    FFI                    RUST SIDE        │
│  ───────────                    ───                    ─────────        │
│                                                                         │
│  from aisdb import *      ┌──────────────┐      #[pyfunction]          │
│         │                 │              │      fn decoder(...)         │
│         ▼                 │   PyO3       │             │                │
│  decoder(path)  ─────────▶│   Bindings   │─────────────▶ VesselData    │
│         │                 │              │             │                │
│         │                 │  Type Conv:  │             │                │
│  List[dict]  ◀────────────│  Vec<T>→List │◀────────────┘                │
│                           │  String→str  │                              │
│                           │  i32→int     │                              │
│                           └──────────────┘                              │
│                                                                         │
│  EXPORTED FUNCTIONS:                                                    │
│  ┌────────────────────┬────────────────────┬─────────────────────────┐ │
│  │ Python Name        │ Rust Function      │ Data Transformation     │ │
│  ├────────────────────┼────────────────────┼─────────────────────────┤ │
│  │ decoder()          │ decoder()          │ Path→Vec<VesselData>    │ │
│  │ haversine()        │ haversine()        │ (f64,f64,f64,f64)→f64   │ │
│  │ encoder_score_fcn()│ encoder_score_fcn()│ arrays→scores           │ │
│  │ receiver()         │ receiver()         │ ReceiverArgs→()         │ │
│  └────────────────────┴────────────────────┴─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
```

**2.2 Python ↔ JavaScript Communication**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                  PYTHON ↔ JAVASCRIPT COMMUNICATION                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  JAVASCRIPT (Browser)              PYTHON (Server)                      │
│  ────────────────────              ───────────────                      │
│                                                                         │
│  ┌─────────────────┐              ┌─────────────────┐                  │
│  │  clientsocket.js│              │  Flask Server   │                  │
│  │                 │──HTTP GET───▶│  web_interface  │                  │
│  │  fetch('/api')  │              │                 │                  │
│  │                 │◀──JSON───────│  return jsonify │                  │
│  └─────────────────┘              └─────────────────┘                  │
│                                                                         │
│  ┌─────────────────┐              ┌─────────────────┐                  │
│  │  clientsocket.js│              │  Rust DB Server │                  │
│  │                 │──WSS:9924───▶│  (tungstenite)  │                  │
│  │  new WebSocket()│              │                 │                  │
│  │                 │◀─Binary/JSON─│  query results  │                  │
│  └─────────────────┘              └─────────────────┘                  │
│                                                                         │
│  MESSAGE PROTOCOL:                                                      │
│  ┌──────────────┬────────────┬──────────────────────────────────────┐  │
│  │ Direction    │ Format     │ Content                              │  │
│  ├──────────────┼────────────┼──────────────────────────────────────┤  │
│  │ JS → Server  │ JSON       │ {type:"track_vectors",params:{...}}  │  │
│  │ Server → JS  │ Binary+GZ  │ Compressed GeoJSON features          │  │
│  └──────────────┴────────────┴──────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

**2.3 Rust ↔ Database Interaction**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    RUST ↔ DATABASE INTERACTION                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  RUST (aisdb_lib/db.rs)                    PostgreSQL                   │
│  ──────────────────────                    ──────────                   │
│                                                                         │
│  ┌───────────────────┐                    ┌───────────────────┐        │
│  │  postgres crate   │                    │                   │        │
│  │                   │                    │  ais_dynamic      │        │
│  │  Client::connect()│───TCP:5432────────▶│  ais_static       │        │
│  │                   │                    │  coarsetype_ref   │        │
│  │  client.execute() │───SQL INSERT──────▶│                   │        │
│  │  client.query()   │◀──Rows────────────│                   │        │
│  └───────────────────┘                    └───────────────────┘        │
│                                                                         │
│  SQL LOADING (Compile-time via build.rs):                              │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │  aisdb/aisdb_sql/*.sql  ──include_dir!()──▶  SQLFILES constant  │   │
│  │                                                                  │   │
│  │  At runtime: SQLFILES.get_file("insert_dynamic.sql")            │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 3: Data Flow Diagrams

**3.1 AIS Data Ingestion Pipeline**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                      AIS DATA INGESTION PIPELINE                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐         │
│  │  AIS     │    │ Receiver │    │  Decoder │    │ Database │         │
│  │  Source  │───▶│  (Rust)  │───▶│  (Rust)  │───▶│  Insert  │         │
│  └──────────┘    └──────────┘    └──────────┘    └──────────┘         │
│       │               │               │               │                 │
│       ▼               ▼               ▼               ▼                 │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐         │
│  │ NMEA     │    │ Raw msgs │    │ VesselData│   │ ais_     │         │
│  │ Sentences│    │ in buffer│    │ structs   │    │ dynamic  │         │
│  │          │    │ (64KB)   │    │           │    │ table    │         │
│  └──────────┘    └──────────┘    └──────────┘    └──────────┘         │
│                                                                         │
│  FILE INGESTION PATH:                                                   │
│  ┌────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐           │
│  │.nm4/.csv│──▶│decode_msgs│──▶│ Checksum │──▶│ Batch    │──▶[DB]    │
│  │ files   │   │ (Python) │    │ Verify   │    │ INSERT   │           │
│  └────────┘    └──────────┘    └──────────┘    └──────────┘           │
│                      │                                                  │
│                      ▼                                                  │
│                ┌──────────┐                                            │
│                │FileCheck-│  (MD5 hash of first 1000 bytes)            │
│                │sums class│                                            │
│                └──────────┘                                            │
└─────────────────────────────────────────────────────────────────────────┘
```

**3.2 Query → Visualization Pipeline**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    QUERY → VISUALIZATION PIPELINE                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  USER ACTION          FRONTEND           BACKEND            DATABASE    │
│  ───────────          ────────           ───────            ────────    │
│                                                                         │
│  Click "Search"                                                         │
│       │                                                                 │
│       ▼                                                                 │
│  ┌─────────┐     ┌─────────────┐                                       │
│  │selectform│────▶│Build query  │                                       │
│  │.js      │     │params (JSON)│                                       │
│  └─────────┘     └──────┬──────┘                                       │
│                         │                                               │
│                         ▼                                               │
│                  ┌─────────────┐     ┌─────────────┐                   │
│                  │WebSocket    │────▶│ DB Server   │                   │
│                  │send(params) │     │ (Rust)      │                   │
│                  └─────────────┘     └──────┬──────┘                   │
│                                             │                           │
│                                             ▼                           │
│                                      ┌─────────────┐    ┌──────────┐  │
│                                      │ SQL Query   │───▶│PostgreSQL│  │
│                                      │ Generator   │    │ + PostGIS│  │
│                                      └─────────────┘    └────┬─────┘  │
│                                                              │         │
│                         ┌────────────────────────────────────┘         │
│                         ▼                                               │
│                  ┌─────────────┐                                       │
│                  │ GeoJSON     │                                       │
│                  │ + Compress  │                                       │
│                  └──────┬──────┘                                       │
│                         │                                               │
│       ┌─────────────────┘                                              │
│       ▼                                                                 │
│  ┌─────────┐     ┌─────────────┐     ┌─────────────┐                   │
│  │ map.js  │◀────│ Parse &     │◀────│ WebSocket   │                   │
│  │ render  │     │ Decompress  │     │ onmessage   │                   │
│  └─────────┘     └─────────────┘     └─────────────┘                   │
│       │                                                                 │
│       ▼                                                                 │
│  ┌─────────────┐                                                       │
│  │ OpenLayers  │  Vessels displayed on map                             │
│  │ Vector Layer│                                                       │
│  └─────────────┘                                                       │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 4: Sequence Diagrams (UML-style)

**4.1 Track Query Sequence**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                       TRACK QUERY SEQUENCE                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Browser        clientsocket.js    DB Server       PostgreSQL           │
│     │                 │                │                │               │
│     │  User clicks    │                │                │               │
│     │  "Query"        │                │                │               │
│     │────────────────▶│                │                │               │
│     │                 │                │                │               │
│     │                 │  WSS connect   │                │               │
│     │                 │───────────────▶│                │               │
│     │                 │                │                │               │
│     │                 │  JSON request  │                │               │
│     │                 │  {type:"track_ │                │               │
│     │                 │   vectors"...} │                │               │
│     │                 │───────────────▶│                │               │
│     │                 │                │                │               │
│     │                 │                │  SQL query     │               │
│     │                 │                │───────────────▶│               │
│     │                 │                │                │               │
│     │                 │                │  Result rows   │               │
│     │                 │                │◀───────────────│               │
│     │                 │                │                │               │
│     │                 │  Binary+GZ     │                │               │
│     │                 │  GeoJSON       │                │               │
│     │                 │◀───────────────│                │               │
│     │                 │                │                │               │
│     │  Render on map  │                │                │               │
│     │◀────────────────│                │                │               │
│     │                 │                │                │               │
│     ▼                 ▼                ▼                ▼               │
└─────────────────────────────────────────────────────────────────────────┘
```

**4.2 File Decode Sequence**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                       FILE DECODE SEQUENCE                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  User Code      decode_msgs()    FileChecksums    Rust decoder    DB    │
│     │                │                │                │           │    │
│     │ decode_msgs    │                │                │           │    │
│     │ (files,dbconn) │                │                │           │    │
│     │───────────────▶│                │                │           │    │
│     │                │                │                │           │    │
│     │                │  Check hash    │                │           │    │
│     │                │───────────────▶│                │           │    │
│     │                │                │                │           │    │
│     │                │  hash exists?  │                │           │    │
│     │                │◀───────────────│                │           │    │
│     │                │                │                │           │    │
│     │                │  [if new file] │                │           │    │
│     │                │                │                │           │    │
│     │                │  decoder(path) │                │           │    │
│     │                │  via PyO3      │                │           │    │
│     │                │───────────────────────────────▶│           │    │
│     │                │                │                │           │    │
│     │                │                │  VesselData[]  │           │    │
│     │                │◀───────────────────────────────│           │    │
│     │                │                │                │           │    │
│     │                │  Batch INSERT (50000 rows)     │           │    │
│     │                │─────────────────────────────────────────▶│    │
│     │                │                │                │           │    │
│     │                │  Store hash    │                │           │    │
│     │                │───────────────▶│                │           │    │
│     │                │                │                │           │    │
│     │  Generator     │                │                │           │    │
│     │  yields tracks │                │                │           │    │
│     │◀───────────────│                │                │           │    │
│     ▼                ▼                ▼                ▼           ▼    │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 5: State Diagrams

**5.1 WebSocket Connection State**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    WEBSOCKET CONNECTION STATE                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                        ┌─────────────┐                                  │
│                        │ DISCONNECTED│◀─────────────────┐               │
│                        └──────┬──────┘                  │               │
│                               │                         │               │
│                               │ connect()               │ error/close   │
│                               ▼                         │               │
│                        ┌─────────────┐                  │               │
│                        │ CONNECTING  │──────────────────┤               │
│                        └──────┬──────┘                  │               │
│                               │                         │               │
│                               │ onopen                  │               │
│                               ▼                         │               │
│                        ┌─────────────┐                  │               │
│              ┌────────▶│  CONNECTED  │──────────────────┘               │
│              │         └──────┬──────┘                                  │
│              │                │                                         │
│              │                │ send(query)                             │
│              │                ▼                                         │
│              │         ┌─────────────┐                                  │
│              │         │   QUERYING  │                                  │
│              │         └──────┬──────┘                                  │
│              │                │                                         │
│              │                │ onmessage                               │
│              └────────────────┘                                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**5.2 Track Processing State**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                     TRACK PROCESSING PIPELINE STATE                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐   │
│  │  RAW   │───▶│GROUPED │───▶│ SPLIT  │───▶│INTERP- │───▶│DENOISED│   │
│  │POSITIONS│   │BY MMSI │    │BY TIME │    │ OLATED │    │        │   │
│  └────────┘    └────────┘    └────────┘    └────────┘    └────────┘   │
│       │             │             │             │             │         │
│       ▼             ▼             ▼             ▼             ▼         │
│  ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐   │
│  │DB rows │    │TrackGen│    │split_  │    │interp_ │    │encode_ │   │
│  │cursor  │    │yields  │    │timedelta│   │time()  │    │score() │   │
│  │        │    │dict    │    │         │    │        │    │        │   │
│  └────────┘    └────────┘    └────────┘    └────────┘    └────────┘   │
│                                                                         │
│  TRACK DICT STRUCTURE AT EACH STAGE:                                   │
│  {                                                                      │
│    'mmsi': int,                                                        │
│    'time': ndarray[int32],    # Unix timestamps                        │
│    'lon': ndarray[float64],   # Longitudes                             │
│    'lat': ndarray[float64],   # Latitudes                              │
│    'sog': ndarray[float32],   # Speed over ground (optional)           │
│    'cog': ndarray[float32],   # Course over ground (optional)          │
│  }                                                                      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 6: Component & Module Diagrams

**6.1 Rust Crate Dependency Tree**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                      RUST CRATE DEPENDENCY TREE                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                          ┌──────────────┐                               │
│                          │    aisdb     │ (root, PyO3 exports)          │
│                          │   src/lib.rs │                               │
│                          └───────┬──────┘                               │
│                                  │                                      │
│               ┌──────────────────┼──────────────────┐                   │
│               │                  │                  │                   │
│               ▼                  ▼                  ▼                   │
│        ┌──────────┐       ┌──────────┐       ┌──────────┐              │
│        │aisdb_lib │       │ receiver │       │db_server │              │
│        │          │       │          │       │ (nightly)│              │
│        └────┬─────┘       └────┬─────┘       └────┬─────┘              │
│             │                  │                  │                     │
│     ┌───────┴───────┐         │          ┌───────┴───────┐             │
│     │               │         │          │               │             │
│     ▼               ▼         ▼          ▼               ▼             │
│ ┌────────┐    ┌────────┐ ┌────────┐ ┌────────┐    ┌────────┐          │
│ │postgres│    │  csv   │ │ mproxy │ │tungsten│    │ flate2 │          │
│ │ 0.19   │    │  1.1   │ │  0.1.8 │ │  0.20  │    │  1.0   │          │
│ └────────┘    └────────┘ └────────┘ └────────┘    └────────┘          │
│                                                                         │
│  FEATURE FLAGS:                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ aisdb_lib: postgres (default), sqlite (optional)                │   │
│  │ db_server: requires nightly (generators feature)                │   │
│  │ receiver:  TLS via mproxy-* crates                              │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

**6.2 Python Module Hierarchy**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                      PYTHON MODULE HIERARCHY                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  aisdb/                                                                 │
│  ├── __init__.py ─────────────────────────────────────────────────────┐│
│  │   EXPORTS: PostgresDBConn, DBQuery, Domain, TrackGen, decode_msgs..││
│  │                                                                     ││
│  ├── database/ ◀──────────────────────────────────────────────────────┘│
│  │   ├── dbconn.py ──────▶ PostgresDBConn class                        │
│  │   ├── dbqry.py ───────▶ DBQuery class (UserDict)                    │
│  │   ├── decoder.py ─────▶ decode_msgs(), FileChecksums                │
│  │   ├── sqlfcn.py ──────▶ CTE builders                                │
│  │   └── sqlfcn_callbacks.py ▶ WHERE clause lambdas                    │
│  │                                                                      │
│  ├── track_gen.py ───────▶ TrackGen (generator function)               │
│  ├── interp.py ──────────▶ interp_time, interp_spacing (4 functions)   │
│  ├── gis.py ─────────────▶ Domain class, coordinate transforms         │
│  ├── proc_util.py ───────▶ 13 utility functions                        │
│  ├── denoising_encoder.py ▶ encode_score, InlandDenoising              │
│  │                                                                      │
│  ├── webdata/                                                          │
│  │   ├── marinetraffic.py ▶ VesselInfo class (Selenium scraper)        │
│  │   ├── bathymetry.py ───▶ Gebco class                                │
│  │   └── shore_dist.py ───▶ ShoreDist, PortDist, CoastDist             │
│  │                                                                      │
│  ├── weather/                                                          │
│  │   ├── weather_fetch.py ▶ ClimateDataStore (Copernicus CDS)          │
│  │   └── data_store.py ───▶ WeatherDataStore                           │
│  │                                                                      │
│  └── discretize/                                                       │
│      └── h3.py ───────────▶ Discretizer class (H3 hexagonal)           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 7: Database Schema Diagrams

**7.1 Entity Relationship Diagram**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    DATABASE ENTITY RELATIONSHIPS                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────┐         ┌─────────────────────┐               │
│  │    ais_dynamic      │         │    ais_static       │               │
│  ├─────────────────────┤         ├─────────────────────┤               │
│  │ PK: mmsi, time,     │         │ PK: mmsi            │               │
│  │     lat, lon        │         ├─────────────────────┤               │
│  ├─────────────────────┤         │ vessel_name         │               │
│  │ mmsi: INTEGER       │◀───────▶│ ship_type: INTEGER  │───┐           │
│  │ time: INTEGER       │  (mmsi) │ imo: INTEGER        │   │           │
│  │ lon: DOUBLE PREC    │         │ call_sign           │   │           │
│  │ lat: DOUBLE PREC    │         │ dim_bow, dim_stern  │   │           │
│  │ sog: REAL           │         │ dim_port, dim_star  │   │           │
│  │ cog: REAL           │         │ draught             │   │           │
│  │ heading: REAL       │         └─────────────────────┘   │           │
│  │ nav_status          │                                    │           │
│  │ rot: REAL           │         ┌─────────────────────┐   │           │
│  │ geom: GEOMETRY      │         │  coarsetype_ref     │◀──┘           │
│  │ (PostGIS generated) │         ├─────────────────────┤  (ship_type)  │
│  └─────────────────────┘         │ PK: coarse_type     │               │
│           │                      ├─────────────────────┤               │
│           │ (TimescaleDB)        │ coarse_type: INT    │               │
│           ▼                      │ coarse_type_txt     │               │
│  ┌─────────────────────┐         └─────────────────────┘               │
│  │   Hypertable        │                                                │
│  │   (7-day chunks)    │         ┌─────────────────────┐               │
│  │   Compression: ON   │         │webdata_marinetraffic│               │
│  └─────────────────────┘         ├─────────────────────┤               │
│                                  │ PK: mmsi            │               │
│                                  │ imo, name, flag     │               │
│                                  │ gross_tonnage, dwt  │               │
│                                  │ length, breadth     │               │
│                                  │ year_built          │               │
│                                  └─────────────────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 8: External Integration Diagrams

**8.1 External Services Integration**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                    EXTERNAL SERVICES INTEGRATION                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│                          ┌─────────────────┐                            │
│                          │   AISdb-Lite    │                            │
│                          │     System      │                            │
│                          └────────┬────────┘                            │
│                                   │                                     │
│       ┌───────────────────────────┼───────────────────────────┐        │
│       │                           │                           │        │
│       ▼                           ▼                           ▼        │
│ ┌───────────────┐         ┌───────────────┐         ┌───────────────┐ │
│ │ MarineTraffic │         │  Copernicus   │         │    GEBCO      │ │
│ │   (Scraper)   │         │   CDS API     │         │  Bathymetry   │ │
│ ├───────────────┤         ├───────────────┤         ├───────────────┤ │
│ │Protocol:      │         │Protocol:      │         │Protocol:      │ │
│ │ HTTP/Selenium │         │ REST API      │         │ File Download │ │
│ │               │         │               │         │               │ │
│ │Data:          │         │Data:          │         │Data:          │ │
│ │ Vessel specs  │         │ Weather grids │         │ Depth rasters │ │
│ │ IMO, name,    │         │ ERA5, HYCOM   │         │ NetCDF format │ │
│ │ dimensions    │         │ GRIB format   │         │               │ │
│ │               │         │               │         │               │ │
│ │Rate Limit:    │         │Rate Limit:    │         │Rate Limit:    │ │
│ │ sleep(1-3s)   │         │ API quotas    │         │ N/A (local)   │ │
│ └───────────────┘         └───────────────┘         └───────────────┘ │
│       │                           │                           │        │
│       ▼                           ▼                           ▼        │
│ ┌───────────────┐         ┌───────────────┐         ┌───────────────┐ │
│ │webdata_marine │         │WeatherData-   │         │ Gebco class   │ │
│ │traffic table  │         │Store class    │         │ merge_tracks()│ │
│ └───────────────┘         └───────────────┘         └───────────────┘ │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### CATEGORY 9: User Interaction Diagrams

**9.1 Web Interface User Flow**
```
┌─────────────────────────────────────────────────────────────────────────┐
│                      WEB INTERFACE USER FLOW                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                        BROWSER WINDOW                             │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │  [Date Picker] [Time Range] [Area Select] [🔍 Search]     │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                    │  │
│  │                              ▼                                    │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │                                                            │  │  │
│  │  │                    OPENLAYERS MAP                          │  │  │
│  │  │                                                            │  │  │
│  │  │     🚢──────────────🚢                                     │  │  │
│  │  │            \       /                                       │  │  │
│  │  │             \     /   (vessel tracks)                      │  │  │
│  │  │              \   /                                         │  │  │
│  │  │               🚢                                           │  │  │
│  │  │                                                            │  │  │
│  │  │  [Click vessel] ──▶ Popup: MMSI, Name, Speed, Course       │  │  │
│  │  │                                                            │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                    │  │
│  │                              ▼                                    │  │
│  │  ┌────────────────────────────────────────────────────────────┐  │  │
│  │  │  Status: Connected ● | Vessels: 1,234 | Last Update: 12:34 │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  USER ACTIONS:                                                          │
│  ┌────────────┬─────────────────────────────────────────────────────┐  │
│  │ Action     │ System Response                                     │  │
│  ├────────────┼─────────────────────────────────────────────────────┤  │
│  │ Load page  │ Connect WebSocket:9924, load IndexedDB cache        │  │
│  │ Set params │ Validate inputs, enable Search button               │  │
│  │ Click Search│ Send query via WebSocket, show loading spinner     │  │
│  │ Receive data│ Parse GeoJSON, render on vector layer              │  │
│  │ Click vessel│ Query metadata, show popup                         │  │
│  │ Pan/zoom   │ Request new tiles, update visible area              │  │
│  └────────────┴─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

#### Diagram Guidelines

**Character Set:**
```
Box drawing:  ┌ ┐ └ ┘ ─ │ ├ ┤ ┬ ┴ ┼ ╔ ╗ ╚ ╝ ═ ║
Arrows:       ▶ ▼ ◀ ▲ ──▶ ──▼ ◀── ▲── ───▶ ◀───
Connectors:   ├── └── ┌── ──┤ ──┘ ──┐
Symbols:      ● ○ ■ □ ◆ ◇ ★ ☆ ✓ ✗
```

**Width:** Keep all diagrams under 78 characters for terminal compatibility.

**Labels:** Always label:
- Protocols (HTTP, WSS, TCP)
- Ports (`:9924`, `:5432`)
- Data formats (JSON, Binary, GeoJSON)
- Direction of data flow

---

#### Required Diagrams Checklist for 0-REPORT

| # | Category | Diagram | Purpose |
|---|----------|---------|---------|
| 1 | System | High-Level Overview | Show 3-language architecture |
| 2 | System | Deployment View | Show ports and services |
| 3 | Cross-Language | Rust↔Python FFI | Document PyO3 boundary |
| 4 | Cross-Language | Python↔JavaScript | Document WebSocket/HTTP |
| 5 | Cross-Language | Rust↔Database | Document SQL interaction |
| 6 | Data Flow | Ingestion Pipeline | AIS source to database |
| 7 | Data Flow | Query→Visualization | User query to map display |
| 8 | Sequence | Track Query | Message sequence for queries |
| 9 | Sequence | File Decode | decode_msgs() execution |
| 10 | State | WebSocket States | Connection lifecycle |
| 11 | State | Track Processing | Pipeline stages |
| 12 | Component | Rust Crates | Dependency tree |
| 13 | Component | Python Modules | Package hierarchy |
| 14 | Database | ER Diagram | Table relationships |
| 15 | External | Services Integration | External APIs |
| 16 | User | Web Interface Flow | User interactions |

---

## Execution Protocol

### Pre-Analysis Phase

Before beginning analysis, the orchestrating agent MUST:

1. **Check for existing `0-REPORT.md`**:
   - If exists: Read the file to understand current state
   - Note the version, last update date, and section structure
   - Identify sections that may need updates vs. sections that are complete

2. **Check for existing `0-CHANGELOG.md`**:
   - If exists: Read to understand what changes have been made previously
   - Identify the last run date and version number
   - Prepare to append new changelog entries

3. **Assess repository state**:
   - Run `git status` to see if there are uncommitted changes
   - Run `git log -10` to see recent commits since last analysis
   - Compare commit hashes with changelog to determine what's new

### Analysis Phase

Launch specialized agents IN PARALLEL where dependencies allow. Each agent returns structured findings that get merged into the final report.

---

## Specialized Agents Configuration

### Agent 1: Rust Architecture Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `very thorough`

**Prompt**:
```
Analyze the Rust architecture of this AISdb-lite repository. Focus on:

1. **Crate Structure** (Cargo.toml files):
   - List all crates: root, aisdb_lib, database_server, receiver, client_webassembly
   - Document dependencies and features for each crate
   - Note version numbers and toolchain requirements (nightly vs stable)

2. **PyO3 Bindings** (src/lib.rs):
   - List ALL #[pyfunction] exports with exact signatures
   - Document BATCHSIZE constant and any other public constants
   - Note the module structure and what's exposed to Python

3. **Core Library** (aisdb_lib/):
   - csvreader.rs: Supported CSV formats, parsing functions, error handling
   - db.rs: Database abstraction, supported backends, SQL loading mechanism
   - decode.rs: VesselData struct (EXACT fields), supported AIS message types
   - util.rs: Utility functions
   - build.rs: Compile-time SQL embedding

4. **Database Server** (database_server/):
   - Query types supported (track_vectors, validrange, meta, zones, etc.)
   - GeneratorIteratorAdapter usage
   - WebSocket configuration (ports, TLS)
   - Response compression

5. **Receiver** (receiver/):
   - ReceiverArgs struct with EXACT field names
   - Network modes (TCP server, UDP server, TCP client, proxy)
   - mproxy integration
   - Buffer configuration

6. **WASM Client** (client_webassembly/):
   - Exported functions via #[wasm_bindgen]
   - Internal structs
   - Known issues (incomplete unzip, etc.)

Output structured findings with file paths and line numbers for every claim.
Flag any TODOs, incomplete implementations, or bugs discovered.
```

### Agent 2: Python Package Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `very thorough`

**Prompt**:
```
Analyze the Python package structure of aisdb/. Focus on:

1. **Package Exports** (aisdb/__init__.py):
   - List ALL exported classes with their source modules
   - List ALL exported functions with their source modules
   - Document version and any package-level constants

2. **Core Modules**:
   - gis.py: Domain class methods, coordinate functions, DomainFromTxts, DomainFromPoints
   - track_gen.py: TrackGen (function vs class?), split_timedelta, split_tracks, fence_tracks
   - interp.py: List ONLY interpolation methods that EXIST (interp_time, interp_spacing, etc.)
   - denoising_encoder.py: encode_score function, InlandDenoising class
   - proc_util.py: All 13+ utility functions
   - wsa.py: Wetted surface area calculation
   - network_graph.py: Graph building functionality
   - web_interface.py: Flask configuration

3. **Database Layer** (database/):
   - dbconn.py: PostgresDBConn class, connection parameters, all methods
   - dbqry.py: DBQuery class, required/optional keys, gen_qry method
   - decoder.py: FileChecksums class (MD5 vs SHA256?), decode_msgs function
   - sqlfcn.py: CTE builder functions
   - sqlfcn_callbacks.py: All 12 WHERE clause builders with exact names

4. **Web Data Services** (webdata/):
   - _scraper.py: Selenium configuration
   - marinetraffic.py: VesselInfo class, search functions (verify which exist)
   - bathymetry.py: Gebco class methods (verify which exist)
   - shore_dist.py: ShoreDist, PortDist, CoastDist classes (verify signatures)

5. **Weather Integration** (weather/):
   - weather_fetch.py: ClimateDataStore class
   - data_store.py: WeatherDataStore class
   - utils.py: SHORT_NAMES_TO_VARIABLES mapping count

6. **Discretization** (discretize/):
   - h3.py: Discretizer class methods

For each function/class, verify it EXISTS in the code. Note any previously documented items that don't exist.
```

### Agent 3: SQL Database Schema Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `very thorough`

**Prompt**:
```
Analyze the SQL database schema in aisdb/aisdb_sql/. Focus on:

1. **Table Definitions**:
   - Static data tables (regional and global)
   - Dynamic data tables (regional and global with PostGIS)
   - Reference tables (coarsetype_ref, webdata_marinetraffic)
   - Virtual tables (gebco_2022 R-tree)

2. **Column Details**:
   - List exact column names, types, constraints
   - Note differences between SQLite and PostgreSQL versions
   - Document PRIMARY KEY compositions
   - Note any GENERATED columns (PostGIS geometry)

3. **TimescaleDB Configuration**:
   - Hypertable settings (partitioning, chunk intervals)
   - Compression settings (enabled/disabled, orderby, segmentby)

4. **Indexes**:
   - All index definitions
   - GIST indexes for spatial queries
   - B-tree indexes for filtering

5. **SQL File Inventory**:
   - Count and categorize all .sql files
   - CREATE TABLE, INSERT, SELECT, CTE files
   - PostgreSQL vs SQLite specific files

6. **Known Issues**:
   - Check insert_webdata_marinetraffic.sql line 24 for column bug
   - Check select_join files for duplicate columns
   - Any other SQL issues discovered

Provide exact SQL snippets for all table definitions.
```

### Agent 4: Web Frontend Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `medium`

**Prompt**:
```
Analyze the web frontend in aisdb_web/. Focus on:

1. **Module Structure**:
   - List all .js and .ts files in aisdb_web/map/
   - Document the purpose of each module

2. **OpenLayers Configuration** (map.js):
   - Vector layers (how many, what types)
   - Default map center and zoom
   - Tile layer configuration

3. **WebSocket Protocol** (clientsocket.js):
   - Message types (outgoing requests, incoming responses)
   - Connection handling (reconnect logic)
   - Port configuration

4. **Live Stream** (livestream.js):
   - Real-time update mechanism
   - Port used

5. **Color Palette** (palette.js):
   - Number of named colors
   - Vessel type color mappings

6. **IndexedDB Storage** (db.ts):
   - VesselMetadata interface
   - VesselDB class methods

7. **Build Configuration** (vite.config.js):
   - Output directories
   - Server configuration
   - Environment variables

8. **URL Parameters** (url.js):
   - Supported query parameters

Document TypeScript types where present.
```

### Agent 5: Testing Architecture Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `very thorough`

**Prompt**:
```
Analyze the test suite in aisdb/tests/. Focus on:

1. **Test Suite Metrics**:
   - Count test files
   - Count test functions
   - Estimate lines of test code

2. **Test Files Inventory**:
   - List each test file with number of test functions
   - Note which ones are PostgreSQL, SQLite, or both
   - VERIFY: Are test_004_sqlfcn.py and test_005_dbqry.py actually SQLite tests?

3. **Test Data Files**:
   - List files in tests/testdata/
   - Note formats (CSV, NM4, NMEA, compressed)
   - Note file sizes

4. **Helper Functions** (create_testing_data.py):
   - sample_dynamictable_insertdata
   - sample_random_polygon
   - sample_gulfstlawrence_bbox
   - random_polygons_domain
   - sample_database_file

5. **Test Patterns**:
   - Database setup patterns
   - Query patterns
   - Track processing patterns

6. **Coverage by Area**:
   - Database connectivity tests
   - Decoding tests
   - SQL generation tests
   - Query API tests
   - GIS tests
   - Track generation tests
   - Interpolation tests
   - Web integration tests

7. **Missing Coverage**:
   - No conftest.py?
   - No parametrized tests?
   - Missing areas

8. **Environment Requirements**:
   - Required environment variables
   - Required services (PostgreSQL, etc.)
```

### Agent 6: Configuration & Build System Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `medium`

**Prompt**:
```
Analyze the build and configuration system. Focus on:

1. **pyproject.toml**:
   - Build system (maturin)
   - Project metadata (version, license, authors)
   - Dependencies
   - Maturin configuration (bindings, compatibility, includes)
   - Pytest configuration

2. **Cargo.toml files** (root and all crates):
   - Version numbers
   - Features
   - Dependencies
   - Profiles

3. **CI/CD Workflows** (.github/workflows/):
   - CI.yml: Multi-platform build matrix
   - Install.yml: Verification steps
   - API_doc_manual.yml: Documentation build

4. **Root build.rs**:
   - WASM build steps
   - NPM installation
   - Vite build process

5. **Docker Configuration**:
   - Dockerfile contents
   - Production readiness

6. **Environment Variables**:
   - Build-time variables
   - Runtime variables

7. **Package.json** (aisdb_web/):
   - Dependencies
   - Scripts
   - Version

Note any gaps or issues in the build system.
```

### Agent 7: Code Quality & Technical Debt Analyzer

**Subagent Type**: `Explore`
**Thoroughness**: `medium`

**Prompt**:
```
Analyze code quality and technical debt. Focus on:

1. **Ghost Functions**:
   - Functions defined but incomplete
   - Functions documented but don't exist
   - Dead code

2. **TODO Comments**:
   - Find all TODO, FIXME, HACK, XXX comments
   - Note file and line number for each

3. **Known Bugs**:
   - SQL bugs (wrong column references, duplicates)
   - Rust bugs (incomplete implementations)
   - Python bugs

4. **Unused Code**:
   - Unused structs
   - Unused functions
   - Unused imports

5. **Inconsistencies**:
   - Naming inconsistencies
   - API inconsistencies
   - Documentation vs implementation mismatches

6. **Security Concerns**:
   - SQL injection vectors
   - Input validation gaps
   - Credential handling

7. **Performance Issues**:
   - N+1 queries
   - Missing indexes
   - Inefficient algorithms

8. **Missing Features**:
   - TODOs that indicate planned but missing features
   - Incomplete error handling
   - Missing tests

Provide specific file:line references for all findings.
```

### Agent 8: Cross-Reference Validator

**Subagent Type**: `Explore`
**Thoroughness**: `very thorough`

**Prompt**:
```
Validate cross-references and documentation accuracy. Focus on:

1. **Function Existence Verification**:
   - For each function documented, verify it exists
   - Check exact signatures match documentation
   - Note any discrepancies

2. **Struct/Class Field Verification**:
   - VesselData struct: exact fields
   - ReceiverArgs struct: exact field names
   - PostgresDBConn: exact methods
   - Domain class: exact methods

3. **Import Verification**:
   - Verify all items in aisdb/__init__.py can be imported
   - Check for circular import issues

4. **SQL Template Verification**:
   - Count SQL files
   - Verify referenced SQL files exist

5. **Configuration Consistency**:
   - Port numbers across different files
   - Default values consistency
   - Environment variable names

6. **Version Consistency**:
   - Version in pyproject.toml
   - Version in Cargo.toml files
   - Version references in documentation

Flag ALL discrepancies found between documentation and implementation.
```

---

## Report Structure

The final `0-REPORT.md` should maintain this structure:

```markdown
# AISdb-Lite System Analysis Report

**Version**: X.X.X-alpha
**Last Updated**: YYYY-MM-DD
**Analysis Agents**: 10 specialized exploration agents
**Code Coverage**: 100%

## Revision Notes
> This section tracks corrections made from cross-report analysis

## Table of Contents
1. Executive Summary
2. System Architecture Overview
3. Project Structure
4. Technology Stack
5. Rust Crate Architecture
6. Python Package Structure
7. SQL Database Schema
8. Core Modules Deep Dive
9. Web Frontend Architecture
10. Testing Architecture
11. Configuration & Build System
12. Code Remnants & Technical Debt
13. System Diagrams
14. Complete Function Reference
15. File Reference Index

Appendix A: Version History
Appendix B: Environment Setup
Appendix C: Security Information
```

### Section Requirements

Each section MUST include:

1. **Accurate Information Only**: Every function, class, method listed must be verified to exist
2. **File Paths**: Include `/Users/gabrielspadon/Desktop/AISdb-lite/...` paths where relevant
3. **Line Numbers**: For key definitions, include line numbers
4. **Code Snippets**: Include actual code for important definitions
5. **Tables**: Use tables for function references, column listings
6. **Diagrams**: ASCII diagrams for architecture, data flow
7. **Corrections**: Note any previous documentation that was wrong

---

## Duplication Avoidance Protocol

When updating an existing report:

### 1. Content Comparison

Before writing any section:
```
1. Read existing section from 0-REPORT.md
2. Compare with new findings
3. If identical: Skip (note in changelog as "Verified - no changes")
4. If different: Update (note specific changes in changelog)
5. If missing: Add (note as "New section added" in changelog)
```

### 2. Change Types

Classify all changes as:
- **CORRECTION**: Previous documentation was incorrect
- **ADDITION**: New content not previously documented
- **UPDATE**: Content changed due to code changes
- **REMOVAL**: Content removed because code no longer exists
- **VERIFICATION**: Content verified as still accurate

### 3. Merge Strategy

When merging agent findings:
```
1. Take the most detailed accurate information
2. Prefer code verification over documentation claims
3. When conflicts exist, verify against actual source code
4. Note confidence level for unverifiable claims
```

---

## Changelog Format

The `0-CHANGELOG.md` file should follow this format:

```markdown
# 0-REPORT.md Changelog

## [Run YYYY-MM-DD HH:MM] - Version X.X.X

### Summary
Brief description of what changed in this run.

### Corrections Made
- [CORRECTION] Section 5.5: ReceiverArgs field names corrected
- [CORRECTION] Section 6.2: TrackGen is a function, not a class

### Additions
- [ADDITION] Section 12.4: New bug discovered in SQL
- [ADDITION] Section 14: Added missing function documentation

### Updates
- [UPDATE] Section 4.1: Version updated from 1.7.0 to 1.8.0
- [UPDATE] Section 7.1: New PostGIS column documented

### Verifications
- [VERIFIED] Section 1-4: No changes needed
- [VERIFIED] Section 8: All interpolation methods verified

### Removals
- [REMOVAL] Section 6.4: Removed non-existent marinetraffic_metadict()
- [REMOVAL] Section 8.2: Removed interp_heading(), interp_utm()

### Agents Used
1. Rust Architecture Analyzer
2. Python Package Analyzer
3. SQL Database Schema Analyzer
4. Web Frontend Analyzer
5. Testing Architecture Analyzer
6. Configuration & Build System Analyzer
7. Code Quality & Technical Debt Analyzer
8. Cross-Reference Validator

### Git State
- Branch: main
- Last Commit: <hash> - <message>
- Uncommitted Changes: Yes/No

---

## [Previous Run YYYY-MM-DD HH:MM] - Version X.X.X
...
```

---

## Execution Command

To run this analysis, use the following approach:

```
1. Read 0-REPORT.md if it exists
2. Read 0-CHANGELOG.md if it exists
3. Launch agents 1-6 in parallel (independent analyses)
4. Wait for results
5. Launch agents 7-8 (depend on earlier findings)
6. Merge all findings
7. Compare with existing report
8. Generate diff/changes
9. Update 0-REPORT.md with merged content
10. Append to 0-CHANGELOG.md with this run's changes
```

---

## Quality Checklist

Before finalizing the report, verify:

- [ ] All function signatures verified against actual code
- [ ] All file paths are valid
- [ ] No ghost functions documented
- [ ] All corrections from previous runs preserved
- [ ] Version numbers consistent
- [ ] All diagrams render correctly
- [ ] No duplicate sections
- [ ] Changelog updated with all changes
- [ ] Git state captured in changelog

---

## Notes for Human Operators

1. **First Run**: If no 0-REPORT.md exists, generate complete report from scratch
2. **Subsequent Runs**: Focus on changes since last run, preserve verified content
3. **Manual Corrections**: Any manual edits to 0-REPORT.md should be noted in changelog
4. **Partial Runs**: Can run individual agents if only specific sections need updating
5. **Confidence Levels**: Mark unverified claims with "(unverified)" suffix

---

*This prompt system is designed for use with Claude Code's multi-agent Task tool.*
*Each agent uses subagent_type='Explore' with appropriate thoroughness levels.*
