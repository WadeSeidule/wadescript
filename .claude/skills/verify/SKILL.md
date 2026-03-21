---
description: >
  Run full CI verification pipeline. Use after making any code changes,
  before committing, or when asked to verify the build. Triggers on:
  editing source files, fixing bugs, adding features, refactoring.
user-invocable: true
allowed-tools: Bash, Read
---

Run the full verification pipeline. Stop on first failure.

1. `make fmt-check` — if fails, run `make fmt` to fix, then re-check
2. `make lint`
3. `make`
4. `make test-rust`
5. `make test`

Report pass/fail summary. See [references/pipeline.md](references/pipeline.md) for troubleshooting.
