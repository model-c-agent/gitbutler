# Skills: Tools in `scripts/bin/`

This document describes the shell tools available in `scripts/bin/` for working with `but` (GitButler CLI). These are thin wrappers around functions defined in `scripts/but_functions.sh`.

**Setup:** Add `scripts/bin/` to your `$PATH`, or source `scripts/but_functions.sh` directly:
```bash
export PATH="$PWD/scripts/bin:$PATH"
# or
source scripts/but_functions.sh
```

---

## Available Tools

### `but-apply-pattern`

Apply (activate) all branches whose name matches a regex pattern.

```bash
# Apply all branches starting with "pr1/"
but-apply-pattern '^pr1/'

# Preview which branches would match without applying
but-apply-pattern '^pr1/s01' --dry-run

# Apply all feat/ branches
but-apply-pattern 'feat/'
```

**How it works:** Runs `but branch list --json`, filters with `jq` for branches matching the pattern, then calls `but apply <branch>` on each match.

---

### `but-unapply-pattern`

Unapply (deactivate) all currently applied branches whose name matches a regex pattern.

```bash
# Unapply all pr1/ branches
but-unapply-pattern '^pr1/'

# Preview which applied branches match
but-unapply-pattern 'wasi-gate' --dry-run

# Unapply everything except the main feature branch
but-unapply-pattern '^pr1/s0[2-9]'
```

**How it works:** Filters `appliedStacks[].heads[]` from `but branch list --json` for matching names, then calls `but unapply <branch>` on each.

---

### `but-stage-all`

Stage multiple file/hunk change IDs to a branch in a single command.

```bash
# Stage changes rw, mm, pp to the feature-flags branch
but-stage-all pr1/s01/feat/wasi-feature-flags rw mm pp

# Stage all listed IDs to a branch
but-stage-all feat/wasi ab cd ef gh ij
```

**How it works:** Iterates over the provided IDs and calls `but stage <id> <branch>` for each. Reports success/failure count.

---

### `but-amend-all`

Amend all unstaged hunks into a target commit, handling cascading commit ID shifts. Detects and skips hunks stuck in cross-stack lock loops.

```bash
# Amend all unstaged changes into the feat/wasi infrastructure commit
but-amend-all 'chore: add but shell tools'

# Preview what would be amended
but-amend-all 'feat: gate process' --dry-run
```

**How it works:** Iterates over unstaged hunks one at a time. After each `but amend`, re-reads the commit ID (since amending shifts IDs in a stack). If the same hunk reappears 3 times, it's stuck in a cross-stack lock loop and is skipped.

---

## Writing New Tools

When working on this project and a `but` operation is repetitive or missing, **write a general-purpose tool** in `scripts/bin/` rather than running ad-hoc commands.

### Convention

1. **Name:** `but-<verb>-<noun>` (e.g., `but-check-deps`, `but-list-wasi-crates`)
2. **Structure:** Thin wrapper that sources `but_functions.sh` and calls a function:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail
   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
   source "$SCRIPT_DIR/but_functions.sh"
   but-your-function "$@"
   ```
3. **Function definition:** Add the actual logic to `scripts/but_functions.sh`
4. **Help flag:** Always handle `--help` and show usage
5. **Make executable:** `chmod +x scripts/bin/but-your-tool`

### When to Create a Tool

- You're running the same `but` command sequence more than twice
- A multi-step `but` workflow could be automated (e.g., apply deps → rebase → check)
- You need to query `but` JSON output and filter/transform it
- You're blocked and need a workaround that others will also need

### Example: Creating a New Tool

If you need to check which sub-PR branches are applied:

**1. Add function to `scripts/but_functions.sh`:**
```bash
but-list-applied() {
  if [[ "$1" == "--help" ]]; then
    echo "Usage: but-list-applied [pattern]"
    echo "  List currently applied branches, optionally filtered by regex."
    return 0
  fi
  local pattern="${1:-.}"
  but branch list --json 2>/dev/null \
    | jq -r --arg pat "$pattern" \
      '.appliedStacks[] | .heads[] | select(.name | test($pat)) | .name'
}
```

**2. Create wrapper in `scripts/bin/but-list-applied`:**
```bash
#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$SCRIPT_DIR/but_functions.sh"
but-list-applied "$@"
```

**3. Make executable:**
```bash
chmod +x scripts/bin/but-list-applied
```

### Tool Ideas for This Project

These don't exist yet but would be useful — create them as needed:

| Tool | Purpose |
|------|---------|
| `but-list-applied` | List applied branches, optionally filtered by pattern |
| `but-apply-deps` | Given a sub-PR folder name, apply all its dependency branches |
| `but-check-wasi` | Run `cargo check` with WASI features and report first error |
| `but-wasi-tree` | Show dependency tree filtered to WASI-problematic crates |
| `but-sync-branch` | Rebase a sub-PR branch onto `feat/wasi` after deps merge |
| `but-amend-matching` | Amend only hunks matching a file path pattern to a commit |
