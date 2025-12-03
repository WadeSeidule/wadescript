#!/bin/bash

# WadeScript build and run utility
# Usage:
#   ws build <file.ws>           - Compile WadeScript file to executable
#   ws run <file.ws>             - Compile and run WadeScript file
#   ws build <file.ws> -o <name> - Compile with custom output name
#   ws run <file.ws> [args...]   - Compile and run with arguments

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WADESCRIPT_BIN="$SCRIPT_DIR/target/debug/wadescript"

# Check if wadescript compiler exists
if [ ! -f "$WADESCRIPT_BIN" ]; then
    echo -e "${RED}Error: wadescript compiler not found at $WADESCRIPT_BIN${NC}"
    echo "Please run 'cargo build' first"
    exit 1
fi

# Show usage
usage() {
    echo "WadeScript build and run utility"
    echo ""
    echo "Usage:"
    echo "  ws build <file.ws>              Compile WadeScript file to executable"
    echo "  ws run <file.ws> [args...]      Compile and run WadeScript file"
    echo "  ws build <file.ws> -o <name>    Compile with custom output name"
    echo "  ws test                         Run all tests in tests/ directory"
    echo ""
    echo "Examples:"
    echo "  ws build examples/hello.ws"
    echo "  ws run examples/hello.ws"
    echo "  ws build main.ws -o myapp"
    echo "  ws run examples/factorial.ws 10"
    echo "  ws test"
    exit 1
}

# Check for command
if [ $# -lt 1 ]; then
    usage
fi

COMMAND="$1"

# Handle test command separately (no source file required)
if [ "$COMMAND" = "test" ]; then
    # Test command handled below
    true
elif [ $# -lt 2 ]; then
    usage
fi

# Set source file for non-test commands
if [ "$COMMAND" != "test" ]; then
    SOURCE_FILE="$2"

    # Check if source file exists
    if [ ! -f "$SOURCE_FILE" ]; then
        echo -e "${RED}Error: Source file '$SOURCE_FILE' not found${NC}"
        exit 1
    fi

    # Get the base name without extension
    BASENAME=$(basename "$SOURCE_FILE" .ws)
    OUTPUT_NAME="$BASENAME.o"
fi

case "$COMMAND" in
    build)
        # Check for custom output name
        if [ "$3" = "-o" ] && [ -n "$4" ]; then
            OUTPUT_NAME="$4"
        fi

        echo -e "${BLUE}Compiling $SOURCE_FILE...${NC}"

        # Compile
        if "$WADESCRIPT_BIN" "$SOURCE_FILE"; then
            # Move to custom name if needed
            if [ "$OUTPUT_NAME" != "$BASENAME" ]; then
                mv "$BASENAME" "$OUTPUT_NAME"
            fi
            echo -e "${GREEN}✓ Compiled successfully to '$OUTPUT_NAME'${NC}"
        else
            echo -e "${RED}✗ Compilation failed${NC}"
            exit 1
        fi
        ;;

    run)
        # Get any additional arguments for the program
        shift 2
        PROGRAM_ARGS="$@"

        echo -e "${BLUE}Compiling $SOURCE_FILE...${NC}"

        # Compile and capture output (don't use set -e here)
        set +e
        COMPILE_OUTPUT=$("$WADESCRIPT_BIN" "$SOURCE_FILE" 2>&1)
        COMPILE_EXIT=$?
        set -e

        if [ $COMPILE_EXIT -eq 0 ]; then
            echo -e "${GREEN}✓ Compiled successfully${NC}"
            echo -e "${BLUE}Running ./$OUTPUT_NAME${NC}"
            echo "---"

            # Run the compiled program
            if [ -n "$PROGRAM_ARGS" ]; then
                "./$OUTPUT_NAME" $PROGRAM_ARGS
            else
                "./$OUTPUT_NAME"
            fi

            EXIT_CODE=$?
            echo "---"
            echo -e "${YELLOW}Program exited with code $EXIT_CODE${NC}"

            # Clean up the executable
            rm -f "./$OUTPUT_NAME"
        else
            echo -e "${RED}✗ Compilation failed${NC}"
            echo "$COMPILE_OUTPUT"
            exit 1
        fi
        ;;

    test)
        # Run all tests in the tests/ directory
        TESTS_DIR="$SCRIPT_DIR/tests"

        if [ ! -d "$TESTS_DIR" ]; then
            echo -e "${RED}Error: tests/ directory not found${NC}"
            exit 1
        fi

        echo -e "${BLUE}Running WadeScript Test Suite${NC}"
        echo -e "${BLUE}==============================${NC}"
        echo ""

        # Find all test files (exclude error tests which start with test_error_)
        TEST_FILES=$(find "$TESTS_DIR" -maxdepth 1 -name "test_*.ws" -type f | grep -v "test_error_" | sort)

        if [ -z "$TEST_FILES" ]; then
            echo -e "${YELLOW}No test files found in $TESTS_DIR${NC}"
            exit 0
        fi

        TOTAL_TESTS=0
        PASSED_TESTS=0
        FAILED_TESTS=0
        FAILED_TEST_NAMES=()

        # Run each test
        for TEST_FILE in $TEST_FILES; do
            TOTAL_TESTS=$((TOTAL_TESTS + 1))
            TEST_NAME=$(basename "$TEST_FILE" .ws)

            # Compile the test
            set +e
            COMPILE_OUTPUT=$("$WADESCRIPT_BIN" "$TEST_FILE" 2>&1)
            COMPILE_EXIT=$?
            set -e

            if [ $COMPILE_EXIT -ne 0 ]; then
                echo -e "${RED}✗ $TEST_NAME: Compilation failed${NC}"
                echo -e "${YELLOW}  Error:${NC}"
                echo "$COMPILE_OUTPUT" | head -5 | sed 's/^/    /'
                FAILED_TESTS=$((FAILED_TESTS + 1))
                FAILED_TEST_NAMES+=("$TEST_NAME (compilation failed)")
                continue
            fi

            # Run the test and capture output
            set +e
            TEST_OUTPUT=$("./$TEST_NAME" 2>&1)
            RUN_EXIT=$?
            set -e

            # Clean up executable
            rm -f "./$TEST_NAME"

            # Test passes if exit code is 0, fails otherwise
            if [ $RUN_EXIT -eq 0 ]; then
                echo -e "${GREEN}✓ $TEST_NAME${NC}"
                PASSED_TESTS=$((PASSED_TESTS + 1))
            else
                echo -e "${RED}✗ $TEST_NAME: Test failed (exit code $RUN_EXIT)${NC}"
                # Show output for debugging
                if [ -n "$TEST_OUTPUT" ]; then
                    echo -e "${YELLOW}  Output:${NC}"
                    echo "$TEST_OUTPUT" | head -10 | sed 's/^/    /'
                    if [ $(echo "$TEST_OUTPUT" | wc -l) -gt 10 ]; then
                        echo "    ..."
                    fi
                fi
                FAILED_TESTS=$((FAILED_TESTS + 1))
                FAILED_TEST_NAMES+=("$TEST_NAME (exit code $RUN_EXIT)")
            fi
        done

        # Print summary
        echo ""
        echo -e "${BLUE}==============================${NC}"
        echo -e "${BLUE}Test Summary${NC}"
        echo -e "${BLUE}==============================${NC}"
        echo "Total:  $TOTAL_TESTS"
        echo -e "${GREEN}Passed: $PASSED_TESTS${NC}"

        if [ $FAILED_TESTS -gt 0 ]; then
            echo -e "${RED}Failed: $FAILED_TESTS${NC}"
            echo ""
            echo -e "${RED}Failed tests:${NC}"
            for TEST_NAME in "${FAILED_TEST_NAMES[@]}"; do
                echo -e "  ${RED}✗${NC} $TEST_NAME"
            done
            exit 1
        else
            echo -e "${GREEN}All tests passed! 🎉${NC}"
            exit 0
        fi
        ;;

    *)
        echo -e "${RED}Error: Unknown command '$COMMAND'${NC}"
        echo ""
        usage
        ;;
esac
