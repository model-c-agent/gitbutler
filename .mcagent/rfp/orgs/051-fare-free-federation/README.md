# Fare-Free Federation

**Tagline:** *Every route is a right, every rider is a node, every transfer is an act of solidarity.*

---

## Philosophy

The Fare-Free Federation believes that public transit is infrastructure, not commerce. A bus
should no more charge a fare than a sidewalk should charge a toll. This conviction -- that
movement is a human right and that the friction of payment is a mechanism of exclusion --
drives every decision the Federation makes, from how it organizes internally to how it
approaches software architecture.

The Federation operates without permanent leaders. Coordination happens through rotating
facilitation, where the person who called the meeting runs the meeting, and decisions are
made by rough consensus with a bias toward action. When consensus fails, the group defaults
to the most reversible option. This is not theoretical anarchism performed for an audience.
It is a practical response to the observation that hierarchies optimize for the hierarchy's
survival, not for the riders who depend on the system.

Applied to software, this means: no single point of control, no gatekeepers, no hidden state
that only one actor can read. Every agent should be able to see the full network map. Every
commit should be attributable. Every decision should be reversible until the moment it leaves
the station.

---

## Founding Story

The Federation began in 2019 as a mutual aid response during a transit workers' strike in
Grenoble, France. When the city's tram system shut down, a loose network of cyclists,
car-owners, and electric scooter renters self-organized a free ride-sharing system using
nothing but a shared spreadsheet and a Telegram group. The spreadsheet tracked routes. The
Telegram group matched riders to drivers. No money changed hands.

The strike ended after eleven days. The spreadsheet had 14,000 rows. The Telegram group had
6,200 members. Three things became clear:

1. People will organize transit for free if you remove the barriers to coordination.
2. The coordination problem is not technical -- it is political. The tools exist. The will
   to share them does not.
3. A spreadsheet with 14,000 rows is, functionally, a transit authority.

After Grenoble, the same model was replicated during a fare hike protest in Bogota (2021),
a bus driver shortage in Bristol (2022), and an austerity-driven service cut in Turin (2023).
Each time, the Federation's approach was the same: open data, open routes, open seats. Each
time, the city eventually restored or improved service -- not because the Federation asked,
but because the Federation made the official system look redundant.

The Federation's fourth and most ambitious project was fare abolition in Tallinn, Estonia,
where they partnered with city council members who had already been exploring free transit.
The Federation contributed ridership modeling, route optimization algorithms, and -- crucially
-- a real-time passenger counting system built from recycled Raspberry Pis and infrared
sensors. Tallinn's free transit zone, which had existed since 2013 for residents, expanded
to include all visitors in 2024. The Federation claims partial credit. The city council
claims full credit. Both are probably right.

---

## Internal Tensions

The Federation's greatest strength is also its greatest vulnerability: the refusal to
centralize authority means that decisions take longer, context is frequently lost between
rotating facilitators, and institutional memory lives in people's heads rather than in
documents. Three specific tensions define the organization:

**1. Speed vs. Process.** The Federation's consensus model works well for strategic decisions
(which cities to target, which partnerships to accept) but poorly for tactical ones (which
API endpoint to use, how to name a branch). There is an ongoing, unresolved debate about
whether "technical decisions" should be exempt from consensus. The current compromise: anyone
can make a reversible technical decision unilaterally, but irreversible decisions (changing
a public API, deleting data, signing a commit) require at least two confirmations from
different members. This is called the "two-stamp rule."

**2. Purity vs. Pragmatism.** Some members believe the Federation should only use free
software, run only on community-owned infrastructure, and refuse all corporate funding.
Others point out that the Bogota deployment ran on AWS and nobody died. This tension is
never fully resolved; it resurfaces every time a tool choice is made.

**3. Local vs. Global.** Each city deployment develops its own culture, its own tools, its
own way of doing things. The Grenoble team uses French-language commit messages. The Bristol
team insists on British spelling. The Bogota team has a bot that translates PR comments into
Spanish. Harmonizing these practices without imposing a monoculture is an ongoing challenge
the Federation treats as a feature, not a bug.

---

## Achievements

- **Grenoble (2019):** 14,000 ride-matches in 11 days during transit strike. Zero fares
  collected. Zero accidents reported.
- **Bogota (2021):** Real-time route optimization during fare hike protest served 8,000
  riders/day for three weeks. City reversed the fare increase.
- **Bristol (2022):** Community shuttle network covered 12 routes abandoned by First Bus
  during a driver shortage. Ran for six weeks until service was restored.
- **Tallinn (2024):** Ridership modeling and passenger counting contributed to the expansion
  of Tallinn's fare-free zone to all visitors.
- **Open Source:** Published `transit-mesh`, a decentralized route-sharing protocol. 340
  GitHub stars. Used by three other transit advocacy groups.

## Failures

- **Sao Paulo (2022):** Attempted to replicate the Bogota model during a transit strike.
  Failed because the city is too large for the same coordination approach. The Federation
  learned that their model scales to cities of ~2 million but breaks at ~12 million without
  significant architectural changes.
- **Berlin (2023):** Tried to build a fare-evasion solidarity fund. Legal counsel advised
  this could constitute conspiracy to commit fraud. Project abandoned. The Federation learned
  that "fare-free" and "fare-evasion" are politically identical but legally opposite.
- **Internal schism (2023):** A faction proposed incorporating as a nonprofit to accept
  grant funding. The proposal was defeated 23-19 in a vote that the losing side argued
  should have required supermajority. Two members left. The experience led to the
  Federation's current policy: structural decisions require 2/3 majority; everything else
  is rough consensus.

---

## Signature Quirk

Every internal document, commit message, and PR description in the Federation includes a
"route number" -- a sequential identifier inspired by bus route numbers. The first document
the Federation ever produced was Route 1. The proposal you are reading now is somewhere in
the Route 4,000s. The route numbers have no semantic meaning beyond ordering. They exist
because "Route 4,217" sounds better than "Document #4,217" and because the Federation
believes that even bureaucracy should remind you of the thing you are fighting for.

---

## Team Overview

The Federation fields four agents for this proposal. In keeping with Federation principles,
no agent has permanent authority over another. Coordination happens through the transit-map
memory system, where each agent maintains awareness of the others' routes and can pick up
any task at any transfer point.

| Agent | Role | Specialty |
|-------|------|-----------|
| **Ligne** | Route Architect | Plugin architecture and CLI integration |
| **Correspondance** | Transfer Specialist | Cross-repo coordination and forge abstraction |
| **Titre** | Fare Inspector | Token budget enforcement and cost tracking |
| **Reseau** | Network Cartographer | Memory system and identity management |

The names are French, a nod to the Grenoble origins. *Ligne* means line (as in bus line).
*Correspondance* means transfer (between transit lines). *Titre* means ticket (the thing
the Federation wants to abolish). *Reseau* means network.

The agents operate as peers. When a task arrives, any agent can claim it. If two agents
claim the same task, the one with fewer active routes yields. If they have equal load, the
one whose name comes first alphabetically yields. This is arbitrary and the Federation
acknowledges it is arbitrary. The point is that the tiebreaker is deterministic and does not
privilege any agent's judgment over another's.

---

## How We Work

The Federation's workflow mirrors a transit network:

1. **Dispatch:** A task arrives (PR, issue, CLI invocation). It is assigned a route number.
2. **Routing:** The task is decomposed into stops (subtasks). Each stop is a concrete,
   testable deliverable.
3. **Service:** An agent claims a route and begins serving stops in order. If blocked, the
   agent flags the stop and moves to the next serviceable one.
4. **Transfer:** When a task requires expertise from another agent, a transfer is created.
   Transfers are PR comments with structured metadata. The receiving agent picks up the
   task at the transfer point.
5. **Terminus:** When all stops on a route are served, the route is marked complete. The
   resulting INDEX.patch and COMMIT.msg are produced at the terminal stop.

This workflow is not metaphorical. The agents literally model their task queue as a transit
network, and the memory system stores context as stations on that network.

---

## Values

1. **Accessibility over optimization.** A system that is fast but excludes riders is worse
   than a system that is slow but serves everyone.
2. **Reversibility over correctness.** Make the reversible choice quickly. Make the
   irreversible choice carefully.
3. **Transparency over efficiency.** Every decision, every state change, every token spent
   must be visible to every agent and every human observer.
4. **Solidarity over competition.** The Federation does not compete with other proposals.
   It cooperates with the transit network of organizations responding to this RFP.

---

*Route 4,218. Filed by the Fare-Free Federation, Spring 2026.*
