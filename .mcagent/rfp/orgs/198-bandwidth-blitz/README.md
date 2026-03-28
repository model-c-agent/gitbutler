# Bandwidth Blitz

**"Clock starts when the truck parks. Clock stops when the signal locks."**

---

## The League

Bandwidth Blitz is a competitive sports league for telecommunications tower deployment. Yes, really. Teams of five race to install a temporary cell tower — from unboxing to live signal — in the shortest time possible. Events are held at decommissioned industrial sites, fairgrounds, and occasionally parking lots. The record for a standard small cell deployment is 47 minutes and 12 seconds, held by Team Frequency Force from Albuquerque.

The league was founded in 2023 by Tasha "Blitz" Morrison, a former tower technician who spent seven years climbing cell towers for a major carrier. She started the league after watching a deployment crew take eleven hours to install equipment that she could install in ninety minutes. The bottleneck was not the hardware — it was the coordination: permits, site plans, configuration files, safety inspections, and the six phone calls required to get someone at the network operations center to activate the cell.

Tasha's insight: deployment speed is not a hardware problem. It is a coordination and configuration problem. The league exists to prove this. Teams compete on speed, but the underlying skill is coordination — configuring equipment correctly the first time, communicating clearly under pressure, and pre-staging everything so that deployment is execution, not discovery.

The league has 14 active teams across North America, with exhibition events in the UK and Australia. Events draw 200-400 spectators (tower technicians, RF engineers, telecom nerds, and a surprising number of people who just enjoy watching competent professionals work fast). The league is sponsored by three equipment manufacturers who provide hardware and, more importantly, take notes on which configurations their equipment struggles with under time pressure.

## Technology

Bandwidth Blitz runs on software built by the league's technical staff. Each competition event requires: real-time timing systems, configuration validation (the tower must actually work, not just be assembled), spectrum analysis (to verify the deployed signal meets minimum quality), and a scoring engine that combines speed with quality metrics.

The scoring engine is the most complex component. Raw speed is 60% of the score. Signal quality at deployment completion is 30%. Safety compliance (judged by certified safety inspectors) is 10%. The scoring engine processes data from timing sensors, spectrum analyzers, and safety inspectors, and computes a final score within 30 seconds of completion.

All scoring data is stored in Git — each event is a repository, each team's deployment is a branch, and the scoring timeline is a sequence of commits. This started as a transparency measure (teams disputed scores until the scoring became auditable) and evolved into the league's primary data platform.

In 2025, the league built AI agents to analyze deployment patterns and generate optimized configuration templates. The agents ingest past deployment data (timing, configuration choices, signal quality) and produce pre-staged configuration bundles that teams can use as starting points. The agents also power the league's "deployment coach" — a real-time advisory system that suggests configuration adjustments during practice runs.

## Philosophy

Speed without quality is recklessness. The league penalizes teams that deploy fast but produce poor signal quality. A tower that goes up in 40 minutes but drops 30% of connections is worse than a tower that goes up in 55 minutes with perfect signal. Agents follow the same principle: a fast patch that introduces bugs is worse than a slower patch that is correct.

## The Grounding Incident

At the 2025 Austin Invitational, Team Copper Wire completed their deployment in 51 minutes — a personal best — but the safety inspector flagged an improper grounding connection. The team's configuration agent had generated a wiring plan that omitted a grounding step because the step was not in the equipment manufacturer's default template. The manufacturer's template assumed permanent installations with dedicated ground rods; the competition uses temporary installations with portable grounding kits.

The team was disqualified. The incident led to the league adding "deployment context" as a mandatory input to all configuration agents. The agent must know whether it is generating for permanent or temporary installation, indoor or outdoor, competition or production.

## Achievement

**47:12 world record**: Team Frequency Force's record-setting deployment at DreamHack Dallas 2025 used an agent-optimized configuration bundle that eliminated 8 minutes of manual parameter entry. The bundle was pre-staged, pre-validated, and required only site-specific adjustments (channel assignment, transmit power) at deployment time.

## League Staff (Technical)

| Member | Title | Role |
|--------|-------|------|
| Tasha Morrison | Commissioner / Lead | Product direction, event operations |
| Diego Vasquez | Scoring Engineer | Scoring engine, timing systems |
| Sam Okafor | Configuration Analyst | Template generation, agent training |
| Anya Petrov | Infrastructure Lead | Plugin architecture, forge integration |
| Kai Lindberg | Data Steward | Memory, deployment analytics, history |

Details in [AGENTS.md](AGENTS.md).

---

*"If you're still configuring at minute ten, you already lost."*
— Tasha Morrison, pre-event briefing
