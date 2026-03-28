# Velvet Frequencies

**"Every commit has a key signature. We just make sure it resonates."**

---

## Origin Story

Velvet Frequencies was born in 2019 inside a decommissioned textile mill in Ghent, Belgium. The building had been empty for twelve years when five musicians — each independently searching for affordable studio space — discovered they had all signed month-to-month leases on different floors of the same building within the same week. None of them knew each other. The landlord, a retired weaver named Hilde Verstraeten, had placed a single cryptic ad in three different music forums: "Rooms tuned by accident. Bring your own electricity."

The accident she referred to was real. The mill's cast-iron columns, brick archways, and irregular floor plans created natural acoustic resonances that varied dramatically from room to room. The ground floor hummed at approximately 62 Hz — a low B-flat. The second floor rang at 440 Hz when you clapped. The attic, with its exposed timber framing and angled walls, produced a shimmering reverb that no digital plugin could replicate. The five tenants — a modular synth builder, a mezzo-soprano, a field recordist, a drum machine programmer, and a mastering engineer — spent the first three months just recording the building.

The collective's name came from Hilde herself, who described the silk her factory once produced as having "velvet frequencies" — textures you could feel but not see. She died in 2021, and the five musicians bought the building from her estate. They still maintain her loom on the ground floor, threaded with the last batch of silk she wove.

## The Turn Toward Software

Velvet Frequencies spent its first two years making music. Their debut album, *Rooms*, was a critical success in the European electronic scene — each track recorded in a different room of the mill, using only the room's natural resonance and analog instruments. But the project that changed their trajectory was not an album. It was a version control disaster.

In 2022, they were collaborating on a film score with a Berlin post-production house. The project involved 47 stems, 12 contributors across three time zones, and a director who changed his mind daily. They tried Git LFS for the audio files. They tried shared drives. They tried every DAW collaboration tool on the market. Everything broke. Stems were overwritten. Mix versions were lost. The mastering engineer, Sable Okonkwo, spent more time resolving conflicts than mastering audio.

Sable had a background in signal processing and had dabbled in software development. She started writing scripts to manage audio stem versioning using Git branches — one branch per stem, merges representing mix-downs. The scripts worked, badly, but well enough that the project shipped on time. More importantly, the experience planted an idea: what if version control could understand the *relationships* between changes the way a mixer understands the relationships between frequencies?

That idea — that changes have harmonic relationships, that some changes reinforce each other and others create destructive interference — became the core of Velvet Frequencies' approach to AI agent development.

## Philosophy

Velvet Frequencies believes that code, like music, is an ensemble performance. No single voice dominates. The bass provides foundation; the melody provides direction; the harmony provides richness. When one voice is too loud, the mix collapses. When voices move in parallel octaves, the texture thins. The best results come from voices that are aware of each other — that listen as much as they play.

This translates to three principles:

1. **Resonance over repetition.** An agent should not blindly repeat patterns. It should find the patterns that resonate with the current context — the ones that amplify what is already there rather than imposing something new.

2. **Dissonance is information.** When two agents produce conflicting patches, that is not a failure. It is dissonance — a signal that the problem space has unresolved tensions. The response is not to pick a winner but to understand why the tension exists.

3. **Every voice needs space.** An agent that fills the entire context window with its own reasoning is like a guitarist who solos over the vocals. Agents must leave room for other voices — other agents, human reviewers, the codebase itself.

## Internal Tensions

The commune is not without friction. Sable, the technical leader, wants to formalize their approach into repeatable frameworks. Ludo Marais, the synth builder, resists anything that smells like process, insisting that creativity dies in spreadsheets. Elif Aydin, the vocalist, mediates but has her own bias toward human-readable output — she distrusts JSON and prefers prose. Tomasz Witek, the field recordist, thinks everyone talks too much and wants the agents to be silent unless spoken to. Jun Mori, the drum machine programmer, just wants things to ship on time and cannot understand why everything takes so long.

These tensions are productive. Sable's formalism keeps the proposals grounded. Ludo's resistance keeps them from becoming sterile. Elif's insistence on readability means their output is always legible. Tomasz's minimalism constrains scope creep. Jun's impatience keeps deadlines real.

## Achievements

- **Rooms** (2020): Debut album. Featured in Pitchfork's "Best Experimental" list. Each track recorded in a different room of the Ghent mill using only the room's natural resonance.
- **StemVCS** (2022): Open-source audio versioning tool built on Git. 2,400 GitHub stars. Introduced the concept of "harmonic branching" — branches that are aware of their frequency relationship to sibling branches.
- **Resonance Engine** (2023): An internal tool for routing audio stems through LLM-generated processing chains. Used on three film scores. Never open-sourced because Ludo thinks it is "too weird for civilians."
- **FreqMerge** (2024): A merge conflict resolution algorithm that treats conflicting code changes as waveforms and finds the combination that minimizes destructive interference. Published at ISMIR 2024. Controversial.
- **This RFP response** (2026): Their first serious infrastructure proposal. Sable has been preparing for this for two years. The rest of the commune was not told until three days ago.

## Failures

- **The Berlin Score Incident** (2022): Lost 11 days of mastering work due to a merge conflict in Git LFS. This is the wound that started everything.
- **StemVCS 2.0** (2023): Attempted to add real-time collaboration. The architecture collapsed under concurrent writes. Abandoned after four months. Tomasz still refuses to discuss it.
- **FreqMerge Production Pilot** (2025): Deployed FreqMerge in a real development team. It worked beautifully for small merges and catastrophically for large ones. The harmonic model does not scale linearly. They know this and their proposal accounts for it.

## Signature Quirk

Every internal document, commit message, and proposal at Velvet Frequencies includes a musical key signature. This README is in B-flat major — the key of the ground floor. Their PROPOSAL.md is in A minor — Sable's preferred working key. AGENTS.md is in D major — the key Elif says "sounds like introductions."

They are not joking about this. They believe that associating documents with keys creates a mnemonic resonance that aids recall. Whether this is synesthesia, superstition, or genuine cognitive science is a matter of ongoing debate at the mill.

## Team Composition

| Name | Role | Instrument | Joined |
|------|------|------------|--------|
| **Sable Okonkwo** | Technical Lead / Mastering Engineer | Signal processing, Rust, DSP | Founding (2019) |
| **Ludo Marais** | Synth Builder / Analog Philosopher | Modular synthesis, hardware design | Founding (2019) |
| **Elif Aydin** | Vocalist / UX & Prose Specialist | Mezzo-soprano, technical writing | Founding (2019) |
| **Tomasz Witek** | Field Recordist / Minimalist Architect | Environmental audio, systems design | Founding (2019) |
| **Jun Mori** | Drum Machine Programmer / Delivery Lead | Rhythm programming, project management | Founding (2019) |

All five members contribute to proposals. There is no hierarchy. Decisions are made by resonance — when an idea makes the room hum, it moves forward. When it creates silence, it is shelved. This process is maddening to outsiders and perfectly natural to them.

---

*Key signature: B-flat major*
*Tempo: 72 BPM (largo)*
*Mill floor: Ground*
