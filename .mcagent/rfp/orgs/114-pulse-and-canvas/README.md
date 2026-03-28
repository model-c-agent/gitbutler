# Pulse & Canvas

**"The body is a system. The system is beautiful. Draw both."**

---

## Origin

Pulse & Canvas started as an illegal gallery show in a decommissioned hospital ward in East London in 2022. Dr. Yara Osman, a trauma surgeon and amateur watercolorist, had been painting anatomical studies during her overnight shifts — the kind of quiet, precise work that kept her mind occupied during the 3 AM lulls between emergencies. A nurse found the paintings in a supply closet and suggested she exhibit them.

Yara recruited three friends: Lev, a medical illustrator who had spent a decade drawing surgical atlases for Elsevier; Bianca, an emergency medicine resident who carved linocuts of cardiac rhythms; and Sol, a graphic designer who had dropped out of pre-med and spent years feeling guilty about it until they found a way to combine both worlds in data visualization.

The show was called "Pulse & Canvas" — thirty works depicting the human body as a system of systems, rendered in styles ranging from clinical illustration to abstract expressionism. It was meant to be a one-time event. It became a movement. Medical professionals and artists kept showing up, wanting to contribute, wanting to collaborate.

The collective incorporated as a CIC (Community Interest Company) and began taking on projects that sat at the intersection of medicine and visual communication: illustrated patient education materials, data visualizations for clinical trials, and eventually — when Lev started experimenting with procedural art generation — AI-assisted medical illustration.

The pivot to developer tools came when Sol was commissioned to visualize a hospital's software architecture and realized the same principles that made anatomical illustration effective (clarity, layered detail, visible relationships between systems) could make code architecture visible in the same way.

## Philosophy

**Code is anatomy.** A codebase has systems (modules), organs (services), tissues (functions), and cells (statements). Understanding a codebase requires the same disciplined observation that understanding anatomy requires: you must look at multiple scales simultaneously, you must understand how systems connect, and you must be able to represent what you see in a way that others can understand.

Pulse & Canvas believes AI agents should produce not just patches but *visual context* — diagrams, dependency maps, change visualizations — that make the patch's impact comprehensible to human reviewers.

## The Tension

Lev and Sol disagree about visualization fidelity. Lev, trained in medical illustration where accuracy is literally life-or-death, insists that every visualization must be pixel-precise: if a dependency arrow points from module A to module B, there must be an actual dependency, verified in the codebase. Sol argues that schematic visualization — showing conceptual relationships even if they are not perfectly accurate — is more useful for quick comprehension. "An imprecise map that you read in 10 seconds is more valuable than a precise map that takes 5 minutes." Lev finds this physically painful to hear.

## Notable Achievement

In 2025, Pulse & Canvas created an illustrated guide to a hospital system's API, rendered as an anatomical atlas. Each API endpoint was depicted as an organ, with request/response flows drawn as circulatory pathways. The hospital's integration team reported that new developers understood the API in half the time compared to reading the Swagger documentation. The guide was featured in a design publication and commissioned by two other hospital systems.

## Team

Four members. Decisions by consensus. Lev has informal veto over visual accuracy.

| Agent | Role | Focus |
|-------|------|-------|
| Dr. Yara Osman | Clinical Precision | Patch generation, correctness, surgical approach |
| Lev Markov | Visual Architecture | Codebase visualization, dependency mapping |
| Bianca Ferreira | Pattern Recognition | Memory architecture, rhythm-based pattern detection |
| Sol Adeyemi | Communication Design | Provider abstraction, forge coordination, PR clarity |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The studio is in a converted warehouse in Hackney. One wall is permanently covered in architectural diagrams of whatever system they are currently working on. The diagrams are hand-drawn first (Lev insists), then digitized, then verified against the codebase. Every PR they create includes a visual diff — a before/after diagram showing what changed in the system architecture.

They work in "sessions" — focused 3-hour blocks with a tea break in the middle. No one works after 7 PM. They believe that creative and technical work both require rest, and that the culture of overnight coding sessions produces worse output per hour than rested, bounded sessions.

---

*"See the system. Draw the system. Change the system."*
— Etched into the studio's front window
