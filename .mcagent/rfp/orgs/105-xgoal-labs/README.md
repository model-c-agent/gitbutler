# xGoal Labs

**"Expected goals changed scouting. We're changing expected goals."**

---

## The Company

xGoal Labs is a Series B sports analytics startup ($18M raised, 2026) that licenses an expected-goals (xG) model to professional football clubs. The model predicts the probability that any given shot will result in a goal, based on 47 features including shot angle, distance, body part, game state, and defensive pressure metrics derived from tracking data.

Six Premier League clubs license the model. Four clubs in the Bundesliga. Two in La Liga. Revenue: $3.2M ARR. The company is not yet profitable but is, in the words of CEO Anika Lindstrom, "close enough to smell it."

Founded in 2023 by three people who met at the MIT Sloan Sports Analytics Conference:

- **Anika Lindstrom** (CEO, 36): Former data scientist at Statsbomb. Left to build what she calls "the next generation of xG" -- a model that accounts for defensive positioning, which existing xG models largely ignore.
- **Diego Fernandez** (CTO, 32): Computer vision researcher. Built the tracking data ingestion pipeline that extracts player positions from broadcast video at 25fps. This is the company's core IP.
- **Priya Mehta** (Head of Product, 29): Former product manager at Opta. Designed the client-facing dashboard that clubs use to integrate xG analysis into their scouting and tactical workflows.

The company operates from a co-working space in East London with twelve employees.

## Why but-ai

xGoal Labs' development workflow is multi-repo: the vision pipeline, the xG model, the API, and the client dashboard are separate repositories with complex interdependencies. A model change ripples through the API schema and the dashboard visualization. Coordinating these changes manually consumes 30% of the engineering team's time.

GitButler's virtual branches helped with per-repo parallelism. The `but-ai` plugin would address cross-repo coordination and automate the repetitive model retraining pipeline (data fetching, feature engineering, hyperparameter search, evaluation).

## Philosophy

Models are products. A model that is 2% more accurate but impossible to explain to a football manager is worse than a model that is 2% less accurate but comes with a clear narrative. xGoal Labs sells understanding, not just predictions.

This extends to their AI agent architecture: an agent's output must be explainable. A patch that improves model accuracy is insufficient; the COMMIT.msg must explain *why* the accuracy improved and what the change means for the end user.

## Internal Tension

**The Explainability Tax.** Diego wants to use deep learning for defensive pressure modeling -- it is more accurate but less interpretable. Anika insists on interpretable models because "managers don't trust black boxes." Diego has a private branch with a neural network that outperforms the production model by 3.1 percentage points. It remains unmerged. The tension is productive but unresolved.

## Notable Achievement

In January 2026, xGoal Labs published a paper showing that their defensive-pressure-adjusted xG model outperformed every publicly available xG model on out-of-sample prediction by 7.2%. The paper was presented at the Opta Pro Forum and led to three new licensing conversations. Anika considers the paper their best marketing investment.

## Team (Plugin Dev Squad)

| Agent | Role | Title |
|-------|------|-------|
| Diego | Patch Generation | CTO |
| Anika | Review / Product QA | CEO |
| Priya | Forge Coordination | Head of Product |
| Jun | Memory / Model Registry | ML Engineer |
| Fatou | Security & Signing | DevOps |
| Sam | Budget & Provider | Backend Engineer |

Details in [AGENTS.md](AGENTS.md).

---

*"xG is a number. We make it a story."*
