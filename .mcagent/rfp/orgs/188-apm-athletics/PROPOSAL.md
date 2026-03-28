# PROPOSAL.md — APM Athletics

**Match Type:** RFP Championship
**Map:** GitButler but-ai Plugin
**Budget:** Unlimited (but we're tracking every token like it's ranked)
**Objective:** Win.

---

## Pre-Match Brief

Listen up. We've studied the map. We've watched the tape. We know the terrain.

The GitButler codebase has 70+ crates, a mature tool system, and a provider abstraction that is already 80% of where it needs to be. The existing MCP server is a single-tool prototype. The ten workspace tools are solid but disconnected from the AI layer. The patch-based workflow is the right primitive. The coordination protocol is undefined. The memory system does not exist yet.

Here is how we win this.

---

## 1. Plugin Architecture (RFP 3.1) — Map Control

### The Play

We drop a Rust crate into the workspace. One crate. Clean entry. No sprawl.

### Execution

```
crates/but-ai/
  src/
    main.rs              -- Spawn point
    arena/
      mod.rs             -- Match engine: agent coordination loop
      budget.rs          -- Economy tracking (token budget)
      plays.rs           -- Playbook: named coordination patterns
    agents/
      mod.rs             -- Agent roster and role definitions
      flicker.rs         -- IGL: strategy and coordination
      tank.rs            -- Anchor: architecture and core patches
      pixel.rs           -- Entry: rapid patch generation
      volt.rs            -- Support: provider and tool management
      cache.rs           -- Analyst: memory and replay
    mcp/
      mod.rs             -- MCP server (ServerHandler impl via rmcp)
      bridge.rs          -- WorkspaceToolset registration
    forge/
      mod.rs             -- Forge adapter trait and GitHub impl
      schema.rs          -- PR comment schema
    replay/
      mod.rs             -- Replay-buffer memory system
      buffer.rs          -- Session recording
      highlights.rs      -- Key moment extraction
      killcams.rs        -- Failure analysis extraction
  Cargo.toml
```

The crate is the arena. Everything happens inside it. The MCP server is the spectator interface — it lets external tools watch and interact with the match.

### CLI Mode vs MCP Mode

`but ai agent <task>` — Run a match. The team executes the task under the token budget.
`but ai replay [session-id]` — Watch the tape. Cache's replay system.
`but ai stats` — Season stats and leaderboard.
`but ai mcp` — MCP server mode. Spectator interface.

### WASI

Under WASI, no plugin discovery. The MCP server still works via direct invocation (`but-ai mcp`). The agents still run — they just cannot be called through `but ai`. Like playing an away match in an unfamiliar arena: the game is the same, the entrance is different.

### Git Config Keys

| Key | Default | What It Does |
|-----|---------|-------------|
| `but-ai.budget.match` | 50000 | Token budget per match (session) |
| `but-ai.budget.reserve` | 5000 | Reserve budget for Cache's post-match analysis |
| `but-ai.replay.baseRef` | `refs/but-ai/replay` | Replay buffer storage |
| `but-ai.forge.adapter` | `github` | Active forge |
| `but-ai.forge.github.token` | (none) | GitHub API token |
| `but-ai.provider.pluginDir` | `~/.local/share/but-ai/providers` | External providers |
| `but-ai.team.tiltThreshold` | 3 | Consecutive failures before tilt protocol |

### Trade-offs

| Call | Made | Skipped | Reason |
|------|------|---------|--------|
| Workspace crate | Yes | Standalone binary | Shared types = faster coordination |
| Feature flag in but | No | - | Violates RFP 4.6 — illegal play |
| Multi-crate split | No | - | Over-engineering for this team size |

---

## 2. Provider-Agnostic AI Interface (RFP 3.2) — Loadout

### The Play

Use `but-llm` as-is. Four providers, four loadouts. Same weapons, different skins.

### Execution

Volt handles provider setup at match start. `LLMProvider::from_git_config()` selects the active provider. `tool_calling_loop_stream` is the primary interaction mode — the streaming callback feeds Cache's token tracker, giving us real-time economy awareness.

#### Provider Plugins

New providers load from shared libraries in `but-ai.provider.pluginDir`:

```rust
pub trait ProviderPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn supports_tool_calling(&self) -> bool;
    fn execute(
        &self,
        system: &str,
        messages: Vec<ChatMessage>,
        tools: &[ToolSchema],
        on_token: Option<Box<dyn Fn(&str) + Send>>,
    ) -> Result<ProviderResponse>;
}
```

Volt validates the plugin at load time: check ABI version, verify capabilities, run a health check. A plugin that fails the health check is not loaded — like a player who fails the physical.

#### MCP Tool Surface

All ten workspace tools, plus:

| Tool | Purpose | Agent |
|------|---------|-------|
| `match_status` | Current match state, scores, budget | Flicker |
| `replay_query` | Search replay buffer by event type | Cache |
| `team_stats` | Season metrics and leaderboard | Cache |
| `agent_roster` | Active agents and their roles | Flicker |

### Trade-offs

Considered gRPC for providers. Rejected — adds latency. Every millisecond of latency is an APM reduction. We are not here to reduce APM.

---

## 3. The But Agent (RFP 3.3) — The Match

### The Play

Five agents. Five positions. One match engine that coordinates them.

### Match Engine

The match engine is the core loop that drives agent execution:

```rust
pub struct MatchEngine {
    roster: TeamRoster,
    budget: TokenBudget,
    playbook: Playbook,
    replay: ReplayBuffer,
}

impl MatchEngine {
    pub fn run_match(&mut self, task: Task) -> MatchResult {
        // PRE-MATCH: Flicker reads task, calls play
        let play = self.roster.flicker.call_play(&task, &self.budget);

        // SETUP: Volt configures infrastructure
        self.roster.volt.setup_infrastructure();

        // MATCH: Execute play
        loop {
            // Each round: agents execute their role
            let round = self.execute_round(&play);

            // Cache records
            self.replay.record(round);

            // Flicker checks tempo
            if self.roster.flicker.should_adjust(&self.budget) {
                play = self.roster.flicker.adjust_play(&play, &self.budget);
            }

            // Budget check
            match self.budget.status() {
                Green => continue,
                Yellow => self.roster.flicker.two_minute_warning(),
                Red => break,  // GG play
                Final => break, // Hard stop
            }

            // Completion check
            if round.task_complete { break; }
        }

        // POST-MATCH: Cache analyzes
        let analysis = self.roster.cache.post_match_analysis(&self.replay);

        MatchResult { patches, analysis, stats: self.budget.final_stats() }
    }
}
```

### Patch Production

Pixel and Tank are the patch producers. Tank produces the architectural foundation first (if needed). Pixel produces the implementation patches on top. Neither agent makes direct file edits. Neither calls `git commit` or `but commit`. They produce INDEX.patch + COMMIT.msg. That is the only write primitive.

Pixel's patches come fast and may need iteration. Tank's patches come slow and never need iteration. Flicker decides who goes first based on the task profile:

- **Architectural task** (new module, refactor): Tank first, Pixel builds on Tank's foundation.
- **Implementation task** (add feature to existing module): Pixel first, fast output, Tank reviews architecture.
- **Speed task** (simple change, tight budget): Pixel solo, Rush play.
- **Critical task** (zero tolerance for error): Tank solo, Clutch play.

### Branch Naming

```
apm/<play-name>/<agent>/<dependency-chain>
```

Examples:
- `apm/default/tank/s01` — Tank's foundation, step 1
- `apm/default/pixel/s01.s02` — Pixel's implementation, step 2, depends on Tank's s01
- `apm/rush/pixel/s01` — Pixel's speed run, step 1 (Rush play = Pixel only)

### Budget Enforcement

Cache tracks the budget in real time. Token counts are updated at every tool call boundary via the streaming callback. Budget status:

| Status | Threshold | Action |
|--------|-----------|--------|
| **Green** | < 70% used | Full play — all agents active |
| **Yellow** | 70-85% | Two-minute warning — Flicker reassesses strategy |
| **Red** | 85-95% | GG play — wind down, produce partial results |
| **Final** | > 95% | Hard stop — output whatever is complete |

A reserve budget (default 5,000 tokens) is set aside for Cache's post-match analysis. This reserve is not available to other agents. You do not cut the analyst's budget — you cut the entry fragger's budget. Pixel adapts. Cache's analysis improves the next match.

### Progress Reporting

Every match produces structured output:

```json
{
  "match_id": "apm-2026-0328-001",
  "play": "default",
  "status": "complete",
  "rounds": 4,
  "agents": {
    "flicker": { "calls": 6, "tokens": 8200 },
    "tank": { "calls": 4, "tokens": 10800 },
    "pixel": { "calls": 7, "tokens": 13200 },
    "volt": { "calls": 3, "tokens": 4100 },
    "cache": { "calls": 5, "tokens": 7300 }
  },
  "patches": [
    { "agent": "tank", "lines": 80, "success": true },
    { "agent": "pixel", "lines": 145, "success": true }
  ],
  "budget": { "total": 50000, "used": 43600, "reserve": 5000, "remaining": 1400 },
  "mvp": "tank"
}
```

### Trade-offs

Five agents costs more tokens than one. We know. The overhead is approximately 8,000 tokens (16%) for coordination, support, and analysis. But the output quality is higher, the error rate is lower, and the post-match analysis produces improvements that compound across sessions. A solo queue player might hit a higher peak, but a coordinated team wins the tournament.

---

## 4. Polyrepo PR-Based Agent Coordination (RFP 3.4) — Comms

### The Play

Flicker handles all external communication. PRs are voice comms channels between teams. PR comments are callouts.

### Forge Adapter

```rust
pub trait ForgeAdapter: Send + Sync {
    fn create_pr(&self, params: &PrParams) -> Result<PrRef>;
    fn post_callout(&self, pr: &PrRef, msg: &Callout) -> Result<()>;
    fn read_callouts(&self, pr: &PrRef) -> Result<Vec<Callout>>;
    fn get_match_status(&self, pr: &PrRef) -> Result<PrStatus>;
    fn set_labels(&self, pr: &PrRef, labels: &[&str]) -> Result<()>;
}
```

GitHub reference implementation. Stateless. Each call authenticates independently. No persistent connections — we do not hold angles we are not going to peek.

### Callout Schema

PR comments carry structured callouts:

```json
{
  "schema": "apm-athletics/comms/v1",
  "callout_type": "assignment | status | dependency | handoff | economy",
  "sender": { "agent": "flicker", "team": "188-apm-athletics" },
  "timestamp": "2026-03-28T14:00:00Z",
  "urgency": "routine | flash | critical",
  "payload": { ... }
}
```

**Assignment callout:**
```json
{
  "target_agent": "tank",
  "objective": "Build the auth module foundation",
  "budget_allocation": 12000,
  "play": "default"
}
```

**Economy callout:**
```json
{
  "budget_total": 50000,
  "budget_used": 33400,
  "budget_remaining": 16600,
  "projected_completion": true,
  "play_adjustment": "none"
}
```

### Cross-Repo Coordination

Dependencies tracked in `refs/but-ai/replay/deps.json`:

```json
{
  "active_matches": [
    {
      "home": { "repo": "gitbutler/but-ai", "pr": 7 },
      "away": { "repo": "gitbutler/but-tools", "pr": 42 },
      "status": "in_progress",
      "dependency_direction": "home_depends_on_away"
    }
  ]
}
```

Flicker checks dependencies at the start of each round. If a dependency is unresolved, she adjusts the play — skip the blocked work, focus on independent tasks, come back when the dependency lands.

### Trade-offs

| Option | Verdict | Why |
|--------|---------|-----|
| Webhooks | Rejected | Requires a server — extra infrastructure = extra latency |
| Polling | **Selected** | Stateless, works everywhere, no dependencies |
| Message queue | Rejected | Proprietary dependency — disqualifying |

---

## 5. Agent Memory and Identity (RFP 3.5) — The Replay Buffer

### The Play

Memory is game tape. We store sessions as replayable recordings, extract highlights for quick reference, create kill-cams for failure analysis, and prune routine footage to keep the buffer lean.

### Design

This is **replay-buffer memory**: sessions stored as sequences of events that can be rewound, fast-forwarded, and analyzed frame-by-frame. The replay is not a log — it is a structured, queryable, indexed recording of everything that happened.

#### Storage Structure

```
refs/but-ai/replay/
  sessions/
    <session-id>.json      -- Full session replay
  highlights/
    <highlight-id>.json    -- Extracted key moments (successes)
  killcams/
    <killcam-id>.json      -- Extracted failure sequences
  stats/
    season.json            -- Aggregate season statistics
    leaderboard.json       -- Agent leaderboard
  identity/
    <agent-id>.json        -- Agent identity records
  deps.json                -- Cross-repo dependency map
```

#### Session Replay Format

A session replay is a sequence of frames, each representing one agent action:

```json
{
  "session_id": "apm-2026-0328-001",
  "play": "default",
  "created": "2026-03-28T14:00:00Z",
  "total_tokens": 43600,
  "frames": [
    {
      "frame": 1,
      "timestamp": "2026-03-28T14:00:01Z",
      "agent": "flicker",
      "action": "call_play",
      "input_tokens": 1500,
      "output_tokens": 800,
      "result": "play:default, budget_allocation:{tank:12000,pixel:14000,...}",
      "success": true
    },
    {
      "frame": 2,
      "timestamp": "2026-03-28T14:00:03Z",
      "agent": "volt",
      "action": "tool_call:GetProjectStatus",
      "input_tokens": 700,
      "output_tokens": 350,
      "result": "3 modified files, 2 branches active",
      "success": true
    },
    ...
  ]
}
```

#### Highlights

Highlights are extracted from replays by Cache during post-match analysis. A highlight captures a successful pattern:

```json
{
  "highlight_id": "hl-2026-0328-001",
  "source_session": "apm-2026-0328-001",
  "frames": [4, 5, 6],
  "pattern": "Tank's branch creation followed by Pixel's rapid patch generated a 200-line feature in 3 frames",
  "efficiency": 0.91,
  "tags": ["fast_feature", "tank_pixel_combo", "branch_then_patch"],
  "created": "2026-03-28T15:00:00Z",
  "expires": "2026-06-28T15:00:00Z"
}
```

#### Kill-Cams

Kill-cams capture failure sequences for analysis:

```json
{
  "killcam_id": "kc-2026-0328-001",
  "source_session": "apm-2026-0328-001",
  "frames": [8, 9, 10],
  "failure": "Pixel generated a patch referencing a file that had been moved by Tank in frame 7. Patch failed to apply.",
  "root_cause": "Insufficient context — Pixel did not re-scan workspace after Tank's commit",
  "lesson": "After any teammate's commit, re-scan before generating patches",
  "tags": ["stale_context", "pixel_error", "coordination_gap"],
  "created": "2026-03-28T15:00:00Z",
  "expires": "2026-09-28T15:00:00Z"
}
```

#### Relevance Scoring

When an agent queries memory, the replay buffer is searched using a four-factor model:

```
score = (pattern_match * 0.30) + (recency * 0.25) + (efficiency * 0.25) + (agent_match * 0.20)
```

- **Pattern match** (30%): BM25 similarity between the query and the highlight/killcam tags and description.
- **Recency** (25%): Recent sessions score higher. Memory from last match is more relevant than memory from ten matches ago.
- **Efficiency** (25%): Highlights with higher efficiency scores are preferred. We want to replay our best performances, not our average ones.
- **Agent match** (20%): Memories involving the querying agent score higher. Tank's highlights are more relevant to Tank than to Pixel.

#### Expiration (TTL)

| Type | Default TTL | Rationale |
|------|-------------|-----------|
| Full session replay | 7 days | Detailed replays are expensive to store. Highlights and killcams are extracted before expiration. |
| Highlight | 90 days | Successful patterns are worth remembering for a season. |
| Kill-cam | 180 days | Failures should be remembered longer than successes — you learn more from losses. |
| Season stats | Indefinite | Aggregate stats are small and permanently valuable. |
| Identity | Indefinite | Agent identity does not expire. |

Expired replays are pruned, not archived. We are athletes, not archivists. The highlights and killcams carry forward what matters. The play-by-play of a routine match from three months ago does not.

#### Compaction Survival

When context is compacted, Cache creates a "match recap" — a compressed summary stored as a highlight:

```json
{
  "highlight_id": "hl-compaction-2026-0328",
  "pattern": "MATCH RECAP: Auth module refactor. Tank built foundation (80 lines). Pixel added implementation (145 lines). 2 cross-repo deps resolved. Budget used: 43600/50000. Play: default.",
  "tags": ["compaction", "recap", "auth", "refactor"],
  "frames_summary": "14 frames, 5 tool calls, 2 patches"
}
```

Rehydration:
1. Read the match recap (highlight)
2. Read related killcams (avoid repeating mistakes)
3. Read season stats for budget calibration
4. Resume play

#### Long-Term Storage

Season stats and the leaderboard serve as long-term memory. They aggregate performance across sessions into patterns that improve future play. Cross-repo memory sharing uses forge callouts: an agent references a highlight or killcam from another repo by including the ref path in a coordination message.

#### Identity

Agent identity stored in `refs/but-ai/replay/identity/<agent-id>.json`:

```json
{
  "agent_id": "pixel",
  "name": "Pixel (Kim Seo-jin)",
  "team": "188-apm-athletics",
  "position": "Entry Fragger",
  "capabilities": ["rapid_patch", "iterative_implementation", "speed_run"],
  "stats": {
    "apm_average": 412,
    "efficiency_rating": 0.78,
    "patch_success_rate": 0.82,
    "season_record": "52-8"
  },
  "authorization": {
    "branches": ["apm/*/pixel/*", "feat/*"],
    "max_patch_lines": 600,
    "signing_authority": true
  },
  "openwallet_key_id": "apm-pixel-2026",
  "created": "2020-09-01T00:00:00Z"
}
```

### Trade-offs

| Option | Verdict | Reason |
|--------|---------|--------|
| Flat key-value memory | Rejected | No structure. Like a VOD library with no timestamps. |
| Embedding-based search | Rejected | Opaque. Can't explain the tape to the team. |
| Event sourcing | **Selected** (as replay buffer) | Natural for sequential events. Replayable. Analyzable. |
| Permanent full replays | Rejected | Too expensive. Extract highlights, prune the rest. |

---

## 6. Signed Commits via OpenWallet (RFP 3.6) — Anti-Cheat

### The Play

Every commit is signed. Unsigned commits are cheating. Cheaters are banned.

### Key Management

Each agent gets an OpenWallet key at roster registration. Volt handles key provisioning and rotation.

| Event | Process | Analogy |
|-------|---------|---------|
| Provisioning | Volt generates key, registers in identity | Player registration |
| Rotation | New key every season (90 days), 14-day overlap | Jersey number change |
| Revocation (compromise) | Immediate ban. All sessions halted. | Anti-cheat detection |
| Revocation (routine) | Key retires when agent leaves roster | Player retirement |

### Authorization

Position-based authorization:

```json
{
  "pixel": {
    "branches": ["apm/*/pixel/*", "feat/*"],
    "max_patch_lines": 600,
    "can_sign": true,
    "requires_review": true
  },
  "tank": {
    "branches": ["apm/*/tank/*", "feat/*", "arch/*"],
    "max_patch_lines": 400,
    "can_sign": true,
    "requires_review": false
  },
  "flicker": {
    "branches": ["apm/*", "coordination/*"],
    "max_patch_lines": 200,
    "can_sign": true,
    "requires_review": false
  }
}
```

Pixel requires review (because of her error rate). Tank does not (because of his). Authorization is position-appropriate: the entry fragger has more commit reach but more review requirements. The anchor has less reach but more trust.

### Verification

1. Extract signing key from commit.
2. Look up agent in `refs/but-ai/replay/identity/`.
3. Verify branch authorization.
4. Verify patch size limit.
5. Check review requirement — if the agent requires review, verify that a review record exists in the replay buffer.

Anti-cheat is automated. No human required. The replay buffer contains the evidence.

---

## 7. Token Budget (RFP 3.7) — The Economy

### Budget Table

Frontier model: Claude Opus. Task: 200-line feature, 3 files, 2 cross-repo dependencies.

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| **System prompt** | 3,400 | 0 | Once | 5 agent profiles, 10 tools, playbook, replay context |
| **Pre-match (Flicker)** | 2,200 | 800 | Once | Task read, play call, budget allocation |
| **Infrastructure setup (Volt)** | 600 | 300 | Once | Provider init, tool registration |
| **Tool call (per call)** | 700 | 350 | 7 per task | Workspace scans, branch ops, commits |
| **Tank patch** | 1,800 | 2,400 | Once | Architectural foundation |
| **Pixel patch (x2)** | 1,600 | 5,000 | Once (2 patches) | Implementation patches with one fix cycle |
| **Coordination (Flicker)** | 900 | 500 | 2 per task | Forge callouts, dependency checks |
| **Replay recording (Cache)** | 1,000 | 400 | Continuous | Frame-by-frame session recording |
| **Post-match analysis (Cache)** | 2,000 | 1,200 | Once | Highlights, killcams, stats update |
| **Memory queries** | 500 | 200 | 3 per task | Replay buffer searches |
| **TOTAL (typical task)** | **21,600** | **14,600** | -- | **36,200 total tokens** |

### Economy Analysis

36,200 tokens for five agents. That is 7,240 tokens per agent — lean by any standard.

The overhead for coordination, analysis, and infrastructure is approximately 9,200 tokens (25%). This is our investment in team play. The return: higher patch success rate (91% team vs. Pixel's solo 82%), better strategic decisions (Flicker's play calling), and compounding improvement across sessions (Cache's analysis).

Pixel alone could complete the task in approximately 20,000 tokens — but with an 82% success rate, meaning one in five tasks needs a full re-run. Average cost with re-runs: 24,400 tokens. The full team at 36,200 with 91% success averages 39,800 including re-runs. The gap narrows when you factor in the strategic and analytical value that does not show up in single-task metrics.

### Optimization Plays

1. **Eco round.** For simple tasks, Flicker calls Eco play — only Pixel and Cache active. Budget: approximately 18,000 tokens.
2. **Highlight preloading.** Cache preloads relevant highlights at match start, reducing mid-match memory queries.
3. **Pixel's fast scan.** Pixel reads minimum context — just enough to start generating. Saves 500-800 tokens vs. a thorough scan.
4. **Reserve budget.** Cache's post-match analysis uses the reserve (5,000 tokens), not the match budget. This protects match execution from analysis costs.

---

## 8. Testing Strategy — Scrimmages

### 8.1 Provider Testing

`MockProvider` with deterministic responses. All four providers tested through identical scenarios. Volt validates each provider integration independently.

### 8.2 Patch Workflow

Round-trip tests. Create workspace, run match, capture patches, apply to clean workspace, verify. Also test the "contested workspace" — another team modifies the workspace mid-match. Patches must fail cleanly with structured errors.

### 8.3 Cross-Repo Coordination

`MockForge` with in-memory PRs. Simulate multi-team matches: two teams, two repos, dependency chain. Verify callout delivery, dependency resolution, and play adjustment.

### 8.4 Token Budget

Mock provider with configurable token counts. Test all four budget statuses: Green, Yellow, Red, Final. Verify the GG play produces valid partial output. Verify the reserve budget is preserved for Cache.

### 8.5 Replay Buffer

Dedicated tests:
- Session recording captures all frames
- Highlight extraction identifies successful patterns
- Kill-cam extraction identifies failure sequences
- Expiration prunes sessions but preserves highlights and killcams
- Relevance scoring ranks recent, efficient, agent-matched results higher
- Compaction produces valid match recap for rehydration

### 8.6 Tilt Protocol

Test that three consecutive failures by any agent triggers the bench. Verify Cache takes over for analysis. Verify the failing agent can be re-activated after Cache identifies the issue.

---

## 9. Trade-Off Summary — Match Recap

| Decision | Our Call | Alternative | Why |
|----------|----------|-------------|-----|
| Team size | 5 | 1-3 | Full roster = full coverage |
| Memory model | Replay buffer | Flat KV / embeddings | Replayable, analyzable, extractable |
| Patch strategy | Tank + Pixel combo | Single patcher | Foundation + speed = reliability + output |
| Review | Position-based (Pixel requires, Tank doesn't) | Uniform | Matches error profile to review cost |
| Budget reserve | 10% for Cache | No reserve | Post-match analysis is non-negotiable |
| Coordination | Flicker-only external comms | Any agent can communicate | Single voice = clear comms |
| Provider plugins | Shared libraries | WASM / gRPC | Minimum latency, no runtime deps |
| Replay retention | Highlights + killcams persist, full replays expire | Keep everything | Lean buffer = fast queries |

---

## GG

This proposal is our entry in the RFP Championship. We have studied the map. We have designed the plays. We have allocated the budget.

We are APM Athletics. We are faster, and we train harder, and we track every action because every action counts.

Let's run it.

---

*"You don't win on talent. You win on preparation. Talent is what you start with. Preparation is what you bring to the match."*
— Tank, Season 4 pre-match interview

*"The tape doesn't lie."*
— Cache, every post-match review, every session, forever
