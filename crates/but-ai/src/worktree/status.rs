//! Agent status tracking via Git refs.
//!
//! Each agent's status is stored as a JSON blob at
//! `refs/but-ai/agent/<agent-id>/status`. This provides immediate
//! visibility to all co-located agents sharing the same `.git` directory.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::types::{AgentId, TaskPhase, REF_PREFIX};

/// Health state of an agent in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentHealth {
    /// Worktree created, agent process starting.
    Spawning,
    /// Agent is actively working.
    Active,
    /// Agent is waiting on a dependency or resource.
    Blocked,
    /// Agent finished its task successfully.
    Completed,
    /// Agent encountered an unrecoverable error.
    Failed,
    /// Agent was explicitly terminated by the orchestrator.
    Aborted,
}

/// Status record for an agent, serialized to a blob ref.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    /// The agent this status belongs to.
    pub agent_id: AgentId,
    /// Current task phase.
    pub phase: TaskPhase,
    /// Current health state.
    pub health: AgentHealth,
    /// Tokens consumed so far.
    pub tokens_used: u64,
    /// Tokens remaining in the budget.
    pub tokens_remaining: u64,
    /// ISO-8601 timestamp when the agent started.
    pub started_at: String,
    /// ISO-8601 timestamp of the last status update.
    pub updated_at: String,
}

/// Build the ref path for an agent's status.
fn status_ref(agent_id: &AgentId) -> String {
    format!("{REF_PREFIX}/agent/{}/status", agent_id.0)
}

/// Write an agent's status as a JSON blob and update the ref.
pub fn write_status(repo_path: &Path, agent_id: &AgentId, status: &AgentStatus) -> Result<()> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let json = serde_json::to_vec_pretty(status)?;
    let blob_id = repo.write_blob(&json)?.detach();

    let ref_name = status_ref(agent_id);
    repo.reference(
        ref_name.as_str(),
        blob_id,
        gix::refs::transaction::PreviousValue::Any,
        "but-ai: update agent status",
    )?;

    Ok(())
}

/// Read an agent's status from its ref.
///
/// Returns `None` if the ref does not exist.
pub fn read_status(repo_path: &Path, agent_id: &AgentId) -> Result<Option<AgentStatus>> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let ref_name = status_ref(agent_id);
    let reference = repo.try_find_reference(&ref_name)?;

    let reference = match reference {
        Some(r) => r,
        None => return Ok(None),
    };

    let binding = reference.target();
    let target = binding.try_id().context("expected direct ref")?;
    let object = repo.find_object(target)?;
    let status: AgentStatus = serde_json::from_slice(&object.data)?;

    Ok(Some(status))
}

/// Scan `refs/but-ai/agent/` and return all agent IDs with their current status.
pub fn list_agents(repo_path: &Path) -> Result<Vec<(AgentId, AgentStatus)>> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let prefix = format!("{REF_PREFIX}/agent/");
    let mut agents = Vec::new();

    let platform = repo.references()?;
    for reference in platform.all()?.filter_map(Result::ok) {
        let name = reference.name().as_bstr().to_string();
        if !name.starts_with(&prefix) {
            continue;
        }
        // Refs look like: refs/but-ai/agent/<agent-id>/status
        let suffix = &name[prefix.len()..];
        if let Some(agent_id_str) = suffix.strip_suffix("/status") {
            let agent_id = AgentId(agent_id_str.to_string());
            if let Ok(Some(status)) = read_status(repo_path, &agent_id) {
                agents.push((agent_id, status));
            }
        }
    }

    Ok(agents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TaskPhase;

    fn make_status(id: &str) -> AgentStatus {
        AgentStatus {
            agent_id: AgentId(id.to_string()),
            phase: TaskPhase::Implement,
            health: AgentHealth::Active,
            tokens_used: 4200,
            tokens_remaining: 27800,
            started_at: "2026-03-29T14:00:00Z".to_string(),
            updated_at: "2026-03-29T14:15:00Z".to_string(),
        }
    }

    #[test]
    fn roundtrip_status() {
        let dir = tempfile::tempdir().unwrap();
        let td = dir.path();
        gix::init_bare(td).unwrap();

        let status = make_status("impl-01");
        write_status(&td, &AgentId("impl-01".to_string()), &status).unwrap();

        let loaded = read_status(&td, &AgentId("impl-01".to_string()))
            .unwrap()
            .expect("status should exist");

        assert_eq!(loaded.agent_id.0, "impl-01");
        assert_eq!(loaded.health, AgentHealth::Active);
        assert_eq!(loaded.tokens_used, 4200);
    }

    #[test]
    fn read_missing_status_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let result = read_status(path, &AgentId("nonexistent".to_string())).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn list_agents_finds_written_statuses() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        write_status(path, &AgentId("agent-a".to_string()), &make_status("agent-a")).unwrap();
        write_status(path, &AgentId("agent-b".to_string()), &make_status("agent-b")).unwrap();

        let agents = list_agents(path).unwrap();
        assert_eq!(agents.len(), 2);

        let ids: Vec<&str> = agents.iter().map(|(id, _)| id.0.as_str()).collect();
        assert!(ids.contains(&"agent-a"));
        assert!(ids.contains(&"agent-b"));
    }
}
