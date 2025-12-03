#!/bin/bash

# Error Test Runner for WadeScript
# Tests that verify error messages include line numbers and proper formatting

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Running WadeScript Error Test Suite${NC}"
echo -e "${BLUE}====================================${NC}"
echo ""

TOTAL=0
PASSED=0
FAILED=0

# Function to run an error test
run_error_test() {
    local test_file="$1"
    local test_name=$(basename "$test_file" .ws)
    local expected_file="${test_file%.ws}.expected"

    TOTAL=$((TOTAL + 1))

    if [ ! -f "$expected_file" ]; then
        echo -e "${RED}✗ $test_name${NC} - Missing .expected file"
        FAILED=$((FAILED + 1))
        return 1
    fi

    # Run the test and capture stderr (where error messages go)
    # Strip ANSI color codes from output
    local actual_output=$(./ws run "$test_file" 2>&1 | grep -A 10 "Runtime Error:" | head -20 | sed 's/\x1b\[[0-9;]*m//g' || true)

    # Read expected output (just the error parts, not compile messages)
    local expected_output=$(cat "$expected_file")

    # Compare actual vs expected (checking if expected is contained in actual)
    if echo "$actual_output" | grep -q "$(echo "$expected_output" | head -1)"; then
        echo -e "${GREEN}✓ $test_name${NC}"
        PASSED=$((PASSED + 1))
        return 0
    else
        echo -e "${RED}✗ $test_name${NC}"
        echo "  Expected error message containing:"
        echo "    $expected_output" | head -1
        echo "  Got:"
        echo "    $actual_output" | head -1
        FAILED=$((FAILED + 1))
        return 1
    fi
}

# Find all error test files
for test_file in tests/test_error_*.ws; do
    if [ -f "$test_file" ]; then
        run_error_test "$test_file"
    fi
done

echo ""
echo -e "${BLUE}====================================${NC}"
echo -e "${BLUE}Error Test Summary${NC}"
echo -e "${BLUE}====================================${NC}"
echo "Total:  $TOTAL"
if [ $PASSED -gt 0 ]; then
    echo -e "${GREEN}Passed: $PASSED${NC}"
fi
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED${NC}"
    exit 1
else
    echo -e "${GREEN}All error tests passed! 🎉${NC}"
fi
