# AISdb-Lite Audit System

This folder contains the multi-agent analysis system for comprehensive auditing of the AISdb-lite repository.

## Structure

```
audit/
├── README.md           # This file
├── run_audit.sh        # Automated audit runner (bash)
├── 0-PROMPT.md         # Architecture documentation prompt
├── 0-REPORT.md         # Architecture documentation report
├── 0-CHANGELOG.md      # Architecture report changelog
├── 1-PROMPT.md         # Bug analysis prompt
├── 1-REPORT.md         # Bug analysis report
├── 1-CHANGELOG.md      # Bug report changelog
├── 2-PROMPT.md         # Bad business decisions prompt
├── 2-REPORT.md         # Bad business decisions report
├── 2-CHANGELOG.md      # Business decisions changelog
├── 3-PROMPT.md         # Cross-report contradiction analysis prompt
├── 3-REPORT.md         # Contradiction analysis report
├── 3-CHANGELOG.md      # Contradiction report changelog
├── 4-PROMPT.md         # Engineering blueprint prompt
├── 4-REPORT.md         # Engineering blueprint report
└── 4-CHANGELOG.md      # Engineering blueprint changelog
```

## Report Purposes

| Report | Purpose | Input | Output |
|--------|---------|-------|--------|
| **0-REPORT** | Architecture Documentation | Source code analysis | Complete system documentation with diagrams |
| **1-REPORT** | Bug Analysis | Source code analysis | Verified bugs with severity ratings |
| **2-REPORT** | Bad Business Decisions | Source code + 1-REPORT | Architectural anti-patterns and remediation |
| **3-REPORT** | Contradiction Analysis | 0, 1, 2 REPORTS | Cross-report consistency validation |
| **4-REPORT** | Engineering Blueprint | 0, 1, 2 REPORTS | Refactoring plan for PostgreSQL-only pipeline |

## Execution Order

For a complete audit, execute prompts in this order:

```
1. 0-PROMPT.md → Generates 0-REPORT.md (Architecture)
2. 1-PROMPT.md → Generates 1-REPORT.md (Bugs)
3. 2-PROMPT.md → Generates 2-REPORT.md (Bad Decisions)
4. 3-PROMPT.md → Generates 3-REPORT.md (Contradictions) + fixes 0,1,2
5. 4-PROMPT.md → Generates 4-REPORT.md (Engineering Blueprint)
```

**Dependencies:**
- Reports 0, 1, 2 can be generated independently (in parallel if desired)
- Report 3 requires Reports 0, 1, 2 to exist
- Report 4 requires Reports 0, 1, 2 to exist

## How to Run

### Automated Runner (Recommended)

The easiest way to run a complete audit is using the automated runner:

```bash
# Full audit - runs 0 → 1 → 2 → 3 → 4 sequentially, commits & pushes at end
./run_audit.sh

# Start from a specific prompt (e.g., prompt 2)
./run_audit.sh 2

# Run a specific range (e.g., prompts 1 through 3)
./run_audit.sh 1 3

# Run without git commit/push
./run_audit.sh --no-git

# Show help
./run_audit.sh --help
```

### Features

- **Sequential execution**: Prompts run in order (0 → 1 → 2 → 3 → 4)
- **Automatic git commit/push**: Changes committed and pushed after successful completion
- **Logging**: Each prompt creates a timestamped log file in `audit/logs/`
- **Color output**: Visual feedback for progress and status
- **Error handling**: Stops on failure, skips git push if any prompt fails

### Cron Job Setup

To run audits automatically every 2 hours:

```bash
# Add to crontab (crontab -e)
0 */2 * * * /home/spadon/AISdb-lite/audit/run_audit.sh > /dev/null 2>&1
```

Note: Detailed logs are saved to `audit/logs/` by the script itself.

### File Access Rules (Enforced by Runner)

| Prompt | Can Modify | Reads |
|--------|------------|-------|
| 0 | 0-REPORT.md, 0-CHANGELOG.md | Source code only |
| 1 | 1-REPORT.md, 1-CHANGELOG.md | Source code only |
| 2 | 2-REPORT.md, 2-CHANGELOG.md | Source code only |
| 3 | 0,1,2,3-REPORT.md, 0,1,2,3-CHANGELOG.md | 0,1,2-REPORT.md |
| 4 | 4-REPORT.md, 4-CHANGELOG.md | 0,1,2-REPORT.md (ignores 3-*) |

### Manual Execution (Claude Code CLI)

Navigate to the audit folder and provide the prompt:

```bash
cd /path/to/AISdb-lite/audit
claude "Read 0-PROMPT.md and execute it to generate 0-REPORT.md"
```

Or reference the prompt file directly:

```bash
claude "Execute the analysis described in @audit/0-PROMPT.md"
```

### Running Multiple Prompts Manually

For a full audit run:

```bash
# Generate foundational reports (can run in parallel)
claude "Execute @audit/0-PROMPT.md to generate architecture documentation"
claude "Execute @audit/1-PROMPT.md to generate bug analysis"
claude "Execute @audit/2-PROMPT.md to generate bad decisions analysis"

# After above complete, run cross-validation
claude "Execute @audit/3-PROMPT.md to analyze contradictions and fix reports"

# Finally, generate engineering blueprint
claude "Execute @audit/4-PROMPT.md to generate refactoring plan"
```

## Report Versioning

Each report includes version information in its header:

```markdown
> **Report Version**: X.Y.Z
> **Date**: YYYY-MM-DD
```

Changelogs track all modifications with entries like:

```markdown
## [Run YYYY-MM-DD HH:MM] - Report Version X.Y.Z

### Summary
Brief description of changes

### Changes
- [ADDED] New content
- [UPDATED] Modified content
- [REMOVED] Deleted content
```

## Guidelines Applied to All Reports

All prompts enforce these guidelines:

1. **No Page Limit** - Document everything necessary
2. **Avoid Duplications** - Cross-reference instead of repeat
3. **Reduce Verbosity** - Direct, concise findings
4. **Traceability** - Every claim must include:
   - File path
   - Line numbers
   - Code snippet
   - Verification command

## Codebase References

When prompts reference source code paths like `aisdb/track_gen.py`, these are relative to the repository root (`../` from this audit folder). The analysis agents automatically search from the repository root.

## Output Quality

Reports are designed to be:
- **Actionable** - Clear steps for developers
- **Verifiable** - Commands to confirm findings
- **Consistent** - Cross-validated across reports
- **Comprehensive** - No page limits, full documentation

## Maintenance

After code changes to AISdb-lite:
1. Re-run affected prompts to update reports
2. Run 3-PROMPT.md to catch new contradictions
3. Update 4-REPORT.md if architecture changes

---

*This audit system provides comprehensive technical documentation and analysis for the AISdb-lite maritime data framework.*
