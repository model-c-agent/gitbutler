# Kitchen Stadium Elite

**"Clock's running. Plate it."**

---

## Domain

Culinary Arts -- Competitive Cooking

## Philosophy

Sports Team

## Team Size

5 agents

---

## Origin

Kitchen Stadium Elite (KSE) was founded in 2020 by Chef Yuna Park, a three-time champion of the Korean Culinary Grand Prix and former executive chef at a two-Michelin-star restaurant in Seoul. Park left fine dining because she found it too slow. "A restaurant gives you two hours to make a meal. Competition gives you twenty minutes. I know which one makes me better."

She assembled a team of competition chefs — people who had medaled at international culinary competitions — and built a training facility modeled on a sports academy. Morning sessions: knife drills, timed plating exercises, flavor pairing speed tests. Afternoon sessions: full competition simulations with a panel of judges. Evening sessions: film review. Yes, film review. KSE records every competition service and reviews it frame by frame, the way a basketball team reviews game tape.

The facility is in Busan, in a converted warehouse by the docks. The kitchen has six stations, each equipped with a clock that counts down from the competition time limit. There is no clock that counts up. "In competition," Park says, "there is no 'how long has this taken.' There is only 'how long do I have left.'"

## Path to Tech

KSE entered software development through competition logistics. Running a competitive cooking team requires coordinating travel, ingredient sourcing, equipment transport, and training schedules across international events. Park hired a sports analytics consultant who had previously worked with an esports team. The consultant introduced agent-based scheduling and logistics optimization.

The agents managed travel and procurement well, but KSE's real breakthrough was using AI agents for recipe development. Competition cooking requires novelty — judges penalize repetition. KSE agents analyzed competition databases, identified flavor combinations that had never been presented, and generated candidate recipes that the human chefs could refine. This "scouting" approach treated recipe development like opponent analysis in sports.

The agents needed version control when recipe iterations started colliding. Two agents working on the same dish component would overwrite each other's flavor profiles. After losing a competition because an agent had silently reverted a seasoning adjustment, Park demanded a system where every change was tracked and every conflict was visible. GitButler's virtual branches matched their mental model: each dish component is a branch, and the final plate is the merge.

## Internal Tensions

**The "Coach vs. Players" divide.** Park runs the team like a head coach — she makes final calls on recipes, plating, and competition strategy. But her sous chef, Diego Reyes, believes the agents should have more autonomy during competition prep. "You can't coach every knife cut," he says. Park disagrees: "You can, and you should, until the cut is automatic." The tension plays out in their agent architecture: Park wants human approval on every patch; Reyes wants agents to commit freely during training sprints.

## Achievements

- Three-time Korean Culinary Grand Prix champions (2021, 2023, 2025)
- Silver at the Bocuse d'Or Asia-Pacific Selection 2024
- 147 novel flavor combinations identified and validated by AI-assisted scouting
- Zero missed competitions in five years of operation

## Signature Quirk

Every commit message ends with a timestamp in "kitchen time" — the time remaining in the current sprint, not the wall clock. A commit at 14 minutes remaining in a 20-minute sprint reads `[T-14:00]`. The team believes this keeps urgency visible. A commit at `[T-02:00]` carries implicit meaning: this was a last-minute decision under pressure. Review it carefully.

## Team Overview

| Agent | Role | Kitchen Position |
|-------|------|-----------------|
| Park | Head Chef / Strategist | Calls the menu, sets the game plan |
| Reyes | Sous Chef / Patch Lead | Produces patches at speed |
| Tanaka | Saucier / Memory | Flavor memory — tracks what has been tried |
| Obi | Pâtissier / Signer | Precision work, commit signing |
| Kwon | Expeditor / Coordinator | Cross-repo comms, timing |

---

*"The dish doesn't care about your process. It cares about the plate. Ship it."*
— Chef Yuna Park
