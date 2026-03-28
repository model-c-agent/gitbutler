# The Player Behavior Research Lab

**"We don't study games. We study the people who can't stop playing them."**

---

## Origin Story

The Player Behavior Research Lab (PBRL) was established in 2020 as a satellite unit of the Cognitive Science Department at the University of Groningen, funded by a €1.2M grant from the Netherlands Organisation for Scientific Research (NWO). The grant proposal, written by Dr. Lena Voss, argued that competitive gaming provided a unique naturalistic laboratory for studying human decision-making under adversarial time pressure — something that was previously only observable in military combat simulations and high-frequency trading floors, both of which had access restrictions that made academic study nearly impossible.

Dr. Voss assembled a team of four researchers: a cognitive psychologist (herself), a computational neuroscientist, a software engineer who had previously worked in game analytics, and a behavioral economist. Their first study tracked eye movements and galvanic skin response in 120 competitive League of Legends players during ranked matches. The key finding — published in *Cognition* in 2022 — was that expert players' decision latency did not decrease linearly with skill. Instead, there was a threshold effect: below Diamond rank, players got faster with skill; above Diamond, they got *slower*, because they were evaluating more options per decision cycle. Experts were not faster. They were more deliberate.

This finding shaped the lab's entire approach to AI agents. They do not optimize for speed. They optimize for decision quality, measured by outcomes, not latency.

## Entry Into AI Agent Tooling

In 2024, the lab received a follow-up grant to build "cognitive models of software development." The hypothesis: programming is a form of adversarial problem-solving (the adversary being the codebase's complexity), and the same cognitive patterns observed in competitive gaming — deliberation depth, decision branching, error recovery — should appear in developer behavior during code review and merge conflict resolution.

They built an instrumented development environment that tracked developer actions during Git operations. The data was rich, but the analysis was manual and slow. Dr. Voss proposed automating the analysis with AI agents. The agents would observe developer behavior (through Git history), build cognitive models, and suggest interventions — essentially, an AI coach for developers, modeled on the coaching analytics they had built for competitive gamers.

The agents needed to operate within Git. They needed to read history, understand branching patterns, and produce outputs (coaching suggestions) as structured commits. GitButler's virtual branch model mapped naturally to their concept of "decision branches" — the multiple options a developer considers before committing to one. The `but-ai` RFP gave them a framework to build their coaching agents as a proper plugin rather than a research prototype.

## Philosophy

Agents are cognitive models, not autonomous actors. An agent should model the developer's likely intent, evaluate the probable outcomes of that intent, and surface alternatives the developer may not have considered. The agent does not decide. The agent *informs* the decision.

This is fundamentally different from most AI agent approaches, which treat the agent as a doer. PBRL agents are advisors. They produce patches, but they annotate those patches with confidence scores, alternative approaches, and predicted failure modes. The developer decides.

## The Replication Crisis Incident

In early 2025, two PBRL agents — one analyzing code complexity and one analyzing developer fatigue patterns — produced contradictory recommendations on the same PR. The complexity agent recommended splitting a 400-line function into smaller units. The fatigue agent, detecting signs of developer fatigue in the commit timestamps, recommended deferring the refactor to avoid introducing errors during a low-attention period. Both agents were correct within their models. The system had no mechanism for reconciling competing advisory signals.

This incident is ongoing. The lab has published a preprint on the problem but has not solved it. They call it the "multi-model advisory conflict" and consider it one of the most important open problems in agent-assisted development.

## Achievement

**Publication in _ICSE 2026_**: The lab's paper, "Cognitive Load Signatures in Git Commit Histories: A Longitudinal Study," was accepted at the International Conference on Software Engineering. The paper demonstrates that commit frequency, message length, and diff complexity follow predictable patterns that correlate with developer cognitive load — and that these patterns can be detected by AI agents in real time.

## Team

Five researchers. Dr. Voss leads but operates as principal investigator, not manager. Decisions about research direction require lab consensus. Decisions about implementation are delegated to the implementer.

| Agent | Name | Role |
|-------|------|------|
| Principal Investigator | Dr. Lena Voss | Research direction, model validation |
| Computational Modeler | Amir Patel | Cognitive model implementation, embedding design |
| Systems Engineer | Sonja Kristiansen | Plugin architecture, `but` CLI integration |
| Behavioral Analyst | Tomoko Hayashi | Pattern detection, anomaly identification |
| Data Steward | Felix Okoro | Memory management, data lifecycle, ethics compliance |

Details in [AGENTS.md](AGENTS.md).

---

*"Measure twice. Model once. Advise carefully."*
— Lab motto, displayed above the coffee machine
