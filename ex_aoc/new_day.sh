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

MODULE_NAME="Day${DAY_PAD}"
FILE_NAME="day_${DAY_PAD}"

SRC_FILE="${SCRIPT_DIR}/lib/ex_aoc/${FILE_NAME}.ex"
TEST_FILE="${SCRIPT_DIR}/test/${FILE_NAME}_test.exs"
INPUT_FILE="${SCRIPT_DIR}/input/${FILE_NAME}.txt"
SAMPLE_FILE="${SCRIPT_DIR}/input/${FILE_NAME}_sample.txt"

# Check if day already exists
if [ -f "$SRC_FILE" ]; then
    echo "Error: ${SRC_FILE} already exists. Day ${DAY_PAD} is already set up."
    exit 1
fi

echo "Setting up Day ${DAY_PAD}..."

# Create directories if needed
mkdir -p "${SCRIPT_DIR}/lib/ex_aoc"
mkdir -p "${SCRIPT_DIR}/test"
mkdir -p "${SCRIPT_DIR}/input"

# Generate source file from template
sed -e "s/DayXX/${MODULE_NAME}/g" \
    -e "s/Day XX/Day ${DAY_PAD}/g" \
    "${SCRIPT_DIR}/templates/day_xx.ex" > "$SRC_FILE"
echo "  Created ${SRC_FILE}"

# Generate test file from template
sed -e "s/DayXX/${MODULE_NAME}/g" \
    -e "s/XX/${DAY_NUM}/g" \
    "${SCRIPT_DIR}/templates/day_xx_test.exs" > "$TEST_FILE"
echo "  Created ${TEST_FILE}"

# Create empty input files
touch "$INPUT_FILE"
touch "$SAMPLE_FILE"
echo "  Created ${INPUT_FILE}"
echo "  Created ${SAMPLE_FILE}"

echo ""
echo "Day ${DAY_PAD} is ready!"
echo ""
echo "Next steps:"
echo "  1. Paste your puzzle input into:  input/${FILE_NAME}.txt"
echo "  2. Paste the sample input into:   input/${FILE_NAME}_sample.txt"
echo "  3. Implement your solution in:    lib/ex_aoc/${FILE_NAME}.ex"
echo "  4. Update expected values in:     test/${FILE_NAME}_test.exs"
echo ""
echo "Run your solution:    mix run -e 'ExAoc.run(${DAY_NUM})'"
echo "Test sample input:    mix test test/${FILE_NAME}_test.exs"
echo "Test (include real):  mix test test/${FILE_NAME}_test.exs --include real"
