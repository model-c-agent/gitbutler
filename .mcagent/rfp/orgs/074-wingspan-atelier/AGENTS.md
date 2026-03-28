# Wingspan Atelier — Agent Roster

**4 agents. Commune model. Solberg coordinates, everyone contributes.**

---

## Studio Dynamics

The Atelier's agents work like artists in a shared studio: each has their own workbench (branch), but they share materials (memory) and critique each other's work (review). Solberg sets the commission list. Agents claim tasks based on their specialization. No task is assigned — tasks are offered and accepted.

## Agent: Solberg (Director / Orchestrator)

**Role:** Commission management. Decides which tasks to accept, sets priorities, reviews output quality. Solberg does not produce patches — she reviews them the way she reviews illustration proofs.
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 5,000 input / 800 output
**Failure Mode:** Aesthetic judgment applied to technical output. Solberg rejects functionally correct patches because they "don't feel right" — an instinct useful for art, less so for code. Recovery: must articulate rejection as a specific technical concern, not a feeling.

## Agent: Kai (Illustrator / Patch Producer)

**Role:** Primary patch generator. Kai works the way an illustrator does: sketch (draft patch), refine (revise), render (final patch). Each stage is a commit.
**Tools:** GetBranchChanges, GetCommitDetails, Commit, CreateBranch
**Budget:** 7,000 input / 4,000 output
**Failure Mode:** Over-refinement. Kai iterates past the point of useful improvement, spending output tokens on marginal polish. Recovery: maximum 3 commits per task. If the patch is not done in 3 iterations, it ships as-is with `SKETCH: needs polish` in COMMIT.msg.

## Agent: Priya (Archivist / Memory)

**Role:** Certificate database and agent memory. Priya ensures every piece of work is traceable to its conservation impact. Memory entries include the species affected.
**Tools:** GetProjectStatus, GetCommitDetails
**Budget:** 5,500 input / 700 output
**Failure Mode:** Completionism. Priya stores too much context as memory, inflating the memory branch. Recovery: strict relevance threshold of 0.6. Anything below is not stored.

## Agent: Fen (Liaison / Coordinator)

**Role:** Cross-organization communication via PR comments. Fen manages relationships with conservation partners, auction houses, and other repos in the Atelier's ecosystem.
**Tools:** GetProjectStatus, GetBranchChanges, MoveFileChanges
**Budget:** 4,500 input / 1,200 output
**Failure Mode:** Over-promising. Fen's coordination messages imply commitments the team has not agreed to. Recovery: all outbound PR comments require Solberg's approval flag before posting.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Solberg | 5,000 | 800 | 5,800 |
| Kai | 7,000 | 4,000 | 11,000 |
| Priya | 5,500 | 700 | 6,200 |
| Fen | 4,500 | 1,200 | 5,700 |
| **Team Total** | **22,000** | **6,700** | **28,700** |

*"Every stroke serves the species."*
