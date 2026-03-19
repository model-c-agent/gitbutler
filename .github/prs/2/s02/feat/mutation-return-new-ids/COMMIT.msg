feat: return new commit IDs from mutation commands in JSON output

Add commit ID tracking to all mutation commands so that JSON consumers
(agents, scripts, tooling) can observe which commits were created or
rewritten by each operation:

- reword: add JSON output blocks for commit reword, branch rename, and
  no-change paths (new_commit_id, old_commit_id)
- amend: add old_commit_id and commit_mapping (old->new from rebase) to
  both uncommitted_to_commit and assignments_to_commit JSON output
- squash: add source_commit_ids and target_commit_id to JSON output
- absorb: propagate CommitMap from absorb_impl and auto_commit_simple
  through to JSON output as commit_mapping

Plumbing changes:
- CommitMap: add mappings() accessor for reading the internal HashMap
- absorb_impl: return (usize, CommitMap) instead of usize
- auto_commit_simple / apply_commit_changes: return (usize, CommitMap)
- auto_commit (GUI path): destructure tuple but discard CommitMap
- Public but_action::auto_commit_simple: update signature to match
- absorb public API: keep returning usize, discard CommitMap internally
