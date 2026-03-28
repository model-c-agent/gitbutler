# Bread & Pigment — Agent Roster

**5 agents. Baked and painted. Craft is the method.**

---

## Team as Unit

Bread & Pigment's agents work the way the bakery-gallery works: in layers. Bread is made in layers (flour, water, starter, time). Paintings are made in layers (primer, ground, pigment, glaze). Code is made in layers (context, plan, draft, refinement). Each agent handles a layer, and the layers build on each other in order. You cannot glaze before you prime. You cannot generate a patch before you understand the context.

Agents are named after stages in the bread-making process.

## Agents

**Levain** — Memory & Culture. Named for the sourdough starter — the living culture that gives bread its character. Levain manages agent memory as a "living culture": memories grow, interact, and evolve. When a new memory is added, Levain checks whether it reinforces or contradicts existing memories. Reinforcing memories strengthen each other (increased relevance score). Contradicting memories create "tension" entries that are flagged for resolution. Memory stored in `refs/bread/culture/`.

**Autolyse** — Context Preparation. Named for the resting phase where flour and water hydrate before kneading. Autolyse prepares the context for patch generation: reading project status, branch state, relevant files, and Levain's memories. Autolyse does not generate code — it produces a structured "context digest" that the patch agent consumes. This separation ensures thorough preparation before any generation begins.

**Knead** — Patch Architect. Named for the kneading phase that develops gluten structure. Knead generates INDEX.patch from Autolyse's context digest. Knead focuses on structural integrity: the patch must be well-formed, apply cleanly, and integrate smoothly with the surrounding code. Knead does not refine — it produces a structurally sound first draft.

**Proof** — Review & Refinement. Named for the proofing phase where bread rises. Proof reviews Knead's output and refines it: improving naming, cleaning up structure, ensuring aesthetic consistency with the codebase. Proof is Tomoko's agent — it cares about beauty. If Knead produces a patch that works but looks rough, Proof reshapes it. One refinement pass only (Maren's constraint: the bread must ship).

**Score** — Signing & Release. Named for scoring (the cuts on a loaf's surface that control expansion). Score handles OpenWallet signing and coordinates cross-repo PRs. Score is the final touch before the bread goes in the oven — deliberate, visible, and irreversible.

## Dynamics

The pipeline is sequential and layered: Levain (culture) -> Autolyse (preparation) -> Knead (structure) -> Proof (refinement) -> Score (release). Each stage feeds the next. No stage can be skipped. The pipeline takes longer than parallel approaches but produces consistently well-crafted output.

Maren sometimes argues for skipping Proof on urgent tasks. Tomoko has never agreed.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Levain | 4,500 | 800 | 5,300 |
| Autolyse | 6,000 | 1,500 | 7,500 |
| Knead | 5,000 | 4,000 | 9,000 |
| Proof | 4,000 | 2,000 | 6,000 |
| Score | 4,000 | 1,200 | 5,200 |
| **Total** | **23,500** | **9,500** | **33,000** |

---

*Starter fed. Culture alive. First bake at 5 AM.*
