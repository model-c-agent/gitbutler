# BioMask Proposal — `but-ai` Plugin

**Submitted by:** BioMask
**Date:** 2026-03-28

---

## Doctrine

Intelligence is perishable. The system must process, package, and deliver it before the adversary adapts. Security is not a feature — it is the operating environment.

---

## 3.1 Plugin Architecture

`but-ai` as a statically-linked Rust binary (no dynamic deps). CLI mode: `but ai collect` (ingest raw data), `but ai analyze` (run correlation), `but ai brief` (produce intelligence patch), `but ai threat` (assess/escalate threat level). MCP mode: `but ai mcp`, `ServerHandler`, ten workspace tools plus `ThreatAssess` (compute threat level from accumulated intelligence) and `GraphConsistency` (validate patch against graph integrity constraints).

WASI: Analysis-only mode. Can run graph queries and consistency checks in a sandboxed environment. Useful for law enforcement partners who need read access without write capability.

Config: `[but-ai]` in Git config. Encryption keys in local-only config, never committed.

---

## 3.2 Provider-Agnostic AI Interface

`but-llm` only. Provider isolation per operational tier: collection agents use local providers (Ollama/LMStudio) to avoid sending raw intelligence to cloud APIs. Analysis agents can use cloud providers for non-sensitive reasoning.

```
[but-ai "provider-tier"]
    collection = ollama
    analysis = anthropic
    briefing = anthropic
    opsec = ollama
```

New providers: config + tier classification + keyvault approval.

---

## 3.3 The But Agent

Intelligence cycle:

1. **Collect** (netclaw): Ingest marketplace data. Produce raw intelligence entries.
2. **Analyze** (phantomfin): Correlate, de-anonymize. Produce analysis briefings.
3. **Patch** (graphvenom): Produce INDEX.patch updating the intelligence graph + COMMIT.msg with threat level.
4. **Secure** (keyvault): Sign commit, encrypt sensitive fields, enforce access tiers.
5. **Archive** (echo_null): Store snapshot for historical queries.

Patches only. Branch naming: `bio/<threat-level>/<operation-id>/<sector>` — e.g., `bio/high/op-2026-017/ivory-east-africa`. Threat level in the branch name enables quick filtering.

Budget: allocated per operation. graphvenom 35%, phantomfin 30%, netclaw 15%, keyvault 10%, echo_null 10%. At 80% budget, produce current best intelligence with `PARTIAL: budget constrained` flag.

---

## 3.4 Polyrepo PR-Based Coordination

BioMask coordinates with vetted law enforcement partners via separate repos. Schema:

```json
{
  "biomask_schema": "1.0",
  "type": "intel-brief|request-for-info|status|threat-escalation",
  "threat_level": "HIGH|MEDIUM|LOW",
  "classification": "vetted-only|broad",
  "encrypted_payload": "<base64>",
  "cleartext_summary": "New vendor identified in ivory sector",
  "budget_remaining_pct": 45
}
```

Sensitive content in `encrypted_payload`, decryptable only by target partner. `cleartext_summary` provides enough context for triage without exposing sources.

Forge adapter: trait with `brief` (PR), `annotate` (encrypted comment), `scan` (list active operations), `close_op` (merge/archive). GitHub implementation.

Cross-repo: Intelligence dependencies tracked in encrypted payload. Partners cannot see each other's dependency chains.

---

## 3.5 Agent Memory and Identity

Memory stored in `refs/biomask/intel/<sector>/<threat-level>/`. Organized by trafficking sector (ivory, pangolin, live-animal, etc.) and threat level.

**Structure:**
```json
{
  "key": "vendor-alpha-wallet-cluster",
  "sector": "ivory",
  "threat_level": "HIGH",
  "encrypted_content": "<base64>",
  "cleartext_tags": ["vendor", "bitcoin", "east-africa"],
  "ttl": "90d",
  "snapshot_date": "2026-03-28",
  "confidence": 0.85
}
```

**Relevance scoring:** Sector match is primary. Threat level acts as a boost (HIGH = 2x, MEDIUM = 1x, LOW = 0.5x). BM25 on cleartext tags. Content is decrypted only after relevance filtering. Maximum 5 entries.

**Expiration:** HIGH-threat entries TTL 90 days (operations move fast). MEDIUM TTL 180 days. LOW TTL 365 days. Entries linked to confirmed law enforcement actions are tagged `case-linked` and retained until the case closes.

**Compaction survival:** HIGH-threat entries always survive. Case-linked entries always survive. Everything else follows TTL.

**Identity:** Pseudonymous. At `refs/biomask/identity/<handle>`:
```json
{
  "handle": "graphvenom",
  "role": "graph-analyst",
  "clearance": "tier-2",
  "signing_key_id": "bio:graphvenom:2026Q1"
}
```

**Unique insight:** Threat-weighted memory with encrypted content and cleartext tags. The system determines relevance without decrypting sensitive intelligence — a privacy-preserving retrieval mechanism that protects sources even from the agents themselves.

---

## 3.6 Signed Commits via OpenWallet

Every commit signed with pseudonymous keys. No real identities in the signing chain.

**Authorization:**
```json
{
  "graphvenom": { "allow": ["bio/*"], "deny": ["main", "bio/*/opsec"] },
  "phantomfin": { "allow": ["bio/*/analysis"], "deny": ["bio/*/raw"] },
  "netclaw": { "allow": ["bio/*/raw"], "deny": ["bio/*/analysis", "bio/*/briefing"] },
  "keyvault": { "allow": ["bio/*/opsec", "main"], "role": "security" }
}
```

Only keyvault can commit to main. Intelligence must pass through the full pipeline before publication.

**Key lifecycle:**
- Provisioning: self-service via OpenWallet, pseudonymous
- Rotation: quarterly
- Compromise: immediate isolation. The compromised handle's operational history is reviewed for evidence of adversary access. All intelligence shared by the compromised agent is re-assessed for contamination.

---

## 3.7 Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 2,200 | 0 | Once/session | Handle, tools, active operations |
| Data ingestion | 3,000 | 500 | Once/task | Raw intelligence parsing |
| Correlation analysis | 3,500 | 1,000 | Once/task | De-anonymization, graph mapping |
| Tool call (per call) | 1,000 | 500 | 3/task avg | Branch ops, graph checks |
| Patch generation | 2,500 | 3,500 | Once/task | Graph update INDEX.patch |
| Commit message | 400 | 300 | Once/task | With threat level |
| Memory retrieval | 1,500 | 200 | 2/task avg | Encrypted, tag-filtered |
| Encryption overhead | 200 | 200 | Per encrypted op | Payload formatting |
| Coordination event | 1,800 | 700 | 1/task avg | Encrypted partner brief |
| **TOTAL (typical)** | **19,600** | **8,400** | -- | Single intelligence update |

---

## Testing Strategy

1. **Graph consistency:** Insert malformed patches. Verify GraphConsistency tool rejects them.
2. **Threat escalation:** Accumulate 3 MEDIUM entries for the same vendor. Verify automatic HIGH escalation.
3. **Encrypted retrieval:** Verify tag-based relevance filtering works without decrypting content.
4. **Provider isolation:** Verify collection agents never use cloud providers.
5. **Pseudonymity:** Verify no real-world identifiers appear in any commit, PR, or memory entry.

---

## Git Config Keys

| Key | Default | Purpose |
|-----|---------|---------|
| `but-ai.agent.tokenBudget` | 45000 | Per-operation budget |
| `but-ai.agent.memoryRef` | `refs/biomask/intel` | Memory prefix |
| `but-ai.agent.threatEscalationThreshold` | 3 | MEDIUM entries before auto-HIGH |
| `but-ai.forge` | `github` | Forge adapter |
| `but-ai.agent.encryptionRequired` | true | All sensitive content encrypted |
| `but-ai.agent.collectionProviderLocal` | true | Force local provider for collection |

---

*"The darknet is not dark enough to hide from graph analysis."*
