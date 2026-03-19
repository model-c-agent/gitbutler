fix: return error when but commit produces null commit ID

When all diff specs are rejected during commit creation, the commit
engine returns `new_commit: None`. Previously, the CLI printed
"✓ Created commit unknown" in human mode or `{"commit_id": null}` in
JSON mode, both with exit code 0 — falsely indicating success.

Now the CLI detects this case and:
- In human mode: returns a descriptive error via `bail!()` (exit code 1)
- In JSON mode: outputs `{"ok": false, "error": "commit_produced_no_result"}`
  and exits with code 1

The partial rejection case (some specs rejected but commit still created)
continues to show a warning followed by the success message, unchanged.

Affected file: crates/but/src/command/legacy/commit.rs
