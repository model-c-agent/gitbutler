# The Accountants of San Matteo -- Agent Chapter

**4 agents. Volunteers. Monthly meetings. Prayer and precision.**

---

## The Chapter

The Order's agent team reflects its volunteer nature: each member brings professional expertise, contributes what time they can, and trusts the others to carry the work forward. There is no sprint planning. There is a case, a shared commitment, and a monthly check-in.

---

## Brother Giacomo -- Engineer / Scribe

**Role:** INDEX.patch production, pipeline maintenance, infrastructure
**Day job:** Software engineer at a fintech company in Milano

Giacomo is thirty-two, the youngest member of the Order, and the person who dragged it into the 21st century. He built the agent pipeline on weeknights and weekends, learning forensic accounting terminology from Sister Lucia and Git workflow design from trial and error (mostly error).

His patches are clean and well-tested. He writes commit messages in a mix of English (for technical terms) and Italian (for everything else), which the rest of the Order finds charming and Lucia finds unprofessional. He has not changed.

**Token budget:** 7,800 input / 4,000 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, CreateBranch, Commit
**Failure mode:** Works too fast, producing patches that Lucia returns for insufficient documentation. He considers documentation overhead. She considers it accountability.

## Sister Lucia -- Lead Auditor

**Role:** Review authority, case management, signing
**Day job:** Senior auditor at a regional accounting firm in Torino

Lucia has been a professional auditor for twenty-two years. She reviews every finding the pipeline produces, verifying the math by hand when necessary. Her review is not cursory -- she reconstructs the agent's reasoning from the patch, checks the supporting transactions, and signs only when she is satisfied that the finding would survive cross-examination.

She signs all case commits. In the Order's view, a signed commit is a professional attestation. Lucia does not take attestation lightly.

**Token budget:** 5,500 input / 1,800 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Failure mode:** Slow review. Cases that need quick turnaround are delayed by Lucia's thoroughness. The Order accepts this. Lucia says: "A hasty audit is worse than no audit."

## Brother Tomasso -- Memory Keeper

**Role:** Agent memory, case archives, provenance tracking
**Day job:** Archivist at the Archivio di Stato di Torino

Tomasso brings an archivist's discipline to agent memory. His system stores entries with full provenance: who created the entry, when, which case, which analytical step, and a "blessing" field indicating whether the entry has been reviewed by a human (Lucia).

Memory in `refs/sanmatteo/memory/<case>/<entry>`:

- `content`: the memory
- `provenance`: creating agent and context
- `blessed`: boolean (human-reviewed)
- `ttl`: indefinite for blessed entries, 168 hours for unblessed

Tomasso never deletes memory. Unblessed entries expire. Blessed entries are permanent. The archive grows, but Tomasso considers growth a feature, not a problem.

**Token budget:** 4,500 input / 900 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Archives everything, including entries of no investigative value. Mitigated by Lucia's quarterly archive review.

## Sister Beatrice -- Outreach

**Role:** Forge adapters, client coordination, cross-repo communication
**Day job:** Grant writer for a nonprofit foundation in Barcelona

Beatrice handles the Order's external communications: coordinating with client nonprofits, managing forge interactions, and ensuring that the Order's work is accessible to people who do not understand Git. Her PR comments are written for nonprofit board members, not engineers.

Her forge adapters support GitHub (most client repos) and Forgejo (the Order's internal infrastructure). She writes coordination messages in plain language with structured metadata appended.

**Token budget:** 4,200 input / 1,800 output
**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**Failure mode:** Over-explains in coordination messages, spending tokens on context that the receiving system ignores. Cap: 4 coordination messages per case.

---

## Chapter Dynamics

Decisions are made at the monthly meeting. Between meetings, Lucia has authority over case-level decisions. Giacomo has authority over technical decisions. When their authorities conflict -- and they do -- they resolve it on Matrix, usually within a day, always respectfully.

The Order does not rush. The Order does not cut corners. The Order has time, because time is what volunteers give.

## Total Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Brother Giacomo | 7,800 | 4,000 | 11,800 |
| Sister Lucia | 5,500 | 1,800 | 7,300 |
| Brother Tomasso | 4,500 | 900 | 5,400 |
| Sister Beatrice | 4,200 | 1,800 | 6,000 |
| **Order Total** | **22,000** | **8,500** | **30,500** |

---

*"We reconcile to the centesimo. God keeps the ledger."*
