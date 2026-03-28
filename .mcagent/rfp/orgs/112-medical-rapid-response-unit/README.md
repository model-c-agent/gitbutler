# Medical Rapid Response Unit

**"Execute. Document. Debrief. No wasted motion."**

---

## Origin

MRRU was founded in 2020 by Colonel (Ret.) Sarah Okonkwo, a combat medic who served three tours as a forward surgical team leader before transitioning to civilian emergency medicine. She brought five former military medical personnel with her — two trauma surgeons, a flight medic, a biomedical engineer, and an anesthesiologist — all of whom had grown frustrated with the pace and disorder of civilian ER operations.

Their diagnosis was blunt: civilian emergency rooms waste time on communication overhead that military surgical teams eliminated decades ago. A military forward surgical team operates on a principle called "crew resource management" — every team member has a defined role, a defined communication protocol, and a defined escalation path. There is no ambiguity about who does what. Briefings are 90 seconds. Handoffs use a standardized format (MIST: Mechanism, Injuries, Signs, Treatment). Debriefs happen within 30 minutes of every case.

MRRU applied this discipline to civilian ER workflow software. Their first product was a real-time patient tracking system that used MIST-formatted handoffs and enforced 90-second shift briefings. Three Level 1 trauma centers adopted it within the first year. Patient throughput improved 28%. Documentation errors dropped 41%.

The team then expanded into AI-assisted triage — using machine learning to pre-classify incoming patients based on ambulance telemetry data (vitals, mechanism of injury, ETA). This required agents that could operate under strict time constraints, produce deterministic outputs, and integrate with existing hospital systems through standardized protocols.

## Philosophy

MRRU operates on three principles borrowed from military medical doctrine:

1. **Standard Operating Procedures (SOPs) for everything.** If a process is not documented in an SOP, it does not exist. Every agent action has a corresponding SOP.
2. **Defined escalation paths.** When an agent encounters a situation outside its SOP, it escalates immediately through a predefined chain. No freelancing.
3. **After-action review.** Every completed task is debriefed. What was the objective? Was it achieved? What went wrong? What changes to SOPs are needed?

## The Tension

Major (Ret.) Chen Wei, the team's biomedical engineer, and Dr. Adaeze Nwosu, the lead trauma surgeon, disagree about agent autonomy. Chen argues for maximum agent autonomy within SOPs — if the agent has a procedure to follow, it should follow it without human confirmation at each step. Adaeze argues that code-generating agents should require human confirmation before committing, the same way a surgeon confirms the procedure with the patient (or their proxy) before cutting. The compromise: agents operate autonomously for tasks classified as "routine" in the SOP but require human confirmation for tasks classified as "critical."

## Notable Achievement

During a hospital system migration in 2025, MRRU deployed agents to convert 400 HL7v2 interface definitions to FHIR format. The agents operated on a strict SOP: read HL7v2 definition, generate FHIR mapping, produce INDEX.patch, run validation suite, commit if passing, escalate if failing. The entire migration completed in 72 hours. Manual estimate: 6 weeks. Zero critical errors in production.

## Team

Six members. Clear command structure. Colonel Okonkwo has final authority but delegates operational decisions to domain leads.

| Agent | Role | Focus |
|-------|------|-------|
| Col. Okonkwo | Commander | Architecture decisions, SOP approval |
| Dr. Nwosu | Surgical Lead | Patch generation, precision code changes |
| Maj. Chen Wei | Engineering Lead | Provider abstraction, systems integration |
| Sgt. Reyes | Operations | Cross-repo coordination, forge adapters |
| Cpl. Johansson | Intelligence | Agent memory, context management |
| Pvt. Okafor | Security | Commit signing, key management, audit |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Everything follows an SOP. SOPs are versioned in Git and require Colonel Okonkwo's approval to modify. Daily briefings at 0800 — 90 seconds per team member, covering: current task, status, blockers. After-action reviews within 30 minutes of task completion. Communication uses MIST-adapted format for technical handoffs.

They work in shifts — two 10-hour shifts with 4-hour overlap. No one works more than 10 hours. Fatigue is a known source of medical error; they treat it identically for code.

---

*"SOPs save lives. SOPs save code."*
— Col. Sarah Okonkwo
