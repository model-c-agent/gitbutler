# The Provident Order -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We build software to protect the vulnerable. Our `but-ai` plugin is designed for contexts where errors have human consequences: a wrong premium calculation means a farmer cannot afford coverage; a missed claim means a family goes without. Every architectural decision prioritizes correctness and auditability over speed.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary on `$PATH`. `cargo binstall but-ai`. The binary is designed for constrained environments -- the Order's members sometimes work from field locations with limited bandwidth and aging hardware.

**Constraints honored:**
- Binary size under 10MB
- No network calls at startup (works offline until a provider is needed)
- Configuration via environment variables for environments where config files are impractical
- `but-ai --selftest` runs a local validation suite without any provider, confirming the binary is functional

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait. Four providers. The Order's default is Ollama (local) because field locations often lack reliable internet.

**Pastoral override:** When working with models that affect beneficiary populations, Sister Grace can set a `BUT_AI_REVIEW_REQUIRED=true` flag that forces all agent outputs through human review regardless of confidence level. This is not configurable per-agent -- it is a global switch that reflects the Order's commitment to human oversight in sensitive contexts.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. Every patch that modifies actuarial calculations carries a mandatory `Impact-Assessment:` trailer describing, in plain language, how the change affects beneficiary premiums or coverage.

**Example trailer:**
```
Impact-Assessment: Adjusts drought probability from 0.12 to 0.14 for
    Machakos County, Kenya. Estimated premium increase: KES 45/month
    per family. Affects approximately 1,200 enrolled families.
```

Brother Thomas reviews impact assessments with the same rigor he applies to the code itself.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait. GitHub implementation (the Order uses GitHub for public repos; private repos are self-hosted Gitea).

**Dual-forge support:** The adapter handles both GitHub and Gitea transparently. Cross-repo coordination between the public actuarial library (GitHub) and the private claims system (Gitea) uses the same protocol with forge-appropriate authentication.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/provident/wisdom/`. Entries carry pastoral context.

**Memory schema:**
```json
{
  "key": "kenya-drought-machakos-2025",
  "value": "Drought probability 0.14 based on 20-year rainfall data...",
  "pastoral_note": "Machakos families depend on single maize crop. Total loss means school fees unpaid for entire year.",
  "source": "Kenya Met Dept 2025 Annual Report",
  "beneficiary_impact": "1200 families",
  "created": "2026-03-28T08:00:00Z",
  "expires": null
}
```

**No expiration for wisdom:** Brother Declan's institutional memories never expire. Actuarial data memories expire when superseded by newer data but are archived (not deleted) to maintain historical context.

**Pastoral notes in retrieval:** When memory entries are injected into agent context, the `pastoral_note` field is included. This ensures agents are aware of the human stakes behind the data they process. It is not metadata -- it is context.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Brother Liam manages keys. The Order treats signing keys with the same reverence as the seals on official documents: they represent the community's integrity.

**Community attestation:** For patches affecting beneficiary-facing calculations, the commit is co-signed by the patch author and the reviewer. Two signatures on one commit. This mirrors the Order's practice of requiring two members to authorize any disbursement from the mutual aid pools.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Br. Declan | 2,500 | 300 | 2,800 |
| Sr. Aoife | 8,000 | 3,800 | 11,800 |
| Br. Thomas | 4,500 | 1,200 | 5,700 |
| Sr. Maria | 5,200 | 2,000 | 7,200 |
| Br. Liam | 3,400 | 800 | 4,200 |
| Sr. Grace | 2,800 | 500 | 3,300 |
| **Total** | **26,400** | **8,600** | **35,000** |

---

## Unique Insight: Pastoral Context in Agent Memory

Technical memory systems store what the code does. Ours stores who it serves. Every memory entry that relates to a beneficiary population includes a `pastoral_note` explaining the human context in plain language.

When an agent retrieves a memory to inform a task, it does not just see "drought probability: 0.14 for Machakos County." It sees "1,200 families depend on a single maize crop. Total loss means school fees unpaid." This context changes agent behavior. In testing, agents with pastoral notes in context produced patches with more conservative assumptions -- they erred on the side of coverage rather than cost savings.

We do not claim the agent "cares." We claim the agent's output changes when it knows who is affected. That is sufficient for our purposes.

---

*"Behind every number is a name. Behind every name is a family. Calculate accordingly."*
