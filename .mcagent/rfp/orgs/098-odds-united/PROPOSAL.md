# Odds United -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We score predictions for a living. Our `but-ai` plugin applies prediction scoring methodology to agent performance: every agent output is a forecast, and every forecast should be scored. Agents that learn from their scores improve. Agents that don't are replaced.

---

## Requirement 1: PATH-Based Plugin Architecture

Static binary. `$PATH` discovery. `cargo binstall`. The binary includes a `--score` subcommand for evaluating agent performance across historical tasks -- not just "did the patch apply?" but "was the agent's confidence calibrated?"

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait, four providers. Provider selection based on calibration data: we track each provider's historical accuracy-vs-confidence and route tasks to the best-calibrated provider for the estimated difficulty level.

**Calibration tracking:** Every provider call records the agent's self-assessed confidence and the actual outcome (patch accepted/rejected). Over time, this builds a calibration curve per provider per task type. Providers that are systematically overconfident are penalized in routing.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. Every COMMIT.msg includes a `X-Confidence:` trailer (0.0-1.0) and a `X-Brier-Eligible:` flag indicating whether the patch's outcome should be scored.

**Scoring loop:**
1. Agent produces patch with confidence score
2. Patch is reviewed (human or automated)
3. Outcome (accepted/rejected/revised) is recorded
4. Brier score is calculated: `(confidence - outcome)^2`
5. Score feeds back into the agent's calibration profile

Lower Brier scores = better calibration. The system tracks rolling 30-day Brier scores per agent and alerts when calibration degrades.

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait. Three repos: scoring engine, question bank, leaderboard. Coordination via structured PR comments.

**Tournament-style merge:** Cross-repo releases follow a "season" schedule. Changes accumulate on development branches during the season. At season end, all repos merge simultaneously in a coordinated release. No mid-season scoring engine changes that could affect active tournaments.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/odds/seasons/<season>/`. Entries are structured prediction records.

**Memory schema:**
```json
{
  "key": "cat-freq-q4-2025-prediction",
  "value": "Hurricane frequency forecast: 0.73 probability of 3+ Cat-3 events",
  "outcome": "actual: 4 Cat-3 events",
  "brier_score": 0.073,
  "season": "S8",
  "tags": ["catastrophe", "hurricane", "q4-2025"]
}
```

**Scored memory:** Memory entries that include outcome data and Brier scores are weighted in retrieval by their score quality. Well-calibrated memories (low Brier score) are preferred over poorly calibrated ones. The system learns from its best predictions.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Deepa manages tournament integrity. Scoring results are signed and timestamped to prevent retroactive manipulation. Signed commits on the scoring engine carry the same weight as sealed tournament results.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Freya | 4,200 | 1,000 | 5,200 |
| Samir | 8,200 | 3,800 | 12,000 |
| Nkechi | 5,600 | 800 | 6,400 |
| Lars | 5,500 | 2,200 | 7,700 |
| Deepa | 3,500 | 900 | 4,400 |
| Marco | 2,800 | 600 | 3,400 |
| **Total** | **29,800** | **9,300** | **39,100** |

---

## Unique Insight: Brier-Scored Agent Calibration

Most AI agent systems track binary success (patch accepted or rejected). Ours tracks calibrated confidence. An agent that says "90% confident" and is accepted 90% of the time is perfectly calibrated. An agent that says "90% confident" and is accepted 60% of the time is dangerously overconfident.

By scoring agent confidence with the Brier scoring rule (the same proper scoring rule used in our prediction tournaments), we create incentive-compatible self-assessment: the agent minimizes its Brier score by reporting its true confidence, not by inflating it.

In league testing, Brier-scored agents converged to well-calibrated confidence within 50 tasks. Unscored agents showed persistent overconfidence (average reported confidence 0.85, actual acceptance rate 0.62). The scored agents learned humility. The unscored ones did not.

---

*"Report your true confidence. The scoring rule will find you out."*
