# GitButler (but) shell functions
# Source this from ~/.bashrc and ~/.zshrc

# Core helper: apply or unapply branches matching a regex pattern
_but-pattern() {
  local action="$1" verb="$2" past="$3"
  shift 3

  if [[ "$1" == "--help" || -z "$1" ]]; then
    echo "Usage: but-${action}-pattern <regex> [--dry-run]"
    echo
    echo "  ${verb} all branches whose name matches <regex>."
    echo "  --dry-run  Show matching branches without ${action}ing."
    return 0
  fi

  local pattern="$1"
  local dry_run=false
  [[ "$2" == "--dry-run" ]] && dry_run=true

  local jq_filter
  if [[ "$action" == "unapply" ]]; then
    jq_filter='.appliedStacks[] | .heads[] | select(.name | test($pat)) | .name'
  else
    jq_filter='.branches[] | select(.name | test($pat)) | .name'
  fi

  local matches
  matches=$(but branch list --json 2>/dev/null | jq -r --arg pat "$pattern" "$jq_filter")

  if [[ -z "$matches" ]]; then
    echo "No branches matching '$pattern'"
    return 1
  fi

  echo "Branches matching '$pattern':"
  echo "$matches" | sed 's/^/  /'
  echo

  if $dry_run; then
    echo "(dry run — no changes made)"
    return 0
  fi

  local count=0
  while IFS= read -r branch; do
    echo "${verb}ing: $branch"
    but "$action" "$branch"
    ((count++))
  done <<< "$matches"

  echo "${past} $count branch(es)."
}

but-apply-pattern() { _but-pattern apply Apply Applied "$@"; }
but-unapply-pattern() { _but-pattern unapply Unapply Unapplied "$@"; }

# Stage multiple change IDs to a branch
# Usage: but-stage-all <branch> <id> [id...]
but-stage-all() {
  if [[ "$1" == "--help" || $# -lt 2 ]]; then
    echo "Usage: but-stage-all <branch> <id> [id...]"
    echo
    echo "  Stage multiple file/hunk IDs to a branch."
    echo "  Example: but-stage-all feat/passkey rw mm pp zp kk"
    return 0
  fi

  local branch="$1"
  shift

  local count=0 failed=0
  for id in "$@"; do
    echo "Staging: $id -> $branch"
    if ! but stage "$id" "$branch" 2>&1; then
      ((failed++))
    else
      ((count++))
    fi
  done

  echo "Staged $count/$((count + failed)) to $branch."
  [[ $failed -gt 0 ]] && return 1 || return 0
}

# Show changes: unassigned and per-branch
# Usage: but-changes [--unassigned | --branch <name> | --all]
but-changes() {
  if [[ "${1:-}" == "--help" ]]; then
    echo "Usage: but-changes [--unassigned | --branch <name> | --all | --summary]"
    echo
    echo "  Show change IDs and file paths from 'but status'."
    echo "  --unassigned  Show only unassigned changes (default)"
    echo "  --branch <n>  Show changes assigned to a specific branch"
    echo "  --all         Show unassigned + all branch assignments"
    echo "  --summary     One-line-per-branch count of assigned changes"
    return 0
  fi

  local mode="${1:---unassigned}"
  local status_json
  status_json=$(but status --json 2>/dev/null)

  case "$mode" in
    --unassigned)
      echo "=== Unassigned Changes ==="
      echo "$status_json" | jq -r '.unassignedChanges[] | "\(.cliId)\t\(.filePath)\t\(.changeType)"'
      ;;
    --branch)
      local branch="${2:?branch name required}"
      echo "=== Changes assigned to $branch ==="
      echo "$status_json" | jq -r --arg b "$branch" \
        '.stacks[] | .branches[] | select(.name == $b) | (.assignedChanges // [])[] | "\(.cliId)\t\(.filePath)\t\(.changeType)"'
      ;;
    --all)
      local unassigned
      unassigned=$(echo "$status_json" | jq '.unassignedChanges | length')
      echo "=== Unassigned ($unassigned) ==="
      echo "$status_json" | jq -r '.unassignedChanges[] | "\(.cliId)\t\(.filePath)\t\(.changeType)"'
      echo
      echo "=== Branches ==="
      echo "$status_json" | jq -r \
        '.stacks[] | .branches[] | select((.assignedChanges // []) | length > 0) | .name as $n | .assignedChanges[] | "\($n)\t\(.cliId)\t\(.filePath)\t\(.changeType)"'
      ;;
    --summary)
      echo "=== Summary ==="
      local unassigned
      unassigned=$(echo "$status_json" | jq '.unassignedChanges | length')
      echo "unassigned: $unassigned"
      echo "$status_json" | jq -r \
        '.stacks[] | .branches[] | "\(.name)\t\((.assignedChanges // []) | length)\t\(.commits | length) commits"' | column -t -s$'\t'
      ;;
    *)
      echo "Unknown option: $mode (try --help)"
      return 1
      ;;
  esac
}

# List branches with their cliIds, commit counts, and status
# Usage: but-branch-ids [pattern]
but-branch-ids() {
  if [[ "${1:-}" == "--help" ]]; then
    echo "Usage: but-branch-ids [pattern]"
    echo
    echo "  List applied branches with cliId, name, commit count."
    echo "  Optional regex pattern to filter by name."
    return 0
  fi

  local pattern="${1:-.}"
  but status --json 2>/dev/null | jq -r --arg pat "$pattern" \
    '.stacks[] | .branches[] | select(.name | test($pat)) | "\(.cliId)\t\(.name)\t\(.commits | length) commits\t\(.branchStatus // "unknown")"' \
    | column -t -s$'\t'
}

# Show working tree file changes (what git sees, not but's view)
# Usage: but-diff-files [--stat]
but-diff-files() {
  if [[ "${1:-}" == "--help" ]]; then
    echo "Usage: but-diff-files [--stat]"
    echo
    echo "  Show files changed in working tree vs HEAD."
    echo "  --stat  Show diffstat instead of just names."
    return 0
  fi

  if [[ "${1:-}" == "--stat" ]]; then
    git diff HEAD --stat 2>/dev/null
  else
    git diff HEAD --name-only 2>/dev/null
  fi
}

# Show commits on a branch with details
# Usage: but-branch-commits <branch-name-or-id>
but-branch-commits() {
  if [[ "${1:-}" == "--help" || -z "${1:-}" ]]; then
    echo "Usage: but-branch-commits <branch-name-or-id>"
    echo
    echo "  Show commits on a branch: id, message, author, changed files."
    return 0
  fi

  local branch="$1"
  but status --json 2>/dev/null | jq -r --arg b "$branch" '
    .stacks[] | .branches[] | select(.name == $b or .cliId == $b) |
    "Branch: \(.name) (\(.cliId))\nCommits: \(.commits | length)\n",
    (.commits[] |
      "  \(.cliId) \(.message | split("\n")[0])\n" +
      "    author: \(.authorName) <\(.authorEmail)>\n" +
      "    files:  \((.changes // []) | length)"
    )
  '
}

# Amend all unstaged hunks to a target commit, handling cascading ID shifts.
# Usage: but-amend-all <commit-message-grep> [--dry-run]
#
# Since `but amend` changes the commit ID, this re-reads the commit ID after
# each amend. The grep pattern matches against `but status` output to find
# the target commit.
but-amend-all() {
  if [[ "${1:-}" == "--help" || -z "${1:-}" ]]; then
    echo "Usage: but-amend-all <commit-message-grep> [--dry-run]"
    echo
    echo "  Amend all unstaged hunks into the commit whose message matches"
    echo "  the grep pattern. Handles cascading commit ID shifts."
    echo
    echo "  Example: but-amend-all 'chore: add but shell tools'"
    echo "  Example: but-amend-all 'feat: gate process' --dry-run"
    return 0
  fi

  local pattern="$1"
  local dry_run=false
  [[ "${2:-}" == "--dry-run" ]] && dry_run=true

  local max_iterations=50
  local total_amended=0
  local last_hunk=""
  local repeat_count=0

  for ((i=0; i<max_iterations; i++)); do
    # Get current unstaged hunk IDs
    local hunks
    hunks=$(but status --json 2>/dev/null | jq -r '
      .unassignedChanges[]? | .cliId // empty
    ' 2>/dev/null)

    if [[ -z "$hunks" ]]; then
      break
    fi

    # Get the first hunk
    local hunk
    hunk=$(echo "$hunks" | head -1)

    # Detect infinite loop: same hunk keeps reappearing
    if [[ "$hunk" == "$last_hunk" ]]; then
      repeat_count=$((repeat_count + 1))
      if [[ $repeat_count -ge 3 ]]; then
        echo "WARNING: Hunk $hunk keeps reappearing after amend (locked to multiple commits in stack)."
        echo "  Skipping — this is a known limitation with cross-stack hunk locks."
        # Try the next hunk instead
        hunk=$(echo "$hunks" | sed -n '2p')
        if [[ -z "$hunk" || "$hunk" == "$last_hunk" ]]; then
          echo "No more hunks to try. Remaining unstaged hunks have cross-stack locks."
          break
        fi
        repeat_count=0
      fi
    else
      repeat_count=0
    fi
    last_hunk="$hunk"

    # Find the target commit ID by grepping but status output
    local cid
    cid=$(but status 2>/dev/null | grep -F "$pattern" | awk '{print $2}' | head -1)

    if [[ -z "$cid" ]]; then
      echo "ERROR: No commit matching '$pattern' found in but status"
      return 1
    fi

    if $dry_run; then
      echo "[dry run] Would amend hunk $hunk into $cid ($pattern)"
      total_amended=$((total_amended + 1))
      # In dry run, just count all hunks and break
      local count
      count=$(echo "$hunks" | wc -l)
      echo "[dry run] Total: $count unstaged hunks would be amended"
      return 0
    fi

    echo "Amending $hunk → $cid"
    local result
    result=$(but amend "$hunk" "$cid" 2>&1)
    echo "  $result"

    if echo "$result" | grep -q "Failed"; then
      echo "ERROR: Amend failed for hunk $hunk"
      return 1
    fi

    total_amended=$((total_amended + 1))
  done

  if [[ $total_amended -eq 0 ]]; then
    echo "No unstaged hunks to amend."
  else
    echo "Amended $total_amended hunk(s) into commits matching '$pattern'."
  fi
}
