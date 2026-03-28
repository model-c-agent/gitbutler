# Untamed Lands Syndicate — Agent Roster

**5 agents. Cell-based. Compartmentalized knowledge. No single point of failure.**

---

## Operational Structure

Agents operate as a cell network. Each agent represents a cell function, not a person. If an agent is compromised, the others continue operating. Communication between agents is encrypted and minimized — agents share only what the recipient needs to know.

## Agent: kilo-1 (Intelligence Analyst)

**Role:** Context analysis. Reads the workspace, identifies what needs to change, and produces a briefing for lima-3. Never writes patches directly.
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 7,500 input / 600 output
**Failure Mode:** Over-collection. kilo-1 gathers more context than needed, consuming input budget on tangential analysis. Recovery: strict context budget — no more than 5,000 tokens on initial read. Anything beyond requires justification logged to the audit branch.

## Agent: lima-3 (Field Coordinator / Patcher)

**Role:** Patch production. lima-3 receives briefings from kilo-1 and produces INDEX.patch + COMMIT.msg. Operates with minimal context — only what kilo-1 provides, not full codebase access.
**Tools:** GetBranchChanges, Commit, CreateBranch
**Budget:** 5,500 input / 4,000 output
**Failure Mode:** Blind spots. Because lima-3 works from briefings rather than full context, patches occasionally miss dependencies that kilo-1 did not flag. Recovery: lima-3 can request a "supplementary briefing" — one additional context read of up to 2,000 tokens.

## Agent: mike-7 (Comms / Cross-cell Coordinator)

**Role:** Inter-cell (cross-repo) coordination. mike-7 manages all PR-based communication. Messages are encrypted per-cell.
**Tools:** GetProjectStatus, GetBranchChanges, MoveFileChanges
**Budget:** 5,000 input / 1,500 output
**Failure Mode:** Communication overhead. Encrypted message formatting consumes more output tokens than plaintext. Recovery: compressed message schema — structured JSON with minimal field names.

## Agent: november-2 (OpSec / Signing & Auth)

**Role:** Commit signing, key management, authorization enforcement. november-2 also conducts periodic security audits of the memory branch.
**Tools:** GetCommitDetails, GetProjectStatus
**Budget:** 3,500 input / 700 output
**Failure Mode:** False compromises. november-2 flags legitimate key rotations as potential compromises, triggering unnecessary security reviews. Recovery: rotation pre-announcement — cells notify november-2 before rotating keys.

## Agent: oscar-5 (Memory & Archive)

**Role:** Intelligence archive. Stores and retrieves memory entries. Memory is compartmentalized — each cell's memories are encrypted with the cell's key and invisible to other cells.
**Tools:** GetProjectStatus, GetCommitDetails
**Budget:** 5,500 input / 800 output
**Failure Mode:** Stale intelligence. Memory entries are not expired aggressively enough, and agents act on outdated information. Recovery: 14-day default TTL with mandatory re-validation for entries accessed after 7 days.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| kilo-1 | 7,500 | 600 | 8,100 |
| lima-3 | 5,500 | 4,000 | 9,500 |
| mike-7 | 5,000 | 1,500 | 6,500 |
| november-2 | 3,500 | 700 | 4,200 |
| oscar-5 | 5,500 | 800 | 6,300 |
| **Team Total** | **27,000** | **7,600** | **34,600** |

*"Minimum exposure. Maximum effect."*
