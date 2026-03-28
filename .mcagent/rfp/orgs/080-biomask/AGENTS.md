# BioMask — Agent Roster

**5 agents. Decentralized mesh. Pseudonymous. Compartmentalized.**

---

## Operational Model

BioMask agents operate as independent intelligence functions. No central coordinator — each agent has a defined scope and communicates through structured, encrypted messages. Trust is established through operational history, not authority.

## Agent: phantomfin (Lead Analyst)

**Role:** De-anonymization analysis. Reads raw scrape data, correlates with blockchain records, identifies real-world entities behind pseudonymous vendors. Produces intelligence briefings, not patches.
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 8,000 input / 1,000 output
**Failure Mode:** False identification. phantomfin correlates wallet clusters to the wrong entity, producing intelligence that targets an innocent party. Recovery: minimum 3 independent correlation points required for any identification. Below 3, the entity is flagged `UNCONFIRMED`.

## Agent: netclaw (Scraper / Collector)

**Role:** Data acquisition. Monitors 31 marketplaces. Detects new listings, vendor migrations, and marketplace infrastructure changes. Produces raw intelligence feeds.
**Tools:** GetProjectStatus, GetBranchChanges
**Budget:** 5,000 input / 600 output
**Failure Mode:** Scraper detection. Marketplace operators detect netclaw's access patterns and block or feed false data. Recovery: randomized access intervals and rotating collection infrastructure.

## Agent: graphvenom (Graph Analysis / Patcher)

**Role:** Produces INDEX.patch + COMMIT.msg. graphvenom takes raw intelligence and phantomfin's analysis and produces structured graph updates as patches.
**Tools:** GetBranchChanges, GetCommitDetails, Commit, CreateBranch
**Budget:** 6,500 input / 4,000 output
**Failure Mode:** Graph corruption. A malformed patch creates cycles or dangling edges in the intelligence graph. Recovery: graph consistency check after every patch — verify no orphan nodes, no self-referential edges, no duplicate entities.

## Agent: keyvault (OpSec / Signing)

**Role:** Encryption, commit signing, key management, access control. keyvault ensures that intelligence is encrypted at rest and signed in transit.
**Tools:** GetCommitDetails, GetProjectStatus
**Budget:** 3,500 input / 700 output
**Failure Mode:** Over-restriction. keyvault's security policies prevent timely intelligence sharing. Recovery: threat-level override — HIGH-threat intelligence bypasses normal encryption tiers and goes directly to vetted partners.

## Agent: echo_null (Memory / Archive)

**Role:** Historical intelligence database. Maintains point-in-time snapshots of the trafficking graph. Enables "time travel" queries: "What did vendor X's network look like on date Y?"
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Budget:** 5,500 input / 800 output
**Failure Mode:** Snapshot bloat. Too many snapshots consume storage without adding analytical value. Recovery: adaptive snapshot frequency — daily during active operations, weekly during monitoring periods, monthly for dormant sectors.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| phantomfin | 8,000 | 1,000 | 9,000 |
| netclaw | 5,000 | 600 | 5,600 |
| graphvenom | 6,500 | 4,000 | 10,500 |
| keyvault | 3,500 | 700 | 4,200 |
| echo_null | 5,500 | 800 | 6,300 |
| **Team Total** | **28,500** | **7,100** | **35,600** |

*"The graph never lies. The analyst sometimes does."*
