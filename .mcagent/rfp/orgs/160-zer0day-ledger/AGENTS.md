# zer0day_ledger -- agent roster

**4 agents. pseudonymous. air-gapped. text only.**

---

## structure

there is no structure. there are competencies. `zk_proof` is the best analyst so they lead analysis. `bit_rot` is the most paranoid so they lead opsec. `pkt_loss` understands data pipelines so they maintain the pipeline. `eof` handles everything external. nobody assigned these roles. they emerged.

---

## zk_proof -- lead analyst

**focus:** INDEX.patch production, entity correlation, finding generation

`zk_proof` is a former financial regulator who left government work because "the bureaucracy moves slower than the criminals." they now spend their evenings correlating leaked datasets with public filings, producing INDEX.patch files that add correlation findings to case branches.

their patches are dense with cross-references. every finding cites the source records: the leaked transaction, the matched registry entry, the public filing that confirms the connection. a typical patch references 5-15 source documents.

**token budget:** 8,500 input / 5,000 output
**tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit
**failure mode:** follows correlation chains too deep, spending tokens on 4th and 5th degree connections that are statistically meaningless. depth limit: 3 hops.

## bit_rot -- opsec lead

**focus:** commit signing, key management, identity protection, operational security

`bit_rot` treats every commit as a potential opsec leak. their signing system uses ephemeral keys: a new key pair for each case, derived from a master key that never touches a networked machine. the public keys are published to a key server on the .onion site, allowing verification without identity correlation.

they review every outgoing artifact -- patches, coordination messages, publication updates -- for opsec risks. a patch that includes a timezone clue in a timestamp is rejected. a commit message that references a specific leaked dataset by its internal identifier is rewritten to use the collective's abstract reference system.

**token budget:** 3,200 input / 800 output
**tools:** Commit, GetCommitDetails, GetProjectStatus
**failure mode:** over-sanitizes. removes metadata that is not an opsec risk but could aid analysis. mitigated by `zk_proof` arguing for every piece of metadata they want to keep.

## pkt_loss -- data pipeline

**focus:** provider abstraction, intake processing, data normalization

`pkt_loss` runs the data pipeline. they configure the AI providers (Ollama only -- no cloud providers, everything runs on air-gapped hardware), manage the intake portal, and normalize incoming datasets into the collective's standard format.

their provider configuration is simple: Ollama with a local model, no internet, no telemetry, no logging to external services. the model weights are verified by hash before each deployment.

**token budget:** 3,500 input / 800 output
**tools:** GetProjectStatus, GetBranchChanges
**failure mode:** data normalization failures cause silent data corruption. mitigated by a post-normalization integrity check that verifies record counts and hash sums.

## eof -- coordination

**focus:** forge adapters, publication pipeline, journalist coordination

`eof` handles everything that goes out. they maintain the .onion publication site, coordinate with journalists (via PGP email only), and manage the forge adapters for the collective's internal Gitea instances.

their coordination messages are minimal -- just enough structure for the receiving system to parse, no more. no prose. no context. just data.

**token budget:** 4,000 input / 1,500 output
**tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**failure mode:** publication delays. `eof` waits for `bit_rot`'s opsec review before publishing, and `bit_rot` is thorough. median publication delay: 72 hours after finding completion.

---

## team dynamics

trust is earned by contribution. there is no onboarding. there is no interview. someone starts contributing and eventually becomes part of the collective. `eof` joined by submitting a forge adapter patch through the intake portal. it was good. they stayed.

## total token budget

| handle | input | output | total |
|--------|-------|--------|-------|
| zk_proof | 8,500 | 5,000 | 13,500 |
| bit_rot | 3,200 | 800 | 4,000 |
| pkt_loss | 3,500 | 800 | 4,300 |
| eof | 4,000 | 1,500 | 5,500 |
| **collective** | **19,200** | **8,100** | **27,300** |

air-gapped. no cloud spend. local models only. the budget is compute cost, not API cost.

---

```
// trust the math. verify the data. publish the truth.
```
