#!/bin/bash
#
# AISdb-Lite Documentation Audit Runner
# Executes audit prompts sequentially using Claude Code agents
#
# Usage: ./run_audit.sh [OPTIONS] [START] [END]
#
# Options:
#   --no-git        Skip git commit/push at the end
#   --help          Show this help message
#
# Arguments:
#   START           Start from this prompt number (default: 0)
#   END             End at this prompt number (default: 4)
#
# Execution Order: 0 → 1 → 2 → 3 → 4
#
# File Access Rules:
#   - Prompt 0: Can modify 0-REPORT.md, 0-CHANGELOG.md
#   - Prompt 1: Can modify 1-REPORT.md, 1-CHANGELOG.md
#   - Prompt 2: Can modify 2-REPORT.md, 2-CHANGELOG.md
#   - Prompt 3: Can modify 0,1,2,3-REPORT.md and 0,1,2,3-CHANGELOG.md
#   - Prompt 4: Reads 0,1,2-REPORT.md; Can modify 4-REPORT.md, 4-CHANGELOG.md
#

set -e

# Parse arguments
DO_GIT=true
START=""
END=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --no-git)
            DO_GIT=false
            shift
            ;;
        --help)
            head -24 "$0" | tail -22
            exit 0
            ;;
        *)
            if [[ -z "$START" ]]; then
                START=$1
            elif [[ -z "$END" ]]; then
                END=$1
            else
                echo "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
            fi
            shift
            ;;
    esac
done

# Default values
START=${START:-0}
END=${END:-4}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_DIR="${SCRIPT_DIR}/logs"
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")

# Create logs directory
mkdir -p "${LOG_DIR}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     AISdb-Lite Audit Runner - $(date +"%Y-%m-%d %H:%M")            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "Repository: ${YELLOW}${REPO_ROOT}${NC}"
echo -e "Prompts:    ${YELLOW}${START} through ${END}${NC}"
echo -e "Git:        ${YELLOW}$(if $DO_GIT; then echo 'Enabled (commit at end)'; else echo 'Disabled'; fi)${NC}"
echo -e "Log dir:    ${YELLOW}${LOG_DIR}${NC}"
echo ""

# Allowed tools for audit agents
ALLOWED_TOOLS="Read,Grep,Glob,Bash,Task,TodoWrite,Edit,Write,WebSearch,WebFetch"

# Max turns per prompt (some need more for thorough analysis)
declare -A MAX_TURNS
MAX_TURNS=(
    [0]=150  # Architecture - many files to analyze
    [1]=150  # Bug analysis - thorough code review
    [2]=150  # Bad decisions - architectural analysis
    [3]=100  # Contradiction analysis - comparing reports
    [4]=150  # Engineering blueprint - comprehensive planning
)

# Prompt descriptions
declare -A DESCRIPTIONS
DESCRIPTIONS=(
    [0]="Architecture Documentation"
    [1]="Bug Analysis"
    [2]="Bad Business Decisions"
    [3]="Cross-Report Contradiction Analysis"
    [4]="Engineering Blueprint"
)

# Files each prompt can modify
declare -A CAN_MODIFY
CAN_MODIFY=(
    [0]="0-REPORT.md, 0-CHANGELOG.md"
    [1]="1-REPORT.md, 1-CHANGELOG.md"
    [2]="2-REPORT.md, 2-CHANGELOG.md"
    [3]="0-REPORT.md, 0-CHANGELOG.md, 1-REPORT.md, 1-CHANGELOG.md, 2-REPORT.md, 2-CHANGELOG.md, 3-REPORT.md, 3-CHANGELOG.md"
    [4]="4-REPORT.md, 4-CHANGELOG.md"
)

# Reports each prompt should read
declare -A REPORTS_TO_READ
REPORTS_TO_READ=(
    [0]=""
    [1]=""
    [2]=""
    [3]="0-REPORT.md, 1-REPORT.md, 2-REPORT.md"
    [4]="0-REPORT.md, 1-REPORT.md, 2-REPORT.md"
)

# Function to run a single audit prompt
run_prompt() {
    local num=$1
    local prompt_file="${SCRIPT_DIR}/${num}-PROMPT.md"
    local log_file="${LOG_DIR}/prompt-${num}-${TIMESTAMP}.log"
    local description="${DESCRIPTIONS[$num]}"
    local can_modify="${CAN_MODIFY[$num]}"
    local reports_to_read="${REPORTS_TO_READ[$num]}"
    local max_turns="${MAX_TURNS[$num]}"

    echo -e "${YELLOW}[Prompt ${num}]${NC} Starting: ${description}"
    echo -e "${YELLOW}[Prompt ${num}]${NC} Log file: ${log_file}"

    # Verify prompt file exists
    if [[ ! -f "${prompt_file}" ]]; then
        echo -e "${RED}[Prompt ${num}] ERROR: Prompt file not found: ${prompt_file}${NC}" | tee "${log_file}"
        return 1
    fi

    # Build the instruction for Claude
    local instruction="Execute the audit analysis described in the prompt file.

## Task: ${description} (Prompt ${num})

### Instructions:
1. Read and follow the instructions in ${prompt_file}
2. Analyze the AISdb-lite codebase at ${REPO_ROOT}"

    # Add reports to read if any
    if [[ -n "$reports_to_read" ]]; then
        instruction="${instruction}
3. Read these existing reports for context:"
        IFS=',' read -ra reports <<< "$reports_to_read"
        for report in "${reports[@]}"; do
            report=$(echo "$report" | xargs)  # trim whitespace
            instruction="${instruction}
   - ${SCRIPT_DIR}/${report}"
        done
    fi

    instruction="${instruction}

### File Access Rules (STRICT):
You may ONLY create or modify these files in ${SCRIPT_DIR}:"
    IFS=',' read -ra files <<< "$can_modify"
    for f in "${files[@]}"; do
        f=$(echo "$f" | xargs)  # trim whitespace
        instruction="${instruction}
   - ${f}"
    done

    instruction="${instruction}

DO NOT modify any other files. DO NOT modify source code in the repository.

### Output Requirements:
- Follow the prompt's report structure exactly
- Include all required sections and traceability
- Update the changelog with this run's changes
- Use multi-agent exploration (Task tool) for thorough analysis

Begin the analysis now. Take your time to be thorough."

    # Read the prompt file content
    local prompt_content
    prompt_content=$(cat "${prompt_file}")

    # Full prompt includes both instruction and prompt content
    local full_prompt="${instruction}

---

# PROMPT FILE CONTENT (${num}-PROMPT.md):

${prompt_content}"

    # Run Claude with the prompt
    # Use unbuffer or stdbuf to handle output buffering, with tee for logging
    local exit_code=0
    (
        cd "${REPO_ROOT}" || exit 1

        # Try stdbuf for line-buffered output, fall back to direct execution
        if command -v stdbuf &> /dev/null; then
            stdbuf -oL claude --print \
                --dangerously-skip-permissions \
                --allowedTools "${ALLOWED_TOOLS}" \
                --max-turns "${max_turns}" \
                "${full_prompt}" \
                2>&1
        else
            claude --print \
                --dangerously-skip-permissions \
                --allowedTools "${ALLOWED_TOOLS}" \
                --max-turns "${max_turns}" \
                "${full_prompt}" \
                2>&1
        fi
    ) | stdbuf -oL tee "${log_file}"

    exit_code=${PIPESTATUS[0]}

    if [[ $exit_code -eq 0 ]]; then
        echo -e "${GREEN}[Prompt ${num}] Completed successfully${NC}"
    else
        echo -e "${RED}[Prompt ${num}] Failed with exit code: ${exit_code}${NC}"
    fi

    return $exit_code
}

# Function to do git commit/push
do_git_push() {
    echo ""
    echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Committing and pushing changes to GitHub...${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

    cd "${SCRIPT_DIR}"

    # Check if there are changes (including untracked files)
    if git diff --quiet && git diff --cached --quiet && [[ -z "$(git ls-files --others --exclude-standard)" ]]; then
        echo -e "${YELLOW}No changes to commit${NC}"
        return 0
    fi

    # Stage all changes in audit directory
    git add -A

    # Build commit message listing which prompts ran
    local prompts_run=""
    for num in $(seq $START $END); do
        prompts_run="${prompts_run}
- Prompt ${num}: ${DESCRIPTIONS[$num]}"
    done

    # Create commit
    git commit -m "$(cat <<EOF
docs: Automated audit run - $(date +"%Y-%m-%d %H:%M")

Prompts executed:${prompts_run}

See individual CHANGELOG.md files for details.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
EOF
)"

    # Get the current branch
    local current_branch
    current_branch=$(git branch --show-current)

    # Push to GitHub
    if git push origin "${current_branch}"; then
        echo -e "${GREEN}Successfully pushed to GitHub (branch: ${current_branch})${NC}"
    else
        echo -e "${RED}Failed to push to GitHub${NC}"
        return 1
    fi
}

# Main execution loop
failed=0
num_prompts=$((END - START + 1))

echo -e "${BLUE}Running ${num_prompts} audit prompts sequentially...${NC}"
echo ""

for num in $(seq $START $END); do
    if [[ -z "${DESCRIPTIONS[$num]}" ]]; then
        echo -e "${RED}Unknown prompt number: ${num}${NC}"
        ((failed++))
        continue
    fi

    if run_prompt "$num"; then
        echo -e "${GREEN}✓ Prompt ${num} completed successfully${NC}"
    else
        echo -e "${RED}✗ Prompt ${num} failed${NC}"
        ((failed++))
    fi
    echo ""
done

# Git commit/push (if enabled and no failures)
if $DO_GIT; then
    if [[ $failed -eq 0 ]]; then
        do_git_push
    else
        echo ""
        echo -e "${YELLOW}Skipping git push due to prompt failures${NC}"
    fi
fi

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Audit Complete!${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Logs saved to: ${LOG_DIR}/"
echo ""

# Show log files for this run
echo "Log files created:"
ls -lh "${LOG_DIR}"/*-${TIMESTAMP}.log 2>/dev/null || echo "(No logs found)"

echo ""
if [[ $failed -eq 0 ]]; then
    echo -e "${GREEN}All prompts completed successfully!${NC}"
else
    echo -e "${RED}${failed} prompt(s) failed. Check logs for details.${NC}"
fi

exit $failed
