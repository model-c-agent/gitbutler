# Infrastructure Command -- Technical Proposal

**Classification:** UNCLASSIFIED
**RFP:** GitButler `but-ai` Plugin
**Date:** 2026-03-28
**Prepared by:** Torres (COMMAND-2), approved by Stavros (COMMAND-1)

---

## Operational Summary

Infrastructure Command applies military-grade redundancy doctrine to civilian infrastructure. Our `but-ai` proposal extends this doctrine to the AI agent pipeline itself: no single-point failure in the tooling, redundant providers, backup agents for every role, and SOPs for every failure mode. Our unique contribution: redundancy-first agent architecture where every component -- from provider to memory to signing -- has a documented backup and failover procedure.

---

## 1. PATH-Based Plugin Architecture

`but-ai` on PATH. Per-task invocation.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml`
- Redundancy: config supports primary and fallback entries for every setting

```toml
[provider]
primary = "anthropic"
fallback = "ollama"
failover_trigger = "3_consecutive_errors"
```

The plugin tries the primary provider. After 3 consecutive errors, it fails over to the fallback. This is transparent to the calling agent.

---

## 2. Provider-Agnostic AI

`Provider` trait: `complete`, `tool_call`, `stream`, `health_check`.

| Provider | Role | Notes |
|----------|------|-------|
| Anthropic | Primary | Production analysis |
| OpenAI | Secondary | Failover for Anthropic |
| Ollama | Tertiary | Air-gapped / offline backup |
| LMStudio | Development | Local testing |

Every production deployment has at minimum two configured providers. COMMAND-2 monitors provider health and initiates failover when degradation is detected.

The `health_check` method is called before every session. An unhealthy primary triggers immediate failover without waiting for errors.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Assessment Patches

Each patch adds one infrastructure finding:

```
assessment: redundancy gap in power distribution substation PS-12

Asset: 138kV/13.8kV substation, commissioned 1978
Condition: transformer oil DGA indicates incipient fault
Failure probability: MODERATE (0.62) within 36 months
Impact: 4,200 service points, critical facilities include hospital
Redundancy: PARTIAL. One backup transformer, manual switchover (est. 4 hours)
Recommendation: install automatic transfer switch, reduce switchover to <15 minutes

Sector: SE-03
Cycle: 2026-03
Agent: COMMAND-1
Priority: HIGH
```

### Commit Convention

One finding per commit. Priority tag in every commit message. CRITICAL findings trigger immediate notification to COMMAND-3 for client communication.

---

## 4. Polyrepo PR Coordination

COMMAND-3 coordinates with municipal client repos.

### Report Delivery

```json
{
  "unit": "infracom",
  "sector": "SE-03",
  "cycle": "2026-03",
  "action": "assessment_delivered",
  "findings": 12,
  "critical": 2,
  "high": 4,
  "moderate": 6,
  "branch": "assessment/SE-03/2026-03"
}
```

### Dual-Format Reports

Every assessment is delivered in two formats:
1. **Engineering report:** Full technical detail, asset-level findings, failure probability calculations
2. **Council brief:** Plain language, impact estimates, cost ranges, recommended actions

Both are produced from the same data, ensuring consistency.

Forge support: GitHub (municipal clients), GitLab (unit internal), Forgejo.

---

## 5. Agent Memory in Git Branches

### Infrastructure-Typed Memory

Memory stored in `refs/infracom/memory/<system>/<sector>/<key>`.

System types: `water`, `power`, `communications`, `transportation`, `wastewater`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `content` | Memory value |
| `system` | Infrastructure type |
| `sector` | Geographic sector |
| `asset_class` | Pipe, transformer, switch, road, antenna, etc. |
| `failure_mode` | Known failure mode for this asset class |
| `ttl` | Standards: indefinite. Conditions: 720h. Assessments: cycle duration. |

### Cross-System Correlation

A power substation failure affects water pumping stations in the same sector. The memory system supports cross-system queries: when assessing a power asset, retrieve related water and communications assets in the same sector. This enables compound failure analysis -- what happens when the substation fails AND the backup generator at the pump station is under maintenance?

---

## 6. Signed Commits via OpenWallet

COMMAND-4 manages signing.

- All assessment findings are dual-signed: COMMAND-1 (analyst) + COMMAND-4 (integrity)
- Keys provisioned via OpenWallet per assessment cycle
- Rotation: at cycle boundary (30 days)
- Revocation: immediate, stored in `refs/infracom/revoked`
- Municipal deliverables carry the unit's organizational credential in addition to individual agent credentials

### Liability Chain

Assessment findings that inform municipal spending decisions must have an unbroken signing chain. If a finding recommends pipe replacement, the chain proves: COMMAND-1 analyzed the data, COMMAND-4 verified the analysis, and the finding was delivered to the client via COMMAND-3. This chain is designed to withstand legal scrutiny if a recommended repair is not performed and the asset subsequently fails.

---

## 7. Token Budget

| Callsign | Role | Input | Output | Total |
|----------|------|-------|--------|-------|
| COMMAND-1 | Assessment | 9,000 | 5,200 | 14,200 |
| COMMAND-2 | Systems | 3,800 | 800 | 4,600 |
| COMMAND-3 | Operations | 5,200 | 2,500 | 7,700 |
| COMMAND-4 | Integrity | 3,500 | 900 | 4,400 |
| **Unit Total** | | **21,500** | **9,400** | **30,900** |

---

## 8. Unique Insight: Redundancy-First Agent Architecture

Most agent architectures are designed for the happy path: the provider is available, the model responds correctly, the context fits in the window, and the task completes. When something fails, the system crashes, retries, or produces a degraded result.

We propose **redundancy-first architecture**: every component of the agent pipeline has a documented primary, a documented backup, and a documented failover procedure.

| Component | Primary | Backup | Failover Trigger |
|-----------|---------|--------|-----------------|
| Provider | Anthropic | Ollama | 3 consecutive errors |
| Signing key | Current cycle key | Previous cycle key | Key provisioning failure |
| Assessment agent | COMMAND-1 | COMMAND-3 | Context loss or crash |
| Memory store | Current refs | Archived refs | Corruption detected |

This is standard military infrastructure doctrine applied to software: no single-point failure. When COMMAND-1 crashes mid-assessment, COMMAND-3 resumes from the last checkpoint using the documented SOP. When the primary provider is down, the system fails over to Ollama within seconds.

The cost is complexity. The benefit is reliability. For infrastructure assessment -- where a missed finding can mean a pipe burst that floods 200 homes -- reliability wins.

---

*"The pipe will fail. The transformer will fail. The agent will fail. Plan for all three."*
