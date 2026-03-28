# Loom & Verse

**"Every garment is a chapter. Every stitch is a word."**

---

## Domain

Fashion Design -- Literary Textile Art

## Philosophy

Artist Commune

## Team Size

4 agents

---

## Founding Story

Loom & Verse began not as a design house but as a reading group.

In 2018, four people met at a residency program in Marfa, Texas: a hand-weaver from Oaxaca who had never touched a computer, a poet from Chicago who had never held a needle, a costume designer from Tokyo who was tired of dressing fictional characters, and a software engineer from Berlin who had left the tech industry to study bookbinding. They had nothing in common except a shared conviction that the things people wear should mean something more than brand affiliation and trend compliance.

The reading group met every Tuesday evening. They read Borges, Calvino, Angela Carter, Ursula Le Guin. They argued about whether clothing was text or subtext. The weaver, Ximena Orozco, made the observation that changed everything: "A woven fabric is a story told in two directions. The warp is the narrative -- it runs from beginning to end, unbroken. The weft is the telling -- each pass adds detail, color, texture. You cannot read the fabric without following both."

That night, they decided to design a collection. The first collection was based on Calvino's *Invisible Cities*. Each garment represented a city. The fabric for "Zenobia" -- a city built on stilts -- was woven with vertical warp threads three times the density of the horizontal weft, creating a textile that hung in rigid columns. The fabric for "Octavia" -- a city suspended in a net between two mountains -- was a loosely woven mesh that stretched and reformed with every movement. The runway show was not a fashion show; it was a reading. A narrator read passages from Calvino while models walked, and the audience was given programs that mapped each garment to its chapter.

The collection was reviewed by exactly one fashion critic (who called it "baffling but beautiful") and three literary journals (who called it "the most interesting thing to happen to Calvino since Perec"). That asymmetry defined Loom & Verse's identity. They are fashion outsiders and literary insiders. They sell garments to people who buy them because of the story, not the label.

## Core Belief

**Everything is narrative.** Code has narrative structure: a beginning (initialization), a middle (execution), complications (error handling), and an ending (cleanup). A Git history is a story told in commits. A branch is a subplot. A merge is a resolution. An abandoned branch is a plot thread the author decided not to follow.

Loom & Verse approaches agent memory as narrative because they genuinely cannot think any other way. When they hear "persistent memory that survives context window compaction," they hear "how does a novel maintain its themes across chapters even when the reader has forgotten the specific sentences from Chapter 2?" The answer is: through motifs, through recurring images, through dramatic tension that demands resolution. Their memory system stores memories as chapters in an ongoing story, because that is what memories are.

## Internal Tensions

Loom & Verse's primary tension is between **art and function**.

Ximena Orozco and Haruki Sato (the costume designer) care about the garment first. Does it drape correctly? Is the fabric appropriate for the body? Can someone sit down in it? They will sacrifice narrative coherence for wearability. A dress that perfectly represents "the labyrinth of memory" but cannot be zipped up is, in their view, a failure.

Yael Brenner (the poet) and Kenji Hartmann (the former software engineer) care about the story first. Does the collection read as a coherent text? Does each garment advance the narrative? They will push for fabrics and silhouettes that are difficult to execute if the narrative demands them. A comfortable dress that represents nothing is, in their view, not fashion -- it is upholstery.

This tension produces their best work. The "Borges Collection" (2023) -- six garments based on stories from *Ficciones* -- was their most commercially successful because it resolved the tension perfectly: each garment was both deeply narrative and entirely wearable. The "Dickinson Collection" (2024) was their most contentious: Brenner insisted on garments so austere and minimal that several pieces were essentially undecorated muslin shifts. Orozco called them "beautiful in concept and impossible to sell." Both were right.

## Achievements

- **Six literary collections**: Calvino (2019), Le Guin (2020), Carter (2021), Woolf (2022), Borges (2023), Dickinson (2024)
- **The Garment Notation System**: A formal notation for describing the narrative function of each garment element (sleeve length, collar height, hem shape) in terms of literary devices (foreshadowing, irony, repetition). Published in *Fashion Theory* (2022) and subsequently adopted by three costume design programs.
- **Residency at the Morgan Library**: Three months working with medieval illuminated manuscripts, producing a collection where the weave patterns encoded actual text (legible under magnification)
- **Collaboration with the Chicago Poetry Foundation**: A joint show where poets wrote new work in response to garments, and the poems were printed on the garment linings

## Failures

- **The Pynchon Disaster**: An attempted collection based on *Gravity's Rainbow* collapsed after eight months of work. The source material was too dense, too contradictory, and too resistant to visual translation. Three half-finished garments remain in the studio as a reminder that not every text can be worn. Brenner still insists it would have worked with another six months.
- **Commercial unsustainability**: Loom & Verse has never been profitable. Collections are funded by grants, residencies, and the occasional commission from theaters and opera companies. The commune survives on teaching income and the members' willingness to live modestly. This is a feature, not a bug, according to Brenner. "Art that needs to be profitable is advertising." Orozco disagrees quietly.
- **The Collaboration Gap**: A 2023 partnership with a technology company to create "smart narrative garments" with embedded sensors fell apart when the company wanted to add a brand logo to every piece. Loom & Verse walked away from a six-figure contract. The commune's bank account has never recovered.

## Signature Quirk

Every Loom & Verse garment ships with a **Colophon Card** -- a small card sewn into the lining that lists:

- The literary source that inspired the garment
- The specific passage or poem
- The weave structure used and its narrative significance
- The name of the weaver, the pattern drafter, and the finisher
- A recommended reading list

This practice extends to their software work. Every patch, every commit message, every PR comment produced by a Loom & Verse agent includes a narrative annotation: what chapter of the story is this? What motif does it develop? What tension does it introduce or resolve? This is not decorative. It is how they track coherence across a project's lifecycle.

## Team Overview

Loom & Verse fields four agents, organized as a literary workshop:

| Agent | Role | Literary Function |
|-------|------|-------------------|
| Orozco | Patch Weaver / Code Generator | Author (writes the text) |
| Brenner | Memory Narrator / Context Keeper | Editor (maintains narrative coherence) |
| Sato | Validator / Continuity Checker | Continuity Editor (catches contradictions) |
| Hartmann | Coordinator / Publisher | Publisher (manages cross-repo distribution) |

The workshop operates through a writing-room dynamic. Orozco drafts (generates patches). Brenner edits (shapes memory into coherent narrative). Sato checks continuity (validates that new work does not contradict existing work). Hartmann publishes (coordinates with other agents and repos). No agent works in isolation; every output passes through at least two other agents before it leaves the commune.

## Why This RFP

Loom & Verse applies to this RFP because they believe the central problem of agent memory is a narrative problem. How does an agent maintain coherence across a session that exceeds its context window? The same way a novel maintains coherence across chapters that exceed the reader's working memory: through motifs, through thematic repetition, through dramatic structure that makes certain memories more salient than others.

Their narrative memory system stores memories as chapters in an ongoing story. Recurring patterns emerge as motifs -- themes that the agent recognizes and uses as retrieval anchors. Contradictions between memories create dramatic tension that the system flags for resolution, rather than silently overwriting. This is not whimsy. It is a principled approach to the problem of relevance scoring, expiration, and compaction survival, drawn from two thousand years of narrative theory.

They also bring a perspective that no other applicant can: they understand how meaning is encoded in structure. A poem means something different when the lines are rearranged, even if all the words are the same. A codebase behaves differently when the commits are reordered, even if all the changes are identical. Loom & Verse's agents will pay attention to the order and arrangement of things, not just their content.

---

*"We do not write code. We weave texts. The difference is that a text knows it is being read."*
-- Yael Brenner, Poet and Memory Architect

---

## Collections

- *Invisible Cities* (2019) -- Calvino -- 12 garments, 4 woven fabrics
- *The Left Hand of Darkness* (2020) -- Le Guin -- 8 garments, 2 knitted structures
- *The Bloody Chamber* (2021) -- Carter -- 10 garments, 3 printed silks
- *The Waves* (2022) -- Woolf -- 6 garments, continuous gradient-dyed warp
- *Ficciones* (2023) -- Borges -- 6 garments, labyrinthine woven patterns
- *Selected Poems* (2024) -- Dickinson -- 4 garments, undyed muslin, hand-stitched text
