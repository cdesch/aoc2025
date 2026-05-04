#!/usr/bin/env bash
#
# Generate boilerplate for a new Advent of Code day.
#
# Usage: ./new_day.sh <day_number>
# Example: ./new_day.sh 1
#          ./new_day.sh 14

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 1"
    exit 1
fi

DAY_NUM=$1

# Validate day number
if ! [[ "$DAY_NUM" =~ ^[0-9]+$ ]] || [ "$DAY_NUM" -lt 1 ] || [ "$DAY_NUM" -gt 25 ]; then
    echo "Error: Day must be a number between 1 and 25."
    exit 1
fi

# Zero-padded day string
DAY_PAD=$(printf "%02d" "$DAY_NUM")

# Struct name: Day01, Day02, etc.
STRUCT_NAME="Day${DAY_PAD}"
MODULE_NAME="day${DAY_PAD}"

SRC_FILE="${SCRIPT_DIR}/src/days/${MODULE_NAME}.rs"
TEST_FILE="${SCRIPT_DIR}/tests/${MODULE_NAME}_test.rs"
INPUT_FILE="${SCRIPT_DIR}/input/${MODULE_NAME}.txt"
SAMPLE_FILE="${SCRIPT_DIR}/input/${MODULE_NAME}_sample.txt"
MOD_FILE="${SCRIPT_DIR}/src/days/mod.rs"

# Check if day already exists
if [ -f "$SRC_FILE" ]; then
    echo "Error: ${SRC_FILE} already exists. Day ${DAY_PAD} is already set up."
    exit 1
fi

echo "Setting up Day ${DAY_PAD}..."

# Create directories if needed
mkdir -p "${SCRIPT_DIR}/src/days"
mkdir -p "${SCRIPT_DIR}/tests"
mkdir -p "${SCRIPT_DIR}/input"

# Generate source file from template
sed -e "s/DayXX/${STRUCT_NAME}/g" \
    "${SCRIPT_DIR}/templates/dayXX.rs" > "$SRC_FILE"
echo "  Created ${SRC_FILE}"

# Generate test file from template
sed -e "s/DayXX/${STRUCT_NAME}/g" \
    -e "s/dayXX/${MODULE_NAME}/g" \
    -e "s/XX/${DAY_NUM}/g" \
    "${SCRIPT_DIR}/templates/dayXX_test.rs" > "$TEST_FILE"
echo "  Created ${TEST_FILE}"

# Create empty input files
touch "$INPUT_FILE"
touch "$SAMPLE_FILE"
echo "  Created ${INPUT_FILE}"
echo "  Created ${SAMPLE_FILE}"

# Register the module in days/mod.rs using a temp file approach (portable)
TMPFILE=$(mktemp)

# 1. Add `pub mod dayXX;` if not already present
if ! grep -q "pub mod ${MODULE_NAME};" "$MOD_FILE"; then
    awk -v mod="pub mod ${MODULE_NAME};" '
    /^\/\/\/ Trait that every day/ && !inserted {
        print mod
        print ""
        inserted = 1
    }
    /^pub mod day[0-9]/ {
        # Track that we have existing mod lines
        last_mod = NR
    }
    { lines[NR] = $0 }
    END {
        # If we already inserted, just print
        if (inserted) {
            for (i = 1; i <= NR; i++) print lines[i]
        } else {
            # Insert after last pub mod line
            for (i = 1; i <= NR; i++) {
                print lines[i]
                if (i == last_mod) print mod
            }
        }
    }
    ' "$MOD_FILE" > "$TMPFILE"

    # Simpler approach: just read the file content and manipulate it
    # Let's use a straightforward method
    rm -f "$TMPFILE"

    # Check if there are existing day modules
    if grep -q "^pub mod day" "$MOD_FILE"; then
        # Add after the last existing pub mod dayNN line
        LAST_LINE_NUM=$(grep -n "^pub mod day" "$MOD_FILE" | tail -1 | cut -d: -f1)
        head -n "$LAST_LINE_NUM" "$MOD_FILE" > "$TMPFILE"
        echo "pub mod ${MODULE_NAME};" >> "$TMPFILE"
        tail -n +"$((LAST_LINE_NUM + 1))" "$MOD_FILE" >> "$TMPFILE"
    else
        # No day modules yet — add before the `/// Trait` doc comment
        TRAIT_LINE_NUM=$(grep -n "^/// Trait that every day" "$MOD_FILE" | head -1 | cut -d: -f1)
        head -n "$((TRAIT_LINE_NUM - 1))" "$MOD_FILE" > "$TMPFILE"
        echo "pub mod ${MODULE_NAME};" >> "$TMPFILE"
        echo "" >> "$TMPFILE"
        tail -n +"$TRAIT_LINE_NUM" "$MOD_FILE" >> "$TMPFILE"
    fi

    mv "$TMPFILE" "$MOD_FILE"
    echo "  Registered module in mod.rs"
fi

# 2. Add match arm in run_day()
MATCH_LINE="        ${DAY_NUM} => run_solution(${DAY_NUM}, &${MODULE_NAME}::${STRUCT_NAME}, &input),"
if ! grep -q "${MODULE_NAME}::${STRUCT_NAME}" "$MOD_FILE"; then
    TMPFILE=$(mktemp)
    while IFS= read -r line; do
        if [[ "$line" == *"// MATCH_ARMS"* ]]; then
            echo "$MATCH_LINE" >> "$TMPFILE"
        fi
        echo "$line" >> "$TMPFILE"
    done < "$MOD_FILE"
    mv "$TMPFILE" "$MOD_FILE"
    echo "  Added match arm in run_day()"
fi

echo ""
echo "Day ${DAY_PAD} is ready!"
echo ""
echo "Next steps:"
echo "  1. Paste your puzzle input into:  input/${MODULE_NAME}.txt"
echo "  2. Paste the sample input into:   input/${MODULE_NAME}_sample.txt"
echo "  3. Implement your solution in:    src/days/${MODULE_NAME}.rs"
echo "  4. Update expected values in:     tests/${MODULE_NAME}_test.rs"
echo ""
echo "Run your solution:    cargo run -- ${DAY_NUM}"
echo "Test sample input:    cargo test --test ${MODULE_NAME}_test"
echo "Test (include real):  cargo test --test ${MODULE_NAME}_test -- --include-ignored"
