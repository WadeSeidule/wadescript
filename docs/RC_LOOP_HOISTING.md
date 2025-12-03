# RC Loop Hoisting Optimization (Phase 4b)

## Overview

Loop hoisting is an optimization that moves invariant operations outside of loops to reduce redundant work. For reference counting, this means detecting variables that are loop-invariant and optimizing their RC operations.

## Goal

Reduce RC overhead in loops from O(n) to O(1) where n is the loop iteration count.

**Expected Impact**: 5-15% improvement in loop-heavy code

## Pattern to Optimize

### Pattern 1: Loop-Invariant Variables

**Before Optimization**:
```wadescript
def process_items() -> void {
    cache: dict[str, int] = {}  // Escaping variable
    for item in items {
        // Even though cache isn't reassigned, if it escapes,
        // we might have RC overhead on each use
        cache[item] = compute(item)
    }
}
```

**Issue**: If `cache` escapes function scope, our conservative analysis might generate RC operations. Even though cache is only assigned once (before the loop), we need to ensure no redundant RC operations happen inside the loop.

### Pattern 2: External References in Loops

**Before Optimization**:
```wadescript
def sum_lengths(lists: list[list[int]]) -> int {
    total: int = 0
    accumulator: list[int] = []  // Escaping
    for sublist in lists {
        // accumulator is modified but not reassigned
        for item in sublist {
            accumulator.push(item)  // Method call on invariant variable
        }
    }
    return accumulator.length
}
```

**Issue**: `accumulator` is defined outside the loops and only modified (not reassigned). It's loop-invariant from an RC perspective.

## Loop-Invariant Definition

A variable is **loop-invariant** if:
1. It's defined outside the loop (not in loop scope)
2. It's not reassigned inside the loop (no `x = ...` statements)
3. It may be read or have methods called on it

Note: A variable can be loop-invariant even if its contents are modified (e.g., `list.push()`, `dict[key] = value`)

## Algorithm

### Step 1: Track Loop Context

Add to CodeGen:
```rust
loop_nesting_depth: usize,  // Current loop depth (0 = not in loop)
loop_invariant_variables: HashSet<String>,  // Variables that are loop-invariant
```

### Step 2: Detect Loop-Invariant Variables

When compiling a loop (While, For):
1. Increment loop_nesting_depth
2. Analyze loop body statements
3. Find variables that are:
   - Accessed inside the loop (reads, method calls)
   - NOT assigned inside the loop
   - Defined in an outer scope (before the loop)
4. Mark these as loop-invariant

### Step 3: Optimize RC Operations

For variables marked as loop-invariant:
- Skip redundant retains inside loops
- Ensure releases happen at the original scope level (not inside loop)
- This is similar to non-escaping optimization but for loop-specific patterns

### Step 4: Safety Constraints

Only apply optimization if:
- Variable is truly not reassigned (checked via AST analysis)
- Variable's lifetime extends beyond the loop
- No control flow (break/continue) would invalidate the analysis

## Implementation Phases

### Phase 4b.1: Detection (Current Phase)
- Add loop_nesting_depth tracking
- Implement loop-invariant variable detection
- Add markers for loop-invariant variables

### Phase 4b.2: Optimization
- Modify RC generation to skip redundant operations for loop-invariant variables
- Ensure scope management respects loop boundaries

### Phase 4b.3: Testing
- Create test cases with nested loops
- Create benchmark showing improvement
- Verify no regressions

## Expected Results

**Benchmark Pattern**:
```wadescript
def bench_loop_hoisting() -> void {
    items: list[int] = [1, 2, 3, 4, 5]  // Escaping
    sum: int = 0

    // Without optimization: potential RC ops on each iteration
    // With optimization: RC ops only at outer scope
    for i in range(10000) {
        sum = sum + items.get(0)  // items is loop-invariant
    }
}
```

**Improvement**: Should see reduction in RC overhead for loop-heavy code, especially with nested loops.

## Current Status

- [ ] Phase 4b.1: Detection (In Progress)
- [ ] Phase 4b.2: Optimization
- [ ] Phase 4b.3: Testing

## Notes

- This optimization is conservative - when in doubt, don't optimize
- Works best in combination with Phase 3 (escape analysis) and Phase 4a (pure functions)
- Most effective for loops with many iterations
