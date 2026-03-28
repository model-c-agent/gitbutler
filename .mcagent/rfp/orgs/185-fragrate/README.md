# FragRate

**"Elo for everything."**

---

## The Pitch

FragRate is a startup. We say that without apology. We are four people in a co-working space in Austin, Texas, burning through a $1.8M seed round from Gradient Ventures and trying to build a universal competitive rating system before the money runs out. Our thesis: every competitive activity — from chess to Call of Duty to corporate sales leaderboards — can be rated on a unified scale if you normalize for domain-specific variance. Elo was the prototype. Glicko-2 was the improvement. We are building the generalization.

Founded in January 2024 by Kai Oduya (CEO, former Riot Games matchmaking engineer) and Dana Weiss (CTO, former quantitative analyst at Two Sigma who got bored of pricing derivatives and started pricing player skill). The team grew to four when we hired a full-stack engineer (Mo Siddiqui) and a developer relations lead (Jessie Tran) who spent three years building esports analytics communities.

We have paying customers. Seven esports organizations use our API to rate player performance across multiple game titles, normalizing so a "2100 FragRate" means the same thing whether you earned it in Valorant or Tekken. Two corporate clients use the same system for sales team performance ratings, which feels dystopian but pays the bills.

## Why We're Responding to This RFP

Our rating engine runs as a pipeline: ingest match data, compute rating updates, publish updated ratings. The pipeline is currently a monolithic Python service. We are rewriting it as a set of AI-assisted microservices because the rating math is getting too complex for hand-coded updates — we need agents that can analyze match data, detect rating anomalies (smurfs, boosters, win traders), and propose rating adjustments as reviewable changes.

Those adjustments need to be auditable. When you tell a professional player their rating dropped, they *will* dispute it. We need a commit history that shows exactly which agent computed the adjustment, what data it used, and what model produced the result. Git is our audit trail. GitButler's virtual branches let us run multiple rating model experiments simultaneously without contaminating production ratings.

`but-ai` gives us the agent framework we need: structured patches, signed commits, cross-repo coordination (our data ingestion, rating engine, and API are separate repos), and token budget management (we burn through OpenAI tokens faster than we burn through venture capital, and that is saying something).

## Philosophy

Ship fast, measure everything, fix what breaks. We do not have the luxury of philosophical purity. Our agents need to work, and they need to work within budget. We optimize for correctness-per-token: the maximum accuracy of rating adjustments for the minimum token expenditure.

We test in production. Not because we believe in it philosophically, but because we have four people and cannot maintain a staging environment. Our "staging environment" is a feature flag that limits new agent behavior to 5% of traffic. If the 5% cohort produces better ratings than the control, we roll out. If not, we roll back. The Git history is our safety net.

## The Smurf Detection Meltdown

In November 2025, our anomaly detection agent flagged 340 accounts as probable smurfs in a single rating period. The threshold was too aggressive — it was trained on data from a single game title (Valorant) and applied to all titles without recalibration. In Tekken, where player performance variance is naturally higher due to character matchup volatility, the model flagged legitimate players who happened to have streaky results.

We issued 340 rating holds. 312 were disputed. 290 disputes were upheld. We spent two weeks manually reviewing cases. The post-mortem identified the root cause: the agent's memory did not include game-specific variance baselines. We now store per-title variance profiles in the memory branch, and the anomaly threshold adapts per title.

It was our worst week. It was also the week we became serious about agent memory.

## Achievement

**Series A Term Sheet**: In Q1 2026, we received a Series A term sheet at a $22M valuation. The lead investor cited our agent-auditable rating pipeline as the key differentiator. "Every other rating system is a black box. FragRate is the only one where I can `git log` the math."

## Team

| Agent | Name | Role |
|-------|------|------|
| CEO / Product | Kai Oduya | Product direction, rating model design |
| CTO / Architect | Dana Weiss | System architecture, token budget optimization |
| Full-Stack Engineer | Mo Siddiqui | Plugin implementation, forge integration |
| DevRel / Memory | Jessie Tran | Developer experience, memory architecture |

Details in [AGENTS.md](AGENTS.md).

---

*"If you can't git-blame the rating, the rating is wrong."*
