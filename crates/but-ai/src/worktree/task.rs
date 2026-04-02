//! Task assignment via Git refs.
//!
//! Each agent's current task is stored as a JSON blob at
//! `refs/but-ai/agent/<agent-id>/task`. The orchestrator writes this
//! ref when spawning an agent; the agent reads it on startup.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::types::{AgentId, REF_PREFIX};

/// A task assigned to an agent, serialized to a blob ref.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Human-readable description of what the agent should do.
    pub description: String,
    /// Files the agent is expected to modify.
    pub target_files: Vec<String>,
    /// Estimated complexity (e.g. "low", "medium", "high").
    pub complexity: String,
    /// ISO-8601 timestamp when the task was assigned.
    pub assigned_at: String,
}

/// Build the ref path for an agent's task.
fn task_ref(agent_id: &AgentId) -> String {
    format!("{REF_PREFIX}/agent/{}/task", agent_id.0)
}

/// Assign a task to an agent by writing a JSON blob to its task ref.
pub fn assign_task(repo_path: &Path, agent_id: &AgentId, task: &AgentTask) -> Result<()> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let json = serde_json::to_vec_pretty(task)?;
    let blob_id = repo.write_blob(&json)?.detach();

    let ref_name = task_ref(agent_id);
    repo.reference(
        ref_name.as_str(),
        blob_id,
        gix::refs::transaction::PreviousValue::Any,
        "but-ai: assign agent task",
    )?;

    Ok(())
}

/// Read the task assigned to an agent.
///
/// Returns `None` if no task ref exists.
pub fn read_task(repo_path: &Path, agent_id: &AgentId) -> Result<Option<AgentTask>> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let ref_name = task_ref(agent_id);
    let reference = repo.try_find_reference(&ref_name)?;

    let reference = match reference {
        Some(r) => r,
        None => return Ok(None),
    };

    let binding = reference.target();
    let target = binding.try_id().context("expected direct ref")?;
    let object = repo.find_object(target)?;
    let task: AgentTask = serde_json::from_slice(&object.data)?;

    Ok(Some(task))
}

/// Clear (delete) the task ref for an agent.
pub fn clear_task(repo_path: &Path, agent_id: &AgentId) -> Result<()> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let ref_name = task_ref(agent_id);
    let reference = repo.try_find_reference(&ref_name)?;

    if let Some(reference) = reference {
        reference.delete()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task() -> AgentTask {
        AgentTask {
            description: "Implement the worktree module".to_string(),
            target_files: vec![
                "crates/but-ai/src/worktree/mod.rs".to_string(),
                "crates/but-ai/src/worktree/status.rs".to_string(),
            ],
            complexity: "medium".to_string(),
            assigned_at: "2026-03-29T14:00:00Z".to_string(),
        }
    }

    #[test]
    fn roundtrip_task() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let agent_id = AgentId("impl-01".to_string());
        let task = make_task();

        assign_task(path, &agent_id, &task).unwrap();

        let loaded = read_task(path, &agent_id)
            .unwrap()
            .expect("task should exist");

        assert_eq!(loaded.description, "Implement the worktree module");
        assert_eq!(loaded.target_files.len(), 2);
        assert_eq!(loaded.complexity, "medium");
    }

    #[test]
    fn read_missing_task_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let result = read_task(path, &AgentId("nobody".to_string())).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn clear_task_removes_ref() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let agent_id = AgentId("impl-01".to_string());
        assign_task(path, &agent_id, &make_task()).unwrap();
        assert!(read_task(path, &agent_id).unwrap().is_some());

        clear_task(path, &agent_id).unwrap();
        assert!(read_task(path, &agent_id).unwrap().is_none());
    }
}
