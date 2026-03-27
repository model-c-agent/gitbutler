#!/usr/bin/env bash
# Layered CoW filesystems for subagent workspaces using fuse-overlayfs.
#
# Each subagent gets an isolated, writable view of the repository.
# Lower layer: real repo (read-only). Upper layer: /dev/shm (RAM-backed tmpfs).
# Writes go to the upper layer; reads fall through to the repo.
#
# Usage:
#   but-overlay-workspaces.sh setup [--pr N] [--repo DIR]
#   but-overlay-workspaces.sh teardown
#   but-overlay-workspaces.sh status
#   but-overlay-workspaces.sh enter <agent-id>
#   but-overlay-workspaces.sh list

set -euo pipefail

WORKSPACE_ROOT="/dev/shm/but-workspaces"
DEFAULT_PR=1

# Colors (if tty)
if [[ -t 1 ]]; then
  GREEN=$'\033[32m' RED=$'\033[31m' YELLOW=$'\033[33m'
  BOLD=$'\033[1m' RESET=$'\033[0m'
else
  GREEN="" RED="" YELLOW="" BOLD="" RESET=""
fi

die()  { echo "${RED}error:${RESET} $*" >&2; exit 1; }
info() { echo "${GREEN}=>${RESET} $*"; }
warn() { echo "${YELLOW}warning:${RESET} $*" >&2; }

# Resolve the repo root (default: git toplevel)
find_repo() {
  local repo="${1:-}"
  if [[ -n "$repo" ]]; then
    echo "$repo"
  else
    git rev-parse --show-toplevel 2>/dev/null || die "not in a git repo and --repo not specified"
  fi
}

# Discover agent IDs from .github/prs/<N>/s*/ directories
discover_agents() {
  local repo="$1" pr="$2"
  local pr_dir="${repo}/.github/prs/${pr}"
  [[ -d "$pr_dir" ]] || die "PR directory not found: ${pr_dir}"

  local agents=()
  for d in "${pr_dir}"/s*/; do
    [[ -d "$d" ]] || continue
    agents+=("$(basename "$d")")
  done

  [[ ${#agents[@]} -gt 0 ]] || die "no subagent directories found in ${pr_dir}"
  printf '%s\n' "${agents[@]}"
}

# Check that fuse-overlayfs is available
require_fuse_overlayfs() {
  command -v fuse-overlayfs >/dev/null 2>&1 \
    || die "fuse-overlayfs not found. Install with: sudo apt install fuse-overlayfs"
}

# Find fusermount binary
find_fusermount() {
  if command -v fusermount3 >/dev/null 2>&1; then
    echo "fusermount3"
  elif command -v fusermount >/dev/null 2>&1; then
    echo "fusermount"
  else
    die "fusermount not found. Install with: sudo apt install fuse3"
  fi
}

cmd_setup() {
  local pr="${DEFAULT_PR}" repo=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --pr)   pr="$2"; shift 2 ;;
      --repo) repo="$2"; shift 2 ;;
      *)      die "unknown option: $1" ;;
    esac
  done

  repo=$(find_repo "$repo")
  require_fuse_overlayfs

  info "repo: ${repo}"
  info "PR: ${pr}"

  # Discover agents
  local agents
  mapfile -t agents < <(discover_agents "$repo" "$pr")
  info "found ${#agents[@]} subagent(s): ${agents[*]}"

  # Create workspace structure on /dev/shm (always tmpfs, no mount needed)
  mkdir -p "${WORKSPACE_ROOT}"/{upper,mnt}

  local count=0 failed=0
  for agent in "${agents[@]}"; do
    local upper="${WORKSPACE_ROOT}/upper/${agent}/upper"
    local work="${WORKSPACE_ROOT}/upper/${agent}/work"
    local mnt="${WORKSPACE_ROOT}/mnt/${agent}"

    mkdir -p "$upper" "$work" "$mnt"

    # Skip if already mounted
    if mountpoint -q "$mnt" 2>/dev/null; then
      warn "${agent}: already mounted at ${mnt}"
      count=$((count + 1))
      continue
    fi

    if fuse-overlayfs \
        -o "lowerdir=${repo}" \
        -o "upperdir=${upper}" \
        -o "workdir=${work}" \
        "$mnt" 2>&1; then
      count=$((count + 1))
    else
      warn "${agent}: fuse-overlayfs mount failed"
      failed=$((failed + 1))
    fi
  done

  echo
  info "${BOLD}${count}${RESET} workspace(s) mounted, ${failed} failed"
  if [[ $count -gt 0 ]]; then
    info "workspaces at: ${WORKSPACE_ROOT}/mnt/<agent-id>/"
    info "teardown with: $0 teardown"
  fi
}

cmd_teardown() {
  local fusermount
  fusermount=$(find_fusermount)

  if [[ ! -d "${WORKSPACE_ROOT}/mnt" ]]; then
    info "no workspaces to tear down"
    return 0
  fi

  local count=0
  for mnt in "${WORKSPACE_ROOT}"/mnt/*/; do
    [[ -d "$mnt" ]] || continue
    local agent
    agent=$(basename "$mnt")

    if mountpoint -q "$mnt" 2>/dev/null; then
      info "unmounting ${agent}"
      "$fusermount" -u "$mnt" 2>/dev/null || warn "failed to unmount ${agent}"
      count=$((count + 1))
    fi
  done

  info "unmounted ${count} workspace(s)"

  # Clean up workspace directory
  rm -rf "${WORKSPACE_ROOT}"
  info "cleaned up ${WORKSPACE_ROOT}"
}

cmd_status() {
  if [[ ! -d "${WORKSPACE_ROOT}/mnt" ]]; then
    info "no workspaces found"
    return 0
  fi

  printf "${BOLD}%-16s %-9s %10s  %s${RESET}\n" "AGENT" "MOUNTED" "UPPER SIZE" "PATH"

  for mnt in "${WORKSPACE_ROOT}"/mnt/*/; do
    [[ -d "$mnt" ]] || continue
    local agent
    agent=$(basename "$mnt")
    local upper="${WORKSPACE_ROOT}/upper/${agent}/upper"

    local mounted="no"
    local upper_size="-"
    local color="$RED"

    if mountpoint -q "$mnt" 2>/dev/null; then
      mounted="yes"
      color="$GREEN"
    fi

    if [[ -d "$upper" ]]; then
      upper_size=$(du -sh "$upper" 2>/dev/null | cut -f1)
    fi

    printf "${color}%-16s %-9s %10s${RESET}  %s\n" "$agent" "$mounted" "$upper_size" "$mnt"
  done
}

cmd_enter() {
  local agent="${1:-}"
  [[ -n "$agent" ]] || die "usage: $0 enter <agent-id>"

  local mnt="${WORKSPACE_ROOT}/mnt/${agent}"
  [[ -d "$mnt" ]] || die "workspace not found: ${mnt}"
  mountpoint -q "$mnt" 2>/dev/null || die "workspace not mounted: ${mnt}"

  echo "$mnt"
}

cmd_list() {
  if [[ ! -d "${WORKSPACE_ROOT}/mnt" ]]; then
    return 0
  fi

  for mnt in "${WORKSPACE_ROOT}"/mnt/*/; do
    [[ -d "$mnt" ]] || continue
    if mountpoint -q "$mnt" 2>/dev/null; then
      echo "$mnt"
    fi
  done
}

cmd_help() {
  cat <<EOF
${BOLD}but-overlay-workspaces${RESET} — Layered CoW filesystems for subagent workspaces

${BOLD}USAGE${RESET}
  $0 <command> [options]

${BOLD}COMMANDS${RESET}
  setup [--pr N] [--repo DIR]
      Create overlay workspaces for all subagents in .github/prs/N/
      --pr N       PR number (default: ${DEFAULT_PR})
      --repo DIR   Repository path (default: git toplevel)

  teardown
      Unmount all workspaces and clean up

  status
      Show mount status and disk usage for all workspaces

  enter <agent-id>
      Print the workspace path for an agent

  list
      Print paths of all mounted workspaces

${BOLD}ARCHITECTURE${RESET}
  Lower layer:  Repository (read-only)
  Upper layer:  /dev/shm (RAM-backed tmpfs, no sudo needed)
  Mount points: ${WORKSPACE_ROOT}/mnt/<agent-id>/
EOF
}

# Dispatch
case "${1:-help}" in
  setup)    shift; cmd_setup "$@" ;;
  teardown) cmd_teardown ;;
  status)   cmd_status ;;
  enter)    shift; cmd_enter "$@" ;;
  list)     cmd_list ;;
  help|--help|-h) cmd_help ;;
  *)        die "unknown command: $1 (try: $0 help)" ;;
esac
