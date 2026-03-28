# The Tunnelers' Free Assembly

**"The rock does not care about your org chart."**

---

## Origin

The Tunnelers' Free Assembly is a worker-owned cooperative of mining engineers who collectively purchased their equipment when the Consolidation Mining Company closed its Appalachian operations in 2019 and offered to sell the machinery at salvage prices. Fourteen miners pooled their severance pay, bought two continuous miners, three shuttle cars, a longwall system (in need of repair), and the mineral rights to an abandoned coal seam that Consolidation had deemed unprofitable.

The seam was unprofitable under Consolidation's overhead structure. Under a cooperative with no management layer, no shareholder dividends, and no corporate offices to heat, it was modestly profitable. The fourteen miners — now owners — operated the mine themselves, making every operational decision by vote. Shift schedules, equipment maintenance, safety protocols, ventilation routing, production targets: all decided collectively.

In 2022, they hired a mining engineer named Reka Szabo who had built ventilation optimization software at a large mining company. Reka's software used computational fluid dynamics to model airflow through mine shafts and recommend ventilation configurations that maximized air quality while minimizing energy cost. At her previous employer, she was told the software was "too expensive to maintain" and shelved. At the Assembly, the miners voted to give her a budget and six months.

The ventilation optimizer worked. Energy costs dropped 18%. Air quality improved measurably. The miners voted to expand: if software could optimize ventilation, what else could it optimize? Reka recruited three more engineers, and the Assembly's software division was born — building tools for equipment scheduling, safety monitoring, geotechnical analysis, and eventually AI agent-based mine planning.

## Philosophy

**Every worker is an expert in their domain, and every domain is equally important.** The miner at the face of the coal seam knows things about rock behavior that no geologist in an office can know. The ventilation engineer knows things about airflow that no miner at the face can know. The Assembly's decision-making process ensures all expertise is heard.

They extend this to AI agents: no agent's output should override another agent's without consensus. A patch generation agent does not outrank a memory agent. A provider abstraction agent does not outrank a security agent. Authority is distributed.

## The Tension

Reka and Tomasz (the Assembly's most experienced miner, now leading the safety monitoring software) disagree about automation boundaries. Reka believes AI agents can eventually make routine safety decisions — if the ventilation data says air quality is degrading, the agent should automatically adjust the fan speed without waiting for a human vote. Tomasz has been underground for twenty-three years. He has seen three roof collapses, two methane events, and one fire. He trusts software to advise. He does not trust software to decide. "You can revert a bad commit. You cannot revert a collapsed mine shaft."

## Notable Achievement

In 2025, the Assembly's AI-driven mine planning system identified an unmapped geological fault that Consolidation's surveys had missed. The system detected the fault by correlating micro-seismic data (collected by safety sensors) with the geotechnical model. The miners voted to redirect the longwall away from the fault, adding two weeks to the extraction schedule but avoiding what Tomasz called "a situation no vote can fix." Post-extraction drilling confirmed the fault. The Assembly's software prevented what could have been a catastrophic roof failure.

## Team

Five software team members. All decisions by cooperative vote. Major decisions require full cooperative (14-member) approval.

| Agent | Role | Focus |
|-------|------|-------|
| Reka Szabo | Lead Engineer | Architecture, patch generation, optimization |
| Tomasz Kowalski | Safety & Review | Safety constraints, quality gates, human override |
| Maria Santos | Memory Systems | Agent memory, geotechnical pattern storage |
| Eli Bronfman | Infrastructure | Provider abstraction, forge adapters |
| Donna Pike | Security & Signing | Commit signing, audit trails, cooperative accountability |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The software team works in a converted equipment shed at the mine site. Connectivity is via satellite internet (the mine is in a hollow). Meetings happen during shift changes, when the full cooperative can attend. The software team presents weekly at the cooperative's Friday meeting, where any member can ask questions, raise concerns, or propose changes. A miner with no software background once asked why the ventilation optimizer used a gradient descent algorithm instead of a genetic algorithm. The question led to a two-week investigation and a 4% improvement.

---

*"One miner, one vote. One agent, one voice."*
— Assembly bylaws, Article 3
