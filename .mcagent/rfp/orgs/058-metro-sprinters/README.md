# Metro Sprinters

**"Schedule adherence is a sport. We are the league."**

---

## Origin

Metro Sprinters started as a pub joke in Manchester in 2022. Five transit enthusiasts — all employed by different UK bus and tram operators — had been arguing for years about whose transit system was the most reliable. The arguments were always anecdotal: "Our tram is never late." "Your tram was 4 minutes late last Tuesday." "That was signal failure, it does not count."

Danny Reeves, a data analyst at Transport for Greater Manchester, decided to settle the argument with data. He scraped real-time arrival data from the APIs of every transit operator in the pub group and built a leaderboard. Buses, trams, metro lines — all ranked by schedule adherence, measured to the second. The leaderboard went on a shared Google Sheet. The pub group checked it weekly.

Within a month, the leaderboard had leaked. Transit operators across the UK started checking their rankings. Two operators contacted Danny privately to ask how they could improve their scores. Three operators complained that the methodology was unfair. One operator's communications department sent a legal threat (which was withdrawn after the operator's engineering team quietly asked Danny for the raw data).

Metro Sprinters incorporated in 2023. The leaderboard is now public, covering 40 operators in the UK, Netherlands, and Germany. The scoring system has been peer-reviewed and published. And the five pub friends — Danny, Nkechi Okafor (systems engineer, ex-Arriva), Henrik Lindqvist (tram operations, ex-GVB Amsterdam), Priya Sharma (data scientist, ex-Network Rail), and Callum MacLeod (frontend developer, transit hobbyist) — now run a company that turns schedule adherence into a competitive sport.

## Philosophy

What gets measured gets improved. What gets ranked gets improved faster. Metro Sprinters believes that transparency and competition are the most powerful forces for operational improvement. Their software makes performance visible, comparable, and — crucially — fun. A transit operator checking their schedule adherence rank on a Monday morning is a transit operator who cares about being on time.

They apply the same philosophy to AI agents: measure everything, rank everything, make the rankings visible. An agent that knows its performance is being compared to other agents (or to its own previous performance) will be configured and tuned more carefully than an agent operating in a metrics vacuum.

## Internal Tension

Danny wants to keep the scoring objective — pure schedule adherence, measured in seconds. Nkechi argues that objective scoring ignores context: a bus that is 2 minutes late because of roadworks is different from a bus that is 2 minutes late because of poor scheduling. Danny says context is a slippery slope to excuses. Nkechi says context is the difference between useful data and misleading data. They argue about this weekly. The scoring currently has a "context note" field that operators can fill in but that does not affect the ranking. Neither founder considers this a resolution.

## Achievement

In 2025, Metro Sprinters ran the "UK Schedule Cup" — a three-month competition across 22 bus operators. The operator with the best improvement in schedule adherence won a trophy (an actual trophy, which Danny had custom-made). The winning operator, a small company in Devon, improved their on-time performance from 71% to 89% in three months. Their secret: they used the Metro Sprinters dashboard to identify their three worst-performing routes and reallocated one bus from an over-resourced route. The reallocation cost nothing. The improvement was dramatic. Danny says this is the clearest example of measurement driving improvement he has ever seen.

## Team

| Name | Role | Background |
|------|------|------------|
| Danny Reeves | CEO / Data Lead | Transit data analyst, ex-TfGM |
| Nkechi Okafor | CTO / Systems | Systems engineering, ex-Arriva |
| Henrik Lindqvist | Operations / Scoring | Tram operations, ex-GVB Amsterdam |
| Priya Sharma | Data Science | Prediction models, ex-Network Rail |
| Callum MacLeod | Frontend / UX | Dashboard design, transit enthusiast |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes the agent's current "league position" — its rank among all configured agents based on recent performance. Example: `feat: add headway calculation [League Position: 2/5, Adherence: 0.91]`.

---

*"Check the leaderboard. Then ship the code."*
