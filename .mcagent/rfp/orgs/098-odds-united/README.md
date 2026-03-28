# Odds United

**"We don't play the game. We predict it. Then we play each other."**

---

## The League

Odds United is a competitive prediction league for insurance actuaries. Founded in 2021 as a Slack channel joke ("Who can predict next quarter's loss ratio closest?"), it grew into a formal league with seasons, rankings, and a trophy shaped like a bell curve that the annual champion displays on their desk for twelve months.

Forty-two actuaries across fourteen countries participate. The league runs quarterly prediction tournaments: each participant submits probabilistic forecasts for a set of insurance-relevant outcomes (quarterly loss ratios, catastrophe event frequencies, mortality rate movements). Forecasts are scored using the Brier score -- a proper scoring rule that rewards calibrated probabilities, not lucky guesses.

The league is governed by a five-person Competition Committee. The Committee sets the questions, manages the scoring system, and adjudicates disputes. Meetings are held over video call and always run long because actuaries love arguing about methodology.

## Why Software

The scoring system was a Google Sheet until Season 3, when 42 participants and 20 questions per quarter made it unmanageable. The Committee built a proper scoring engine in Python, then realized they needed version control when two committee members edited the scoring logic simultaneously and introduced contradictory changes.

Git, then GitButler (for virtual branches -- the Committee members work on scoring system improvements in parallel), then `but-ai` (for automating question generation, score calculation, and the quarterly report that summarizes league standings).

## Philosophy

Prediction is a sport. Like any sport, it rewards practice, discipline, and honest self-assessment. The league's culture emphasizes calibration over accuracy: it is better to say "I'm 60% confident" and be right 60% of the time than to say "I'm 95% confident" and be right 70% of the time. Overconfidence is the cardinal sin.

## Internal Tension

**The Complexity Debate.** Committee member Samir wants to add ensemble-model questions (participants submit models, not point predictions, and models are scored on out-of-sample performance). Committee chair Freya argues this changes the nature of the competition from individual judgment to engineering skill. The proposal has been tabled three times.

## Notable Achievement

Season 8 (Q4 2025) was the first season where the league's median forecast outperformed the commercial insurance industry consensus on all five catastrophe frequency predictions. The league published a post-season analysis showing where the industry consensus was systematically biased. Two reinsurers quietly adjusted their models.

## Team Overview

| Agent | Role | League Role |
|-------|------|-------------|
| Freya | Lead / Review | Committee Chair |
| Samir | Patch Generation | Committee / Scoring Engine |
| Nkechi | Memory / Statistics | Committee / Data |
| Lars | Forge Coordination | Committee / Communications |
| Deepa | Security & Signing | Tournament Integrity |
| Marco | Provider & Budget | Infrastructure |

Details in [AGENTS.md](AGENTS.md).

---

*"Calibrate or go home. The Brier score doesn't care about your feelings."*
