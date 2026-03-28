# Numbers in Color -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**From:** The Commune (Detroit, MI)

---

## Summary

Numbers in Color proposes a `but-ai` implementation that supports dual-practice workflows: forensic investigation and artistic data rendering. The same data passes through two pipelines -- one producing legally defensible evidence, the other producing structured datasets for art installations. The plugin must support both with branch-level isolation and practice-aware memory.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task invocation.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Practice routing: task schema includes a `practice` field (`forensic` or `artistic`) that determines which pipeline processes the output

The practice field is the first thing the plugin reads. It determines branch targeting, memory scope, validation level, and output format.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`.

| Provider | Practice | Notes |
|----------|----------|-------|
| Anthropic | Forensic | Best reasoning for transaction analysis |
| OpenAI | Artistic | Good for structured data generation |
| Ollama | Both | Local models for sensitive case data |
| LMStudio | Development | Local iteration |

Provider can differ by practice within the same project. Forensic tasks may use Anthropic while artistic tasks use OpenAI, based on cost and capability fit.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Dual-Branch Output

Forensic patches target `case/<id>/evidence`. Artistic patches target `installation/<name>/data`. No patch targets both.

### Forensic Commit Format

```
evidence: identify circular transaction pattern between entities A-D

4 entities, 23 transactions, $2.1M total.
Pattern: A->B->C->D->A with consistent 2% markup at each step.

Case: NIC-2026-007
Agent: iris
Evidence-class: secondary
Confidence: HIGH
```

### Artistic Commit Format

```
data: add entity relationship graph for "Cascade II"

7 entities, 340 edges. Edge weight = transaction volume.
Rendering notes: fiber-optic intensity maps to log(volume).

Installation: cascade-ii
Agent: iris
Data-source: NIC-2026-007
Accuracy: verified (forensic branch cross-ref)
```

---

## 4. Polyrepo PR Coordination

Tomas coordinates across case repos, art repos, and gallery partner repos.

### Coordination Schema

```json
{
  "commune": "numbers-in-color",
  "practice": "artistic",
  "installation": "cascade-ii",
  "action": "data_ready",
  "branch": "installation/cascade-ii/data",
  "commit": "abc1234"
}
```

Supported forges: GitHub, GitLab, Forgejo. Gallery partners typically use GitLab. Case clients use GitHub.

---

## 5. Agent Memory in Git Branches

### Practice-Partitioned Memory

Memory stored in `refs/nic/memory/<practice>/<namespace>/<key>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `value` | Memory content |
| `practice` | `forensic` or `artistic` |
| `data_hash` | Integrity hash of underlying data |
| `accuracy` | `verified` or `unverified` |
| `rendering_notes` | Artistic context (artistic entries only) |
| `ttl` | Hours (forensic: 720, artistic: 336) |

### Cross-Practice Reference

An artistic memory entry can reference a forensic entry by hash but cannot access its content directly. This preserves forensic isolation while allowing the art pipeline to confirm that its source data has been verified.

---

## 6. Signed Commits via OpenWallet

Tomas signs all external-facing commits. Internal work-in-progress commits are unsigned.

- Keys provisioned via OpenWallet Verifiable Credentials
- Credential includes: agent name, role, practice authorization
- Rotation: monthly
- Revocation: immediate, stored in `refs/nic/revoked`

Forensic commits carry additional metadata in the credential: case authorization and evidence-class designation.

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Iris | Investigation/art | 8,500 | 4,800 | 13,300 |
| Kip | Data architecture | 5,200 | 1,100 | 6,300 |
| Zhara | Provider/rendering | 3,800 | 900 | 4,700 |
| Tomas | Forge/signing | 4,500 | 2,000 | 6,500 |
| **Commune** | | **22,000** | **8,800** | **30,800** |

---

## 8. Unique Insight: Dual-Practice Branch Isolation

Agent systems typically assume a single workflow per repository. We operate two workflows -- forensic and artistic -- over the same underlying data, with different accuracy requirements, different audiences, and different legal implications.

Our insight: **branch-level practice isolation with cross-practice referencing**. Forensic branches have strict evidence standards. Artistic branches have creative latitude. But artistic branches can reference forensic branches by hash, ensuring that the art is grounded in verified data without inheriting the forensic branch's procedural constraints.

This pattern generalizes beyond our use case. Any project that produces both regulated and unregulated outputs from the same data -- medical research and patient education, legal analysis and public reporting, security audits and engineering documentation -- could benefit from practice-partitioned branches with cross-referencing.

---

*"The numbers are beautiful. Even the ones that represent terrible things."*
