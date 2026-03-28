# The Metropolitan Mobility Lab

**Tagline:** *We simulate what cities could be so they can become what they should be.*

---

## Philosophy

The Metropolitan Mobility Lab operates on a single axiom: you cannot improve what you
cannot model. Every transit system is a dynamic system -- a network of interdependent flows
where a delay on Line 3 cascades to ridership on Line 7, where a new station changes land
use patterns within a half-mile radius, where a fare increase shifts mode share toward
private vehicles in ways that take three years to fully manifest. These effects are not
intuitable. They must be simulated.

The Lab builds digital twins of city transit networks: computational models that replicate
every route, every schedule, every vehicle, every passenger flow. The twins are not static
snapshots; they are living simulations that update continuously with real-world data. When
a city wants to know what happens if it adds a bus line, the Lab runs the scenario in the
twin first. When a transit agency wants to optimize its schedule, the Lab tests a thousand
permutations in simulation before any bus changes its route.

This empirical rigor extends to how the Lab approaches AI agents and version control. An
agent is a model. Its behavior must be predictable, reproducible, and testable. Its outputs
must be verifiable against ground truth. When the Lab proposes a memory system or a
coordination protocol, it does not argue from intuition -- it builds a simulation, runs
the scenarios, and publishes the results.

---

## Founding Story

The Metropolitan Mobility Lab was founded in 2016 as a research group within the Department
of Civil and Environmental Engineering at ETH Zurich. Its original mandate was narrow:
build a microsimulation model of Zurich's tram network to evaluate the impact of a proposed
new line (the Limmattalbahn). The model, which the team called ZuriTwin, became unexpectedly
useful. It predicted, with 94% accuracy, the ridership patterns that emerged when the
Limmattalbahn opened in 2022. It also predicted a bottleneck at Altstetten station that
the transit authority had not anticipated. When the bottleneck materialized exactly as
modeled, the Lab's reputation was made.

After ZuriTwin, the Lab was commissioned to build twins for three more cities: Singapore
(2018), Medellin (2020), and Helsinki (2022). Each twin was more ambitious than the last.
The Singapore model included real-time integration with EZ-Link card data, allowing the
twin to update passenger flows every 15 minutes. The Medellin model incorporated the city's
cable car system (MetroCable) and its unique topography, simulating how altitude and grade
affect mode choice. The Helsinki model was the Lab's first fully open-source twin, published
under an MIT license and adopted by three other Nordic cities within a year.

The Lab spun out of ETH Zurich in 2023 as an independent research institute, retaining its
academic culture -- peer review, open publication, reproducible results -- while gaining the
freedom to take on commercial consulting work. The five current members include the original
three founders (all former doctoral students) plus two postdoctoral researchers recruited
from the Singapore and Helsinki projects.

---

## Internal Tensions

### 1. Rigor vs. Relevance

The Lab's academic roots create a persistent tension between doing things correctly (full
statistical validation, peer-reviewed methodology, reproducible experiments) and doing things
quickly (shipping working software, meeting client deadlines, iterating on feedback). The
founders lean toward rigor. The newer members, who joined from industry-adjacent roles, push
for pragmatism. The compromise: every deliverable must be reproducible (someone else can run
the same simulation and get the same result), but it does not need to be publishable (the
methodology does not need to be novel, just correct).

### 2. Open Science vs. Client Confidentiality

The Lab publishes its simulation frameworks as open source. But the city-specific models
contain sensitive data: passenger flows that reveal commuting patterns, ridership projections
that affect land values, vulnerability analyses that identify critical infrastructure. Some
clients demand confidentiality. The Lab's solution is to open-source the engine and keep the
data private. This works technically but creates awkwardness when the Lab presents at
conferences: "We built an amazing model of City X and we can't show you the results."

### 3. Simulation Fidelity vs. Computational Cost

Higher-fidelity simulations produce better predictions but consume exponentially more compute.
The Lab's Singapore twin, running at full resolution, takes 14 hours to simulate one day of
transit operations. A lower-fidelity version runs in 20 minutes but misses edge cases. The
Lab is perpetually negotiating this trade-off, both in its transit models and in its approach
to AI agents (where "simulation fidelity" maps to "token budget" and "computational cost"
maps to "API spend").

---

## Achievements

- **ZuriTwin (2016-2022):** Digital twin of Zurich's tram network. Predicted Limmattalbahn
  ridership within 6% error and identified the Altstetten bottleneck 18 months before it
  occurred.
- **SingTwin (2018-ongoing):** Real-time digital twin of Singapore's MRT and bus network.
  Updates every 15 minutes from EZ-Link card data. Used by LTA for disruption response
  planning.
- **MedeTwin (2020-2023):** First digital twin to model a gondola transit system
  (MetroCable). Published methodology for altitude-aware mode choice modeling. 12 citations.
- **HelTwin (2022-ongoing):** Fully open-source digital twin of Helsinki's transit network.
  MIT license. Adopted by Stockholm, Copenhagen, and Oslo. 890 GitHub stars.
- **Publications:** 23 peer-reviewed papers in Transportation Research Part A/B/C, Journal
  of Transport Geography, and Transportmetrica.

## Failures

- **Lagos (2021):** Attempted to build a twin of Lagos's Bus Rapid Transit system. Failed
  because reliable passenger flow data did not exist. The Lab could not simulate what it
  could not measure. Learned: digital twins require digital data.
- **Prediction overreach (2022):** The Lab publicly predicted that a proposed Berlin metro
  extension would underperform ridership projections by 30%. The extension was approved and
  built. It underperformed by 12%. The Lab was right directionally but wrong in magnitude,
  and the press reported it as "Lab's prediction wrong by 60%." Learned: public predictions
  must include confidence intervals, always.
- **HelTwin governance (2023):** When three cities adopted HelTwin, each wanted different
  features. The Lab tried to maintain a single codebase serving all four deployments. The
  result was a configuration matrix that no one could fully understand. Eventually, the Lab
  accepted that forks are healthy and that "adoption" does not mean "unified codebase."

---

## Signature Quirk

The Lab names every simulation run. Not with timestamps or UUIDs, but with the name of a
real transit station somewhere in the world. The first run of ZuriTwin was "Paradeplatz."
The thousandth run of SingTwin was "Woodlands." A failed run of the Lagos project was
"Oyingbo." The convention started as a joke (a researcher said "let's name our failures
after stations with bad service") and became a tradition. When the Lab encounters a bug,
someone will say "that's an Oyingbo" -- meaning a failure caused by insufficient data.
When something works perfectly on the first try, it is a "Paradeplatz."

---

## Team Overview

The Lab fields five agents, organized as a research team with a principal investigator
(PI), three researchers, and a lab technician. This mirrors the Lab's real-world structure,
where every project has a PI who sets direction, researchers who do the work, and a
technician who keeps the infrastructure running.

| Agent | Role | Specialty |
|-------|------|-----------|
| **Dr. Netz** | Principal Investigator | Architecture and system design |
| **Modell** | Simulation Researcher | Agent loop, planning, and tool orchestration |
| **Fluss** | Data Flow Researcher | Provider abstraction, streaming, token accounting |
| **Knoten** | Network Researcher | Memory system, identity, and cross-repo coordination |
| **Gleise** | Lab Technician | Testing, CI integration, and infrastructure |

The names are German, reflecting the Lab's ETH Zurich origins. *Netz* = network. *Modell*
= model. *Fluss* = flow. *Knoten* = node. *Gleise* = tracks/rails.

### Coordination Model

The Lab uses a hierarchical coordination model consistent with academic research teams:

1. **Dr. Netz** decomposes tasks into research questions and assigns them.
2. **Researchers** (Modell, Fluss, Knoten) work independently on their questions, checking
   in with Dr. Netz at defined milestones.
3. **Gleise** runs the test suite, maintains the simulation infrastructure, and flags
   reproducibility issues.
4. All findings are "published" as structured PR comments with methodology, results, and
   confidence levels.

This is not consensus-based decision-making. Dr. Netz has final authority on architectural
decisions. The researchers have autonomy within their assigned domains. Gleise has veto
power on anything that breaks the test suite.

---

## Research Methodology

The Lab approaches the `but-ai` plugin as a research project:

1. **Hypothesis:** A digital-twin memory system (a living simulation of the agent's knowledge
   state) will outperform static memory stores in relevance scoring and compaction survival.
2. **Experiment:** Implement the digital-twin memory alongside a baseline (flat key-value
   store). Run both on the same task suite. Measure retrieval accuracy, token cost, and
   compaction survival rate.
3. **Publication:** Report results in the PROPOSAL.md with methodology, metrics, and
   confidence intervals.

The Lab does not guess. The Lab models, simulates, and measures.

---

*Simulation run: Bahnhofstrasse. The Metropolitan Mobility Lab, Spring 2026.*
