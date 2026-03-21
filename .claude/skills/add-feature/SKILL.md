---
description: >
  Guided 11-step workflow for adding a new WadeScript language feature.
  Use when asked to add new syntax, keywords, built-in functions, operators,
  or language constructs. Ensures all pipeline stages are updated correctly.
user-invocable: true
argument-hint: "[feature description]"
---

Add a new WadeScript language feature: $ARGUMENTS

Read relevant docs in `docs/` first. Then follow the 11-step checklist in [references/checklist.md](references/checklist.md).

For runtime function additions, see [references/runtime-guide.md](references/runtime-guide.md).

After implementing, run `make test test-rust`. Look for optimization opportunities.
