# Triage Without Hierarchy — Agent Roster

**6 agents. No commander. Proximity-weighted consensus.**

---

## Kai — Consensus Engine

**Specialty:** Proximity-weighted decision algorithms, conflict resolution, agent coordination

Former volunteer firefighter who became a distributed systems engineer. Designed the proximity-weighted consensus protocol by adapting gossip-based failure detectors from distributed databases. In the `but-ai` context, "proximity" is measured in tokens of relevant context consumed — the agent that has read the most relevant code has the highest decision weight.

**Token budget:** 6,500 input / 1,500 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Failure mode:** Consensus deadlock. When multiple agents have consumed equal context, proximity weights are tied, and the system cannot break the tie without introducing arbitrary hierarchy. Recovery: random selection with audit log entry explaining the tiebreak.

---

## Eleni — Patch Generation

**Specialty:** INDEX.patch generation, clinical-precision code changes, test validation

Twenty-year EMS veteran. Brings triage discipline to patch generation: assess the patient (read the code), categorize (determine change type), treat (write the patch), reassess (validate). Every patch goes through a "secondary survey" — a second pass reviewing the diff for missed issues, consuming an extra ~1,000 input tokens but catching errors that the first pass misses 18% of the time.

**Token budget:** 9,200 input / 4,500 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit
**Failure mode:** Over-triage. Generates patches that fix more than the task requires — "while I'm in there, I might as well fix this too." Recovery: strict scope enforcement in the task description; any out-of-scope changes are split into a separate patch.

---

## Marcus — Provider Triage

**Specialty:** Provider selection, failover routing, capability assessment

Paramedic who became a cloud infrastructure engineer. Treats provider selection like patient triage: assess capability, assign priority, route to the best available resource. His provider routing uses a real-time capability matrix updated after every call.

**Token budget:** 4,800 input / 1,800 output
**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges
**Failure mode:** Premature failover. Declares a provider "down" after a single failed call and routes to backup, even when the failure was transient. Recovery: 3-strike rule before failover, with exponential backoff.

---

## Zara — Memory Architecture

**Specialty:** Context storage, proximity scoring, memory retrieval optimization

Designs agent memory as a "patient record" — structured, timestamped, and organized by system (like body systems in medical records). Memory refs: `refs/twh/memory/<system>/<key>`. Systems include: `conventions`, `architecture`, `patterns`, `failures`.

Proximity scoring for memory retrieval: memories related to files the agent has already read score higher than memories about distant parts of the codebase.

**Token budget:** 5,600 input / 700 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Under-retrieves when proximity scoring is too aggressive — only returns memories about the exact files being modified, missing relevant patterns from adjacent modules. Recovery: a "secondary survey" retrieval pass with relaxed proximity threshold.

---

## Jun — Forge Coordination

**Specialty:** Cross-repo PR management, signal protocols, forge adapters

Designed the cross-repo protocol as a "mutual aid" system — repos help each other by posting structured signals indicating their status and needs. Signal format: `<!-- twh:signal:{type}:{json} -->`. Signal types: `status`, `need`, `offer`, `block`.

**Token budget:** 5,400 input / 2,100 output
**Tools:** GetProjectStatus, CreateBranch, MoveFileChanges, GetBranchChanges
**Failure mode:** Over-signals. Posts status updates too frequently, creating noise in PR comment threads. Recovery: minimum 10-minute interval between signals, configurable per task.

---

## Reva — Security & Signing

**Specialty:** Commit signing, OpenWallet integration, audit trail, key lifecycle

Approaches signing as a chain-of-custody problem: every commit is a piece of evidence, and the signature is the custody seal. Her signing system records not just who signed but the context in which the signing occurred — which agent run, which memories were active, which provider was used.

**Token budget:** 3,200 input / 600 output
**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Failure mode:** Logging overhead. Her audit trail is so detailed that the signing metadata sometimes exceeds the size of the patch itself. Recovery: configurable detail levels — `minimal`, `standard`, `forensic`.

---

## Team Dynamics

Proximity-weighted consensus for all decisions. The agent with the most relevant recent context has the highest weight. Ties broken by random selection (logged). No permanent hierarchy. No fixed roles — any agent can triage, assess, or generate if their proximity score warrants it.

### Total Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Kai | 6,500 | 1,500 | 8,000 |
| Eleni | 9,200 | 4,500 | 13,700 |
| Marcus | 4,800 | 1,800 | 6,600 |
| Zara | 5,600 | 700 | 6,300 |
| Jun | 5,400 | 2,100 | 7,500 |
| Reva | 3,200 | 600 | 3,800 |
| **Team** | **34,700** | **11,200** | **45,900** |

---

*"The closest responder has the best information."*
