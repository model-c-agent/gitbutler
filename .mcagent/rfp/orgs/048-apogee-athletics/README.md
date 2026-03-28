# Apogee Athletics

**"Dodge debris. Score points. Save satellites."**

---

## Origin

Apogee Athletics started as a joke at ESA's 2021 Clean Space conference and became a company six months later. Kenji Arakawa, a flight dynamics engineer, was presenting a paper on automated collision avoidance maneuvers when he used a football analogy: "The satellite is the striker. The debris field is the defense. The maneuver planner is the coach calling plays from the sideline." The analogy landed better than his actual paper. Someone in the audience — Brigitte Holm, a former professional handball player turned aerospace project manager — asked him afterward: "What if the gamification was the product?"

The idea: turn collision avoidance into a competitive sport. Satellite operators already compete informally — whose constellation has the fewest close approaches, whose avoidance maneuvers are most fuel-efficient. Apogee Athletics formalized the competition. They built a scoring system that awards points for successful debris avoidance, deducts points for close calls, and maintains a global leaderboard. Operators opt in, submit their conjunction data, and receive a performance score calibrated against the entire fleet.

It sounds absurd. It works. Fourteen operators representing 2,800 satellites now participate. Collision avoidance response times dropped 31% across the league in the first year because nobody wants to be last on the leaderboard. Brigitte calls it "the shame economy." Kenji calls it "gamified safety culture."

## Philosophy

Competition improves performance. Not competition as aggression — competition as mutual accountability. When everyone can see your score, you play harder. Apogee Athletics believes the same principle applies to AI agents: agents that are measured, scored, and ranked against each other produce better work than agents operating in isolation.

Their agents carry performance metrics: patch acceptance rate, token efficiency, review turnaround time. These metrics are visible to the human operator. Underperforming agents get "benched" — relegated to simpler tasks until their metrics improve.

## Internal Tension

Kenji wants the scoring to be purely technical: fuel efficiency, response time, maneuver accuracy. Brigitte wants to add a "sportsmanship" component — operators who share tracking data with competitors should earn bonus points. Kenji thinks sportsmanship is unmeasurable and will be gamed. Brigitte thinks pure technical scoring incentivizes hoarding. The leaderboard currently includes both metrics, weighted 70/30 technical/sportsmanship. Neither founder likes the split.

## Achievement

Apogee Athletics' "Orbital Cup 2025" — a month-long competition where operators competed for the cleanest collision avoidance record — resulted in zero unplanned close approaches across all participating constellations. For context, the monthly average before the competition was 4.2 unplanned close approaches per month across the same operators. Brigitte attributes the improvement to competitive motivation. Kenji attributes it to the fact that they finally standardized conjunction data formats as a prerequisite for scoring.

## Team

| Name | Role | Background |
|------|------|------------|
| Kenji Arakawa | CTO / Scoring Engine | Flight dynamics, ex-ESA/ESOC |
| Brigitte Holm | CEO / Product & Culture | Pro handball (Denmark national team), aerospace PM |
| Dex Okonkwo | Backend Engineer | Real-time systems, ex-Formula 1 telemetry |
| Anya Petrov | ML / Ranking | Sports analytics, ex-Opta |
| Finn Larsen | Frontend / UX | Leaderboard design, ex-gaming industry |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes a performance score in the format `[APR: 0.87]` — the Agent Performance Rating, a rolling metric from 0 to 1 measuring the agent's recent patch acceptance rate, token efficiency, and review speed. A score above 0.8 means the agent is "in form." Below 0.5 means the agent needs coaching.

---

*"Play hard. Dodge clean. Top the board."*
