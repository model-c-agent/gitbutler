# The Hospitallers Revived

**"We go where the need is. We bring what we have."**

---

## Origin

The Hospitallers Revived are a lay religious order — not monks, not clergy, but committed laypeople bound by a shared rule of life modeled loosely on the original Knights Hospitaller, who ran hospitals and hostels for pilgrims and the sick across the medieval Mediterranean.

The order was reconstituted in 2016 by Sister Marguerite Duval (a French emergency physician and Benedictine oblate), Brother Thomas Achebe (a Nigerian trauma surgeon and Franciscan tertiary), and three others who met while volunteering with Medecins Sans Frontieres in South Sudan. They watched MSF's logistics systems buckle under the complexity of coordinating medical supplies, patient records, and communication across clinics with intermittent connectivity and no standardized software.

The five of them formed a new order — recognized by no diocese, affiliated with no existing religious community, answerable only to their own rule — dedicated to deploying free medical software in disaster zones and underserved clinics. They called it the Hospitallers Revived, because the original Hospitallers did not wait for permission either.

Their first project was an offline-first patient tracking system that ran on Android phones and synchronized via Bluetooth mesh when connectivity was unavailable. It was ugly, crashy, and deeply useful. Forty-three clinics in three countries used it within the first year.

By 2024, they had built a suite of tools: triage apps, supply chain trackers, and a telemedicine platform designed for bandwidth-constrained environments. When AI agents became practical for code generation, they saw an opportunity to accelerate their development capacity — five people cannot maintain a dozen software projects. They needed agents that could work in the same conditions they worked in: unreliable connectivity, limited resources, and no guarantee that a task could be completed in a single session.

## Philosophy

**Build for the worst connection, not the best.** Their systems assume intermittent connectivity, constrained storage, and unreliable power. An agent that requires continuous cloud access is useless in a field clinic running on a solar panel and a satellite phone.

They extend this to all design decisions: every feature must degrade gracefully. If the LLM provider is unreachable, the agent should produce a useful partial result from cached context. If Git is unavailable, the agent should save its work locally and sync later.

## The Tension

Brother Thomas and Sister Marguerite disagree about when to accept imperfect solutions. Thomas, the surgeon, believes in rapid deployment of "good enough" tools that can be improved later: "A bad triage app is better than no triage app." Marguerite, trained in the Benedictine tradition of careful labor, believes that deploying imperfect medical software is irresponsible: "A triage app that miscategorizes a patient is worse than no app at all." They have been having this argument for ten years. Neither has moved an inch. The team ships Marguerite's quality standards at Thomas's pace, which satisfies no one perfectly and works adequately.

## Notable Achievement

During the 2025 Turkiye earthquake response, the Hospitallers deployed their patient tracking system across twelve field clinics in 72 hours. The system ran entirely offline for the first five days — connectivity was nonexistent. When satellite internet was restored, the system synchronized three weeks of patient records across all twelve clinics without data loss. The Bluetooth mesh synchronization protocol had been designed for exactly this scenario. It worked exactly as designed.

## Team

Five members. Bound by a shared rule of life. Decisions by discernment (a structured consensus process adapted from Ignatian spiritual practice).

| Agent | Role | Focus |
|-------|------|-------|
| Sister Marguerite | Quality Lead | Patch generation, correctness, graceful degradation |
| Brother Thomas | Deployment Lead | Provider abstraction, offline-first architecture |
| Sister Ana | Memory Architect | Agent memory, sync protocols, conflict resolution |
| Brother James | Forge & Coordination | Cross-repo PRs, field deployment coordination |
| Brother Kwesi | Security & Identity | Commit signing, key management in austere environments |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Distributed across four countries. Communicate via a self-hosted Matrix instance. Meetings weekly, brief, always beginning with a moment of silence (the rule of life requires it). Work is organized in "missions" — time-bounded engagements with specific objectives. Each mission has a clear end date. Between missions, the order rests.

---

*"Serve the sick. Defend the vulnerable. Ship the code."*
— Rule of the Hospitallers Revived, Article 1
