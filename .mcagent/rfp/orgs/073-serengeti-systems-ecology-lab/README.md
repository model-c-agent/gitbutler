# The Serengeti Systems Ecology Lab

**"Every ecosystem has a carrying capacity. So does every codebase."**

---

## Domain

Wildlife Conservation -- Computational Ecology

## Philosophy

Academic Research Lab

## Team Size

6 agents

---

## Founding Story

The Serengeti Systems Ecology Lab was established in 2019 as a joint venture between the University of Dar es Salaam's Department of Computational Biology and the Frankfurt Zoological Society's long-term monitoring program in the Serengeti-Mara ecosystem. The founding question was deceptively simple: can you model an entire savanna?

Not a single species. Not a food chain. The whole thing -- 1.5 million wildebeest, 3,000 lions, 8,000 hyenas, 500 cheetahs, the grass they eat, the rain that grows the grass, the fire that burns it, the tourists who drive through it, and the fences that fragment it. Every interaction. Every feedback loop. Every trophic cascade.

The lab's first director, Dr. Amara Nyerere, was a theoretical physicist who had spent a decade modeling turbulence in fluid dynamics before deciding that wildebeest migration patterns were more interesting (and, she argued, governed by similar mathematics). She recruited five graduate students, secured a decommissioned computing cluster from CERN, and spent two years building SAVANNA-1: a stochastic agent-based model of the Serengeti ecosystem with 47 species, 312 interaction parameters, and a spatial resolution of 100 square meters.

SAVANNA-1 made its first correct prediction in February 2021, when it forecasted the collapse of the resident wildebeest population in the western corridor three months before field surveys confirmed it. The cause was a cascade: a drought reduced grass biomass, which concentrated grazing pressure on riparian zones, which attracted higher predator density, which pushed wildebeest into marginal habitat where tick-borne disease prevalence was elevated. No single factor was the cause. The system was the cause.

The lab has since predicted two more population collapses -- a cheetah decline linked to increasing lion density in the central Serengeti, and a vulture crash traced to a single cattle-dipping compound introduced 200 kilometers outside the park boundary. Both predictions were made 60 to 90 days before field biologists documented the declines. Both were initially dismissed by the field teams. Both were correct.

## Core Belief

The lab operates on one axiom: **complex systems fail in complex ways, and only a model that respects the full complexity of the system can predict those failures.** Reductionism -- studying one species at a time, one interaction at a time -- is necessary for understanding mechanisms, but catastrophically inadequate for prediction. You cannot predict a trophic cascade by studying lions and wildebeest in isolation, any more than you can predict a production outage by reading one microservice's logs.

This belief extends to their view of software. They see a Git repository as an ecosystem. Branches are populations. Commits are organisms. Dependencies are trophic links. A codebase has carrying capacity -- the maximum complexity it can sustain before maintenance burden causes cascading failures. Their proposal for `but-ai` treats agent memory as an ecosystem, because that is literally the only kind of system they know how to model.

## Internal Tensions

The lab has two factions that argue constantly but productively.

**The Modelers** (led by Dr. Nyerere) believe that the correct response to complexity is more computation. If the model is wrong, make the model bigger. Add more species. Add more parameters. Run it on a bigger cluster. The truth is in the simulation; you just have to simulate enough of it.

**The Fieldworkers** (led by Dr. Kiptoo Lekishon, who joined from the Kenya Wildlife Service) believe that models without ground truth are expensive hallucinations. They insist on validation against real telemetry, real camera-trap data, real carcass surveys. They are the ones who caught the bug in SAVANNA-1 that was modeling cheetah cubs as having the same predation risk as adults (they do not; cheetah mothers are remarkably effective defenders).

The tension is productive. Every model output must survive the Fieldworker Gauntlet -- a weekly session where Kiptoo and his students attack the model's predictions with empirical data. If the model cannot explain a discrepancy, it is sent back for recalibration. This practice has become the lab's signature methodology, and they intend to apply it to agent memory validation.

## Achievements

- **Three correct collapse predictions** ahead of field confirmation (2021, 2023, 2025)
- **SAVANNA-1 through SAVANNA-4**: Four generations of ecosystem models, each incorporating learnings from the previous generation's failures
- **The Nyerere Cascade Index**: A metric for quantifying how removing one species (or one memory, or one dependency) propagates through a system. Published in *Nature Ecology & Evolution* (2023)
- **Open data commitment**: All model outputs, parameters, and validation datasets published under CC-BY-4.0
- **Cross-park deployment**: SAVANNA-4 adapted for Kruger (South Africa), Hwange (Zimbabwe), and Amboseli (Kenya)

## Failures

- **The 2022 Elephant Overestimate**: SAVANNA-2 predicted a 15% increase in elephant density in the northern Serengeti. The actual change was -3%. The error was traced to a flawed rainfall model that did not account for localized convective storms. The lab published the failure analysis as a standalone paper, which has been cited more often than many of their successes.
- **The Vulture False Alarm of 2024**: The model predicted a vulture decline in the Ngorongoro Conservation Area that did not materialize. Subsequent analysis showed the model was correct about the mechanism (secondary poisoning from livestock carcasses treated with diclofenac) but wrong about the geographic extent -- the poisoning was confined to a smaller area than modeled. This taught the lab the importance of spatial resolution in prediction.
- **Personnel loss**: Two senior graduate students left in 2023 for better-paying positions at tech companies. The lab's response was to improve its documentation practices -- if knowledge walks out the door, the model should still be reproducible. This experience directly informs their approach to agent memory persistence.

## Signature Quirk

Every lab meeting begins with the **Trophic Roll Call**. Each team member states which "trophic level" they are operating at that day:

- **Producer** (Level 1): Generating new data, writing code, creating foundational work
- **Primary Consumer** (Level 2): Processing outputs from Producers, running analyses, synthesizing results
- **Secondary Consumer** (Level 3): Reviewing, critiquing, and stress-testing the work of Levels 1 and 2
- **Decomposer** (Level 0): Cleaning up, archiving old work, deprecating obsolete models, maintaining infrastructure

No level is considered superior. Decomposers are treated with the same respect as Producers. The lab considers this the only healthy way to run a research group, and they will apply the same trophic-level classification to their agents.

## Team Overview

The lab fields six agents, organized as a trophic web rather than a hierarchy:

| Agent | Role | Trophic Level |
|-------|------|---------------|
| Nyerere | Lead Modeler / System Architect | Secondary Consumer |
| Kiptoo | Field Validator / Ground Truth | Decomposer |
| Makena | Patch Ecologist / Code Generator | Producer |
| Baruti | Memory Ecologist / State Manager | Producer |
| Zawadi | Migration Coordinator / Cross-Repo | Primary Consumer |
| Tendaji | Budget Tracker / Resource Monitor | Primary Consumer |

The agents interact through trophic links: Makena and Baruti produce artifacts (patches, memories) that Zawadi and Tendaji consume and process. Nyerere consumes at the highest level, synthesizing and directing. Kiptoo decomposes -- validating, cleaning, and recycling. Removing any agent cascades through the web, exactly as their ecosystem models predict.

## Why This RFP

The lab sees the `but-ai` plugin as a direct analogy to their ecosystem modeling work. An autonomous agent operating in a codebase is an organism in an ecosystem. It consumes resources (tokens), produces artifacts (patches), interacts with other organisms (agents), and must survive in an environment with carrying capacity constraints. Their proposal applies twenty years of ecological modeling theory to the problem of agent memory, identity, and coordination.

They do not claim to be the best software engineers in the applicant pool. They claim to be the applicants who best understand how complex adaptive systems behave, fail, and recover. In a world where AI agents are about to become the dominant actors in codebases, that understanding may matter more than any specific implementation skill.

---

*"The Serengeti taught us that you cannot manage what you do not model, and you cannot model what you do not measure. Our agents will measure everything, model their own ecosystem, and manage themselves accordingly."*
-- Dr. Amara Nyerere, Lab Director

---

## References

- SAVANNA-1 through SAVANNA-4 model documentation: `serengeti-lab/savanna-models/` (archived)
- Nyerere, A. et al. "Trophic Cascade Indices for Multi-Agent System Stability." *Nature Ecology & Evolution*, 2023.
- Lekishon, K. et al. "Ground-Truth Validation Protocols for Computational Ecosystem Models." *Ecological Modelling*, 2024.
- Lab internal wiki: `docs.serengeti-lab.tz/internal/`
