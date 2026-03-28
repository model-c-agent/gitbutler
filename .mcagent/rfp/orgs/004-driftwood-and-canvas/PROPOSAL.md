# Driftwood & Canvas — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

We design the plugin as two layers: a **functional core** (Rust crate, `crates/but-ai/`) and a **presentation shell** (thin wrapper that formats all output for the target audience).

In CLI mode, every output passes through a formatter that respects `BUT_OUTPUT_FORMAT`. Our human-mode output is not just readable — it is *designed*. Status reports use indentation hierarchies. Errors use natural language. Progress indicators show both percentage and token budget remaining.

In MCP mode, the presentation layer is bypassed. Raw structured JSON flows to the MCP client. The `ServerHandler` implementation wraps the existing `WorkspaceToolset` and adds our three new tools: `AgentTask`, `MemorySearch`, `FormatOutput`.

**WASI degradation:** We propose a "headless" mode where all formatting is stripped and the functional core operates with minimal I/O. Under WASI, `but-ai` becomes a library that returns structured data; the caller is responsible for display.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We wrap `but-llm` in a `ProviderFacade` that normalizes provider differences. Some providers return tool calls as structured objects; others return them embedded in text. The facade handles parsing inconsistencies so downstream code sees a uniform interface.

New providers register via a configuration file (`but-ai.providers.toml`) that maps provider names to adapter modules. Each adapter is a Rust trait implementation compiled as a separate crate. Adding Google Gemini means writing `crates/but-ai-provider-gemini/` — no changes to `but-ai` core or `but-llm`.

**Key design decision:** We do not abstract away provider capabilities. If a provider does not support tool calling, the agent switches to a text-based tool invocation protocol (the agent outputs JSON tool calls in text, and the facade parses them). This is slower and less reliable but ensures all providers work.

## 3. The But Agent (RFP 3.3)

The agent follows our studio workflow: **brief → draft → crit → ship**.

1. **Brief:** Read task description, retrieve relevant memory, establish context. The agent produces an internal work plan (not exposed to the user, consumed internally).
2. **Draft:** Execute tool calls, gather data, generate `INDEX.patch` in unified diff format.
3. **Crit:** Self-review pass. The agent re-reads its own patch and checks for: missing imports, style inconsistencies with surrounding code, patch size versus budget. If issues are found, it revises.
4. **Ship:** Generate `COMMIT.msg` with a structured format: one-line summary, blank line, body explaining *why* not *what*, blank line, token budget consumed.

**Branch naming:** `dc/<role>/<task-id>/<dependency>`. Example: `dc/engineer/T042/s01.s03`.

**Budget enforcement:** Each phase has a budget allocation. If draft exceeds 70% of total budget, crit is shortened to a single check. If draft exceeds 90%, crit is skipped and the patch ships with a `UNCRITIQUED` tag in COMMIT.msg.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```
trait Forge {
    fn open_pr(&self, spec: PrSpec) -> Result<PrHandle>;
    fn post_message(&self, pr: PrHandle, msg: AgentMsg) -> Result<()>;
    fn read_messages(&self, pr: PrHandle) -> Result<Vec<AgentMsg>>;
    fn set_labels(&self, pr: PrHandle, labels: &[&str]) -> Result<()>;
    fn cross_ref(&self, from: PrHandle, to: PrHandle) -> Result<()>;
}
```

Reference implementation: GitHub REST API. The interface is deliberately minimal — we only use features that every forge supports (PRs, comments, labels).

**PR comment format:**

We design agent messages as legible structured comments — readable by both humans and machines. The comment begins with a human-readable summary, followed by a machine-parseable JSON block in a collapsed `<details>` tag. Humans see context; machines see data.

```markdown
**[Agent: dc/engineer]** Task T042 complete. Patch: 127 lines across 2 files.

<details><summary>Agent Data</summary>

```json
{"schema":"dc-msg/v1","type":"status","agent":"dc/engineer","tokens":{"used":18400,"budget":30000}}
```

</details>
```

## 5. Agent Memory and Identity (RFP 3.5)

**Storage scheme:** Memory entries stored as files in a dedicated branch `dc/memory`. Directory structure: `<agent-name>/<category>/<key>.json`. Categories: `patterns`, `decisions`, `context`, `errors`.

**Relevance scoring:** We use a three-factor score: (1) keyword overlap with current task (0.0-0.4), (2) recency with linear decay (0.0-0.3), (3) access frequency — memories retrieved more often score higher (0.0-0.3). Total score determines injection order.

**Compaction survival:** Persistent memories are stored in a `core/` subdirectory within each agent's memory space. During compaction, the agent reconstructs its context from `core/` entries first, then fills remaining budget with highest-scoring ephemeral entries.

**Identity:** Each agent's identity is a signed JSON file at `dc/identity/<agent-name>.json`. Fields: name, role, studio, capabilities, branch permissions, public key fingerprint. The identity file is itself committed with the agent's OpenWallet signature, creating a self-certifying identity record.

**Long-term storage:** A shared `dc/archive` branch serves as the collective's knowledge base. Entries require studio manager sign-off before archival. The archive is indexed by domain tags and searchable via keyword matching against entry summaries.

## 6. Signed Commits via OpenWallet (RFP 3.6)

All agent commits are signed using OpenWallet-managed keys. The signing occurs at the orchestrator level — agents produce unsigned patches, the orchestrator signs when committing.

**Authorization:** Policy defined in `.but-ai/canvas-policy.toml`:

```toml
[agents.engineer-1]
branches = ["dc/engineer/*", "feat/*"]
max_patch_lines = 800
require_crit = true

[agents.designer-1]
branches = ["dc/docs/*", "dc/format/*"]
max_patch_lines = 200
```

The `require_crit` flag means commits from this agent must include a crit review record in their metadata. If absent, the commit is rejected at signing time.

**Key lifecycle:** Provisioned on agent creation. Rotated every 14 days (shorter than default because designers insisted the rotation ceremony should be "more frequent, like changing brushes"). Compromise revocation flags all commits since last known-good state for human re-review.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,500 | 0 | Once/session | Includes formatting guidelines |
| Task ingestion | 2,000 | 300 | Once/task | PR body, branch metadata |
| Planning | 1,200 | 600 | Once/task | Internal work plan |
| Tool call (per call) | 1,000 | 500 | ~4/task | Data gathering |
| Patch generation | 2,800 | 4,000 | Once/task | Draft phase |
| Self-review (crit) | 2,000 | 800 | Once/task | Re-reads own patch |
| Commit message | 600 | 400 | Once/task | Formatted, human-readable |
| Memory retrieval | 500 | 150 | 2/task | Query + inject |
| Coordination event | 1,200 | 600 | 1/task | PR comment with formatting |
| **TOTAL (typical task)** | **17,800** | **10,850** | -- | Includes crit overhead |

## Unique Insight

From two decades of signage design, we know that information has a speed limit. A truck driver reading a sign at 40km/h can absorb about 7 words. A developer scanning a PR with 40 other PRs in their queue can absorb about one sentence. A code reviewer at hour six of a review session is functionally illiterate for anything beyond the diff.

Agent output must be designed for these real conditions. A technically perfect patch with an incomprehensible commit message will be rejected or, worse, merged without understanding. We design every agent output for the worst-case reader: tired, hurried, and skeptical.

---

*"Legibility is not a feature. It is the product."*
