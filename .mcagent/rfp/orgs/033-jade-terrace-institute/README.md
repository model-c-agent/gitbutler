# The Jade Terrace Institute

**"Knowledge, like water, flows downhill. Our job is to build the terraces that catch it."**

---

## Origin Story

The Jade Terrace Institute was founded in 2016 as a joint venture between Zhejiang University's College of Biosystems Engineering and the International Rice Research Institute (IRRI) in Los Banos, Philippines. The founding director, Dr. Mei-Ling Chen, had spent fifteen years studying the Honghe Hani Rice Terraces in Yunnan Province — a 1,300-year-old irrigation system that remains, in her words, "the most sophisticated distributed computing network ever built by hand."

The terraces fascinated her not because they were old but because they were *correct*. Water enters at the top, flows through hundreds of paddies at precisely calibrated rates, and arrives at the bottom having nourished every level without waste. Each terrace filters sediment, controls temperature, and regulates nutrient concentration for the terrace below it. The system requires no pumps, no sensors, and no central controller. It works because the topology is right.

Dr. Chen believed that this topological principle — intelligence encoded in structure rather than in a central brain — could transform agricultural automation. She was tired of precision agriculture systems that required a $200,000 tractor, a cloud subscription, and a PhD to operate. She wanted something that worked like a rice terrace: locally intelligent, globally coherent, and maintainable by the people who actually farm.

The Institute's first project was a robotic rice transplanter that navigated terraced paddies autonomously. It took four years and seventeen prototypes. The eighteenth worked. It has since been deployed in 340 terrace systems across Southeast Asia, and the paper describing its navigation algorithm ("Gravitational Path Planning for Non-Planar Agricultural Surfaces") has been cited 1,200 times.

## The Computational Turn

The Institute's entry into software agent development was, like the terraces themselves, a matter of gravity. In 2023, they were coordinating a fleet of 50 autonomous transplanters across a 200-hectare terrace complex in Vietnam. The coordination software was a disaster — a monolithic cloud application that required constant connectivity, crashed when the satellite link dropped (which happened daily), and could not handle the fact that terraces at different elevations had different planting schedules.

Dr. Anh Nguyen, the Institute's software lead, proposed replacing the monolithic coordinator with a terraced architecture: a hierarchy of autonomous agents, each responsible for a specific elevation band, passing context downhill like water. The top-level agent understood the whole season plan. Mid-level agents translated that into weekly schedules for their terrace band. Bottom-level agents handled individual paddies. Information flowed downhill by default and uphill only when something went wrong — just like water flowing down and a farmer walking up to check a blocked channel.

The architecture worked. The fleet operated for 47 days without cloud connectivity during a typhoon season. Every transplanter completed its schedule. The paper ("Terraced Multi-Agent Coordination for Disconnected Agricultural Fleets") was published in Autonomous Agents and Multi-Agent Systems and is now assigned reading in three graduate programs.

That paper caught the attention of the GitButler team, who saw in it a model for AI agent coordination that did not require a central orchestrator running at all times.

## Philosophy

The Institute operates on three principles borrowed from terrace agriculture:

1. **Gravity is free.** Information should flow from general to specific without effort. A well-structured system does not need to push context to agents — agents at the right level in the hierarchy naturally receive the context they need, filtered and refined by the levels above them.

2. **Every terrace is a filter.** Each level of abstraction removes noise and adds clarity. A top-level task description ("implement authentication") becomes a mid-level specification ("add JWT middleware to the Express app") becomes a bottom-level instruction ("create file src/middleware/auth.ts with these specific function signatures"). Each terrace catches what it needs and lets the rest flow through.

3. **The farmer walks uphill.** Exceptions flow up. Normal operations flow down. An agent that encounters an error does not broadcast it to the whole system. It reports to its immediate upstream terrace. If that terrace cannot resolve it, it escalates further. This contains blast radius naturally.

## Internal Tensions

The Institute is an academic lab, and academic labs have politics. The primary tension is between Dr. Chen's insistence on publishing everything and Dr. Nguyen's desire to ship products. Chen believes that unpublished work does not exist. Nguyen believes that unshipped software does not matter. They argue about this weekly and have developed a compromise: every project must produce both a paper and a deployable artifact. If you cannot explain it in a paper, you do not understand it. If you cannot ship it, it does not work.

The secondary tension is generational. The four postdoctoral researchers (Dr. Priya Sharma, Dr. Kenji Tanaka, Dr. Fatou Diallo, and Riku Hoshino, a doctoral candidate) have different relationships with AI. Sharma distrusts LLMs and insists on formal verification. Tanaka embraces them enthusiastically and has to be restrained from giving agents too much autonomy. Diallo is the pragmatist who brokers compromises. Hoshino is the youngest and the most technically skilled, but struggles to articulate his design decisions in writing — a serious liability in an institution that publishes everything.

## Achievements

- **Gravitational Path Planner** (2020): Autonomous navigation for terraced agricultural robots. 1,200 citations. Deployed in 340 terrace systems.
- **Terraced Fleet Coordination** (2024): 50-robot fleet operating for 47 days without cloud connectivity. Published in AAMAS.
- **PaddyNet** (2025): A mesh networking protocol for agricultural IoT that routes messages using elevation as a natural priority — sensors at higher elevations have higher routing priority because their data affects more downstream paddies. Open-sourced, 890 GitHub stars.
- **IrrigationQL** (2025): A query language for agricultural sensor networks inspired by SQL but with spatial and temporal operators native to terrace topologies. Used internally, not yet published (Chen and Nguyen are arguing about whether it is ready).

## Failures

- **The Cloud Coordinator** (2023): The monolithic fleet management system that failed during typhoon season. This failure catalyzed the terraced architecture that defines the Institute's current approach.
- **Autonomous Harvest Timing** (2024): An ML model that predicted optimal harvest dates. It was 94% accurate in test but failed catastrophically in production because it did not account for localized microclimate variations between adjacent terraces. The lesson: global models fail in terraced systems. Every terrace is local.
- **Real-Time Terrace Monitoring** (2025): Attempted to stream video from 200 paddies simultaneously. The bandwidth requirements exceeded what rural infrastructure could support. Abandoned in favor of periodic snapshot-based monitoring. Nguyen still insists it would have worked with better compression.

## Signature Quirk

Every document, commit message, and API response at the Jade Terrace Institute includes an elevation marker — a number in meters above sea level that indicates the document's level of abstraction. README files are at 2,000m (highest level, broadest view). Implementation details are at 200m (ground level, specific). This README is marked at 2,000m. The PROPOSAL.md is at 800m (mid-level: detailed but still architectural). AGENTS.md is at 1,200m (above the implementation, below the vision).

They inherited this practice from their fieldwork, where every sensor reading is tagged with elevation. It stuck.

## Team Composition

| Name | Role | Specialty | Joined |
|------|------|-----------|--------|
| **Dr. Mei-Ling Chen** | Director / Principal Investigator | Terrace hydraulics, distributed systems theory | Founding (2016) |
| **Dr. Anh Nguyen** | Software Lead / Systems Architect | Multi-agent systems, mesh networking | 2018 |
| **Dr. Priya Sharma** | Formal Methods Researcher | Verification, type systems, property-based testing | 2021 |
| **Dr. Kenji Tanaka** | AI/ML Researcher | LLM integration, autonomous planning, reinforcement learning | 2022 |
| **Dr. Fatou Diallo** | Agricultural Robotics Researcher | Field deployment, sensor fusion, edge computing | 2023 |
| **Riku Hoshino** | Doctoral Candidate / Implementation Lead | Rust, systems programming, low-level optimization | 2024 |

The Institute operates on a flat hierarchy within each "terrace" (subteam) but has a clear vertical structure between terraces. Chen and Nguyen set direction. Sharma, Tanaka, and Diallo lead their research areas. Hoshino implements. Everyone publishes.

---

*Elevation: 2,000m*
*Terrace band: Summit overview*
*Season: Early planting*
