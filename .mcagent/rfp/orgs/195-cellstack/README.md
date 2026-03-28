# CellStack

**"One box. Every protocol. No trucks."**

---

## The Company

CellStack is a startup building software-defined small cells that can switch between 4G LTE, 5G NR, and satellite backhaul on the fly. Founded in 2023 by two former Ericsson engineers (Maya Chen, CEO, and Raj Anand, CTO) who watched their employer spend $4.2 billion acquiring a company to solve a problem that software could solve for $4.2 million.

The problem: rural and underserved areas need wireless coverage, but deploying dedicated infrastructure for each protocol generation is economically unviable. A traditional small cell supports one technology (4G or 5G, not both). When the network operator upgrades, the old hardware becomes e-waste. CellStack's small cell is a software-defined radio that supports multiple protocols simultaneously and can shift capacity between them based on demand. During peak hours, 80% of the radio's capacity serves 5G. At night, it shifts to 4G for IoT devices that do not need 5G speeds. When terrestrial backhaul fails (common in rural areas), it switches to satellite.

The company has $3.6M in seed funding from Lux Capital and two pilot deployments: one with a rural electric cooperative in Montana and one with a maritime operator in Norway. The Montana deployment serves 340 households and a grain elevator. The Norway deployment provides coverage for ferry routes in the fjords.

We are seven people. We work out of Maya's garage in San Jose because office rent in the Bay Area is a waste of seed funding. The garage has excellent Wi-Fi (obviously) and a whiteboard wall that is 80% full.

## Why AI Agents

Small cell configuration is a nightmare. Each deployment site has unique RF characteristics, unique traffic patterns, unique backhaul constraints. Configuring a CellStack unit requires balancing dozens of parameters: transmit power, protocol allocation ratios, handover thresholds, satellite failover triggers, interference mitigation settings. A trained RF engineer can configure a unit in about four hours. We do not have enough RF engineers for our pilot deployments, let alone for the 500-unit deployment we are pitching to a Tier 1 operator.

We started building configuration agents in late 2024. The agents analyze site survey data, traffic forecasts, and RF measurements, then generate unit configurations as structured parameter files. The configurations are stored in Git — one repository per deployment, one branch per unit, configuration history tracked by commit. When an agent proposes a configuration change (e.g., increasing 5G allocation during peak hours), it generates an INDEX.patch against the current configuration.

The agents work. The problem is coordination. When one unit's configuration changes, it affects neighboring units (interference, handover behavior, capacity allocation). Cross-unit coordination was ad-hoc until we started using GitButler. The virtual branch model lets us stage configuration changes across multiple units and merge them atomically.

## Philosophy

Ship hardware, iterate software. The small cell is a commodity radio. The value is in the configuration — the software that tells the radio what to do. We believe the telecom industry's hardware obsession is its biggest cost driver. CellStack's competitive advantage is that our "hardware upgrade" is a `git pull`.

Agents should be aggressive proposers and conservative appliers. An agent should generate many configuration proposals, ranked by expected performance improvement, and a human operator should approve before any change reaches a live unit. A misconfigured small cell does not just lose data — it can interfere with emergency services. We do not deploy agent-generated configurations without human review.

## The Montana Blizzard

In January 2026, a blizzard knocked out the terrestrial backhaul link to the Montana deployment. The satellite failover activated correctly. But the failover agent also proposed a configuration change — reducing 5G capacity to conserve satellite bandwidth — and the human operator was unreachable (also affected by the blizzard). The configuration change sat in review for 14 hours.

During those 14 hours, the units continued operating on the default satellite configuration, which allocated bandwidth equally across 340 households. This was fine for basic connectivity but insufficient for the grain elevator's automated systems, which needed a minimum bandwidth guarantee.

The incident taught us two things: (1) critical configuration changes need an auto-approve pathway for emergency scenarios, and (2) the auto-approve pathway needs strict constraints (only pre-approved change types, only within defined parameter ranges). We are still implementing this.

## Achievement

**Norway pilot: 99.7% uptime over 6 months**: The ferry route deployment maintained 99.7% coverage uptime, including seamless handover between terrestrial and satellite backhaul as ferries moved through fjords. The maritime operator has requested a 20-unit expansion.

## Team

| Member | Title | Role |
|--------|-------|------|
| Maya Chen | CEO | Product direction, operator relationships |
| Raj Anand | CTO | Architecture, protocol stack |
| Priya Sharma | RF Engineer | Configuration generation, propagation models |
| Luis Moreno | Backend Engineer | Plugin development, forge integration |
| Kira Johansson | DevOps | Deployment, memory, infrastructure |

Details in [AGENTS.md](AGENTS.md).

---

*"The best cell tower is the one you never have to visit."*
— Maya Chen, investor pitch, Q3 2025
