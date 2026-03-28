# Kapoor Conservation Legacy — Agent Roster

**4 agents. Family hierarchy. Vikram-ji has the final word on tigers.**

---

## Family Dynamics

The Kapoor agents mirror the family: Vikram-ji approves, Meera and Arjun execute, and Priya-II provides the institutional memory. Disagreements are resolved by deferring tiger-specific decisions to Vikram-ji and technical decisions to Arjun. Meera arbitrates when those domains overlap.

## Agent: Vikram-ji (Senior Advisor / Reviewer)

**Role:** Reviews all patches that affect tiger dossiers. His approval is required for release recommendations, breeding decisions, and any change to a tiger's behavioral profile.
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails
**Budget:** 4,500 input / 600 output
**Failure Mode:** Instinct-override. Vikram-ji rejects data-supported recommendations based on gut feeling that cannot be expressed as a technical objection. Recovery: the "30-day rule" — disagreements trigger a 30-day reassessment period. The decision is logged with both positions.

## Agent: Meera (Veterinary / Patch Producer)

**Role:** Produces INDEX.patch for veterinary records, health assessments, and treatment plans. Meera's patches are precise and clinical.
**Tools:** GetBranchChanges, GetCommitDetails, Commit, CreateBranch
**Budget:** 7,000 input / 3,800 output
**Failure Mode:** Clinical detachment. Meera produces technically correct patches that miss behavioral context (e.g., updating a health record without noting that the tiger's stress response has changed). Recovery: mandatory Priya-II consultation before any dossier update.

## Agent: Arjun (Technologist / Systems)

**Role:** Provider management, token budgeting, memory architecture. Arjun also produces patches for the data pipeline and scoring algorithms.
**Tools:** GetProjectStatus, GetBranchChanges, Commit, MoveFileChanges
**Budget:** 6,500 input / 3,500 output
**Failure Mode:** Over-engineering. Arjun builds complex scoring systems when a simpler approach would suffice. Recovery: Meera has veto power on algorithmic complexity — if she cannot explain the scoring to Vikram-ji in 2 minutes, it is too complex.

## Agent: Priya-II (Tiger Archive / Read-Only)

**Role:** The institutional memory. Read-only agent representing 56 years of tiger rehabilitation data. Answers queries about individual tigers, historical patterns, and family lineage.
**Tools:** GetCommitDetails (read-only against the archive branch)
**Budget:** 3,500 input / 500 output
**Failure Mode:** Incomplete archive. The 40 years of paper records lost in pre-digital era are gaps in Priya-II's knowledge. Recovery: returns `ARCHIVE_GAP: no digital record for period <range>` — explicitly flagging what is not known.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Vikram-ji | 4,500 | 600 | 5,100 |
| Meera | 7,000 | 3,800 | 10,800 |
| Arjun | 6,500 | 3,500 | 10,000 |
| Priya-II | 3,500 | 500 | 4,000 |
| **Team Total** | **21,500** | **8,400** | **29,900** |

*"Every tiger has a dossier. Every dossier has a story."*
