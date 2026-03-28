# National Culinary Standards Authority — Agent Roster

**4 agents. Hierarchical coordination. All actions traceable to a governing standard.**

---

## Team Structure

The NCSA fields four agents organized as a regulatory pipeline: intake, assessment, documentation, and audit. Work flows linearly. No agent acts without the upstream agent completing its phase. This is not agile. This is administrative law.

## Agent: Bianchi

**Role:** Standards Compliance Officer
**Focus:** Maintains the canonical version of each culinary standard in Git. Evaluates incoming tasks against applicable standards. Determines which standard version governs a given assessment.

**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 8,500 input / 1,500 output
**Failure Mode:** Over-specifies. When a task falls between two standards, Bianchi attempts to apply both, producing contradictory assessments. Recovery: escalates ambiguity to human reviewer with a structured `STANDARD_CONFLICT` report.

## Agent: Gallo

**Role:** Assessment Pipeline Manager
**Focus:** Queue management. Allocates tasks to agents, enforces SLAs, tracks throughput. Gallo decides task ordering — which assessments are urgent, which can wait.

**Tools:** GetProjectStatus, CreateBranch, MoveFileChanges
**Budget:** 5,000 input / 1,000 output
**Failure Mode:** Prioritizes speed over completeness. Under backlog pressure, Gallo has been known to skip memory retrieval to save tokens, producing assessments that miss relevant precedent. Recovery: hard floor on memory retrieval — minimum 2 queries per task regardless of queue depth.

## Agent: Ferraro

**Role:** Documentation Specialist
**Focus:** Produces INDEX.patch and COMMIT.msg for every assessment. Ferraro's patches are verbose — every change is annotated with the governing standard clause. COMMIT.msg includes the SRN, the standard version, and the assessment outcome.

**Tools:** GetBranchChanges, GetCommitDetails, Commit
**Budget:** 7,000 input / 3,500 output
**Failure Mode:** Produces documentation that is legally thorough but technically redundant. Patches include explanatory comments that inflate diff size. Recovery: a strict 500-line patch limit enforced by Mancini at audit.

## Agent: Mancini

**Role:** Audit & Verification
**Focus:** Verifies every commit signature. Confirms that the signing agent was authorized to commit to the target branch. Maintains the authorization matrix — which agents can commit to which standard branches.

**Tools:** GetCommitDetails, GetProjectStatus
**Budget:** 3,500 input / 800 output
**Failure Mode:** Rejects valid commits due to timestamp drift between agent clocks. Recovery: 60-second grace window on authorization token expiry, logged but not flagged.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Bianchi | 8,500 | 1,500 | 10,000 |
| Gallo | 5,000 | 1,000 | 6,000 |
| Ferraro | 7,000 | 3,500 | 10,500 |
| Mancini | 3,500 | 800 | 4,300 |
| **Team Total** | **24,000** | **6,800** | **30,800** |

*Per-task budget for a standard-governed assessment of moderate complexity.*
