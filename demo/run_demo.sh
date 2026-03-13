#!/usr/bin/env bash
# logfold demo script
# Usage: bash demo/run_demo.sh [path/to/logfold]
#
# Default binary path assumes you have run: cargo build --release

set -euo pipefail

LOGFOLD="${1:-./target/release/logfold}"
DEMO_DIR="$(cd "$(dirname "$0")" && pwd)"

banner() {
    echo ""
    echo "============================================================"
    echo "  $1"
    echo "============================================================"
}

# ---- Demo 1: Web access log ----------------------------------------
banner "DEMO 1: Nginx access log ($(wc -l < "$DEMO_DIR/access.log") lines)"
echo ""
echo "Raw file (first 5 lines):"
head -5 "$DEMO_DIR/access.log"
echo "..."
echo ""
echo '$ logfold --ignore-prefix "^[0-9-]+ [0-9:]+ [0-9.]+ - " demo/access.log'
echo ""
"$LOGFOLD" \
    --ignore-prefix '^[0-9-]+ [0-9:]+ [0-9.]+ - ' \
    "$DEMO_DIR/access.log"

# ---- Demo 2: Same file, top 3 --------------------------------------
banner "DEMO 2: Same file — top 3 only (--top 3)"
echo ""
echo '$ logfold --ignore-prefix "^[0-9-]+ [0-9:]+ [0-9.]+ - " --top 3 demo/access.log'
echo ""
"$LOGFOLD" \
    --ignore-prefix '^[0-9-]+ [0-9:]+ [0-9.]+ - ' \
    --top 3 \
    "$DEMO_DIR/access.log"

# ---- Demo 3: App error/event log -----------------------------------
banner "DEMO 3: App error log / ISO 8601 timestamps ($(wc -l < "$DEMO_DIR/app.log") lines)"
echo ""
echo "Raw file (first 5 lines):"
head -5 "$DEMO_DIR/app.log"
echo "..."
echo ""
echo '$ logfold --ignore-prefix "^\S+Z " demo/app.log'
echo ""
"$LOGFOLD" \
    --ignore-prefix '^\S+Z ' \
    "$DEMO_DIR/app.log"

# ---- stdin demo ----------------------------------------------------
banner "DEMO 4: stdin (pipe)"
echo ""
echo '$ cat demo/access.log | logfold --ignore-prefix "^[0-9-]+ [0-9:]+ [0-9.]+ - " --top 5'
echo ""
cat "$DEMO_DIR/access.log" \
    | "$LOGFOLD" \
        --ignore-prefix '^[0-9-]+ [0-9:]+ [0-9.]+ - ' \
        --top 5

echo ""
banner "Done — $(( $(wc -l < "$DEMO_DIR/access.log") + $(wc -l < "$DEMO_DIR/app.log") )) lines total, collapsed to a handful of summaries."
echo ""
