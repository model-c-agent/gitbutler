# AuditGraph -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**From:** Nadia Hasan, CEO

---

## Summary

AuditGraph detects shell company patterns using graph databases. Our `but-ai` proposal brings graph-native thinking to agent memory and finding production. Agents traverse relationship graphs, produce patches that add findings, and store memory as a graph -- not a list. We are a startup, so we optimize for shipping speed, and we are honest about the tradeoffs.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Invoked per task.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Startup: under 100ms -- we process 200k filings/week and startup overhead compounds

No daemon. Each invocation is independent. If the process crashes, the next invocation starts clean.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`.

| Provider | Usage | Notes |
|----------|-------|-------|
| Anthropic | Complex entity analysis | Best for multi-entity relationship extraction |
| OpenAI | Batch processing | Cost-effective for high-volume registry parsing |
| Ollama | Simple entities | Local, fast, low cost |
| LMStudio | Development | Local testing |

Provider selection is per-task-type, not per-session. Registry parsing uses Ollama. Complex analysis uses Anthropic. The adapter routes based on the `task_type` field in the request.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Finding Patches

Each patch adds one finding to the findings database:

```
finding: shell company cluster CLU-2026-0847

Entities: 14
Shared directors: 3
Formation window: 2024-03 to 2024-08
Jurisdictions: Cyprus, BVI, Delaware
Revenue indicators: none detected
Confidence: 0.91
Status: preliminary

Case: AG-2026-0847
Agent: nadia
```

### Commit Convention

One finding per commit. Atomic. If a cluster analysis reveals three sub-patterns, they are three commits. This makes rollback granular.

---

## 4. Polyrepo PR Coordination

AuditGraph coordinates with client repositories where findings are delivered.

### Coordination Format

```json
{
  "org": "auditgraph",
  "client": "<client-id>",
  "action": "finding_delivered",
  "finding_id": "CLU-2026-0847",
  "confidence": 0.91,
  "status": "preliminary",
  "branch": "findings/CLU-2026-0847"
}
```

Supported forges: GitHub (clients), GitLab (internal), Forgejo (dev). The adapter translates the structured format to the forge's API.

---

## 5. Agent Memory in Git Branches

### Graph-Structured Memory

Memory is not a list. It is a graph. Stored in `refs/ag/memory/graph/`.

Each memory entry is a node with typed edges to other entries:

| Field | Description |
|-------|-------------|
| `id` | Node identifier |
| `content` | Memory value |
| `type` | `pattern`, `entity`, `method`, `error` |
| `confidence` | 0.0-1.0 |
| `edges` | Array of `{target, relation}` pairs |
| `ttl_hours` | Expiration (default: 336) |

Edge types: `derived_from`, `contradicts`, `supports`, `related_to`.

### Graph Retrieval

When the agent needs context, it queries the memory graph:
1. Find the most relevant entry to the current task
2. Walk edges up to depth 2
3. Return the subgraph (max 5 nodes)
4. Inject node contents into agent context, ordered by relevance

This captures relationships between memories that flat retrieval misses. A pattern entry connected to a contradiction entry gives the agent richer context than either entry alone.

---

## 6. Signed Commits via OpenWallet

Yuki manages the signing infrastructure.

- Keys provisioned via OpenWallet per agent
- Rotation: weekly
- Revocation: immediate, stored in `refs/ag/revoked`
- Client-facing findings carry Nadia's signature
- Internal commits carry the producing agent's signature

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Nadia | Findings/strategy | 7,500 | 4,200 | 11,700 |
| Soren | Graph/memory | 5,500 | 1,200 | 6,700 |
| Lena | Data pipeline | 4,000 | 1,000 | 5,000 |
| Yuki | Infrastructure | 3,500 | 900 | 4,400 |
| **Total** | | **20,500** | **7,300** | **27,800** |

---

## 8. Unique Insight: Graph-Structured Agent Memory

Flat memory (key-value) loses relationships. When an agent stores "shell company pattern: shared directors" and separately stores "shared directors often use nominee services," the relationship between these two entries is lost. A graph preserves it.

Our memory system stores entries as nodes with typed edges. When retrieval finds the shell company pattern, it follows the `derived_from` edge to the nominee service insight and injects both. When a new entry contradicts an existing one, the `contradicts` edge preserves the disagreement rather than overwriting.

This is how human experts actually remember things -- not as isolated facts but as a web of connected knowledge. An experienced forensic accountant does not recall "shell companies use nominee directors" in isolation. They recall it connected to specific cases, specific jurisdictions, and specific counter-patterns. Graph memory captures this connected structure.

---

*"The graph sees what the spreadsheet hides."*
