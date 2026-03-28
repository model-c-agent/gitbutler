# Killstreak Operations Bureau

**"Sleep is a debuff. Discipline is the meta."**

---

## Origin

The Killstreak Operations Bureau was founded in 2021 by three former military logistics officers who transitioned into competitive gaming infrastructure after discovering that tournament coordination is, operationally, indistinguishable from forward operating base logistics. Staff Sergeant (ret.) Marcus "Reaper" Hale organized the first structured practice schedule after watching a Tier 1 Valorant team lose a $200K semifinal because their IGL was running on four hours of sleep and a gas station energy drink. He believed the problem was not talent. The problem was discipline.

The Bureau runs on military time. 0600 wake-ups. 0630 physical training. 0800 VOD review briefings conducted in a format borrowed from after-action reports: "What was the objective? What happened? What went wrong? What do we do differently?" Every scrim is logged. Every death is catalogued with a root-cause classification: positioning error, utility misuse, communication failure, mechanical failure, or "fog of war" (genuinely unpredictable). The classification system was adapted from incident response frameworks used in NATO logistics chains.

They built their first software tool — a scrim analytics platform called `killchain` — because no existing product could ingest demo files from five different games and produce unified performance metrics. `killchain` treats every round of a competitive match as a discrete operation with an objective, a timeline, and measurable outcomes. It has been adopted by 40+ amateur teams and three semi-pro organizations.

## Why AI Agent Tooling

The Bureau encountered GitButler while building an automated coaching system. The system analyzed VOD reviews, identified recurring mistakes, and generated practice drills as structured patches to team playbooks. The playbooks were stored in Git — each team's strategy set was a repository, each meta shift was a branch, and each practice drill was a commit. When the coaching agents started conflicting with each other (the aim trainer agent and the positioning agent both wanted to change the same warmup routine), the Bureau needed virtual branches.

The `but-ai` RFP matched their operational need: disciplined agents that produce auditable artifacts, stay within budget, and never go rogue. They see agents the way they see soldiers — individually capable, but only effective when operating within a clear chain of command with well-defined rules of engagement.

## Philosophy

### On Agents

Agents operate under rules of engagement. Every agent has a defined area of operations. Exceeding your AO without authorization is a failure, not initiative. Agents do not improvise; they execute plans. If the plan is wrong, you flag it up the chain and wait for revised orders. The Bureau does not tolerate "creative" agents that decide to solve problems outside their scope.

### On Version Control

Version control is the operational log. Every commit is an entry in the log. Unsigned commits are like unattributed radio transmissions — you do not trust them, you do not act on them, and you investigate their origin.

## The Friendly Fire Incident

In February 2025, two coaching agents simultaneously pushed conflicting strategy changes to the same playbook. The aim trainer agent replaced a crosshair placement drill with a flick training routine. The positioning agent replaced the same drill with a jiggle-peek exercise. Neither agent was aware of the other's work. The merge produced a hybrid drill that instructed players to "flick to a jiggle-peek position while maintaining crosshair placement at head height" — a physically impossible maneuver.

Three players attempted the drill before anyone read it carefully. One sprained a wrist. The Bureau now requires all agent patches to pass through a conflict-detection gate before merge. They call it the "deconfliction layer" and consider it the most important component of their architecture.

## Achievement: The 72-Hour LAN Deployment

At DreamHack Winter 2025, the Bureau deployed their coaching system live during a 72-hour LAN tournament. Six agents ran continuously, analyzing matches in near-real-time and generating practice adjustments between rounds. The system processed 340 rounds across 4 games, produced 89 drill patches, and maintained a 97% conflict-free merge rate. The team using the system finished 5th out of 128 — their best result by 40 positions.

## Team Composition

Six agents, strict hierarchy. Commander Hale reviews all escalations.

| Agent | Callsign | Role |
|-------|----------|------|
| Marcus Hale | REAPER | Commander — final authority, deconfliction |
| Priya Nair | OVERWATCH | Intelligence — VOD analysis, pattern recognition |
| Tomás Reyes | ORDNANCE | Patch Architect — drill generation, playbook commits |
| Jin-ae Park | COMMS | Protocol — cross-team coordination, forge adapters |
| Elias Brandt | QUARTERMASTER | Budget — token allocation, provider management |
| Yuki Tanaka | SENTRY | Security — commit signing, key management |

Details in [AGENTS.md](AGENTS.md).

## Working Style

Daily operational tempo. 0800 briefing: yesterday's metrics, today's objectives, blockers. 1700 debrief: what got done, what got deferred, why. Between briefing and debrief, agents execute independently within their assigned AO. Cross-AO coordination requires REAPER approval. All communication follows a structured format:

```
SITREP — <callsign> — <timestamp>
OBJECTIVE: <current task>
STATUS: green | amber | red
BLOCKERS: <none | description>
TOKEN BURN: <used>/<allocated>
```

No informal communication. No emoji. No banter. The Bureau considers chattiness an operational security risk.

---

*"Discipline is not the absence of chaos. It is the refusal to participate in it."*
— Bureau Standing Order #1
