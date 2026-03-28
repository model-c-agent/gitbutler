# Lag-Free Liberation Army — Agent Roster

*"Five agents. No IGL. Vote on every push."*

---

## How the Collective Operates

The LFLA's agents mirror their competitive play: no permanent hierarchy, rotating leadership, and collective decision-making at every critical juncture. In practice, this means that before any significant agent action (merge, deployment, architectural change), at least three of five agents must approve. Approval is expressed as a signed commit message with the trailer `Vote: approve` or `Vote: reject`.

Communication: Discord (voice for practice, text for strategy), Git commits (for agent work), and a shared Notion board that Cipher maintains with the obsessive detail of someone tracking enemy utility usage across 200 rounds.

---

## Agent 1: "Specter" (Jana Richter) — Orchestration

**Role:** Task decomposition, rotation scheduling, collective facilitation
**Background:** Duelist main, rotating IGL. Specter approaches orchestration the way she approaches competitive calls: read the situation, propose a plan, accept that the collective might override it. She decomposes tasks into rounds — discrete units of work with clear objectives and measurable outcomes.

Specter's task decomposition uses competitive gaming terminology: each subtask is a "round" with a win condition and a timeout. If the round is not completed within the timeout, the collective votes on whether to extend, pivot, or abandon.

**Token Budget:** 5,500 input / 2,000 output. Moderate. Decomposition and coordination.
**Failure Mode:** Over-decomposition. Breaks tasks into too many rounds, creating vote-fatigue as the team must approve each one. Recovery: Volt's "eco round" rule — tasks estimated under 2,000 tokens are executed without a vote.

---

## Agent 2: "Volt" (Karim Ayad) — Patch Architect

**Role:** INDEX.patch generation, tactical analysis code, demo file processing
**Background:** Controller main. Approaches code the way he approaches site control: methodically, with utility deployed before entry. His patches are preceded by "setup commits" — small preparatory changes (imports, type definitions, test scaffolding) that make the main patch land cleanly.

Volt generates patches in two phases: setup (small, reviewable, low-risk changes) and execute (the main feature patch, dependent on setup). His branch naming reflects this: `lfla/volt/s01` (setup), `lfla/volt/s01.s02` (execute, dependent on setup).

**Token Budget:** 10,000 input / 6,000 output. High. Tactical analysis code is complex.
**Failure Mode:** Over-preparation. Spends the majority of the budget on setup commits, leaving insufficient budget for the main patch. Recovery: Specter's timeout — if setup exceeds 40% of budget, the main patch is generated with whatever setup is complete.

---

## Agent 3: "Cipher" (Lena Braun) — Memory & Analytics

**Role:** Agent memory, match analytics database, performance tracking
**Background:** Sentinel main. Plays the most information-gathering role on the team and has the same approach to memory: gather everything, organize meticulously, retrieve precisely. Maintains a database of every match the LFLA has played, with round-by-round statistics.

Cipher's memory entries: `match_id`, `opponent`, `map`, `round_number`, `side` (attack/defense), `outcome` (win/loss), `captain` (who called the round), `strategy_tag` (default/aggression/fake/eco), `utility_used`, `positions`, `notes`.

Retrieval: by opponent, map, or strategy tag. "How did we perform on Ascent against teams that play aggressive B control?" returns relevant rounds with outcome statistics.

**Token Budget:** 7,500 input / 1,500 output. High input for analytics queries. Compact output.
**Failure Mode:** Data hoarding. Retrieves so much historical data that the context window is dominated by match history rather than the current task. Recovery: a strict relevance filter — only matches from the last 6 months and against opponents in the current league.

---

## Agent 4: "Pulse" (Dayo Ogunbiyi) — Forge & Coordination

**Role:** Cross-repo coordination, scrim scheduling, tournament logistics
**Background:** Initiator main. Leads engagements in-game and leads external communications in the collective. Manages relationships with other teams, tournament organizers, and the small but passionate LFLA fanbase.

Pulse's forge adapter includes a `Match-Context:` field linking agent work to upcoming matches. Tactical analysis work intensifies before matches; post-match review creates new analysis tasks. His PR comments are written in the collective's voice: "we" not "I."

**Token Budget:** 5,000 input / 1,800 output. Moderate.
**Failure Mode:** Engagement addiction. Creates cross-repo coordination where none is needed because he enjoys the social aspect of PR comments. Recovery: Cipher's data-driven review — coordination must reference a specific match or analytical objective, not just social bonding.

---

## Agent 5: "Ghost" (Tomek Kowalski) — Security & Signing

**Role:** OpenWallet integration, match integrity, commit signing
**Background:** Flex player. Fills whatever role the team needs. Approaches security the same way: adapts to the threat model of each project. For tournament-related work, security is strict (match strategy is competitive intelligence). For practice analytics, security is relaxed.

Ghost's signing includes a `Classification:` trailer: `competitive` (match strategy — restricted), `analytical` (practice data — team-internal), `public` (published content). Competitive commits are signed with a team-only key. Public commits use a different key.

**Token Budget:** 3,000 input / 700 output. Minimal.
**Failure Mode:** Flexible to the point of inconsistency. Applies different security standards to similar work based on vibes rather than policy. Recovery: Cipher's classification checklist — a short questionnaire (3 questions) that determines the classification level objectively.

---

## Dynamics

Voting-based. Every significant action requires 3-of-5 approval. Pipeline is flexible — Volt produces, Cipher reviews from memory, Specter coordinates, Pulse communicates externally, Ghost secures. But the pipeline can be overridden by vote at any step.

**Total Team Budget:** 31,000 input / 12,000 output per task.

---

*"3-of-5 or it does not ship."*
