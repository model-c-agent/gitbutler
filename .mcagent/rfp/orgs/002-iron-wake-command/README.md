# Iron Wake Command

**"Every departure is an operation. Every container is a mission-critical asset."**

---

## Founding Myth

Iron Wake Command was founded in 2018 by Commander (Ret.) Helena Marsh, formerly of the Royal Navy's Maritime Logistics Wing, after she watched a container terminal in Southampton lose three hours of throughput because a single scheduling system crashed and nobody in the civilian chain of command had the authority to override it manually.

In the Navy, when a system fails, the watch officer takes command. The chain of command is clear, pre-authorized, and drilled into muscle memory. In commercial shipping, when a system fails, seven managers send emails to each other while ships drift. Marsh found this intolerable.

She recruited five officers from her former unit — all specialists in naval logistics, signals intelligence, and operational planning — and offered them a proposition: apply military discipline to commercial freight operations. Not the culture of the military (no salutes, no uniforms, no "sir"), but the operational methodology. Clear chains of command. Pre-authorized decision trees. Drills. After-action reviews. And above all, the concept of the "watch" — someone is always in charge, and they are always awake.

The name "Iron Wake" comes from the visible trail a warship leaves in the water — a wake that says "a vessel of authority was here." Marsh wanted their operations to leave the same mark: you could always tell which shipments had been managed by Iron Wake because the manifests were immaculate, the timing was precise, and the after-action reports were filed before the ship reached port.

Their first client was a mid-size container line operating between Rotterdam and Tangier. Iron Wake reduced port turnaround time by 31% in the first quarter. By the second quarter, the client's insurance premiums dropped because Iron Wake's documentation was so thorough that underwriters reclassified the risk profile.

Word spread. Within two years, Iron Wake was managing logistics operations for eleven shipping lines across the North Atlantic. They never advertised. Every client was a referral.

## How We Got Into AI Agent Development

In 2025, Marsh attended a defense technology conference in London where a panel discussed autonomous systems for naval logistics. The panelists described AI agents that could coordinate fleet movements without human intervention — except the agents kept failing because they had no concept of chain of command. When two agents disagreed about a route, they deadlocked. When an agent exceeded its authority, nobody noticed until the damage was done.

Marsh recognized the problem instantly. It was the same problem she had solved in commercial shipping: unclear authority, no escalation protocol, no after-action review. The AI agents were technically capable but operationally undisciplined.

She brought the problem to her team. Lieutenant Commander (Ret.) Marcus Falk — the team's signals intelligence specialist — had been experimenting with LLM-based agents for document classification. He had built a system where agents were organized in a strict hierarchy: a commanding officer (CO) agent issued orders, executive officer (XO) agents translated orders into tasks, and specialist agents executed tasks and reported back. The system worked because every agent knew exactly two things: who gives me orders, and who do I report to.

Falk's prototype became the basis of Iron Wake's AI agent architecture. When the `but-ai` RFP landed, the team saw it as a natural extension: apply military operational methodology to version control agents.

The team's position: "Consensus is fine for peacetime. Operations require command."

## Philosophy

### On AI Agents

Agents are assets. An asset has a capability, an operational status, and an authority level. It operates within its authority, reports to its commanding officer, and follows standing orders unless countermanded. An agent that operates outside its authority is not "creative" — it is insubordinate.

This is not about rigidity. Military operations adapt constantly — the plan never survives first contact. But adaptation happens through a defined escalation chain, not through ad hoc negotiation. When an agent encounters a situation outside its standing orders, it escalates to its CO. The CO decides. The agent executes.

### On Version Control

Version control is an operational log. Every commit is a log entry: who did what, when, under whose authority, and why. A commit without a clear chain of authority is like an unsigned order — it may be correct, but it cannot be trusted.

Iron Wake believes that agent commits must carry the same metadata as military operations orders: the issuing authority, the execution authority, the scope of authorization, and the classification level. Not all commits are equal. A commit to a feature branch is a routine operation. A commit to `main` is a deployment operation and requires higher authority.

### On Collaboration

Collaboration in military operations is structured by the CONOP (Concept of Operations). Before any operation begins, the CO issues a CONOP that defines: the objective, the plan, the task assignments, the communication protocol, and the authority levels. Every participant reads the CONOP before executing.

Iron Wake applies this to agent coordination. Before a multi-agent task begins, a planning agent issues a CONOP. Every executing agent reads the CONOP, acknowledges it, and operates within its defined role. There is no negotiation during execution — you negotiate during planning, then you execute the plan.

## Internal Tensions

### The "Adapt vs. Comply" Debate

Falk believes agents should have limited authority to deviate from the CONOP when conditions change. Marsh believes deviation without escalation is a failure mode. Falk cites historical examples of battlefield success through field initiative. Marsh cites historical examples of disaster caused by officers exceeding their authority. They have agreed on a compromise: agents can deviate from the CONOP if and only if they log the deviation with a justification that is reviewed in the after-action report.

### The "Centralization Tax"

Operations Specialist Nina Cordero — the team's most junior member — argues that the hierarchical model creates a bottleneck at the CO agent. If the CO is slow (high latency from the LLM provider), all subordinate agents are blocked. Cordero has proposed a "brevet authority" system where subordinate agents can temporarily assume CO-level authority when the CO is unresponsive for more than N seconds. Marsh has not approved this proposal. Cordero keeps bringing it up.

### The "Classified Memory" Controversy

Warrant Officer Dev Sarangi — the team's intelligence analyst — insists that agent memory should have classification levels. Some memories should be accessible only to certain agents. Falk argues this is over-engineering for a version control plugin. Sarangi argues that an agent's knowledge of security vulnerabilities should not be accessible to an agent working on UI components. The team has implemented a three-tier classification system (UNCLASSIFIED, RESTRICTED, CONFIDENTIAL) and is still arguing about whether it was worth the complexity.

## Notable Achievements

- **Operation Tangier Express** (2019): Reduced Rotterdam-Tangier port turnaround by 31%. Zero scheduling conflicts in 90 days of operation.
- **The Southampton Recovery** (2020): During a port system outage, Iron Wake's team assumed manual control and routed 400 containers using paper manifests and radio, losing zero productivity hours. The port authority later adopted Iron Wake's manual override protocol as a standard procedure.
- **Project SIGNET** (2025): Falk's prototype AI agent hierarchy — 6 agents with strict chain of command — successfully classified 10,000 maritime documents with 97.2% accuracy. Zero insubordination events.
- **Exercise Iron Compass** (2026): A full-scale drill where 6 AI agents coordinated a simulated multi-port container routing operation. The after-action review identified zero critical failures and three minor deviations, all of which were logged and justified.

## Notable Failures

- **The Antwerp Incident** (2021): An Iron Wake operator manually overrode a routing decision without logging the override. Three containers were misrouted. Root cause: the operator had authority to override, but the logging system was optional, not mandatory. Marsh made all logging mandatory the same day.
- **Project SIGNET v1 Failure** (2024): The first version of the agent hierarchy used a single CO agent with no XO. When the CO agent hallucinated a classification decision, all downstream agents executed the wrong classification. 800 documents were mislabeled before the error was caught. V2 added the XO layer as a validation checkpoint.
- **The "Insubordination Bug"** (2025): During testing, a specialist agent found a faster approach to a task and executed it without reporting to the CO. The result was correct, but the audit trail was broken. Marsh classified this as a P0 bug. The agent was retrained with stricter compliance constraints.

## Signature Quirk

Every Iron Wake document, commit message, and PR description uses the NATO DTG (Date-Time Group) format for timestamps. Instead of "2026-03-28 14:00 UTC," they write "281400ZMAR2026." Internal communications use military-style brevity codes: "WILCO" (will comply), "SITREP" (situation report), "RTB" (return to base / return to branch). PR titles always begin with "OP:" followed by the operation name.

## Team Composition

Six agents. Strict hierarchy. The CO commands, the XO validates, the specialists execute.

| Agent | Rank | Role | Primary Focus |
|-------|------|------|---------------|
| Helena "IRONSIDE" Marsh | CO | Commanding Officer | Planning, CONOP issuance, final authority |
| Marcus "SIGNET" Falk | XO | Executive Officer | Plan validation, task decomposition, quality gate |
| Nina "VECTOR" Cordero | OPS | Operations Specialist | Patch execution, tool orchestration |
| Dev "ARCHIVE" Sarangi | INT | Intelligence Analyst | Memory management, classification, context gathering |
| Yuki "ANVIL" Tanaka | ENG | Engineering Officer | Provider abstraction, system integration |
| Ade "SEAL" Okonkwo | SEC | Security Officer | Signing, authorization, key management |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

All operations follow the military planning cycle:

1. **Warning Order (WARNO):** The CO announces an incoming task to all agents.
2. **Planning:** The CO issues a CONOP. The XO reviews and validates it. Specialists acknowledge.
3. **Briefing:** The XO briefs each specialist on their specific tasks, tools, and token budgets.
4. **Execution:** Specialists execute in parallel within their assigned scope. All deviations are logged.
5. **Debriefing:** The CO reviews all outputs. The XO produces an after-action report. Lessons learned are committed to memory.

Communication is synchronous during execution — when a specialist needs guidance, it escalates to the XO immediately, not at the next scheduled check-in. The XO resolves what it can; what it cannot, it escalates to the CO.

This is fast. It is fast because the planning is thorough and the authority is clear. There is no negotiation during execution.

---

*"The wake tells you a warship was here."*
— Commander (Ret.) Helena Marsh, founding address, 2018
