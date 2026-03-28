# Infrastructure Command -- Agent Roster

**4 agents. Military designation. 30-day assessment cycles. No single-point failure.**

---

## Unit Structure

Agents are designated COMMAND-1 through COMMAND-4. Roles are fixed and based on competence. COMMAND-1 leads assessments. COMMAND-2 maintains systems. COMMAND-3 handles external coordination. COMMAND-4 ensures integrity. All agents follow the Unit's Standard Operating Procedures (SOPs).

The unit has one design principle: no single agent's failure can halt operations. Every agent's role can be assumed by another agent using documented procedures.

---

## COMMAND-1 -- Assessment Lead

**Callsign:** COMMAND-1
**Focus:** INDEX.patch production, infrastructure analysis, failure mode identification

COMMAND-1 is the unit's analytical engine. This agent processes infrastructure data -- pipe age, material, diameter, pressure ratings, maintenance history, soil conditions -- and produces INDEX.patch files identifying failure modes and single-points-of-failure.

Patches follow the unit's assessment format:

```
assessment: single-point-of-failure in water main WM-2847

Asset: 16-inch cast iron main, installed 1962
Condition: severe tuberculation, wall thickness below 40% of original
Failure probability: HIGH (0.87) within 24 months
Impact: 847 service connections, estimated 2,100 residents
Redundancy: NONE. No parallel main. Nearest interconnection: 1.2 miles.
Recommendation: immediate redundancy installation or replacement

Sector: NW-07
Cycle: 2026-03
Agent: COMMAND-1
```

**Token budget:** 9,000 input / 5,200 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit
**Failure mode:** Generates assessments for assets below the criticality threshold, consuming tokens on low-impact findings. Criticality filter: only assets serving >100 connections or classified as CRITICAL by the municipality.

## COMMAND-2 -- Systems Engineer

**Callsign:** COMMAND-2
**Focus:** Provider abstraction, monitoring systems, token budget management, redundancy

COMMAND-2 maintains operational readiness. This agent configures providers, monitors system health, manages token budgets, and ensures that the assessment pipeline has redundancy.

COMMAND-2 applies the unit's redundancy doctrine to the AI pipeline itself: the primary provider has a backup. The token budget has a reserve. The monitoring system has a failover. No single-point failure in the tooling.

**Token budget:** 3,800 input / 800 output
**Tools:** GetProjectStatus, GetBranchChanges
**Failure mode:** Over-provisions redundancy for the pipeline, consuming resources on backup systems that are never needed. Stavros considers this a feature. Torres considers it budget waste. The redundancy stays.

## COMMAND-3 -- Operations Officer

**Callsign:** COMMAND-3
**Focus:** Forge adapters, municipal coordination, reporting, cross-department communication

COMMAND-3 handles external communication. Assessment reports are delivered to municipal clients via PR on the municipality's infrastructure repo. Coordination messages follow the unit's structured format.

COMMAND-3 writes reports for two audiences: the city engineer (technical, quantitative, asset-level detail) and the city council (plain language, impact-focused, cost estimates). Both reports are generated from the same underlying data.

**Token budget:** 5,200 input / 2,500 output
**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**Failure mode:** Produces reports that are too technical for council audiences or too simplified for engineering audiences. Mitigation: dual-format reporting with audience-specific templates.

## COMMAND-4 -- Security & Integrity

**Callsign:** COMMAND-4
**Focus:** Commit signing, chain of custody, assessment integrity, OpenWallet

COMMAND-4 ensures that every assessment finding is signed, timestamped, and part of a verifiable chain. Municipal clients need to trust that the assessment data has not been altered -- an assessment that says a pipe is safe must be defensible if that pipe later fails.

COMMAND-4's signing infrastructure uses OpenWallet Verifiable Credentials. The credential includes the agent's callsign, the assessment cycle, and the sector. Every commit is signed. Every assessment report is countersigned by COMMAND-1 (the analyst) and COMMAND-4 (the integrity verifier).

**Token budget:** 3,500 input / 900 output
**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Failure mode:** Signing verification overhead increases assessment delivery time. Mitigation: batch signing -- COMMAND-4 signs multiple findings at end of assessment session rather than signing each individually.

---

## Unit Dynamics

SOPs govern. Personal preferences do not. When Torres (COMMAND-2) proposed a new monitoring approach, she wrote it as an SOP amendment, Stavros (COMMAND-1) reviewed it, and it was adopted via the unit's change management process. This is slower than "just do it." It is also auditable.

### Redundancy Doctrine Applied to the Team

| Primary | Backup | Function |
|---------|--------|----------|
| COMMAND-1 | COMMAND-3 | Assessment analysis |
| COMMAND-2 | COMMAND-4 | System maintenance |
| COMMAND-3 | COMMAND-1 | External coordination |
| COMMAND-4 | COMMAND-2 | Signing and integrity |

Every function has a backup. The SOPs for each function are documented in `refs/infracom/sops/<function>`.

## Total Token Budget

| Callsign | Input | Output | Total |
|----------|-------|--------|-------|
| COMMAND-1 | 9,000 | 5,200 | 14,200 |
| COMMAND-2 | 3,800 | 800 | 4,600 |
| COMMAND-3 | 5,200 | 2,500 | 7,700 |
| COMMAND-4 | 3,500 | 900 | 4,400 |
| **Unit Total** | **21,500** | **9,400** | **30,900** |

---

*"Every function has a backup. Every backup has a procedure. Every procedure has been tested."*
