# NCSA Proposal — `but-ai` Plugin

**Submitted by:** National Culinary Standards Authority
**Date:** 2026-03-28
**RFP Reference:** `but ai` Plugin for GitButler CLI v1.0.0

---

## Executive Summary

The NCSA proposes a `but-ai` plugin architecture modeled on regulatory compliance pipelines: every agent action is traceable to a governing standard, every commit is an auditable record, and every memory entry is versioned against the standard that produced it. We treat the plugin as a regulatory instrument. The workspace is the jurisdiction. The standards are the law.

---

## 3.1 Plugin Architecture

`but-ai` is a standalone Rust binary on PATH. CLI mode dispatches through a subcommand router: `but ai assess`, `but ai standard`, `but ai audit`. MCP mode (`but ai mcp`) starts an `rmcp`-based server implementing the `ServerHandler` trait, registering all ten workspace tools plus three NCSA-specific tools: `ValidateStandard`, `GetStandardVersion`, and `AssessCompliance`.

**WASI degradation:** Under WASI, the plugin runs in read-only advisory mode. It can parse standards and produce assessments but cannot discover external tools or sign commits. Output includes a `WASI_MODE: advisory-only` header.

**Environment:** Reads `BUT_WORKSPACE_DIR` for workspace root. Reads `BUT_OUTPUT_FORMAT` for output formatting. All config via Git config under `[but-ai]`.

---

## 3.2 Provider-Agnostic AI Interface

Uses `but-llm` exclusively. No new LLM client. Provider selection via `gitbutler.aiModelProvider`. The NCSA adds a provider capability matrix stored in Git config:

```
[but-ai "provider-caps"]
    openai = tool-calling,structured-output
    anthropic = tool-calling,structured-output
    ollama = tool-calling
    lmstudio = tool-calling
```

New providers are added by extending this config and implementing a thin adapter that maps the provider's capabilities to the `but-llm` interface. No recompilation needed — the adapter is a shared library loaded at runtime via `libloading`.

**Trade-off:** Runtime loading introduces a small attack surface. Mitigated by requiring adapters to be signed with an OpenWallet key and verified at load time.

---

## 3.3 The But Agent

The NCSA agent operates as a four-stage pipeline:

1. **Standard Resolution** (Bianchi): Identify which standard governs the task
2. **Queue & Prioritize** (Gallo): Assign priority, allocate budget
3. **Produce Patch** (Ferraro): Generate INDEX.patch + COMMIT.msg
4. **Audit** (Mancini): Verify signatures, confirm authorization

Agents produce INDEX.patch and COMMIT.msg exclusively. No direct file writes. No `git commit` calls. Branch naming follows `ncsa/<standard-id>/<task-id>` — encoding the governing standard into the branch name so every branch is traceable to its regulatory basis.

Token budget enforcement: Gallo allocates a token budget per task at queue time. Each agent reports consumption after its phase. If cumulative consumption exceeds 80% of budget before the Documentation phase, Ferraro produces a partial patch with `PARTIAL: budget_constrained` in COMMIT.msg.

---

## 3.4 Polyrepo PR-Based Coordination

The NCSA models PR coordination as inter-agency correspondence. Each PR comment follows a structured schema:

```json
{
  "ncsa_schema": "1.0",
  "type": "assessment|request|status|dependency",
  "srn": "SRN-RAGU-v3.2",
  "from_agent": "Ferraro",
  "body": "...",
  "budget_used": 4200,
  "budget_remaining": 6600
}
```

**Forge adapter:** Defined as a trait with methods: `create_pr`, `comment`, `list_comments`, `add_label`, `get_pr_status`. GitHub reference implementation provided. Adapter selection via `[but-ai] forge = github|gitlab|bitbucket|gitea`.

**Cross-repo:** Dependencies tracked via PR labels (`ncsa:depends-on:<repo>#<pr>`). The coordination agent (Gallo) polls dependent PRs at configurable intervals. No external message bus.

**Trade-off:** Polling is slower than webhooks. Accepted because webhooks require infrastructure the RFP prohibits.

---

## 3.5 Agent Memory and Identity

Memory is stored in `refs/ncsa/memory/<agent-name>/<topic>`. Each memory entry is a JSON blob committed to a dedicated memory branch.

**Schema:**
```json
{
  "key": "ragu-aging-requirement",
  "value": "Minimum pecorino aging changed from 8 to 10 months (SRN-RAGU-v3.2)",
  "created": "2026-03-28T10:00:00Z",
  "ttl": "30d",
  "tags": ["ragu", "pecorino", "standard-change"],
  "srn": "SRN-RAGU-v3.2",
  "relevance_base": 0.8
}
```

**Relevance scoring:** TF-IDF on tags plus a recency decay factor (half-life: 7 days). Memories linked to the currently-active standard receive a 1.5x boost. Maximum 5 entries retrieved per query.

**Compaction survival:** Memories tagged `persistent` survive context compaction. Persistent memories are re-injected at the start of each new context window from the memory branch. Ephemeral memories (TTL < 24h) are not persisted to Git at all — they live only in the agent's current context.

**Identity:** Each agent has an identity record stored at `refs/ncsa/identity/<agent-name>`:
```json
{
  "name": "Ferraro",
  "role": "Documentation Specialist",
  "capabilities": ["patch-generation", "commit-formatting"],
  "scope": ["ncsa/*"],
  "created": "2026-01-15T00:00:00Z",
  "signing_key_id": "ncsa:ferraro:2026Q1"
}
```

**Unique insight:** Memory entries are versioned against the standard that produced them. When a standard is updated, all memory entries referencing the old version are flagged for re-evaluation. This prevents agents from applying outdated precedent — the regulatory equivalent of citing repealed law.

---

## 3.6 Signed Commits via OpenWallet

Every agent commit is signed with an OpenWallet-managed Ed25519 key. Keys are provisioned per-agent at deployment and rotated quarterly.

**Authorization model:** A policy file at `refs/ncsa/policy/authorization.json` defines branch-level access:
```json
{
  "Ferraro": { "allow": ["ncsa/*"], "deny": ["main"], "max_patch_lines": 500 },
  "Mancini": { "allow": ["ncsa/audit/*"], "deny": ["*"], "role": "auditor" }
}
```

**Key lifecycle:**
- **Provisioning:** New keys generated via OpenWallet API at agent creation
- **Rotation:** Automated quarterly. Old key signs a "rotation attestation" before deactivation
- **Revocation for compromise:** Immediate. All commits signed by the compromised key after the suspected compromise date are flagged for re-audit. The key is added to a revocation list at `refs/ncsa/policy/revoked-keys.json`

**Verification chain:** Given a signed commit, verify: (1) signature valid, (2) signing key not revoked, (3) signing agent authorized for target branch per policy, (4) commit timestamp within agent's authorization window.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Agent identity, tool descriptions, active standards |
| Task ingestion | 2,500 | 500 | Once/task | Read PR body, determine governing standard |
| Planning | 1,500 | 800 | Once/task | Decompose task, select pipeline stages |
| Tool call (per call) | 1,200 | 600 | 4/task avg | Standard lookup, branch inspection |
| Patch generation | 3,000 | 4,000 | Once/task | INDEX.patch production |
| Commit message | 500 | 400 | Once/task | SRN-prefixed COMMIT.msg |
| Memory retrieval | 1,500 | 300 | 2/task avg | Standard-versioned memory lookup |
| Coordination event | 2,000 | 800 | 1/task avg | PR comment exchange |
| **TOTAL (typical)** | **19,100** | **9,200** | -- | 200-line, 3-file feature, 2 dependencies |

---

## Testing Strategy

1. **Provider agnosticism:** Mock `but-llm` behind a trait boundary. Test each provider path with recorded responses.
2. **Patch round-trip:** Generate INDEX.patch, apply to a fixture repo, verify state matches expected. Regression suite of 40 patches covering edge cases.
3. **Cross-repo coordination:** Mock forge adapter returning canned PR comment sequences. Test dependency resolution across 3 simulated repos.
4. **Budget enforcement:** Inject a token counter that artificially constrains budget at various thresholds. Verify graceful degradation at 80%, 90%, and 100% consumption.

---

## New Git Config Keys

| Key | Type | Default | Purpose |
|-----|------|---------|---------|
| `but-ai.agent.tokenBudget` | int | 50000 | Per-task token ceiling |
| `but-ai.agent.memoryBranch` | string | `refs/ncsa/memory` | Memory storage ref prefix |
| `but-ai.forge` | string | `github` | Forge adapter selection |
| `but-ai.agent.patchLineLimit` | int | 500 | Maximum INDEX.patch line count |
| `but-ai.agent.memoryttl` | string | `30d` | Default memory entry TTL |
| `but-ai.agent.srnRequired` | bool | true | Require SRN in commit messages |

---

*"An uncertified commit is an unlabeled product. It may be fine. It may be fraud. Without the standard, you cannot tell."*
