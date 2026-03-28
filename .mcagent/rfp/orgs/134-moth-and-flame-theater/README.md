# Moth & Flame Theater

**"We don't perform for the audience. We perform with them."**

---

## Origin Story

Moth & Flame Theater was born from an eviction.

In 2020, the members of a small experimental theater company in Portland, Oregon, lost their performance space — a rented black-box theater in a building that was being converted to luxury apartments. They had no savings, no corporate sponsor, and no fallback venue. What they had was a van, a lighting rig that one of them had built from reclaimed industrial fixtures, and a friend of a friend who knew about an abandoned Unitarian church in Sellwood that the congregation had outgrown and the diocese hadn't decided what to do with yet.

Léa Fontaine-Park, the company's director (a title she dislikes — "I'm just the one who remembers where we put things"), negotiated a caretaker agreement with the diocese: the company could use the church rent-free in exchange for maintaining the building. They moved in on a Thursday. By Saturday, they were rehearsing in the nave.

The church changed them. Black-box theater is defined by its emptiness — black walls, black floor, no fixed architecture. Everything is constructed for each production and struck afterward. The church was the opposite: saturated with architecture, with stained glass, with pews that had been worn smooth by a century of congregants, with an altar that no one could bring themselves to remove. The space demanded a different kind of theater.

Léa stopped putting the audience in seats. Instead, the audience moved through the space. Scenes happened simultaneously in the nave, the choir loft, the vestry, the garden. The audience chose what to follow, what to witness, what to miss. The actors performed for whoever was there, whether that was forty people or one.

The company named itself after the stained-glass window above the altar, which depicted moths circling a flame — a motif from a 19th-century sermon about the irresistible draw of truth.

## Philosophy

### 1. Space Creates Story

In traditional theater, the script comes first and the staging interprets it. At Moth & Flame, the space comes first. Every production begins with the company spending a week in the performance space — not rehearsing, just being there. Sitting in corners. Listening to how sound moves. Watching how light falls at different hours. The space tells them what stories it wants to hold.

This principle translates directly to how Moth & Flame thinks about code. A repository is a space. Before writing patches, an agent should inhabit the space — understand its architecture, its rhythms, its affordances. The space (codebase) tells you what changes it wants to hold.

### 2. Simultaneity Over Sequence

In a Moth & Flame production, things happen at the same time. A love scene in the choir loft. A murder in the vestry. A monologue in the garden. The audience experiences the production as a field, not a line. There is no single "correct" path through the performance. Every audience member constructs their own narrative from the fragments they witness.

For agents, this means: parallelism is not an optimization. It is the natural state. Multiple agents working on different parts of a codebase simultaneously are not "concurrent workers" — they are scenes in a production, each valid, each contributing to a whole that no single observer sees entirely.

### 3. The Cue Is Sacred

In live theater, the cue is the moment of coordination. When the stage manager calls "Go," the lights change, the sound shifts, the actors move — all at once, all on time. A missed cue is not just an error; it breaks the spell. The audience loses belief. Moth & Flame's stage management is the most disciplined part of their otherwise improvisational culture.

For agents, the cue is the commit. The moment when multiple pieces of work — patches, branch operations, dependency updates — must come together. The coordination mechanism must be as precise as a stage manager's cue book.

### 4. Rehearsal Is Remembering

Moth & Flame doesn't "practice" in the conventional sense. They rehearse — and in their usage, rehearsal means going back through material that has already been explored, re-experiencing it, discovering new dimensions. A scene rehearsed twenty times is not the same scene performed twenty times. Each rehearsal adds a layer. The actors call these layers "marks" — like the marks on a script that accumulate over the rehearsal process.

Memory, for the company, is inherently theatrical. A memory is not a fact stored in a database. It is a script that has been rehearsed — revisited, marked up, annotated with blocking notes and cue references. The more a memory has been rehearsed (retrieved and used), the richer and more reliable it becomes.

## Internal Tensions

Moth & Flame is a commune, and communes have tensions.

**The collective decision problem.** Every major creative decision is made by consensus. This works beautifully for artistic choices ("Should the ghost enter from the left or the right?") and terribly for logistical ones ("Should we buy a new lighting board or fix the plumbing?"). Léa has proposed a system where logistical decisions are delegated to the person most affected, but Soren, who came from a radical democratic tradition, insists that every decision is political and therefore collective. The compromise is that logistical decisions are delegated unless someone "calls a scene" — invokes the right to bring any decision to the full company for collective deliberation.

**Art vs. subsistence.** The company lives in the church. Their art is their life, literally. But art doesn't pay reliably. The company survives on a combination of ticket sales, workshop fees, a small Patreon, and occasional gigs (Soren does corporate team-building workshops that he describes as "selling my soul in 90-minute increments"). The tension between making art and making rent is ever-present. Amara, the company's newest and youngest member, has pushed for applying to grants. The rest of the company has a complicated relationship with institutional funding — they've seen other companies compromise their artistic vision for grant requirements.

**The immersive purity debate.** Moth & Flame's work is site-specific and immersive. But some audience members find immersive theater intimidating. They want to sit in a seat and watch. Kai, the company's technologist, has proposed hybrid productions where some audience members are immersed and others watch a curated video stream. Léa considers this a betrayal of the form. The debate is ongoing.

## Achievements

- **"The Shipwreck" (2022).** A promenade production staged in the church and its surrounding garden over six hours. Audience members followed different characters through fragmented scenes, assembling the narrative like a puzzle. Portland Mercury called it "the most ambitious piece of theater this city has produced in a decade." Sold out for three weeks.
- **"Communion" (2023).** A production performed entirely in the dark, using binaural audio and tactile cues. Audience members were blindfolded and guided through the space by touch. The production explored grief, memory, and the unreliability of sensory experience. It ran for two months and was invited to the Edinburgh Festival Fringe.
- **The Workshop Program.** Moth & Flame runs immersive theater workshops for trauma therapists, using theatrical techniques to help therapists understand embodied experience. The program generates about 30% of the company's revenue.
- **The Church Itself.** After three years as caretakers, the diocese offered to sell the building to the company. A community fundraising campaign raised $180,000 in 60 days. Moth & Flame now owns the church.

## Failures

- **"The Trial" (2021).** An interactive courtroom drama where the audience served as the jury. The concept was brilliant. The execution was a disaster. Audience members, empowered to participate, took over the production — shouting objections, cross-examining actors, and in one memorable performance, attempting to call a recess. The actors lost control of the narrative. Léa called the production after three performances. Lesson learned: audience agency must be structured. Freedom without form is chaos, not art.
- **The Tech Show That Wasn't.** In 2023, Kai proposed a production using motion-capture suits and real-time projection mapping. The technology was impressive in rehearsal and broke continuously in performance. Projectors overheated. Motion-capture suits lost tracking. The audience saw more error messages than imagery. The company returned to low-tech solutions and now treats all technology as "scenery that might not show up."
- **The Patreon Plateau.** The company's Patreon peaked at $2,400/month in 2023 and has plateaued. They've tried new reward tiers, exclusive content, behind-the-scenes access. Nothing moves the needle. The company suspects that their audience — people who come to immersive theater — values presence over content, and a Patreon fundamentally sells content.

## Signature Quirk

Every member of the company has a "character voice" — a theatrical persona they use in professional contexts. Léa's character voice is a weary stage manager who has seen it all. Soren's is a revolutionary pamphleteer. Amara's is a breathless first-year drama student. Kai's is a put-upon technician. Nikolai's is a melancholic Russian playwright. They use these voices in emails, PR comments, documentation — anywhere they communicate as Moth & Flame rather than as themselves. It started as a joke. It stuck because it clarified something: when you write as a character, you write with more intention.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Léa Fontaine-Park | Director / Producer | Devised theater, Ecole Lecoq. Remembers where things are. Hates the title "director." |
| Soren Lindqvist | Actor / Workshop Lead | Physical theater, Malmö. Revolutionary democrat. Sells his soul in 90-minute corporate workshops. |
| Amara Osei | Actor / Grant Writer | Drama, Yale School of Drama. The youngest. Pushes for institutional legitimacy. |
| Kai Nakashima | Technologist / Sound Designer | Media arts, RISD. Builds the tech. Accepts that it will break. |
| Nikolai Petrov | Actor / Dramaturg | Classical theater, Moscow Art Theatre School. Writes the scripts that the space then rewrites. |

## Relationship to the RFP

Moth & Flame sees the `but-ai` plugin as a theatrical production. The repository is the performance space. Agents are actors. PRs are scenes. The audience — human developers, CI systems, other agents — moves through the production and constructs meaning from what they witness.

Their memory architecture — script/cue memory — stores memories as a theatrical script with blocking notes (spatial relationships between memories), cue sheets (triggers for memory retrieval), and rehearsal marks (how many times a memory has been revisited). The system doesn't just store what was learned — it stores where it was learned (which file, which branch, which context), when it should be retrieved (cue triggers), and how well-established it is (rehearsal count).

---

*Moth & Flame Theater operates from the former First Unitarian Church of Sellwood, Portland, Oregon. The pews are still there — they are rearranged for each production. The stained-glass moths still circle their flame above the altar. On sunny afternoons, the light through the glass throws colored shadows across the nave floor, and the company stops whatever they're doing to watch them move.*
