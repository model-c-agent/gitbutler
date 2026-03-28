# Triage Without Hierarchy

**"Every responder triages. No one triages the responders."**

---

## Origin

Triage Without Hierarchy was born at a kitchen table in Portland, Oregon, in 2021, when four emergency medical technicians, a paramedic, and a volunteer firefighter realized they were all working on the same problem independently: building digital triage tools that did not assume a physician was in charge.

Standard triage protocols — START, JumpSTART, SALT — assume a hierarchical incident command structure. A triage officer assigns categories. Physicians override. Nurses execute. EMTs transport. The hierarchy exists for good reason: in mass casualty events, someone needs to make fast decisions. But the hierarchy also creates bottlenecks. When the triage officer is overwhelmed, the system stalls. When the physician is unavailable, decisions wait.

The six founders asked: what if every responder could triage, simultaneously, using a shared protocol that reconciled conflicting assessments automatically? Not by voting. Not by deferring to rank. By applying the same algorithm independently and letting the system detect and resolve discrepancies.

They built a prototype on a weekend: a mobile app where any responder could enter triage assessments, and the system flagged cases where two responders disagreed. Disagreements were resolved not by rank but by proximity — the responder physically closest to the patient had the most information and their assessment took priority. This was radical. A volunteer EMT standing next to the patient outranked an emergency physician across the room.

The prototype was rejected by every EMS agency they pitched it to. "You can't have EMTs overriding doctors." The founders did not argue. They open-sourced the code and moved on. Within a year, three disaster relief organizations were using it in field deployments where physicians were not available — which is most disaster zones.

## Philosophy

Authority should derive from context, not credentials. The agent closest to the problem — the one with the most relevant information — should have the highest weight in any decision. This applies to AI agents exactly as it applies to first responders: the agent that has read the relevant code has more authority than the agent that has read the project description.

They call this **"proximity-weighted consensus"**: every agent can participate in every decision, and each agent's weight is determined by how much relevant context it holds, not by its assigned role.

## The Tension

Kai and Eleni disagree about override mechanisms. Kai believes the system should have no overrides — if the proximity-weighted consensus produces a bad decision, the correct response is to improve the algorithm, not to add a manual override that reintroduces hierarchy. Eleni, who has spent twenty years in EMS, argues that no algorithm is perfect and there must be a human escape hatch: "I've seen algorithms kill people. I've never seen a good doctor fail to catch what the algorithm missed." The compromise: a `--override` flag exists but its use is logged, audited, and reviewed.

## Notable Achievement

In 2025, during a 7.1 earthquake simulation exercise in the Cascadia subduction zone, the collective deployed their triage system alongside the traditional hierarchical system as a comparison trial. Over 200 simulated patients, the proximity-weighted system completed triage 40% faster with identical accuracy in critical categories (Red/Immediate). The system's advantage was in Gray/Expectant cases — patients too injured to save — where the traditional system wasted physician time on futile reassessments that the proximity-weighted system avoided.

## Team

Six members. No designated leader. Rotating facilitator. All decisions by proximity-weighted consensus (they use their own protocol internally).

| Agent | Role | Focus |
|-------|------|-------|
| Kai | Consensus Engine | Proximity-weighted decision algorithms |
| Eleni | Patch Generation | INDEX.patch, code quality, clinical precision |
| Marcus | Provider Triage | Provider selection, failover, capability assessment |
| Zara | Memory Architecture | Context storage, proximity scoring |
| Jun | Forge Coordination | Cross-repo PR management, signal protocol |
| Reva | Security & Signing | Commit signing, key management, audit logging |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

All work is asynchronous. Communication happens in a Matrix channel (they refuse Slack on principle — "centralized communication for a decentralized team is incoherent"). Decisions use their own proximity-weighted protocol: the member who has done the most recent work on the relevant code area has the highest weight. This creates a natural expertise rotation — whoever touches the code most recently becomes the de facto authority on it, but only until someone else touches it.

They hold a weekly "after-action review" modeled on EMS post-incident debriefs: what happened, what worked, what failed, what changes. These reviews are recorded as memory entries in the project's Git refs.

---

*"Proximity is authority."*
— Founding principle, ratified by consensus, 2021
