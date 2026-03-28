# Ranger League — Agent Roster

**5 agents. Championship structure. Fair scoring above all.**

---

## Team Dynamics

The League's agents operate like a sports officiating crew: Oduya sets the rules, Kimathi scores, Nguyen validates, Torres tracks history, and Abdi coordinates between teams. During championships, agents run in real-time. During training, they run in batch mode. The mode switch is config-driven.

## Agent: Oduya (Commissioner / Strategist)

**Role:** Sets competition parameters, defines scoring criteria, resolves disputes. Does not score — impartiality is absolute.
**Tools:** GetProjectStatus, GetBranchChanges
**Budget:** 4,000 input / 800 output
**Failure Mode:** Rule complexity. Oduya's scoring criteria become too complex for agents to apply consistently. Recovery: each championship has a frozen rule set committed to a tagged branch. No mid-championship rule changes.

## Agent: Kimathi (Scorer / Patch Lead)

**Role:** Primary scorer. Reads team data, applies scoring algorithm, produces INDEX.patch with scored results. The highest-throughput agent.
**Tools:** GetBranchChanges, GetCommitDetails, Commit, CreateBranch
**Budget:** 8,000 input / 4,500 output
**Failure Mode:** Scoring bias. Subtle bugs in the scoring algorithm that systematically advantage teams with certain data formats. Recovery: every scoring patch includes a `fairness_check` — the score is recomputed with randomized team labels to detect format-dependent variance.

## Agent: Torres (Statistician / Memory)

**Role:** Maintains historical performance data. Tracks trends across championships. Produces season summaries and ranking tables.
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Budget:** 6,000 input / 1,000 output
**Failure Mode:** Stale baselines. Torres' historical comparisons use outdated baselines, making current performance look artificially good or bad. Recovery: baselines are tagged with the championship year and only compared within the same biome category.

## Agent: Nguyen (Field Systems / Validator)

**Role:** Validates species identifications and GPS coordinates submitted by teams. Cross-references against reference databases. Rejects invalid data.
**Tools:** GetCommitDetails, GetBranchChanges
**Budget:** 5,500 input / 1,200 output
**Failure Mode:** False rejections. Valid species sightings rejected because the reference database is incomplete (newly described species, subspecies not yet catalogued). Recovery: "provisional acceptance" — data that fails validation is flagged but not rejected, pending human review.

## Agent: Abdi (Coordinator / Comms)

**Role:** Cross-team communication, schedule coordination, result announcements. Manages the PR-based communication between team repos and the central scoring repo.
**Tools:** GetProjectStatus, GetBranchChanges, MoveFileChanges
**Budget:** 4,500 input / 1,200 output
**Failure Mode:** Announcement timing. Abdi publishes partial results before all teams' data is scored, creating confusion. Recovery: results are only published after Kimathi has scored all teams in the current round. No partial announcements.

---

## Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Oduya | 4,000 | 800 | 4,800 |
| Kimathi | 8,000 | 4,500 | 12,500 |
| Torres | 6,000 | 1,000 | 7,000 |
| Nguyen | 5,500 | 1,200 | 6,700 |
| Abdi | 4,500 | 1,200 | 5,700 |
| **Team Total** | **28,000** | **8,700** | **36,700** |

*"The scoreboard does not lie. The algorithm must not either."*
