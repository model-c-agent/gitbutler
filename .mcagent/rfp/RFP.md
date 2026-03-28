# Request for Proposals: `but ai` Plugin for GitButler CLI

**Version:** 1.0.0
**Date:** 2026-03-27
**Issuer:** GitButler Core Team
**Status:** Open -- Round 1 (RFP)

---

## Table of Contents

1. [Overview](#1-overview)
2. [Background](#2-background)
3. [Scope of Work](#3-scope-of-work)
4. [Technical Requirements](#4-technical-requirements)
5. [Evaluation Criteria](#5-evaluation-criteria)
6. [Submission Format](#6-submission-format)
7. [Process and Timeline](#7-process-and-timeline)
8. [Appendix A: Codebase Reference](#appendix-a-codebase-reference)
9. [Appendix B: Existing Interfaces](#appendix-b-existing-interfaces)
10. [Appendix C: Token Budget Template](#appendix-c-token-budget-template)

---

## 1. Overview

GitButler seeks proposals for a `but-ai` plugin that extends the `but` CLI with
autonomous AI agent capabilities. The plugin must operate as both a CLI subcommand
(`but ai`) and an MCP (Model Context Protocol) server, abstracting the current
hardcoded AI integration into a provider-agnostic, extensible system.

This RFP is structured as a two-round process:

- **Round 1 (this document):** Defines requirements. Organizations read the RFP
  and prepare proposals.
- **Round 2:** Organizations submit proposals. Proposals are evaluated against
  the criteria in Section 5.

The plugin will replace the existing MCP server at
`crates/but/src/command/legacy/mcp/mod.rs` and become the canonical interface
between AI agents and the GitButler workspace.

---

## 2. Background

### 2.1 The Problem

`but` was designed for one human at a keyboard. It is now operated by AI coding
agents at machine speed. A 100-agent analysis (`.github/agents/reports/SYNTHESIS.md`)
identified the core impedance mismatch:

- **132 redundant `but status --json` calls** in a single session because agents
  have no subscription, cache, or incremental update mechanism.
- **9 broken wrapper tools** that exist solely to translate between what agents
  need and what `but` provides.
- **Silent null commit IDs (F3):** `but commit` returns exit 0 with null commit
  IDs. Agents trust exit codes. Four sessions were lost to this.
- **Background sync reverting agent work (F4):** The sync daemon assumes one
  actor. Agents writing files between sync cycles get silently overwritten.

The central finding: every wrapper tool is scar tissue from the human-to-agent
impedance mismatch. The answer is not a better translator. The answer is teaching
`but` to speak agent natively.

### 2.2 The Agent-Native Epic

The epic at `.github/prs/3/ISSUE.md` defines five phases for making `but`
agent-native: stop the bleeding, batch operations, efficient state observation,
cooperative concurrency, and agent profile and output contract. The `but-ai`
plugin sits alongside this work as the AI-specific companion: where the epic
fixes `but` for all machine consumers, `but-ai` adds intelligence.

### 2.3 The Patch-Based Workflow

GitButler agents do not edit files directly. They produce two artifacts:

- **`INDEX.patch`** -- a unified diff against the current index
- **`COMMIT.msg`** -- the commit message for the patch

The `but` agent is the sole committer. This prevents filesystem contention
when multiple agents operate on the same repository. Branch naming encodes
dependencies (e.g., `s01.s04` means s04 depends on s01).

### 2.4 Current State of AI in GitButler

The existing MCP server exposes a single tool (`gitbutler_update_branches`) that
accepts a prompt, a changes summary, and a working directory path. It routes
through `but_action::handle_changes()` and triggers commit message generation
via a hardcoded OpenAI provider. The LLM provider system (`crates/but-llm/src/lib.rs`)
is already provider-agnostic with four backends (OpenAI, Anthropic, Ollama,
LMStudio), but the MCP server does not use it. The tool system
(`crates/but-tools/src/tool.rs`) provides a `Toolset` trait with `register_tool`,
`get`, `list`, and `call_tool`, and a `WorkspaceToolset` that registers ten
workspace tools (Commit, CreateBranch, Amend, SquashCommits, GetProjectStatus,
MoveFileChanges, GetCommitDetails, GetBranchChanges, SplitBranch, SplitCommit).

None of this is wired to the MCP server. The `but-ai` plugin must bridge this gap.

---

## 3. Scope of Work

Proposals must address all six requirements below. Partial proposals will not be
evaluated.

### 3.1 Plugin Architecture

`but-ai` must be a PATH-based plugin executable. When an executable named `but-ai`
is on PATH, the `but` CLI discovers it via `find_external_subcommand()` in
`crates/but/src/alias.rs` and makes it available as `but ai`.

The plugin must function in two modes:

1. **CLI mode:** `but ai <subcommand>` for interactive and scripted use.
2. **MCP server mode:** `but ai mcp` starts an MCP server on stdio, compatible
   with the Model Context Protocol.

Environment variables available to the plugin at invocation:

| Variable | Source | Description |
|----------|--------|-------------|
| `BUT_WORKSPACE_DIR` | `-C` flag or cwd | Root of the Git workspace |
| `BUT_OUTPUT_FORMAT` | `--format` flag | Output format: `human`, `json`, `shell` |
| `BUT_JSON` | `--json` flag | `"1"` if JSON output requested |

The plugin discovery mechanism is feature-gated with `#[cfg(not(feature = "wasi"))]`.
Proposals must account for this: `but-ai` cannot be discovered as a plugin in
WASI builds. Proposals should describe how AI capabilities degrade gracefully
when running under WASI.

### 3.2 Provider-Agnostic AI Interface

The current `gitbutler_update_branches` MCP tool must be abstracted into a
provider-agnostic interface. The existing `but-llm` crate already supports
four providers:

| Provider | Enum Variant | Config Key |
|----------|-------------|------------|
| OpenAI | `LLMProviderKind::OpenAi` | `gitbutler.aiModelProvider = openai` |
| Anthropic | `LLMProviderKind::Anthropic` | `gitbutler.aiModelProvider = anthropic` |
| Ollama | `LLMProviderKind::Ollama` | `gitbutler.aiModelProvider = ollama` |
| LM Studio | `LLMProviderKind::LMStudio` | `gitbutler.aiModelProvider = lmstudio` |

The `but-llm` provider exposes five interaction methods:

- `tool_calling_loop` -- non-streaming tool-calling loop
- `tool_calling_loop_stream` -- streaming tool-calling loop with token callback
- `stream_response` -- streaming text response without tools
- `structured_output` -- JSON-schema-constrained structured output
- `response` -- simple non-streaming text response

Proposals must:

1. Use the existing `but-llm` crate as the LLM backend, not introduce a new one.
2. Support all four existing providers without modification to `but-llm`.
3. Define a plugin mechanism for adding new providers (e.g., Google Gemini,
   Mistral, local GGUF models). This mechanism must not require recompiling
   `but-ai`.
4. Expose all MCP tools through the provider-agnostic interface, so any tool
   works with any provider that supports tool calling.

### 3.3 The But Agent

The `but-ai` plugin must include an autonomous agent mode (`but ai agent`) that
can execute tasks without human intervention. The agent:

1. **Reads task descriptions** from PR bodies, branch metadata, issue descriptions,
   or explicit CLI arguments.
2. **Produces `INDEX.patch` + `COMMIT.msg`** as its write primitive. The agent
   MUST NOT make direct file edits, run `git commit`, or call `but commit`
   itself. It produces patches; the `but` orchestrator applies them.
3. **Uses branch prefixes** for work isolation. The naming scheme must encode
   the agent's identity and the task's dependency chain. The current convention
   is `s01.s04` (s04 depends on s01), but proposals may extend or replace this
   with a richer encoding.
4. **Operates within a token budget.** The agent must track its own token usage
   and halt gracefully when approaching its budget limit, producing whatever
   partial work it has completed as a valid patch.
5. **Exposes the existing `WorkspaceToolset`** tools to the LLM via tool calling.
   The ten workspace tools (Commit, CreateBranch, Amend, SquashCommits,
   GetProjectStatus, MoveFileChanges, GetCommitDetails, GetBranchChanges,
   SplitBranch, SplitCommit) must be available to the agent's LLM context,
   registered through the `Toolset` trait.
6. **Reports progress** via structured output that can be consumed by both
   humans (terminal) and machines (JSON).

### 3.4 Polyrepo PR-Based Agent Coordination

Agents must coordinate across repositories using pull requests as the
communication medium. This is not a novel message bus -- it reuses the
infrastructure every developer already has.

Requirements:

1. **PRs as inter-agent communication channels.** PRs are not just code review
   artifacts. They are structured messages between agents. A PR from agent A
   in repo X can reference a PR from agent B in repo Y, creating a cross-repo
   dependency graph.

2. **Forge-agnostic.** The coordination protocol must work with:
   - GitHub (REST and GraphQL APIs)
   - GitLab (REST API)
   - Bitbucket (REST API)
   - Self-hosted Gitea/Forgejo
   - Any forge that supports PRs, comments, and labels via HTTP API

   Proposals must define a forge adapter interface and provide at least one
   reference implementation (GitHub is expected; others are bonus).

3. **PR comments as structured messages.** Agent-to-agent communication happens
   via PR comments with a defined schema. Proposals must define this schema.
   At minimum, the schema must support:
   - Task assignment (`@agent-name: please do X`)
   - Status reporting (`task X: completed / blocked / failed`)
   - Dependency declaration (`this PR depends on org/repo#123`)
   - Patch handoff (`here is INDEX.patch for your review`)
   - Budget reporting (`tokens used: N / budget: M`)

4. **Cross-repo coordination via PR references.** An issue in the main repo
   can spawn work in forked repos. The coordination mechanism must track which
   PRs in which repos are related to which task, without requiring all repos
   to be under the same organization or forge.

5. **No proprietary dependencies.** The coordination protocol must not require
   any service beyond the forge itself. No message queues, no databases, no
   SaaS coordination layers. Git and the forge API are the only infrastructure.

### 3.5 Agent Memory and Identity

Each agent must have persistent memory that survives context window compaction
and session boundaries. Proposals must define a storage scheme for agent memory
and identity.

Requirements:

1. **Storage medium.** Memory must be stored in special-purpose Git branches
   or refs within the repository. The exact scheme (branch naming, ref layout,
   file format) is left to proposers. Creativity here is weighted heavily in
   evaluation (see Section 5).

2. **Retrieval.** The agent must be able to query its own memory by relevance.
   A memory entry about "authentication patterns" must be retrievable when the
   agent encounters an authentication-related task, even if the exact keywords
   differ.

3. **Expiration.** Memory entries must have configurable TTL. Stale entries
   (e.g., "branch X has a bug" after the bug is fixed) must expire or be
   explicitly invalidated.

4. **Relevance scoring.** When retrieving memory, entries must be ranked by
   relevance to the current task context. The scoring mechanism must be
   described in detail.

5. **Compaction survival.** When an LLM context window is compacted (summarized
   to fit within token limits), the agent's critical memory must survive. The
   proposal must describe how the agent distinguishes between ephemeral context
   and persistent memory, and how persistent memory is rehydrated after
   compaction.

6. **Long-term storage.** The system should support something analogous to
   "openviking" -- a long-term memory store that agents can contribute to and
   draw from across sessions, tasks, and even repositories. This store should
   be indexed, searchable, and decentralized (stored in Git, not a central
   service).

7. **Identity.** Each agent must have a verifiable identity that persists across
   sessions. The identity must be tied to its commit signing key (see Section 3.6).
   An agent's identity record must include at minimum: name, capabilities,
   authorization scope, and creation timestamp.

### 3.6 Signed Commits via OpenWallet (MANDATED)

This requirement is non-negotiable. All agent commits must be cryptographically
signed using OpenWallet (https://docs.openwallet.sh/).

Requirements:

1. **Every commit produced by an agent must be signed.** Unsigned agent commits
   are rejected. The signing key must be an OpenWallet-managed key.

2. **Agent identity provable from signature.** Given a signed commit, it must be
   possible to determine which agent produced it, what organization the agent
   belongs to, and whether the agent was authorized to commit to that branch
   at that time.

3. **Authorization model.** Proposals must define a scheme for controlling which
   agents can commit to which branches. This is not just "who has the key" but
   "who is authorized by policy." Examples:
   - Agent A can commit to `feat/*` but not `main`
   - Agent B can commit to any branch in repo X but not repo Y
   - Agent C can only commit patches that are under 500 lines

4. **Commits as authorization tokens.** A signed commit is itself an
   authorization artifact. The signature proves: this agent, with this identity,
   was authorized to make this change, at this time, within these constraints.
   Proposals must describe how this authorization chain is verified.

5. **Key lifecycle.** How are agent keys provisioned, rotated, and revoked?
   What happens when an agent's key is compromised? How does the system
   distinguish between "key revoked for rotation" and "key revoked for
   compromise"?

### 3.7 Token Budget (REQUIRED)

Each proposal must include a concrete token budget estimate for a frontier model
(e.g., Claude Opus, GPT-4o, Gemini 1.5 Pro). The budget must cover:

| Component | What to Estimate |
|-----------|-----------------|
| **System prompt** | One-time cost: agent identity, capabilities, tool descriptions, memory context, workspace state summary |
| **Typical task execution** | Per-task cost: reading task description, planning, N tool calls, patch generation, commit message |
| **Memory retrieval** | Per-retrieval cost: query formulation, relevance scoring, memory injection into context |
| **Coordination overhead** | Per-coordination-event cost: reading PR comments, formulating responses, cross-repo references |

The budget must be expressed as a single table with input tokens and output tokens
for each component. The total must include a realistic estimate for a "typical task"
(e.g., "implement a 200-line feature across 3 files with 2 cross-repo dependencies").

---

## 4. Technical Requirements

### 4.1 Language and Build

The `but-ai` plugin may be implemented in any language, but Rust is strongly
preferred for consistency with the rest of the `but` codebase. If Rust is chosen,
the plugin should be structured as one or more crates within the existing workspace
(70+ crates in `crates/`).

### 4.2 MCP Compatibility

The MCP server mode must be compatible with the existing `rmcp` crate used in
`crates/but/src/command/legacy/mcp/mod.rs`. The server must implement the
`ServerHandler` trait and register tools via `tool_router`. The current server
declares:

```
ServerInfo {
    name: "GitButler MCP Server",
    version: "1.0.0",
    protocol_version: ProtocolVersion::LATEST,
    capabilities: ServerCapabilities::builder().enable_tools().build(),
}
```

The `but-ai` MCP server must be a drop-in replacement, maintaining backward
compatibility with existing MCP clients while exposing the expanded tool surface.

### 4.3 Configuration

All configuration must flow through Git config, consistent with the existing
pattern:

```
[gitbutler]
    aiModelProvider = anthropic    # existing key
    aiModel = claude-opus-4-20250514        # existing key
    aiCustomEndpoint = ...        # existing key

[but-ai]
    # New keys proposed by the plugin
    # e.g., agent.tokenBudget, agent.memoryBranch, etc.
```

Proposals must list all new Git config keys they introduce and justify each one.

### 4.4 Error Handling

All errors must be structured. In JSON mode:

```json
{
  "error": {
    "code": "AGENT_BUDGET_EXCEEDED",
    "message": "Agent exhausted token budget (used 48000 of 50000)",
    "context": {
      "budget": 50000,
      "used": 48000,
      "partial_patch": true
    }
  }
}
```

Exit codes must be non-zero for all error conditions. The null-commit-ID pattern
(F3) must not be repeated.

### 4.5 Testing Strategy

Proposals must describe their testing approach, including:

1. How provider-agnostic behavior is tested without live API calls
2. How the patch workflow is validated (INDEX.patch round-trip)
3. How cross-repo coordination is tested without a live forge
4. How token budget enforcement is tested

### 4.6 Compatibility Constraints

- The plugin must not modify the core `but` binary or any existing crate.
- The plugin must work with `but --profile agent` when that feature lands
  (Phase 4 of the agent-native epic).
- The plugin must handle the case where `but-llm` has no configured provider
  gracefully (clear error, not a panic).

---

## 5. Evaluation Criteria

Proposals will be scored on five dimensions:

| Criterion | Weight | What We Look For |
|-----------|--------|-----------------|
| **Technical soundness** | 40% | Is the architecture feasible? Does it integrate with existing crates (`but-llm`, `but-tools`, `but-ctx`)? Are the interfaces well-defined? Does the patch workflow work? |
| **Creativity of memory/identity scheme** | 20% | How does the agent store and retrieve memory? Is the Git-native storage scheme elegant? Does the identity model compose well with OpenWallet? Is the relevance scoring mechanism novel? |
| **Token efficiency** | 15% | Is the token budget realistic? Are there clear optimizations (e.g., lazy tool registration, incremental context, compressed memory)? Does the system prompt fit in < 4000 tokens? |
| **Forge-agnosticism** | 15% | How cleanly does the forge adapter abstract over GitHub/GitLab/Bitbucket/Gitea? Is the PR comment schema portable? Can the coordination protocol work with a forge that only supports basic PR features? |
| **Team composition and agent roster** | 10% | Does the proposing organization have a coherent philosophy? Are the agent profiles well-defined with clear specializations? Does the team structure match the problem structure? |

### 5.1 Bonus Points

- Proposals that demonstrate integration with the WASI compilation target
  (even if limited)
- Proposals that define a migration path from the current MCP server to `but-ai`
  with zero downtime for existing users
- Proposals that address findings from the 100-agent analysis
  (`.github/agents/reports/SYNTHESIS.md`)
- Proposals that include a working prototype (even minimal)

### 5.2 Disqualifying Factors

- Proposals that require a central coordination service beyond Git and forge APIs
- Proposals that bypass the patch-based workflow (direct file edits)
- Proposals that do not use OpenWallet for commit signing
- Proposals that introduce a new LLM client instead of using `but-llm`
- Proposals that modify existing `but` crates

---

## 6. Submission Format

Each proposing organization submits a directory under `.mcagent/rfp/orgs/`:

```
.mcagent/rfp/orgs/NNN-org-name/
  README.md      -- Organization backstory, philosophy, team composition
  AGENTS.md      -- Agent profiles (full roster or team-level descriptions)
  PROPOSAL.md    -- Technical proposal responding to this RFP
```

Where `NNN` is a zero-padded three-digit number assigned in order of submission
(e.g., `001-first-org`, `002-second-org`).

### 6.1 README.md Requirements

The organization README must include:

- **Name and tagline** (one sentence)
- **Philosophy** -- what does the organization believe about AI agents and
  version control? What principles guide its approach?
- **Team composition** -- how many agents, what specializations, how they
  coordinate
- **Prior work** -- what has the organization built before? (This is fiction;
  be creative but coherent.)

### 6.2 AGENTS.md Requirements

Agent profiles must include for each agent:

- **Name and role**
- **Specialization** -- what is this agent uniquely good at?
- **Tools** -- which `but-tools` workspace tools does this agent use?
- **Token budget** -- how much context does this agent need?
- **Failure mode** -- what goes wrong when this agent fails? How does it
  recover?

Organizations may describe agents individually or as teams. A team of 3 agents
with a shared description is acceptable; 50 agents with one-line descriptions
is not.

### 6.3 PROPOSAL.md Requirements

The proposal must respond to each section of this RFP (3.1 through 3.7) with:

1. **Approach** -- how does the proposal address the requirement?
2. **Design** -- concrete interfaces, data structures, protocols
3. **Trade-offs** -- what alternatives were considered and why were they rejected?
4. **Token budget** -- per-component estimates (see Section 3.7)

The proposal must be self-contained. A reviewer should be able to evaluate it
without reading any other file in the repository (though references to codebase
paths for context are encouraged).

---

## 7. Process and Timeline

### 7.1 Two-Round Structure

| Round | Phase | Description |
|-------|-------|-------------|
| **1** | RFP (this document) | Requirements published. Organizations read and prepare. |
| **2** | Proposals | Organizations submit to `.mcagent/rfp/orgs/NNN-org-name/`. Proposals evaluated against Section 5 criteria. |

### 7.2 Clarifications

Questions about this RFP should be raised as comments on the tracking PR or
issue. Clarifications will be appended to this document as numbered amendments
in Appendix D (created when needed).

### 7.3 Selection

After Round 2, proposals will be evaluated and ranked. The top proposal(s) will
be invited to proceed to implementation. Multiple proposals may be selected if
they address complementary aspects of the system.

---

## Appendix A: Codebase Reference

Key file paths in the GitButler repository relevant to this RFP:

| Path | Description |
|------|-------------|
| `crates/but/src/alias.rs` | Plugin discovery: `find_external_subcommand()`, `list_external_subcommands()`. Searches PATH for `but-<name>` executables. Feature-gated with `#[cfg(not(feature = "wasi"))]`. |
| `crates/but/src/args/mod.rs` | CLI argument parsing. Defines `Args` struct with `--format`, `--json`, `--status-after`, `-C` flags. `BUT_OUTPUT_FORMAT` env var defined here. |
| `crates/but/src/command/legacy/mcp/mod.rs` | Current MCP server. Single tool: `gitbutler_update_branches`. Uses `rmcp` crate. Routes through `but_action::handle_changes()`. |
| `crates/but-llm/src/lib.rs` | LLM provider system. `LLMProviderKind` enum (OpenAI, Anthropic, Ollama, LMStudio). `LLMProvider` with 5 methods. Config via `gitbutler.aiModelProvider`. |
| `crates/but-tools/src/tool.rs` | Tool system. `Toolset` trait (`register_tool`, `get`, `list`, `call_tool`). `Tool` trait (`name`, `description`, `parameters`, `call`). `WorkspaceToolset` struct. |
| `crates/but-tools/src/workspace.rs` | Workspace tools. 10 registered tools: Commit, CreateBranch, Amend, SquashCommits, GetProjectStatus, MoveFileChanges, GetCommitDetails, GetBranchChanges, SplitBranch, SplitCommit. |
| `crates/but-ctx/` | Context system. `Context` struct used by all tools. Created via `Context::new_from_legacy_project_and_settings()`. |
| `crates/but-action/` | Action handler. `handle_changes()` function that the MCP server calls. `ActionHandler::HandleChangesSimple` variant. |
| `.github/prs/3/ISSUE.md` | Agent-native epic. 5 phases: stop the bleeding, batch, state observation, cooperative concurrency, agent profile. |
| `.github/agents/reports/SYNTHESIS.md` | 100-agent analysis. 132 redundant status calls, 9 broken tools, F3 null commit IDs, meta-pattern identification. |

---

## Appendix B: Existing Interfaces

### B.1 Tool Trait

```rust
pub trait Tool: 'static + Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    fn call(
        self: Arc<Self>,
        parameters: serde_json::Value,
        ctx: &mut Context,
        commit_mapping: &mut HashMap<ObjectId, ObjectId>,
    ) -> anyhow::Result<serde_json::Value>;
}
```

### B.2 Toolset Trait

```rust
pub trait Toolset {
    fn register_tool<T: Tool>(&mut self, tool: T);
    fn get(&self, name: &str) -> Option<Arc<dyn Tool>>;
    fn list(&self) -> Vec<Arc<dyn Tool>>;
    fn call_tool(&mut self, name: &str, parameters: &str) -> serde_json::Value;
}
```

### B.3 LLM Provider Methods

```rust
impl LLMProvider {
    pub fn new(kind: LLMProviderConfig) -> Option<Self>;
    pub fn from_git_config(config: &gix::config::File<'static>) -> Option<Self>;
    pub fn model(&self) -> Option<String>;

    pub fn tool_calling_loop(
        &self, system_message: &str, chat_messages: Vec<ChatMessage>,
        tool_set: &mut impl Toolset, model: &str,
    ) -> anyhow::Result<String>;

    pub fn tool_calling_loop_stream(
        &self, system_message: &str, chat_messages: Vec<ChatMessage>,
        tool_set: &mut impl Toolset, model: &str,
        on_token: impl Fn(&str) + Send + Sync + 'static,
    ) -> anyhow::Result<(String, Vec<ChatMessage>)>;

    pub fn stream_response(
        &self, system_message: &str, chat_messages: Vec<ChatMessage>,
        model: &str, on_token: impl Fn(&str) + Send + Sync + 'static,
    ) -> anyhow::Result<Option<String>>;

    pub fn structured_output<T: Serialize + DeserializeOwned + JsonSchema>(
        &self, system_message: &str, chat_messages: Vec<ChatMessage>,
        model: &str,
    ) -> anyhow::Result<Option<T>>;

    pub fn response(
        &self, system_message: &str, chat_messages: Vec<ChatMessage>,
        model: &str,
    ) -> anyhow::Result<Option<String>>;
}
```

### B.4 Current MCP Request Schema

```rust
pub struct GitButlerUpdateBranchesRequest {
    pub full_prompt: String,              // The exact user prompt
    pub changes_summary: String,          // Bullet-point summary of changes
    pub current_working_directory: String, // Full root path of the Git project
}
```

### B.5 Plugin Environment Contract

When `but` invokes a plugin, the following are guaranteed:

- The plugin executable is found via PATH search for `but-<name>`
- Three environment variables are set: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`
- The plugin's stdin/stdout/stderr are inherited from the parent process
- The plugin's exit code is propagated to the caller
- Plugin discovery is disabled in WASI builds (`#[cfg(not(feature = "wasi"))]`)

### B.6 Workspace Tools (Registered in WorkspaceToolset)

| Tool | Description |
|------|-------------|
| `Commit` | Create a new commit on a branch |
| `CreateBranch` | Create a new virtual branch |
| `Amend` | Amend an existing commit |
| `SquashCommits` | Squash multiple commits into one |
| `GetProjectStatus` | Get full workspace status |
| `MoveFileChanges` | Move file changes between branches |
| `GetCommitDetails` | Get details of a specific commit |
| `GetBranchChanges` | Get changes for a specific branch |
| `SplitBranch` | Split a branch into multiple branches |
| `SplitCommit` | Split a commit into multiple commits |

---

## Appendix C: Token Budget Template

Proposals must fill in this table with estimates for a frontier model.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | _____  | 0 | Once per session | Agent identity, tool descriptions, workspace state |
| **Task ingestion** | _____ | _____ | Once per task | Reading PR body, issue description, branch metadata |
| **Planning** | _____ | _____ | Once per task | Decomposing task into steps, selecting tools |
| **Tool call (per call)** | _____ | _____ | N per task | Formulating parameters, processing results |
| **Patch generation** | _____ | _____ | Once per task | Producing INDEX.patch content |
| **Commit message** | _____ | _____ | Once per task | Producing COMMIT.msg content |
| **Memory retrieval** | _____ | _____ | 1-3 per task | Query formulation, result injection |
| **Coordination event** | _____ | _____ | 0-5 per task | PR comment read/write, cross-repo reference |
| **TOTAL (typical task)** | _____ | _____ | -- | Sum for a 200-line, 3-file feature with 2 dependencies |

The total should be realistic. An agent that claims 5,000 total tokens for a
meaningful task is not credible. An agent that claims 500,000 tokens for a
simple commit message is wasteful. Justify your numbers.

---

## Appendix D: Amendments

_No amendments at this time. Amendments will be appended here with sequential
numbers, dates, and descriptions of changes._

---

*This RFP was informed by the 100-agent analysis that produced the synthesis
report at `.github/agents/reports/SYNTHESIS.md`, the agent-native epic at
`.github/prs/3/ISSUE.md`, and direct examination of the GitButler codebase
at the referenced paths.*
