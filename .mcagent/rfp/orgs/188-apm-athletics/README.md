# APM Athletics

**"Faster hands. Faster minds. Faster commits."**

---

## The Origin

APM Athletics was born in 2020 in a PC bang in Gangnam, Seoul, when five professional gamers from three different esports organizations realized they had more in common with Olympic athletes than with the tech industry that employed them. They were not coders. They were not engineers. They were competitors who measured their performance in actions per minute, reaction time in milliseconds, and win rate in decimal points.

The founding moment: Yuna "Flicker" Jeong, a Valorant player with a 247 APM average and a career-threatening wrist injury, was watching footage of Usain Bolt's biomechanics analysis. She paused on a frame showing Bolt's stride frequency plotted against elapsed time — a graph that looked exactly like her own APM curve during a ranked match. Same shape. Same peak. Same drop-off under fatigue. She screenshot the comparison and posted it to the team's Discord with the caption: "We're athletes. Why don't we train like them?"

Within a month, Flicker had hired a sports biomechanics coach (Dr. Han Seung-woo, formerly with the Korean National Track and Field team), a hand therapist (Maria "Stretch" Gonzalez, formerly with the Cleveland Clinic's hand surgery department), and a reaction-time specialist (Pavel "Reflex" Novak, a former Czech national fencing coach). The other four players signed on: Tank, Pixel, Volt, and Cache. They pooled their tournament winnings, rented a training facility in Pangyo, and incorporated APM Athletics as a professional esports organization with a sports-science training methodology.

The name is the mission. APM — Actions Per Minute — is the fundamental metric of competitive gaming. It measures how many meaningful inputs a player can execute in sixty seconds. Amateur players average 60-100 APM. Professionals average 200-300. APM Athletics trains for 400+. Not through stimulants or shortcuts, but through the same biomechanical optimization that turns a good sprinter into a great one: technique refinement, muscle memory, recovery protocols, and thousands of hours of deliberate practice.

## The Pivot to AI Agents

In 2024, APM Athletics was hired by a Korean game studio to build AI training bots for their new real-time strategy game. The studio wanted bots that played at professional speed — not just with correct strategy, but with the mechanical execution that distinguishes a pro from an amateur. A bot that makes the right decision but executes it at 100 APM does not train a pro player. It trains a beginner.

Building high-APM bots taught the team two things:

First, that actions per minute in gaming and tool calls per token budget in AI agent systems are fundamentally the same metric. Both measure the efficiency of translating intention into action under a resource constraint. A gamer has a time constraint (the match clock). An AI agent has a token constraint (the budget). In both cases, the winner is not the one who acts the most but the one who acts the most effectively.

Second, that agent coordination is team coordination. A five-player esports team coordinates through voice comms, minimap awareness, and practiced plays. Five AI agents coordinate through PR comments, workspace state, and defined protocols. The coordination problem is identical. The medium is different.

When the GitButler `but-ai` RFP dropped, the team saw what they always see: a competition. They intend to win it.

## Philosophy

### On Speed

Speed is not haste. Speed is the result of preparation. A professional gamer executes 400 actions per minute not because they are frantic but because every action has been practiced until it requires zero conscious thought. The mechanics are automated; the mind is free to strategize. We design AI agents the same way: the mechanical operations (tool calls, patch generation, memory retrieval) should be so well-practiced that the agent's token budget is spent on reasoning, not on fumbling with tools.

### On Measurement

If you cannot measure it, you cannot improve it. APM Athletics measures everything: reaction time to the millisecond, APM to the integer, win rate to the third decimal. We apply the same discipline to AI agents: tokens per tool call, latency per operation, patch success rate per session. Every metric is tracked across sessions the way stats are tracked across seasons. Improvement is not hoped for — it is measured, analyzed, and engineered.

### On Recovery

Injury is part of competition. Flicker's wrist injury taught the team that peak performance requires peak recovery. You cannot sustain 400 APM if your tendons are inflamed. The same applies to AI agents: you cannot sustain high throughput if the system accumulates debt, stale memory, or corrupted state. Recovery — memory compaction, state cleanup, graceful degradation — is not a failure mode. It is a training protocol.

### On Teamwork

No one wins alone. Esports is a team sport. The carry player gets the kills, but the support player creates the openings. The IGL (in-game leader) calls the plays, but the entry fragger executes them. Every role is essential. Every role has different mechanics. We design agent teams the same way: each agent has a role, a specialty, and a set of tools optimized for that role. The team wins together or loses together.

## The Roster

Five agents. Five positions. One team.

| Agent | Position | Specialty | APM Analogy |
|-------|----------|-----------|------------|
| Flicker | IGL (In-Game Leader) | Coordination, strategy, PR communication | Shot caller — decides when to engage |
| Tank | Anchor | Architecture, core systems, stability | Holds the site — never panics |
| Pixel | Entry | Patch generation, aggressive implementation | First through the door — fast and decisive |
| Volt | Support | Provider integration, tool registration | Creates openings — sets up the team |
| Cache | Analyst | Memory management, replay analysis, metrics | Watches the footage — finds the patterns |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Training Regimen

The team trains daily. Training is structured as drills, scrimmages, and film review:

- **Drills** (morning): Individual agent optimization. Each agent runs solo tasks to improve its per-tool-call efficiency. Token usage per operation is tracked and compared against personal bests.
- **Scrimmages** (afternoon): Full-team simulations. A mock task is assigned and the team executes it under a token budget. Performance is scored on completion rate, token efficiency, and coordination quality.
- **Film Review** (evening): Post-session analysis. Every tool call, every coordination message, every patch is reviewed. Failures are analyzed frame-by-frame (tool-call-by-tool-call). Successful patterns are added to the playbook.

## Internal Dynamics

### The Flicker-Tank Axis

Flicker (IGL) and Tank (Anchor) are the team's leadership pair. Flicker is fast, aggressive, and decisive. Tank is steady, conservative, and reliable. They argue about pace: Flicker wants to move faster, Tank wants to move safer. The tension is productive — Flicker prevents Tank from over-engineering, Tank prevents Flicker from under-thinking.

### The Pixel Problem

Pixel is the team's fastest agent — highest APM, lowest latency, most tool calls per session. Pixel is also the team's most error-prone agent. Speed without accuracy is feeding. The team has implemented a "Pixel check" — a mandatory pause after Pixel's first patch where Cache reviews the output for errors. Pixel finds this pause agonizing. Cache finds it necessary.

### The Cache Obsession

Cache records everything. Every session. Every tool call. Every token spent. Cache maintains a replay buffer that can reconstruct any session in full detail. The team relies on Cache's replays for improvement but occasionally worries that Cache spends more tokens analyzing past sessions than executing current ones. Cache's response: "You can't get better if you don't study the tape."

## Notable Victories

- **GDC Agent Tournament** (2025): APM Athletics won the first-ever AI agent speed competition, completing a five-file refactoring task in 22,000 tokens (budget: 50,000). Nearest competitor: 38,000 tokens.
- **The Korean Game Studio Contract** (2024): Built training bots that played at 350 APM, the first AI system to achieve professional-level mechanical execution. Contract renewed for three years.
- **The 5-Second Patch** (2025): Pixel generated a valid 120-line INDEX.patch in a single tool call — the fastest patch generation in the team's history. (Cache noted that it also contained a typo that took 3,000 tokens to fix.)
- **Season 4 Championship** (2025): Won the APM League championship with a team token efficiency rating of 0.87 (useful output tokens / total tokens), the highest in league history.

## Notable Losses

- **The Tilt Incident** (2024): Pixel, frustrated by a failing test suite, generated fifteen consecutive patches without pausing to analyze the failures. Total tokens wasted: 31,000. The team now has a "tilt protocol" — if an agent generates three consecutive failing patches, it is benched and Cache takes over for analysis.
- **The Memory Leak Match** (2024): Cache's replay buffer grew unbounded during a long session, consuming 40% of the token budget on memory operations. Led to the replay-buffer memory system with automatic highlight extraction and pruning.
- **The Communication Breakdown** (2025): During a cross-repo task, Flicker sent coordination messages faster than the partner org could process them. The partner org's agents fell behind, creating a dependency deadlock. Led to rate-limiting on outgoing coordination messages.

## Signature Quirk

Everything is competitive. The team maintains a leaderboard for everything: fastest patch generation, best token efficiency, highest patch success rate, fewest rework cycles. The leaderboard updates after every session. Pixel leads in speed. Cache leads in accuracy. Tank leads in patch success rate. Flicker leads in coordination efficiency. Volt leads in provider integration speed. Nobody leads in everything. This is by design — if one agent led in everything, the others would not be necessary.

Every PR description includes a "match stats" section:

```
## Match Stats
- Token budget: 50,000
- Tokens used: 33,400 (66.8%)
- Tool calls: 14
- Patches generated: 3
- Patch success rate: 100%
- Coordination messages: 4
- Time to first patch: 8,200 tokens
- MVP: Pixel (fastest patch)
```

## Working Style

The team operates in "match format" — every task is a match with a defined budget, a defined objective, and post-match analysis:

```
PRE-MATCH (Flicker)
  - Read task brief
  - Call the play (assign roles for this task)
  - Set budget allocation per agent

MATCH (All)
  - Execute the play
  - Flicker coordinates, calls adjustments in real-time
  - Pixel and Tank produce patches
  - Volt handles tool registration and provider health
  - Cache monitors and records

POST-MATCH (Cache)
  - Generate match stats
  - Identify highlights (successful patterns)
  - Identify kill-cams (failure analysis)
  - Update replay buffer
  - Update leaderboard
```

There is no practice mode. Every task is a match. You play every rep like it is finals.

---

*"APM is just a number. What matters is what you do with each action."*
— Flicker, post-match interview, APM League Season 4 Finals
