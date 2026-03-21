---
description: Guided workflow for adding a new WadeScript language feature
user-invocable: true
argument-hint: "[feature description]"
---

Add a new WadeScript language feature: $ARGUMENTS

Read relevant docs in `docs/` first. Then follow the 11-step checklist in [references/checklist.md](references/checklist.md).

For runtime function additions, see [references/runtime-guide.md](references/runtime-guide.md).

After implementing, run `make test test-rust`. Look for optimization opportunities.
