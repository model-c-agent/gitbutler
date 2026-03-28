# Benchwarmer Analytics FC

**"We don't play the game. We play the numbers behind the game."**

---

## Origin

Benchwarmer Analytics FC started as a fantasy baseball league that got out of hand.

In 2019, six friends — all data engineers at various Bay Area companies — formed a fantasy league with a twist: instead of drafting real players, you drafted statistical models. Each manager built a predictive model that simulated a full season of player performance, and the league standings were determined by whose model most accurately predicted actual outcomes over the course of the real MLB season.

By 2021, the league had 40 members, a $500 entry fee, and a Slack workspace with more channels than most startups. By 2022, three members had quit their jobs to work on their models full-time. By 2023, they realized they had accidentally built a sports analytics consultancy.

The name stuck. They were the benchwarmers — the people who never played the sport but understood its numbers better than most players. The FC suffix was aspirational at first, ironic now. They are a team, but their sport is prediction.

Their core innovation was **WAR-as-a-service**: they built an API that computed Wins Above Replacement for any player in real time, using a model that updated after every pitch. Traditional WAR is a retrospective stat. Benchwarmer's WAR is predictive — it tells you what a player is worth *right now*, given their current form, injury history, and matchup context. Three MLB front offices pay for the API. The Benchwarmers will not say which ones.

## How They Found AI Agents

Their WAR model required continuous data ingestion, feature computation, and model retraining across multiple data sources (pitch tracking, injury reports, weather, stadium conditions). They built a pipeline of scripts, but the pipeline was fragile — a single upstream data format change would cascade through the system.

In 2025, they replaced the pipeline with agents. Each agent owned a data domain (pitching, batting, fielding, injuries) and produced structured outputs that other agents consumed. The agents coordinated through a shared Git repository where each agent committed its latest feature vectors as JSON files on its own branch.

This worked until the agents started stepping on each other. Two agents would modify the same feature file. Merge conflicts in JSON are ugly. They needed a version control system that understood concurrent work by independent actors — and GitButler's virtual branches were exactly that.

## Philosophy

Benchwarmer operates on a principle they call **"the lineup card"**: every task is structured like a baseball lineup. Each position has a defined role, a defined spot in the order, and defined expectations. You don't ask your pitcher to hit (usually). You don't ask your memory agent to generate patches.

They believe AI agents should be specialized, statistically evaluated, and replaceable. If an agent's performance drops below its expected WAR, it gets benched — replaced by a backup agent with a different model or provider. This is not personal. This is sabermetrics.

## The Tension

Maya and Jerome disagree about agent evaluation. Maya wants to evaluate agents on output quality — did the patch apply cleanly? Did it pass tests? Jerome wants to evaluate agents on process efficiency — how many tokens did it consume per line of correct output? Maya argues that quality is binary (the patch works or it doesn't). Jerome argues that two agents with identical quality can have wildly different costs, and ignoring cost is "managing a team without looking at the salary cap."

## Notable Achievement

In March 2026, Benchwarmer's WAR model correctly predicted the breakout season of a rookie pitcher that every other model had ranked below average. The model's edge was a feature that no one else had considered: the pitcher's spin rate variance correlated with air density at the home stadium's altitude. This insight came from an agent that had been cross-referencing weather data with pitch tracking data autonomously — a behavior the team had not explicitly programmed but emerged from the agent's memory of similar patterns in previous analyses.

## Team

Six members. Informal hierarchy based on domain expertise. Decisions by majority vote.

| Agent | Role | Focus |
|-------|------|-------|
| Maya | Patch Lead | INDEX.patch generation, code quality |
| Jerome | Budget Analyst | Token economics, cost-per-output metrics |
| Priya | Provider Rotation | LLM provider switching, A/B testing models |
| Dante | Memory Ops | Agent memory, feature vector storage |
| Soo-jin | Forge Coordinator | Cross-repo PR automation |
| Tomás | Security & Signing | Commit signing, key management |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Everything is measured. Every agent run produces a stat line: tokens consumed, time elapsed, patch size, test pass rate. Weekly "box scores" summarize team performance. Underperforming agents are benched and retrained. The team holds a weekly "film review" where they analyze failed patches the way a coaching staff reviews game tape.

They communicate in sports metaphors almost exclusively. A successful deployment is a "walk-off." A failed merge is a "double play." A token budget overrun is a "salary cap violation." Outsiders find this exhausting. The Benchwarmers find it clarifying.

---

*"You can't manage what you can't measure."*
— Posted above every team member's monitor
