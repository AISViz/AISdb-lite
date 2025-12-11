# AISdb-Lite: Cross-Report Contradiction Analysis & Reconciliation Prompt

> **Prompt Version**: 1.2.0
> **Target Report**: 3-REPORT.md
> **Analysis Type**: Cross-Report Contradiction Detection and Resolution
> **Last Updated**: December 2025

---

## Overview

This prompt orchestrates a systematic cross-report contradiction analysis of the AISdb-lite repository's analysis reports. The goal is to ensure **consistency across all three reports** (0-REPORT.md, 1-REPORT.md, and 2-REPORT.md) by:

1. **Detecting contradictions** between reports (unbiased fresh analysis)
2. **Verifying claims** against actual source code
3. **Reconciling differences** with authoritative findings
4. **Merging with existing findings** (only at the end, to avoid bias)
5. **Updating all reports** to maintain consistency

### Report Roles

| Report | Purpose | Focus |
|--------|---------|-------|
| **0-REPORT.md** | Architecture Documentation | What exists - structures, functions, APIs |
| **1-REPORT.md** | Bug Analysis | What's broken - implementation errors, defects |
| **2-REPORT.md** | Bad Business Decisions | What's poorly designed - architectural flaws |
| **3-REPORT.md** | Contradiction Analysis | Cross-validation and reconciliation |

---

## Report Writing Guidelines

### No Page Limit
- **There is NO page limit** for the report
- Document every contradiction discovered
- Include full reasoning for resolution decisions

### Avoid Duplications
- One entry per contradiction - use unique IDs (CONTRA-XX-NNN)
- Never report the same contradiction in multiple categories
- Cross-reference related contradictions: "See also: CONTRA-FN-002"
- If same issue appears in multiple reports, document once with all affected reports listed

### Reduce Verbosity
- State the contradiction first, then the resolution
- Use tables for comparisons across reports
- Keep verification evidence minimal but complete
- No narrative explanations: "Upon examining..." - just state facts
- Resolution reasoning in 1-2 sentences maximum

### Traceability Requirements
Every contradiction MUST include:

```
REQUIRED FOR EACH CONTRADICTION:
├── ID: CONTRA-[TYPE]-NNN (e.g., CONTRA-FP-001)
├── Reports Affected: which reports contain conflicting info
├── Contradiction: what differs (with quotes from each report)
├── Verification: source code check performed
├── Resolution: authoritative answer with file:line proof
├── Corrections: exact changes needed for each affected report
└── Status: NEW | VERIFIED | RESOLVED | REGRESSION

EXAMPLE:
### CONTRA-FP-001: load_raster.py Location

**Status:** RESOLVED
**Reports Affected:** 2-REPORT.md

**Contradiction:**
- 2-REPORT Section 4.3 states: `aisdb/weather/load_raster.py`

**Verification:**
```bash
$ ls aisdb/weather/load_raster.py
ls: cannot access: No such file or directory
$ ls aisdb/webdata/load_raster.py
aisdb/webdata/load_raster.py  # EXISTS
```

**Resolution:** Correct path is `aisdb/webdata/load_raster.py`

**Corrections:**
- 2-REPORT.md Section 4.3: Change `weather/` → `webdata/`
```

### Comparison Tables
Use tables to show discrepancies across reports:

```markdown
| Claim | 0-REPORT | 1-REPORT | 2-REPORT | Actual (Verified) |
|-------|----------|----------|----------|-------------------|
| TrackGen type | Class | N/A | N/A | Function |
| Interp methods | 6 | N/A | N/A | 4 |
```

### Writing Style
- Evidence-first: show the proof before the conclusion
- Definitive: "The file is at X" not "The file appears to be at X"
- Action-oriented: every contradiction ends with specific corrections
- Source code is authoritative: when reports conflict with code, code wins

---

## Critical Workflow: Unbiased Analysis First

### IMPORTANT: Analysis Order to Prevent Bias

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    UNBIASED ANALYSIS WORKFLOW                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  PHASE 1: FRESH ANALYSIS (DO NOT read existing 3-REPORT.md yet!)        │
│  ─────────────────────────────────────────────────────────────────────  │
│  1. Read source reports: 0-REPORT.md, 1-REPORT.md, 2-REPORT.md          │
│  2. Execute all 10 analysis agents                                       │
│  3. Verify claims against actual source code                             │
│  4. Document ALL contradictions found (fresh findings)                   │
│  5. Build complete list of corrections needed                            │
│                                                                          │
│  PHASE 2: MERGE WITH EXISTING (Only AFTER Phase 1 is complete)          │
│  ─────────────────────────────────────────────────────────────────────  │
│  6. NOW read existing 3-REPORT.md (if it exists)                         │
│  7. Compare fresh findings with existing documented contradictions       │
│  8. Reason over differences:                                             │
│     - New contradictions not previously documented → ADD                 │
│     - Previously documented, still present → VERIFY                      │
│     - Previously documented, now resolved → mark RESOLVED                │
│     - Previously documented, but incorrect → CORRECT                     │
│  9. Create unified 3-REPORT.md merging all findings                      │
│                                                                          │
│  PHASE 3: APPLY CORRECTIONS                                              │
│  ─────────────────────────────────────────────────────────────────────  │
│  10. Apply corrections to 0-REPORT.md, 1-REPORT.md, 2-REPORT.md         │
│  11. Update all changelogs                                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**WHY THIS ORDER MATTERS:**
- Reading the existing 3-REPORT.md first would bias the analysis toward confirming existing findings
- Fresh analysis may discover contradictions that were previously missed
- Fresh analysis may find that previously "resolved" items have regressed
- Only by analyzing independently first can we ensure objectivity

---

## Phase 1: Fresh Analysis Protocol

### Step 1: Read Source Reports ONLY

```
MANDATORY - Read these reports for analysis:
1. Read 0-REPORT.md - Architecture documentation
2. Read 1-REPORT.md - Bug analysis
3. Read 2-REPORT.md - Bad business decisions

DO NOT READ YET:
- 3-REPORT.md (existing contradiction analysis)
- 3-CHANGELOG.md (existing changelog)

This ensures unbiased fresh analysis of contradictions.
```

### Step 2: Build Cross-Reference Index

Create an index of all claims across the three source reports:

```
For each report (0, 1, 2), extract:
- File paths mentioned
- Function/class names documented
- Line number references
- Code snippets included
- Severity ratings (for 1-REPORT and 2-REPORT)
- Status markers (OPEN, FIXED, FALSE POSITIVE, etc.)
- Quantitative claims (counts, statistics)
```

### Step 3: Identify Potential Contradictions

Look for these contradiction types:

| Type | Code | Description | Example |
|------|------|-------------|---------|
| File Path | FP | Different paths for same file | `weather/` vs `webdata/` |
| Function Existence | FN | One report says exists, another says doesn't | TrackGen class vs function |
| Line Number | LN | Different line numbers for same code | Bug at line 61 vs line 60 |
| Code Snippet | CS | Different code shown for same location | Different function signatures |
| Severity | SV | Same issue with different severity ratings | CRITICAL vs HIGH |
| Status | ST | Conflicting status (bug vs not-a-bug) | FALSE POSITIVE disagreements |
| Feature Claim | FC | Different claims about what code does | SQLite support yes/no |
| Quantity | QT | Different counts for same metric | 4 vs 6 interpolation methods |

---

## Agent Execution Framework

### Agent 1: File Path Cross-Validator

**Thoroughness:** very thorough
**Focus:** Verify all file paths mentioned across reports exist

**Prompt:**
```
Cross-validate file paths across 0-REPORT.md, 1-REPORT.md, and 2-REPORT.md:

1. Extract ALL file paths mentioned in each report
2. Verify each path exists in the actual codebase using filesystem checks
3. Identify paths that:
   a. Are mentioned but don't exist
   b. Are mentioned with different paths in different reports
   c. Have moved since the report was written

For each discrepancy found, document:
- Which report(s) mention the path
- What the correct path is (verified against filesystem)
- The correction needed

Output format:
| Report | Claimed Path | Actual Path | Status |
```

---

### Agent 2: Function/Class Existence Validator

**Thoroughness:** very thorough
**Focus:** Verify all functions/classes mentioned across reports

**Prompt:**
```
Cross-validate function and class existence across all reports:

1. From 0-REPORT.md: Extract all documented functions and classes
2. From 1-REPORT.md: Extract all functions/classes where bugs are reported
3. From 2-REPORT.md: Extract all functions/classes with bad decisions

For each function/class:
- Verify it exists in the codebase using grep/search
- Verify its signature matches documentation
- Check if it's a function vs class
- Verify method existence on classes

Document contradictions:
| Item | 0-REPORT Claim | 1-REPORT Claim | 2-REPORT Claim | Actual (verified) |
```

---

### Agent 3: Line Number Accuracy Validator

**Thoroughness:** thorough
**Focus:** Verify line numbers in bug reports match actual code

**Prompt:**
```
Validate line number references across 1-REPORT.md and 2-REPORT.md:

1. For each bug/issue with line number reference:
   a. Read the file at that line
   b. Compare code snippet in report to actual code
   c. If different, find where the actual code is

2. Check if line numbers have drifted due to code changes since report

For each discrepancy:
| Bug ID | Claimed Line | Actual Line | Code Match | Notes |
```

---

### Agent 4: Code Snippet Accuracy Validator

**Thoroughness:** very thorough
**Focus:** Verify code snippets match actual source

**Prompt:**
```
Validate all code snippets in reports against actual source files:

1. For each code snippet in 0-REPORT (architecture docs):
   - Verify it matches the actual code
   - Flag any differences

2. For each code snippet in 1-REPORT (bugs):
   - Verify the bug exists as described
   - Check if marked illustrative vs actual code

3. For each code snippet in 2-REPORT (bad decisions):
   - Verify the pattern exists
   - Check if marked ILLUSTRATIVE vs actual

Document:
| Report | Section | Snippet Type | Matches Actual | Correction Needed |
```

---

### Agent 5: Cross-Report Claim Reconciler

**Thoroughness:** very thorough
**Focus:** Find claims that contradict between reports

**Prompt:**
```
Identify claims that contradict across reports:

SYSTEMATIC CHECK - For each claim in one report, verify against others:

1. **Type definitions**: Is it a class or function? (e.g., TrackGen)
2. **Method/function counts**: How many methods does X have? (e.g., interpolation)
3. **Algorithm details**: What algorithm is used? (e.g., MD5 vs SHA256)
4. **Feature existence**: Does feature X exist? (e.g., SQLite support)
5. **Test configuration**: What database do tests use?
6. **Safety features**: Does X have rate limiting/validation/etc.?
7. **File locations**: Where is X located?

For each contradiction:
1. Note which reports make conflicting claims
2. Verify against actual code
3. Determine authoritative answer
4. Document corrections needed for each report
```

---

### Agent 6: Severity Rating Reconciler

**Thoroughness:** thorough
**Focus:** Compare severity ratings between 1-REPORT and 2-REPORT

**Prompt:**
```
Compare severity classifications between bug report and bad decisions report:

For issues that appear in BOTH reports:
1. Extract severity from 1-REPORT (CRITICAL/HIGH/MEDIUM/LOW)
2. Extract severity from 2-REPORT (Critical/High/Medium/Low)
3. Compare and flag mismatches
4. Verify which severity is appropriate based on actual impact

Common issues appearing in both:
- SQL injection vulnerability
- Y2038 timestamp bug
- XSS vulnerability
- Floating-point primary key
- No TLS/SSL

Output:
| Issue | 1-REPORT ID | 1-REPORT Severity | 2-REPORT Section | 2-REPORT Severity | Reconciled |
```

---

### Agent 7: False Positive Cross-Checker

**Thoroughness:** very thorough
**Focus:** Verify false positives are consistent across reports

**Prompt:**
```
Cross-check false positive determinations:

1. From 1-REPORT: List all items marked FALSE POSITIVE
2. From 2-REPORT: Check if any of these are still reported as issues
3. From 0-REPORT: Check if documentation contradicts false positive determination

For each false positive:
- Re-verify against actual code (don't trust previous analysis)
- Ensure consistent treatment across reports

Check if 2-REPORT has any issues that should be FALSE POSITIVE based on code verification.

Output:
| Item | 1-REPORT Status | 2-REPORT Status | Fresh Verification | Action |
```

---

### Agent 8: Statistics Reconciler

**Thoroughness:** medium
**Focus:** Verify statistics are consistent and accurate

**Prompt:**
```
Reconcile statistics across reports:

1. From 1-REPORT:
   - Total bugs claimed
   - By severity breakdown
   - Count actual bug entries to verify

2. From 2-REPORT:
   - Total issues claimed
   - By severity breakdown
   - Count actual sections to verify

3. From 0-REPORT:
   - Function/class counts claimed
   - Verify against actual code

4. Check for overlaps:
   - Are some issues counted in BOTH reports?
   - Should they be cross-referenced?

Output statistics reconciliation table with fresh counts.
```

---

### Agent 9: Correction Propagation Checker

**Thoroughness:** thorough
**Focus:** Ensure corrections in changelogs were actually applied

**Prompt:**
```
Check if corrections documented in changelogs were actually applied:

1. Read 0-CHANGELOG.md - Note corrections claimed
2. Read 1-CHANGELOG.md - Note false positives claimed
3. Read 2-CHANGELOG.md - Note corrections claimed

For each correction claimed in any changelog:
- Verify the correction was actually applied to the report
- Check if related claims in other reports were also corrected
- Flag any claimed corrections that weren't applied

This catches cases where changelog says "corrected" but report still has error.
```

---

### Agent 10: Source Code Verification Agent

**Thoroughness:** very thorough
**Focus:** Spot-check critical claims against actual source code

**Prompt:**
```
Perform FRESH source code verification for highest-priority claims:

Do not assume previous verifications are correct. Re-verify:

1. **Primary Key Design**:
   - Read aisdb/aisdb_sql/timescale_createtable_dynamic.sql
   - Document actual PRIMARY KEY definition

2. **SQL Injection**:
   - Read aisdb/database/sql_query_strings.py
   - Document actual SQL construction patterns

3. **Y2038 Timestamp Bug**:
   - Read database schema files
   - Document actual timestamp column types

4. **XSS Vulnerability**:
   - Read aisdb_web/map/map.js
   - Document actual DOM manipulation patterns

5. **Rate Limiting**:
   - Read aisdb/webdata/_scraper.py
   - Document actual rate limiting implementation (or lack thereof)

6. **Haversine Order**:
   - Read src/lib.rs haversine function
   - Document actual parameter order

For each verification:
| Claim | Report(s) | Fresh Verification Result | Discrepancy? |
```

---

## Phase 2: Merge Protocol

### Step 6: NOW Read Existing 3-REPORT.md

**Only after completing Phase 1 (fresh analysis), read the existing report:**

```
IF 3-REPORT.md exists:
    Read 3-REPORT.md
    Read 3-CHANGELOG.md

    Create comparison:
    - Fresh findings list (from Phase 1)
    - Existing findings list (from 3-REPORT.md)
```

### Step 7: Reasoning Over Merge

For each finding, apply this decision logic:

```
┌─────────────────────────────────────────────────────────────────────┐
│                      MERGE DECISION MATRIX                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  CASE 1: Found in Fresh Analysis, NOT in Existing 3-REPORT          │
│  ───────────────────────────────────────────────────────────────    │
│  Action: ADD as new contradiction                                    │
│  Changelog: Mark as [ADDITION]                                       │
│  Reasoning: Previously missed or newly introduced                    │
│                                                                      │
│  CASE 2: Found in Fresh Analysis AND in Existing 3-REPORT           │
│  ───────────────────────────────────────────────────────────────    │
│  Action: VERIFY - confirm still present                              │
│  Changelog: Mark as [VERIFIED]                                       │
│  Reasoning: Contradiction persists                                   │
│                                                                      │
│  CASE 3: In Existing 3-REPORT as RESOLVED, but Fresh finds it       │
│  ───────────────────────────────────────────────────────────────    │
│  Action: REOPEN - mark as regression                                 │
│  Changelog: Mark as [REGRESSION]                                     │
│  Reasoning: Fix was reverted or incomplete                           │
│                                                                      │
│  CASE 4: In Existing 3-REPORT, NOT found in Fresh Analysis          │
│  ───────────────────────────────────────────────────────────────    │
│  Sub-case A: Was marked RESOLVED → Keep as RESOLVED                  │
│  Sub-case B: Was marked OPEN → Verify if truly fixed, mark RESOLVED  │
│  Sub-case C: Was incorrect finding → Mark as INVALID                 │
│  Changelog: Mark appropriately                                       │
│                                                                      │
│  CASE 5: Fresh Analysis contradicts Existing 3-REPORT conclusion    │
│  ───────────────────────────────────────────────────────────────    │
│  Action: Trust FRESH analysis (source code is authoritative)         │
│  Changelog: Mark as [CORRECTED]                                      │
│  Reasoning: Previous analysis may have been wrong                    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Step 8: Create Unified 3-REPORT.md

Merge all findings into a single coherent report:

```markdown
# Structure of Unified Report

1. Executive Summary (updated statistics)
2. Current Run Findings (fresh analysis results)
3. Comparison with Previous Analysis (what changed)
4. Unified Contradiction List (merged, deduplicated)
5. Corrections to Apply (complete list)
6. Historical Record (previously resolved items)
```

---

## Phase 3: Apply Corrections

### Step 9: Correction Execution Protocol

```
For each contradiction requiring correction:

1. IDENTIFY the correction needed
   - Which report(s) need modification
   - What section/bug ID
   - What the correct information is

2. APPLY the correction using Edit tool
   - Update the specific content
   - Add CORRECTION NOTE with date and CONTRA-ID reference
   - Update line numbers if code references changed

3. UPDATE the respective changelog
   - Add entry referencing the contradiction ID
   - Note the correction was from cross-report analysis

4. VERIFY the correction
   - Re-read the modified section
   - Confirm correction is accurate
```

### Correction Priority Order

```
Priority 1 (MUST FIX):
- File paths that don't exist
- Functions/classes that don't exist
- Code snippets that don't match actual code
- FALSE POSITIVE items still listed as bugs
- Factually incorrect statements

Priority 2 (SHOULD FIX):
- Line number drift
- Severity rating inconsistencies
- Statistics mismatches

Priority 3 (NICE TO FIX):
- Minor wording inconsistencies
- Cross-reference improvements
```

### Correction Markers

When correcting reports, add markers:

```markdown
> **CORRECTION NOTE (Date - from 3-REPORT CONTRA-XX-NNN)**: [Description of correction]
```

---

## Report Structure Template

The `3-REPORT.md` must follow this structure:

```markdown
# AISdb-Lite: Cross-Report Contradiction Analysis

**Analysis Date:** [Month Year]
**Reports Analyzed:** 0-REPORT.md, 1-REPORT.md, 2-REPORT.md
**Analysis Method:** Unbiased fresh analysis with post-hoc merge
**Total Contradictions Found:** [Count]
**New This Run:** [Count]
**Verified (Still Present):** [Count]
**Resolved:** [Count]
**Regressions:** [Count]

> **RECONCILIATION STATUS:** [Summary of consistency state]

---

## Executive Summary

[Brief description of cross-report consistency status]

### This Run's Analysis

**Fresh Analysis Results:**
- Contradictions detected: N
- Source code verifications performed: N
- Discrepancies from existing 3-REPORT: N

### Contradiction Statistics

| Category | Total | New | Verified | Resolved | Regression |
|----------|-------|-----|----------|----------|------------|
| File Paths | N | N | N | N | N |
| Function Existence | N | N | N | N | N |
| Line Numbers | N | N | N | N | N |
| Code Snippets | N | N | N | N | N |
| Severity Ratings | N | N | N | N | N |
| Status Conflicts | N | N | N | N | N |
| Statistics | N | N | N | N | N |

---

## Part 1: File Path Contradictions

### CONTRA-FP-NNN: [Description]

**Status:** [NEW | VERIFIED | RESOLVED | REGRESSION]
**Reports Affected:** [List]
**Contradiction:** [What differs]
**Fresh Verification:** [What source code shows]
**Resolution:** [Authoritative answer]
**Corrections Required:**
- 0-REPORT.md: [Change needed or "None"]
- 1-REPORT.md: [Change needed or "None"]
- 2-REPORT.md: [Change needed or "None"]

---

[Continue for Parts 2-8...]

---

## Part 9: Comparison with Previous Analysis

### New Findings (Not in Previous 3-REPORT)
| ID | Description | Impact |

### Regressions (Were Resolved, Now Present Again)
| ID | Description | When Originally Resolved |

### Confirmed Resolutions (Still Fixed)
| ID | Description | Original Resolution Date |

### Corrections to Previous Analysis
| ID | Previous Conclusion | Fresh Finding | Correction |

---

## Part 10: Corrections Applied This Run

### Corrections to 0-REPORT.md
| Section | Original | Corrected | Reason | CONTRA-ID |

### Corrections to 1-REPORT.md
| Bug ID | Original | Corrected | Reason | CONTRA-ID |

### Corrections to 2-REPORT.md
| Section | Original | Corrected | Reason | CONTRA-ID |

---

## Appendix A: Verification Commands

[Bash commands to verify findings]

## Appendix B: Cross-Reference Matrix

[Table mapping related items across reports]

## Appendix C: Merge Decision Log

[Document reasoning for each merge decision made]

---

*Report generated by cross-report contradiction analysis system*
*Analysis Method: Unbiased fresh analysis with post-hoc merge*
*Last Updated: [Date]*
```

---

## Changelog Entry Template

Update `3-CHANGELOG.md` with:

```markdown
## [Run YYYY-MM-DD HH:MM] - Report Version X.X.X

### Summary
Brief description of this analysis run.

### Analysis Method
- Fresh analysis completed: Yes
- Existing 3-REPORT.md found: Yes/No
- Merge performed: Yes/No

### New Contradictions Found
- [ADDITION] CONTRA-XX-NNN: Brief description

### Contradictions Verified (Still Present)
- [VERIFIED] CONTRA-XX-NNN: Confirmation notes

### Contradictions Resolved
- [RESOLVED] CONTRA-XX-NNN: How it was resolved

### Regressions Detected
- [REGRESSION] CONTRA-XX-NNN: Was resolved, now present again

### Previous Analysis Corrections
- [CORRECTED] CONTRA-XX-NNN: Previous conclusion was wrong, now fixed

### Corrections Applied to Source Reports
- [CORRECTED] 0-REPORT.md Section X: Change description
- [CORRECTED] 1-REPORT.md Bug ID: Change description
- [CORRECTED] 2-REPORT.md Section X.X: Change description

### Statistics
- Total Contradictions: [Current count]
- New This Run: [Count]
- Resolved This Run: [Count]
- Regressions: [Count]
- Reports Modified: [List]

### Git State
- Branch: [name]
- Last Commit: [hash] - [message]
- Uncommitted Changes: Yes/No
```

---

## Contradiction ID Assignment

```
Format: CONTRA-[TYPE]-NNN

Types:
  FP = File Path
  FN = Function/Class existence
  LN = Line Number
  CS = Code Snippet
  SV = Severity Rating
  ST = Status Conflict
  FC = Feature Claim
  QT = Quantity/Statistics
  XR = Cross-Reference

Rules:
- IDs are never reused, even for resolved contradictions
- New contradictions get next sequential number
- Resolved contradictions keep their ID for historical reference
```

---

## Quality Checklist

Before finalizing 3-REPORT.md:

**Phase 1 Checklist (Fresh Analysis):**
- [ ] All three source reports read completely
- [ ] All 10 agents executed
- [ ] All file paths verified against filesystem
- [ ] All function/class existence verified against code
- [ ] Critical code snippets verified against source
- [ ] Statistics independently counted
- [ ] Complete list of fresh findings documented

**Phase 2 Checklist (Merge):**
- [ ] Existing 3-REPORT.md read (if exists)
- [ ] Each finding categorized (NEW/VERIFIED/RESOLVED/REGRESSION)
- [ ] Merge reasoning documented
- [ ] Unified report created

**Phase 3 Checklist (Corrections):**
- [ ] All Priority 1 corrections applied
- [ ] All changelogs updated
- [ ] Corrections verified accurate
- [ ] 3-CHANGELOG.md updated with this run's findings

---

## Execution Summary

```
┌─────────────────────────────────────────────────────────────────────┐
│                    COMPLETE EXECUTION FLOW                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. Read 0-REPORT.md, 1-REPORT.md, 2-REPORT.md                      │
│  2. Execute 10 analysis agents (fresh, unbiased)                     │
│  3. Build complete fresh findings list                               │
│  4. Verify all findings against source code                          │
│  5. ─────────────── CHECKPOINT ───────────────                       │
│  6. NOW read existing 3-REPORT.md (if exists)                        │
│  7. Compare fresh findings with existing                             │
│  8. Reason and document merge decisions                              │
│  9. Create unified 3-REPORT.md                                       │
│  10. Apply corrections to 0-REPORT, 1-REPORT, 2-REPORT              │
│  11. Update all changelogs                                           │
│  12. Final verification                                              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

*This prompt is designed to ensure consistency across all AISdb-lite analysis reports.*
*The unbiased-first approach prevents confirmation bias from existing findings.*
*Cross-report verification is essential for maintaining documentation integrity.*
