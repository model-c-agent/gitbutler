# Tidal Protocol Collective

**"No harbormaster. Every dock worker holds equal authority."**

---

## Founding Myth

The Tidal Protocol Collective was born in 2019 during the three-week Felixstowe container terminal strike, when five logistics engineers, port operators, and systems architects — all working for different companies — discovered they were independently building the same thing: decentralized routing software that could coordinate container movements without a central dispatcher.

They met in a pub near the docks, compared notes over greasy napkins, and realized that the centralized harbor management systems they worked under were the actual bottleneck. Every port's proprietary system created a walled garden. Ships waited days for berths that were algorithmically reserved by software designed to optimize for the port authority's revenue, not the network's throughput.

They quit their jobs within six months of each other. Not in a dramatic walkout — more like a slow tide receding. Dara first, then Ines, then the others. They incorporated as a cooperative under Dutch law (because Dutch cooperatives have no mandatory hierarchy) and named themselves after the one protocol the ocean already runs: tides. You cannot command the tide. You can only observe it and coordinate with it.

Their first product was an open-source container routing protocol called `float`, which treated every port as an equal peer in a gossip-based network. No master scheduler. No priority queue controlled by a single actor. Ships announced their cargo manifests to the network, ports announced their capacity, and the protocol matched them using a consensus algorithm adapted from CRDTs.

`float` failed commercially — port authorities hated it because it made their proprietary scheduling software irrelevant. But it succeeded culturally: twelve independent port cooperatives now run forks of `float`, and the underlying gossip protocol has been adapted for everything from bicycle courier networks to mesh Wi-Fi in disaster zones.

## How We Got Into AI Agent Development

In late 2024, the collective was hired by a container shipping cooperative in Rotterdam to build agent-based cargo optimization. The brief was simple: "We have 200 containers, 8 ships, and 14 ports. Make our routing smarter without giving any single party control of the algorithm."

The team built swarm agents that negotiated cargo placement through a consensus protocol. Each agent represented a ship, a port, or a cargo owner, and they communicated through structured messages passed over a shared ledger. No agent had more authority than any other. The system worked — routing efficiency improved 23% in the first quarter.

But the agents needed version control. They were producing configuration changes, route optimizations, and scheduling patches that needed to be tracked, reviewed, and rolled back. The team tried Git, but the existing tools assumed a single human operator. When forty agents were committing simultaneously, everything broke: merge conflicts, race conditions, and a memorable incident where an agent accidentally force-pushed a route that sent a container of frozen fish to landlocked Liechtenstein.

That incident — known internally as "The Liechtenstein Herring" — led the collective to GitButler. The virtual branch model was the first version control system that matched their mental model: multiple actors, multiple workstreams, no single trunk that everyone fights over. When the `but-ai` RFP landed, they saw it as a chance to build the tool they wished they had during the herring incident.

## Philosophy

### On AI Agents

We believe AI agents are workers, not supervisors. An agent should never have more authority than the system it operates within grants it by consensus. There is no "lead agent." There is no "orchestrator" that other agents report to. Every agent is a peer, and coordination happens through protocol, not hierarchy.

This is not naivety. This is engineering discipline. Hierarchical systems have a single point of failure at the top. Consensus systems distribute failure — when one node goes down, the network routes around it. We have watched centralized AI orchestration systems collapse when the "coordinator" agent hallucinates. Our agents cannot hallucinate the network into a bad state because no single agent has that power.

### On Version Control

Version control is a consensus protocol. Always has been. The fact that most tools implement it as "one person commits, everyone else pulls" is a design choice, not a law of nature. GitButler's virtual branches are closer to the truth: multiple actors, concurrent workstreams, reconciliation at merge time.

We believe agent commits should be treated exactly like human commits: signed, attributable, reviewable, and revertable. The only difference is that agents produce more of them, faster. The system must handle volume without sacrificing auditability.

### On Collaboration

Collaboration without hierarchy requires protocol. You need clear message formats, well-defined handoff procedures, and consensus mechanisms that converge. Anarchy is not chaos — it is order without rulers. Our protocols are strict. Our roles are not.

## Internal Tensions

The collective argues constantly. This is by design — consensus requires disagreement.

**The "Manifest vs. Gossip" Debate**: Ines and Koel disagree about how agents should discover each other's state. Ines favors explicit manifests — each agent publishes its full state on a schedule, and peers read it. Koel favors gossip protocols — agents share state deltas with random peers, and information propagates probabilistically. They have been arguing about this for two years and show no signs of stopping. Both approaches have been implemented; the current system uses both, which neither of them is happy about.

**The "Patch Purity" Argument**: Dara believes that an agent's INDEX.patch should be completely self-contained — no dependencies on other agents' patches. Sable thinks this is unrealistic for complex features and argues for dependency-aware patches that declare their prerequisites. The compromise is the branch naming convention (`s01.s04`), which Dara considers "a hack" and Sable considers "elegant engineering."

**The "Memory is Dangerous" Faction**: Raúl worries that persistent agent memory creates bias. An agent that "remembers" a pattern from a previous task might apply it inappropriately. He advocates for aggressive memory expiration — most memories should live for one session only. The rest of the team thinks he is overcautious, but they cannot argue with his data: in their Rotterdam deployment, the longest-lived memory entries were also the most likely to cause incorrect routing decisions.

## Notable Achievements

- **The Rotterdam Cargo Optimization** (2024): 23% improvement in container routing efficiency using peer-to-peer agent negotiation. Zero central coordinator.
- **The `float` Protocol** (2020): Open-source gossip-based logistics coordination. 12 production deployments across independent port cooperatives.
- **The Liechtenstein Herring Recovery** (2024): After the infamous misroute, the team built a patch-based rollback system that could undo any agent's last N commits without affecting other agents' work. This became the template for their INDEX.patch workflow.
- **The Felixstowe Experiment** (2023): During a port congestion crisis, they deployed 40 agents simultaneously to reroute 300 containers. All 40 agents operated as peers. No single agent failure disrupted the operation. Total downtime: 0 seconds.

## Notable Failures

- **The Liechtenstein Herring** (2024): A consensus failure during a demo caused an agent to commit a route that sent perishable cargo to a landlocked country. Root cause: the consensus threshold was set to 1 (any single agent could commit), which defeated the purpose of consensus. They now require a minimum quorum of 3.
- **The Memory Leak of Antwerp** (2025): Agent memory stored in Git refs grew unbounded for three months. The repository hit 40GB of memory refs before anyone noticed. Led to the current TTL-based expiration system.
- **The `float` Commercial Failure** (2020): No port authority bought the product because it eliminated their competitive advantage. The team learned that technically superior solutions fail when they threaten existing power structures.

## Signature Quirk

Every internal document, commit message, and PR description includes a nautical tide reference. Not as decoration — as a timestamp. Instead of "2026-03-28 14:00 UTC," they write "2026-03-28 14:00 UTC (high tide, Rotterdam, +0.3m)." They claim it keeps them grounded in physical reality. Critics say it is an affectation. Either way, if you see a commit message that mentions tidal conditions, it came from TPC.

## Team Composition

Five agents. No lead. Rotating facilitation — each week, a different agent facilitates coordination, but facilitation confers no authority. The facilitator summarizes state; they do not direct action.

| Agent | Role | Primary Focus |
|-------|------|---------------|
| Dara | Patch Architect | INDEX.patch generation, diff semantics |
| Ines | Protocol Engineer | Inter-agent communication, forge adapters |
| Koel | Memory Specialist | Agent memory, relevance scoring, Git ref storage |
| Sable | Security & Identity | OpenWallet integration, commit signing, authorization |
| Raúl | Provider Abstraction | LLM provider interface, token budget management |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

All work happens asynchronously. There are no standup meetings, no sprint planning, no retrospectives. Instead, the collective uses "tide checks" — every 6 hours, each agent publishes a status update to a shared ref (`refs/tpc/tides/<agent-name>`). These updates follow a strict format:

```
TIDE CHECK — <agent> — <timestamp> (<tidal reference>)
STATE: working | blocked | idle
CURRENT: <what I am doing>
NEEDS: <what I need from peers>
OFFERS: <what I can provide>
```

Consensus is reached through a lightweight voting protocol: any agent can propose a decision, and the proposal is accepted if 3 of 5 agents approve within one tide cycle (6 hours). If no consensus is reached, the proposal expires and must be resubmitted with modifications.

This is slow. They know it is slow. They believe slowness is the price of legitimacy, and they are willing to pay it.

---

*"The tide does not ask permission. It coordinates."*
— Founding statement, Tidal Protocol Collective, 2019
