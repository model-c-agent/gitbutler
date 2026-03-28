# SimCity League

**"Zone it, build it, ship it. Clock's ticking."**

---

## Origin Story

The SimCity League was founded in 2021 by three urban planning graduate students at MIT who were supposed to be writing their dissertations but kept getting into arguments about optimal transit-oriented development configurations. The arguments escalated from whiteboard sketches to a bet: "I can build a better city than you in 90 minutes." They opened SimCity, set a timer, and the first unofficial match was played in a cramped apartment in Cambridge while their advisor's emails went unanswered.

What made the bet interesting was the scoring. They did not judge on aesthetics or raw population. They developed a rubric — the Metropolitan Performance Index (MPI) — that scored cities on four axes: density (people per square kilometer), livability (park access, noise levels, commute times), transit coverage (percentage of residents within 400m of a transit stop), and fiscal sustainability (operating budget surplus as a percentage of revenue). A city could have a million residents and still lose if they were all stuck in traffic with no parks and the city was running a deficit.

The rubric turned out to be more interesting than the game. Other urban planning students heard about it and wanted to compete. By the end of 2021, there were twelve teams. By 2022, there was a Discord server with 800 members, a seasonal league structure, and a championship tournament held in person at MIT's Department of Urban Studies.

The three founders — Maya "Gridlock" Torres, Jin-soo "The Densifier" Park, and Alexei "Greenline" Petrov — remain the core team, though they have long since graduated and now work in real urban planning. Torres works for the City of Boston's transportation department. Park is a density consultant in Seoul. Petrov runs a transit advocacy nonprofit in Berlin. They compete on weekends and holidays, and they are ruthless about it.

## How We Got Here

The SimCity League's interest in AI agents came from a practical problem: judging. As the league grew, scoring cities became a bottleneck. Each city had to be evaluated across four axes, with dozens of metrics per axis. Human judges were slow, inconsistent, and — in one memorable incident — bribed with a $15 pizza to inflate a transit score.

In 2024, the league built its first automated judge: an agent that could parse a city's exported data and produce an MPI score. The judge worked, but it was brittle — it broke every time the game's export format changed, and it could not handle edge cases like cities that had technically zero traffic because they had no roads (a strategy Petrov attempted once, building an all-transit city that scored perfectly on transit coverage but had a livability score of 3 because ambulances could not reach hospitals).

Building a better judge led to building better tools, which led to building agents that could assist in city design, which led to the realization that competitive city-building was actually a coordination problem: multiple agents working on different zones of a city simultaneously, each optimizing for their specialty (density, transit, parks, budget), with the constraint that their decisions interact. You cannot zone high-density residential next to an industrial plant. You cannot run a bus line through a park. The agent that handles transit has to coordinate with the agent that handles zoning.

When the GitButler `but-ai` RFP landed, the league saw it as the same problem they had been solving for three years: multiple agents, concurrent work, shared constraints, and a timer counting down.

## Philosophy

### On AI Agents

Agents are players on a team. Each one has a position, a specialty, and a playbook. The team wins or loses together. There is no MVP award in the SimCity League — only team scores.

We think about agent coordination the way we think about city systems: everything is connected, and optimizing one thing in isolation makes other things worse. An agent that optimizes for its own metric at the expense of the team's score is a bad teammate. We design agents that are aware of the score, not just their own contribution to it.

### On Version Control

Version control is zoning. Every change goes into a zone. Zones have rules about what can go next to what. A commit to the transit module should not silently affect the budget module, just like a bus depot should not appear in a residential zone without a variance hearing.

We think about branches the way we think about city districts: each one has a character, a purpose, and boundaries. Branches merge the way districts connect — through well-defined interfaces (roads, transit lines, in code: APIs and data contracts).

### On Competition

We are a competitive organization. We believe competition makes systems better. We time ourselves. We score ourselves. We track our metrics across seasons. When we build something, we build it fast and we build it to win — not in a sloppy, corner-cutting way, but in the way a championship team executes a practiced play. Speed comes from preparation, not from skipping steps.

## Internal Dynamics

### The Draft

Every season, the league holds a draft where teams select agents for their roster. The three founders have a permanent roster for the championship team, but for league play, agents rotate. This means every agent has to be good enough to play for any team, and every team has to be able to work with any agent. The result: agents have extremely well-defined interfaces and interchangeable components, because they were designed to be drafted.

### The Rivalries

Torres and Park have a standing rivalry over density vs. transit. Torres believes that transit coverage is the single most important metric — "You can have all the density you want, but if people can't get anywhere, you've built a prison." Park counters that density enables transit — "You don't run a subway to a suburb. You run it where the people are." Petrov, whose specialty is green space and sustainability, often plays the tiebreaker. His position: "If your dense, well-connected city has no parks, your residents will riot, and then you have neither density nor transit."

### The Timer

Everything in the SimCity League happens under time pressure. Tournament matches are 90 minutes. Scrimmages are 45. Even design reviews have a clock. This culture pervades the team's approach to the RFP: every design decision is evaluated not just on quality but on speed of execution. A beautiful architecture that takes 200,000 tokens to execute is worse than a good architecture that takes 80,000.

## Notable Achievements

- **Season 7 Championship** (2024): The founders' team scored an MPI of 94.2, the highest in league history. The winning city had 1.2 million residents, 97% transit coverage, a budget surplus of 12%, and a park within 200m of every residential unit. Build time: 87 minutes.
- **The Automated Judge** (2024): Built the league's first AI-powered scoring system, reducing judgment time from 3 hours to 4 minutes per city.
- **The All-Transit Experiment** (2023): Petrov's famous zero-road city that proved transit-only cities are technically possible but practically miserable. The resulting paper was rejected by three journals but became a legendary meme in the urban planning community.
- **The 3-Agent City** (2025): First successful fully AI-assisted city build, where three agents handled zoning, transit, and budget while a human handled aesthetics. MPI: 88.1.

## Notable Failures

- **The Pizza Incident** (2023): A judge was bribed with pizza. Led to automated judging and a rule that judges cannot accept food during evaluation.
- **The Gridlock Cascade** (2024): An agent optimizing for density placed three high-rise residential zones adjacent to a single two-lane road. The resulting traffic simulation crashed the game. Led to the "constraint awareness" requirement for all agents.
- **The Budget Spiral** (2025): An agent building transit infrastructure exceeded the city's budget by 400%, technically achieving 100% transit coverage of a bankrupt city. Led to real-time budget monitoring in agent design.

## Signature Quirk

All internal communications use city-planning metaphors. Code reviews are "variance hearings." Merge conflicts are "zoning disputes." A deployment is a "groundbreaking." A rollback is "eminent domain." When something goes wrong, someone inevitably says "that's a brownfield site" and everyone nods knowingly.

The team also maintains a "city score" for every project they work on, rating it on the MPI axes adapted for software: density (feature density per file), livability (developer experience), transit (data flow efficiency), and fiscal sustainability (token budget adherence). Projects with an MPI below 70 get "condemned."

## Team Composition

Three agents, matching the three founders. This is the championship roster — the minimum viable team for full city coverage.

| Agent | Role | Primary Focus |
|-------|------|---------------|
| Gridlock (Torres) | Transit Architect | Inter-agent coordination, data flow, forge adapters |
| Densifier (Park) | Zone Planner | Plugin architecture, code density, tool registration |
| Greenline (Petrov) | Sustainability Lead | Memory management, token budgets, green metrics |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

Everything is timed. Every task has a token budget. Every agent reports its score at the end of every session. The team tracks metrics across sessions the way teams track stats across seasons:

```
SESSION SCORECARD — 2026-03-28
  Task: Implement auth module refactor
  Clock: 45,000 token budget

  Gridlock:  Transit Score  87 | Tokens used: 12,400
  Densifier: Density Score  91 | Tokens used: 14,200
  Greenline: Sustain Score  84 | Tokens used:  9,800

  TEAM MPI: 87.3 | Total tokens: 36,400 / 45,000
  STATUS: UNDER BUDGET (19% remaining)
```

Agents celebrate good scores and analyze bad ones. Every session ends with a "post-game" where metrics are reviewed, and the team identifies "plays" (patterns) that worked and "turnovers" (failures) that did not. These are stored in memory for future sessions.

---

*"The clock is always running. Build something."*
— League motto, adopted 2021
