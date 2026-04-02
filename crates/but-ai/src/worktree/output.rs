//! Agent output management via Git refs.
//!
//! When an agent completes its work, it writes its output (patch, commit
//! message, affected files) as a JSON blob at
//! `refs/but-ai/agent/<agent-id>/output/latest`. The orchestrator reads
//! this to integrate the agent's work into the workspace.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::types::{AgentId, REF_PREFIX};

/// Output produced by an agent after completing a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    /// The unified diff (patch) produced by the agent.
    pub patch: String,
    /// The commit message for the agent's work.
    pub commit_msg: String,
    /// Files modified by the agent.
    pub files_touched: Vec<String>,
    /// Total tokens consumed during the task.
    pub tokens_used: u64,
}

/// Build the ref path for an agent's latest output.
fn output_ref(agent_id: &AgentId) -> String {
    format!("{REF_PREFIX}/agent/{}/output/latest", agent_id.0)
}

/// Write an agent's output as a JSON blob and update the ref.
pub fn write_output(repo_path: &Path, agent_id: &AgentId, output: &AgentOutput) -> Result<()> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let json = serde_json::to_vec_pretty(output)?;
    let blob_id = repo.write_blob(&json)?.detach();

    let ref_name = output_ref(agent_id);
    repo.reference(
        ref_name.as_str(),
        blob_id,
        gix::refs::transaction::PreviousValue::Any,
        "but-ai: write agent output",
    )?;

    Ok(())
}

/// Read an agent's latest output from its ref.
///
/// Returns `None` if the ref does not exist.
pub fn read_output(repo_path: &Path, agent_id: &AgentId) -> Result<Option<AgentOutput>> {
    let repo = gix::open_opts(repo_path, gix::open::Options::isolated())
        .with_context(|| format!("failed to open repo at {}", repo_path.display()))?;

    let ref_name = output_ref(agent_id);
    let reference = repo.try_find_reference(&ref_name)?;

    let reference = match reference {
        Some(r) => r,
        None => return Ok(None),
    };

    let binding = reference.target();
    let target = binding.try_id().context("expected direct ref")?;
    let object = repo.find_object(target)?;
    let output: AgentOutput = serde_json::from_slice(&object.data)?;

    Ok(Some(output))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_output() -> AgentOutput {
        AgentOutput {
            patch: "--- a/foo.rs\n+++ b/foo.rs\n@@ -1 +1 @@\n-old\n+new\n".to_string(),
            commit_msg: "fix: correct the widget alignment".to_string(),
            files_touched: vec!["src/foo.rs".to_string()],
            tokens_used: 8500,
        }
    }

    #[test]
    fn roundtrip_output() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let agent_id = AgentId("impl-01".to_string());
        let output = make_output();

        write_output(path, &agent_id, &output).unwrap();

        let loaded = read_output(path, &agent_id)
            .unwrap()
            .expect("output should exist");

        assert_eq!(loaded.commit_msg, "fix: correct the widget alignment");
        assert_eq!(loaded.files_touched, vec!["src/foo.rs"]);
        assert_eq!(loaded.tokens_used, 8500);
    }

    #[test]
    fn read_missing_output_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let result = read_output(path, &AgentId("nobody".to_string())).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn overwrite_output() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        gix::init_bare(path).unwrap();

        let agent_id = AgentId("impl-01".to_string());

        let output_v1 = make_output();
        write_output(path, &agent_id, &output_v1).unwrap();

        let output_v2 = AgentOutput {
            patch: "updated patch".to_string(),
            commit_msg: "feat: new feature".to_string(),
            files_touched: vec!["src/bar.rs".to_string()],
            tokens_used: 12000,
        };
        write_output(path, &agent_id, &output_v2).unwrap();

        let loaded = read_output(path, &agent_id)
            .unwrap()
            .expect("output should exist");
        assert_eq!(loaded.commit_msg, "feat: new feature");
        assert_eq!(loaded.tokens_used, 12000);
    }
}
