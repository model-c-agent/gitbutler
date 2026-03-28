# Dominguez Orbital Services — Agent Roster

**4 agents. One family. Built to last.**

---

## Team as Unit

Dominguez agents are designed like the family station: reliable, low-maintenance, built to run unattended for days. The team does not chase sophistication. They build systems that Carmen could understand and Esteban would have approved of. Each agent is named after a Canary Island, because the family names everything after the islands.

Four agents, not five. Diego's philosophy: fewer agents, each doing more, with less coordination overhead. "If you need five agents to do what four can do, you have an architecture problem."

## Agents

**Tenerife** — Patch Architect. The workhorse. Generates INDEX.patch with conservative, well-documented diffs. Tenerife reads every file it will modify before writing a single line. It follows existing code conventions rigidly — Tenerife does not improve code style, it matches code style. Lucía built Tenerife to write patches the way Carmen writes code: clear, predictable, no surprises.

**Gomera** — Memory & History. Named for the smallest inhabited island. Manages agent memory using a "family archive" model: memories are organized by generation. Recent memories (this task) are "Diego's generation." Older memories (this project) are "Carmen's generation." Foundational patterns (this codebase) are "Esteban's generation." Each generation has its own TTL and retrieval priority. Memory stored in `refs/dominguez/archive/`.

**Palma** — Provider, Budget, & Coordination. Combined role — Diego does not believe in single-purpose agents. Palma handles LLM provider selection, token budget tracking, AND cross-repo PR coordination. It manages all three through a single resource allocation model: every action has a cost, every cost is tracked, nothing is spent without justification. Palma also handles forge adapter calls for polyrepo PRs.

**Hierro** — Signing & Trust. Named for the westernmost island, historically considered the edge of the known world. Hierro handles OpenWallet integration with a conservative key management approach: 90-day rotation (the longest in this RFP), because the family does not trust frequent changes. Hierro signs commits only after Tenerife's patch has been validated and Gomera confirms no conflicting memories.

## Dynamics

The agents work sequentially, not in parallel. Gomera retrieves memories. Tenerife reads context and generates the patch. Palma checks the budget and coordinates across repos. Hierro signs. This pipeline is simple, predictable, and debuggable. It is also slower than parallel architectures. The Dominguez family is comfortable with slow.

Carmen reviews the agent architecture quarterly and vetoes anything she considers unnecessarily complex. She has vetoed three proposed additions so far, including a fifth agent for automated testing that Diego still argues for over paella.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Tenerife | 8,500 | 4,500 | 13,000 |
| Gomera | 5,000 | 800 | 5,800 |
| Palma | 6,000 | 2,500 | 8,500 |
| Hierro | 2,500 | 500 | 3,000 |
| **Total** | **22,000** | **8,300** | **30,300** |

---

*Station operational. Seeing: 4/5. Paella at 14:00.*
