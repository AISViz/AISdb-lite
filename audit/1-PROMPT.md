# AISdb-Lite: Comprehensive Bug Analysis Prompt

> **Prompt Version**: 1.1.0
> **Target Report**: 1-REPORT.md
> **Analysis Type**: Bug Discovery and Verification
> **Last Updated**: December 2025

---

## Overview

This prompt orchestrates a systematic bug analysis of the AISdb-lite repository using 10 specialized exploration agents. The analysis focuses exclusively on **real bugs** - not style suggestions, best practices, or potential improvements. Each identified bug must represent actual broken functionality, data corruption risk, crash potential, or security vulnerability.

---

## Report Writing Guidelines

### No Page Limit
- **There is NO page limit** for the report
- Document every confirmed bug with full detail
- Prioritize completeness - every bug must be fully documented

### Avoid Duplications
- Each bug gets ONE entry with a unique ID (e.g., RUST-001)
- Never document the same bug in multiple sections
- If a bug spans multiple files, document once and list all affected locations
- Cross-reference related bugs: "See also: RUST-003"

### Reduce Verbosity
- Lead with the bug, not the context
- One bug per entry - no compound bug descriptions
- Use code snippets (< 10 lines) not full function dumps
- Remove filler: "It appears that", "We found that", "Interestingly"
- State the bug directly: "Function X panics when Y is empty"

### Traceability Requirements
Every bug MUST include:

```
REQUIRED FOR EACH BUG:
├── Bug ID: CATEGORY-NNN (e.g., RUST-001)
├── File: exact path from repository root
├── Line(s): specific line numbers
├── Code: minimal snippet showing the bug (< 10 lines)
├── Problem: one sentence describing what's wrong
├── Impact: what happens when bug is triggered
└── Verify: command to reproduce or locate the bug

EXAMPLE:
### RUST-001: Early Return Terminates CSV Processing (CRITICAL)

**File:** `aisdb_lib/src/csvreader.rs:394-399`

```rust
None => {
    return Ok(());  // BUG: Exits entire function
}
```

**Problem:** Invalid timestamp causes `return Ok(())`, terminating all CSV processing.

**Impact:** Single malformed row loses all subsequent data silently.

**Verify:** `grep -n "return Ok(())" aisdb_lib/src/csvreader.rs`
```

### Writing Style
- Bug-first: state what's broken before explaining why
- Active voice: "The function crashes" not "A crash is caused"
- Definitive language: "This will panic" not "This might panic"
- No speculation: verify against code before documenting

---

## Pre-Analysis Protocol

Before executing bug analysis, follow this protocol to ensure incremental updates and avoid duplications:

### Step 1: Check for Existing Report

```
1. Check if 1-REPORT.md exists in the repository root
2. If it exists:
   a. Read the entire report carefully
   b. Note all existing bug IDs (e.g., RUST-001, PYDB-002, SQL-003)
   c. Note all FALSE POSITIVE markers and their reasoning
   d. Note the last update date and total bug count
3. If it doesn't exist:
   a. Create a fresh report following the template structure below
```

### Step 2: Check Changelog

```
1. Check if 1-CHANGELOG.md exists
2. If it exists:
   a. Read the latest entry to understand recent changes
   b. Note any bugs previously marked as FIXED
   c. Note any bugs previously identified as FALSE POSITIVE
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
   d. Re-verify all CRITICAL and HIGH severity bugs still exist
```

---

## Duplication Avoidance Protocol

When analyzing and documenting bugs:

### Bug ID Assignment

```
- Use the format: CATEGORY-NNN (e.g., RUST-001, PYDB-014)
- Categories: RUST, PYDB, SQL, TRACK, WEB, WEBDATA, TEST, BUILD, INT, DISC
- If a bug already exists with an ID, DO NOT reassign a new ID
- If a bug is fixed, mark it as [FIXED] but keep the ID reserved
- New bugs get the next sequential number in their category
```

### Content Comparison

```
Before adding a bug:
1. Check if the same file/line is already documented
2. Check if the same logical issue is documented (may be different line due to code changes)
3. If duplicate found:
   a. Update the existing entry with new line numbers if needed
   b. Mark as [UPDATED] in changelog
4. If truly new:
   a. Add to appropriate section
   b. Mark as [ADDITION] in changelog
```

### False Positive Handling

```
If a previously reported bug is found to be invalid:
1. Mark the section with: ~~Original title~~ (FALSE POSITIVE - REMOVED)
2. Add **Status:** FALSE POSITIVE - This is NOT a bug.
3. Provide detailed **Analysis:** explaining why
4. Add **Verdict:** with clear conclusion
5. DO NOT delete the entry - keep it for reference
6. Update summary statistics
```

---

## Agent Execution Framework

### Agent 1: Rust Crate Bug Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb/` (Rust library root)
- `aisdb_lib/src/` (Core library)
- `receiver/src/` (AIS receiver)
- `database_server/src/` (Server)
- `client_webassembly/src/` (WASM client)

**Bug Categories to Find:**
- `panic!()` and `unwrap()` calls without proper error handling
- Index out of bounds risks (array/vector access without length checks)
- Integer overflow/underflow (especially i64 → i32 casts)
- Unsafe UTF-8 conversions (`from_utf8().unwrap()`)
- Early returns that terminate loops prematurely
- Resource leaks (unclosed connections, files)
- Race conditions in async code
- Memory safety issues

**Investigation Commands:**
```rust
// Search for panic-inducing patterns
grep -rn "unwrap()" --include="*.rs"
grep -rn "expect(" --include="*.rs"
grep -rn "panic!" --include="*.rs"
grep -rn "assert!" --include="*.rs"  // In non-test code

// Search for unsafe casts
grep -rn "as i32" --include="*.rs"
grep -rn "as i64" --include="*.rs"

// Search for index access without bounds checking
grep -rn "\[0\]" --include="*.rs"
grep -rn "\[.*\.len()" --include="*.rs"
```

---

### Agent 2: Python Database Layer Bug Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb/database/dbconn.py`
- `aisdb/database/dbqry.py`
- `aisdb/database/decoder.py`
- `aisdb/database/sql_query_strings.py`

**Bug Categories to Find:**
- SQL injection vulnerabilities (string interpolation in SQL)
- Parameter signature mismatches
- Unclosed database cursors (resource leaks)
- Mutable default arguments (`def foo(args=[])`)
- Variable scope issues (undefined names)
- Counter/index out of bounds risks
- Type coercion issues

**Investigation Commands:**
```python
# Search for SQL injection risks
grep -rn "f\".*{" --include="*.py" | grep -i sql
grep -rn "f'.*{" --include="*.py" | grep -i sql

# Search for mutable defaults
grep -rn "def.*=\[\]" --include="*.py"
grep -rn "def.*={}" --include="*.py"

# Search for unclosed cursors
grep -rn "\.cursor()" --include="*.py"
grep -rn "cur\.close()" --include="*.py"
```

---

### Agent 3: SQL File Bug Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/aisdb_sql/*.sql`

**Bug Categories to Find:**
- Wrong column references in UPSERT (ON CONFLICT DO UPDATE)
- Missing conflict target specifications
- Duplicate column selections
- Missing foreign key indexes
- Inconsistent NULL handling
- Invalid SQL syntax for target database (PostgreSQL vs SQLite)
- VACUUM in transaction blocks

**Investigation Commands:**
```sql
-- Check UPSERT patterns
grep -rn "excluded\." --include="*.sql"
grep -rn "ON CONFLICT" --include="*.sql"

-- Check for duplicate columns
grep -rn "SELECT" --include="*.sql" | sort | uniq -d

-- Check table alias usage
grep -rn "LEFT JOIN.*ref" --include="*.sql"
```

---

### Agent 4: Track Processing Bug Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- `aisdb/gis.py`
- `aisdb/proc_util.py`
- `aisdb/track_gen.py`
- `aisdb/interp.py`
- `aisdb/denoising_encoder.py`
- `aisdb/network_graph.py`

**Bug Categories to Find:**
- NumPy array dimension mismatches
- Invalid np.all()/np.any() usage in assertions
- Coordinate order swaps (lat/lon vs lon/lat)
- Division by zero (especially delta_t = 0)
- Empty array access (`arr[0]` without checking `len(arr)`)
- Index array vs boolean mask confusion
- Floating point comparison issues
- Off-by-one errors in loops

**Investigation Commands:**
```python
# Search for numpy assertions
grep -rn "np.all\|np.any" --include="*.py"

# Search for division operations
grep -rn "/ delta\|/delta" --include="*.py"

# Search for coordinate access patterns
grep -rn "haversine\|lat\[.*lon\[" --include="*.py"
```

---

### Agent 5: Web Frontend Bug Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb_web/map/*.js`
- `aisdb_web/map/*.ts`

**Bug Categories to Find:**
- Incorrect array indexing (JavaScript comma operator trap)
- Event handler misspellings
- DOM XSS vulnerabilities (innerHTML with untrusted data)
- Uninitialized variables
- Wrong UI messages (copy-paste errors)
- Race conditions in WebSocket handlers
- Memory leaks (unbounded object growth)
- Unhandled promise rejections

**Investigation Commands:**
```javascript
// Search for comma operator trap in array access
grep -rn "\[-1," --include="*.js"

// Search for event handlers
grep -rn "on[a-z]*unload" --include="*.js"

// Search for innerHTML usage
grep -rn "innerHTML\|innerText\|textContent.*=" --include="*.js"
```

---

### Agent 6: Webdata and Weather Bug Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/webdata/*.py`
- `aisdb/weather/*.py`

**Bug Categories to Find:**
- Wrong array indexing (lat vs lon swap)
- Bare except clauses that hide errors
- Silent failures (empty dict returned on error)
- Undefined variables after exceptions
- API client initialization failures
- Unclosed cursors and connections
- Resource leaks (temp directories)
- Missing key validation in dictionaries

**Investigation Commands:**
```python
# Search for bare except
grep -rn "except:" --include="*.py"

# Search for silent failure patterns
grep -rn "= {}\|= \[\]" --include="*.py" | grep try -A5

# Search for tempfile usage
grep -rn "mkdtemp\|TemporaryDirectory" --include="*.py"
```

---

### Agent 7: Test Suite Bug Analyzer

**Thoroughness:** medium
**Focus Areas:**
- `aisdb/tests/*.py`

**Bug Categories to Find:**
- Missing imports (NameError at runtime)
- Tests with no assertions (always pass)
- Incomplete assertions (only partial verification)
- Ambiguous exception handling (test passes on either success or specific exception)
- Wildcard import dependencies (fragile tests)
- Wrong exception types caught

**Investigation Commands:**
```python
# Search for tests without assertions
grep -rn "def test_" --include="*.py" -A20 | grep -v assert

# Search for wildcard imports
grep -rn "from .* import \*" --include="*.py"

# Search for exception handling in tests
grep -rn "except.*:" --include="test_*.py"
```

---

### Agent 8: Build Configuration Bug Analyzer

**Thoroughness:** medium
**Focus Areas:**
- `.github/workflows/*.yml`
- `Cargo.toml`, `*/Cargo.toml`
- `pyproject.toml`
- `rust-toolchain.toml`

**Bug Categories to Find:**
- CI branch mismatches (master vs main)
- Invalid TOML sections (wrong file type)
- Configuration typos (e.g., "compatability")
- Hardcoded paths (non-portable CI)
- Version string comparison bugs
- Wildcard version specifications
- Dependency conflicts (same crate, different versions)
- Incomplete step names

**Investigation Commands:**
```bash
# Check branch references
grep -rn "master\|main" .github/workflows/*.yml

# Check version specs
grep -rn '"\*"' */Cargo.toml
grep -rn "version" pyproject.toml

# Check for hardcoded paths
grep -rn "D:\\\|/home/runner" .github/workflows/*.yml
```

---

### Agent 9: Cross-Cutting Integration Bug Analyzer

**Thoroughness:** very thorough
**Focus Areas:**
- Interface points between Rust ↔ Python
- Interface points between Server ↔ Frontend
- Database schema vs. application code consistency
- Timestamp handling across all layers

**Bug Categories to Find:**
- Year 2038 problem (i32 timestamps)
- Type mismatches between layers
- WebSocket frame type mismatches
- Silent deserialization with panics on missing fields
- Error message text that doesn't match the check
- Floating point precision loss
- Hardcoded region-specific values

**Investigation Commands:**
```bash
# Check timestamp types
grep -rn "i32\|INTEGER" --include="*.rs" --include="*.sql" | grep -i time

# Check WebSocket message handling
grep -rn "Message::Binary\|Message::Text" --include="*.rs"
grep -rn "\.text()\|\.json()" --include="*.js"
```

---

### Agent 10: Discretization and Miscellaneous Bug Analyzer

**Thoroughness:** thorough
**Focus Areas:**
- `aisdb/discretize/*.py`
- `aisdb/web_interface.py`
- Any files not covered by other agents

**Bug Categories to Find:**
- Hardcoded coordinate systems (UTM zones, EPSGs)
- Missing return statements
- Missing cursor cleanup in generators
- Invalid function parameters
- Missing input validation
- Type annotation errors
- Unreachable code
- Module-level side effects

**Investigation Commands:**
```python
# Search for hardcoded EPSG codes
grep -rn "epsg=\|EPSG:" --include="*.py"

# Search for generators without cleanup
grep -rn "yield" --include="*.py"

# Search for type annotations
grep -rn "-> (" --include="*.py"  # Potentially invalid tuple syntax
```

---

## Report Structure Template

The report must follow this exact structure:

```markdown
# AISdb-Lite: Comprehensive Bug Analysis Report

> **Generated**: [Month Year]
> **Version Analyzed**: [Version from pyproject.toml]
> **Analysis Method**: 10 specialized exploration agents covering all code paths
> **Total Bugs Found**: [Count]
> **Critical Bugs**: [Count]
> **High Severity**: [Count]
> **Medium Severity**: [Count]
> **Low Severity**: [Count]
>
> **CORRECTION NOTE (Date)**: [Any corrections from cross-verification]

---

## Executive Summary

[Summary paragraph describing the analysis scope and key findings]

### Bug Distribution by Component

| Component | Critical | High | Medium | Low | Total |
|-----------|----------|------|--------|-----|-------|
| [Component] | N | N | N | N | N |

---

## Table of Contents

[Auto-generated from sections]

---

## 1. Rust Crate Bugs

### RUST-NNN: [Descriptive Title] (SEVERITY)

**File:** `path/to/file.rs`
**Lines:** [line numbers]

```rust
[Relevant code snippet]
```

**Problem:** [Clear explanation of what's wrong]

**Expected Behavior:** [What should happen instead]

**Impact:** [Consequences if bug is triggered]

---

[Repeat for all 10 sections]

---

## Summary Statistics

### By Severity
[Table with counts and percentages]

### By Category
[Table categorizing bugs by type: Data Corruption, Crash/Panic, Resource Leak, etc.]

### Priority Recommendations
[Ordered list of bugs to fix by priority]

---

## Appendix: Verification Commands

[Bash commands to verify bugs exist]
```

---

## Severity Classification Guide

| Severity | Criteria |
|----------|----------|
| **CRITICAL** | Data corruption, security vulnerability, silent data loss, system-wide failure |
| **HIGH** | Application crash, resource exhaustion, incorrect calculations, broken core functionality |
| **MEDIUM** | Resource leaks, silent failures, inconsistent behavior, test failures |
| **LOW** | Minor issues, cosmetic problems, edge cases unlikely to occur |

---

## Bug Classification Categories

| Category | Description |
|----------|-------------|
| Data Corruption | Wrong calculations, swapped coordinates, corrupted values |
| Crash/Panic | Unhandled exceptions, panics, application termination |
| Resource Leak | Unclosed cursors, memory leaks, file handle leaks |
| Silent Failure | Errors suppressed, data lost without indication |
| Type Mismatch | Wrong types between system layers |
| Logic Error | Off-by-one, wrong conditions, incorrect flow |
| Security | Injection vulnerabilities, XSS, unsafe operations |
| Build/Config | CI issues, dependency problems, configuration errors |

---

## Changelog Entry Template

After completing analysis, update 1-CHANGELOG.md with:

```markdown
## [Run YYYY-MM-DD HH:MM] - Report Version X.X.X

### Summary
Brief description of this analysis run.

### New Bugs Found
- [ADDITION] CATEGORY-NNN: Brief description

### Bugs Fixed (Verified Resolved)
- [FIXED] CATEGORY-NNN: Brief description of fix

### Bugs Updated
- [UPDATED] CATEGORY-NNN: What changed (line numbers, severity, etc.)

### False Positives Identified
- [FALSE POSITIVE] CATEGORY-NNN: Why it's not a bug

### Bugs Re-Verified (Still Present)
- [VERIFIED] List of bug IDs confirmed still present

### Statistics
- Total Bugs: [Current count]
- Changes from Previous: +[new] -[fixed] ~[updated]

### Git State
- Branch: [name]
- Last Commit: [hash] - [message]
```

---

## Cross-Report Verification Protocol

After completing initial analysis, cross-verify against other analysis reports in the repository:

1. **Check 0-REPORT.md** (Architecture Documentation)
   - Verify bug locations match documented architecture
   - Confirm function signatures match documentation
   - Check if documented features actually exist

2. **Check CONTRADICTIONS-ANALYSIS.md** (if exists)
   - Look for already-identified false positives
   - Look for confirmed bugs

3. **Self-Verification Checklist**
   - [ ] All bug IDs are unique
   - [ ] All file paths verified to exist
   - [ ] All line numbers verified accurate
   - [ ] All code snippets match actual code
   - [ ] All severity ratings justified
   - [ ] Summary statistics match detailed counts
   - [ ] Verification commands tested and working

---

## Execution Instructions

### For Fresh Analysis (No Existing Report)

1. Execute all 10 agents in sequence
2. Compile findings into 1-REPORT.md using template
3. Create 1-CHANGELOG.md with initial entry
4. Run verification commands to confirm bugs
5. Perform cross-report verification

### For Incremental Analysis (Report Exists)

1. Read existing 1-REPORT.md and 1-CHANGELOG.md
2. Check git changes since last analysis
3. Run relevant agents on changed files
4. Re-verify CRITICAL and HIGH bugs still exist
5. Spot-check 10% of other bugs
6. Update report with changes only
7. Add changelog entry with changes

### Quality Gates

Before finalizing the report:
- [ ] At least 5 verification commands tested
- [ ] No duplicate bug IDs
- [ ] All FALSE POSITIVE entries have analysis
- [ ] Statistics match line-by-line count
- [ ] All code snippets accurate to actual files

---

*This prompt is designed to produce consistent, verifiable bug analysis reports with minimal duplication across multiple runs.*
