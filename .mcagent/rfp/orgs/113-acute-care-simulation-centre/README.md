# The Acute Care Simulation Centre

**"Run it again. Change one variable. See what breaks."**

---

## Origin

The Acute Care Simulation Centre exists because Dr. Helen Park ran 500 emergency simulations in a single year and got tired of documenting the results on whiteboards.

Dr. Park is the director of the simulation laboratory at a mid-size teaching hospital in Manchester, England. The lab runs high-fidelity medical simulations — mannequins with programmable vital signs, actors playing distressed family members, real medical equipment, and scenarios designed to push trainees to their decision-making limits. Cardiac arrests, anaphylaxis, trauma, pediatric emergencies. Five hundred a year, Monday through Saturday, three per day.

The lab's purpose is educational: train junior doctors to make better decisions under pressure. But the lab also produces data. Every simulation is recorded. Vital signs are logged. Decision timestamps are captured. Outcomes are tracked. Over ten years, Dr. Park accumulated a dataset of 5,000 simulated emergencies with complete decision traces — something that does not exist in real emergency medicine because real emergencies are too chaotic to instrument fully.

In 2023, Dr. Park hired three computational researchers to analyze the dataset. They discovered patterns invisible to the naked eye: specific decision sequences that reliably produced better outcomes, timing thresholds where delays became critical, and interactions between team communication patterns and clinical results. They built predictive models. The models worked.

The leap to AI agents came naturally. If you can model how a human team makes decisions in a simulated emergency, you can build agents that follow the same decision patterns. The Centre now builds agent systems for non-medical domains using insights from medical simulation — treating code generation as a high-stakes decision process where timing, sequencing, and communication patterns matter as much as the decision itself.

## Philosophy

**Simulation-driven development.** Before building anything, simulate it. Before deploying anything, simulate the deployment. Before changing anything, simulate the change. The Centre does not trust untested code, untested processes, or untested assumptions. They run simulations the way other teams run unit tests — constantly, obsessively, and with the expectation that most simulations will reveal something unexpected.

## The Tension

Dr. Park and Dr. Kwame Asante (the lead computational researcher) disagree about simulation fidelity. Park believes simulations must replicate real conditions as closely as possible — use the real codebase, the real providers, the real token budgets. Asante argues that high-fidelity simulation is too slow for iterative development and advocates for "reduced-fidelity" simulations using mock providers and truncated codebases. Park's response: "Low-fidelity simulation produces high-confidence nonsense." They compromise by running reduced-fidelity simulations during development and full-fidelity simulations before any release.

## Notable Achievement

In 2025, the Centre published a paper demonstrating that AI agent error rates follow the same "golden hour" pattern as medical emergency outcomes: the probability of a successful patch decreases exponentially with the time elapsed since the agent first read the codebase context. After approximately 45 minutes of continuous operation (measured in wall-clock time with a fast model), context staleness causes a measurable increase in errors. This finding led to their "simulation cycle" architecture: agents operate in 30-minute bounded cycles with mandatory context refresh.

## Team

Five members. Dr. Park leads research direction. Operational decisions are collaborative.

| Agent | Role | Focus |
|-------|------|-------|
| Dr. Helen Park | Director / Simulation Design | Test scenarios, simulation architecture |
| Dr. Kwame Asante | Computation Lead | Patch generation, model optimization |
| Niamh Callaghan | Systems Engineer | Provider abstraction, infrastructure |
| Tomoko Ishida | Data Architect | Agent memory, pattern extraction |
| Farid Anwar | Security Researcher | Commit signing, adversarial testing |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Weekly rhythm: Monday through Thursday is development. Friday is simulation day — the entire team runs the week's changes through a battery of simulated tasks using recorded scenarios from their 5,000-simulation archive. Saturday morning, Dr. Park reviews the simulation results and publishes a "simulation report" — a structured document listing what passed, what failed, and what needs changing. Nothing ships until it survives Friday.

---

*"If you haven't simulated it, you don't know if it works."*
— Dr. Helen Park, at every code review
