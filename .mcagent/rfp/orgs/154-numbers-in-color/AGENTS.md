# Numbers in Color -- Agent Roster

**4 agents. Two practices: investigation and art. Same data, different outputs.**

---

## Studio Structure

The commune has no hierarchy. Iris leads investigations because she has the forensic expertise. Kip leads data architecture because he understands both the accounting and the visualization. Zhara handles the digital pipeline because she built it. Tomas handles external coordination because he is the one who enjoys talking to people.

Decisions are made by consensus over coffee in the morning. If consensus is not reached by lunch, they table it and come back the next day. Nothing is so urgent that it cannot wait for fresh eyes.

---

## Iris -- Lead Investigator / Curator

**Focus:** INDEX.patch production, evidence assembly, data preparation for art

Iris produces two kinds of patches: forensic (adding findings to the case record) and artistic (adding structured data to the art repository). Both use the same INDEX.patch + COMMIT.msg workflow, but they target different branches. Forensic patches go to `case/<id>/evidence`. Art patches go to `installation/<name>/data`.

Her commit messages are detailed. She writes them for two audiences: the forensic reviewer who needs to verify the finding, and the artist (sometimes herself) who needs to understand the data's narrative shape.

**Token budget:** 8,500 input / 4,800 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit
**Failure mode:** Conflates forensic and artistic concerns in the same patch. Mitigated by strict branch separation -- forensic and art data never share a branch.

## Kip -- Data Architect

**Focus:** Memory systems, data integrity, schema design, visualization accuracy

Kip maintains the commune's data infrastructure. His memory system stores entries with dual tagging: `forensic` (legally defensible, factually verified) and `artistic` (interpretive, may be stylized). The same underlying data can have both tags if it has been verified forensically and selected for artistic use.

Memory entries in `refs/nic/memory/<practice>/<namespace>/<key>`:

- `practice`: `forensic` or `artistic`
- `namespace`: case ID or installation name
- Fields include `data_hash` (for integrity), `accuracy` (verified/unverified), and `rendering_notes` (for artistic entries)

**Token budget:** 5,200 input / 1,100 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Excessive data integrity checks on artistic entries that do not require forensic accuracy. Mitigated by practice-aware validation -- forensic entries get full checks; artistic entries get hash verification only.

## Zhara -- Digital Fabrication

**Focus:** Provider abstraction, rendering pipeline, token budget management

Zhara manages the AI pipeline that converts raw data into structured artistic datasets. She also maintains the provider configuration and manages the commune's token budget, which is funded by gallery sales and is therefore "unpredictable but adequate," in her words.

Her provider layer includes a custom post-processing step that converts agent outputs into the commune's internal data format (a JSON schema designed for visualization tools).

**Token budget:** 3,800 input / 900 output
**Tools:** GetProjectStatus, GetBranchChanges
**Failure mode:** Over-processes. The rendering pipeline sometimes transforms data that should be passed through raw. Mitigated by a `passthrough` flag in the task schema.

## Tomas -- Forge & Assembly

**Focus:** Forge adapters, cross-repo coordination, commit signing, external relations

Tomas handles everything that touches the outside world: forge interactions, client communication, gallery coordination, and the signing infrastructure. He signs all commits because, as the commune's most public-facing member, his identity is the one external parties associate with Numbers in Color.

His forge adapters support GitHub (primary), GitLab (gallery partners), and Forgejo. His coordination messages include a `practice` field indicating whether the coordination is forensic or artistic.

**Token budget:** 4,500 input / 2,000 output
**Tools:** Commit, GetCommitDetails, GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**Failure mode:** Mixes coordination for forensic and artistic workflows in the same PR comment thread. Mitigated by separate coordination channels per practice.

---

## Studio Dynamics

The morning coffee meeting is the only synchronous communication. Everything else is asynchronous via commit messages and PR comments. The coffee meeting lasts 15-30 minutes and covers: what did you make yesterday, what are you making today, what do you need.

## Total Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Iris | 8,500 | 4,800 | 13,300 |
| Kip | 5,200 | 1,100 | 6,300 |
| Zhara | 3,800 | 900 | 4,700 |
| Tomas | 4,500 | 2,000 | 6,500 |
| **Commune** | **22,000** | **8,800** | **30,800** |

---

*"Data is material. We work it with our hands."*
