---
description: >
  Review recent code changes for correctness, consistency, and missing pieces.
  Use when asked to review code, check work, audit changes, or after completing
  a multi-file implementation to verify nothing was missed.
user-invocable: true
allowed-tools: Bash, Read, Grep, Glob
---

Review recent changes for correctness and completeness.

## Changes to review

!`git diff --stat HEAD~1 2>/dev/null || git diff --stat --staged 2>/dev/null`

Use the checklist in [references/review-checklist.md](references/review-checklist.md). Run `make test test-rust` to verify. Report findings with file:line references.
