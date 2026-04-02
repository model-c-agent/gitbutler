//! Co-location detection for agent worktrees.
//!
//! Two agents are "co-located" when their worktrees share the same `.git`
//! directory (i.e., they are linked worktrees of the same repository).
//! Co-located agents can skip the gossip protocol entirely because
//! their refs are immediately visible to each other.

use std::path::Path;

use anyhow::{Context, Result};

use crate::types::{AgentId, REF_PREFIX};

/// Returns `true` if both paths resolve to the same Git repository
/// (i.e., they share the same `common_dir`).
///
/// This indicates the two paths are worktrees of the same repo and
/// therefore share refs, objects, and config.
pub fn is_co_located(our_path: &Path, their_path: &Path) -> bool {
    let our_repo = match gix::open_opts(our_path, gix::open::Options::isolated()) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let their_repo = match gix::open_opts(their_path, gix::open::Options::isolated()) {
        Ok(r) => r,
        Err(_) => return false,
    };

    our_repo.common_dir() == their_repo.common_dir()
}

/// Discover all agent IDs that have status refs in the given repository.
///
/// This scans `refs/but-ai/agent/` for status refs and returns the
/// extracted agent IDs. Since all co-located worktrees share refs,
/// this returns agents across all worktrees of the same repository.
pub fn discover_co_located_agents(repo_path: &Path) -> Result<Vec<AgentId>> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let prefix = format!("{REF_PREFIX}/agent/");
    let mut agents = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let platform = repo.references()?;
    for reference in platform.all()?.filter_map(Result::ok) {
        let name = reference.name().as_bstr().to_string();
        if !name.starts_with(&prefix) {
            continue;
        }
        // Extract agent ID from refs like:
        //   refs/but-ai/agent/<agent-id>/status
        //   refs/but-ai/agent/<agent-id>/task
        //   refs/but-ai/agent/<agent-id>/output/latest
        let suffix = &name[prefix.len()..];
        if let Some(slash_pos) = suffix.find('/') {
            let agent_id_str = &suffix[..slash_pos];
            if seen.insert(agent_id_str.to_string()) {
                agents.push(AgentId(agent_id_str.to_string()));
            }
        }
    }

    Ok(agents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worktree::status::{write_status, AgentHealth, AgentStatus};
    use crate::types::TaskPhase;

    #[test]
    fn same_repo_is_co_located() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        // The same path should be co-located with itself.
        assert!(is_co_located(path, path));
    }

    #[test]
    fn different_repos_are_not_co_located() {
        let dir_a = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        gix::init_bare(dir_a.path()).unwrap();
        gix::init_bare(dir_b.path()).unwrap();

        assert!(!is_co_located(dir_a.path(), dir_b.path()));
    }

    #[test]
    fn invalid_path_is_not_co_located() {
        let dir = tempfile::tempdir().unwrap();
        gix::init_bare(dir.path()).unwrap();

        assert!(!is_co_located(dir.path(), Path::new("/nonexistent/path")));
    }

    #[test]
    fn discover_finds_agents_with_status_refs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let status = AgentStatus {
            agent_id: AgentId("arch-01".to_string()),
            phase: TaskPhase::Plan,
            health: AgentHealth::Active,
            tokens_used: 0,
            tokens_remaining: 32000,
            started_at: "2026-03-29T14:00:00Z".to_string(),
            updated_at: "2026-03-29T14:00:00Z".to_string(),
        };

        write_status(path, &AgentId("arch-01".to_string()), &status).unwrap();
        write_status(
            path,
            &AgentId("impl-01".to_string()),
            &AgentStatus {
                agent_id: AgentId("impl-01".to_string()),
                ..status
            },
        )
        .unwrap();

        let agents = discover_co_located_agents(path).unwrap();
        assert_eq!(agents.len(), 2);

        let ids: Vec<&str> = agents.iter().map(|a| a.0.as_str()).collect();
        assert!(ids.contains(&"arch-01"));
        assert!(ids.contains(&"impl-01"));
    }
}
