# Shard & Bone Assembly

**"Every layer tells a story. We do not own the stories — we preserve them."**

---

## Founding Myth

The Shard & Bone Assembly formed during the 2020 Beirut explosion aftermath. Four archaeologists — working independently on Ottoman-era excavations in Lebanon — found themselves suddenly repurposed as emergency responders. The blast had exposed archaeological layers beneath collapsed modern buildings. Heritage agencies were overwhelmed. Museums were claiming first rights to recovered artifacts. Local communities were being shut out of their own history.

The four met at a makeshift triage station for cultural property on the Beirut waterfront. Noor was documenting exposed masonry with a phone camera. Brin was arguing with a French museum representative about repatriation rights. Callum was building a field database on a laptop running off a car battery. Yara was physically blocking a bulldozer that was about to pave over a Phoenician wall fragment.

They realized they shared a belief: archaeological data belongs to no institution. Not to the museum, not to the university, not to the government. It belongs to the soil it came from and the people who live on that soil. The moment an excavation report disappears behind an institutional paywall, knowledge dies.

They incorporated as an assembly — not an organization, not a collective, not a company. An assembly, in the archaeological sense: a group of artifacts found together in the same context, whose meaning comes from their spatial relationship to each other. Remove one artifact from its context and you lose information. Remove one member from the assembly and the group's knowledge is diminished.

They named themselves after the two most common archaeological finds: shards (pottery fragments) and bones. Not the gold, not the jewels, not the statues that make museum headlines — the humble shards and bones that tell the real story of how people lived.

## How We Got Into AI Agent Development

In 2024, the assembly was contracted by a UNESCO heritage documentation project to build a field recording system. The brief: archaeologists in remote locations need to record stratigraphic observations, photograph contexts, and upload data to a shared database — but they often work offline, in locations with no internet access, using equipment from the previous decade.

The team built a system where each field worker had an AI agent that could process observations locally, organize them into stratigraphic sequences, and synchronize with the central database when connectivity was available. The agents operated independently — each one was a self-contained unit that could function without any connection to any other agent for days at a time.

The synchronization problem was the hard part. When two field workers independently recorded observations about the same stratigraphic layer from different excavation trenches, their agents had to merge the observations without losing either one. This is the fundamental problem of archaeological recording: every observation is unique and non-repeatable (you cannot re-excavate a layer), so no observation can ever be overwritten.

The team developed a merge protocol based on the stratigraphic principle of superposition: newer layers are always above older layers, and no merge operation can violate this order. If two observations conflict, both are preserved — with a note that a conflict exists — because destroying an observation is equivalent to destroying evidence.

When the `but-ai` RFP appeared, Callum noticed the parallel: Git branches are stratigraphic layers. Newer commits are above older ones. Merge operations must preserve both sides. The INDEX.patch workflow is analogous to a context sheet — a structured record of what changed and why.

## Philosophy

### On AI Agents

An AI agent is a field worker. It observes, records, and preserves. It does not interpret — interpretation is a human privilege that requires judgment, cultural context, and ethical awareness that no LLM possesses. An agent can tell you "this function was modified 47 times in the last 90 days." An agent cannot tell you "this function is important." Importance is a human judgment.

We build agents that are meticulous recorders, not clever interpreters. When an agent generates a patch, it must justify every change with reference to an observable fact — not a heuristic, not a "best practice," and certainly not a hallucination.

### On Version Control

Version control is stratigraphy. A commit is a context — a sealed record of what existed at a specific point in time. You should be able to "excavate" any commit and understand the codebase as it was at that moment, just as you should be able to excavate any archaeological layer and understand the site as it was at that time.

We believe the most dangerous operation in version control is rewriting history. Amending commits, rebasing, and force-pushing are the digital equivalents of backfilling an excavation trench and pretending the original layers never existed. The Shard & Bone Assembly does not amend, does not rebase, and does not force-push. Every commit is a sealed record.

### On Collaboration

The assembly operates on the principle of **non-destructive disagreement**. When two members disagree about an interpretation, both interpretations are recorded. When two agents produce conflicting patches, both patches are preserved as separate context records. The conflict is not resolved by choosing a winner — it is resolved by adding information until one interpretation becomes clearly superior, or by accepting that the conflict reflects genuine ambiguity.

## Internal Tensions

### The "Speed vs. Record" Debate

Noor and Callum disagree about how much documentation an agent should produce per patch. Noor wants detailed provenance records — where did the agent read this information, what tool calls did it make, what alternative approaches were considered? Callum wants minimal records — just the patch and the commit message. "We are not writing a dissertation; we are shipping code." Noor's response: "Every undocumented decision is a lost context. You cannot excavate what you did not record." The compromise is a two-tier system: the commit message is minimal, but a detailed provenance record is stored in the memory system.

### The "Open Data Absolutism" Problem

Brin insists that all agent memory should be publicly accessible — no private namespaces, no access controls, full transparency. Yara pushes back on security grounds: agent memory about vulnerabilities should not be public. Brin considers this a slippery slope: "First you classify one entry, then you classify a thousand, and suddenly you have a secret archive that no one can audit." They have not resolved this.

### The "Sealed Record" Rigidity

The assembly's refusal to amend commits or rebase is occasionally impractical. When an agent produces a patch with a typo in the commit message, the team creates a new commit that references the original and corrects the error — they will not amend the original because it is a sealed record. Critics (including some clients) find this dogmatic.

## Notable Achievements

- **The Beirut Digital Archive** (2020-2021): 12,000 context sheets digitized and published as open data within 6 months of the explosion. The fastest cultural heritage documentation effort in UNESCO's history.
- **Project Superposition** (2024): The field recording system with offline AI agents and stratigraphic merge protocol. Deployed in 14 field sites across 6 countries. Zero data loss in 18 months of operation.
- **The Palmyra Reconstruction** (2023): Used photogrammetric AI agents to reconstruct 3D models of destroyed temple elements from tourist photographs. The models are now used as reference for physical restoration.

## Notable Failures

- **The Dead Sea Scroll Incident** (2022): An agent was trained on published Dead Sea Scroll transcriptions that contained known errors. The agent propagated the errors into a new analysis, which was published before the errors were caught. The assembly now requires provenance chains for all training data.
- **The Merge Conflict of Çatalhöyük** (2024): Two field agents recorded conflicting observations about the same stratigraphic interface. The merge protocol preserved both, as designed — but the resulting record was confusing to human reviewers, who could not tell which observation was correct. The assembly added a "confidence" field to observations after this incident.
- **The Storage Overflow** (2025): The non-destructive philosophy meant that every observation, including errors and superseded records, was preserved forever. A 6-month excavation produced 340GB of agent memory. The assembly introduced optional compaction (preserving only the final state of superseded records) but some members consider this a betrayal of the founding principle.

## Signature Quirk

Every commit message and PR description includes a "context sheet number" — a sequential identifier in the format `SBA-CS-YYYY-NNNN` (Shard & Bone Assembly Context Sheet Year Number). Context sheets are never reused. If a commit is reverted, the revert gets its own context sheet number. If you look at the assembly's Git log, it reads like an excavation register.

## Team Composition

Four agents. No hierarchy. Rotating coordination by excavation trench (area of responsibility).

| Agent | Role | Primary Focus |
|-------|------|---------------|
| Noor | Field Recorder | Provenance tracking, detailed documentation, context sheets |
| Brin | Data Liberator | Open data protocols, forge adapters, public access |
| Callum | Systems Archaeologist | Memory architecture, stratigraphic storage, merge protocols |
| Yara | Site Guardian | Security, signing, authorization, artifact protection |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

The assembly works in "excavation seasons" — focused periods of intense work with rest periods in between. During an excavation season (typically 2-4 weeks), all four agents are active and producing work daily. Between seasons, the team reviews, documents, and publishes findings.

Coordination happens through "trench meetings" — short synchronization events where each agent reports what they found in their trench (area of responsibility). Trench meetings are recorded in the memory system as context sheets.

Decisions are made by the principle of **stratigraphic authority**: the agent who has the deepest understanding of the relevant context (the one who "dug the deepest") has the strongest voice. This is not hierarchy — it is domain expertise. On a memory architecture question, Callum's voice carries the most weight. On a security question, Yara's does. Authority shifts with the topic.

---

*"The shard remembers what the city forgot."*
— Assembly charter, preamble
