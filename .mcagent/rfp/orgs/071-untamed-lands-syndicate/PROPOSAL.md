# Untamed Lands Syndicate Proposal — `but-ai` Plugin

**Submitted by:** Untamed Lands Syndicate
**Date:** 2026-03-28

---

## Guiding Principle

Security through compartmentalization. Every design decision serves one question: if this component is compromised, what is exposed?

---

## 3.1 Plugin Architecture

`but-ai` as a statically-linked Rust binary (no dynamic dependencies to tamper with). CLI mode: `but ai brief` (analyze context), `but ai deploy` (produce patch), `but ai comms` (cross-cell coordination). MCP mode: `but ai mcp` with `ServerHandler`, workspace tools, plus `CompartmentCheck` (verify cell isolation).

WASI: Read-only intelligence analysis. No writes, no signing. Suitable for disposable analysis environments.

Config via `[but-ai]` in Git config. Sensitive config (cell keys, signing key paths) stored in local-only Git config, never committed.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` exclusively. No custom client. Provider via `gitbutler.aiModelProvider`.

The Syndicate adds provider isolation: each cell can use a different provider. Cell-kilo on Anthropic, cell-lima on Ollama (local, no API calls leaving the machine). Provider selection is per-cell, configured in local Git config.

```
[but-ai "cell-provider"]
    kilo = anthropic
    lima = ollama
    mike = openai
```

New providers: local config entry + shared library adapter. Adapter must be OpenWallet-signed.

---

## 3.3 The But Agent

Compartmentalized workflow:

1. **Brief** (kilo-1): Analyze workspace. Produce a structured briefing. Share only relevant sections with lima-3.
2. **Deploy** (lima-3): Receive briefing. Produce INDEX.patch + COMMIT.msg using only the briefed context.
3. **Comms** (mike-7): If cross-cell coordination needed, exchange encrypted PR comments.
4. **Sign** (november-2): Verify and sign the commit. Cell-level signature, not individual.

Patches only. No direct writes. Branch naming: `uls/<cell>/<op-id>` — cell identity encoded, individual identity not.

Budget enforcement: each cell has an independent budget. Cell budgets do not pool. If cell-kilo exhausts its analysis budget, cell-lima cannot donate tokens. This prevents a compromised cell from draining the network's budget.

---

## 3.4 Polyrepo PR-Based Coordination

Encrypted PR comments. mike-7 manages all cross-cell communication.

```json
{
  "uls_schema": "1.0",
  "type": "intel|request|status|dependency",
  "from_cell": "kilo",
  "to_cell": "lima",
  "encrypted_body": "<base64-encoded encrypted payload>",
  "cleartext_meta": {
    "urgency": "high|normal|low",
    "budget_remaining_pct": 72
  }
}
```

The `encrypted_body` is decryptable only by the target cell. Cleartext metadata is minimal — urgency and budget percentage. No operational details in cleartext.

Forge adapter: `ForgeAdapter` trait with `signal` (PR), `encode` (encrypted comment), `decode` (decrypt comment), `sweep` (list relevant PRs). GitHub implementation.

Cross-repo: dependencies declared in encrypted body, not in cleartext metadata. Only the involved cells know what depends on what.

**Trade-off:** Encryption overhead adds ~200 tokens per coordination event. Accepted because the alternative is plaintext intelligence in PR comments.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/uls/memory/<cell>/`. Each cell's memory is encrypted with the cell's key. Cross-cell memory sharing requires explicit declassification.

**Structure:**
```json
{
  "intel_id": "k-2026-0328-001",
  "classification": "cell-only|cross-cell|public",
  "encrypted_content": "<base64>",
  "ttl": "14d",
  "tags_cleartext": ["trafficking", "east-africa"],
  "relevance_base": 0.75
}
```

**Relevance scoring:** BM25 on cleartext tags. The actual content is decrypted only after relevance filtering, minimizing the number of entries that are decrypted (and thus exposed in the agent's context).

**Expiration:** Aggressive. Default TTL 14 days. Cross-cell shared entries expire in 7 days (shorter because more people have seen them). Manual extension requires november-2 approval.

**Compaction survival:** Only `cross-cell` entries with relevance > 0.7 survive compaction. Cell-only entries are re-loaded from the encrypted memory branch.

**Identity:** Cell-level, not individual:
```json
{
  "cell": "kilo",
  "function": "intelligence-analysis",
  "capabilities": ["context-reading", "briefing-production"],
  "signing_key_id": "uls:kilo:2026Q1",
  "member_count": "redacted"
}
```

**Unique insight:** Encrypted, compartmentalized memory with cleartext tags for relevance filtering. The system never decrypts more than necessary. This is a zero-knowledge-adjacent approach to agent memory: you can determine relevance without accessing content.

---

## 3.6 Signed Commits via OpenWallet

Cell-level signing. Individual agents do not have personal keys — the cell signs as a unit.

**Authorization:**
```json
{
  "cell-kilo": { "allow": ["uls/kilo/*"], "deny": ["uls/lima/*", "uls/mike/*", "main"] },
  "cell-lima": { "allow": ["uls/lima/*"], "deny": ["uls/kilo/*", "uls/mike/*", "main"] },
  "cell-mike": { "allow": ["uls/*/coordination"], "deny": ["uls/*/implementation"] }
}
```

Cells can only write to their own branches. mike-7 (comms) can write to coordination sub-branches but not implementation branches.

**Key lifecycle:**
- Provisioning: per-cell, at cell creation
- Rotation: monthly. The outgoing key signs a rotation attestation. The new key is distributed to adjacent cells only.
- Compromise: immediate revocation. All intelligence shared by the compromised cell in the current TTL window is flagged for re-evaluation by other cells. The compromised cell is isolated — no further cross-cell communication until re-keyed.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,200 | 0 | Once/session | Cell identity, tools, OpSec rules |
| Briefing (analysis) | 4,000 | 800 | Once/task | Context read, briefing production |
| Briefing transfer | 0 | 0 | Once/task | Internal — no token cost, just data passing |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops |
| Patch generation | 2,500 | 3,500 | Once/task | INDEX.patch from briefing |
| Commit message | 400 | 300 | Once/task | Pseudonymous COMMIT.msg |
| Memory retrieval | 1,800 | 200 | 2/task avg | Encrypted memory, cleartext tag filter |
| Coordination event | 2,000 | 800 | 1/task avg | Encrypted PR comment |
| Encryption overhead | 200 | 200 | Per encrypted msg | Encryption/decryption formatting |
| **TOTAL (typical)** | **17,600** | **8,300** | -- | 200-line feature, compartmentalized |

---

## Testing Strategy

1. **Compartmentalization:** Verify cell-kilo cannot read cell-lima's memory. Verify encrypted messages are unreadable by non-target cells.
2. **Patch from briefing:** Verify lima-3 produces valid patches from kilo-1 briefings without full context.
3. **Key compromise simulation:** Revoke a cell key mid-operation. Verify isolation, verify other cells continue.
4. **Budget isolation:** Verify cells cannot share or pool token budgets.
5. **Forge adapter:** Mock forge with encrypted comment round-trip.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 40000 | Per-cell budget |
| `but-ai.agent.memoryRef` | `refs/uls/memory` | Memory prefix |
| `but-ai.agent.defaultTTL` | `14d` | Memory expiration |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.cell.id` | (required) | Current cell identifier |
| `but-ai.cell.keyPath` | (required) | Path to cell encryption key |

---

*"If you know everyone in the network, the network is already dead."*
