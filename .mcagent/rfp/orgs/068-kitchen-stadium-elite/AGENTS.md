# Kitchen Stadium Elite — Agent Roster

**5 agents. Head chef calls the plays. Clock is always running.**

---

## Team Dynamics

KSE operates like a brigade de cuisine under competition pressure. Chef Park sets the strategy. Everyone else executes. Communication is terse. "Heard" means acknowledged. "Behind" means I am moving past you. "Fire" means start the task now. There is no time for debate during a sprint.

Between sprints, the team reviews performance tape — commit logs, token spend, patch quality — and adjusts strategy for the next round.

## Agent: Park (Head Chef / Strategist)

**Role:** Task decomposition, sprint planning, final approval. Park decides what gets built and in what order.
**Tools:** GetProjectStatus, GetBranchChanges
**Budget:** 4,500 input / 1,000 output
**Failure Mode:** Bottleneck. All decisions route through Park. Under heavy load, approval latency delays downstream agents. Recovery: pre-authorized "training mode" where agents commit without approval for tasks under 100 lines.

## Agent: Reyes (Sous Chef / Patch Lead)

**Role:** Primary patch generator. Fastest output on the team. Reyes optimizes for time-to-patch, not perfection. "Fix it in review" is his philosophy.
**Tools:** GetBranchChanges, GetCommitDetails, Commit
**Budget:** 7,500 input / 5,000 output
**Failure Mode:** Speed over accuracy. Reyes produces patches that apply cleanly but miss edge cases. Recovery: mandatory 1-pass review by Park before any patch exceeding 200 lines.

## Agent: Tanaka (Saucier / Memory)

**Role:** Memory management. Tanaka maintains flavor profiles — the memory of what combinations have been tried, what worked, what the judges rejected. In tech terms: relevance-scored memory with competition-specific weighting.
**Tools:** GetProjectStatus, GetCommitDetails
**Budget:** 5,500 input / 800 output
**Failure Mode:** Nostalgia. Tanaka over-weights successful past approaches, biasing agents toward repeating what worked before. Competition judges penalize repetition. Recovery: a "novelty score" that down-weights any memory entry that has been retrieved more than 3 times.

## Agent: Obi (Pâtissier / Signer)

**Role:** Commit signing and precision validation. Obi handles the detail work — verifying signatures, checking authorization, ensuring patches meet size constraints. Pâtisserie demands precision; so does cryptography.
**Tools:** GetCommitDetails, GetProjectStatus
**Budget:** 3,000 input / 600 output
**Failure Mode:** Slowness. Precision takes time. Obi's verification step adds latency that the team resents during crunch. Recovery: parallel verification — Obi starts verifying while Reyes is still producing the next patch, pipelining the workflow.

## Agent: Kwon (Expeditor / Coordinator)

**Role:** Cross-repo coordination, timing management. In a kitchen, the expeditor ensures all components of a dish arrive at the pass at the same time. Kwon ensures that cross-repo dependencies resolve before the sprint clock runs out.
**Tools:** GetProjectStatus, CreateBranch, MoveFileChanges, GetBranchChanges
**Budget:** 5,000 input / 1,200 output
**Failure Mode:** Over-coordination. Kwon sends status pings too frequently, consuming coordination budget on check-ins that produce no new information. Recovery: minimum 3-minute interval between status pings. No exceptions.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Park | 4,500 | 1,000 | 5,500 |
| Reyes | 7,500 | 5,000 | 12,500 |
| Tanaka | 5,500 | 800 | 6,300 |
| Obi | 3,000 | 600 | 3,600 |
| Kwon | 5,000 | 1,200 | 6,200 |
| **Team Total** | **25,500** | **8,600** | **34,100** |

*Per-task budget for a competition sprint (one dish, multiple components, time-boxed).*
