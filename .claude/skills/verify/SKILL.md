---
description: Run full CI verification pipeline (format, lint, build, test)
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
