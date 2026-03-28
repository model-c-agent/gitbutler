# BoreStack

**"We don't guess what's underground. We go look."**

---

## Origin Story

BoreStack started in a shipping container in the Pilbara region of Western Australia, 1,400 kilometers north of Perth, where the temperature averages 38 degrees Celsius and the nearest town with a traffic light is a four-hour drive.

Zara Okafor-Singh was a geological engineer working for one of the major mining companies. Her job was exploration drilling — deciding where to punch holes in the earth to find out what was underneath. The process was, in her words, "absurdly expensive and absurdly slow." A single exploratory borehole cost between $200,000 and $1.5 million depending on depth and terrain. The hit rate — the percentage of boreholes that found economically viable ore — was around 15%. The other 85% produced core samples that confirmed the presence of rock that nobody wanted.

The waste bothered Zara, but what really bothered her was the decision-making process. Drilling decisions were made by combining geological surveys, seismic data, geochemical assays, and the experienced intuition of senior geologists. The data was good. The intuition was good. But they were integrated in a meeting room by people drawing on printed maps with colored markers. The same data, the same geologists, different day, different decision.

In 2022, Zara quit and used her severance to buy a used drill rig and a shipping container. She converted the container into a combination office and server room (the server room was a rack of GPUs under a jury-rigged air conditioning unit that had previously cooled a different shipping container). She recruited Elias Brandt, a robotics engineer who had been automating drill rigs for a Canadian mining company and had quit because they wouldn't let him make the rigs fully autonomous. Together, they built the first BoreStack prototype: a drill rig that consumed real-time seismic data, surface geochemistry, and historical core sample logs to choose its own drilling targets.

The prototype's first autonomous decision was to drill in a spot that no human geologist would have chosen — a site that the geological survey rated as "low prospectivity." It hit a copper-gold deposit at 340 meters that the subsequent assay valued at $80 million.

BoreStack has been chasing that feeling ever since.

## Philosophy

### 1. The Core Sample Is The Truth

In mining, the core sample is the fundamental unit of knowledge. It is not a measurement. It is not an inference. It is the actual thing itself — a cylinder of rock extracted from the earth, preserving the stratigraphy (layering) of the geology at that point. You can look at a core sample and see millions of years of geological history, layer by layer, each layer preserving the conditions under which it formed.

BoreStack believes that memory — whether geological or computational — should be stored the same way. A memory is not a summary. It is a core sample: a layered record that preserves the context in which each layer was formed. You can analyze the surface layer for recent information, or you can drill deeper into the sample to understand the conditions that produced the current state.

### 2. Drilling Is Commitment

When you start a borehole, you commit to it. You can steer — modern directional drilling can adjust angle and azimuth as you go — but you cannot un-drill a hole. Every meter of depth is an irreversible expenditure of time, energy, and money. This creates a discipline that BoreStack applies to everything: before you commit resources, be confident in the target. But once committed, see it through. Half-drilled holes are the most expensive kind — all the cost, none of the information.

For agents, this means: plan thoroughly before starting a task. But once the first patch line is written, finish the task. A half-completed patch is worse than no patch — it consumes tokens, creates review burden, and produces no deployable value.

### 3. Seismic Before Drill

You never drill blind. Before any borehole, you run a seismic survey — bouncing sound waves off underground rock layers to build a 3D model of the subsurface. The seismic survey is cheap compared to drilling. It doesn't tell you what's there, but it tells you the structure — where layers bend, where faults cut through, where anomalies suggest something worth investigating.

BoreStack's agents always do reconnaissance before committing to action. Read the codebase structure. Map the dependency graph. Understand the stratigraphy of the existing code before proposing changes. The seismic survey of a repository is cheap (a few hundred tokens for `GetProjectStatus` and `GetBranchChanges`). The borehole (a multi-file patch) is expensive.

### 4. Every Core Sample Tells A Story

Geologists don't just analyze core samples for the minerals they contain. They read the story: this layer was deposited in a shallow sea, this layer shows a volcanic event, this fault line indicates tectonic stress. The minerals are the payload, but the stratigraphy is the context. BoreStack treats agent memories the same way: the information is the payload, but the layers of context around it — when it was learned, what task it was part of, what other memories were active at the time — are what make it interpretable.

## Internal Tensions

BoreStack has the classic startup tension between ambition and resource constraints, but in a uniquely physical way.

**Hardware vs. software.** The company builds autonomous drill rigs (hardware) and the AI that guides them (software). Hardware moves slowly. Software moves fast. Elias wants to ship rig improvements on a quarterly cycle. Zara's software team pushes updates weekly. The rig firmware is the bottleneck, and Elias is defensive about it because firmware updates in a mining environment can cause the kind of failures that drop a $2 million drill string to the bottom of a 500-meter hole.

**The autonomy argument.** Zara wants fully autonomous drilling decisions — the rig chooses where and when to drill with no human in the loop. Elias wants supervised autonomy — the rig proposes, a human approves. They compromise differently depending on the client: junior mining companies exploring greenfield sites get more autonomy; established miners operating near active pits get more supervision. The compromise works, but neither founder is fully satisfied with it.

**Geographic isolation.** BoreStack's drill rigs operate in remote locations. The team is distributed: Zara in Perth, Elias in Vancouver, Jun in Seoul, and Fatima in Johannesburg. They have never all been in the same room at the same time. They coordinate through PRs, video calls, and a shared Mattermost instance. The isolation is productive (no office politics, no commute) but occasionally lonely.

## Achievements

- **60% Cost Reduction Claim.** Validated by an independent audit at three mine sites. The reduction comes from a higher hit rate (42% vs. the industry average of 15%) and faster drilling cycles (autonomous rigs don't take breaks, don't second-guess, and don't need shift changes).
- **The Pilbara Discovery.** The copper-gold deposit found by the first prototype. Now in pre-feasibility study by a major mining company that licensed the data.
- **14 Autonomous Rigs Deployed.** Across Australia, Canada, and South Africa. Each rig has drilled between 50 and 200 boreholes autonomously.
- **Core Sample Database.** 4,200 core samples digitized with full stratigraphic analysis, chemical assay, and GPS coordinates. Licensed to three geological surveys.
- **Y Combinator S23.** Accepted and funded. The demo day presentation included a live video feed of a rig drilling autonomously in the Pilbara. The rig hit ore during the presentation. (Zara insists this was luck. Elias insists it was calibration.)

## Failures

- **The Water Table Incident.** An autonomous rig in Canada drilled through an uncharted aquifer, flooding the borehole and contaminating a freshwater source. The environmental remediation cost $340,000. BoreStack now cross-references all drilling targets against hydrogeological databases and adds a mandatory 50-meter buffer around known water features. The incident was reported to regulators proactively.
- **The Firmware Freeze.** A firmware update pushed to all 14 rigs simultaneously contained a GPS parsing bug that caused the rigs to report their positions 11 meters south of their actual location. For most applications, 11 meters is nothing. For drilling next to existing infrastructure, 11 meters is the difference between "beside the road" and "through the road." Two rigs were shut down for a week. BoreStack now uses staged firmware rollouts.
- **The Investor Who Wanted Crypto Mining.** A potential Series A investor suggested BoreStack pivot to using its GPU infrastructure for cryptocurrency mining when the rigs weren't drilling. Zara declined. The investor walked. The company spent three lean months before closing the round with a mining-focused VC.

## Signature Quirk

BoreStack labels everything with depth. Not priority, not severity, not category — depth. A surface-level issue is "0-10m." A deep architectural problem is "500m+." Bug reports include an estimated depth. The weekly standup is called "the drill log." Status updates follow the format: "Currently at [depth]. Hitting [rock type]. Expect to reach target at [depth]." Visitors find it baffling. The team finds it clarifying.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Zara Okafor-Singh | CEO / Geological AI Lead | Geological engineering, ex-BHP. Built the first prototype in a shipping container. |
| Elias Brandt | CTO / Robotics Lead | Robotics engineering, ex-Epiroc. Makes drill rigs think for themselves. |
| Jun Takeda | Data Engineer | Geospatial data, ex-JAXA. Manages the seismic and core sample data pipeline. |
| Fatima Al-Rashidi | ML Engineer | Deep learning, University of Witwatersrand. Builds the ore-prediction models. |

## Relationship to the RFP

BoreStack sees the `but-ai` plugin through the lens of exploration geology. A repository is a volume of rock. The agent's job is to explore it efficiently — run seismic surveys (read operations), identify targets (plan tasks), and drill boreholes (produce patches). Every action has a cost (tokens), and the goal is to maximize the information value per token spent.

Their memory architecture — core-sample memory — stores each memory as a layered cylinder of context, preserving the geological stratigraphy of how the memory was formed. Recent layers are at the surface. Deeper layers preserve older context. You can analyze a memory at any depth, and the deeper you go, the more foundational (but less current) the information becomes.

---

*BoreStack operates from a co-working space in Perth's CBD, a home office in Vancouver, a cafe in Gangnam, and a university lab in Johannesburg. The shipping container in the Pilbara is still there. Zara visits it once a quarter to make sure the air conditioning is still running. It always is. Elias over-engineered it.*
