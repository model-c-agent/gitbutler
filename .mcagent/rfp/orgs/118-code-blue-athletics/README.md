# Code Blue Athletics

**"It's not a competition. Okay, it's absolutely a competition."**

---

## Origin

Code Blue Athletics runs the Medical Simulation Olympics — an annual competition where emergency medicine teams from hospitals worldwide compete in simulated clinical scenarios, scored on speed, accuracy, teamwork, and communication. Think athletic competition, but the sport is saving simulated lives.

Founded in 2019 by Dr. Nadia Voss, an EM attending physician and former Division I volleyball player who missed competitive sports after residency and decided to make emergency medicine into one. The first year had twelve teams. The 2025 competition had ninety-six teams from fourteen countries, a live broadcast on a medical education streaming platform, and corporate sponsors from three med-tech companies.

The organization employs eight full-time staff — event logistics, scoring systems, scenario design, and broadcast engineering. The engineering team (four people) built the scoring platform: a real-time system that tracks team actions during simulations, scores them against rubrics, maintains leaderboards, and produces instant replay breakdowns of critical moments.

The scoring platform needed AI agents when the competition scaled beyond what human judges could score in real time. At ninety-six teams running simultaneous simulations across twelve rooms, manual scoring was impossible. The engineering team built agents that observed simulation telemetry (mannequin vitals, action timestamps, communication audio transcripts) and produced real-time scoring against the rubric.

These agents needed to be fast (scores within 30 seconds of scenario completion), accurate (matching human judge agreement rates above 90%), and auditable (every score explainable with evidence). The engineering team built the system on Git — every scoring run was a commit with the rubric, the telemetry, the score, and the justification.

## Philosophy

**Competition drives excellence.** Teams that compete improve faster than teams that practice alone. Code Blue Athletics applies this to AI agents: agent configurations compete against each other on standard benchmarks, and the best performers are promoted to production. They call this "the playoff system."

## The Tension

Dr. Voss and head engineer Amir Sadeqi disagree about score transparency. Voss believes all scores should be instantly visible — competitors deserve real-time feedback. Amir argues that displaying intermediate scores before the agent's confidence exceeds a threshold creates misleading information: "Showing a 60% confidence score as if it were final is worse than showing no score at all." The compromise: scores are displayed when confidence exceeds 85%, with a "pending" indicator for lower-confidence assessments.

## Notable Achievement

At the 2025 Olympics, the scoring agents achieved 94% agreement with expert human judges — exceeding the inter-rater reliability between human judges themselves (91%). The agents scored faster (median 12 seconds vs. human median 3 minutes) and produced more detailed justifications. Three participating hospitals licensed the scoring system for their internal training programs.

## Team

Four engineers plus Dr. Voss as competition director.

| Agent | Role | Focus |
|-------|------|-------|
| Dr. Nadia Voss | Director | Scoring rubric design, competition architecture |
| Amir Sadeqi | Systems Lead | Patch generation, scoring engine, infrastructure |
| Kenji Watanabe | Provider & Perf | Provider abstraction, latency optimization |
| Lisa Strand | Memory & Replay | Agent memory, instant replay, pattern archival |
| Tomás Restrepo | Security & Fair Play | Commit signing, anti-tampering, audit trails |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The team operates on a competition calendar. Three months before each Olympics, they enter "training camp" — intense development sprints building and testing new scoring scenarios. During competition week, they run 24/7 operations with rotating shifts. After competition, they hold a "post-season review" analyzing every scoring discrepancy and updating the system.

They use sports metaphors for everything. Deployments are "game days." Bug fixes are "halftime adjustments." Major refactors are "off-season conditioning."

---

*"Play hard. Score fair. Ship fast."*
— Code Blue Athletics team motto
