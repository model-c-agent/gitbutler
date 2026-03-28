# Tactical Analytics Division -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

Military-grade analytical discipline applied to AI agent orchestration. Our `but-ai` plugin enforces classification boundaries, chain-of-command review, and intelligence-grade audit trails. Every agent output is an intelligence product: sourced, assessed, and verified before delivery.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary. `$PATH` deployment. The binary supports classification levels via a `--classification` flag that restricts which repositories and memory namespaces the agent can access during a given session.

**Classification levels:**
- `UNCLASSIFIED`: Open-source tools and public data only
- `CLIENT-CONFIDENTIAL`: Client-specific data, restricted to named client's namespace
- `INTERNAL`: Division's proprietary analytical models

A single binary, three operational modes. Classification is set at session start and cannot be escalated during the session.

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait. Provider selection constrained by classification:
- `UNCLASSIFIED`: Any provider
- `CLIENT-CONFIDENTIAL`: Anthropic or local models only (contractual data residency requirements)
- `INTERNAL`: Local models only

**Provider clearance:** Each provider undergoes a security assessment before being added to the approved list. Assessment criteria include data handling policy, logging practices, and compliance with the Division's data protection obligations.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce intelligence products (patches). Every product follows the intelligence report format:

**COMMIT.msg structure:**
```
SITREP: [One-line tactical summary]

SITUATION: [What changed in the data/code]
ANALYSIS: [Why this change is correct]
RECOMMENDATION: [Next steps]

Classification: CLIENT-CONFIDENTIAL
Client: [client-id]
Analyst: dr-okonkwo
Reviewed-by: col-harding
```

Review follows the chain of command: Dr. Okonkwo generates, Colonel Harding reviews. No patch ships to a client repo without command review.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait. Repos are separated by classification. Cross-classification coordination requires explicit declassification review.

**Operational coordination:**
- Data ingestion (UNCLASSIFIED) feeds analysis engine (INTERNAL)
- Analysis engine outputs feed client report repos (CLIENT-CONFIDENTIAL)
- Each handoff is a classification boundary crossing and is logged in the audit trail

**No lateral coordination:** Client A's report repo never interacts with Client B's report repo. This is enforced at the forge adapter level.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/tad/intel/<classification>/<client>/`. Classification boundaries are enforced: an agent operating at CLIENT-CONFIDENTIAL for Client A cannot read memory from Client B's namespace.

**Intelligence report format:**
```json
{
  "key": "opponent-high-press-vulnerability-2026-03",
  "value": "Gap opens between LCB and LB during pressing trigger...",
  "source": "Match footage analysis, 5 games, 2025-26 season",
  "reliability": "B",
  "classification": "CLIENT-CONFIDENTIAL",
  "client": "club-x",
  "created": "2026-03-28T08:00:00Z",
  "ttl": 7776000
}
```

**Reliability ratings:** Each memory entry carries a reliability grade (A = confirmed by multiple sources, F = unverified single source). Retrieval weights by reliability: A-rated memories are injected first; F-rated memories are injected only if no higher-rated alternatives exist.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. WO2 Kowalski manages the key infrastructure using military-adapted protocols. Key compromise triggers an immediate operational pause: all in-progress tasks are halted, all recent commits are verified, and operations resume only after the compromised key is revoked and a new key is provisioned.

**Incident response time:** Detection to revocation target: 30 minutes. The Division drills this quarterly.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Col. Harding | 3,500 | 1,000 | 4,500 |
| Dr. Okonkwo | 8,500 | 4,000 | 12,500 |
| Maj. Chen | 5,500 | 700 | 6,200 |
| Capt. Adeyemi | 5,800 | 2,300 | 8,100 |
| WO2 Kowalski | 3,600 | 900 | 4,500 |
| Lt. Nakamura | 2,800 | 600 | 3,400 |
| **Total** | **29,700** | **9,500** | **39,200** |

---

## Unique Insight: Classification-Enforced Memory Isolation

Sports analytics firms serve multiple clients who are direct competitors. A tactical insight about Club A's weakness is valuable to Club B. Most agent memory systems provide no isolation guarantee -- a memory retrieved during one client's analysis could contaminate another's.

Our classification-enforced memory isolation makes cross-client contamination architecturally impossible. Memory namespaces are partitioned by client at the Git ref level. An agent session is bound to a classification level and client ID at startup, and the memory retrieval system physically cannot access refs outside that partition.

This is not access control (which can be misconfigured). It is namespace isolation (which cannot be bypassed without modifying the binary). In military terms: it is not a locked door; it is a separate building.

Three of the Division's five Premier League clients cited data isolation as the primary reason they chose the Division over competitors. The architecture sells itself.

---

*"Intelligence is compartmentalized. Always."*
