# AISdb-Lite: Engineering Blueprint & Refactoring Plan Prompt

> **Prompt Version**: 1.0.0
> **Target Report**: 4-REPORT.md
> **Analysis Type**: Engineering Plan for High-Performance PostgreSQL-Only AIS Pipeline
> **Last Updated**: December 2025

---

## Overview

This prompt orchestrates the creation of a comprehensive engineering blueprint for refactoring AISdb-lite into a **high-performance, PostgreSQL-only AIS data pipeline**. The analysis uses Reports 0, 1, and 2 as primary input and coordinates multiple specialized agents to produce an actionable implementation plan.

**Core Transformation Goals:**
1. PostgreSQL as the ONLY database backend (remove SQLite entirely)
2. Headless backend (remove ALL visualization)
3. Rust for performance-critical paths
4. Python for flexibility and rapid development
5. Single fixed machine deployment optimization

---

## Report Writing Guidelines

### No Page Limit
- **There is NO page limit** for the report
- Document every architectural decision with full justification
- Include complete code examples, diagrams, and SQL schemas
- Comprehensive coverage is essential for implementation

### Avoid Duplications
- One section per major topic - no repeated analysis
- Cross-reference related sections: "See Section X.X"
- Reference bug IDs from 1-REPORT.md and decision IDs from 2-REPORT.md
- Consolidate related recommendations into single coherent sections

### Reduce Verbosity
- Lead with the decision/recommendation, then justify
- Use tables for comparisons (current vs target, Rust vs Python, etc.)
- Keep code examples focused on the pattern being illustrated
- Distinguish ACTUAL code from ILLUSTRATIVE examples
- No filler: "In this section we will examine..." - just present findings

### Traceability Requirements
Every recommendation MUST include:

```
REQUIRED FOR EACH COMPONENT/DECISION:
├── Section ID: Part.Section (e.g., 1.1, 4.3)
├── Location: affected file path(s) from repository root
├── Current State: what exists now (with line numbers if applicable)
├── Target State: what it should become
├── Justification: why this change is necessary (with metrics if possible)
├── Dependencies: what must be done first
├── Verification: how to confirm the change is correct
└── Related: links to 0-REPORT, 1-REPORT, 2-REPORT sections

EXAMPLE:
### 1.2 SQLite Database Abstraction Removal

**Files:**
- `aisdb_lib/src/db.rs:27-66` (sqlite_prepare_dynamic_insertion)
- `aisdb_lib/src/db.rs:217-251` (insert_dynamic_sqlite)

**Current State:** Dual database support with ~600 lines of SQLite-specific Rust code

**Target State:** PostgreSQL-only with zero SQLite references

**Justification:**
- SQLite cannot handle concurrent reads/writes (single-writer lock)
- No spatial indexing (full table scans for geo queries)
- Code duplication doubles maintenance burden

**Dependencies:** None (first pruning step)

**Verification:**
```bash
rg -n "sqlite|rusqlite" --type rust . && echo "FAIL" || echo "PASS"
```

**Related:** 1-REPORT SQL-001, 2-REPORT Section 8.4
```

### ASCII Diagrams Requirements
The report MUST include comprehensive ASCII diagrams:

1. **Target System Architecture** - Main components in Rust and Python with interactions
2. **Data Flow Diagram** - AIS ingestion through post-processing
3. **Database Schema Diagram** - Tables, relationships, partitioning layout
4. **Before/After Comparison** - Current vs target architecture
5. **Rust-Python Interface Diagram** - PyO3 boundaries and data transformations

Use box-drawing characters (┌ ─ ┐ │ └ ┴ ┘ ├ ┼ ┤) for clear diagrams.

---

## Phase 1: Read Input Reports

Before any analysis, read and internalize the three source reports:

### Agent 1.1: Architecture Report Analysis

```
TASK: Analyze 0-REPORT.md for current system architecture

READ: 0-REPORT.md

EXTRACT:
1. Complete component inventory (Rust, Python, JavaScript modules)
2. Database interaction patterns
3. Data flow descriptions
4. Cross-language interfaces (PyO3 bindings)
5. External dependencies

OUTPUT: Structured summary of current architecture to inform pruning decisions
```

### Agent 1.2: Bug Report Analysis

```
TASK: Analyze 1-REPORT.md for bugs that influence architecture decisions

READ: 1-REPORT.md

EXTRACT:
1. Database-related bugs (SQL, connection management)
2. Performance bugs (FFI overhead, N+1 queries)
3. Data integrity bugs (precision, Y2038)
4. Concurrency bugs (race conditions, locking)

OUTPUT: List of bugs that must be addressed by the refactoring plan
```

### Agent 1.3: Bad Decisions Report Analysis

```
TASK: Analyze 2-REPORT.md for architectural anti-patterns

READ: 2-REPORT.md

EXTRACT:
1. Database design flaws
2. Scalability limitations
3. Security concerns
4. Maintenance burden patterns

OUTPUT: List of architectural decisions to reverse or improve
```

---

## Phase 2: Component Pruning (MUST COMPLETE FIRST)

**CRITICAL**: Before proposing ANY additions, identify and document ALL components to remove.

### Agent 2.1: SQLite Removal Analysis

```
TASK: Produce complete SQLite removal plan

SEARCH:
- All Rust files for: sqlite, rusqlite, SqliteConnection
- All Python files for: sqlite, SQLite
- Cargo.toml files for sqlite dependencies
- SQL files with "sqlite" in name

FOR EACH FINDING:
- File path and exact line numbers
- Lines of code to remove
- Dependencies on this code
- Verification command

OUTPUT FORMAT:
| File | Lines | Function/Block | Lines to Remove |
|------|-------|----------------|-----------------|
| aisdb_lib/src/db.rs | 27-66 | sqlite_prepare_dynamic_insertion | 40 |

DELIVERABLES:
1. Complete file-by-file removal checklist
2. Cargo.toml changes (features, dependencies)
3. Verification script to confirm complete removal
4. Total lines removed count
```

### Agent 2.2: Visualization Removal Analysis

```
TASK: Produce complete visualization removal plan

IDENTIFY FOR DELETION:
1. Entire directories:
   - aisdb_web/ (JavaScript/TypeScript frontend)
   - Any other visualization-specific directories

2. Python files:
   - web_interface.py
   - Any matplotlib plotting code
   - Flask/WebSocket server code

3. Rust files:
   - WebAssembly client code
   - WebSocket server bindings

4. Dependencies:
   - Flask, websockets, matplotlib in Python
   - wasm-bindgen, web-sys in Rust
   - Node.js/npm ecosystem

FOR EACH COMPONENT:
- Exact path
- Lines of code
- Why it should be removed
- What depends on it (impact analysis)

OUTPUT FORMAT:
| Component | Type | Lines | Reason for Removal |
|-----------|------|-------|-------------------|
| aisdb_web/ | Directory | ~2000 | Frontend visualization |
| web_interface.py | File | 225 | Flask WebSocket server |

DELIVERABLES:
1. Directory deletion list with sizes
2. File deletion list with line counts
3. Import statement removals from __init__.py
4. Dependency removals from requirements.txt/Cargo.toml
5. Verification script
6. API design sketch for external visualization tools
```

### Agent 2.3: Legacy Abstraction Removal

```
TASK: Identify legacy abstractions that should be removed or simplified

SEARCH FOR:
1. Database abstraction layers (DBConn classes, connection wrappers)
2. Unused utility functions
3. Compatibility shims for old Python/Rust versions
4. Dead code paths (functions never called)
5. Deprecated modules (marked for removal)

FOR EACH FINDING:
- Why it exists
- Why it should be removed
- Impact on remaining code
- Safe removal verification

OUTPUT: Prioritized list of abstractions to simplify or remove
```

---

## Phase 3: Rust vs Python Decision Framework

### Agent 3.1: Rust Migration Analysis

```
TASK: Determine which components should migrate to Rust

EVALUATION CRITERIA (weight 1-5):
| Criterion | Weight | Description |
|-----------|--------|-------------|
| CPU-bound computation | 5 | Loops, math, encoding/decoding |
| Memory-critical | 4 | Large arrays, zero-copy requirements |
| Latency-sensitive | 4 | Real-time processing paths |
| FFI overhead victim | 5 | Currently suffering Python↔Rust crossing |
| Safety-critical | 3 | Parsing untrusted input |
| Concurrency required | 4 | Parallel processing needs |

CURRENT PYTHON CODE TO EVALUATE:
- aisdb/track_gen.py - Track generation
- aisdb/interp.py - Interpolation functions
- aisdb/proc_util.py - Processing utilities
- aisdb/denoising.py - Denoising algorithms
- aisdb/gis.py - Geospatial calculations
- aisdb/distance.py - Distance computations

FOR EACH CANDIDATE:
1. Profile current Python performance
2. Estimate Rust improvement factor
3. Identify library alternatives (prefer existing crates)
4. Design PyO3 interface
5. Specify data types for zero-copy transfer

OUTPUT FORMAT:
| Module | Current Lang | Target Lang | Speedup Est | Library | Justification |
|--------|-------------|-------------|-------------|---------|---------------|
| track_gen.py | Python | Rust | 10x | custom | Hot loop with NumPy overhead |

LIBRARY PREFERENCE RULE:
- ALWAYS prefer existing, well-maintained libraries
- Only implement custom code when:
  1. No suitable library exists, OR
  2. Clear, documented justification for deviation
- Name specific crate versions (e.g., geo = "0.27.0")
```

### Agent 3.2: Python Retention Analysis

```
TASK: Determine which components should remain in Python

RETENTION CRITERIA:
| Criterion | Justification |
|-----------|---------------|
| I/O-bound operations | Network/disk waits dominate, not CPU |
| Complex orchestration | Workflow coordination, easier in Python |
| Rapid iteration needed | Frequently changing logic |
| External API integration | Python ecosystem advantage |
| Database queries | SQL generation, cursor management |
| Configuration | YAML/JSON parsing, environment handling |

CURRENT PYTHON CODE TO EVALUATE:
- aisdb/database/*.py - Database operations
- aisdb/weather/*.py - Weather data integration
- aisdb/webdata/*.py - External data sources
- aisdb/*.py - Top-level modules

FOR EACH MODULE:
1. Primary workload type (CPU, I/O, orchestration)
2. Change frequency (stable vs volatile)
3. External dependencies
4. Interoperability requirements

OUTPUT FORMAT:
| Module | Reason to Keep in Python | Optimization Opportunity |
|--------|--------------------------|-------------------------|
| dbqry.py | I/O-bound, SQL generation | Use server-side cursors |
```

### Agent 3.3: Interface Design

```
TASK: Design clean Rust↔Python interface

PRINCIPLES:
1. Minimize FFI crossings (batch operations)
2. Zero-copy where possible (NumPy arrays ↔ Rust slices)
3. Clear ownership semantics
4. Type safety with PyO3

CURRENT INTERFACE (from 0-REPORT):
| Python Function | Rust Function | Data Flow |
|-----------------|---------------|-----------|
| decoder() | decoder() | Path → Vec<VesselData> |
| haversine() | haversine() | Scalars → f64 |

DESIGN NEW INTERFACE:
| Python Function | Rust Function | Input Types | Output Types | Transfer Method |
|-----------------|---------------|-------------|--------------|-----------------|
| track_distance_batch() | track_distance_batch() | (np.ndarray, np.ndarray) | np.ndarray | Zero-copy view |

OUTPUT:
1. Complete PyO3 binding specifications
2. Type conversion table
3. Memory ownership rules
4. Error handling strategy
```

---

## Phase 4: Database Architecture Deep Design

### Agent 4.1: Schema Design

```
TASK: Design optimal PostgreSQL schema for AIS data

REQUIREMENTS:
- Support billions of AIS messages
- Efficient time-range queries
- Efficient spatial queries (bounding box, radius)
- Efficient vessel-specific queries (MMSI lookup)
- TimescaleDB compatibility
- PostGIS integration

NORMALIZATION ANALYSIS:
1. Identify functional dependencies
2. Apply normalization to 3NF minimum
3. Justify any denormalization decisions
4. Document trade-offs

SCHEMA COMPONENTS:
1. ais_dynamic (position reports) - hypertable
2. ais_static (vessel metadata) - regular table
3. Reference tables (ship types, navigation status)
4. Aggregation tables (continuous aggregates)

FOR EACH TABLE:
| Column | Type | Constraint | Justification |
|--------|------|------------|---------------|
| time | TIMESTAMPTZ | NOT NULL | TimescaleDB requirement |
| mmsi | INTEGER | NOT NULL | 9-digit vessel ID |
| lon | DOUBLE PRECISION | CHECK(-180..180) | Full precision needed |

OUTPUT:
1. Complete CREATE TABLE statements
2. Primary key justifications
3. Foreign key relationships
4. Check constraints with rationale
5. Storage size estimates per billion rows
```

### Agent 4.2: Index Architecture

```
TASK: Design comprehensive indexing strategy

INDEX TYPES TO CONSIDER:
| Type | Use Case | When to Use |
|------|----------|-------------|
| B-tree | Equality, range queries | Default for most columns |
| BRIN | Time-ordered data | Large tables with natural ordering |
| GiST | Spatial queries | PostGIS geometry columns |
| GIN | Array/JSONB containment | Full-text, array membership |
| Hash | Equality only | Faster than B-tree for exact match |

CRITICAL QUERY PATTERNS:
1. Track retrieval: WHERE mmsi = X AND time BETWEEN A AND B
2. Spatial search: WHERE ST_Within(geom, bbox)
3. Time slice: WHERE time BETWEEN A AND B
4. Vessel lookup: WHERE mmsi IN (...)

FOR EACH INDEX:
| Index Name | Table | Columns | Type | Query Pattern | Justification |
|------------|-------|---------|------|---------------|---------------|
| idx_dynamic_mmsi_time | ais_dynamic | (mmsi, time DESC) | B-tree | Track retrieval | Covering index |

OUTPUT:
1. Complete CREATE INDEX statements
2. Column order justifications
3. Partial index definitions (WHERE clauses)
4. Covering index recommendations
5. INCLUDE column decisions
6. Index size estimates
```

### Agent 4.3: Partitioning Strategy

```
TASK: Design TimescaleDB partitioning strategy

HYPERTABLE CONFIGURATION:
- Chunk interval selection (1 day, 1 week, 1 month?)
- Compression policy (age threshold)
- Retention policy (data lifecycle)
- Continuous aggregate intervals

ANALYSIS:
1. Query patterns vs chunk boundaries
2. Compression ratio estimates
3. Retention requirements
4. Archive strategy (if needed)

OUTPUT:
1. create_hypertable() calls with parameters
2. add_compression_policy() configuration
3. add_retention_policy() if applicable
4. Continuous aggregate definitions
5. Chunk interval justification
```

### Agent 4.4: PostgreSQL Tuning

```
TASK: Design PostgreSQL configuration for AIS workload

TUNING CATEGORIES:
1. Memory (shared_buffers, work_mem, maintenance_work_mem)
2. Parallelism (max_parallel_workers, parallel_tuple_cost)
3. WAL (wal_buffers, checkpoint_completion_target)
4. Autovacuum (aggressive for high-insert workload)
5. Connection pooling (PgBouncer configuration)

WORKLOAD PROFILE:
- Insert-heavy (bulk ingestion)
- Read-heavy (track queries)
- Mixed (concurrent ingest + query)

OUTPUT FORMAT:
| Parameter | Default | Recommended | Justification |
|-----------|---------|-------------|---------------|
| shared_buffers | 128MB | 8GB | 25% of 32GB RAM |

OUTPUT:
1. postgresql.conf template with comments
2. PgBouncer configuration
3. VACUUM tuning recommendations
4. Statistics maintenance schedule
5. Query plan inspection guidance
```

---

## Phase 5: Algorithm Selection

### Agent 5.1: Geodesic Algorithms

```
TASK: Select algorithms for geodesic calculations

ALGORITHMS TO EVALUATE:
| Algorithm | Accuracy | Speed | Use Case |
|-----------|----------|-------|----------|
| Haversine | ±0.5% | Fast | Short distances (<100km) |
| Vincenty | ±0.0003% | Medium | Medium distances |
| Karney (GeographicLib) | ±15nm | Slow | High precision needed |

CURRENT STATE (from 1-REPORT, 2-REPORT):
- Using Haversine for all distances
- Interpolation errors at high latitudes

RECOMMENDATIONS:
- Which algorithm for which use case
- Library recommendations (Rust and Python)
- Migration path from current implementation

OUTPUT:
1. Algorithm selection matrix
2. Library version specifications
3. API design for new functions
4. Benchmark expectations
```

### Agent 5.2: Track Processing Algorithms

```
TASK: Select algorithms for track segmentation and processing

ALGORITHMS TO EVALUATE:
1. Segmentation: time-gap, distance-gap, speed-threshold, port detection
2. Simplification: Douglas-Peucker, Visvalingam-Whyatt, radial distance
3. Interpolation: linear, cubic spline, geodesic
4. Smoothing: moving average, Kalman filter, Savitzky-Golay

FOR EACH ALGORITHM:
- Time complexity
- Space complexity
- Quality characteristics
- Library availability (prefer existing implementations)

OUTPUT:
1. Algorithm selection with justification
2. Library recommendations with versions
3. Custom implementation needs (with justification)
4. Performance benchmarks
```

### Agent 5.3: Ingestion Algorithms

```
TASK: Select algorithms for high-performance data ingestion

PATTERNS TO EVALUATE:
1. Batch sizing: fixed vs adaptive
2. Parallelism: file-level vs chunk-level
3. Buffering: ring buffer, SPSC channel
4. Backpressure: blocking vs dropping

STATE-OF-THE-ART REFERENCES:
- LMAX Disruptor pattern (lock-free ring buffer)
- Tokio async runtime (Rust)
- PostgreSQL COPY protocol
- Event sourcing for crash recovery

OUTPUT:
1. Ingestion pipeline architecture
2. Algorithm selection for each stage
3. Library recommendations
4. Performance targets with justification
```

---

## Phase 6: Implementation Roadmap

### Agent 6.1: Phased Implementation Plan

```
TASK: Create actionable implementation phases

PHASE STRUCTURE:
For each phase:
- Objective (one sentence)
- Tasks (prioritized list)
- Dependencies (what must be done first)
- Deliverables (concrete outputs)
- Verification (how to confirm completion)
- Risk factors

PHASES (suggested, adjust as needed):
1. Pruning - Remove SQLite, visualization, legacy code
2. Rust Core - Implement vectorized functions
3. Database - Schema migration, indexes, tuning
4. Ingestion - High-performance pipeline
5. Integration - End-to-end testing
6. Documentation - API docs, deployment guide

OUTPUT FORMAT:
### Phase N: [Name]

**Objective:** [One sentence]

**Tasks:**
| Task | Priority | Effort | Impact |
|------|----------|--------|--------|
| ... | HIGH | 2 days | ... |

**Dependencies:** [List]

**Deliverables:**
- [ ] Concrete output 1
- [ ] Concrete output 2

**Verification:**
```bash
# Commands to verify phase completion
```
```

### Agent 6.2: Risk Assessment

```
TASK: Identify and mitigate implementation risks

RISK CATEGORIES:
1. Technical risks (complexity, unknowns)
2. Performance risks (may not achieve targets)
3. Compatibility risks (breaking changes)
4. Resource risks (time, skills)

FOR EACH RISK:
| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| ... | HIGH | HIGH | ... |

OUTPUT:
1. Risk matrix
2. Mitigation strategies
3. Fallback options
4. Go/no-go decision criteria
```

---

## Phase 7: Verification and Validation

### Agent 7.1: Verification Scripts

```
TASK: Create comprehensive verification scripts

SCRIPTS NEEDED:
1. SQLite removal verification
2. Visualization removal verification
3. Schema validation
4. Index effectiveness testing
5. Performance benchmarks
6. Integration tests

FOR EACH SCRIPT:
- Purpose
- Commands
- Expected output
- Pass/fail criteria

OUTPUT: Complete bash scripts with documentation
```

### Agent 7.2: Performance Benchmarks

```
TASK: Define performance benchmarks and targets

BENCHMARK CATEGORIES:
1. Ingestion throughput (rows/second)
2. Query latency (p50, p95, p99)
3. Memory usage (peak, steady-state)
4. Storage efficiency (bytes/row)

FOR EACH BENCHMARK:
| Metric | Current | Target | Method |
|--------|---------|--------|--------|
| Bulk insert | 50K/sec | 500K/sec | COPY binary, 100K batch |

OUTPUT:
1. Benchmark definitions
2. Measurement methodology
3. Target values with justification
4. Regression detection thresholds
```

---

## Phase 8: Report Assembly and Merge

### Agent 8.1: Existing Report Analysis (If Exists)

```
TASK: If 4-REPORT.md exists, analyze and merge

IF FILE EXISTS:
1. Read existing 4-REPORT.md completely
2. Compare with newly generated content
3. For each section, decide:
   - KEEP existing (if more accurate/complete)
   - REPLACE with new (if new analysis is better)
   - MERGE (combine best of both)

MERGE DECISION MATRIX:
| Scenario | Action |
|----------|--------|
| New finding, not in existing | ADD |
| Same finding, new has more detail | REPLACE |
| Same finding, existing has more detail | KEEP |
| Contradictory findings | INVESTIGATE, document resolution |

4. Document all merge decisions in 4-CHANGELOG.md

OUTPUT:
- Final merged 4-REPORT.md
- Merge decision log
- Contradiction resolutions
```

### Agent 8.2: Cross-Report Consistency Check

```
TASK: Ensure 4-REPORT recommendations align with 0, 1, 2 findings

CHECK FOR:
1. All bugs from 1-REPORT addressed or acknowledged
2. All bad decisions from 2-REPORT remediated or justified
3. Architecture from 0-REPORT accurately reflected
4. No contradictions with source reports

FOR EACH DISCREPANCY:
- Document the conflict
- Provide resolution
- Update 4-REPORT if needed

OUTPUT: Consistency verification report
```

---

## Agent Coordination Protocol

### Multi-Agent Workflow

```
COORDINATION RULES:
1. Phase 1 (Input Reading) must complete before Phase 2
2. Phase 2 (Pruning) must complete before Phase 3-5
3. Phases 3, 4, 5 can run in parallel
4. Phase 6 depends on 3, 4, 5
5. Phase 7 depends on 6
6. Phase 8 runs last

CROSS-CHECKING:
- After each phase, validate findings with previous phases
- Resolve contradictions before proceeding
- Document all cross-agent validations

DEBUGGING:
- If an agent produces unexpected results, re-run with narrower scope
- If contradictions found, spawn focused investigation agent
- All final conclusions must be verified against source code
```

### Output Quality Gates

```
QUALITY REQUIREMENTS:
1. Every code reference verified against actual files
2. Every line number confirmed accurate
3. Every algorithm named must exist in referenced library
4. Every SQL statement must be syntactically valid
5. Every diagram must be renderable

VERIFICATION COMMANDS:
- Code exists: Use Grep/Read to confirm
- Line numbers: Use Read with offset to verify
- SQL valid: Use psql syntax check
- Libraries exist: Check crates.io/pypi
```

---

## Changelog Management

### 4-CHANGELOG.md Update Protocol

```
ON EACH RUN:
1. Check if 4-CHANGELOG.md exists
2. If exists, read current content
3. Add new entry at top with:

## [Run YYYY-MM-DD HH:MM] - Report Version X.X.X

### Summary
Brief description of this analysis run.

### Changes
- [ADDED] New section/finding description
- [UPDATED] Section X.X: change description
- [REMOVED] Obsolete content description
- [MERGED] Content from existing report

### Statistics
- Sections Added: N
- Sections Updated: N
- Sections Removed: N
- Total Report Size: N lines

### Source Reports Used
- 0-REPORT.md: version/date
- 1-REPORT.md: version/date
- 2-REPORT.md: version/date

### Git State
- Branch: [name]
- Last Commit: [hash] - [message]
```

---

## Final Report Structure

The generated 4-REPORT.md should follow this structure:

```
# AISdb-Lite Engineering Blueprint: High-Performance PostgreSQL-Only AIS Pipeline

## Executive Summary
- Performance targets
- Document structure overview

# PART I: COMPONENT PRUNING
## 1. SQLite Removal Plan
## 2. Visualization Removal Plan
## 3. Legacy Abstraction Removal

# PART II: RUST VS PYTHON ARCHITECTURE
## 4. Rust Migration Strategy
## 5. Python Retention Strategy
## 6. Interface Design

# PART III: DATABASE ARCHITECTURE
## 7. Schema Design
## 8. Index Architecture
## 9. Partitioning Strategy
## 10. PostgreSQL Configuration

# PART IV: STATE-OF-THE-ART ALGORITHMS
## 11. Geodesic Algorithms
## 12. Track Processing Algorithms
## 13. Ingestion Algorithms
## 14. Spatial Indexing

# PART V: ARCHITECTURE DIAGRAMS
## 15. Target System Architecture
## 16. Data Flow Diagram
## 17. Database Schema Diagram

# PART VI: IMPLEMENTATION ROADMAP
## 18. Phased Implementation Plan
## 19. Risk Assessment
## 20. Verification Scripts
## 21. Performance Benchmarks

## 22. Summary
- Key recommendations recap
- Critical path items
- Success criteria
```

---

## Deployment Target Constraints

**CRITICAL**: All recommendations MUST be optimized for:

```
DEPLOYMENT TARGET: Single Fixed Machine

Assumptions:
- CPU: 8-16 cores (modern x86_64)
- RAM: 32-64 GB
- Storage: NVMe SSD, 1-10 TB
- Network: Gigabit Ethernet
- OS: Linux (Ubuntu 22.04 or similar)

NOT designing for:
- Kubernetes/container orchestration
- Multi-node clusters
- Cloud-specific services (RDS, BigQuery, etc.)
- Horizontal scaling

Optimization focus:
- Maximum single-machine throughput
- Efficient resource utilization
- Minimal operational complexity
- Best possible performance on this hardware
```

---

## Library Preference Rules

**MANDATORY**: Always prefer existing, well-maintained libraries over custom implementations.

```
DECISION FRAMEWORK:
1. Does a standard library function exist? → USE IT
2. Does a well-known third-party library exist? → USE IT
3. Is custom implementation clearly justified? → DOCUMENT WHY

JUSTIFICATION REQUIRED FOR CUSTOM CODE:
- No suitable library exists (name libraries evaluated)
- Library has critical bug/limitation (document it)
- Performance requirement not met (show benchmarks)
- License incompatibility (specify licenses)

LIBRARY DOCUMENTATION FORMAT:
| Purpose | Library | Version | Why This Library |
|---------|---------|---------|------------------|
| Geodesic math | geographiclib-rs | 0.2.3 | Karney algorithm, maintained |
| Spatial index | geo | 0.27.0 | R-tree, PostGIS compatible |
```

---

## Execution Instructions

1. **Read input reports** (0-REPORT, 1-REPORT, 2-REPORT) completely
2. **Execute Phase 2 (Pruning) first** - identify ALL removals before additions
3. **Execute Phases 3-5** - Architecture, Database, Algorithms
4. **Execute Phase 6** - Implementation roadmap
5. **Execute Phase 7** - Verification and benchmarks
6. **Execute Phase 8** - Merge with existing 4-REPORT.md if present
7. **Update 4-CHANGELOG.md** with run summary
8. **Final validation** - Cross-check against source reports

---

*This prompt generates the engineering blueprint for transforming AISdb-lite into a high-performance, PostgreSQL-only AIS data pipeline.*
