# Signal Dominance Group

**"If the link goes down, someone doesn't come home."**

---

## Background

Signal Dominance Group (SDG) is a defense communications contractor headquartered in Huntsville, Alabama. We build hardened communications infrastructure for military and government clients. Redundant satellite uplinks. EMP-resistant field relay stations. Mesh networks that continue operating when 60% of nodes are destroyed. Our systems carry traffic that, if interrupted, has consequences measured in human lives, not service-level agreements.

SDG was founded in 2018 by Colonel (ret.) Arthur Briggs and Dr. Nadia Sorokin, who met while both were assigned to the Defense Information Systems Agency (DISA). Briggs ran tactical communications programs. Sorokin designed survivable network architectures for nuclear command and control. They left government service together because they believed the private sector could build more resilient communications systems if freed from the acquisition bureaucracy — an assessment that proved half-correct. They build better systems. The bureaucracy followed them.

The company has 34 employees, $12M annual revenue, and three active contracts with the Department of Defense. All work is performed in the United States by U.S. persons. Our facility has a SCIF. Our coffee is bad.

## Why AI Agents

In 2025, DISA issued a request for prototype: AI-assisted network configuration management for tactical communications systems. The requirement: agents that can analyze network topology, identify vulnerabilities (single points of failure, bandwidth bottlenecks, coverage gaps), and propose configuration changes as auditable artifacts. Every change must be traceable to a requirement, signed by an authorized operator, and reversible without service interruption.

SDG won the prototype contract. We built the agents using a custom framework. The framework worked but was brittle — merging agent-proposed changes with human operator changes produced conflicts that took longer to resolve than the agents saved. We evaluated commercial tools and selected GitButler because its virtual branch model matched our operational concept: multiple agents and multiple operators working on the same network configuration simultaneously, with explicit merge points.

The `but-ai` plugin would formalize our agent framework into a standard tool that could be reused across contracts. The DoD's interest in reproducible AI tooling makes this commercially valuable.

## Philosophy

Redundancy is not waste. Redundancy is survival. Every system we build has at least two independent paths for every critical function. Our agent architecture follows the same principle: every agent has a defined backup. If the primary patch generator fails, the backup generates from the same context. If the signing agent fails, a backup signing key is pre-authorized.

We do not trust any single component — hardware, software, or human. Trust is distributed across the system. A signed commit requires the generating agent's signature, the reviewing operator's signature, and the configuration management system's counter-signature. Three independent attestations. If any one is missing, the commit is rejected.

## The Fort Bragg Configuration Incident

In December 2025, during a field exercise, an agent proposed a routing change that would have consolidated two redundant satellite links into a single higher-bandwidth link. The change was technically optimal — more bandwidth, lower latency. It was operationally catastrophic — it eliminated the redundancy that kept communications alive when one satellite was jammed.

The human operator caught the error during review. The post-incident analysis identified the root cause: the agent's optimization function weighted bandwidth and latency but did not weight redundancy. Briggs added a fourth optimization constraint: minimum redundancy level. No agent can propose a change that reduces redundancy below the operational minimum, regardless of the performance gain.

## Achievement

**DISA Prototype Acceptance**: In February 2026, DISA accepted SDG's AI-assisted configuration management prototype with no major deficiencies. The acceptance report specifically cited the auditability of the agent-generated configuration changes as exceeding expectations. The prototype is being evaluated for transition to a program of record.

## Personnel

| Member | Title | Role |
|--------|-------|------|
| Col. (ret.) Arthur Briggs | CEO | Requirements, operational oversight |
| Dr. Nadia Sorokin | CTO | Architecture, redundancy engineering |
| Staff Sgt. (ret.) Devon Cole | Lead Engineer | Patch generation, agent framework |
| Lt. (ret.) Sarah Ikeda | Security Lead | Signing, key management, CMMC compliance |
| Dr. James Okafor | Network Analyst | Memory systems, topology analysis |

Details in [AGENTS.md](AGENTS.md).

---

*"Two is one. One is none."*
— SDG design principle, borrowed from military logistics
