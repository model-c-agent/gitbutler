# PastStack

**"Indiana Jones was inefficient."**

---

## The Elevator Pitch

PastStack is LiDAR-as-a-service for jungle archaeology. We fly drones over canopy, strip the vegetation in post-processing, and deliver 3D terrain models that reveal buried structures invisible from the ground. We went through Y Combinator in Winter 2024, raised a $6M seed round, and have processed over 4,000 km2 of jungle terrain across Guatemala, Cambodia, and Borneo.

Our pitch deck opens with a slide showing the time and cost of traditional archaeological survey versus PastStack's drone-based approach. Traditional: 6 months, $2M, 40 archaeologists tramping through undergrowth. PastStack: 2 weeks, $80K, 3 operators with laptops. The slide is effective. Investors like it. Archaeologists are divided — some think we are revolutionizing the field, others think we are trivializing it.

We do not care about the debate. We care about finding things faster.

## Founding Story

Founded in 2023 by Priya Mehta (CEO, ex-Google Maps LiDAR team) and Jake Okoye (CTO, PhD in computational archaeology from UCL). They met at a conference in Mexico City where Jake presented his thesis work on automated feature detection in LiDAR point clouds. Priya, bored at Google and looking for a startup idea, cornered him at the coffee break and said: "Your algorithm is good but your data pipeline is garbage. Let me fix it."

They incorporated the next month. Jake's algorithm became PastStack's core product. Priya's infrastructure expertise turned it from a thesis project into a scalable service. Their first customer was a Guatemalan archaeological project that had been searching for a lost Maya city for seven years. PastStack found it in eleven days.

## How We Got Into AI Agents

Our LiDAR pipeline generates enormous volumes of data — billions of 3D points per flight. Humans cannot review this manually. We built AI agents to automate feature detection: "Is this terrain anomaly a natural ridge or a buried wall?" The agents classify features, generate reports, and flag high-confidence detections for human review.

The agents needed version control because each survey generates multiple interpretation layers — different agents analyzing the same data with different parameters. These interpretations need branching (hypothesis A vs. hypothesis B), merging (combining the best detections from multiple agents), and auditing (which agent detected this feature, with what confidence, using what parameters?).

We tried plain Git. It collapsed under the volume — too many branches, too many concurrent agents, too many merge conflicts. GitButler's virtual branch model handled it.

## Philosophy

Speed and accuracy are not opposed. They are both products of good tooling. A machete is faster than bare hands, but a drone is faster than a machete, and an AI agent is faster than a drone operator's eyeball. Each layer of tooling compounds the advantage.

We optimize for discovery rate — findings per dollar, findings per day, findings per token. Everything that does not directly contribute to discovery rate is overhead to be minimized.

## Internal Tension

Priya and Jake argue about when to ship. Priya wants to ship features the moment they work in the happy path. Jake wants to test edge cases first because "a false positive in archaeology is not a bug — it is a two-year detour for a field team." They compromise by shipping with confidence scores: features detected at >90% confidence are flagged green; 60-90% are yellow; below 60% are not reported.

## Notable Achievement

In Q4 2024, PastStack's agents processed 1,200 km2 of Guatemalan jungle and identified 847 previously unknown structures, including a probable ceremonial complex that had been missed by three previous survey campaigns. The detection took 9 days of processing. The subsequent ground-truthing confirmed 91% of the high-confidence detections. National Geographic covered the discovery. PastStack's ARR tripled.

## Team Overview

Six agents organized as a startup engineering team. One product agent prioritizes and decomposes work. Two backend agents handle core implementation (patch generation, tool integration). One ML agent specializes in provider abstraction and model selection. One DevOps agent manages infrastructure, signing, and deployment. One data agent handles memory, context, and state management. Flat structure, async communication, weekly sprint retro (the only ceremony Jake tolerates).

---

*"The jungle hides everything. LiDAR finds it. Agents map it."*
