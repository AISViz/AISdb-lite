# 0-REPORT.md Changelog

This file tracks all changes made to `0-REPORT.md` across successive analysis runs.

---

## [Run 2025-12-11 Multi-Agent Re-Analysis] - Version 1.8.0-alpha

### Summary
Comprehensive re-analysis using 8 specialized exploration agents. Verified existing documentation, added new bug discoveries, and corrected ReceiverArgs struct field documentation.

### Corrections Applied

#### Section 5.5: ReceiverArgs Struct (lines 705-725)
- [CORRECTION] ReceiverArgs struct fields updated with exact 12 fields from `receiver/src/receiver.rs:85-99`
- [CORRECTION] Added missing fields: `udp_output_addr`, `tee`
- [CORRECTION] Corrected field name: `postgres_connection_string` (was `postgres_connect_string` in some docs)
- [CORRECTION] Removed non-existent fields: `multicast_rebroadcast_rawdata`, `multicast_rebroadcast_parsed`

### Additions

#### Section 12.4: Known Bugs (lines 2347-2400)
- [BUG DOCUMENTED] Bug #4 expanded: CSV parser early return also exists at line 558 (postgres variant)
- [BUG DOCUMENTED] Bug #7 expanded: Version mismatches in `client_webassembly/Cargo.toml` (1.7.0) and `aisdb_web/package.json` (1.7.0)
- [BUG DOCUMENTED] Bug #9 added: Coordinate array swap in `aisdb/webdata/load_raster.py:61` - latitude lookup uses longitude array
- [BUG DOCUMENTED] Bug #10 added: Uninitialized `tracer` variable in `aisdb/webdata/bathymetry.py:81-92` causes crash in non-DEBUG mode
- [BUG DOCUMENTED] Bug #11 added: Missing `toml` dependency in `pyproject.toml` - imported in `aisdb/__init__.py:5` but not declared

### Verifications
- [VERIFIED] All 35+ exported functions in `aisdb/__init__.py` exist with correct signatures
- [VERIFIED] VesselData struct contains exactly 2 fields: `payload` and `epoch`
- [VERIFIED] All 30 SQL template files exist and are correctly referenced
- [VERIFIED] Port numbers consistent across codebase (9920, 9921, 9922, 9924, 5432)
- [VERIFIED] TrackGen is a generator function (not a class)
- [VERIFIED] Only 4 interpolation methods exist (interp_time, geo_interp_time, interp_spacing, interp_cubic_spline)
- [VERIFIED] FileChecksums uses MD5 algorithm
- [VERIFIED] Test files 004 and 005 are PostgreSQL-only (not SQLite)

### Agents Used
1. Rust Architecture Analyzer - Crate structure, PyO3 bindings, VesselData struct
2. Python Package Analyzer - Module exports, function verification
3. SQL Database Schema Analyzer - 30 SQL files, table definitions, bugs
4. Web Frontend Analyzer - OpenLayers, WebSocket protocol, IndexedDB
5. Test Suite Analyzer - 19 test files, 60 functions, coverage areas
6. Build System Analyzer - Maturin, Cargo, Vite, version inconsistencies
7. Code Quality Analyzer - Bugs, security concerns, technical debt
8. Cross-Reference Validator - Function existence, import verification

### Git State
- Branch: audit
- Last Commit: f1c610e - Fix the pipeline
- Uncommitted Changes: Yes (audit reports)

---

## [Run 2025-12-11 Post-3-REPORT] - Version 1.8.0-alpha

### Summary
Corrections applied based on 3-REPORT.md cross-report contradiction analysis.

### Corrections Applied
- [CORRECTED] Section 14 (Function Reference): TrackGen type changed from "Class" to "Function" in exports table (line 2506) - CONTRA-FN-001

### Git State
- Branch: main
- Last Commit: f1c610e - Fix the pipeline

---

## [Run 2025-12-11 Initial] - Version 1.8.0-alpha

### Summary
Initial changelog creation. The existing 0-REPORT.md was created through multiple analysis runs that identified and corrected numerous documentation errors. This changelog will track all future changes.

### Historical Corrections (Pre-Changelog)

The following corrections were made in previous analysis sessions before this changelog was established:

#### Section 5: Rust Crate Architecture
- [CORRECTION] ReceiverArgs struct: Field names like `db_path`, `source`, `tls_cert`, `tls_key` were documented but DO NOT EXIST. Actual fields: `udp_listen_addr`, `tcp_listen_addr`, `tcp_connect_addr`, `multicast_addr_rawdata`, `multicast_addr_parsed`, etc.
- [CORRECTION] VesselData struct: Simplified to only contain `payload: Option<ParsedMessage>` and `epoch: Option<i32>`. Previous documentation incorrectly listed many individual fields.

#### Section 6: Python Package Structure
- [CORRECTION] TrackGen: Documented as class but is actually a generator FUNCTION
- [CORRECTION] `interp_heading()` and `interp_utm()`: Documented but DO NOT EXIST. Only 4 interpolation methods exist.
- [CORRECTION] Gebco class: `get_depth()` and `get_depths()` methods DO NOT EXIST. Only `merge_tracks()` is implemented.
- [CORRECTION] ShoreDist.get_distance(): Signature is `get_distance(tracks)` not `get_distance(lon, lat)`
- [CORRECTION] FileChecksums: Uses MD5, not SHA256 as previously documented
- [CORRECTION] `marinetraffic_metadict()`: Function DOES NOT EXIST

#### Section 7: SQL Database Schema
- [BUG DOCUMENTED] insert_webdata_marinetraffic.sql line 24: `summer_dwt = excluded.gross_tonnage` should be `summer_dwt = excluded.summer_dwt`
- [BUG DOCUMENTED] select_join_dynamic_static_clusteredidx.sql: Contains duplicate utc_second column

#### Section 10: Testing Architecture
- [CORRECTION] test_004_sqlfcn.py and test_005_dbqry.py: Previously documented as "SQLite tests" but ALL tests are PostgreSQL-only. These are for different PostgreSQL configurations (monthly tables vs global hypertables).

### Agents Used (Historical)
The report was created using 10 specialized exploration agents with cross-report contradiction analysis.

### Git State at Changelog Creation
- Branch: main
- Last Commit: f1c610e - Fix the pipeline

---

## Changelog Format Reference

Future entries should follow this format:

```markdown
## [Run YYYY-MM-DD HH:MM] - Version X.X.X

### Summary
Brief description of changes.

### Corrections Made
- [CORRECTION] Section X.X: Description of what was wrong and what's correct

### Additions
- [ADDITION] Section X.X: New content added

### Updates
- [UPDATE] Section X.X: What changed and why

### Verifications
- [VERIFIED] Section X-Y: Content confirmed accurate

### Removals
- [REMOVAL] Section X.X: What was removed and why

### Agents Used
List of agents executed

### Git State
- Branch: <name>
- Last Commit: <hash> - <message>
- Uncommitted Changes: Yes/No
```

---

## Change Classification Guide

| Type | Symbol | Description |
|------|--------|-------------|
| CORRECTION | [CORRECTION] | Previous documentation was incorrect |
| ADDITION | [ADDITION] | New content not previously documented |
| UPDATE | [UPDATE] | Content changed due to code changes |
| REMOVAL | [REMOVAL] | Content removed because code no longer exists |
| VERIFIED | [VERIFIED] | Content verified as still accurate |
| BUG DOCUMENTED | [BUG DOCUMENTED] | Code bug discovered and documented |

---

## Analysis Run Statistics

| Run Date | Version | Corrections | Additions | Updates | Verifications |
|----------|---------|-------------|-----------|---------|---------------|
| 2025-12-11 (Re-Analysis) | 1.8.0-alpha | 4 | 5 | 0 | 8 |
| 2025-12-11 (Post-3-REPORT) | 1.8.0-alpha | 1 | 0 | 0 | 0 |
| 2025-12-11 (Initial) | 1.8.0-alpha | 10+ | - | - | - |

---

*This changelog is automatically maintained by the multi-agent analysis system.*
*See `0-ANALYSIS-PROMPT.md` for the analysis prompt configuration.*
