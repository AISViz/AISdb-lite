# AISdb-Lite: Comprehensive Bad Business Decisions Analysis Prompt

> **Prompt Version**: 1.1.0
> **Target Report**: 2-REPORT.md
> **Analysis Type**: Strategic and Architectural Decision Assessment
> **Last Updated**: December 2025

---

## Overview

This prompt orchestrates a systematic analysis of **bad business decisions** in the AISdb-lite repository using 10 specialized exploration agents. Unlike bug analysis (which focuses on implementation errors), this analysis examines **strategic and architectural decisions** that fundamentally compromise:

- **Reliability** - System stability and uptime
- **Scalability** - Ability to handle growth
- **Maintainability** - Long-term code health
- **Correctness** - Mathematical and logical accuracy
- **Security** - Protection of data and systems
- **Operability** - Production deployment and monitoring

Each identified issue must represent a **design decision** (not a simple bug) that has systemic implications.

---

## Report Writing Guidelines

### No Page Limit
- **There is NO page limit** for the report
- Document every architectural flaw with full analysis
- Include remediation recommendations for each issue

### Avoid Duplications
- One section per bad decision - no repeated analysis
- If a decision affects multiple areas, document once and cross-reference
- Reference related bug IDs from 1-REPORT.md when applicable
- Use "See Section X.X" for related architectural issues

### Reduce Verbosity
- State the decision, then explain why it's problematic
- Use numbered lists for "Why This Is Bad" explanations (max 4-5 points)
- Keep code examples minimal - show the pattern, not the full implementation
- Distinguish clearly between ACTUAL code and ILLUSTRATIVE examples
- No preamble: "This section examines..." - just start with the finding

### Traceability Requirements
Every bad decision MUST include:

```
REQUIRED FOR EACH ISSUE:
├── Section ID: Part.Section (e.g., 1.1, 2.3)
├── Location: file path(s) where decision is implemented
├── Decision: one sentence describing what was decided
├── Code: actual code OR clearly marked ILLUSTRATIVE example
├── Problems: numbered list of why this is bad (with evidence)
├── Correct approach: what should have been done
└── Related: links to bugs (1-REPORT) or other sections

EXAMPLE:
### 1.1 Floating-Point Primary Key Design

**Location:** `aisdb/aisdb_sql/timescale_createtable_dynamic.sql:5-10`

**Decision:** Use latitude/longitude (DOUBLE PRECISION) in composite primary key.

```sql
-- ACTUAL CODE from timescale_createtable_dynamic.sql
PRIMARY KEY (mmsi, time, latitude, longitude)
```

**Why This Is A Bad Decision:**
1. IEEE 754 floating-point has no equality guarantee (`0.1 + 0.2 ≠ 0.3`)
2. B-tree indexes require total ordering; floats violate this (NaN ≠ NaN)
3. UPSERT operations fail unpredictably on "same" coordinates
4. Deduplication impossible - identical positions may not match

**Correct Approach:** Use integer geohash or scaled integers (microdegrees).

**Related:** See 1-REPORT PYDB-001 (SQL injection in queries using these tables)
```

### Distinguishing Code Types
- **ACTUAL CODE**: Direct copy from codebase - include file:line reference
- **ILLUSTRATIVE**: Pattern demonstration - clearly mark with comment:
  ```python
  # ILLUSTRATIVE EXAMPLE - demonstrates pattern, not actual code
  ```

### Writing Style
- Decision-first: state what was decided before analyzing it
- Evidence-based: every "why it's bad" point needs code/documentation support
- Actionable: remediation must be specific, not generic advice
- No hedging: "This decision causes X" not "This decision may cause X"

---

## Pre-Analysis Protocol

Before executing analysis, follow this protocol to ensure incremental updates and avoid duplications:

### Step 1: Check for Existing Report

```
1. Check if 2-REPORT.md exists in the repository root
2. If it exists:
   a. Read the entire report carefully
   b. Note all existing decision IDs (e.g., Section 1.1, Section 2.3)
   c. Note all CORRECTION markers and their reasoning
   d. Note the last update date and total issue count
3. If it doesn't exist:
   a. Create a fresh report following the template structure below
```

### Step 2: Check Changelog

```
1. Check if 2-CHANGELOG.md exists
2. If it exists:
   a. Read the latest entry to understand recent changes
   b. Note any issues previously marked as RESOLVED
   c. Note any issues previously identified as INVALID
3. If it doesn't exist:
   a. Create it following the template in this prompt
```

### Step 3: Determine Analysis Scope

```
1. If first run: Full comprehensive analysis
2. If subsequent run:
   a. Run git diff to identify changed files since last analysis
   b. Focus detailed analysis on changed files
   c. Perform spot-check verification on unchanged sections
   d. Re-verify all CRITICAL severity issues still exist
```

---

## Duplication Avoidance Protocol

When analyzing and documenting issues:

### Issue Identification

```
- Use the format: Part X: Section Title → Subsection X.X: Description
- Categories by Part:
  Part 1: Database Layer Decisions
  Part 2: Data Processing Pipeline Decisions
  Part 3: Rust Data Handling Decisions
  Part 4: Web Data Services Decisions
  Part 5: Frontend Data Handling Decisions
  Part 6: Spatial Indexing Decisions
  Part 7: Data Ingestion Decisions
  Part 8: Configuration and Testing Decisions
  Part 9: Receiver and Real-Time Streaming Decisions
  Part 10: Cross-Language Data Model Decisions
  Part 11: Documentation and API Design Decisions
  Part 12: Cross-Cutting Concerns
  Part 13: Priority Remediation Roadmap (Summary only)
```

### Content Comparison

```
Before adding an issue:
1. Check if the same file/location is already documented
2. Check if the same architectural concern is documented (may be in different section)
3. If duplicate found:
   a. Update the existing entry with new information
   b. Mark as [UPDATED] in changelog
4. If truly new:
   a. Add to appropriate Part/Section
   b. Mark as [ADDITION] in changelog
```

### Invalid Issue Handling

```
If a previously reported issue is found to be invalid:
1. Mark the section title with: ~~Original title~~ (INVALID - CORRECTED)
2. Add **CORRECTION NOTE:** explaining the correction
3. Provide accurate information about actual behavior
4. DO NOT delete the entry - keep for reference
5. Update summary statistics
```

---

## Agent Execution Framework

### Agent 1: Database Layer Decisions Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb/aisdb_sql/*.sql` (Schema definitions)
- `aisdb/database/dbconn.py` (Connection management)
- `aisdb/database/dbqry.py` (Query patterns)
- `aisdb/database/sql_query_strings.py` (Query construction)

**Decision Categories to Assess:**
- Primary key design (floating-point in keys?)
- Timestamp data types (32-bit vs 64-bit)
- SQL construction patterns (injection risks?)
- Connection management strategy (pooling?)
- Query patterns (N+1 queries?)
- Conflict resolution (ON CONFLICT handling)
- Index strategy (missing indexes?)
- Transaction boundaries (too large/small?)

**Investigation Approach:**
```sql
-- Check primary key design
Search for PRIMARY KEY, UNIQUE constraints with float columns

-- Check timestamp types
Look for INTEGER vs BIGINT for time columns

-- Check SQL construction
Search for f-strings, string interpolation in SQL

-- Check index coverage
Compare WHERE clauses with existing indexes
```

---

### Agent 2: Data Processing Pipeline Decisions Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb/track_gen.py` (Track generation)
- `aisdb/interp.py` (Interpolation)
- `aisdb/gis.py` (Geospatial operations)
- `aisdb/proc_util.py` (Processing utilities)
- `aisdb/denoising_encoder.py` (Encoding)

**Decision Categories to Assess:**
- Data structure choices (dict vs. structured arrays)
- Mathematical correctness (spherical vs. planar operations)
- Projection handling (hardcoded vs. configurable)
- Memory management (bounded vs. unbounded growth)
- Algorithm efficiency (vectorized vs. scalar)
- Index alignment (static vs. dynamic data)

**Investigation Approach:**
```python
# Check data structure patterns
Search for dict( with many fields - memory overhead

# Check mathematical operations
Look for np.interp on lat/lon - planar interpolation on sphere

# Check projection handling
Search for hardcoded EPSG codes

# Check memory patterns
Look for list.append in loops without bounds
```

---

### Agent 3: Rust Data Handling Decisions Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb_lib/src/` (Core library)
- `receiver/src/` (AIS receiver)
- `database_server/src/` (Database server)

**Decision Categories to Assess:**
- Error handling philosophy (panic vs. Result)
- Type casting strategy (lossy casts)
- Batch processing approach (configurable sizes?)
- Buffer management (fixed vs. adaptive)
- Resource cleanup (early returns, leaks)
- FFI boundary handling (PyO3 panics)

**Investigation Approach:**
```rust
// Check error handling patterns
Search for .unwrap(), .expect(), panic!

// Check type casts
Look for "as i32", "as f32" - potential data loss

// Check batch handling
Search for hardcoded buffer/batch sizes

// Check return patterns
Look for early returns in loops (? operator)
```

---

### Agent 4: Web Data Services Decisions Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/webdata/*.py` (Web scraping)
- `aisdb/weather/*.py` (Weather data integration)

**Decision Categories to Assess:**
- Rate limiting strategy (exists? effective?)
- Exception handling (blanket except?)
- Coordinate handling (lat/lon swaps?)
- Caching strategy (none? redundant fetches?)
- External API integration (credential handling?)
- Download resilience (resume capability?)

**Investigation Approach:**
```python
# Check rate limiting
Search for time.sleep, rate limiting patterns

# Check exception handling
Look for "except:" without specific exception

# Check coordinate usage
Verify lat/lon variable assignment consistency

# Check caching
Look for redundant fetch patterns
```

---

### Agent 5: Frontend Data Handling Decisions Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb_web/map/*.js` (JavaScript modules)
- `aisdb_web/map/*.ts` (TypeScript modules)

**Decision Categories to Assess:**
- Event handler implementation (typos?)
- Client-side storage (IndexedDB patterns)
- Memory management (unbounded object growth?)
- Security (XSS via innerHTML?)
- WebSocket lifecycle (cleanup on unload?)
- State management (race conditions?)

**Investigation Approach:**
```javascript
// Check event handlers
Search for misspelled event names (onbefureunload)

// Check DOM security
Look for innerHTML, insertAdjacentHTML with user data

// Check memory patterns
Look for module-level objects that grow unbounded

// Check storage patterns
Review IndexedDB transaction handling
```

---

### Agent 6: Spatial Indexing Decisions Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/discretize/*.py` (Spatial discretization)
- `aisdb/gis.py` (GIS operations)
- Database spatial patterns

**Decision Categories to Assess:**
- H3/geohash integration (computed but stored?)
- Projection selection (hardcoded UTM zones?)
- Spatial query efficiency (brute-force vs. indexed?)
- Coordinate validation (range checking?)
- PostGIS utilization (installed but unused?)

**Investigation Approach:**
```python
# Check H3 integration
Look for h3 usage - is index persisted?

# Check projection handling
Search for hardcoded EPSG codes

# Check spatial algorithms
Look for nested loops over coordinates (O(n*m))

# Check validation
Search for coordinate assertions (np.all misuse?)
```

---

### Agent 7: Data Ingestion Decisions Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/database/decoder.py` (File decoding)
- `aisdb_lib/src/csvreader.rs` (CSV reading)
- `aisdb_lib/src/decode.rs` (Message decoding)

**Decision Categories to Assess:**
- Checksum strategy (partial file hash?)
- Duplicate detection defaults (skip_checksum=True?)
- Validation consistency (MMSI validation layers)
- Date/time handling (ETA year inference)
- Format detection (content vs. extension based)
- Error recovery (early return vs. accumulate)

**Investigation Approach:**
```python
# Check checksum implementation
Look for partial file reads in hash functions

# Check default parameters
Search for skip_checksum, default values

# Check validation
Compare MMSI validation in Rust vs Python

# Check format detection
Look for extension-only file type detection
```

---

### Agent 8: Configuration and Testing Decisions Analyzer

**Thoroughness:** medium
**Focus Areas:**
- `aisdb/tests/*.py` (Test suite)
- `pyproject.toml` (Package configuration)
- `Dockerfile` (Container configuration)
- `.github/workflows/*.yml` (CI configuration)

**Decision Categories to Assess:**
- Test isolation (shared vs. isolated fixtures)
- Assertion usage (assert vs. explicit validation)
- Test coverage ratio (unit vs. integration)
- Test parameterization (duplicated test files)
- Error suppression in tests (silent passes)
- Container functionality (entrypoint correctness)
- Package contents (test data in distribution?)

**Investigation Approach:**
```python
# Check test patterns
Look for database requirements in every test

# Check assertion usage
Search for assert used for validation, not testing

# Check test duplication
Compare paired test files for copy-paste

# Check error handling in tests
Look for except blocks that print instead of fail
```

---

### Agent 9: Receiver and Real-Time Streaming Decisions Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `receiver/src/receiver.rs` (AIS receiver)
- `database_server/src/main.rs` (Server)

**Decision Categories to Assess:**
- I/O architecture (blocking vs. async)
- Backpressure handling (buffer overflow protection?)
- Buffer sizing (fixed vs. adaptive)
- Thread management (bounded vs. unbounded spawning)
- Error recovery (crash vs. graceful degradation)
- Security (TLS/SSL implementation?)
- Observability (metrics, logging?)

**Investigation Approach:**
```rust
// Check I/O patterns
Look for blocking recv_from followed by blocking DB ops

// Check buffer management
Search for fixed buffer sizes, SO_RCVBUF configuration

// Check thread patterns
Look for unbounded spawn() calls

// Check TLS
Search for TODO: SSL comments, plaintext connections
```

---

### Agent 10: Cross-Language Data Model Decisions Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- Type definitions across Rust, Python, TypeScript, SQL
- Interface boundaries (FFI, JSON, Database)

**Decision Categories to Assess:**
- Timestamp consistency (i32/i64/uint32 across layers)
- Precision preservation (f64 → f32 → f64 chain)
- NULL handling philosophy (default to zero vs. preserve NULL)
- Field naming conventions (cross-language mapping)
- Schema versioning (evolution strategy)

**Investigation Approach:**
```
// Trace data flow
Follow a value from NMEA decode → Rust → Python → SQL → JSON → TypeScript

// Check type definitions
Compare struct fields across languages

// Check NULL handling
Look for unwrap_or_default() patterns

// Check field names
Map Rust field names to SQL columns to Python keys
```

---

## Report Structure Template

The report must follow this exact structure:

```markdown
# AISdb-Lite: Comprehensive Analysis of Bad Business Decisions
## Data Storage, Management, and Handling Assessment

**Project:** AISdb-Lite v[VERSION]
**Analysis Date:** [Month Year]
**Scope:** Architectural decisions, data handling patterns, storage strategies, and systemic design flaws

> **CORRECTION NOTE ([Date])**: [Any corrections from cross-verification]

---

## Executive Summary

[Summary paragraph describing the analysis scope and key findings]

### Critical Finding Categories

| Category | Severity | Count | Impact |
|----------|----------|-------|--------|
| **Data Integrity** | Critical | N+ | [Description] |
| **Architecture** | Critical | N+ | [Description] |
| [Additional categories...] |

---

## Part 1: Database Layer Decisions

### 1.1 [Decision Title]

**Location:** `path/to/file`
**Decision:** [What decision was made]

```[language]
[Relevant code snippet]
```

**Why This Is A Bad Business Decision:**

1. [Reason 1]
2. [Reason 2]
3. [Reason 3]
4. [Reason 4]

**Correct Decision Would Be:**
- [Alternative approach 1]
- [Alternative approach 2]
- [Alternative approach 3]

---

[Continue for Parts 2-12]

---

## Part 13: Priority Remediation Roadmap

### Critical (Immediate Action Required)

| Issue | Impact | Effort | Fix |
|-------|--------|--------|-----|
| [Issue] | [Impact] | [Effort] | [Fix approach] |

### High Priority (Next Sprint)
[Table format]

### Medium Priority (Next Quarter)
[Table format]

### Low Priority (Technical Debt)
[Table format]

---

## Appendices

### Appendix A: Code Locations Reference
[Table mapping issues to files and line numbers]

### Appendix B: Severity Definitions
- **Critical**: System compromise, data loss, or security breach imminent
- **High**: Significant impact on reliability, correctness, or scalability
- **Medium**: Noticeable degradation in maintainability or performance
- **Low**: Technical debt with minor current impact

### Appendix C: Analysis Methodology
[Description of analysis approach]

### Appendix D: Impact Summary by Severity
[Statistics table]

---

*Report generated by multi-agent analysis system*
*AISdb-Lite Bad Business Decisions Assessment*
*[Month Year]*
```

---

## Severity Classification Guide

| Severity | Criteria |
|----------|----------|
| **Critical** | Fundamental design flaw causing data corruption, security vulnerability, or system-wide failure potential |
| **High** | Architectural decision significantly impacting reliability, scalability, or correctness |
| **Medium** | Design choice creating maintenance burden, technical debt, or operational challenges |
| **Low** | Suboptimal decision with minor impact, typically easy to remediate |

---

## Decision Category Definitions

| Category | Description |
|----------|-------------|
| Data Integrity | Decisions affecting data accuracy, consistency, and preservation |
| Architecture | Structural decisions affecting system organization and scalability |
| Security | Decisions affecting authentication, authorization, and data protection |
| Scalability | Decisions affecting ability to handle growth in data or users |
| Correctness | Decisions affecting mathematical or logical accuracy |
| Maintainability | Decisions affecting long-term code health and developer productivity |
| Testing | Decisions affecting test effectiveness and coverage |
| Documentation | Decisions affecting API contracts and knowledge transfer |
| Operability | Decisions affecting deployment, monitoring, and incident response |

---

## Illustrative Code Examples Protocol

When documenting bad decisions, distinguish between:

### Actual Code
```python
# From actual file - path/to/file.py line 123
actual_code_from_repository()
```

### Illustrative Example
```python
# ILLUSTRATIVE EXAMPLE - Demonstrates the pattern, not actual code
# The actual implementation shows this anti-pattern in [description]
illustrative_code_showing_pattern()
```

Always mark illustrative examples clearly to avoid confusion about what exists in the codebase.

---

## Changelog Entry Template

After completing analysis, update 2-CHANGELOG.md with:

```markdown
## [Run YYYY-MM-DD HH:MM] - Report Version X.X.X

### Summary
Brief description of this analysis run.

### New Issues Found
- [ADDITION] Part X, Section X.X: Brief description

### Issues Resolved (Verified Fixed)
- [RESOLVED] Part X, Section X.X: Brief description of fix

### Issues Updated
- [UPDATED] Part X, Section X.X: What changed

### Invalid Issues Identified
- [INVALID] Part X, Section X.X: Why it's not actually a bad decision

### Issues Re-Verified (Still Present)
- [VERIFIED] List of sections confirmed still present

### Statistics
- Total Issues: [Current count]
- Changes from Previous: +[new] -[resolved] ~[updated]

### Git State
- Branch: [name]
- Last Commit: [hash] - [message]
```

---

## Cross-Report Verification Protocol

After completing initial analysis, cross-verify against other analysis reports:

### Check Against 0-REPORT.md (Architecture Documentation)
- Verify file paths and structures match actual codebase
- Confirm function signatures and class definitions are accurate
- Check if documented features actually exist

### Check Against 1-REPORT.md (Bug Analysis)
- Ensure issues aren't duplicated as bugs AND bad decisions
- Reference related bugs where architectural decision causes the bug
- Note when a bad decision manifests as multiple bugs

### Check Against CONTRADICTIONS-ANALYSIS.md (if exists)
- Look for already-identified invalid issues
- Look for confirmed issues

### Self-Verification Checklist
- [ ] All file paths verified to exist
- [ ] All code snippets match actual code (or clearly marked illustrative)
- [ ] All severity ratings justified
- [ ] No duplicate issues across Parts
- [ ] Summary statistics match detailed counts
- [ ] Remediation roadmap prioritized correctly

---

## Execution Instructions

### For Fresh Analysis (No Existing Report)

1. Execute all 10 agents in sequence
2. Compile findings into 2-REPORT.md using template
3. Create 2-CHANGELOG.md with initial entry
4. Cross-verify code examples against actual files
5. Perform cross-report verification

### For Incremental Analysis (Report Exists)

1. Read existing 2-REPORT.md and 2-CHANGELOG.md
2. Check git changes since last analysis
3. Run relevant agents on changed files
4. Re-verify CRITICAL severity issues still exist
5. Spot-check 10% of other issues
6. Update report with changes only
7. Add changelog entry with changes

### Quality Gates

Before finalizing the report:
- [ ] All code examples verified against actual files
- [ ] Illustrative examples clearly marked
- [ ] No duplicate issues
- [ ] All file paths exist
- [ ] Statistics match line-by-line count
- [ ] Cross-language issues traced through all layers

---

## Difference from Bug Analysis (1-REPORT.md)

| Aspect | Bug Report (1-REPORT) | Bad Decisions Report (2-REPORT) |
|--------|----------------------|--------------------------------|
| Focus | Implementation errors | Architectural/strategic choices |
| Scope | Individual code issues | System-wide design patterns |
| Fix | Patch the code | May require redesign |
| Impact | Local functionality | Systemic implications |
| Example | `if x > 0:` should be `>=` | Using float in primary key |

---

*This prompt is designed to produce consistent, verifiable bad business decisions analysis reports with minimal duplication across multiple runs.*
