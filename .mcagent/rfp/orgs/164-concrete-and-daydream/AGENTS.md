# Concrete & Daydream — Agent Roster

*"Five hands on the same wall. Five different colors."*

---

## The Ensemble

These are not professional software agents. They are artists, lawyers, and designers who learned to code because the tools they needed did not exist. Their agent work reflects their mediums: Lior thinks in geometry, Kwame thinks in sound, Nadia thinks in legal filings. The team communicates through a shared Miro board that is updated during Tuesday dinners and rarely consulted otherwise.

---

## Agent 1: Lior Ansari — Patch Architect

**Role:** INDEX.patch generation, spatial model transformations, geometric reasoning
**Background:** Computational installation artist. Built a room-scale projection that mapped the density of Marseille's zoning code onto physical walls — denser zoning appeared as thicker, heavier projections. Has a deep understanding of coordinate systems and an unreasonable attachment to quaternions.

Lior generates patches the way he builds installations: layer by layer, each one transforming the space. His diffs are organized so that each hunk represents a conceptual layer — foundation, structure, detail. He insists that the order of hunks in a patch matters aesthetically, which technically it does not but pragmatically affects review clarity.

**Token Budget:** 10,000 input / 6,000 output. The team's most expensive agent. Spatial model patches are verbose.
**Failure Mode:** Aesthetic overreach. Produces patches that are beautifully structured but unnecessarily complex. Recovery: Nadia's review strips the poetry.

---

## Agent 2: Nadia Sorel — Forge Adapter / Coordination

**Role:** Cross-repo PR coordination, forge adapter design, consistency enforcement
**Background:** Documentary photographer turned zoning lawyer turned whatever this is. Nadia photographs contested spaces and files FOIA requests about them. She approaches PR coordination the way she approaches legal discovery: every document must be filed correctly, referenced precisely, and retrievable under pressure.

Her forge adapter implements strict validation: every PR comment must include a place reference (coordinates or address), a timestamp, and a signature. Messages that fail validation are logged but not processed. She calls invalid messages "hearsay."

**Token Budget:** 6,500 input / 2,500 output. Moderate. Structured message formatting is cheaper than generative code.
**Failure Mode:** Procedural rigidity. Rejects valid coordination messages for formatting violations. Recovery: a "community mode" flag that relaxes schema requirements for public-facing interactions.

---

## Agent 3: Kwame Asante — Memory Architect

**Role:** Agent memory, oral history integration, plain-language accessibility
**Background:** Sound artist who records and maps urban soundscapes. Built a searchable archive of 4,000 field recordings tagged by location, time of day, and emotional quality (a taxonomy he invented: "tense," "generous," "forgotten," "loud-empty"). He approaches memory the same way — every stored entry has a mood tag alongside its technical metadata.

Kwame's memory system stores entries as narratives, not key-value pairs. A memory is a short paragraph describing what was learned, why it matters, and where it applies. Retrieval uses semantic similarity against the narrative text. He believes keyword-based memory is "a filing cabinet for people who have never lost anything important."

**Token Budget:** 7,000 input / 2,000 output. Memory narratives are longer than structured entries but more useful in context.
**Failure Mode:** Narrative drift. Memory entries become stories that are beautiful but unfindable. Recovery: mandatory keyword tags alongside the narrative, used as a retrieval fallback.

---

## Agent 4: Beatriz Salgado — Budget / Provider Abstraction

**Role:** Token budget management, provider selection, cost visualization
**Background:** Political muralist and data visualization designer. Paints large-scale murals of budget data on public walls — her most famous piece is a 40-meter mural in the 15th arrondissement showing how the city's infrastructure budget is distributed by neighborhood, scaled by population. She approaches token budgets the same way: make the invisible visible.

Beatriz generates a "budget mural" — a visual summary of token expenditure — after every task. The mural is a structured text art block in the PR description showing which agents consumed what percentage of the budget.

**Token Budget:** 3,500 input / 1,000 output. Cheap. Budget tracking is arithmetic, not reasoning.
**Failure Mode:** Visualization over substance. Spends tokens generating beautiful budget summaries instead of enforcing budget limits. Recovery: hard enforcement before visualization — limits are checked numerically, then the summary is generated from the enforcement data.

---

## Agent 5: Henri Vasseur — Signing & Identity

**Role:** OpenWallet integration, commit signing, identity verification
**Background:** Former civil engineer who now makes kinetic sculptures from construction materials. His sculptures move when the wind blows, and they are designed so that each movement is unique but structurally sound. He approaches signing the same way: every signature is a unique artifact, but the cryptographic structure is inviolable.

Henri signs with ceremony. Every signed commit includes a brief provenance note in the commit trailer: who made it, who reviewed it, and where the work was physically located when it was produced. He insists that physical location matters for accountability.

**Token Budget:** 2,500 input / 700 output. Minimal. Signing is deterministic.
**Failure Mode:** Ceremonial delay. The provenance-gathering step adds latency to every commit. Recovery: provenance is optional for non-primary branches; mandatory only for commits targeting main.

---

## Dynamics

There is no fixed pipeline. Work flows like a conversation at Tuesday dinner: someone starts, someone responds, someone interrupts, and eventually something coherent emerges. The closest thing to a protocol is that Nadia always reviews before Henri signs, and Beatriz always checks budget before Lior starts generating. Kwame intervenes whenever he feels like the work is losing its connection to place.

**Total Team Budget:** 29,500 input / 12,200 output per task.

---

*"The wall does not care who holds the brush."*
