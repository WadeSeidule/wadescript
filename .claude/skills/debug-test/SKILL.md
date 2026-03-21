---
description: Diagnose and fix a failing WadeScript test
user-invocable: true
argument-hint: "[test file or name]"
---

Diagnose and fix: $ARGUMENTS

1. Run the failing test to see the error
2. Read the test to understand expectations
3. Trace through the pipeline to find the bug
4. **Fix the bug, not the test** (project rule)
5. Verify: `make test test-rust`

See [references/debug-strategies.md](references/debug-strategies.md) for diagnosis techniques.
