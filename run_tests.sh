#!/bin/bash

# WadeScript Test Runner
# Compiles and runs all tests, comparing output to expected results

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Setup
. "$HOME/.cargo/env"
export LLVM_SYS_170_PREFIX="/opt/homebrew/opt/llvm@17"

COMPILER="./target/release/wadescript"
TEST_DIR="tests"
PASSED=0
FAILED=0
TOTAL=0

echo "=================================="
echo "WadeScript Test Suite"
echo "=================================="
echo ""

# Build compiler if needed
if [ ! -f "$COMPILER" ]; then
    echo "Building compiler..."
    cargo build --release
    echo ""
fi

# Find all test files
for test_file in $TEST_DIR/test_*.ws; do
    if [ ! -f "$test_file" ]; then
        continue
    fi

    TOTAL=$((TOTAL + 1))
    test_name=$(basename "$test_file" .ws)
    expected_file="${TEST_DIR}/${test_name}.expected"
    executable="./${test_name}"

    echo "Running: $test_name"

    # Check if expected output file exists
    if [ ! -f "$expected_file" ]; then
        echo -e "${YELLOW}SKIP${NC}: No expected output file"
        echo ""
        continue
    fi

    # Compile the test
    if ! $COMPILER "$test_file" > /dev/null 2>&1; then
        echo -e "${RED}FAIL${NC}: Compilation failed"
        FAILED=$((FAILED + 1))
        echo ""
        continue
    fi

    # Run the test and capture output
    if ! actual_output=$($executable 2>&1); then
        echo -e "${RED}FAIL${NC}: Runtime error"
        FAILED=$((FAILED + 1))
        echo ""
        continue
    fi

    # Compare output
    expected_output=$(cat "$expected_file")
    if [ "$actual_output" = "$expected_output" ]; then
        echo -e "${GREEN}PASS${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}FAIL${NC}: Output mismatch"
        echo "Expected:"
        echo "$expected_output"
        echo ""
        echo "Actual:"
        echo "$actual_output"
        FAILED=$((FAILED + 1))
    fi

    # Cleanup
    rm -f "$executable"
    echo ""
done

# Summary
echo "=================================="
echo "Test Results"
echo "=================================="
echo "Total:  $TOTAL"
echo -e "Passed: ${GREEN}$PASSED${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "Failed: ${RED}$FAILED${NC}"
else
    echo -e "Failed: $FAILED"
fi
echo ""

# Exit with error if any tests failed
if [ $FAILED -gt 0 ]; then
    exit 1
fi

echo -e "${GREEN}All tests passed!${NC}"
exit 0
