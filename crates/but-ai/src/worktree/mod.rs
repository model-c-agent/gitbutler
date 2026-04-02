//! Agent lifecycle management via Git worktrees.
//!
//! This module provides the infrastructure for managing agents that run
//! in isolated Git worktrees. Each agent gets its own working tree, HEAD,
//! and index, while sharing the object database and ref store with all
//! other worktrees of the same repository.
//!
//! ## Ref layout
//!
//! ```text
//! refs/but-ai/agent/<agent-id>/
//!   status          -- AgentStatus (phase, health, token counts)
//!   task            -- AgentTask (description, target files, complexity)
//!   output/latest   -- AgentOutput (patch, commit message, files touched)
//! ```
//!
//! ## Co-location
//!
//! Agents sharing a `.git` directory ("co-located agents") have immediate
//! visibility into each other's refs. This eliminates the need for the
//! gossip protocol -- ref writes are instantly visible to all co-located
//! agents.
//!
//! ## Submodules
//!
//! - [`status`]: Agent health and phase tracking
//! - [`task`]: Task assignment and retrieval
//! - [`output`]: Agent output (patches, commit messages)
//! - [`colocation`]: Co-location detection for gossip optimization

pub mod colocation;
pub mod output;
pub mod status;
pub mod task;

use std::path::PathBuf;

use crate::types::AgentId;

pub use colocation::{discover_co_located_agents, is_co_located};
pub use output::{write_output, read_output, AgentOutput};
pub use status::{write_status, read_status, list_agents, AgentHealth, AgentStatus};
pub use task::{assign_task, read_task, clear_task, AgentTask};

/// An agent bound to a specific worktree.
///
/// This struct ties together the agent's identity, its worktree path,
/// and its current status. It serves as the entry point for lifecycle
/// operations: spawning, status updates, task assignment, and output
/// collection.
#[derive(Debug, Clone)]
pub struct AgentWorktree {
    /// The agent's unique identifier.
    pub agent_id: AgentId,
    /// Path to the agent's worktree directory.
    pub worktree_path: PathBuf,
    /// Path to the repository (or any worktree of the same repo).
    pub repo_path: PathBuf,
    /// Current status of the agent.
    pub status: AgentStatus,
}

impl AgentWorktree {
    /// Create a new `AgentWorktree` with the given initial status.
    pub fn new(
        agent_id: AgentId,
        worktree_path: PathBuf,
        repo_path: PathBuf,
        status: AgentStatus,
    ) -> Self {
        Self {
            agent_id,
            worktree_path,
            repo_path,
            status,
        }
    }

    /// Persist the current status to the ref store.
    pub fn flush_status(&self) -> anyhow::Result<()> {
        write_status(&self.repo_path, &self.agent_id, &self.status)
    }

    /// Reload the status from the ref store.
    pub fn refresh_status(&mut self) -> anyhow::Result<()> {
        if let Some(s) = read_status(&self.repo_path, &self.agent_id)? {
            self.status = s;
        }
        Ok(())
    }

    /// Check whether another path belongs to a co-located agent.
    pub fn is_co_located_with(&self, other_path: &std::path::Path) -> bool {
        is_co_located(&self.repo_path, other_path)
    }
}
