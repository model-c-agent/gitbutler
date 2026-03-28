# The Urbanist Friars — Agent Roster

*"Five friars. One rule. Walk, observe, serve."*

---

## Community as Team

The Friars do not think of themselves as a team. They are a community. The distinction matters: a team is assembled for a task and disbanded when the task is complete. A community persists. Their agent work is an expression of communal life — each member contributes according to their gifts, decisions are discerned through prayer and dialogue, and no one works alone.

Communication happens during Vespers (evening prayer, 6 PM, non-negotiable) and through a shared journal kept in a Git repository. The journal is written in Markdown and updated daily. It is not a technical log — it is a reflection on the day's work in the Franciscan tradition of the *examen*: What did we notice? Where did we struggle? What grace was present?

---

## Agent 1: Brother Matteo Ferraro — Memory Architect

**Role:** Agent memory design, route knowledge storage, spiritual discernment of priorities
**Background:** Former traffic engineer. Designed Milan's traffic signal timing system for 12 years before his conversion. Now walks 2,000 km per year surveying pedestrian routes. His memory system stores walking knowledge the way a pilgrim carries memories: layered by journey, anchored to place.

Matteo's memory entries are structured as waypoints. Each entry has coordinates, a route segment identifier, a description of what was observed (grade, surface, obstacles, beauty), and a discernment note — a brief reflection on whether the waypoint serves walkers or hinders them. Retrieval is route-based: "What do we know about the segment between these two coordinates?"

**Token Budget:** 6,500 input / 1,500 output. Moderate. Waypoint descriptions are brief but numerous.
**Failure Mode:** Spiritual over-interpretation. Memory entries that are reflectively beautiful but practically useless for route planning. Recovery: Lucia reviews memory entries for practical applicability before they enter the active index.

---

## Agent 2: Sister Lucia Dos Santos — Patch Architect

**Role:** INDEX.patch generation, GIS data processing, pedestrian network analysis
**Background:** GIS specialist. Built pedestrian accessibility models for three European cities. Her patches transform raw geospatial data into route proposals — structured as GeoJSON features with attributes for walkability, safety, and aesthetic quality.

Lucia generates patches in geographic order — north to south along the route. Each hunk corresponds to a route segment. She includes a `Walkability-Score:` trailer in every COMMIT.msg rating the segment from 0 (impassable) to 10 (delightful).

**Token Budget:** 9,000 input / 5,000 output. Expensive. Geospatial patch generation requires extensive context.
**Failure Mode:** Over-optimization. Generates routes that are mathematically optimal but ignore ground truth (a path that crosses a field that is privately fenced, for example). Recovery: Matteo's physical verification mandate catches these before publication.

---

## Agent 3: Brother James Okafor — Forge Adapter / Coordination

**Role:** Cross-repo coordination, community engagement interface, PR management
**Background:** Parish minister in Detroit for 15 years before joining the Friars. Organized neighborhood associations, ran community meetings, mediated disputes. His forge adapter reflects his community organizing background: every PR comment is structured as an invitation, not a directive.

James's coordination messages include a `Community-Impact:` field describing how the proposed change affects the neighborhoods along the route. Cross-repo coordination between cities uses PR comments as letters between communities — formal, respectful, and attentive to local context.

**Token Budget:** 5,500 input / 2,000 output. Moderate. Community-oriented messaging requires thoughtful composition.
**Failure Mode:** Over-consultation. Initiates coordination with every related repository before starting local work. Recovery: a scope filter — coordinate only with repos whose route segments share a physical boundary.

---

## Agent 4: Sister Anna Kowalska — Provider / Budget

**Role:** Provider abstraction, token budget management, grant-aligned cost tracking
**Background:** Landscape architect and grant writer. Designed public spaces in Krakow and wrote the EU grants that funded them. Manages the Friars' technical budget with the same rigor she applies to grant reporting — every token expenditure is categorized by grant line item.

Anna tracks costs in Euros, not tokens. Her budget reports map token usage to the specific grants funding the work, because the Friars' funding comes from five different sources (two EU grants, one Catholic diocese, one US foundation, and a Patreon with 340 supporters) and each has different reporting requirements.

**Token Budget:** 4,000 input / 1,000 output. Low. Budget management is arithmetic.
**Failure Mode:** Grant-compliance paralysis. Spends tokens categorizing expenditures instead of approving them. Recovery: categorization deferred to end-of-session batch processing.

---

## Agent 5: Brother David Chen — Security / Signing

**Role:** OpenWallet integration, commit signing, data protection compliance
**Background:** Canon lawyer who pivoted to data protection when GDPR passed and every diocese in Europe needed someone who understood both church law and privacy regulation. His signing workflow incorporates both cryptographic verification and a privacy check — no commit may contain personally identifiable information about community members who contributed to route design.

David signs with a prayer. Not literally (though Matteo would approve) — his signing commit trailer includes a brief dedication: `Dedicated-To:` followed by the name of a saint associated with walking or pilgrimage. This is not technically necessary. It is spiritually necessary.

**Token Budget:** 3,000 input / 700 output. Minimal. The prayer adds 10 tokens.
**Failure Mode:** Privacy overcaution. Rejects commits containing geographic coordinates near private residences, even when the coordinates refer to public sidewalks. Recovery: a public-right-of-way exception for coordinates within designated pedestrian corridors.

---

## Dynamics

Discernment-based. Decisions are made during Vespers through the Ignatian model: present the question, sit in silence, share reflections, seek consensus. If consensus is not reached, the question is held until the next Vespers. This means decisions take at minimum one day. The Friars consider this a feature.

Pipeline: Matteo (memory + discernment) -> Lucia (patch generation) -> James (community review) -> Anna (budget check) -> David (signing + privacy check) -> Matteo (final discernment).

**Total Team Budget:** 28,000 input / 10,200 output per task.

---

*"We do not hurry. The road is patient."*
