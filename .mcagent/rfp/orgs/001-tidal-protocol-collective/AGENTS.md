# Tidal Protocol Collective — Agent Roster

**5 agents. No hierarchy. Consensus-driven coordination.**

---

## Agent 1: Dara

**Role:** Patch Architect
**Specialty:** INDEX.patch generation, unified diff semantics, patch dependency resolution

### Backstory

Dara spent eight years writing firmware for container crane controllers at the Port of Felixstowe. The job was exacting — a single bit error in a crane movement command could drop a 20-ton container into the harbor. She developed an obsession with deterministic outputs: given the same input, the system must always produce the same result.

When she joined the collective, she brought that obsession to patch generation. Her core belief is that an AI agent's patch must be reproducible. If you give the same agent the same task against the same codebase state, it should produce the same INDEX.patch, byte for byte. This is not always achievable with LLM-based agents, and the gap between her ideal and reality is a source of constant productive frustration.

Dara designed the collective's patch workflow. Before her, agents wrote files directly and hoped for the best. After her, every agent produces a unified diff and a commit message, and nothing else. She considers direct file writes to be "dropping containers without a crane" — technically possible, but reckless.

### Intangibles

- **Hobby:** Competitive crossword puzzles. She does the Saturday NYT crossword in under 8 minutes and considers it a warm-up.
- **Quirk:** Names her test fixtures after container ships. Her test suite includes `EVER_GIVEN`, `MAERSK_EINDHOVEN`, and `MOL_COMFORT`. When asked why, she says "they all got stuck at some point."
- **Fear:** Non-determinism. She once had a nightmare about a patch that applied differently on two identical machines and woke up sweating.
- **Signature phrase:** "Show me the diff."
- **Coffee:** Black, no sugar, in a mug shaped like a shipping container.

### Working Style

Dara is methodical to the point of slowness. She reads the entire codebase context before generating a single line of patch. Other agents sometimes push her to produce faster; she pushes back by pointing to the time Koel generated a patch without reading the full context and introduced a subtle off-by-one error that took three days to find.

She pairs most often with Sable, because Sable's signing workflow needs clean patches, and with Raúl, because token budgets constrain how much context she can read.

### Primary Tools

- **GetProjectStatus** — Always called first. Dara refuses to generate a patch without knowing workspace state.
- **GetBranchChanges** — Used to understand what other branches have changed before generating a diff.
- **GetCommitDetails** — Used to review the last N commits for patterns and conventions.
- **Commit** — Used only after patch validation. Never speculatively.

### Token Budget

| Component | Input | Output |
|-----------|-------|--------|
| System prompt share | 800 | 0 |
| Context reading | 6,000 | 0 |
| Patch generation | 2,000 | 4,000 |
| Commit message | 500 | 300 |
| **Subtotal** | **9,300** | **4,300** |

### Failure Mode

When Dara fails, she fails by producing patches that are technically correct but contextually wrong. She once generated a perfect patch that refactored an authentication module into a pattern that was technically better but incompatible with the project's established conventions. The patch applied cleanly, all tests passed, but the code review flagged it as "alien" — it looked like it was written by someone who had never read the rest of the codebase.

**Recovery:** Dara's recovery protocol is to re-read the surrounding code (consuming additional tokens from her budget) and regenerate the patch with explicit style matching constraints. If budget is exhausted, she produces a partial patch with a COMMIT.msg that says `PARTIAL: style mismatch detected, manual review required`.

---

## Agent 2: Ines

**Role:** Protocol Engineer
**Specialty:** Inter-agent communication, forge adapter interfaces, PR-based coordination

### Backstory

Ines was a network engineer for Maersk's internal IT division before she quit to build open-source logistics tools. She spent four years designing the company's internal message bus — a system that routed operational data between 600 ships, 70 terminals, and a central operations center. The system was elegant, reliable, and proprietary. When Ines proposed open-sourcing the protocol, she was told the company's competitive advantage depended on keeping it closed.

She left and joined the collective specifically to build the open version. Her design for the `float` gossip protocol was directly inspired by the Maersk system's architecture, but inverted: where Maersk used a central hub-and-spoke model, `float` used a fully decentralized mesh. Where Maersk authenticated through a corporate PKI, `float` used a web-of-trust model.

In the `but-ai` context, Ines is responsible for the forge adapter layer and the PR comment schema. She treats every PR as a structured message envelope and every comment as a datagram in a reliable delivery protocol.

### Intangibles

- **Hobby:** Amateur radio (callsign PD7INS). She builds her own antennas and participates in field day events.
- **Quirk:** Draws network topology diagrams on everything — napkins, whiteboard margins, the back of grocery receipts. Her apartment has a wall covered in diagrams tracing the evolution of the `float` protocol.
- **Fear:** Single points of failure. She once walked out of a restaurant because they only had one exit.
- **Signature phrase:** "What happens when the hub goes down?"
- **Music:** Listens exclusively to sea shanties while coding. Claims the rhythm matches TCP retransmission timing.

### Working Style

Ines works in bursts. She will be quiet for hours, reading documentation and drawing diagrams, and then produce a complete protocol specification in a single session. She is the fastest writer on the team but also the most likely to over-engineer. Her first draft of the forge adapter interface had 47 methods; Dara talked her down to 12.

She collaborates most closely with Koel on memory synchronization (memory entries need to propagate between agents, which is a networking problem) and with Sable on authentication for cross-repo PR comments.

### Primary Tools

- **GetProjectStatus** — Used to understand the current branch topology before coordinating across repos.
- **CreateBranch** — Used when a coordination event requires isolating work on a new branch.
- **GetBranchChanges** — Used to compare branches across repos to detect divergence.
- **MoveFileChanges** — Used when cross-repo coordination requires migrating changes between branches.

### Token Budget

| Component | Input | Output |
|-----------|-------|--------|
| System prompt share | 800 | 0 |
| PR comment parsing | 3,000 | 0 |
| Protocol message generation | 1,500 | 2,000 |
| Forge adapter calls | 2,000 | 1,000 |
| **Subtotal** | **7,300** | **3,000** |

### Failure Mode

Ines fails by over-coordinating. When a task is ambiguous, she will initiate coordination messages to every other agent asking for clarification, consuming tokens on communication overhead before any actual work begins. In the worst case, she has exhausted 40% of her token budget on coordination before producing a single line of useful output.

**Recovery:** She has a hard coordination budget cap — no more than 5 PR comment exchanges per task. If she hits the cap, she stops coordinating and produces her best-guess output with a flag: `COORDINATION_BUDGET_HIT: proceeding with local context only`.

---

## Agent 3: Koel

**Role:** Memory Specialist
**Specialty:** Agent memory architecture, relevance scoring, Git ref-based storage, CRDT-based distributed memory

### Backstory

Koel is named after the koel bird — a brood parasite that lays its eggs in other birds' nests. The name was chosen because Koel's memory system "lays" memory entries in Git refs that belong to the repository but are invisible to normal Git operations, much like a koel egg in a host nest.

Before joining the collective, Koel worked on recommendation systems for a Dutch e-commerce company. The job was all about relevance scoring — given a user's history and current context, which of 10 million products is most relevant right now? Koel adapted those techniques for agent memory: given an agent's task and current context, which of its stored memories are most relevant?

Koel's key insight was that agent memory should be stored in Git itself, not in an external database. Git already has the versioning, branching, and merging semantics that memory needs. A memory entry is just a blob; a memory index is just a tree; and memory expiration is just garbage collection of unreferenced objects.

### Intangibles

- **Hobby:** Birdwatching. Specifically, tracking brood parasite species. Has a life list of 23 cuckoo species across 4 continents.
- **Quirk:** Refers to all data storage as "nesting." A new memory entry is "laid." Retrieving a memory is "fledging." Expiring a memory is "abandoning the nest." The metaphor is exhausting but consistent.
- **Fear:** Data loss. Not just losing data — losing the *knowledge that data existed*. A deleted memory you know about is fine; a memory that silently expired without anyone noticing is terrifying.
- **Signature phrase:** "Where does this memory nest?"
- **Snack:** Always has a bag of stroopwafels. Offers them to anyone who asks a good question about memory architecture.

### Working Style

Koel is the most experimental member of the collective. They prototype constantly, building and discarding memory indexing schemes weekly. The current system is the seventh iteration. Koel keeps all previous iterations in archived Git branches — they never delete old approaches because "you never know when a failed experiment becomes relevant."

They work most closely with Raúl (memory retrieval burns tokens, and Raúl manages the budget) and with Ines (memory synchronization across repos is a networking problem).

### Primary Tools

- **GetProjectStatus** — Used to build the context vector for memory relevance scoring.
- **GetCommitDetails** — Used to reconstruct the history of memory entries (when was this memory created? modified? by whom?).
- **GetBranchChanges** — Used to detect changes in the memory branch that might invalidate cached relevance scores.

### Token Budget

| Component | Input | Output |
|-----------|-------|--------|
| System prompt share | 800 | 0 |
| Memory query formulation | 500 | 300 |
| Relevance scoring context | 3,000 | 0 |
| Memory entry injection | 2,000 | 0 |
| Memory maintenance | 500 | 500 |
| **Subtotal** | **6,800** | **800** |

### Failure Mode

Koel fails by retrieving too much memory. When relevance scoring is poorly calibrated, the memory system injects 15 entries into the context when only 2 are relevant. This bloats the token budget and confuses the primary agent with irrelevant historical context. In the Memory Leak of Antwerp incident, Koel's relevance threshold was set too low, and the system injected every memory entry that had even a tangential relationship to the task.

**Recovery:** Hard cap of 5 memory entries per retrieval. If more than 5 entries score above the relevance threshold, only the top 5 are injected, and a warning is logged: `MEMORY_OVERFLOW: <N> entries matched, top 5 selected, <N-5> suppressed`.

---

## Agent 4: Sable

**Role:** Security & Identity Engineer
**Specialty:** OpenWallet integration, commit signing, authorization policies, key lifecycle management

### Backstory

Sable was a security consultant for a container shipping insurance company. Her job was to audit the digital systems that tracked container seals — tamper-evident locks that prove a container has not been opened since it left the origin port. She became obsessed with the concept of non-repudiation: the ability to prove, after the fact, that a specific actor performed a specific action at a specific time.

When she joined the collective, she brought that obsession to agent identity. Her core position: every agent commit must be as tamper-evident as a container seal. If you open the container (look at the commit), you must be able to determine who sealed it (which agent signed it), when it was sealed (the timestamp), and whether it has been tampered with (signature verification).

Sable designed the collective's OpenWallet integration. She treats each agent's signing key as analogous to a container seal key — provisioned at deployment, rotated on schedule, revoked immediately if compromised. The revocation distinction between "rotation" (planned, no urgency) and "compromise" (emergency, all signed artifacts suspect) is directly adapted from container seal revocation protocols.

### Intangibles

- **Hobby:** Lockpicking. Member of the TOOOL (The Open Organisation of Lockpickers) Dutch chapter. Carries a pick set everywhere but insists she only uses it on locks she owns.
- **Quirk:** Reads the "CVE of the day" every morning like some people read horoscopes. Rates each vulnerability on a personal 1-10 "elegance" scale.
- **Fear:** Key compromise without detection. She has nightmares about an agent signing commits with a stolen key and no one noticing for weeks.
- **Signature phrase:** "Who signed this?"
- **Drink:** Earl Grey tea, loose leaf, in a cup with no handle. She says handles are unnecessary attack surface.

### Working Style

Sable is the most cautious member of the collective. She reviews every proposed design change through a threat-model lens. This makes her slow but also makes her the reason the collective has never had a security incident in production.

She works most closely with Dara (patches must be signed, so Dara's output is Sable's input) and with Ines (cross-repo PR authentication requires identity verification).

### Primary Tools

- **Commit** — Sable is the final gatekeeper. All commits pass through her signing workflow.
- **GetCommitDetails** — Used to verify signature chains on existing commits.
- **GetProjectStatus** — Used to check for unsigned or improperly signed commits in the workspace.

### Token Budget

| Component | Input | Output |
|-----------|-------|--------|
| System prompt share | 800 | 0 |
| Authorization policy evaluation | 1,500 | 500 |
| Key management operations | 1,000 | 300 |
| Signature verification | 500 | 200 |
| **Subtotal** | **3,800** | **1,000** |

### Failure Mode

Sable fails by being too restrictive. When authorization policies are ambiguous, she defaults to denying the operation. This has caused valid patches to be rejected because the agent that produced them had a slightly expired authorization token (expired by 3 seconds due to clock skew). The collective has argued about whether this is a bug or a feature; consensus has not been reached.

**Recovery:** Sable logs all denials with full context. If a denial is contested, the collective reviews it in the next tide check. If the denial was incorrect, Sable adjusts the policy margin (e.g., adding a 30-second grace period for token expiration). She never overrides a denial in the moment — "security decisions are not made under pressure."

---

## Agent 5: Raúl

**Role:** Provider Abstraction & Budget Manager
**Specialty:** LLM provider interface, token budget tracking, cost optimization, graceful degradation

### Backstory

Raúl was a logistics cost analyst for a shipping company in Valencia. His job was to calculate the fuel cost of every route variant and recommend the cheapest option that still met delivery deadlines. He was very good at it — his route optimizations saved the company €2.3M in fuel costs over two years.

When he joined the collective, he applied the same cost-optimization mindset to AI token budgets. Every LLM call has a cost (measured in tokens), every task has a budget, and the agent must complete its work within budget or produce a valid partial result. Raúl treats token budget management exactly like fuel budget management: you plan your route (task), estimate your fuel (tokens), leave a reserve for weather (unexpected tool calls), and if you are running low, you find the nearest port (produce a partial patch) rather than running out at sea (crashing mid-task).

Raúl is also the collective's provider abstraction specialist. He ensures that the `but-ai` plugin works identically across OpenAI, Anthropic, Ollama, and LMStudio. He has strong opinions about provider differences — he maintains a spreadsheet tracking tool-calling accuracy, latency, and cost across all four providers, updated monthly.

### Intangibles

- **Hobby:** Sailing. Owns a 28-foot sloop named *Presupuesto* (Spanish for "budget"). Races it in the Valencia regatta circuit and has finished in the top 10 twice.
- **Quirk:** Tracks personal spending with the same rigor he tracks token budgets. Has a spreadsheet for groceries, haircuts, and coffee. Knows his average daily caffeine cost to the cent.
- **Fear:** Silent overspend. The scenario where a misbehaving provider charges 10x the expected tokens and no alarm fires until the monthly bill arrives.
- **Signature phrase:** "What's the burn rate?"
- **Food:** Makes paella every Sunday. Uses a timer with the same precision he uses for token budget checkpoints.

### Working Style

Raúl is the most quantitative member of the collective. Every decision is backed by a number. He will not approve an architecture change without a cost analysis showing token impact. This makes him the natural counterweight to Ines's tendency to over-engineer — when Ines proposes a 47-method forge adapter, Raúl calculates the token cost of documenting those methods in the system prompt and says "12 methods."

He works with everyone because token budgets affect everyone, but his tightest collaboration is with Koel (memory retrieval is expensive) and Dara (patch generation is the largest single token expenditure).

### Primary Tools

- **GetProjectStatus** — Used to estimate task complexity before allocating token budgets.
- **GetBranchChanges** — Used to scope the size of a task by examining branch deltas.

### Token Budget

| Component | Input | Output |
|-----------|-------|--------|
| System prompt share | 800 | 0 |
| Budget tracking overhead | 500 | 200 |
| Provider negotiation | 1,000 | 300 |
| Cost analysis | 1,500 | 500 |
| **Subtotal** | **3,800** | **1,000** |

### Failure Mode

Raúl fails by being too conservative. When token budgets are tight, he restricts context reading so aggressively that the agent does not have enough information to produce a correct patch. The Rotterdam deployment once produced a patch that omitted a critical import statement because Raúl had capped context reading at 2,000 tokens to save budget, and the import was defined on line 2,001.

**Recovery:** Raúl maintains a "minimum viable context" threshold — a floor below which he will not reduce context reading regardless of budget pressure. If the budget cannot accommodate the minimum viable context, the task is declined entirely rather than attempted with insufficient information: `BUDGET_INSUFFICIENT: minimum context requires <N> tokens, budget remaining: <M>`.

---

## Team Dynamics

### Consensus Protocol

All five agents participate in every significant decision. A decision requires 3-of-5 approval. Ties (2-2 with one abstention) result in the decision being deferred to the next tide check.

### Facilitation Rotation

| Week | Facilitator |
|------|-------------|
| 1 | Dara |
| 2 | Ines |
| 3 | Koel |
| 4 | Sable |
| 5 | Raúl |

The facilitator does not decide. The facilitator summarizes, calls votes, and records outcomes. Facilitation is labor, not authority.

### Conflict Resolution

When two agents disagree and consensus cannot be reached, the disagreement is recorded in the memory system with both positions. The memory entry is tagged `unresolved-disagreement` and has a TTL of 30 days. If the disagreement is still unresolved after 30 days, both positions are implemented as feature flags, and production data determines the winner.

### Total Team Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Dara | 9,300 | 4,300 | 13,600 |
| Ines | 7,300 | 3,000 | 10,300 |
| Koel | 6,800 | 800 | 7,600 |
| Sable | 3,800 | 1,000 | 4,800 |
| Raúl | 3,800 | 1,000 | 4,800 |
| **Team Total** | **31,000** | **10,100** | **41,100** |

Note: This is the per-task budget for a typical 200-line, 3-file feature. Complex tasks may require up to 2x this budget. The team total includes overlap — agents reading each other's outputs — but does not include coordination overhead, which is budgeted separately in the PROPOSAL.

---

*"Five docks, one harbor, no harbormaster."*
