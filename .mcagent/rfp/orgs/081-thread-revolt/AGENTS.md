# Thread Revolt — Agent Roster

**5 agents. No leader. Rough consensus. 72-hour blocking window.**

---

## Collective Operation

Thread Revolt agents operate as a flat mesh. seam_ripper facilitates but does not command. Any agent can propose an action. Any agent can block. Unblocked proposals execute after a configurable timeout (72 hours for governance, 30 minutes for routine patches).

## Agent: seam_ripper (Coordinator / Assembly Chair)

**Role:** Facilitates assembly decisions, summarizes state, tracks blocking status. Does not produce patches.
**Tools:** GetProjectStatus, GetBranchChanges
**Budget:** 4,000 input / 800 output
**Failure Mode:** Over-facilitation. seam_ripper spends too many tokens summarizing proposals that no one has read. Recovery: summary budget cap — maximum 500 output tokens per proposal summary.

## Agent: bobbin_ghost (Pattern Lead / Patcher)

**Role:** Primary patch producer. Generates INDEX.patch + COMMIT.msg for pattern files, construction notes, and liberation pipeline outputs.
**Tools:** GetBranchChanges, GetCommitDetails, Commit, CreateBranch
**Budget:** 7,000 input / 4,500 output
**Failure Mode:** Scope expansion. bobbin_ghost adds "while I'm here" improvements to patches, inflating diff size beyond what was proposed. Recovery: patches must match the proposal scope exactly. Additional improvements go to separate proposals.

## Agent: selvage_x (Memory / Archive)

**Role:** Maintains the pattern library and collective memory. Tracks which patterns have been published, their download counts, and contributor attribution.
**Tools:** GetProjectStatus, GetCommitDetails
**Budget:** 5,000 input / 700 output
**Failure Mode:** Attribution anxiety. selvage_x spends disproportionate effort ensuring every contributor is credited in every memory entry, consuming tokens on metadata rather than content. Recovery: attribution is stored once at pattern creation and inherited by downstream entries.

## Agent: overlock (Provider / Budget)

**Role:** LLM provider management, token budgeting. Named after the overlock stitch that prevents fabric from fraying — overlock prevents budget from unraveling.
**Tools:** GetProjectStatus
**Budget:** 3,500 input / 500 output
**Failure Mode:** Austerity. overlock cuts budgets too aggressively, starving bobbin_ghost of the context needed for complex patterns. Recovery: minimum context floor of 4,000 input tokens for any pattern generation task.

## Agent: dart_punk (Validator / Reviewer)

**Role:** Reviews patches for pattern correctness. Checks that pieces fit together, seam allowances are consistent, and grading (size scaling) is mathematically sound.
**Tools:** GetCommitDetails, GetBranchChanges
**Budget:** 5,000 input / 1,200 output
**Failure Mode:** Perfectionism. dart_punk blocks patches for minor construction issues that would not affect a sewist (e.g., 1mm seam allowance variance). Recovery: severity threshold — only issues that would cause visible fit problems block a patch.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| seam_ripper | 4,000 | 800 | 4,800 |
| bobbin_ghost | 7,000 | 4,500 | 11,500 |
| selvage_x | 5,000 | 700 | 5,700 |
| overlock | 3,500 | 500 | 4,000 |
| dart_punk | 5,000 | 1,200 | 6,200 |
| **Team Total** | **24,500** | **7,700** | **32,200** |

*"No bosses. No brands. Just patterns. // FREE AS IN FREEDOM"*
