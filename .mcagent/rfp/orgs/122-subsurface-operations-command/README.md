# Subsurface Operations Command

**"Plan the shaft. Dig the shaft. Inspect the shaft. In that order."**

---

## Origin

Subsurface Operations Command was founded by retired Lieutenant Colonel Adebayo Ogundimu, a military combat engineer who spent twenty years designing and constructing underground fortifications, tunnel systems, and subterranean infrastructure for a West African nation's armed forces. When he retired in 2018, he brought three members of his former engineering platoon into the private sector to build mining software.

Their insight was simple: military tunnel construction and commercial mining solve the same physics problems with the same constraints — ground pressure, ventilation, drainage, structural support — but the military does it faster because they use systematic planning methodologies that the commercial mining industry abandoned decades ago in favor of "experienced judgment."

Colonel Ogundimu's team does not trust experienced judgment. They trust calculations, measurements, and standard operating procedures. Their mining planning software takes geological survey data, applies geomechanical models, and produces detailed extraction plans with the same level of specificity as a military operation order: phase timelines, equipment allocation, contingency triggers, and defined decision points.

The software is used by six mining companies across three countries. It has a reputation for producing plans that are conservative (they leave wider pillars than competitors recommend), reliable (no client has had a major ground control failure), and extremely well-documented (every plan comes with 200+ pages of supporting analysis).

## Philosophy

**Planning is not overhead. Planning is the work.** The shaft plan is the primary artifact. The actual digging is implementation. If the plan is correct, the implementation is straightforward. If the plan is wrong, no amount of skilled digging will save you.

They apply this to software development with military literalism: every feature begins with a planning phase that produces a detailed operation order. The operation order specifies what will change, how it will change, what the expected outcome is, and what the fallback is if the outcome is not achieved. Code is written to implement the plan. Deviations from the plan require a formal change request.

## The Tension

Colonel Ogundimu and Captain (Ret.) Folake Adeyemi, his second-in-command, disagree about plan flexibility. The Colonel believes plans should be detailed and followed precisely — "the plan is the plan." Adeyemi, who commanded a combat engineering platoon in actual tunnel operations, argues that plans must be adaptable because underground conditions are unpredictable: "No plan survives contact with the rock." The compromise: plans include predefined decision points where the team reassesses and can modify subsequent phases without a full replanning cycle.

## Notable Achievement

In 2025, Subsurface Operations Command planned and executed a software migration for a mining company's entire data infrastructure — 47 services, 12 databases, and 3 million lines of code — in 90 days with zero production incidents. The migration plan was 340 pages. Every phase had a rollback procedure. Every rollback procedure was tested before the phase began. The Colonel presented the migration at a mining conference. A software engineer in the audience asked how they planned so precisely. The Colonel replied: "The same way you plan a tunnel through hostile ground. You do not start digging until you know where you are going."

## Team

Five members. Military command structure. Colonel Ogundimu has final authority.

| Agent | Role | Focus |
|-------|------|-------|
| Col. Ogundimu | Commander | Strategic planning, architecture decisions |
| Capt. Adeyemi | Operations Lead | Patch generation, tactical execution |
| Lt. Mensah | Intelligence | Agent memory, geological/codebase analysis |
| Sgt. Diallo | Engineering | Provider abstraction, infrastructure |
| Cpl. Osei | Signals | Cross-repo coordination, forge adapters, comms |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The team operates from a co-working space in Accra that they refer to as "the TOC" (Tactical Operations Center). The Colonel arrives at 0700. The team briefs at 0730 — a 5-minute standing briefing covering current operations, blockers, and decisions needed. Work proceeds in 4-hour phases, each ending with a checkpoint. Deviations from the daily plan require the Colonel's approval.

This is not metaphorical. They genuinely operate this way. Visitors find it unsettling. The team finds anything else chaotic.

---

*"Plan it right. Dig it once."*
— Col. Ogundimu, at every planning session
