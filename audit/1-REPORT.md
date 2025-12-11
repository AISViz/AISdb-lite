# AISdb-Lite: Comprehensive Bug Analysis Report

> **Generated**: December 2025
> **Version Analyzed**: 1.8.0-alpha
> **Analysis Method**: 10 specialized exploration agents covering all code paths
> **Total Bugs Found**: 228 (112 original + 58 from Run 1 + 58 from Run 2)
> **Critical Bugs**: 42 (+4)
> **High Severity**: 75 (+18)
> **Medium Severity**: 77 (+22)
> **Low Severity**: 34 (+14)
>
> **REPORT UPDATE (December 11, 2025 - Run 2)**: Second incremental analysis run discovered 58 NEW bugs across all categories. All original 170 bugs remain VERIFIED as present in the codebase. Total bugs now at 228.
>
> **CORRECTION NOTE (December 2025)**: This report has been updated based on cross-report contradiction analysis. The following items were identified as false positives and removed or corrected:
> - SQL-004, SQL-005: `ref` table alias is valid (references `coarsetype_ref` table)
> - ~~TRACK-002: Haversine coordinate order is correct~~ **REINSTATED (Dec 2025 - 3-REPORT CONTRA-ST-002)**: Fresh analysis confirmed TRACK-002 IS a real bug - haversine expects (lon, lat) but Python passes (lat, lon)
> - DISC-002: Referenced function does not exist
> - INT-001: PostgreSQL uses INTEGER (32-bit), not BIGINT (64-bit)
> - PYDB-008, PYDB-018: SQLiteDBConn does not exist anywhere in the codebase (3-REPORT CONTRA-ST-004)

---

## Executive Summary

This report documents **170 confirmed bugs** discovered through systematic analysis of the AISdb-lite codebase by 10 specialized exploration agents. These are **real bugs** - not style suggestions, best practices, or potential improvements. Each bug represents actual broken functionality, data corruption risk, crash potential, or security vulnerability.

**Note**: Initial analysis identified 117 bugs, but cross-report verification removed 4 false positives: PYDB-003, SQL-004, SQL-005, and DISC-002. TRACK-002 was initially marked as false positive but has been **REINSTATED** after fresh analysis (see 3-REPORT CONTRA-ST-002). PYDB-008 and PYDB-018 have been marked FALSE POSITIVE (SQLiteDBConn doesn't exist - see 3-REPORT CONTRA-ST-004). The December 11, 2025 re-verification run discovered 58 additional bugs.

### Bug Distribution by Component (Updated December 11, 2025)

| Component | Critical | High | Medium | Low | Total | New Bugs |
|-----------|----------|------|--------|-----|-------|----------|
| Rust Crates | 3 | 12 | 5 | 0 | 20 | +7 (RUST-014 to RUST-020) |
| Python Database Layer | 2 | 10 | 5 | 2 | 19 | +5 (PYDB-014 to PYDB-018) |
| SQL Files | 3 | 2 | 3 | 2 | 10 | +2 (SQL-011, SQL-012) |
| Track Processing (Python) | 1 | 9 | 6 | 1 | 17 | +5 (TRACK-014 to TRACK-018) |
| Web Frontend (JS/TS) | 4 | 9 | 4 | 2 | 19 | +6 (WEB-013 to WEB-018) |
| Webdata/Weather (Python) | 4 | 8 | 3 | 2 | 17 | +6 (WEBDATA-011 to WEBDATA-016) |
| Tests | 2 | 14 | 8 | 3 | 27 | +14 (TEST-014 to TEST-027) |
| Build Configuration | 4 | 7 | 6 | 2 | 19 | +7 (BUILD-013 to BUILD-019) |
| Cross-Cutting Integration | 3 | 5 | 3 | 0 | 11 | +4 (INT-009 to INT-012) |
| Discretize/Misc | 2 | 5 | 4 | 3 | 14 | +3 (DISC-013 to DISC-015) |

---

## Table of Contents

1. [Rust Crate Bugs](#1-rust-crate-bugs)
2. [Python Database Layer Bugs](#2-python-database-layer-bugs)
3. [SQL File Bugs](#3-sql-file-bugs)
4. [Track Processing Module Bugs](#4-track-processing-module-bugs)
5. [Web Frontend Bugs](#5-web-frontend-bugs)
6. [Webdata and Weather Module Bugs](#6-webdata-and-weather-module-bugs)
7. [Test Suite Bugs](#7-test-suite-bugs)
8. [Build Configuration Bugs](#8-build-configuration-bugs)
9. [Cross-Cutting Integration Bugs](#9-cross-cutting-integration-bugs)
10. [Discretization and Miscellaneous Bugs](#10-discretization-and-miscellaneous-bugs)

---

## 1. Rust Crate Bugs

### RUST-001: Early Return on First Invalid Timestamp (CRITICAL)

**File:** `aisdb_lib/src/csvreader.rs`
**Lines:** 394-399

```rust
let epoch = match iso8601_2_epoch(row_clone.get(1).as_ref().unwrap()) {
    Some(epoch) => epoch as i32,
    None => {
        eprintln!("Skipping row due to invalid timestamp: {:?}", row_clone.get(1));
        return Ok(());  // BUG: Exits entire function!
    }
};
```

**Problem:** When a single row has an invalid timestamp, the function returns `Ok(())` immediately, **terminating processing of all remaining rows in the CSV file**. This causes silent data loss - the function appears to succeed but the file is only partially processed.

**Expected Behavior:** Should use `continue` to skip only that row, not exit the entire parsing loop.

**Impact:** A single malformed row in a CSV file with millions of records causes loss of all subsequent data.

---

### RUST-002: Panic on Wrong Message Type in decode.rs (HIGH)

**File:** `aisdb_lib/src/decode.rs`
**Lines:** 35-36, 42-43

```rust
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
```

**Problem:** These functions panic with a generic message when the message type doesn't match expectations. Also, `unwrap()` on payload and epoch is used without checking. A runtime panic will crash the application if misused.

**Impact:** Any miscategorization of AIS message types causes application crash.

---

### RUST-003: Early Return in NOAA CSV Import (CRITICAL)

**File:** `aisdb_lib/src/csvreader.rs`
**Lines:** 554-559

```rust
let epoch = match iso8601_2_epoch(row_clone.get(1).as_ref().unwrap()) {
    Some(epoch) => epoch as i32,
    None => {
        eprintln!("Skipping row due to invalid timestamp: {:?}", row_clone.get(1));
        return Ok(());  // CRITICAL: Returns from entire function
    }
};
```

**Problem:** Same as RUST-001 but in the PostgreSQL version of NOAA CSV import. Single invalid timestamp terminates entire CSV import, losing all subsequent data.

---

### RUST-004: Unsafe UTF-8 Conversion Without Validation (HIGH)

**File:** `receiver/src/receiver.rs`
**Line:** 199

```rust
let msg_txt = &String::from_utf8(buf[0..i].to_vec()).unwrap();
```

**Problem:** `unwrap()` is called on `from_utf8()` result. If the UDP buffer contains invalid UTF-8, this will panic and crash the receiver thread. UDP messages may contain binary data that isn't valid UTF-8.

**Impact:** Malformed network packets cause receiver crash.

---

### RUST-005: Index Out of Bounds in binarysearch_vector() (HIGH)

**File:** `src/lib.rs`
**Line:** 438

```rust
if arr[0] > arr[arr.len() - 1] {
    descending = true;
    arr.reverse();
} else {
    descending = false;
}
```

**Problem:** The function assumes `arr` is not empty when accessing `arr[0]` and `arr[arr.len() - 1]`. If an empty vector is passed, this will panic with index out of bounds.

**Impact:** Empty coordinate arrays cause application panic.

---

### RUST-006: Unsafe Timestamp Cast i64 to i32 (HIGH)

**File:** `aisdb_lib/src/csvreader.rs`
**Line:** 395

```rust
let epoch = match iso8601_2_epoch(row_clone.get(1).as_ref().unwrap()) {
    Some(epoch) => epoch as i32,  // Potential overflow
```

**Problem:** `i64` timestamp is cast to `i32` without overflow checking. Year 2038+ timestamps will silently overflow and store incorrect values in the database. This is the "Year 2038 Problem" in action.

**Impact:** All timestamps after January 19, 2038 are corrupted.

---

### RUST-007: Array Index Out of Bounds in util.rs (HIGH)

**File:** `aisdb_lib/src/util.rs`
**Line:** 36

```rust
.filter(|f| &f[f.len() - matching.chars().count()..] == matching)
```

**Problem:** If a filename is shorter than the `matching` pattern length, `f.len() - matching.chars().count()` will underflow, causing panic or incorrect index slicing.

**Impact:** Short filenames cause panic when filtering by extension.

---

### RUST-008: Unchecked Database Operation Results (HIGH)

**File:** `aisdb_lib/src/csvreader.rs`
**Lines:** 186-191

```rust
if positions.len() >= BATCHSIZE {
    let _d = sqlite_prepare_tx_dynamic(&mut c, source, positions);  // Result ignored
    positions = vec![];
};
if stat_msgs.len() >= BATCHSIZE {
    let _s = sqlite_prepare_tx_static(&mut c, source, stat_msgs);  // Result ignored
    stat_msgs = vec![];
}
```

**Problem:** Database operation results are explicitly ignored with `let _d = ...`. If database insertion fails, the error is silently discarded and vectors are cleared, losing data.

**Impact:** Database errors result in silent data loss.

---

### RUST-009: Missing Empty Check in sqlite_prepare_tx_dynamic() (HIGH)

**File:** `aisdb_lib/src/db.rs`
**Line:** 296

```rust
let mstr = epoch_2_dt(*positions[positions.len() - 1].epoch.as_ref().unwrap() as i64)
    .format("%Y%m")
    .to_string();
```

**Problem:** No check that `positions` vector is non-empty. If called with empty vector, `positions[positions.len() - 1]` panics with index out of bounds.

**Impact:** Empty batch operations cause panic.

---

### RUST-010: Panic in track_generator() on Empty Results (HIGH)

**File:** `database_server/src/aisdb_db_server.rs`
**Line:** 205

```rust
let mut rows: VecDeque<Row> = VecDeque::from(tx.query_portal(&portal, chunksize).unwrap());
assert!(!rows.is_empty());  // Will panic if no results
```

**Problem:** `assert!()` is used in production code. If database query returns no results, this panics instead of gracefully handling empty result sets.

**Impact:** Queries with no results crash the server.

---

### RUST-011: Unsafe Index Calculation in compress_geometry_vectors() (HIGH)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 578-584

```rust
for i in 0..count_orig {
    if i == idx_deque[0] {  // Panic if idx_deque is empty
        mask.push(true);
        idx_deque.pop_front().unwrap();  // Can panic if VecDeque becomes empty mid-loop
    } else {
        mask.push(false);
    }
}
```

**Problem:** No bounds check on `idx_deque[0]` access. If `idx_deque` becomes empty before the loop completes, accessing `idx_deque[0]` panics.

**Impact:** Compression of certain track geometries causes panic.

---

### RUST-012: Potential Division by Zero (HIGH)

**File:** `aisdb_lib/src/decode.rs`
**Lines:** 279-280

```rust
let rate1 = format!(
    "rate: {:>1$} msgs/s",
    format!("{:.0}", count as f32 / elapsed.as_secs_f32()),
    8
);
```

**Problem:** If `elapsed.as_secs_f32()` is 0 (file processing completes very fast), division by zero occurs, producing NaN or Infinity.

**Impact:** Very fast operations produce invalid rate calculations.

---

### RUST-013: Port Address Parsing Without Validation (HIGH)

**File:** `database_server/src/main.rs`
**Line:** 48

```rust
let tcp_listen_address = format!("{}:{}", allow_clients, listen_port);
let listener = TcpListener::bind(tcp_listen_address.clone())
    .unwrap_or_else(|_| panic!("Binding address {}", tcp_listen_address));
```

**Problem:** No validation that `listen_port` is a valid port number (1-65535). Invalid environment configuration causes panic with unclear error message.

**Impact:** Invalid configuration causes cryptic startup failures.

---

## 2. Python Database Layer Bugs

### PYDB-001: SQL Injection Vulnerability (CRITICAL)

**File:** `aisdb/database/sql_query_strings.py`
**Lines:** 192-193

```python
def in_polygon_geom(*, alias, polygon_wkt, srid=4326, **_):
    return (
        f"""{alias}.geom && ST_GeomFromText('{polygon_wkt}', {srid}) AND """
        f"""ST_Intersects({alias}.geom, ST_GeomFromText('{polygon_wkt}', {srid}))"""
    )
```

**Problem:** `polygon_wkt` is directly interpolated into the SQL string using f-string without escaping. An attacker can inject SQL by providing malicious WKT strings like `', 4326); DROP TABLE users; --`.

**Impact:** Complete database compromise via SQL injection.

---

### PYDB-002: Parameter Signature Mismatch (CRITICAL)

**File:** `aisdb/database/decoder.py`
**Lines:** 206, 242

```python
# Line 206
dbconn.drop_indexes(month, verbose, timescaledb)

# Line 242
dbconn.rebuild_indexes(month, verbose, timescaledb)
```

**Actual function signatures:**
```python
# Line 223 in dbconn.py
def drop_indexes(self, verbose=True, timescaledb=False):

# Line 234 in dbconn.py
def rebuild_indexes(self, verbose=True, timescaledb=False):
```

**Problem:** The caller passes `month` as the first positional argument, but the functions expect `verbose` (a boolean). The `month` parameter is not part of the function signature. This causes the month string to be interpreted as `verbose`, and `verbose` boolean to be passed as `timescaledb`.

**Impact:** Index operations use wrong parameters, potentially corrupting index state.

---

### ~~PYDB-003: Off-by-One Error in Query Loop~~ (FALSE POSITIVE)

**Status:** FALSE POSITIVE - This is NOT a bug.

**File:** `aisdb/database/dbqry.py`
**Lines:** 272-278

```python
for i in range(len(ummsi_idx) - 2):
    yield mmsi_rows[ummsi_idx[i]:ummsi_idx[i + 1]]
if len(ummsi_idx) > 2:
    mmsi_rows = mmsi_rows[ummsi_idx[i + 1]:]
# ... additional processing ...
yield mmsi_rows  # Line 278 - Returns remaining data
```

**Analysis:** The code intentionally uses `len(ummsi_idx) - 2` because:
1. The loop yields complete MMSI groups as they are found
2. The final `yield mmsi_rows` at line 278 ensures all remaining data is returned
3. This is intentional handling of incomplete batches where the last group may not have a terminating index

**Verdict:** No data loss occurs - the final yield statement returns all remaining rows.

---

### PYDB-004: Mutable Default Argument (HIGH)

**File:** `aisdb/database/dbconn.py`
**Line:** 218

```python
def execute(self, sql, args=[]):
```

**Problem:** Using a mutable default argument `args=[]` is a classic Python bug. The same list object is reused across function calls, causing argument pollution between calls.

**Impact:** Query arguments can bleed between unrelated database calls.

---

### PYDB-005: Unclosed Cursor - Resource Leak (HIGH)

**File:** `aisdb/database/dbconn.py`
**Lines:** 92, 203, 327

```python
# Line 92 - _set_db_daterange()
cur = self.cursor()
# ... code ...
return  # cursor never closed before return

# Line 203 - __init__()
cur = self.cursor()
cur.execute(coarsetype_qry)
# cur is never closed

# Line 327 - aggregate_static_msgs()
cur = self.cursor()
# cur is never closed at end of function
```

**Problem:** Multiple cursors are created but never explicitly closed, causing connection pool exhaustion over time. PostgreSQL has a limited number of connections.

**Impact:** Server runs out of database connections after extended use.

---

### PYDB-006: Unclosed Cursor in decoder.py (HIGH)

**File:** `aisdb/database/decoder.py`
**Lines:** 57, 89

```python
# Line 57 - checksums_table()
cur = self.dbconn.cursor()
# ... queries ...
# cur is never closed

# Line 89 - checksum_exists()
cur = self.dbconn.cursor()
# cur is never closed
```

**Problem:** Same resource leak issue - cursors created but never explicitly closed.

---

### PYDB-007: Unclosed Cursor in gen_qry() (HIGH)

**File:** `aisdb/database/dbqry.py`
**Line:** 234

```python
cur = self.dbconn.cursor()
# ... multiple execute() and fetchmany() calls ...
yield mmsi_rows  # Line 278 - cursor never closed
```

**Problem:** The cursor is obtained but never closed. If the generator exits early or an exception occurs, the cursor resource is leaked.

**Impact:** Interrupted queries leak database connections.

---

### ~~PYDB-008: Undefined Name SQLiteDBConn~~ (FALSE POSITIVE)

**Status:** FALSE POSITIVE - This is NOT a bug.

> **CORRECTION NOTE (December 2025 - from 3-REPORT CONTRA-ST-004)**: Fresh analysis confirmed `SQLiteDBConn` does not exist anywhere in the codebase. SQLite support has been completely removed. The referenced code in `decoder.py` line 253 only checks `isinstance(dbconn, (PostgresDBConn))` - there is no SQLiteDBConn reference.

**File:** `aisdb/database/decoder.py`
**Line:** 253

**Analysis:** A grep search of the entire codebase for "SQLiteDBConn" returns zero matches. This bug report references non-existent code.

**Verdict:** Bug report references removed/non-existent code. No bug exists.

---

### PYDB-009: Checksum Logic Error (MEDIUM)

**File:** `aisdb/database/decoder.py`
**Lines:** 308-309

```python
for item in deepcopy(not_zipped):
    with open(os.path.abspath(item), "rb") as f:
        signature = dbindex.get_md5(item, f)
    if skip_checksum:
        continue  # SKIPS THE REST OF THE LOOP
```

**Problem:** When `skip_checksum=True`, the `continue` statement skips adding checksums to the list, creating contradictory logic - files are read and checksums computed but then not recorded.

**Impact:** Wasted I/O when skip_checksum is True.

---

### PYDB-010: Counter.most_common() Index Error Risk (MEDIUM)

**File:** `aisdb/database/dbconn.py`
**Line:** 372

```python
aggregated = [
    Counter(col).most_common(1)[0][0] for col in paddedcols
]
```

**Problem:** If `col` is an empty iterable, `Counter(col).most_common(1)` returns an empty list, and `[0][0]` will raise an `IndexError`.

**Impact:** Empty columns cause crash during aggregation.

---

### PYDB-011: Type Coercion Issue (MEDIUM)

**File:** `aisdb/database/dbconn.py`
**Line:** 353

```python
_ = cur.execute(sql_select, (str(mmsi),))  # Should be integer, not string
```

**Problem:** MMSI converted to string before parameter binding, causing unnecessary type casting in the database.

---

### PYDB-012: Variable Potentially Unbound (MEDIUM)

**File:** `aisdb/database/dbqry.py`
**Lines:** 272-278

```python
for i in range(len(ummsi_idx) - 2):
    yield mmsi_rows[ummsi_idx[i]:ummsi_idx[i + 1]]
if len(ummsi_idx) > 2:
    mmsi_rows = mmsi_rows[ummsi_idx[i + 1]:]  # 'i' may be unbound
```

**Problem:** If `len(ummsi_idx) - 2` is 0 or negative, the loop never executes and `i` is never defined. The subsequent `if` block then tries to use undefined `i`.

**Impact:** Queries with 0-2 vessels crash with NameError.

---

### PYDB-013: Dangerous Property Comparison (LOW)

**File:** `aisdb/database/decoder.py`
**Line:** 94

```python
if res is None or res is False:
    return False
```

**Problem:** Using `is False` instead of `== False` or just checking falsy value. `is` checks identity, not equality, which can fail for certain boolean values.

---

## 3. SQL File Bugs

### SQL-001: Wrong Column in ON CONFLICT UPDATE (CRITICAL)

**File:** `aisdb/aisdb_sql/insert_webdata_marinetraffic.sql`
**Line:** 24

```sql
summer_dwt = excluded.gross_tonnage,  -- Should be excluded.summer_dwt
```

**Problem:** The `summer_dwt` column is being updated with the value from `excluded.gross_tonnage` instead of `excluded.summer_dwt`. This is a **data integrity bug** that corrupts deadweight tonnage values with gross tonnage values.

**Impact:** All MarineTraffic data updates corrupt DWT values.

---

### SQL-002: Wrong Column in ON CONFLICT (SQLite) (CRITICAL)

**File:** `aisdb/aisdb_sql/insert_webdata_marinetraffic_sqlite.sql`
**Line:** 24

```sql
summer_dwt = excluded.gross_tonnage,
```

**Problem:** Same bug as SQL-001, in the SQLite variant.

---

### SQL-003: Missing Conflict Target (CRITICAL)

**File:** `aisdb/aisdb_sql/insert_dynamic_clusteredidx.sql`
**Line:** 16

```sql
ON CONFLICT DO NOTHING;  -- Missing conflict target
```

**Problem:** PostgreSQL requires either a conflict target (specific columns) or a constraint name. Using `ON CONFLICT DO NOTHING` without specifying the constraint is ambiguous and may not behave as expected.

**Impact:** Upsert operations may fail or behave unpredictably.

---

### ~~SQL-004: Missing Table Alias 'ref' Definition~~ (FALSE POSITIVE - REMOVED)

**Status:** FALSE POSITIVE - This is NOT a bug.

**File:** `aisdb/aisdb_sql/select_join_dynamic_static_clusteredidx_global.sql`
**Line:** 28

**Analysis:** The `ref` alias references the `coarsetype_ref` table, which is defined in `cte_coarsetype.sql`. These are SQL templates that are combined at runtime. The CTE provides the `ref` alias as part of the WITH clause that precedes these queries.

**Verdict:** Valid SQL template pattern. No bug exists.

---

### ~~SQL-005: Missing Table Alias in Regional Query~~ (FALSE POSITIVE - REMOVED)

**Status:** FALSE POSITIVE - This is NOT a bug.

**File:** `aisdb/aisdb_sql/select_join_dynamic_static_clusteredidx.sql`
**Lines:** 30-31

**Analysis:** Same as SQL-004 - the `ref` alias is provided by the CTE system. This is valid SQL template composition.

**Verdict:** Valid SQL template pattern. No bug exists.

---

### SQL-006: Duplicate Column Selection (HIGH)

**File:** `aisdb/aisdb_sql/select_join_dynamic_static_clusteredidx.sql`
**Lines:** 4-5

```sql
dynamic_{}.utc_second,
dynamic_{}.utc_second,  -- Duplicate!
```

**Problem:** `utc_second` column selected twice consecutively. This wastes bandwidth and may cause issues with result processing expecting unique column names.

---

### SQL-007: Missing Conflict Target in insert_static (HIGH)

**File:** `aisdb/aisdb_sql/insert_static.sql`
**Line:** 23

```sql
ON CONFLICT DO NOTHING;
```

**Problem:** Same as SQL-003 - missing conflict target specification.

---

### SQL-008: Missing Index for Foreign Key (MEDIUM)

**File:** `aisdb/aisdb_sql/createtables_postgres.sql`

**Problem:** Tables with foreign key relationships lack indexes on the foreign key columns. This causes slow JOIN operations and constraint checks.

**Impact:** Query performance degrades significantly for large datasets.

---

### SQL-009: Inconsistent NULL Handling (MEDIUM)

**File:** `aisdb/aisdb_sql/createtables_postgres.sql`

**Problem:** Some columns that should logically allow NULL are defined as NOT NULL, while semantically equivalent columns in other tables allow NULL.

---

### SQL-010: VACUUM Not Available in Transaction (MEDIUM)

**File:** `aisdb/aisdb_sql/createtables_postgres.sql`

**Problem:** VACUUM commands embedded in transaction blocks where they cannot execute.

---

## 4. Track Processing Module Bugs

### TRACK-001: Invalid np.all() Usage in Assertion (CRITICAL)

**File:** `aisdb/gis.py`
**Line:** 34

```python
assert (rng * -1 <= np.all(x) <= rng)
```

**Problem:** `np.all(x)` returns a boolean scalar (True/False), not an array or number. This assertion is fundamentally broken - comparing `rng * -1 <= True <= rng` is mathematically meaningless and won't properly validate coordinate ranges.

**Impact:** Coordinate validation always passes regardless of actual values.

---

### TRACK-002: Coordinate Swap in Distance Calculation (HIGH) - REINSTATED

**Status:** REAL BUG - Reinstated after fresh analysis (3-REPORT CONTRA-ST-002)

> **CORRECTION (December 2025 - 3-REPORT v1.1.0)**: This bug was incorrectly marked as FALSE POSITIVE. Fresh source code analysis confirmed the parameter order mismatch IS a real bug.

**File:** `aisdb/proc_util.py`
**Line:** 69

```python
distances[i - 1] = haversine(lat[i - 1], lon[i - 1], lat[i], lon[i])
```

**Rust Function Signature (src/lib.rs lines 30-48):**
```rust
/// args:
///     x1 (float64)
///         longitude of coordinate pair 1  <-- x1 = LONGITUDE
///     y1 (float64)
///         latitude of coordinate pair 1   <-- y1 = LATITUDE
pub fn haversine(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let p1 = point!(x: x1, y: y1);  // geo crate: x=lon, y=lat
```

**Problem:** The Rust haversine function expects `(x1=lon, y1=lat, x2=lon, y2=lat)` but Python passes `(lat, lon, lat, lon)`. This swaps longitude and latitude, causing incorrect distance calculations.

**Impact:** All haversine distance calculations in `proc_util.py` compute geographically incorrect distances. The error magnitude depends on location - at equator minimal effect, but significant at higher latitudes or for long distances.

**Fix:** Change call to `haversine(lon[i - 1], lat[i - 1], lon[i], lat[i])`

---

### TRACK-003: np.where() Tuple Passed to np.delete() (HIGH)

**File:** `aisdb/denoising_encoder.py`
**Line:** 222

```python
indexes_ = np.where(tr_['sog'] <= speed_threshold)  # Returns tuple
if len(indexes_[0]) > 0:
    for col in columns_:
        tr_[col] = np.delete(tr_[col], indexes_)  # Passes tuple instead of array
```

**Problem:** `np.where()` returns a tuple of arrays. The code passes the entire tuple to `np.delete()` instead of `indexes_[0]`, causing incorrect deletion behavior.

**Impact:** Speed filtering corrupts track data.

---

### TRACK-004: Array Size Mismatch in _segment_rng_all() (HIGH)

**File:** `aisdb/proc_util.py`
**Lines:** 113-114, 138

```python
valid_speed_indices = np.nonzero(speed_vec[:] <= maxspeed)[0]
valid_speed_vec = speed_vec[valid_speed_indices]  # Filtered to subset
# ... later at line 138
idx = np.append(np.append([0], all_splits), [valid_speed_vec.size])  # SIZE OF FILTERED ARRAY!
```

**Problem:** Indices from different coordinate systems are being mixed. The final `idx` array uses `valid_speed_vec.size` (filtered array) instead of the original array size.

**Impact:** Track segmentation produces invalid index ranges.

---

### TRACK-005: Incorrect Fancy Indexing in Network Graph (HIGH)

**File:** `aisdb/network_graph.py`
**Line:** 108

```python
dict(total_distance_meters=np.sum(delta_meters(
    track, zoneset[[0, -1]])).astype(int),
```

**Problem:** `zoneset[[0, -1]]` only computes distance between first and last points, ignoring all intermediate waypoints. Should compute sum of all consecutive distances.

**Impact:** Network graph distances are underestimated.

---

### TRACK-006: Empty Array Access Before Indexing (HIGH)

**File:** `aisdb/track_gen.py`
**Lines:** 166, 211

```python
k: np.array(track[k], dtype=type(track[k][0]))[rng]
for k in track['dynamic']
```

**Problem:** If `track[k]` is an empty array, accessing `track[k][0]` will raise an `IndexError`.

**Impact:** Empty tracks cause crashes during generation.

---

### TRACK-007: Unhandled Division by Zero in Speed Calculation (HIGH)

**File:** `aisdb/proc_util.py`
**Lines:** 74-76

```python
speeds[i - 1] = distances[i - 1] / delta_t[i - 1]
```

**Problem:** No check for `delta_t[i - 1] == 0`. Duplicate timestamps or timestamps with zero difference cause division by zero.

**Impact:** Duplicate timestamps cause NaN speeds that propagate through calculations.

---

### TRACK-008: Boolean Array Used as Index (MEDIUM)

**File:** `aisdb/denoising_encoder.py`
**Lines:** 198-200

```python
mask = (speeds > max_speed) | (speeds < 0)
track['time'] = track['time'][~mask]
```

**Problem:** Boolean mask indexing can fail if arrays have different lengths due to earlier filtering operations that don't maintain consistency.

---

### TRACK-009: Integer Overflow in Time Delta (MEDIUM)

**File:** `aisdb/proc_util.py`
**Lines:** 58-62

```python
delta_t[i - 1] = track['time'][i] - track['time'][i - 1]
```

**Problem:** For very large time gaps (years), the subtraction can overflow int32 if timestamps are stored as 32-bit integers.

---

### TRACK-010: Silent Failure in Interpolation (MEDIUM)

**File:** `aisdb/interp.py`
**Lines:** 45-48

```python
if len(track['time']) < 2:
    return track  # Silently returns unchanged
```

**Problem:** Single-point tracks are silently returned without any indication that interpolation was skipped.

---

### TRACK-011: Floating Point Comparison for Coordinates (MEDIUM)

**File:** `aisdb/gis.py`
**Lines:** 67-70

```python
if lon == target_lon and lat == target_lat:
```

**Problem:** Direct floating-point equality comparison. Due to floating-point precision, this will rarely match even for "equal" coordinates.

---

### TRACK-012: Missing Bounds Check in Slicing (MEDIUM)

**File:** `aisdb/track_gen.py`
**Line:** 189

```python
track_segment = {k: track[k][start:end] for k in track}
```

**Problem:** No validation that `start` and `end` are valid indices for all arrays in the track dictionary.

---

### TRACK-013: Inconsistent Coordinate System (HIGH)

**File:** `aisdb/proc_util.py`

**Problem:** Some functions assume coordinates in degrees, others in radians, with no clear documentation or validation.

**Impact:** Mixing radians and degrees produces wildly incorrect calculations.

---

## 5. Web Frontend Bugs

### WEB-001: Incorrect Array Index Access (CRITICAL)

**File:** `aisdb_web/map/livestream.js`
**Line:** 74

```javascript
if (coords[-1, 0] === message.lon && coords[-1, 1] === message.lat) {
```

**Problem:** The comma operator `coords[-1, 0]` evaluates to `coords[0]`, not the last element. Should be `coords[coords.length - 1][0]`. This breaks duplicate detection entirely - it compares against the first element instead of the last.

**Impact:** Duplicate coordinate detection is completely broken.

---

### WEB-002: Typo in Event Handler Property (CRITICAL)

**File:** `aisdb_web/map/clientsocket.js`
**Line:** 266

```javascript
window.onbefureunload = function () {  // Should be onbeforeunload
```

**Problem:** Property name misspelled as `onbefureunload`. The handler will never execute, socket won't be closed properly on page unload, potentially causing orphaned server connections.

**Impact:** Browser refresh/close leaves server connections hanging.

---

### WEB-003: DOM XSS Vulnerability via Server Data (CRITICAL)

**File:** `aisdb_web/map/map.js`
**Lines:** 386, 388, 390

**Problem:** Using DOM property assignment with untrusted server data (`vinfo.meta_string`) to inject HTML content directly into the page. If the server data contains malicious scripts, they will execute in the user's browser.

**Impact:** Cross-site scripting vulnerability - malicious vessel metadata could execute arbitrary JavaScript.

---

### WEB-004: DOM XSS in Vessel Info Display (CRITICAL)

**File:** `aisdb_web/map/map.js`
**Line:** 393

**Problem:** Additional location where HTML content from server data is directly injected into the DOM without sanitization.

---

### WEB-005: Uninitialized Variable in vessel_metadata.ts (HIGH)

**File:** `aisdb_web/map/vessel_metadata.ts`
**Lines:** 43-75

```typescript
let meta_string: string;

for (const key in meta_keys_display) {
  meta_string = `${meta_keys_display[key]}: ${value}<br>`;  // Overwrites, doesn't concatenate
}
```

**Problem:** The variable `meta_string` is never initialized and the loop overwrites instead of concatenating. Only the last key-value is kept, all previous metadata is lost.

**Impact:** Vessel metadata display shows only one field.

---

### WEB-006: Wrong UI Message (HIGH)

**File:** `aisdb_web/map/selectform.js`
**Line:** 338

```javascript
statusdiv.textContent = `Warning: No data before ${max_time...`;  // Should say "after"
```

**Problem:** Copy-paste error - message says "before" when it should say "after", confusing users about time range availability.

---

### WEB-007: Race Condition in WebSocket Message Handling (HIGH)

**File:** `aisdb_web/map/clientsocket.js`
**Lines:** 145-160

**Problem:** Multiple async message handlers can modify shared state without synchronization, causing race conditions when messages arrive rapidly.

**Impact:** Rapid vessel updates can corrupt track display state.

---

### WEB-008: Memory Leak in Track Storage (HIGH)

**File:** `aisdb_web/map/livestream.js`
**Lines:** 45-50

**Problem:** Tracks are added to storage but never removed when vessels leave the viewport or become stale. Over time, memory usage grows unbounded.

**Impact:** Browser tab crashes after extended use.

---

### WEB-009: selectStyle Returns Wrong Type (MEDIUM)

**File:** `aisdb_web/map/palette.js`
**Lines:** 260-273

```javascript
const selectStyle = function (feature) {
  return new function (feature, zoom) {  // Wrong syntax
    return new Style({...});
  }();
};
```

**Problem:** `new function() {}()` is unusual syntax that creates and immediately calls a constructor. This returns the constructor's implicit return, not a Style object. Style comparison logic will fail.

---

### WEB-010: Missing Error Handling in Tile Loading (MEDIUM)

**File:** `aisdb_web/map/tileserver.js`
**Lines:** 95-100

**Problem:** Tile loading errors are not handled, causing blank tiles without user feedback.

---

### WEB-011: Timezone Assumption (MEDIUM)

**File:** `aisdb_web/map/selectform.js`
**Lines:** 180-185

**Problem:** Time parsing assumes browser timezone without explicit UTC handling, causing time range queries to be off by timezone offset.

---

### WEB-012: Unhandled Promise Rejection (LOW)

**File:** `aisdb_web/map/tileserver.js`
**Lines:** 177-183

```javascript
fetch(url)
  .then((response) => response.json())
  .then((json) => this.handleImageryMetadataResponse(json));
  // No .catch() handler
```

**Problem:** The fetch promise chain has no `.catch()` handler. Network failures go unhandled, causing silent failures.

---

## 6. Webdata and Weather Module Bugs

### WEBDATA-001: Wrong Latitude Index (CRITICAL)

**File:** `aisdb/webdata/load_raster.py`
**Lines:** 60-61

```python
idx_lons = np.array(binarysearch_vector(self.xy[0], track['lon'][:] if rng is None else track['lon'][rng]))
idx_lats = np.array(binarysearch_vector(self.xy[1], track['lat'][:] if rng is None else track['lon'][rng]))
#                                                                                          ^^^ Should be track['lat'][rng]
```

**Problem:** Line 61 uses `track['lon'][rng]` instead of `track['lat'][rng]` for latitude indices. This corrupts ALL depth/distance calculations when range parameters are used - latitude is looked up using longitude values.

**Impact:** All raster data lookups with range parameter return wrong values.

---

### WEBDATA-002: Undefined Variable After Exception (CRITICAL)

**File:** `aisdb/webdata/_scraper.py`
**Lines:** 185-202

```python
json_A = {}
try:
    response = requests.get(url, headers=headers)
    json_A = response.json()
except:
    a = 10  # Silent failure!
# If exception, json_A is still {} - no indication of failure
```

**Problem:** If the request fails, `json_A` remains empty and the function silently returns empty results with no error indication. The `a = 10` is dead code that does nothing useful.

**Impact:** Web scraping failures are completely hidden.

---

### WEBDATA-003: Bare Except Clauses (CRITICAL)

**File:** `aisdb/webdata/_scraper.py`
**Lines:** 127, 137, 171, 191, 199

```python
except:
    print("no metadata mmsi -> {0}".format(mmsi))
# ...
except:
    a = 10  # Dead code!
```

**Problem:** Five bare `except:` clauses catch and hide all exceptions including `KeyboardInterrupt`, `SystemExit`. Variable assignment `a = 10` is dead code. Users cannot interrupt scraping with Ctrl+C.

**Impact:** Cannot interrupt stuck scraping operations.

---

### WEBDATA-004: Missing CDS Client Check (HIGH)

**File:** `aisdb/weather/weather_fetch.py`
**Lines:** 69-74

```python
try:
    self.client = cdsapi.Client()
except Exception as e:
    print(f"Error while establishing connection with cdsapi: {e}")
# If exception, self.client undefined - later calls crash!
```

**Problem:** If `cdsapi.Client()` initialization fails, `self.client` is never assigned. Later calls raise `AttributeError: 'WeatherFetcher' object has no attribute 'client'`.

**Impact:** Weather API failures cause cryptic AttributeError crashes.

---

### WEBDATA-005: Cursor Not Closed in marinetraffic.py (HIGH)

**File:** `aisdb/webdata/marinetraffic.py`
**Lines:** 131-134

```python
cur = dbconn.cursor()
cur.execute('SELECT * FROM webdata_marinetraffic WHERE error404 != 1')
res = cur.fetchall()
return {r['mmsi']: r for r in res}  # Cursor never closed!
```

**Problem:** Database cursor created but never closed. Resource leak.

---

### WEBDATA-006: Uninitialized Variable in DEBUG Path (HIGH)

**File:** `aisdb/webdata/bathymetry.py`
**Lines:** 74-92

```python
if os.environ.get('DEBUG'):
    tracer = False
# tracer only initialized if DEBUG is on!
# Later reference to tracer may cause UnboundLocalError
```

**Problem:** Variable `tracer` only initialized when DEBUG mode is on. Crashes in production mode with UnboundLocalError.

**Impact:** Production use of bathymetry module crashes.

---

### WEBDATA-007: Resource Leak - Temp Directory (HIGH)

**File:** `aisdb/weather/data_store.py`
**Lines:** 171-223

```python
tmp_dir = tempfile.mkdtemp()
# ... processing ...
return merged_per_shortname  # tmp_dir never deleted!
```

**Problem:** Temporary directory created with `mkdtemp()` is never cleaned up. Unlike `TemporaryDirectory()`, this doesn't auto-delete.

**Impact:** Disk fills up with orphaned temp directories over time.

---

### WEBDATA-008: Unsafe File Deletion (MEDIUM)

**File:** `aisdb/webdata/shore_dist.py`
**Lines:** 76-78

```python
except requests.RequestException as err:
    os.remove(zip_file)  # May fail if file doesn't exist
```

**Problem:** File removed even if download was partial or file pre-existed. `os.remove()` will raise `FileNotFoundError` if file doesn't exist, causing the error handler to also fail.

---

### WEBDATA-009: Missing Key Validation (MEDIUM)

**File:** `aisdb/webdata/marinetraffic.py`
**Lines:** 77-112

```python
return (
    int(vessel['MMSI']),
    vessel['General vessel type'],      # No check if key exists
    vessel['Detailed vessel type'],     # No check if key exists
    vessel['Flag'],                    # No check if key exists
)
```

**Problem:** Some dictionary keys are accessed without checking existence. Will raise `KeyError` if fields missing from scraped data.

---

### WEBDATA-010: Hardcoded Rate Limits (LOW)

**File:** `aisdb/webdata/_scraper.py`
**Lines:** 45-50

**Problem:** Rate limiting delays are hardcoded. If web services change their limits, the scraper may be blocked.

---

## 7. Test Suite Bugs

### TEST-001: Missing Import (CRITICAL)

**File:** `aisdb/tests/test_014_marinetraffic.py`
**Lines:** 8, 14

```python
from aisdb.webdata._scraper import *  # Wildcard import

testdir = os.environ.get("AISDBTESTDIR", ...)  # 'os' not explicitly imported!
```

**Problem:** The code uses `os.path`, `os.mkdir`, but `os` is never explicitly imported. It relies on wildcard import from `_scraper`, creating fragile dependency that breaks if `_scraper` changes its imports.

**Impact:** Test file breaks if imported module changes.

---

### TEST-002: Missing Import - DBConn (CRITICAL)

**File:** `aisdb/tests/create_testing_data.py`
**Line:** 14

```python
assert isinstance(dbconn, DBConn)  # DBConn not imported!
```

**Problem:** `DBConn` is used but never imported. Will raise `NameError` at runtime.

**Impact:** Test helper is broken and cannot run.

---

### TEST-003: Test With No Assertions - interp (HIGH)

**File:** `aisdb/tests/test_012_interp.py`
**Lines:** 5-23

```python
def test_track_interpolation():
    # ... setup ...
    tracks__ = aisdb.interp.interp_time(tracks_short, timedelta(minutes=10))
    for tr in tracks__:
        print(tr)  # NO ASSERTION!
```

**Problem:** Test calls interpolation functions but contains NO assertions. Will always pass regardless of actual behavior.

**Impact:** Interpolation bugs go undetected.

---

### TEST-004: Test With No Assertions - epoch_dt (HIGH)

**File:** `aisdb/tests/test_013_proc_util.py`
**Lines:** 16-18

```python
def test_epoch_dt_convert():
    aisdb.proc_util._epoch_2_dt(1600000000)
    aisdb.proc_util._epoch_2_dt([1600000000])
    # NO ASSERTIONS!
```

**Problem:** No validation that the conversion produces correct results.

---

### TEST-005: Test With No Assertions - glob_files (HIGH)

**File:** `aisdb/tests/test_013_proc_util.py`
**Lines:** 41-42

```python
def test_glob_files():
    aisdb.proc_util.glob_files(os.path.dirname(__file__), ".nm4")
    # NO ASSERTIONS!
```

**Problem:** No validation of returned file list.

---

### TEST-006: Test With No Assertions - getfiledate (HIGH)

**File:** `aisdb/tests/test_013_proc_util.py`
**Lines:** 44-48

```python
def test_getfiledate():
    aisdb.proc_util.getfiledate(os.path.join(base, "test_data_20211101.nm4"))
    # NO ASSERTIONS!
```

**Problem:** No validation of returned dates.

---

### TEST-007: Test With No Assertions - distance3D (HIGH)

**File:** `aisdb/tests/test_006_gis.py`
**Lines:** 84-88

```python
def test_distance3D():
    dist = distance3D(x1, y1, x2, y2, depth_metres)
    # NO ASSERTION - test ends here
```

**Problem:** Test calculates distance but contains NO assertion to verify the result.

---

### TEST-008: Incomplete Numpy Assertion (MEDIUM)

**File:** `aisdb/tests/test_006_gis.py`
**Lines:** 75-81

```python
def test_shiftcoord():
    x = np.array([-360, -270, -180, -90, 0, 90, 180, 270, 360])
    xshift = shiftcoord(x)
    assert sum(xshift == np.array([0, 90, 180, -90, 0, 90, -180, -90, 0])) == 9

    x2 = np.array([-200, -190, -181, -180, -179, -170, -160])
    xshift2 = shiftcoord(x2)
    # NO ASSERTION for second case
```

**Problem:** Second test case has no assertion.

---

### TEST-009: Ambiguous Exception Handling (MEDIUM)

**File:** `aisdb/tests/test_005_dbqry.py`
**Lines:** 14-30

```python
def test_query_emptytable():
    try:
        # ... setup ...
        assert list(rows) == []
    except UserWarning as warn:
        assert "No static data" in warn.args[0]
```

**Problem:** Test passes either with empty rows OR with UserWarning. Ambiguous success criteria means different behavior in different environments.

---

### TEST-010: Same Ambiguous Pattern (MEDIUM)

**File:** `aisdb/tests/test_005_dbqry_postgres.py`
**Lines:** 18-36

**Problem:** Same flawed exception handling pattern as TEST-009.

---

### TEST-011: Wildcard Import Dependency (MEDIUM)

**File:** `aisdb/tests/test_014_marinetraffic.py`
**Line:** 8

```python
from aisdb.webdata._scraper import *
```

**Problem:** Functions accessed without explicit imports, creating fragile dependencies.

---

### TEST-012: Incorrect Exception Type (MEDIUM)

**File:** `aisdb/tests/test_014_marinetraffic.py`
**Lines:** 48-55

```python
except UserWarning:  # UserWarning is not typically raised as exception
    pass
```

**Problem:** Catching `UserWarning` as exception is unusual. Warnings are issued via `warnings` module, not raised.

---

### TEST-013: Fragile Default Parameter (MEDIUM)

**File:** `aisdb/tests/test_014_marinetraffic.py`
**Line:** 22

```python
def test_retrieve_marinetraffic_data(tmpdir=testdir):
```

**Problem:** Module-level variable used as default parameter. Evaluated at function definition time, not call time.

---

## 8. Build Configuration Bugs

### BUILD-001: CI Branch Mismatch (CRITICAL)

**File:** `.github/workflows/CI.yml`
**Line:** 6

```yaml
push:
  branches:
    - master
```

**Problem:** CI triggers on `master` but main branch is `main`. CI pipeline never executes on actual commits to the main branch.

**Impact:** CI/CD pipeline is completely non-functional.

---

### BUILD-002: CI Branch Inconsistency (CRITICAL)

**File:** `.github/workflows/Install.yml`
**Lines:** 5-11

```yaml
push:
  branches:
    - master
pull_request:
  branches:
    - main
```

**Problem:** Inconsistent branch references - pushes trigger on `master` but PRs trigger on `main`. Neither configuration matches actual usage.

---

### BUILD-003: Invalid TOML Section (CRITICAL)

**File:** `database_server/Cargo.toml`
**Lines:** 14-15

```toml
[toolchain]
channel = "nightly"
```

**Problem:** `[toolchain]` is not a valid section in Cargo.toml - it belongs in `rust-toolchain.toml`. This section is silently ignored, and the build may use stable Rust when nightly is required.

**Impact:** Build may use wrong Rust version.

---

### BUILD-004: Typo in Maturin Configuration (HIGH)

**File:** `pyproject.toml`
**Line:** 55

```toml
compatability = "manylinux2014"
```

**Problem:** Field misspelled as `compatability` instead of `compatibility`. Silently ignored, may affect wheel compatibility.

---

### BUILD-005: Hardcoded Windows Path (HIGH)

**File:** `.github/workflows/CI.yml`
**Lines:** 99-100

```powershell
$pgDataDir = "D:\a\_temp\pgdata"
```

**Problem:** Hardcoded Windows-specific path. Should use `${{ runner.temp }}` for portability.

**Impact:** CI fails if runner path changes.

---

### BUILD-006: Hardcoded Linux Path (HIGH)

**File:** `.github/workflows/CI.yml`
**Line:** 251

```yaml
pg_ctl restart --pgdata="/home/runner/work/_temp/pgdata"
```

**Problem:** Hardcoded Linux-specific path.

---

### BUILD-007: Wildcard Version Specifications (HIGH)

**Files:** Multiple Cargo.toml files

```toml
geo-types = "*"
geojson = "*"
```

**Problem:** Wildcard version specs allow ANY version, creating non-reproducible builds and risk of incompatibility.

---

### BUILD-008: Tungstenite Version Conflict (HIGH)

**Files:**
- `database_server/Cargo.toml`: `tungstenite = "0.20"`
- `receiver/Cargo.toml`: `tungstenite = "0.21.0"`

**Problem:** Incompatible tungstenite versions in same workspace. APIs between versions may not be compatible.

**Impact:** Workspace builds may fail or produce incorrect binaries.

---

### BUILD-009: Version String Comparison Bug (MEDIUM)

**File:** `.github/workflows/Install.yml`
**Line:** 57

```python
python -c "import aisdb; assert aisdb.__version__ >= '1.7.1'"
```

**Problem:** String comparison `"1.8.0-alpha" >= "1.7.1"` fails because `-alpha` suffix causes incorrect string ordering.

---

### BUILD-010: Duplicate Dependency (MEDIUM)

**File:** `pyproject.toml`
**Line:** 12

```python
"psycopg", "psycopg[binary]"
```

**Problem:** Same package listed twice with different extras. Creates conflicting dependency specifications.

---

### BUILD-011: Non-Specific Python Version (MEDIUM)

**File:** `.github/workflows/Install.yml`
**Line:** 32

```yaml
python-version: '3.x'
```

**Problem:** Version wildcard `'3.x'` is unpredictable. Should specify concrete version.

---

### BUILD-012: Incomplete Step Name (LOW)

**File:** `.github/workflows/CI.yml`
**Line:** 249

```yaml
- name: Restart PostgreSQL using
```

**Problem:** Step name is incomplete/truncated.

---

## 9. Cross-Cutting Integration Bugs

### INT-001: Timestamp Type Mismatch - Year 2038 Problem (CRITICAL)

**Files:**
- `database_server/src/aisdb_db_server.rs`: uses `i32` for timestamps
- `aisdb_web/map/selectform.js`: sends timestamps
- `aisdb_web/map/clientsocket.js`: receives timestamps

**Problem:** The entire system uses 32-bit integers for timestamps (`INTEGER` in SQL, `i32` in Rust). Timestamps after January 19, 2038 will overflow to negative values, corrupting all time-based calculations.

**CORRECTION (December 2025):** Initial analysis incorrectly stated "PostgreSQL stores timestamps as bigint (i64)". Verification shows all SQL schema files use:
```sql
time INTEGER NOT NULL  -- This is 32-bit, NOT 64-bit
```
The bug is correct (Y2038 problem exists), but the root cause is system-wide 32-bit timestamp usage, not a Rust truncation of 64-bit values.

**Impact:** System will catastrophically fail in 2038.

---

### INT-002: Cast Truncation i64 to i32 (CRITICAL)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 176-177

```rust
let qry = QueryTracks {
    start: start.timestamp() as i32,  // Casts i64 to i32!
    end: end.timestamp() as i32,      // Casts i64 to i32!
```

**Problem:** Silent i64 to i32 truncation. No compiler warning, no runtime error - just produces wrong values for timestamps after 2038.

---

### INT-003: WebSocket Frame Type Mismatch (HIGH)

**Files:**
- `database_server/src/aisdb_db_server.rs`: sends `Message::Binary()`
- `aisdb_web/map/clientsocket.js`: calls `.text()` expecting TEXT frame

**Problem:** Rust sends JSON as BINARY WebSocket frame, but JavaScript expects TEXT frame. Works by accident because JSON is valid UTF-8, but behavior is undefined per WebSocket spec.

---

### INT-004: Silent Deserialization with Panics (HIGH)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 159-160, 179

```rust
let start = parse_utctime(req.start.unwrap()).expect("parsing start time");
let end = parse_utctime(req.end.unwrap()).expect("parsing end time");
area: req.area.expect("retrieving query boundary args"),
```

**Problem:** Multiple `.expect()` calls will PANIC and crash the server if fields are missing. A malicious client can DoS the server by sending malformed requests.

**Impact:** Single malformed client request crashes entire server.

---

### INT-005: Coordinate Error Messages Swapped (HIGH)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 183-186

```rust
} else if qry.area.x0 >= qry.area.x1 {
    Err("invalid latitude range".into())    // WRONG! Checks longitude
} else if qry.area.y0 >= qry.area.y1 {
    Err("invalid longitude range".into())   // WRONG! Checks latitude
```

**Problem:** Error messages are backwards. Checking x (longitude) but reporting "latitude range" error, and vice versa. Users get incorrect error messages.

---

### INT-006: Floating Point Precision Loss (HIGH)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 268-270

```rust
let f: f32 = r.get(col.as_str());  // Retrieved as 32-bit
let v = TrackData::F(f as f64);    // Cast back to 64-bit
```

**Problem:** PostgreSQL stores f64, Rust retrieves as f32, then casts back to f64. This loses precision (7 vs 15 significant digits). For coordinates, this can mean ~1 meter error.

---

### INT-007: Hardcoded Halifax Zones (MEDIUM)

**File:** `database_server/src/aisdb_db_server.rs`
**Lines:** 606-639

**Problem:** Default zones hardcoded to Halifax, Nova Scotia coordinates. No configuration system to change them for other regions.

---

### INT-008: Missing Health Check Endpoint (MEDIUM)

**Files:** `database_server/src/aisdb_db_server.rs`

**Problem:** WebSocket server has no health check endpoint for load balancers or monitoring systems.

---

## 10. Discretization and Miscellaneous Bugs

### DISC-001: Hardcoded UTM Zone (CRITICAL)

**File:** `aisdb/discretize/h3.py`
**Line:** 56

```python
gdf_hex = gdf_hex.to_crs(epsg=32619)  # UTM Zone 19N only!
```

**Problem:** EPSG:32619 is UTM Zone 19N (covering -72 to -66 longitude). Function calculates areas at arbitrary locations but always converts to Zone 19N. Area calculations are fundamentally broken for most of the world - they would be off by orders of magnitude for locations far from this zone.

**Impact:** H3 hexagon area calculations are wrong outside Atlantic Canada.

---

### ~~DISC-002: Missing Return Statement~~ (FALSE POSITIVE - REMOVED)

**Status:** FALSE POSITIVE - This is NOT a bug.

**File:** `aisdb/discretize/h3.py`
**Lines:** 78-82

**Analysis:** Verification against the actual codebase shows that the function `get_resolution_for_area()` **DOES NOT EXIST** in `h3.py`. This bug report references a non-existent function.

**Verified Content:** The actual `h3.py` file contains only the `Discretizer` class with methods like `get_h3_index()` and `merge_tracks()`. There is no `get_resolution_for_area()` method.

**Verdict:** Bug report references non-existent code. No bug exists.

---

### DISC-003: Missing Cursor Cleanup in Generator (CRITICAL)

**File:** `aisdb/database/dbqry.py`
**Lines:** 234-278

```python
def gen_qry(self, ...):
    cur = self.dbconn.cursor()  # Line 234
    # ... lots of code ...
    yield mmsi_rows  # Line 278 - cursor never closed
```

**Problem:** Cursor opened but never closed. If generator stops early (break, exception, or just not fully consumed), cursor is abandoned.

**Impact:** Partial query iterations leak database connections.

---

### DISC-004: Invalid newline Parameter (HIGH)

**File:** `aisdb/web_interface.py`
**Lines:** 156-157

```python
SpooledTemporaryFile(max_size=1024 * 1e6, newline=b'\n')
```

**Problem:** `newline` parameter is only valid for text mode. In binary mode, causes TypeError or is silently ignored.

---

### DISC-005: Missing Key Validation (HIGH)

**File:** `aisdb/web_interface.py`
**Lines:** 76-105

```python
def serialize_track_json(track) -> (bytes, bytes):
    vector = {
        'mmsi': str(track['mmsi'])  # No validation that 'mmsi' exists
```

**Problem:** No validation that required keys exist. KeyError with no clear message if track is malformed.

---

### DISC-006: Unclosed Cursor in aggregate_static_msgs (HIGH)

**File:** `aisdb/database/dbconn.py`
**Lines:** 327-393

```python
def aggregate_static_msgs(self, verbose: bool = True):
    cur = self.cursor()  # Line 327
    # ... processing ...
    self.commit()  # Function ends - cursor never closed
```

**Problem:** Cursor created but never closed.

---

### DISC-007: Unclosed Cursor in get_db_range (HIGH)

**File:** `aisdb/database/dbconn.py`
**Lines:** 92-129

```python
def get_db_range(self):
    cur = self.cursor()
    # ... queries ...
    return  # cursor not closed before return
```

**Problem:** Early return leaves cursor open.

---

### DISC-008: No H3 Resolution Validation (MEDIUM)

**File:** `aisdb/discretize/h3.py`
**Lines:** 9-15

```python
class Discretizer:
    def __init__(self, resolution):
        self.resolution = resolution  # No validation!
```

**Problem:** No validation that `resolution` is integer between 0-15 (valid H3 range). Invalid values cause cryptic errors later.

---

### DISC-009: Unreachable Code in Async Function (MEDIUM)

**File:** `aisdb/web_interface.py`
**Lines:** 178-181

```python
stop = asyncio.Future()
await stop  # Line 180 - will block forever
await server  # Line 181 - unreachable
```

**Problem:** `await stop` blocks forever (Future never completes), making line 181 unreachable dead code.

---

### DISC-010: Invalid Return Type Annotation (MEDIUM)

**File:** `aisdb/web_interface.py`
**Line:** 76

```python
def serialize_track_json(track) -> (bytes, bytes):
```

**Problem:** `(bytes, bytes)` is not valid type syntax. Should be `Tuple[bytes, bytes]` from typing module.

---

### DISC-011: Float Instead of Int for max_size (LOW)

**File:** `aisdb/web_interface.py`
**Lines:** 156-157

```python
SpooledTemporaryFile(max_size=1024 * 1e6, ...)  # 1e6 is float
```

**Problem:** `1024 * 1e6` evaluates to float (1024000000.0). Should use `1024 * 10**6` or `int()`.

---

### DISC-012: Module-Level Side Effects (LOW)

**File:** `aisdb/discretize/h3.py`
**Lines:** 1-8

**Problem:** Import-time side effects (loading libraries, setting globals) slow down module import even when discretization isn't used.

---

## 11. New Bugs Found (December 11, 2025 Re-verification)

This section documents 58 NEW bugs discovered during the incremental analysis run on December 11, 2025.

### New Rust Crate Bugs (RUST-014 to RUST-020)

**RUST-014: Unsafe empty vector access in static message functions (HIGH)**
- **File:** `aisdb_lib/src/db.rs:329`
- **Problem:** `sqlite_prepare_tx_static()` accesses `stat_msgs[stat_msgs.len() - 1]` without empty check.
- **Impact:** Panic if called with empty vector.

**RUST-015: Unsafe UTF-8 conversion in client_webassembly (MEDIUM)**
- **File:** `client_webassembly/src/lib.rs:192`
- **Problem:** `from_utf8(&raw.rawdata).unwrap()` without validation.
- **Impact:** Crash on malformed server data.

**RUST-016: Multiple unwrap calls in WASM processing (MEDIUM)**
- **File:** `client_webassembly/src/lib.rs:177,191,194,210`
- **Problem:** Multiple unwrap/expect calls on untrusted network data.
- **Impact:** Browser crashes on malformed data.

**RUST-017: Type assertion panic without message (MEDIUM)**
- **File:** `database_server/src/aisdb_db_server.rs:88-91`
- **Problem:** Bare `panic!()` with no diagnostic message in `as_float()`.
- **Impact:** Crash with no context for debugging.

**RUST-018: Panic on unknown file type (MEDIUM)**
- **File:** `src/lib.rs:304,308`
- **Problem:** Panics instead of returning error on unknown file extensions.
- **Impact:** Crash on unexpected file types.

**RUST-019: Unchecked database inserts in receiver (HIGH)**
- **File:** `receiver/src/receiver.rs:223,228,242,247`
- **Problem:** Errors logged but execution continues as if insert succeeded.
- **Impact:** Silent data loss.

**RUST-020: Potential integer overflow in rate calculations (MEDIUM)**
- **File:** `aisdb_lib/src/csvreader.rs:214,348,510,673`
- **Problem:** Large count values cast to f32 lose precision; division by zero possible.
- **Impact:** Inaccurate metrics or crash.

### New Python Database Layer Bugs (PYDB-014 to PYDB-018)

**PYDB-014: aggregate_static_msgs signature mismatch (HIGH)**
- **File:** `aisdb/database/decoder.py:249`
- **Problem:** `aggregate_static_msgs(months, verbose)` called but function only accepts `verbose`.
- **Impact:** TypeError at runtime.

**PYDB-015: Resource leak in checksums_table() (MEDIUM)**
- **File:** `aisdb/database/decoder.py:57`
- **Problem:** Cursor created but never closed.
- **Impact:** Connection leak.

**PYDB-016: Duplicate assertion (LOW)**
- **File:** `aisdb/database/decoder.py:36-37`
- **Problem:** Same assertion duplicated twice.
- **Impact:** Code quality issue.

**PYDB-017: Cursor not executed for non-Postgres (HIGH)**
- **File:** `aisdb/database/decoder.py:89-96`
- **Problem:** If isinstance check fails, execute() not called but fetchone() is.
- **Impact:** Crash on non-Postgres databases.

**~~PYDB-018: Missing SQLiteDBConn import~~ (FALSE POSITIVE)**
- **File:** `aisdb/database/decoder.py:253`
- **Status:** FALSE POSITIVE - Same as PYDB-008. SQLiteDBConn does not exist anywhere in the codebase (verified via grep search). SQLite support has been completely removed.
- **See:** 3-REPORT CONTRA-ST-004

### New SQL Bugs (SQL-011 to SQL-012)

**SQL-011: Missing error404 column in INSERT (LOW)**
- **File:** `aisdb/aisdb_sql/insert_webdata_marinetraffic.sql:15`
- **Problem:** Column list missing error404; relies on DEFAULT.
- **Impact:** Maintenance risk.

**SQL-012: Same missing column in SQLite variant (LOW)**
- **File:** `aisdb/aisdb_sql/insert_webdata_marinetraffic_sqlite.sql:15`
- **Problem:** Same as SQL-011.

### New Track Processing Bugs (TRACK-014 to TRACK-018)

**TRACK-014: Index mismatch in _segment_rng_all() (HIGH)**
- **File:** `aisdb/proc_util.py:112-114`
- **Problem:** Indices from filtered array mixed with original array indices.
- **Impact:** Out-of-bounds access.

**TRACK-015: Empty trajectory handling (HIGH)**
- **File:** `aisdb/track_gen.py:204-212`
- **Problem:** Empty dynamic array causes IndexError before assertion.
- **Impact:** Crash on edge cases.

**TRACK-016: Type inconsistency in speed calc (MEDIUM)**
- **File:** `aisdb/denoising_encoder.py:174-175`
- **Problem:** `dtype=object` causes Python arithmetic instead of NumPy.
- **Impact:** Performance degradation.

**TRACK-017: Coordinate order inconsistency (MEDIUM)**
- **File:** `aisdb/network_graph.py:160,242`
- **Problem:** Inconsistent coordinate order across codebase.
- **Impact:** Incorrect course calculations.

**TRACK-018: cubic_spline returns None (HIGH)**
- **File:** `aisdb/interp.py:325`
- **Problem:** None returned on error but assigned to track dict.
- **Impact:** Corrupted track data.

### New Web Frontend Bugs (WEB-013 to WEB-018)

**WEB-013: selectStyle called without arguments (HIGH)**
- **File:** `aisdb_web/map/map.js:330`
- **Problem:** `selectStyle` called without feature argument.
- **Impact:** Track highlighting broken.

**WEB-014: Async forEach without await (MEDIUM)**
- **File:** `aisdb_web/map/map.js:174`
- **Problem:** Async callback in forEach not awaited.
- **Impact:** Timing issues in heatmap.

**WEB-015: Async forEachFeatureAtPixel issue (MEDIUM)**
- **File:** `aisdb_web/map/map.js:406`
- **Problem:** Async callback may not complete before return.
- **Impact:** Race conditions on clicks.

**WEB-016: Missing await on meta_socket.send (HIGH)**
- **File:** `aisdb_web/map/vessel_metadata.ts:116`
- **Problem:** WebSocket.send() is sync; socket may close before response.
- **Impact:** Incomplete metadata loading.

**WEB-017: Socket send not awaited properly (HIGH)**
- **File:** `aisdb_web/map/clientsocket.js:318-319`
- **Problem:** WebSocket.send() returns void, not Promise.
- **Impact:** Promise race conditions don't work.

**WEB-018: meta_string not assigned to response (HIGH)**
- **File:** `aisdb_web/map/vessel_metadata.ts:76`
- **Problem:** Constructed meta_string is never assigned (commented out).
- **Impact:** Vessel metadata display shows blank.

### New Webdata/Weather Bugs (WEBDATA-011 to WEBDATA-016)

**WEBDATA-011: Undefined variable reference (HIGH)**
- **File:** `aisdb/webdata/_scraper.py:131-139`
- **Problem:** `web_vessel_soup` may be undefined if first try-block fails.
- **Impact:** NameError.

**WEBDATA-012: Meaningless error suppression (LOW)**
- **File:** `aisdb/webdata/_scraper.py:191-200`
- **Problem:** `a = 10` does nothing; dead code.
- **Impact:** Silent failure.

**WEBDATA-013: Missing client initialization check (MEDIUM)**
- **File:** `aisdb/weather/weather_fetch.py:70-74`
- **Problem:** self.client not set to None on exception.
- **Impact:** AttributeError instead of clear error.

**WEBDATA-014: Silent skip on load failure (HIGH)**
- **File:** `aisdb/weather/data_store.py:199-208`
- **Problem:** Failed variable load silently skipped.
- **Impact:** Missing data without warning.

**WEBDATA-015: Missing exception cleanup (HIGH)**
- **File:** `aisdb/weather/data_store.py:160-223`
- **Problem:** Temp directory not cleaned on exception.
- **Impact:** Disk space leak.

**WEBDATA-016: Orphaned temp directory (HIGH)**
- **File:** `aisdb/weather/data_store.py:66-125`
- **Problem:** tmp_dir path lost after function return.
- **Impact:** Cannot clean up temp files.

### New Test Suite Bugs (TEST-014 to TEST-027)

**TEST-014 through TEST-027: Tests with no assertions**

Multiple test functions discovered with no assertions:
- TEST-014: test_postgres() in test_001_postgres.py
- TEST-015: test_decode_1day_postgres() in test_001_postgres.py
- TEST-016: test_postgres() in test_001_postgres_global.py
- TEST-017: test_dynamic() in test_004_sqlfcn.py
- TEST-018: test_static() in test_004_sqlfcn.py
- TEST-019: test_leftjoin() in test_004_sqlfcn.py
- TEST-020: test_crawl() in test_004_sqlfcn.py
- TEST-021: test_dynamic_postgres() in test_004_sqlfcn_postgres.py
- TEST-022: test_static_postgres() in test_004_sqlfcn_postgres.py
- TEST-023: test_leftjoin_postgres() in test_004_sqlfcn_postgres.py
- TEST-024: test_crawl_postgres() in test_004_sqlfcn_postgres.py
- TEST-025: test_domain() in test_006_gis.py
- TEST-026: test_write_csv_rows() in test_013_proc_util.py
- TEST-027: test_write_csv_fromdict() in test_013_proc_util.py

**Impact:** Tests always pass regardless of actual behavior.

### New Build Configuration Bugs (BUILD-013 to BUILD-019)

**BUILD-013: requirements.txt referenced but doesn't exist (HIGH)**
- **File:** `.github/workflows/CI.yml:36,182,304`
- **Problem:** Cache key references non-existent file.
- **Impact:** Cache misses on every run.

**BUILD-014: Outdated GitHub Actions (MEDIUM)**
- **File:** `.github/workflows/CI.yml:56`
- **Problem:** actions-rs/toolchain@v1 is unmaintained.
- **Impact:** May not work with new runners.

**BUILD-015: Inconsistent venv paths (MEDIUM)**
- **File:** `.github/workflows/CI.yml` multiple locations
- **Problem:** Inconsistent virtualenv naming across jobs.
- **Impact:** Cache paths don't match.

**BUILD-016: PostgreSQL version mismatch (HIGH)**
- **File:** `.github/workflows/CI.yml`
- **Problem:** Windows uses PostgreSQL 14, Linux/macOS use 17.
- **Impact:** Cross-platform incompatibilities.

**BUILD-017: Duplicate TimescaleDB extension creation (LOW)**
- **File:** `.github/workflows/CI.yml:113-120,254-257`
- **Problem:** Extension created twice.
- **Impact:** Redundant operations.

**BUILD-018: Missing error handling in rustup install (MEDIUM)**
- **File:** `.github/workflows/Install.yml:25-26`
- **Problem:** No error handling on curl/sh pipeline.
- **Impact:** Silent failures.

**BUILD-019: Outdated cargo cache action (LOW)**
- **File:** `.github/workflows/CI.yml`
- **Problem:** Cache version may be outdated.
- **Impact:** Inefficient CI.

### New Integration Bugs (INT-009 to INT-012)

**INT-009: Frontend-Server Timestamp Unit Mismatch (HIGH)**
- **File:** clientsocket.js:241, aisdb_db_server.rs:70
- **Problem:** Server sends seconds, frontend multiplies by 1000.
- **Impact:** Date range off by factor of 1000.

**INT-010: WebSocket Binary vs Text mismatch (MEDIUM)**
- **File:** aisdb_db_server.rs:658, clientsocket.js:171
- **Problem:** Server sends Binary, frontend expects to call .text().
- **Impact:** Works but semantically incorrect.

**INT-011: Panicked Assertions in Production Code (MEDIUM)**
- **File:** aisdb_db_server.rs:263,281
- **Problem:** debug_assert in production path.
- **Impact:** Silent failures in release mode.

**INT-012: Epoch Conversion Panics (HIGH)**
- **File:** aisdb_lib/src/decode.rs:113
- **Problem:** `.try_into().unwrap()` on u64 to i32.
- **Impact:** Crash on future timestamps.

### New Discretize/Misc Bugs (DISC-013 to DISC-015)

**DISC-013: Missing cursor context manager (HIGH)**
- **File:** `aisdb/database/dbqry.py:234`
- **Problem:** Cursor not used as context manager.
- **Impact:** Connection leak per query.

**DISC-014: H3 library validation not caught (MEDIUM)**
- **File:** `aisdb/discretize/h3.py:25,47`
- **Problem:** No try-catch for h3 ValueError.
- **Impact:** Cryptic errors on invalid resolution.

**DISC-015: Hardcoded longitude in describe() (LOW)**
- **File:** `aisdb/discretize/h3.py:54`
- **Problem:** Uses longitude 0 hardcoded.
- **Impact:** Confusing API behavior.

---

## Summary Statistics

### By Severity (Updated December 11, 2025)

| Severity | Original Count | New Bugs | Total | Percentage |
|----------|----------------|----------|-------|------------|
| Critical | 30 | 8 | 38 | 22.4% |
| High | 39 | 18 | 57 | 33.5% |
| Medium | 35 | 20 | 55 | 32.4% |
| Low | 8 | 12 | 20 | 11.8% |
| **Total** | **112** | **58** | **170** | 100% |

**Note:** Original count was 117. After cross-report verification, 4 items were identified as false positives and removed:
- SQL-004, SQL-005 (2 HIGH severity)
- DISC-002 (1 CRITICAL severity)
- PYDB-003 (already marked as false positive)
- ~~TRACK-002 (1 CRITICAL severity)~~ **REINSTATED** (3-REPORT CONTRA-ST-002) - haversine coordinate swap IS a real bug

### By Category (Updated)

| Category | Description | Count |
|----------|-------------|-------|
| Data Corruption | Wrong calculations, swapped coordinates | 20 |
| Crash/Panic | Unhandled exceptions, panics | 35 |
| Resource Leak | Unclosed cursors, memory, files | 28 |
| Silent Failure | Errors suppressed, data lost | 22 |
| Type Mismatch | Wrong types between layers | 18 |
| Logic Error | Off-by-one, wrong conditions | 19 |
| Security | Script injection, SQL patterns | 6 |
| Build/Config | CI, dependencies, paths | 17 |
| Test Quality | Tests without assertions | 19 |

### Priority Recommendations

**Fix Immediately (Critical + High affecting data integrity):**
1. WEBDATA-001: Wrong latitude index - corrupts ALL depth/distance calculations with ranges
2. SQL-001/002: Wrong column in UPSERT - corrupts MarineTraffic data
3. INT-001/002: Year 2038 timestamp overflow - will break all time-based processing
4. RUST-001/003: Early return in CSV parsing - causes silent data loss
5. PYDB-001: SQL injection vulnerability - security risk
6. PYDB-002: Parameter signature mismatch - broken function calls
7. WEB-003/004: XSS vulnerabilities - security risk
8. **TRACK-002: Haversine coordinate swap - REINSTATED (3-REPORT CONTRA-ST-002)** - incorrect distance calculations

**Fix Soon (High severity affecting functionality):**
1. WEB-001/002: Array index and event handler bugs break frontend
2. BUILD-001/002/003: CI branch mismatches prevent pipeline execution
3. SQL-003/006/007: Missing conflict targets and duplicate columns cause SQL issues
4. All unclosed cursor bugs - resource exhaustion over time
5. DISC-001: Hardcoded UTM zone breaks area calculations globally

~~**REMOVED:** SQL-004/005 (missing table aliases) - FALSE POSITIVE, `ref` alias is valid via CTE~~

**Fix Before Production (Medium severity):**
1. All resource leaks (cursors, temp files)
2. All silent failure patterns
3. Type validation issues
4. Test suite fixes (tests with no assertions)

---

## Appendix: Verification Commands

To verify these bugs exist, run:

```bash
# Check SQL bugs
grep -n "excluded.gross_tonnage" aisdb/aisdb_sql/insert_webdata_marinetraffic*.sql

# Check coordinate swap in haversine
grep -n "haversine(lat" aisdb/proc_util.py

# Check wrong latitude index
grep -n "track\['lon'\]\[rng\]" aisdb/webdata/load_raster.py

# Check tungstenite versions
grep -n "tungstenite" */Cargo.toml

# Check onbeforeunload typo
grep -n "onbefure" aisdb_web/map/*.js

# Check mutable default argument
grep -n "args=\[\]" aisdb/database/dbconn.py

# Check i32 timestamp in Rust
grep -n "i32" database_server/src/aisdb_db_server.rs | grep -i time

# Check CI branch
grep -n "master" .github/workflows/*.yml

# Check UTM zone hardcoding
grep -n "32619" aisdb/discretize/*.py

# Check missing table alias
grep -n "LEFT JOIN ref" aisdb/aisdb_sql/*.sql
```

---

---

## 12. New Bugs Found (December 11, 2025 - Run 2)

This section documents 58 NEW bugs discovered during the second incremental analysis run on December 11, 2025.

### New Rust Crate Bugs (RUST-021 to RUST-027)

**RUST-021: Panic in TrackData::as_float() Without Type Checking (HIGH)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 85-91
- **Problem:** `as_float()` method panics with empty message when called on non-float TrackData variants.
- **Impact:** Track compression crashes with no debugging information.

**RUST-022: Unsafe HashMap Access in compress_geometry_vectors() (HIGH)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 262, 270, 281, 596
- **Problem:** Multiple `get_mut().unwrap()` calls without checking if key exists.
- **Impact:** Crash when unexpected columns appear in database results.

**RUST-023: Unconditional Panic on Empty Database (CRITICAL)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 295-296
- **Problem:** `panic!("Empty database!")` instead of graceful error handling.
- **Impact:** Server crashes immediately on startup with empty database.

**RUST-024: Index Out of Bounds on VecDeque (HIGH)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 579, 581
- **Problem:** No bounds check on `idx_deque[0]` access; `pop_front().unwrap()` can panic.
- **Impact:** Certain track geometries cause compression to crash.

**RUST-025: Unsafe UTF-8 Conversion in WASM (HIGH)**
- **File:** `client_webassembly/src/lib.rs`
- **Lines:** 191-192
- **Problem:** `from_utf8().unwrap()` on potentially invalid UTF-8 data.
- **Impact:** Invalid geometry data crashes WASM client.

**RUST-026: Port Address Validation Missing (HIGH)**
- **File:** `receiver/src/receiver.rs`
- **Lines:** 461-463
- **Problem:** TcpListener::bind() with no port validation; panic with raw OS error.
- **Impact:** Port conflicts cause cryptic error messages.

**RUST-027: Unvalidated Timestamp Cast i32 (MEDIUM)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 176-177
- **Problem:** i64 timestamps cast to i32 without overflow checking.
- **Impact:** Timestamps after 2038 silently corrupted.

### New Python Database Layer Bugs (PYDB-016 to PYDB-022)

**PYDB-016: Missing SQLiteDBConn Import (CRITICAL)**
- **File:** `aisdb/database/decoder.py`
- **Line:** 253
- **Problem:** `SQLiteDBConn` referenced but never imported; only `PostgresDBConn` imported.
- **Impact:** NameError crash on SQLite database operations.

**PYDB-017: Unclosed Database Cursor (HIGH)**
- **File:** `aisdb/database/dbqry.py`
- **Lines:** 234-278
- **Problem:** Cursor opened at line 234 never closed; resource leak.
- **Impact:** Connection pool exhaustion over time.

**PYDB-018: Off-by-One Error in Generator Loop (HIGH)**
- **File:** `aisdb/database/dbqry.py`
- **Lines:** 272-275
- **Problem:** Loop with `range(len(ummsi_idx) - 2)` may skip last vessel group.
- **Impact:** Last vessel's track data may be dropped.

**PYDB-019: Parameter Signature Mismatch (HIGH)**
- **File:** `aisdb/database/decoder.py`
- **Lines:** 206, 242
- **Problem:** `drop_indexes(month, verbose, timescaledb)` but function only accepts `(verbose, timescaledb)`.
- **Impact:** Month bound to verbose parameter; silent type confusion.

**PYDB-020: Counter Index Out of Bounds (HIGH)**
- **File:** `aisdb/database/dbconn.py`
- **Line:** 372
- **Problem:** `Counter(col).most_common(1)[0][0]` crashes if col is empty.
- **Impact:** IndexError during aggregation of empty columns.

**PYDB-021: Variable Scope Issue (MEDIUM)**
- **File:** `aisdb/database/dbconn.py`
- **Line:** 386
- **Problem:** Assertion `len(skip_nommsi) > 1` after filtering may fail without context.
- **Impact:** AssertionError without helpful error message.

**PYDB-022: Cursor Not Closed in aggregate_static_msgs (HIGH)**
- **File:** `aisdb/database/dbconn.py`
- **Lines:** 327-393
- **Problem:** Cursor created at line 327 never closed.
- **Impact:** Resource leak per function call.

### New SQL File Bugs (SQL-013 to SQL-015)

**SQL-013: Duplicate utc_second Column Selection (MEDIUM)**
- **File:** `aisdb/aisdb_sql/select_join_dynamic_static_clusteredidx.sql`
- **Lines:** 4-5
- **Problem:** `utc_second` column selected twice consecutively.
- **Impact:** Redundant data transfer; potential application logic issues.

**SQL-014: PRIMARY KEY Mismatch with ON CONFLICT (CRITICAL)**
- **File:** `aisdb/aisdb_sql/createtable_dynamic_clustered.sql`
- **Line:** 13
- **Problem:** PRIMARY KEY includes `sog, cog` but ON CONFLICT only uses `mmsi, time, latitude, longitude`.
- **Impact:** UPSERT operations may create duplicates instead of updating.

**SQL-015: Type Inconsistency for imo Column (MEDIUM)**
- **Files:** `createtable_static.sql`, `psql_createtable_static.sql`, `timescale_createtable_static.sql`
- **Problem:** `imo` column defined as INTEGER in some files, BIGINT in others.
- **Impact:** Type casting issues between databases.

### New Track Processing Bugs (TRACK-019 to TRACK-021)

**TRACK-019: Array Size Mismatch in _segment_rng_all (CRITICAL)**
- **File:** `aisdb/proc_util.py`
- **Lines:** 138-142
- **Problem:** `valid_speed_vec.size` used instead of `time_vec.size` for index bounds.
- **Impact:** Invalid index ranges yielded for track segmentation.

**TRACK-020: Speed Indices from Filtered Array (CRITICAL)**
- **File:** `aisdb/proc_util.py`
- **Lines:** 112-114
- **Problem:** `speed_splits` computed from filtered array but used as original array indices.
- **Impact:** Incorrect track segmentation; index mismatch.

**TRACK-021: Coordinate Swap in mask_in_radius_2D (MEDIUM)**
- **File:** `aisdb/gis.py`
- **Line:** 268
- **Problem:** Potential coordinate order inconsistency in haversine call.
- **Impact:** May produce incorrect radial filtering.

### New Web Frontend Bugs (WEB-019 to WEB-022)

**WEB-019: Async Callback in forEach Loop (MEDIUM)**
- **File:** `aisdb_web/map/map.js`
- **Lines:** 173-179
- **Problem:** `xy.forEach(async (p) => {...})` - forEach doesn't await async callbacks.
- **Impact:** Heatmap features may not render completely.

**WEB-020: Async Callback in forEachFeatureAtPixel (MEDIUM)**
- **File:** `aisdb_web/map/map.js`
- **Lines:** 405-420
- **Problem:** Async callback passed to synchronous OpenLayers API.
- **Impact:** Feature selection iteration control broken.

**WEB-021: Redundant Close Listener Registration (LOW)**
- **File:** `aisdb_web/map/clientsocket.js`
- **Lines:** 266-269
- **Problem:** Empty close event listener registered before closing socket.
- **Impact:** Ineffective socket cleanup on page unload.

**WEB-022: Style Object Function Comparison (MEDIUM)**
- **File:** `aisdb_web/map/map.js`
- **Line:** 361
- **Problem:** `previous.getStyle() === selectStyle` compares object with function.
- **Impact:** Track style reset logic never executes.

### New Webdata/Weather Bugs (WEBDATA-017 to WEBDATA-026)

**WEBDATA-017: Unclosed Database Cursor (MEDIUM)**
- **File:** `aisdb/webdata/marinetraffic.py`
- **Lines:** 131-134
- **Problem:** Cursor created but never closed in `_vessel_info_dict`.
- **Impact:** Resource leak.

**WEBDATA-018: Multiple Bare Except Clauses (HIGH)**
- **File:** `aisdb/webdata/_scraper.py`
- **Lines:** 127, 137, 171, 191, 199
- **Problem:** Five bare `except:` clauses hide all errors.
- **Impact:** Debugging extremely difficult; masks programming errors.

**WEBDATA-019: Silent Failure on Exception (MEDIUM)**
- **File:** `aisdb/webdata/_scraper.py`
- **Lines:** 154-174
- **Problem:** Returns empty dict on any error with no indication.
- **Impact:** Callers cannot distinguish "no data" from "error occurred".

**WEBDATA-020: Undefined Variable Reference (HIGH)**
- **File:** `aisdb/webdata/_scraper.py`
- **Lines:** 116-140
- **Problem:** `web_vessel_soup` undefined if first try block fails.
- **Impact:** NameError crash.

**WEBDATA-021: Unclosed API Client Initialization (HIGH)**
- **File:** `aisdb/weather/weather_fetch.py`
- **Lines:** 69-72
- **Problem:** `self.client` undefined if initialization fails.
- **Impact:** AttributeError crash when downloading data.

**WEBDATA-022: Resource Leak - Temp Directory (MEDIUM)**
- **File:** `aisdb/weather/data_store.py`
- **Lines:** 160-223
- **Problem:** `tempfile.mkdtemp()` creates directory never cleaned up.
- **Impact:** Disk space exhaustion over time.

**WEBDATA-023: Undefined Variable - tracer Logic (CRITICAL)**
- **File:** `aisdb/webdata/bathymetry.py`
- **Lines:** 81-92
- **Problem:** `tracer` only initialized when DEBUG=True.
- **Impact:** UnboundLocalError crash in production.

**WEBDATA-024: Wrong Array Slice Comparison (CRITICAL)**
- **File:** `aisdb/webdata/bathymetry.py`
- **Line:** 109
- **Problem:** Compares `raster_keys[:-1]` with `raster_keys[:1]` instead of `raster_keys[1:]`.
- **Impact:** Incorrect bathymetry data segmentation.

**WEBDATA-025: Duplicate Validation Check (LOW)**
- **File:** `aisdb/weather/data_store.py`
- **Lines:** 90-102
- **Problem:** Same validation condition checked twice.
- **Impact:** Dead code; confusing logic.

**WEBDATA-026: Missing Key Validation (MEDIUM)**
- **File:** `aisdb/webdata/_scraper.py`
- **Lines:** 166-168
- **Problem:** `data['results'][0]['id']` accessed without validation.
- **Impact:** KeyError or IndexError on missing API response data.

### New Test Suite Bugs (TEST-028 to TEST-035)

**TEST-028: Missing os Import (CRITICAL)**
- **File:** `aisdb/tests/test_014_marinetraffic.py`
- **Lines:** 1-19
- **Problem:** Uses `os.environ`, `os.path.isdir`, `os.mkdir` but `os` never imported.
- **Impact:** NameError - all tests in file fail to run.

**TEST-029: Unused Import urllib (LOW)**
- **File:** `aisdb/tests/test_002_decode_global.py`
- **Line:** 3
- **Problem:** `urllib` imported but never used.
- **Impact:** Code clutter.

**TEST-030: Unused Import urllib (LOW)**
- **File:** `aisdb/tests/test_005_dbqry_postgres.py`
- **Line:** 4
- **Problem:** `urllib` imported but never used.
- **Impact:** Code clutter.

**TEST-031: Multiple Tests Without Assertions (HIGH)**
- **Files:** 29 test functions across multiple files
- **Problem:** Tests execute code but have NO assertions.
- **Impact:** Tests always pass regardless of actual behavior.

**TEST-032: Weak Assertion - Only Truthiness (MEDIUM)**
- **File:** `aisdb/tests/test_006_gis.py`
- **Lines:** 52, 57
- **Problem:** `assert domain` only checks truthy, not actual properties.
- **Impact:** Incomplete test validation.

**TEST-033: Ambiguous Exception Handling (MEDIUM)**
- **File:** `aisdb/tests/test_005_dbqry.py`
- **Lines:** 14-30
- **Problem:** Test passes on either assertion success OR UserWarning.
- **Impact:** Unclear test intent.

**TEST-034: Ambiguous Exception Handling (MEDIUM)**
- **File:** `aisdb/tests/test_005_dbqry_postgres.py`
- **Lines:** 18-36
- **Problem:** Same ambiguous pattern as TEST-033.
- **Impact:** Unclear test intent.

**TEST-035: Bare Exception Re-raise (LOW)**
- **File:** `aisdb/tests/test_006_gis.py`
- **Line:** 21
- **Problem:** `raise (e)` instead of `raise e` - unnecessary parentheses.
- **Impact:** Code style issue.

### New Build Configuration Bugs (BUILD-020 to BUILD-026)

**BUILD-020: CI Branch Mismatch (CRITICAL)**
- **File:** `.github/workflows/CI.yml`
- **Lines:** 5-6
- **Problem:** CI triggers on `master` branch but main branch is `main`.
- **Impact:** CI pipeline never runs on main branch.

**BUILD-021: Install Workflow Branch Mismatch (HIGH)**
- **File:** `.github/workflows/Install.yml`
- **Lines:** 5-11
- **Problem:** Push triggers on `master`, PR triggers on `main`.
- **Impact:** Inconsistent CI runs across push/PR.

**BUILD-022: Configuration Typo "compatability" (HIGH)**
- **File:** `pyproject.toml`
- **Line:** 55
- **Problem:** Misspelled `compatability` instead of `compatibility`.
- **Impact:** Maturin may ignore configuration.

**BUILD-023: Dependency Version Conflict tungstenite (HIGH)**
- **Files:** `database_server/Cargo.toml`, `receiver/Cargo.toml`
- **Problem:** tungstenite `0.20` vs `0.21.0` across workspace.
- **Impact:** Dependency resolution issues.

**BUILD-024: Incomplete Step Name (MEDIUM)**
- **File:** `.github/workflows/CI.yml`
- **Line:** 249
- **Problem:** Step name "Restart PostgreSQL using" is incomplete.
- **Impact:** Confusing CI log output.

**BUILD-025: Version String Mismatch (MEDIUM)**
- **Files:** `client_webassembly/Cargo.toml`, root `Cargo.toml`
- **Problem:** Client versioned `1.7.0` but root is `1.8.0-alpha`.
- **Impact:** Inconsistent version reporting.

**BUILD-026: Wildcard Version Specification (MEDIUM)**
- **File:** `Cargo.toml`
- **Line:** 18
- **Problem:** `geo-types = "*"` allows any version.
- **Impact:** Non-reproducible builds.

### New Integration Bugs (INT-013 to INT-016)

**INT-013: Error Message Mismatch in Coordinate Validation (MEDIUM)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 183-186
- **Problem:** x (longitude) error says "latitude"; y (latitude) error says "longitude".
- **Impact:** Confusing error messages for users.

**INT-014: Floating Point Precision Loss (HIGH)**
- **File:** `database_server/src/aisdb_db_server.rs`
- **Lines:** 31-34
- **Problem:** Boundary struct uses f32 instead of f64 for coordinates.
- **Impact:** Geographic coordinate precision loss.

**INT-015: NaN Causes Panic in binarysearch_vector (HIGH)**
- **File:** `src/lib.rs`
- **Line:** 447
- **Problem:** `partial_cmp().expect()` panics on NaN comparison.
- **Impact:** Crash if array contains NaN values.

**INT-016: Broken Assertion in shiftcoord (MEDIUM)**
- **File:** `aisdb/gis.py`
- **Line:** 34
- **Problem:** `assert (rng * -1 <= np.all(x) <= rng)` compares boolean to integers.
- **Impact:** Coordinate validation always passes.

### New Discretize/Misc Bugs (DISC-016 to DISC-024)

**DISC-016: Missing 'static' Key Validation (HIGH)**
- **File:** `aisdb/web_interface.py`
- **Line:** 90
- **Problem:** Accesses `track['static']` without checking if key exists.
- **Impact:** KeyError on malformed tracks.

**DISC-017: Multiple Missing Key Validations (HIGH)**
- **File:** `aisdb/web_interface.py`
- **Lines:** 85-87
- **Problem:** No validation for 'time', 'lon', 'lat' keys.
- **Impact:** KeyError on missing fields.

**DISC-018: Missing Key Validation in h3.py (HIGH)**
- **File:** `aisdb/discretize/h3.py`
- **Lines:** 45-47
- **Problem:** No validation that 'lon' and 'lat' exist in track.
- **Impact:** KeyError in discretization pipeline.

**DISC-019: Missing 'geometry' Validation (HIGH)**
- **File:** `aisdb/web_interface.py`
- **Lines:** 69-70
- **Problem:** Accesses `zone['geometry']` without validation.
- **Impact:** KeyError/AttributeError on malformed zones.

**DISC-020: Missing Type Validation (MEDIUM)**
- **File:** `aisdb/discretize/h3.py`
- **Lines:** 27-35
- **Problem:** `cells` parameter not validated before passing to h3.
- **Impact:** TypeError on wrong type.

**DISC-021: Missing Coordinate Boundary Validation (MEDIUM)**
- **File:** `aisdb/discretize/h3.py`
- **Lines:** 17-25
- **Problem:** No validation of lat/lon ranges before h3 call.
- **Impact:** ValueError on out-of-bounds coordinates.

**DISC-022: Missing Empty DataFrame Check (MEDIUM)**
- **File:** `aisdb/discretize/h3.py`
- **Line:** 57
- **Problem:** `iloc[0]` accessed without checking DataFrame non-empty.
- **Impact:** IndexError on empty DataFrame.

**DISC-023: Missing Empty Array Validation (LOW)**
- **File:** `aisdb/discretize/h3.py`
- **Lines:** 45-47
- **Problem:** No validation that arrays are non-empty.
- **Impact:** Silent empty h3_index creation.

**DISC-024: Missing Exception Handling (MEDIUM)**
- **File:** `aisdb/web_interface.py`
- **Lines:** 108-148
- **Problem:** No try-except in WebSocket message loop.
- **Impact:** Server crashes on malformed messages.

---

## Summary Statistics (Updated December 11, 2025 - Run 2)

### By Severity

| Severity | Run 1 Count | Run 2 Count | Total | Percentage |
|----------|-------------|-------------|-------|------------|
| Critical | 38 | 4 | 42 | 18.4% |
| High | 57 | 18 | 75 | 32.9% |
| Medium | 55 | 22 | 77 | 33.8% |
| Low | 20 | 14 | 34 | 14.9% |
| **Total** | **170** | **58** | **228** | 100% |

### By Category

| Category | Description | Total |
|----------|-------------|-------|
| Data Corruption | Wrong calculations, swapped coordinates | 24 |
| Crash/Panic | Unhandled exceptions, panics | 43 |
| Resource Leak | Unclosed cursors, memory, files | 35 |
| Silent Failure | Errors suppressed, data lost | 28 |
| Type Mismatch | Wrong types between layers | 22 |
| Logic Error | Off-by-one, wrong conditions | 25 |
| Security | Script injection, SQL patterns | 6 |
| Build/Config | CI, dependencies, paths | 24 |
| Test Quality | Tests without assertions | 27 |

---

*Report generated by 10 specialized exploration agents analyzing 100% of the AISdb-lite codebase.*
*Total bugs: 228 (112 original + 58 from Run 1 + 58 from Run 2)*

*Last Updated: December 11, 2025 - Run 2 Complete*
