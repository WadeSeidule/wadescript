# WadeScript Makefile
# Handles compilation of both the compiler and runtime library

# Cargo and LLVM configuration
CARGO := $(shell which cargo || echo "$$HOME/.cargo/bin/cargo")
LLVM_PREFIX := /opt/homebrew/opt/llvm@17

# Build directories
BUILD_DIR := target
DEBUG_DIR := $(BUILD_DIR)/debug
RELEASE_DIR := $(BUILD_DIR)/release

# Outputs
COMPILER_DEBUG := $(DEBUG_DIR)/wadescript
COMPILER_RELEASE := $(RELEASE_DIR)/wadescript
RUNTIME_DEBUG := $(DEBUG_DIR)/libwadescript_runtime.a
RUNTIME_RELEASE := $(RELEASE_DIR)/libwadescript_runtime.a

# Source files (for dependency tracking)
RUST_SOURCES := $(shell find src -name '*.rs' 2>/dev/null)
CARGO_TOML := Cargo.toml

# Colors for output
GREEN := \033[0;32m
BLUE := \033[0;34m
YELLOW := \033[1;33m
NC := \033[0m

# Default target
.PHONY: all
all: debug

# Help target
.PHONY: help
help:
	@echo "WadeScript Build System"
	@echo ""
	@echo "Targets:"
	@echo "  make              Build compiler and runtime (debug mode)"
	@echo "  make debug        Build in debug mode"
	@echo "  make release      Build in release mode (optimized)"
	@echo "  make compiler     Build only the compiler (debug)"
	@echo "  make runtime      Build only the runtime library (debug)"
	@echo "  make test         Run the test suite"
	@echo "  make clean        Remove debug build artifacts"
	@echo "  make clean-all    Remove all build artifacts (debug + release)"
	@echo "  make examples     Compile all example programs"
	@echo "  make check        Check code without building"
	@echo "  make help         Show this help message"

# Debug build (default)
.PHONY: debug
debug: $(COMPILER_DEBUG) $(RUNTIME_DEBUG)
	@echo "$(GREEN)✓ Debug build complete$(NC)"
	@echo "  Compiler: $(COMPILER_DEBUG)"
	@echo "  Runtime:  $(RUNTIME_DEBUG)"

# Release build
.PHONY: release
release: $(COMPILER_RELEASE) $(RUNTIME_RELEASE)
	@echo "$(GREEN)✓ Release build complete$(NC)"
	@echo "  Compiler: $(COMPILER_RELEASE)"
	@echo "  Runtime:  $(RUNTIME_RELEASE)"

# Build debug compiler and runtime
$(COMPILER_DEBUG) $(RUNTIME_DEBUG): $(RUST_SOURCES) $(CARGO_TOML)
	@echo "$(BLUE)Building compiler and runtime (debug mode)...$(NC)"
	@export LLVM_SYS_170_PREFIX=$(LLVM_PREFIX) && \
		$(CARGO) build

# Build release compiler and runtime
$(COMPILER_RELEASE) $(RUNTIME_RELEASE): $(RUST_SOURCES) $(CARGO_TOML)
	@echo "$(BLUE)Building compiler and runtime (release mode)...$(NC)"
	@export LLVM_SYS_170_PREFIX=$(LLVM_PREFIX) && \
		$(CARGO) build --release

# Build only the compiler (debug)
.PHONY: compiler
compiler: $(COMPILER_DEBUG)
	@echo "$(GREEN)✓ Compiler built: $(COMPILER_DEBUG)$(NC)"

# Build only the compiler (release)
.PHONY: compiler-release
compiler-release: $(COMPILER_RELEASE)
	@echo "$(GREEN)✓ Compiler built: $(COMPILER_RELEASE)$(NC)"

# Build only the runtime (debug)
.PHONY: runtime
runtime: $(RUNTIME_DEBUG)
	@echo "$(GREEN)✓ Runtime library built: $(RUNTIME_DEBUG)$(NC)"

# Build only the runtime (release)
.PHONY: runtime-release
runtime-release: $(RUNTIME_RELEASE)
	@echo "$(GREEN)✓ Runtime library built: $(RUNTIME_RELEASE)$(NC)"

# Check code without building
.PHONY: check
check:
	@echo "$(BLUE)Checking code...$(NC)"
	@export LLVM_SYS_170_PREFIX=$(LLVM_PREFIX) && \
		$(CARGO) check

# Run tests
.PHONY: test
test: debug
	@echo "$(BLUE)Running test suite...$(NC)"
	@./ws test

# Compile all examples (skip library files)
.PHONY: examples
examples: debug
	@echo "$(BLUE)Compiling examples...$(NC)"
	@failed=0; \
	for file in examples/*.ws; do \
		base=$$(basename $$file); \
		if [ "$$base" != "math_lib.ws" ]; then \
			echo "  Compiling $$file..."; \
			./ws build "$$file" || failed=$$((failed + 1)); \
		fi; \
	done; \
	if [ $$failed -eq 0 ]; then \
		echo "$(GREEN)✓ All examples compiled successfully$(NC)"; \
	else \
		echo "$(YELLOW)⚠ $$failed example(s) failed to compile$(NC)"; \
	fi
	@echo "$(YELLOW)Cleaning up executables...$(NC)"
	@rm -f hello fibonacci factorial loops conditions print_demo for_loops_demo \
		range_demo list_methods import_demo multi_import lists_demo comprehensive \
		arrays_test class_demo class_tests dict_stress_test dict_test dict_update_test \
		for_loop_test for_loop_with_data fstring_test list_simple list_test lists_complete \
		print_test private_method_test private_test string_concat string_features test

# Clean debug artifacts
.PHONY: clean
clean:
	@echo "$(YELLOW)Cleaning debug build artifacts...$(NC)"
	@$(CARGO) clean --profile dev
	@rm -f *.o examples/*.o tests/*.o
	@rm -f hello fibonacci factorial loops conditions print_demo for_loops_demo \
		range_demo list_methods import_demo multi_import lists_demo comprehensive \
		class_demo test_basic_types test_comparisons test_control_flow \
		test_dictionaries test_for_loops test_functions test_helpers test_imports \
		test_integration test_lists
	@echo "$(GREEN)✓ Debug artifacts cleaned$(NC)"

# Clean all artifacts (debug and release)
.PHONY: clean-all
clean-all:
	@echo "$(YELLOW)Cleaning all build artifacts...$(NC)"
	@$(CARGO) clean
	@rm -f *.o examples/*.o tests/*.o
	@rm -f hello fibonacci factorial loops conditions print_demo for_loops_demo \
		range_demo list_methods import_demo multi_import lists_demo comprehensive \
		class_demo test_basic_types test_comparisons test_control_flow \
		test_dictionaries test_for_loops test_functions test_helpers test_imports \
		test_integration test_lists
	@echo "$(GREEN)✓ All artifacts cleaned$(NC)"

# Show build information
.PHONY: info
info:
	@echo "$(BLUE)WadeScript Build Information$(NC)"
	@echo ""
	@echo "Cargo:       $(CARGO)"
	@echo "LLVM Prefix: $(LLVM_PREFIX)"
	@echo ""
	@echo "Build directories:"
	@echo "  Debug:   $(DEBUG_DIR)"
	@echo "  Release: $(RELEASE_DIR)"
	@echo ""
	@echo "Outputs:"
	@echo "  Compiler (debug):   $(COMPILER_DEBUG)"
	@echo "  Compiler (release): $(COMPILER_RELEASE)"
	@echo "  Runtime (debug):    $(RUNTIME_DEBUG)"
	@echo "  Runtime (release):  $(RUNTIME_RELEASE)"
	@echo ""
	@if [ -f "$(COMPILER_DEBUG)" ]; then \
		echo "$(GREEN)✓ Debug compiler exists$(NC)"; \
	else \
		echo "$(YELLOW)✗ Debug compiler not built$(NC)"; \
	fi
	@if [ -f "$(RUNTIME_DEBUG)" ]; then \
		echo "$(GREEN)✓ Debug runtime exists$(NC)"; \
	else \
		echo "$(YELLOW)✗ Debug runtime not built$(NC)"; \
	fi
	@if [ -f "$(COMPILER_RELEASE)" ]; then \
		echo "$(GREEN)✓ Release compiler exists$(NC)"; \
	else \
		echo "$(YELLOW)✗ Release compiler not built$(NC)"; \
	fi
	@if [ -f "$(RUNTIME_RELEASE)" ]; then \
		echo "$(GREEN)✓ Release runtime exists$(NC)"; \
	else \
		echo "$(YELLOW)✗ Release runtime not built$(NC)"; \
	fi

# Rebuild everything from scratch
.PHONY: rebuild
rebuild: clean-all all

# Format code
.PHONY: fmt
fmt:
	@echo "$(BLUE)Formatting Rust code...$(NC)"
	@$(CARGO) fmt
	@echo "$(GREEN)✓ Code formatted$(NC)"

# Check formatting
.PHONY: fmt-check
fmt-check:
	@echo "$(BLUE)Checking code formatting...$(NC)"
	@$(CARGO) fmt --check

# Run clippy linter
.PHONY: lint
lint:
	@echo "$(BLUE)Running clippy linter...$(NC)"
	@export LLVM_SYS_170_PREFIX=$(LLVM_PREFIX) && \
		$(CARGO) clippy -- -D warnings

# Install (copy ws script to a location in PATH)
.PHONY: install
install: release
	@echo "$(BLUE)Installing WadeScript...$(NC)"
	@echo "$(YELLOW)Note: This copies the release binaries and ws script$(NC)"
	@mkdir -p $$HOME/.local/bin
	@cp $(COMPILER_RELEASE) $$HOME/.local/bin/wadescript
	@cp ws $$HOME/.local/bin/ws
	@chmod +x $$HOME/.local/bin/ws
	@echo "$(GREEN)✓ Installed to ~/.local/bin$(NC)"
	@echo ""
	@echo "Make sure ~/.local/bin is in your PATH:"
	@echo "  export PATH=\"\$$HOME/.local/bin:\$$PATH\""

# Uninstall
.PHONY: uninstall
uninstall:
	@echo "$(YELLOW)Uninstalling WadeScript...$(NC)"
	@rm -f $$HOME/.local/bin/wadescript
	@rm -f $$HOME/.local/bin/ws
	@echo "$(GREEN)✓ Uninstalled$(NC)"

# Development build (with all checks)
.PHONY: dev
dev: check fmt-check lint test
	@echo "$(GREEN)✓ All development checks passed$(NC)"
