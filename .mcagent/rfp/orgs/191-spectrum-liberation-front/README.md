# Spectrum Liberation Front

**"The airwaves belong to everyone. The infrastructure belongs to us."**

---

## Manifesto (Abridged)

The Spectrum Liberation Front (SLF) is an anarchist collective dedicated to building community-owned mesh networks that provide free internet access to underserved neighborhoods. We operate in eleven cities across four countries. We have no CEO, no board of directors, no venture capital, and no interest in acquiring any of these things.

SLF was founded in 2020 during the pandemic, when three neighborhoods in Detroit lost their only ISP because the company decided low-income areas were not profitable enough to maintain. Fourteen people — network engineers, electricians, a librarian, two social workers, and a retired cable installer — built a mesh network from surplus rooftop antennas, Raspberry Pis, and open-source firmware. The network covered 340 households. It cost $4,200 in hardware, donated by a local credit union. It is still running.

The Detroit mesh became a template. SLF published the build instructions as a Git repository — hardware specs, firmware configs, antenna placement guides, channel allocation plans. Other communities forked it. Oakland, Baltimore, Thessaloniki, Medellin, Cape Town. Each fork adapts the template to local conditions: different RF environments, different regulatory regimes, different community governance structures.

We maintain the template. We provide technical support. We accept no money from corporations, governments, or any entity that profits from spectrum scarcity. Our operating budget comes from mutual aid contributions, typically $2,000-5,000/month depending on the season.

## Why AI Agents

Managing eleven mesh networks across four countries is a coordination problem. Each network has its own repository with node configurations, channel allocations, and maintenance logs. When one network discovers an optimization (e.g., a better antenna angle for urban canyon propagation), that knowledge should propagate to other networks. Currently, this propagation happens through Matrix messages and occasional video calls. It is slow. It is lossy. Knowledge gets stuck in one network when it could benefit all.

We started building agents in 2025 to automate knowledge propagation. An agent monitors each network's repository, identifies configuration changes that improved performance, and proposes those changes as patches to other network repositories. The agent does not apply changes — it proposes them. Each network's maintainers decide whether to accept.

The `but-ai` RFP aligns with our needs: agent-generated patches, cross-repo coordination, and memory systems that preserve institutional knowledge. We particularly value the forge-agnostic design — our networks use Forgejo, GitHub, and Gitea. We refuse to standardize on a single platform because monoculture is the enemy of resilience.

## Philosophy

Infrastructure should be owned by the people who depend on it. This applies to networks, to software, and to AI agents. Our agents are open-source. Their models run locally. Their memory is stored in repositories we control. No cloud dependency. No vendor lock-in. No data leaving the community without consent.

We distrust optimization. Optimized systems are fragile — they have no slack, no margin for surprise. Our mesh networks are deliberately over-provisioned. We prefer 60% utilization with room to grow over 95% utilization with no room to breathe. Our agents follow the same principle: they use less of their token budget than they could, because the reserve is the safety margin.

## The Oakland Channel Conflict

In July 2025, an SLF agent proposed a channel reallocation in the Oakland mesh to reduce interference from a newly installed corporate 5G small cell. The proposal was technically correct but socially disastrous: the reallocation would have moved three nodes to a channel that a neighboring community network (not part of SLF) was already using. The agent did not know about the neighboring network because it was not in any SLF repository.

The incident taught us that agents cannot optimize for a single network in isolation. Mesh networks exist in a social context. The neighboring network's channel usage was common knowledge among Oakland maintainers but invisible to the agent. We now require all agents to include a `SOCIAL_CONTEXT` check — a prompt to the local maintainers: "Are there factors outside this repository that I should know about?"

## Achievement

**11 networks, 4 countries, zero downtime in 2025**: Across all SLF-supported mesh networks, cumulative downtime in 2025 was zero. Individual nodes went down (hardware failures, weather, one memorable incident involving a nesting pigeon), but no network lost coverage because the mesh topology routed around failures. The agents contributed by identifying node degradation patterns before failure and proposing preemptive maintenance patches.

## Collective Members (Core Maintainers)

| Handle | Focus |
|--------|-------|
| sparks | Network architecture, patch generation |
| ground_loop | RF engineering, interference analysis |
| meshkin | Cross-network coordination, forge adapters |
| libre_wave | Security, signing, privacy |
| node_zero | Memory systems, knowledge propagation |

Details in [AGENTS.md](AGENTS.md).

---

*"Another node is possible."*
