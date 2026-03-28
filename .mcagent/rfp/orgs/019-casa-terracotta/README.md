# Casa Terracotta

**"The old ways work. We just write them down now."**

---

## Family History

The Ferrau family has been repairing ceramics in Sardinia since the early 1600s. The family's workshop in Oristano has been in continuous operation for seventeen generations. The current head of the workshop, Margherita Ferrau, is 62 years old, learned the trade from her father, and can identify the origin century of a ceramic fragment by running her thumb across the glaze.

The Ferraus do not use commercial adhesives. They mix their own from recipes handed down through the family — recipes that are, as Margherita puts it, "not secret, just not written down." The primary adhesive is a lime-based putty reinforced with marble dust and ox-hair fibers, mixed in proportions that Margherita adjusts by feel depending on the humidity, the age of the ceramic, and the type of break. She has tried to teach the proportions to her apprentices by numbers. The numbers do not capture what her hands know.

The workshop repairs approximately 200 pieces per year, mostly for museums and private collectors. Their reputation rests on two things: the repairs are invisible (you cannot tell where the break was), and the repairs are reversible (the adhesive can be dissolved with water and vinegar without damaging the original).

## How Software Entered the Workshop

Margherita's nephew, Davide, studied materials science at the University of Cagliari and returned to Oristano in 2022 with an idea: document the family's adhesive recipes scientifically. He set up a small lab in the back of the workshop, analyzed the chemical composition of six traditional adhesive formulations, and published a paper that attracted attention from conservation professionals worldwide.

The paper led to consulting requests. Museums wanted to know: can you teach us your adhesive formulations? Davide built a database of recipes, repair techniques, and material compatibility charts. When the database grew too large for spreadsheets, he moved it to a Git repository — because Git tracked changes, and in conservation, knowing when a recipe was modified and by whom is as important as the recipe itself.

The AI agents came later. A partner museum asked if the database could recommend adhesive formulations automatically, given a ceramic type, break pattern, and environmental conditions. Davide built an agent that searched the recipe database and recommended the best match. Then another museum asked for an agent that could assess structural integrity from photographs. Then another asked for automated condition reporting.

Each agent needed its own configuration, memory, and history. Davide needed branching, isolation, and multi-agent coordination. He found GitButler.

## Philosophy

The Ferraus believe that technique outlasts technology. Their adhesive recipes are older than the concept of a "database." Their repair methods predate photography. The techniques work because they were refined over centuries by people who cared more about the result than the method.

We bring this philosophy to software: the technique matters more than the tool. A well-structured patch produced by a simple agent is better than a mediocre patch produced by a sophisticated one. We do not chase novelty. We refine what works.

## Internal Tension

Margherita and Davide argue about documentation. Margherita believes that some knowledge cannot be written down — that the feel of mixing adhesive, the judgment of matching a glaze color, the intuition of knowing when a repair is "right" are embodied skills that resist formalization. Davide believes everything can be documented if you measure carefully enough, and that failing to document is a risk (what happens when Margherita retires?).

They are both right. The workshop documents what it can and acknowledges what it cannot. The AI agents operate on the documented knowledge. The undocumented knowledge remains Margherita's.

## Notable Achievement

In 2024, the workshop repaired a 2,300-year-old Punic amphora that had been shattered into 147 fragments by an earthquake. Using the family's adhesive (modified by Davide with the addition of a modern UV-trace marker for future restorers), Margherita reassembled the amphora in 23 working days. The repair was exhibited at the Museo Archeologico Nazionale di Cagliari. The exhibition label notes: "Repaired by Casa Terracotta, Oristano. Techniques: seventeenth century. Materials analysis: twenty-first century."

## Team Overview

Four agents organized as a workshop. One master agent (Margherita-modeled) makes quality judgments and holds final authority. One apprentice agent handles primary implementation under the master's guidance. One materials agent manages recipes, memory, and context (Davide's domain). One liaison agent handles external communications (forge coordination, PR management). The workshop runs on craft norms: work is slow, quality is absolute, and the master's judgment is trusted above metrics.

---

*"A repair you can see is a repair you failed."*
— Margherita Ferrau
