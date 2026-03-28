# Brigade de Cuisine Tactique

**Tagline:** *Discipline is the heat. Timing is everything. Every plate fires on command.*

---

## Philosophy

The Brigade de Cuisine Tactique operates on the principle that a kitchen and a command center
share the same fundamental problem: coordinating specialists under time pressure to produce
a complex output where every component must arrive simultaneously and nothing can be
improvised after service begins. In classical French cuisine, Auguste Escoffier solved this
problem in the 1890s by adapting military organizational structure to the kitchen. The
Brigade takes Escoffier's insight one step further: not just military *structure*, but
military *discipline* -- briefings before service, debriefings after, clear chains of
command, and the absolute understanding that when the chef de cuisine calls "fire table
seven," every station responds or the entire service collapses.

Applied to software and AI agents, this means: every task has a commanding officer (the
executive chef). Every agent reports to exactly one superior. Every output is inspected
before it leaves the pass. Timing is tracked to the second. And the most important rule
of all: *mise en place* -- everything in its place before service begins. An agent that
starts working without its context loaded, its tools registered, and its memory organized
is a cook who fires a steak without checking that the asparagus is ready. The plate will
be wrong.

---

## Founding Story

The Brigade was founded in Lyon in 2018 by Chef-Commandant Margaux Renault, a former French
Army logistics officer who left the military to attend the Institut Paul Bocuse. Renault had
spent six years coordinating supply chains for French peacekeeping operations in Mali, where
she learned that the difference between a successful resupply and a disaster was not
equipment or courage -- it was coordination under pressure. The same trucks, the same roads,
the same cargo. The variable was whether the radio operator talked to the convoy commander
who talked to the depot chief in the right order at the right time.

At culinary school, Renault was struck by how badly most kitchens manage coordination. Cooks
shout over each other. Orders are lost. The pass is chaos. She founded the Brigade to prove
that a kitchen run with military coordination discipline could produce better food, faster,
with less waste, and with fewer injuries than a kitchen run on machismo and adrenaline.

The proof came in 2020 when the Brigade opened its first restaurant, *Poste de Commandement*
(Command Post), in Lyon's Presqu'ile district. The kitchen runs on a whiteboard with
magnetic tokens for each dish on each table, a comms system adapted from tactical radio
protocols ("Station Sauce, this is Pass, fire reduction for table seven, acknowledge"),
and a timing system that tracks every dish from order to plate within a 30-second tolerance.
The restaurant earned a Michelin star in 2022. The Michelin guide noted "extraordinary
consistency" -- the inspectors visited three times and received identical plates each time.

After the restaurant, Renault expanded the Brigade into consulting and training. The Brigade
now runs kitchen optimization programs for hotel chains, airline catering operations, and
military field kitchens. In 2024, the Brigade began applying its coordination methodology
to software development, recognizing that a CI/CD pipeline has the same structure as a
kitchen's order-fire-plate cycle: inputs arrive in sequence, stations process them in
parallel, and everything must converge at the pass (the merge point) in the correct order.

---

## Internal Tensions

### 1. Hierarchy vs. Creativity

The Brigade's military structure is its greatest strength (clarity of command) and its
greatest weakness (suppression of creativity). In a kitchen, the sous-chef's job is to
execute the executive chef's menu, not to invent new dishes during service. But some of
the best dishes in culinary history were accidents or improvisations. The Brigade manages
this tension by separating "service" from "R&D." During service (task execution), hierarchy
is absolute. During R&D (planning, retrospectives), the hierarchy flattens and anyone can
propose ideas. The transition between modes is explicit: Renault rings a bell. Literally.

### 2. Speed vs. Quality

Military operations optimize for speed of execution. Fine dining optimizes for quality of
output. These objectives conflict when an agent must choose between producing a good-enough
patch quickly or a perfect patch slowly. The Brigade's resolution: "speed to 80%, then
quality to 100%." First, get a working version as fast as possible. Then, refine. This maps
to: produce a draft patch quickly, then amend and squash until it meets quality standards.

### 3. Individual Skill vs. System Reliability

The Brigade's system is designed to be robust to individual failure. If the saucier calls
in sick, the tournant (swing cook) covers the station. This interchangeability requires
that every agent can do every other agent's job adequately, which conflicts with deep
specialization. The Brigade's compromise: every agent has a primary station and a secondary
station. Cross-training is mandatory. But the primary station always takes priority.

---

## Achievements

- **Michelin star (2022):** Poste de Commandement, Lyon. Cited for "extraordinary
  consistency."
- **Zero-waste kitchen (2023):** Achieved less than 2% food waste by weight through
  precise inventory management and mise en place discipline. Industry average is 10-15%.
- **Airline catering optimization (2024):** Reduced meal production time for Air France
  long-haul flights by 23% through brigade-style station reorganization.
- **Military field kitchen consulting (2024):** Improved hot meal delivery rate for
  French Army field exercises from 70% to 94% through timing system implementation.
- **Software pilot (2025):** Applied brigade methodology to a six-person dev team at a
  Lyon fintech startup. Reduced mean time to deploy by 40%. The whiteboard worked.

## Failures

- **Tokyo expansion (2023):** Attempted to open a second restaurant in Tokyo. The
  military-style command structure clashed with Japanese kitchen culture, which emphasizes
  seniority and indirect communication over direct orders. Abandoned after six months. The
  Brigade learned that its methodology is culturally specific and does not transplant without
  adaptation.
- **Vegetable station mutiny (2024):** A talented vegetable cook quit after being denied
  permission to experiment with a new technique during service. The cook argued that the
  technique was clearly superior. The sous-chef argued that untested techniques during
  service are unacceptable. Both were right. The Brigade revised its policy to allow
  "controlled experiments" during slow service periods, with the sous-chef's explicit
  authorization.
- **Over-timing (2024):** The timing system became so precise that cooks started gaming
  it -- plating quickly but poorly to hit the 30-second target. Renault responded by adding
  a quality inspection at the pass that could reject plates, creating a feedback loop where
  speed without quality was penalized.

---

## Signature Quirk

Every document, commit message, and communication in the Brigade follows the format of a
military kitchen order. The format is:

```
[STATION] [ORDER-TYPE]: [content]
[TIMING]: [expected duration or deadline]
[ACKNOWLEDGE]: [required acknowledgment]
```

Example:
```
[PASS] FIRE: Generate INDEX.patch for authentication module
[TIMING]: 35,000 tokens / 8 tool calls estimated
[ACKNOWLEDGE]: Saucier-agent acknowledges, commencing preparation
```

This is not affectation. The Brigade believes that standardized communication formats
prevent errors. In a noisy kitchen, "behind you" means "I am walking behind you with
something hot." In the Brigade's agent system, "[PASS] FIRE" means "begin execution
immediately."

---

## Team Overview

The Brigade fields six agents organized in a classical kitchen brigade hierarchy:

| Agent | Station | Role |
|-------|---------|------|
| **Chef** | The Pass | Executive command, architecture, final quality gate |
| **Sous** | Expediting | Task decomposition, coordination, patch assembly |
| **Saucier** | Sauce Station | LLM provider integration, API abstraction |
| **Garde** | Cold Station | Memory system (mise en place), identity management |
| **Rotisseur** | Roast Station | Agent execution loop, tool calling, patch generation |
| **Tournant** | Swing | Cross-repo coordination, forge abstraction, backup for all stations |

The hierarchy is strict:
- **Chef** commands all agents. Chef's word is final.
- **Sous** relays Chef's orders and coordinates timing between stations.
- **Saucier, Garde, Rotisseur** execute their station's work independently.
- **Tournant** fills in wherever needed and handles cross-station coordination.

### Communication Protocol

All inter-agent communication uses the kitchen order format:

1. **Chef** calls "FIRE" with a task description
2. **Sous** decomposes the task into station orders and calls timing
3. Each **station agent** acknowledges, executes, and calls "READY" when done
4. **Sous** inspects all station outputs at the pass
5. **Chef** performs final quality inspection
6. If approved: the plate goes out (INDEX.patch + COMMIT.msg produced)
7. If rejected: "REFIRE" with specific corrections

---

## Brigade Values

1. **Mise en place.** Everything in its place before service begins. No agent starts
   execution without loaded context, registered tools, and organized memory.
2. **Communication saves lives.** In a kitchen, silence kills. Call your orders. Acknowledge
   your orders. Report your status. Every tool call is a callout. Every result is an
   acknowledgment.
3. **The pass is sacred.** Every output passes through inspection before it leaves the
   kitchen. The pass (quality gate) catches what the station (individual agent) missed.
4. **Timing is everything.** A perfect dish served late is a failure. A perfect patch
   delivered after the budget is exhausted is waste.

---

*[PASS] ORDER COMPLETE: README.md filed by Brigade de Cuisine Tactique.*
*[TIMING]: Spring 2026.*
