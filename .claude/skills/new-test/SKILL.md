---
description: >
  Scaffold a new WadeScript test file with correct conventions.
  Use when asked to write tests, add test coverage, or create a test
  for a specific feature. Ensures naming, structure, and assertions
  follow project standards.
user-invocable: true
argument-hint: "[feature-name]"
---

Create a new WadeScript test for: $ARGUMENTS

See [references/conventions.md](references/conventions.md) for test patterns and naming.

1. Create the test file following project conventions
2. Run it: `./ws run tests/test_<name>.ws`
3. Run full suite: `make test`
