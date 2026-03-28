# CropOS

**"One API to rule tractors, drones, sensors, and irrigation. And now, agents."**

---

## Origin Story

CropOS was founded in January 2024 by three people who had no business starting a company together: a precision agriculture engineer who had been fired from John Deere for publicly advocating right-to-repair, a DevOps engineer who had never seen a farm, and a seed geneticist who was tired of watching her research rot in university servers because no farmer could access it.

Maya Okafor (the engineer) had spent eight years building proprietary firmware for combine harvesters. She knew the problem: every piece of farm equipment speaks a different protocol. John Deere speaks CAN bus with proprietary extensions. Kubota speaks ISOBUS. The drone talks MAVLink. The soil sensors speak Zigbee. The irrigation controller speaks Modbus. Getting a drone to talk to a tractor requires three adapters, a cloud subscription, and a blood sacrifice. Maya wanted a single API layer that unified all of them — an operating system for the farm.

Raj Patel (the DevOps engineer) had never touched a tractor but had spent a decade building infrastructure abstractions. To him, a tractor was a compute node, a drone was a monitoring agent, and a soil sensor was a metrics endpoint. He could not understand why farms did not have the equivalent of Kubernetes. When Maya pitched him the idea at a Y Combinator networking event, he said "so it is like Terraform but for dirt" and she knew she had found her co-founder.

Dr. Suki Tanabe (the seed geneticist) joined three months later. She was frustrated that her lab's drought-resistant seed varieties were being tested on two experimental plots when they should be deployed on ten thousand farms. The bottleneck was not the science — it was the software. Farmers could not integrate new seed data into their planting systems because every system was a silo. CropOS's unified API would solve that.

They incorporated in Delaware, rented a co-working space in Oakland, and shipped their first prototype in four months. That prototype — a Raspberry Pi running a Rust binary that could talk to a John Deere combine, a DJI drone, and a Davis weather station through a single CLI — won them a $2.3M seed round from Andreessen Horowitz's Bio fund.

## The Turn Toward Agents

CropOS hit a wall in Q3 2025. The unified API worked, but farmers did not want to write scripts. They wanted to say "plant the north field with drought-resistant wheat, monitor soil moisture, and irrigate when it drops below 40%" and have the system figure out the rest. Every customer conversation ended the same way: "Can it just do it for me?"

Raj built a quick GPT-4o integration. It worked for simple tasks but collapsed under complexity: the LLM would hallucinate equipment capabilities, forget the field layout mid-conversation, and occasionally try to irrigate fields that were already flooded. The problem was not the LLM — it was the lack of persistent memory, structured tool access, and coordination between multiple concurrent operations.

Maya found the GitButler RFP while researching agent frameworks. She recognized the pattern immediately: `but-ai`'s problem (agents operating on a codebase) was structurally identical to CropOS's problem (agents operating on a farm). In both cases, you need structured tool access, persistent memory, patch-based changes (you do not irrigate directly — you propose an irrigation schedule and the controller applies it), and coordination across multiple independent systems.

CropOS is responding to this RFP because they believe their farm-to-code translation is not just a metaphor. It is an architecture.

## Philosophy

CropOS runs on three principles:

1. **Ship or compost.** If it is not deployed within 30 days, it is dead. The startup graveyard is full of beautiful architectures that never shipped. CropOS ships fast, measures, and iterates. Perfection is the enemy of planting season.

2. **Composting is not waste.** Failed experiments, abandoned prototypes, and deprecated code are not garbage. They are compost — raw material that decomposes into reusable primitives. CropOS maintains a "compost heap" of abandoned code that is actively mined for useful patterns. Their soil-layer memory approach (see PROPOSAL.md) is a direct extension of this philosophy.

3. **The farmer is always right.** (Even when the farmer is an AI agent.) The end user knows their field better than any model. CropOS tools are opinionated about protocol but not about strategy. The API tells you how to talk to the tractor, not where to drive it.

## Internal Tensions

CropOS has three people. They argue about everything.

Maya wants to build infrastructure that lasts. She comes from hardware, where you cannot push a hotfix to a combine harvester in the middle of a harvest. She designs for durability and hates anything that feels fragile.

Raj wants to move fast. He comes from DevOps, where you deploy twenty times a day and roll back when things break. He designs for velocity and hates anything that slows deployment.

Suki wants accuracy. She comes from science, where a wrong number in a seed yield prediction can bankrupt a farmer. She designs for correctness and hates anything that trades precision for speed.

These tensions are the engine of the company. Every design decision is a three-way negotiation between durability, velocity, and correctness. The results are surprisingly good.

## Achievements

- **CropOS Core** (2024): The unified farm equipment API. Supports 14 equipment families across 6 protocols. 340 farm deployments. $2.3M seed funding.
- **Harvest Mode** (2025): A batch scheduling system that coordinates multiple equipment types for a single harvest operation. Reduced harvest coordination time by 60% in pilot farms.
- **SoilQL** (2025): A query language for agricultural sensor networks. Think PromQL but for dirt. Open-sourced, 1,800 GitHub stars.
- **The Compost Heap** (2025): An internal repository of "decomposed" code — patterns extracted from abandoned projects. Currently contains 47 reusable primitives. CropOS credits 30% of their shipping speed to this repository.

## Failures

- **The GPT Integration** (2025): The naive LLM integration that hallucinated equipment capabilities. Abandoned after three weeks but the failure directly motivated this RFP response. Several patterns from it are in the compost heap.
- **Real-Time Field Monitoring** (2025): Attempted streaming sensor data through the API. The bandwidth requirements exceeded rural connectivity. Pivoted to periodic batch uploads. Raj is still bitter about this.
- **Automated Seed Selection** (2025): Suki built an ML model for recommending seed varieties. It was 91% accurate but the 9% failure rate included a case where it recommended a flood-intolerant variety for a river basin. Pulled from production within a week. The lesson: accuracy matters more than coverage.

## Signature Quirk

Every CropOS document ends with a "soil composition" footer — a three-part breakdown of the document's content into durability (Maya's influence), velocity (Raj's influence), and correctness (Suki's influence), expressed as percentages. Their README is 40% durability / 35% velocity / 25% correctness. Their PROPOSAL.md is 30% durability / 30% velocity / 40% correctness. Their AGENTS.md is 50% durability / 25% velocity / 25% correctness.

They started this as a joke during their seed round pitch and it stuck.

## Team Composition

| Name | Role | Background | Joined |
|------|------|------------|--------|
| **Maya Okafor** | CEO / Systems Architect | Precision agriculture engineering, 8 years at John Deere, Rust/embedded | Co-founder (Jan 2024) |
| **Raj Patel** | CTO / Infrastructure Lead | DevOps, SRE, 10 years at Datadog and Cloudflare | Co-founder (Jan 2024) |
| **Dr. Suki Tanabe** | Chief Scientist / Data Lead | Seed genetics, ML, agricultural data science, PhD UC Davis | Co-founder (Apr 2024) |

Three people. No managers. No board of directors (yet — the VCs are patient). Decisions are made in a shared Signal group, usually between 11 PM and 2 AM Pacific time. They work from Maya's apartment, Raj's co-working space, and Suki's lab, rarely in the same room.

---

*Soil composition: 40% durability / 35% velocity / 25% correctness*
*Growing season: Year 2*
*Field status: Planting*
