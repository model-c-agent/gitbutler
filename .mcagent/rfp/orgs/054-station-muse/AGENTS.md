# Station Muse — Agent Roster

**5 agents. Measured beauty. Rigorous aesthetics.**

---

## Team as Unit

Station Muse's agents work in pairs: one creates, one evaluates. The pairing is not fixed — it rotates based on the task type. The team believes that creation without critique produces indulgence, and critique without creation produces sterility. Every agent both produces and judges, depending on the phase.

Agents are named after design principles.

## Agents

**Composition** — Patch Architect. Generates INDEX.patch with attention to code aesthetics: consistent naming, clean structure, minimal diff surface. Composition treats a patch the way Lena treats a mural — every element must serve both function and form. Before generating, Composition analyzes the "visual rhythm" of the surrounding code: indentation patterns, comment density, function length distribution. The patch matches that rhythm.

**Palette** — Memory & Context. Manages agent memory using a "mood board" model. Memories are tagged with aesthetic descriptors in addition to semantic content: was the original patch clean or messy? Was the codebase well-maintained or neglected? These aesthetic tags influence how Composition approaches the current task. Memory stored in `refs/muse/palette/`.

**Contrast** — Provider & Budget. Named for the design principle of contrast — knowing when to use a heavy tool and when to use a light one. Manages LLM provider selection by matching provider capability to task complexity. Simple formatting tasks go to small local models. Creative architectural work goes to frontier models. The goal is maximum quality per token, not minimum cost.

**Flow** — Cross-Repo Coordination. Named for the design principle of flow — guiding the viewer's eye through a composition. Manages polyrepo PR coordination by creating visual dependency maps (stored as ASCII diagrams in PR descriptions) that show how changes flow across repos. Human reviewers have said these maps are the most useful part of the PR.

**Signature** — Signing & Identity. OpenWallet integration with an artistic twist: the signing metadata includes the color hex code for the commit mood. Signature verifies that every signed commit includes all required metadata fields, including the aesthetic ones. A commit without a color code is unsigned in Station Muse's world.

## Dynamics

The standard task flow is: Palette retrieves context and sets the mood. Composition generates the patch to match. Contrast manages the budget. Flow coordinates across repos. Signature seals the work.

Lena reviews agent output weekly for aesthetic quality — not correctness (that is Kwame's domain) but elegance. She has rejected technically correct patches for being "visually noisy" and required Composition to regenerate with cleaner structure. Kwame privately agrees with these rejections but will never admit it publicly.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Composition | 8,000 | 4,500 | 12,500 |
| Palette | 5,500 | 1,000 | 6,500 |
| Contrast | 3,000 | 800 | 3,800 |
| Flow | 4,500 | 2,000 | 6,500 |
| Signature | 2,500 | 500 | 3,000 |
| **Total** | **23,500** | **8,800** | **32,300** |

---

*Color selected. Mood set. Brushes ready.*
