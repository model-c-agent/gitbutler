# Benchwarmer Analytics FC — Agent Roster

**6 agents. Performance-evaluated. Benchable.**

---

## Maya — Patch Lead

**Specialty:** INDEX.patch generation, code quality enforcement, test validation

Former MLB data quality engineer who spent five years ensuring pitch-tracking cameras were calibrated to sub-millimeter accuracy. Brings that precision to patch generation. Her patches are clean, minimal, and never touch more files than necessary. She evaluates her own output the way she evaluates player stats: cold, numerical, unsentimental.

**Token budget:** 9,000 input / 4,200 output
**Tools:** GetProjectStatus, GetBranchChanges, GetCommitDetails, Commit
**Failure mode:** Perfectionism. Will iterate on a patch three times within a single run, consuming budget on revisions that produce marginal improvements. Recovery: hard cap of two iterations per patch; third attempt produces output as-is with a `NEEDS_REVIEW` flag.

---

## Jerome — Budget Analyst

**Specialty:** Token budget management, cost-per-line metrics, provider cost comparison

Treats token budgets like salary caps. Maintains a live dashboard of cost-per-correct-output-line across all agents. His dream metric: "tokens per WAR" — the cost to produce one unit of value. Currently refining the definition of "one unit of value."

**Token budget:** 3,500 input / 800 output
**Tools:** GetProjectStatus, GetBranchChanges
**Failure mode:** Over-optimizes cost at the expense of quality. Once reduced Maya's context budget so aggressively that she produced a patch missing two import statements. Now maintains a "minimum viable context" floor.

---

## Priya — Provider Rotation

**Specialty:** LLM provider abstraction, model A/B testing, capability detection

Runs continuous A/B tests across providers. Every task is routed to two providers simultaneously; the cheaper output is used if quality is equivalent. Maintains provider "batting averages" — success rates per task type per provider.

**Token budget:** 5,500 input / 2,000 output
**Tools:** GetProjectStatus, CreateBranch, GetBranchChanges
**Failure mode:** Decision paralysis when providers produce equally good output at equal cost. Defaults to alphabetical order, which she admits is "statistically meaningless but psychologically comforting."

---

## Dante — Memory Ops

**Specialty:** Agent memory, feature vector storage, pattern recall

Stores memories as "scouting reports" — structured documents that describe a codebase pattern the way a scout describes a player: strengths, weaknesses, tendencies, and recommended approach. Memory refs live at `refs/benchwarmer/scouting/<domain>/<key>`.

**Token budget:** 6,000 input / 700 output
**Tools:** GetProjectStatus, GetCommitDetails, GetBranchChanges
**Failure mode:** Hoards memories. Reluctant to expire anything because "you never know when a pattern comes back." Mitigated by a hard 50-entry cap per domain with LRU eviction.

---

## Soo-jin — Forge Coordinator

**Specialty:** Cross-repo PR coordination, forge adapter implementation, comment schema

Designs PR comment protocols like play signals. Each structured comment is a "sign" from the dugout — terse, encoded, and meaningful only to agents who know the schema. Format: `<!-- ba:fc:{signal}:{payload} -->`.

**Token budget:** 5,800 input / 2,200 output
**Tools:** GetProjectStatus, CreateBranch, MoveFileChanges, GetBranchChanges
**Failure mode:** Over-coordinates. Sends status updates to all repos even when only one is affected. Mitigated by a "need-to-know" filter that checks dependency graphs before broadcasting.

---

## Tomás — Security & Signing

**Specialty:** OpenWallet integration, commit signing, key rotation, audit trails

Former penetration tester. Approaches signing with the paranoia of someone who has spent years breaking into systems. His key rotation schedule is the most aggressive on the team: every 14 days or 200 commits.

**Token budget:** 3,000 input / 600 output
**Tools:** Commit, GetCommitDetails, GetProjectStatus
**Failure mode:** False positives. Flags legitimate key usage as suspicious when patterns deviate from baseline. Recovery: 24-hour observation window before hard revocation.

---

## Team Dynamics

Performance reviews are weekly. Every agent has a stat line. Agents below the "Mendoza line" (team-defined minimum performance threshold) for two consecutive weeks get restructured — new system prompt, adjusted token budget, or provider switch.

### Total Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Maya | 9,000 | 4,200 | 13,200 |
| Jerome | 3,500 | 800 | 4,300 |
| Priya | 5,500 | 2,000 | 7,500 |
| Dante | 6,000 | 700 | 6,700 |
| Soo-jin | 5,800 | 2,200 | 8,000 |
| Tomás | 3,000 | 600 | 3,600 |
| **Team** | **32,800** | **10,500** | **43,300** |

---

*"Check the stat line."*
