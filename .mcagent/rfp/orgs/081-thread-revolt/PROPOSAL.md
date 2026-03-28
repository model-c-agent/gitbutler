# Thread Revolt Proposal — `but-ai` Plugin

**Submitted by:** Thread Revolt
**Date:** 2026-03-28

---

## Manifesto

Code, like patterns, should be free. This proposal builds a plugin that respects contributor autonomy, enforces collective governance, and never locks anyone out.

---

## 3.1 Plugin Architecture

`but-ai` as a Rust binary. CLI mode: `but ai propose` (submit patch proposal), `but ai block` (register objection), `but ai assemble` (check proposal status), `but ai liberate` (run the liberation pipeline). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `ProposalStatus` (check blocking window) and `FreedomNotice` (inject CC license assertion).

WASI: Full read access, pattern validation, and proposal submission. No commit capability. This enables contributors on any platform to participate.

Config: `[but-ai]` in Git config.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider via `gitbutler.aiModelProvider`. Thread Revolt prefers open-source/self-hosted providers (Ollama, LMStudio) on principle. Cloud providers are permitted but flagged in commit metadata: `provider: cloud` vs. `provider: self-hosted`.

```
[but-ai "provider-preference"]
    preferred = ollama,lmstudio
    acceptable = openai,anthropic
```

The system tries preferred providers first and falls back to acceptable providers only if preferred are unavailable. New providers: config entry + adapter.

---

## 3.3 The But Agent

Assembly-governed workflow:

1. **Propose** (any agent): Submit a task proposal with scope, approach, and estimated budget
2. **Blocking window:** 30-minute window for routine tasks, 72 hours for governance changes. Any agent can block.
3. **Execute** (bobbin_ghost): If unblocked, produce INDEX.patch + COMMIT.msg
4. **Validate** (dart_punk): Review for pattern correctness
5. **Archive** (selvage_x): Update the pattern library

Every COMMIT.msg ends with `// FREE AS IN FREEDOM`.

Branch naming: `revolt/<pattern-type>/<proposal-id>` — e.g., `revolt/outerwear/prop-2026-089`.

Budget: collectively allocated. No single agent gets a fixed share — budget is requested per-proposal and approved by the assembly (or auto-approved if unblocked within 30 minutes).

---

## 3.4 Polyrepo PR-Based Coordination

Thread Revolt coordinates with allied collectives (fabric cooperatives, sewing machine repair networks, textile recycling groups). Schema:

```json
{
  "revolt_schema": "1.0",
  "type": "pattern-share|collaboration|solidarity|dependency",
  "license": "CC-BY-SA-4.0",
  "from": "thread-revolt",
  "freedom_notice": true,
  "body": "...",
  "budget_remaining_pct": 58
}
```

Every message asserts the license. Messages from entities that use proprietary licenses are flagged for assembly review.

Forge adapter: trait with `share` (PR), `discuss` (comment), `catalog` (list by pattern type), `merge_pattern` (merge). GitHub implementation — though the collective grumbles about using a proprietary platform.

Cross-repo: Dependencies declared in PR descriptions. Allied collectives can contribute pattern components (e.g., a buttonhole technique) as cross-repo patches.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/revolt/memory/<pattern-type>/`. Organized by garment category: outerwear, tops, bottoms, underwear, accessories.

**Structure:**
```json
{
  "key": "winter-coat-v3-construction",
  "pattern_type": "outerwear",
  "value": "Two-piece sleeve with gusset insert eliminates shoulder binding",
  "attribution": "bobbin_ghost + dart_punk",
  "license": "CC-BY-SA-4.0",
  "ttl": "365d",
  "tags": ["sleeve", "gusset", "outerwear", "construction"]
}
```

**Relevance scoring:** Pattern-type match is primary. BM25 on tags. Attribution proximity provides a small boost — memories created by the same agent working on the current task are slightly preferred (context continuity). Maximum 5 entries.

**Expiration:** Construction technique memories TTL 365 days (techniques do not expire quickly). Operational memories (contributor coordination) TTL 30 days. Attribution records never expire.

**Compaction survival:** Construction techniques and attribution records always survive. Operational memories are dropped.

**Identity:** At `refs/revolt/identity/<handle>`:
```json
{
  "handle": "bobbin_ghost",
  "role": "pattern-maker",
  "joined": "2019-03-15",
  "contributions": 147,
  "signing_key_id": "revolt:bobbin_ghost:2026"
}
```

**Unique insight:** Attribution-preserved memory. In a collective where credit matters (no individual can claim ownership, but contributors deserve recognition), every memory entry tracks who created it. This enables fair attribution in a system with no hierarchy — contribution history, not authority, determines standing.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed with the contributor's pseudonymous key. Collective decisions require 3-of-5 signatures.

**Authorization:**
```json
{
  "bobbin_ghost": { "allow": ["revolt/*"], "deny": ["main"], "max_lines": 500 },
  "dart_punk": { "allow": ["revolt/*/review"], "deny": [] },
  "seam_ripper": { "allow": ["revolt/governance/*"], "deny": ["revolt/*/pattern"] },
  "any_3_of_5": { "allow": ["main"], "role": "assembly-quorum" }
}
```

Merging to main requires 3 signatures. This enforces the assembly's blocking mechanism at the cryptographic level.

**Key lifecycle:**
- Provisioning: self-service. Any contributor generates a key.
- Rotation: voluntary, recommended annually
- Compromise: announced on the collective's Matrix channel. Compromised key's contributions are re-verified by dart_punk. No single compromise can merge to main (requires 3 keys).

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,100 | 0 | Once/session | Handles, tools, collective governance rules |
| Proposal review | 1,000 | 300 | Once/task | Read + blocking check |
| Task ingestion | 1,800 | 400 | Once/task | Pattern specification |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Patch generation | 3,500 | 4,500 | Once/task | INDEX.patch (pattern file) |
| Freedom notice | 0 | 50 | Once/task | Appended to COMMIT.msg |
| Commit message | 400 | 350 | Once/task | With freedom notice |
| Memory retrieval | 1,200 | 200 | 2/task avg | Pattern-type lookup |
| Validation | 1,500 | 800 | Once/task | Construction review |
| Coordination | 1,500 | 600 | 0.5/task avg | Allied collective comms |
| **TOTAL (typical)** | **17,500** | **9,200** | -- | Single pattern, full pipeline |

---

## Testing Strategy

1. **Blocking mechanism:** Submit proposal, block it, verify execution prevented.
2. **Freedom notice:** Verify every commit contains `// FREE AS IN FREEDOM`.
3. **3-of-5 merge:** Verify main merge fails with fewer than 3 signatures.
4. **Pattern validation:** Submit patterns with known construction errors. Verify dart_punk catches them.
5. **Provider preference:** Verify self-hosted providers are tried before cloud.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 40000 | Per-proposal budget |
| `but-ai.agent.memoryRef` | `refs/revolt/memory` | Memory prefix |
| `but-ai.agent.blockingWindowMinutes` | 30 | Routine proposal window |
| `but-ai.agent.governanceWindowHours` | 72 | Governance proposal window |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.quorumSize` | 3 | Signatures required for main merge |

---

*"You cannot own a shape. You cannot own a stitch. // FREE AS IN FREEDOM"*
