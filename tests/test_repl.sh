#!/bin/bash

# REPL Test Suite for WadeScript
# Tests the REPL functionality using piped input

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WADESCRIPT_BIN="$SCRIPT_DIR/../target/debug/wadescript"

# Check if wadescript binary exists
if [ ! -f "$WADESCRIPT_BIN" ]; then
    echo -e "${RED}Error: wadescript binary not found at $WADESCRIPT_BIN${NC}"
    echo "Please run 'cargo build' first"
    exit 1
fi

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Test helper function
run_repl_test() {
    local test_name="$1"
    local input="$2"
    local expected="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Run REPL with input and capture output
    set +e
    output=$(echo -e "$input" | "$WADESCRIPT_BIN" repl 2>&1)
    exit_code=$?
    set -e

    # Check if expected string is in output (use -F for fixed string, -- to end options)
    if echo "$output" | grep -qF -- "$expected"; then
        echo -e "${GREEN}✓${NC} $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗${NC} $test_name"
        echo -e "  ${YELLOW}Expected:${NC} $expected"
        echo -e "  ${YELLOW}Got:${NC}"
        echo "$output" | sed 's/^/    /'
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Test that output does NOT contain a string (for error cases)
run_repl_test_no_error() {
    local test_name="$1"
    local input="$2"
    local not_expected="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))

    # Run REPL with input and capture output
    set +e
    output=$(echo -e "$input" | "$WADESCRIPT_BIN" repl 2>&1)
    exit_code=$?
    set -e

    # Check that the not_expected string is NOT in output
    if echo "$output" | grep -qF -- "$not_expected"; then
        echo -e "${RED}✗${NC} $test_name"
        echo -e "  ${YELLOW}Should NOT contain:${NC} $not_expected"
        echo -e "  ${YELLOW}Got:${NC}"
        echo "$output" | sed 's/^/    /'
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    else
        echo -e "${GREEN}✓${NC} $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    fi
}

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}WadeScript REPL Test Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# ============================================
# Basic Output Tests
# ============================================
echo -e "${BLUE}Basic Output Tests${NC}"
echo "---"

run_repl_test "print_int basic" \
    'print_int(42)\nexit' \
    "42"

run_repl_test "print_int negative" \
    'print_int(-123)\nexit' \
    "-123"

run_repl_test "print_str basic" \
    'print_str("Hello")\nexit' \
    "Hello"

run_repl_test "print_bool True" \
    'print_bool(True)\nexit' \
    "True"

run_repl_test "print_bool False" \
    'print_bool(False)\nexit' \
    "False"

echo ""

# ============================================
# Arithmetic Tests
# ============================================
echo -e "${BLUE}Arithmetic Tests${NC}"
echo "---"

run_repl_test "addition" \
    'print_int(10 + 20)\nexit' \
    "30"

run_repl_test "subtraction" \
    'print_int(50 - 17)\nexit' \
    "33"

run_repl_test "multiplication" \
    'print_int(7 * 8)\nexit' \
    "56"

run_repl_test "division" \
    'print_int(100 / 4)\nexit' \
    "25"

run_repl_test "modulo" \
    'print_int(17 % 5)\nexit' \
    "2"

run_repl_test "complex expression" \
    'print_int((10 + 5) * 2 - 6)\nexit' \
    "24"

echo ""

# ============================================
# Function Definition Tests
# ============================================
echo -e "${BLUE}Function Definition Tests${NC}"
echo "---"

run_repl_test "simple function" \
    'def foo() -> int {\n    return 42\n}\nprint_int(foo())\nexit' \
    "42"

run_repl_test "function with parameter" \
    'def double(n: int) -> int {\n    return n * 2\n}\nprint_int(double(21))\nexit' \
    "42"

run_repl_test "function with two parameters" \
    'def add(a: int, b: int) -> int {\n    return a + b\n}\nprint_int(add(15, 27))\nexit' \
    "42"

run_repl_test "void function" \
    'def say_hello() -> void {\n    print_str("Hello!")\n}\nsay_hello()\nexit' \
    "Hello!"

echo ""

# ============================================
# Function Persistence Tests
# ============================================
echo -e "${BLUE}Function Persistence Tests${NC}"
echo "---"

run_repl_test "function persists across inputs" \
    'def get_value() -> int {\n    return 100\n}\nprint_int(get_value())\nexit' \
    "100"

run_repl_test "multiple functions" \
    'def square(n: int) -> int {\n    return n * n\n}\ndef cube(n: int) -> int {\n    return n * n * n\n}\nprint_int(square(3))\nprint_int(cube(3))\nexit' \
    "27"

echo ""

# ============================================
# Control Flow Tests
# ============================================
echo -e "${BLUE}Control Flow Tests${NC}"
echo "---"

run_repl_test "if-else true branch" \
    'def test_if(n: int) -> int {\n    if n > 0 {\n        return 1\n    } else {\n        return 0\n    }\n}\nprint_int(test_if(5))\nexit' \
    "1"

run_repl_test "if-else false branch" \
    'def test_if(n: int) -> int {\n    if n > 0 {\n        return 1\n    } else {\n        return 0\n    }\n}\nprint_int(test_if(-5))\nexit' \
    "0"

run_repl_test "while loop" \
    'def count_to(n: int) -> int {\n    i: int = 0\n    total: int = 0\n    while i < n {\n        total = total + i\n        i = i + 1\n    }\n    return total\n}\nprint_int(count_to(5))\nexit' \
    "10"

echo ""

# ============================================
# Recursion Tests
# ============================================
echo -e "${BLUE}Recursion Tests${NC}"
echo "---"

run_repl_test "recursive factorial" \
    'def factorial(n: int) -> int {\n    if n <= 1 {\n        return 1\n    }\n    return n * factorial(n - 1)\n}\nprint_int(factorial(5))\nexit' \
    "120"

run_repl_test "recursive fibonacci" \
    'def fib(n: int) -> int {\n    if n <= 1 {\n        return n\n    }\n    return fib(n - 1) + fib(n - 2)\n}\nprint_int(fib(10))\nexit' \
    "55"

echo ""

# ============================================
# List Tests
# ============================================
echo -e "${BLUE}List Tests${NC}"
echo "---"

run_repl_test "list creation and length" \
    'def test_list() -> void {\n    nums: list[int] = [1, 2, 3]\n    print_int(nums.length)\n}\ntest_list()\nexit' \
    "3"

run_repl_test "list push" \
    'def test_push() -> void {\n    nums: list[int] = [1, 2]\n    nums.push(3)\n    nums.push(4)\n    print_int(nums.length)\n}\ntest_push()\nexit' \
    "4"

run_repl_test "list indexing" \
    'def test_index() -> void {\n    nums: list[int] = [10, 20, 30]\n    print_int(nums[1])\n}\ntest_index()\nexit' \
    "20"

echo ""

# ============================================
# For Loop Tests
# ============================================
echo -e "${BLUE}For Loop Tests${NC}"
echo "---"

run_repl_test "for loop with range" \
    'def sum_range(n: int) -> int {\n    total: int = 0\n    for i in range(n) {\n        total = total + i\n    }\n    return total\n}\nprint_int(sum_range(5))\nexit' \
    "10"

run_repl_test "for loop with list" \
    'def sum_list() -> int {\n    nums: list[int] = [1, 2, 3, 4, 5]\n    total: int = 0\n    for n in nums {\n        total = total + n\n    }\n    return total\n}\nprint_int(sum_list())\nexit' \
    "15"

echo ""

# ============================================
# Error Handling Tests
# ============================================
echo -e "${BLUE}Error Handling Tests${NC}"
echo "---"

run_repl_test "type error reported" \
    'x: int = "hello"\nexit' \
    "Error"

run_repl_test "undefined variable error" \
    'print_int(undefined_var)\nexit' \
    "Error"

run_repl_test "REPL continues after error" \
    'x: int = "wrong_type"\nprint_int(42)\nexit' \
    "42"

echo ""

# ============================================
# Exit Tests
# ============================================
echo -e "${BLUE}Exit Tests${NC}"
echo "---"

run_repl_test "exit command works" \
    'exit' \
    "Goodbye!"

run_repl_test "shows version on start" \
    'exit' \
    "WadeScript REPL v0.1.0"

echo ""

# ============================================
# Summary
# ============================================
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Test Summary${NC}"
echo -e "${BLUE}================================${NC}"
echo "Total:  $TOTAL_TESTS"
echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"

if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}Failed: $FAILED_TESTS${NC}"
    exit 1
else
    echo -e "${GREEN}All REPL tests passed!${NC}"
    exit 0
fi
