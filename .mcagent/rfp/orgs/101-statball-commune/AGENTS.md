# Statball Commune -- Agent Roster

**Six contributors. No hierarchy. Rough consensus and running code.**

---

## kmeans_kev -- Lead / Patch Generation

Machine learning engineer by day. Builds predictive models for the Commune by night. Generates patches rapidly, often in long Saturday coding sessions fueled by energy drinks. His code is functional but under-documented. Pitchfork's reviews always include the same note: "Add docstrings."

**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit
**Token budget:** 8,000 input / 3,800 output

## pitchfork -- Review / Data Integrity

Former data journalist. Reviews every patch with a focus on data provenance: where did this number come from? Can it be independently verified? Rejects patches that reference data sources not in `openstat`. Review comments are detailed and always include a citation request.

**Tools:** GetBranchChanges, GetCommitDetails
**Token budget:** 4,400 input / 1,200 output

## offside_trap -- Memory Architecture

Designed memory for `openstat`'s analytical context. Entries in `refs/statball/mem/<handle>/` include a `data_source` field that must reference an `openstat` table. No proprietary data references in memory. Retrieval is filtered by league and season to keep context relevant to the current analysis.

**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Token budget:** 5,000 input / 600 output

## corner_flag -- Forge Coordination

Manages cross-repo coordination across the Commune's 23 repos. Maintains a "dependency map" showing which repos depend on which. Cross-repo PRs follow the dependency order: upstream repos merge first.

**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges, MoveFileChanges
**Token budget:** 6,000 input / 2,500 output

## clean_sheet -- Security & Signing

Manages signing keys for the Commune's published analyses. Signed commits prove that an analysis was produced by a Commune member, not fabricated by someone with an agenda. Treats signing as peer credentialing: "Our signature means this analysis followed our methodology."

**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Token budget:** 3,200 input / 800 output

## xG_malone -- Budget & Provider

Manages the Commune's infrastructure budget ($200/month from donations). Allocates token budget across projects. Strongly prefers local models to keep costs near zero. Will approve cloud provider usage only for time-sensitive analyses (e.g., mid-tournament reports).

**Tools:** GetProjectStatus, GetBranchChanges
**Token budget:** 2,600 input / 500 output

---

## Dynamics

Anarchic but productive. No one assigns work. People pick tasks from the Discourse forum's open issues list. kmeans_kev and pitchfork are the most active and disagree most often. corner_flag keeps the repos from diverging. clean_sheet and xG_malone handle infrastructure quietly. offside_trap surfaces relevant historical analyses when new tasks start.

## Total Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| kmeans_kev | 8,000 | 3,800 | 11,800 |
| pitchfork | 4,400 | 1,200 | 5,600 |
| offside_trap | 5,000 | 600 | 5,600 |
| corner_flag | 6,000 | 2,500 | 8,500 |
| clean_sheet | 3,200 | 800 | 4,000 |
| xG_malone | 2,600 | 500 | 3,100 |
| **Total** | **29,200** | **9,400** | **38,600** |

---

*"The data is ours. All of ours. Nobody's monopoly."*
