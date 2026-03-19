feat: add `but diff --name-only` flag for file-path-only output

Add a `--name-only` flag to `but diff` that outputs only the paths of
changed files instead of the full diff content, analogous to
`git diff --name-only`.

Supports all diff targets (worktree, commit, branch, hunk assignments)
and both human/shell and JSON output modes. In JSON mode, outputs
`{"files": ["path1", "path2"]}`. The flag conflicts with `--tui` and
`--no-tui` since name-only output is non-interactive.
