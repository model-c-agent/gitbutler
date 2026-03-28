# Streets for All Assembly

**"Nobody asked us to design our streets. So we did it ourselves."**

---

## What We Are

Streets for All Assembly is an anarchist urban planning collective based in Oakland, California. We are residents -- not planners, not engineers, not politicians -- who design, propose, and sometimes build streetscape improvements for our own neighborhoods. We operate without permits when we can, with permits when we must, and with city council approval never, because the city council does not approve our projects. They tolerate them, which is close enough.

The Assembly was founded in 2020 during the pandemic, when four neighbors on a dead-end street in the Fruitvale district blocked their street with planters, painted a mural on the asphalt, and installed two benches made from reclaimed lumber. The city sent a code enforcement notice. Forty-seven neighbors signed a petition to keep the improvements. The city withdrew the notice. The Assembly was born.

Since 2020, we have completed 23 neighborhood streetscape projects: intersection murals, parklets, traffic calming installations, community bulletin boards, and one unauthorized protected bike lane that lasted six months before the city removed it and then, nine months later, built an official version in the same location. We consider this a win.

## The People

The Assembly has no members in the traditional sense. It has participants -- people who show up. The core participants who show up consistently:

**Rosa Vega**: Community organizer. She knocks on doors, collects petitions, negotiates with city inspectors, and manages the Assembly's social media (poorly, by her own admission). She does not write code. She writes demands.

**Tomoko Hara**: Landscape architect (licensed). The only professionally trained planner in the group. She produces the design drawings that transform community input into buildable plans. She insists on structural soundness even for "tactical" installations. "Anarchist benches still need to support weight."

**Jamal Washington**: Carpenter and fabricator. He builds what Tomoko designs. He also writes Python scripts for the Assembly's mapping platform because he taught himself programming during COVID lockdowns and discovered that GIS software is "just woodworking for data."

**Pen Sirikit**: Data analyst by day, transportation equity researcher by night. She analyzes traffic patterns, pedestrian counts, and crash data to prioritize which streets need intervention most. She also manages the Assembly's Git infrastructure because she is the only participant who knows what Git is.

## How Software Got Involved

In 2023, Pen built a neighborhood mapping platform: a web application that displayed crash data, pedestrian counts, traffic speeds, and community complaints overlaid on a street map. The data came from public sources (city open data portals, NHTSA crash reports, Census transportation data) and from the Assembly's own observations (manual pedestrian counts conducted by volunteers).

The platform was version-controlled in Git because Pen version-controls everything. Jamal contributed analysis scripts. Tomoko contributed design files (saved as SVG in the repo). Rosa contributed community input data (surveys, petition signatures, meeting notes).

In 2024, Pen added AI agents to automate data collection and analysis. Agents scraped public data portals, normalized the data, and produced INDEX.patch files that updated the mapping platform's database. The agents also generated preliminary prioritization reports: ranked lists of streets that needed intervention, based on crash frequency, pedestrian volume, and community complaints.

The `but-ai` RFP arrived when Pen was debugging a merge conflict between two agents that had simultaneously updated the crash data for different intersections. She forwarded the RFP to the Assembly with: "This solves our merge problem and gives us a real agent framework."

## Philosophy

### On Streets

Streets are public space. The most public space. More people use streets than any park, plaza, or civic building. Yet streets are designed by engineers for cars, not by residents for people. We design streets for people, because we are the people who use them.

### On Authority

We do not seek permission to improve our neighborhoods. We seek participation. If the people who live on a street want a bench, they should have a bench. The fact that this requires a permit, a review, and a 90-day comment period is a failure of the system, not of the request.

### On AI

AI agents collect and analyze data that we do not have the volunteer hours to collect manually. A pedestrian count at 47 intersections over 30 days requires 1,410 observation sessions. We have 12 regular volunteers. The math does not work without automation. Agents make the math work.

## Tension

**The Data vs. Intuition Fight.** Pen prioritizes streets based on data: crash frequency, pedestrian volume, speed measurements. Rosa prioritizes based on community input: which streets are people complaining about, where do parents feel unsafe walking kids to school, which intersections do elderly residents avoid. Pen's data and Rosa's community input sometimes agree. When they disagree, the Assembly argues. Jamal mediates by asking "can we just build both?" which is usually yes, because the Assembly does not have a budget constraint -- it has a volunteer constraint.

## Achievement

In 2025, the Assembly completed the Fruitvale Corridor Project: a series of 8 intersection improvements along International Boulevard, designed entirely from community input and data analysis. The improvements included painted curb extensions, reflective bollards, neighborhood wayfinding signs, and three public seating areas built from reclaimed freeway barrier material. Pedestrian injuries at the 8 intersections dropped 34% in the first six months compared to the same period in the prior year. The city's transportation department cited the project in a federal grant application without crediting the Assembly. The Assembly considers this the highest form of compliment.

## Team

| Agent | Role | Focus |
|-------|------|-------|
| Pen | Data Lead | INDEX.patch, analysis pipeline, infrastructure |
| Tomoko | Design Lead | Memory systems, spatial data, design standards |
| Jamal | Builder / Scripter | Provider abstraction, tooling, forge adapters |
| Rosa | Community Lead | Coordination, community input, PR communication |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The Assembly meets every other Thursday at 7 PM in the backyard of whoever is hosting. Meetings are potluck. Agenda is set by whoever has something to say. Decisions are made by consensus of those present. If you are not present, you accept the decision. This is the social contract.

Between meetings, work happens asynchronously on GitHub. Pen and Jamal commit code. Tomoko commits design files. Rosa commits community data. The agents commit everything else.

---

*"The street belongs to the people who walk it."*
-- Painted on the Fruitvale mural, 2020
