# Kitchen Stadium Elite Proposal — `but-ai` Plugin

**Submitted by:** Kitchen Stadium Elite
**Date:** 2026-03-28

---

## Philosophy

Every task is a timed competition. The clock starts when the task arrives. The clock stops when the patch is plated. Everything between is execution.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary on PATH. CLI mode uses sprint-oriented subcommands: `but ai fire` (start task), `but ai plate` (finalize patch), `but ai clock` (show remaining budget as time equivalent). MCP mode via `but ai mcp`, implementing `ServerHandler` with all ten workspace tools.

WASI degradation: "Training mode" only. Agents can analyze and plan but not commit. Output includes `[TRAINING]` prefix.

Environment: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. All new config under `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` exclusively. No custom LLM client. Provider selection via existing Git config. KSE adds a latency profile per provider:

```
[but-ai "provider-speed"]
    anthropic = 2.1s
    openai = 1.8s
    ollama = 0.9s
    lmstudio = 0.7s
```

The agent selects the fastest available provider that supports the required capabilities (tool calling, structured output). Under time pressure (sprint clock below 25%), the agent automatically switches to the lowest-latency provider.

New providers: config entry + shared library adapter, OpenWallet-signed.

---

## 3.3 The But Agent

Sprint-based execution:

1. **Mise en place:** Read task, inspect workspace, load relevant memory. No output yet.
2. **Fire:** Decompose into components. Each component is a branch (`kse/<dish>/<component>`).
3. **Cook:** Parallel patch generation. Reyes works main components; other agents handle sub-tasks.
4. **Plate:** Assemble final INDEX.patch + COMMIT.msg. Kwon coordinates timing.

All output is INDEX.patch + COMMIT.msg. No direct writes. Branch naming encodes the "dish" (feature) and "component" (sub-task): `kse/auth-refactor/token-validation`.

Budget enforcement: the "clock" metaphor. Total token budget is displayed as time remaining. Agents see `[T-14:00]` and intuitively understand urgency. At `[T-05:00]` (25% budget remaining), agents enter "plating mode" — finalize whatever they have and produce output.

---

## 3.4 Polyrepo PR-Based Coordination

Kwon manages all cross-repo coordination. PR comments use a kitchen call-and-response schema:

```json
{
  "kse_schema": "1.0",
  "call": "fire|heard|behind|pickup|86",
  "component": "sauce-module",
  "agent": "Reyes",
  "clock": "T-11:30",
  "body": "..."
}
```

Call types:
- `fire`: Start work on this component
- `heard`: Acknowledged
- `behind`: I am blocked, need help
- `pickup`: Component ready for integration
- `86`: Component abandoned (kitchen slang for "out of stock")

Forge adapter: trait with `call_out` (create PR), `respond` (comment), `check_board` (list open PRs), `close_ticket` (merge). GitHub implementation provided.

Cross-repo dependencies tracked via PR labels: `kse:needs:<repo>#<n>`. Kwon polls every 60 seconds during active sprints.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/kse/memory/<agent>/<season>`. "Season" here means competition season, not calendar — a new season starts with each major event.

**Structure:** Each memory entry records a "play" — a decision and its outcome:
```json
{
  "play": "used-tamarind-in-dessert",
  "outcome": "judges-loved-it",
  "score": 0.92,
  "competition": "KCGP-2025",
  "ttl": "180d",
  "novelty_uses": 0
}
```

**Relevance scoring:** Score-weighted retrieval. Entries with high competition scores are retrieved first. A "novelty decay" penalizes entries that have been retrieved more than 3 times — the system actively discourages repeating successful plays.

**Compaction survival:** Entries from championship-winning competitions are marked `trophy` and never expire. All other entries follow TTL.

**Identity:** Stored at `refs/kse/roster/<agent>`:
```json
{
  "name": "Reyes",
  "position": "sous-chef",
  "specialization": "patch-generation",
  "speed_rating": 9.2,
  "precision_rating": 7.1,
  "signing_key_id": "kse:reyes:2026S1"
}
```

**Unique insight:** Novelty-penalized memory. Most memory systems optimize for retrieving what worked. KSE's system actively deprioritizes overused memories, forcing agents toward novel approaches. This mirrors competition cooking, where judges reward creativity.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed. Keys provisioned per agent at "team registration" (deployment). Rotated at season boundaries.

**Authorization:**
```json
{
  "Reyes": { "allow": ["kse/*"], "deny": ["main"], "max_lines": 500 },
  "Kwon": { "allow": ["kse/coordination/*"], "deny": ["kse/*/implementation"] },
  "Park": { "allow": ["*"], "role": "head-chef" }
}
```

Only Park can commit to `main`. Sprint branches are open to their assigned agents.

**Key lifecycle:**
- Provisioning: at roster registration via OpenWallet
- Rotation: per-season (quarterly)
- Compromise: immediate revocation, all post-compromise commits flagged, Park notified

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,200 | 0 | Once/session | Roster, tools, current sprint context |
| Task ingestion | 1,800 | 300 | Once/task | Read task, set clock |
| Planning (mise en place) | 1,500 | 800 | Once/task | Component decomposition |
| Tool call (per call) | 1,000 | 500 | 5/task avg | Workspace reads, branch ops |
| Patch generation | 3,500 | 4,500 | Once/task | INDEX.patch |
| Commit message | 400 | 300 | Once/task | COMMIT.msg with clock stamp |
| Memory retrieval | 1,200 | 200 | 2/task avg | Play lookup with novelty decay |
| Coordination event | 1,500 | 600 | 2/task avg | Kitchen calls |
| **TOTAL (typical)** | **18,100** | **9,200** | -- | 200-line feature, timed sprint |

---

## Testing Strategy

1. **Provider speed:** Benchmark each provider's latency. Verify auto-switching under time pressure.
2. **Patch round-trip:** 25-fixture suite with competition-speed constraints (patch must generate in < 5s).
3. **Coordination:** Mock forge with 2 repos simulating a multi-component dish.
4. **Budget/clock:** Verify plating mode triggers at 25% remaining. Verify partial output quality.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Sprint budget |
| `but-ai.agent.platingThreshold` | 0.25 | Budget fraction triggering plating mode |
| `but-ai.agent.memoryRef` | `refs/kse/memory` | Memory prefix |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.noveltyDecay` | 3 | Retrieval count before novelty penalty |
| `but-ai.agent.sprintMinutes` | 20 | Default sprint duration |

---

*"The clock doesn't care about your feelings. Plate the patch."*
