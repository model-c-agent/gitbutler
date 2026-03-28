# Statball Commune

**"No one owns the data. No one owns the analysis. The insights belong to the game."**

---

## Formation

The Statball Commune emerged in 2021 from a Reddit thread titled "Why do three companies own all sports data?" The thread attracted 2,000 comments and a dozen data scientists who were tired of building analytics tools on top of proprietary data feeds controlled by Opta, Stats Perform, and ESPN.

Within six months, the dozen became thirty. They built an open-source sports data collection platform called `openstat` that crowdsources play-by-play data from volunteer observers at live games. The data is imperfect -- volunteer-reported event coordinates have a margin of error -- but it is free, open, and growing. By 2026, `openstat` covers football (soccer), basketball, and cricket across 14 leagues in 9 countries.

The Commune has no leader, no board, and no revenue model. Infrastructure costs are covered by member donations and a Patreon with 340 supporters. Decisions are made by rough consensus on a Discourse forum. If someone wants to build something, they build it. If others find it useful, they adopt it. If not, it remains a personal fork. This is messy, slow, and works better than anyone expected.

## Why but-ai

The Commune's codebase is sprawling: data collection scripts, statistical models, visualization tools, and league-specific adapters scattered across 23 repositories. Coordination is the bottleneck. Contributors work on overlapping problems without knowing it, and merging parallel efforts after the fact is painful.

GitButler's virtual branches helped with per-repo coordination. The `but-ai` plugin could help with cross-repo coordination and the mechanical parts of model development (data cleaning, feature engineering, report generation) that consume time the Commune's volunteers would rather spend on analysis.

## Philosophy

Data monopolies distort analysis. When three companies control all sports data, they control the questions that can be asked. The Commune exists to break that monopoly by producing an alternative dataset that anyone can query, analyze, and publish from.

They apply the same principle to AI agents: agent outputs must be reproducible from open data. If an agent's patch depends on a proprietary data source, the patch is rejected. Every analysis must be reproducible by anyone with access to `openstat`.

## Internal Tension

**The Quality Problem.** Volunteer-collected data has errors. `kmeans_kev` (machine learning lead) wants to build error-correction models that infer missing data. `pitchfork` (data integrity lead) insists on publishing only what was directly observed, errors and all. "Inferred data is fabricated data," pitchfork says. The debate cycles monthly. Both approaches coexist in different branches.

## Notable Achievement

In 2025, a Commune member published an analysis of referee bias in the English Championship using `openstat` data. The analysis found a statistically significant home-team advantage in foul calls (p<0.01) that correlated with crowd noise levels estimated from stadium capacity data. The paper was covered by The Athletic and prompted the EFL to commission an independent review. The review used proprietary Opta data and reached the same conclusion, which the Commune considers the ultimate validation.

## Team Overview

| Handle | Role | Timezone |
|--------|------|----------|
| kmeans_kev | Lead / Patch Generation | UTC-5 |
| pitchfork | Review / Data Integrity | UTC+0 |
| offside_trap | Memory Architecture | UTC+1 |
| corner_flag | Forge Coordination | UTC+8 |
| clean_sheet | Security & Signing | UTC-8 |
| xG_malone | Budget & Provider | UTC+5:30 |

Details in [AGENTS.md](AGENTS.md).

---

*"Open data. Open analysis. Open game."*
