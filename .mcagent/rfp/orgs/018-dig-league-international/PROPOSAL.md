# Dig League International — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. Optimized for speed — we measure startup latency and track it in CI. Target: sub-50ms cold start for CLI mode.

CLI subcommands: `agent dig` (execute task), `agent score` (evaluate an existing patch against quality rubric), `agent clock` (show token budget status), `memory`, `mcp`.

The `score` subcommand is standalone: feed it a patch and a rubric, and it returns a numeric score. Useful for evaluating patches from any source, not just our agents.

MCP mode: drop-in replacement. All `WorkspaceToolset` tools plus `DigTask`, `ScorePatch`, `BudgetClock`.

**WASI:** Under WASI, the `score` subcommand works (it's pure computation). `dig` is unavailable (requires plugin discovery and forge access). The scoring capability alone is useful enough to justify a WASI build.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` directly. We benchmark every provider quarterly on our competition scoring metrics: response latency, tool-calling accuracy, patch correctness rate, and tokens-per-correct-patch. The results are published in our `BENCHMARKS.md` (updated quarterly).

Current standings: Anthropic leads on patch correctness. OpenAI leads on latency. Ollama is competitive for simple tasks at zero marginal cost. LMStudio is useful for offline tournaments.

New providers: TOML configuration mapping provider name to API URL and format. If OpenAI-compatible, it works. We add new providers when they appear in our benchmark pipeline. No upfront engineering cost — we pay the integration cost when the provider earns it.

## 3. The But Agent (RFP 3.3)

Competition round model:

1. **Whistle** — Captain reads task, allocates budget, assigns field agents
2. **Dig** — Field agents execute. Alpha handles complex work; Beta handles supporting work. Parallel when possible.
3. **Document** — Documentarian formats outputs, polishes commit messages
4. **Score** — Judge evaluates the complete package. Score = 0.4*correctness + 0.3*documentation + 0.3*efficiency
5. **Submit** — If score >= 0.7, ship. If < 0.7, revise with remaining budget

**Branch naming:** `dli/<round>/<team>/<deps>`. Example: `dli/R3/alpha/s01`.

**Budget enforcement:** The captain runs a "clock" — a token budget tracker that updates after every tool call and LLM interaction. The clock is visible in progress output. When the clock reaches 80%, the captain shifts to "closing" mode: no new tool calls, finish whatever patch is in progress, ship. When it reaches 95%, hard stop.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Scoreboard {
    fn open_round(&self, repo: &str, round: RoundSpec) -> Result<RoundId>;
    fn submit_entry(&self, round: RoundId, entry: TeamEntry) -> Result<()>;
    fn get_entries(&self, round: RoundId) -> Result<Vec<TeamEntry>>;
    fn post_score(&self, round: RoundId, score: ScoreCard) -> Result<()>;
}
```

We model PRs as "rounds" and comments as "entries" and "scores." GitHub reference implementation.

**PR comment schema:** Scoreboard format:

```markdown
**[DLI Round 3]** Agent: dli/alpha | Score: 0.84

| Metric | Score |
|--------|-------|
| Correctness | 0.91 |
| Documentation | 0.78 |
| Efficiency | 0.82 |

```json
{"v":1,"round":3,"agent":"alpha","lines":156,"files":3,"tokens":{"used":17200,"budget":25500}}
```
```

Human-readable score table at the top. Machine-parseable JSON at the bottom.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Branch `dli/memory`. Flat structure: `<agent-id>/<key>.json`. We tried hierarchical. Flat is faster to scan and simpler to maintain.

**Relevance scoring:** Competition-tuned scoring. Each memory entry has a "win rate" — how often retrieving this entry led to a successful task completion. Entries with high win rates are retrieved first. Win rate is updated after every task: if the task succeeded and the entry was in context, win rate increases. If the task failed, win rate decreases. This is a simple reinforcement signal.

**TTL:** Entries with win rate > 0.5 persist for 30 days. Entries with win rate <= 0.5 persist for 7 days. Entries that have never been accessed expire in 3 days. Natural selection for useful memories.

**Compaction survival:** The captain maintains a "game plan" — a single memory entry with the current project's conventions, common patterns, and strategic notes. The game plan always survives compaction.

**Identity:** JSON file per agent. Contains: name, position (role), stats (tasks completed, average score, win rate), public key. Identity is signed by the captain.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The judge signs all commits after scoring. A commit with a score below 0.7 is not signed — it is returned for revision. This means every signed commit in our system has a quality score attached.

**Authorization:**

```toml
[agents.alpha]
branches = ["dli/*", "feat/*"]
max_lines = 800

[agents.beta]
branches = ["dli/*", "feat/*"]
max_lines = 400

[agents.judge]
branches = ["dli/*", "feat/*"]
can_sign = true
min_score = 0.7
```

**Key lifecycle:** Keys provisioned at agent creation. Rotated between "seasons" (quarterly). Compromise: the judge revokes the key and initiates a scoring review of all commits since last known-good state — any commit that scores below 0.7 on re-evaluation is flagged.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Scoring rubric + tool descriptions |
| Task ingestion | 1,800 | 300 | Once/task | Captain reads and strategizes |
| Tool call (per call) | 1,000 | 500 | ~5/task | Both field agents |
| Patch generation (Alpha) | 2,800 | 4,000 | Once/task | Primary patch |
| Patch generation (Beta) | 2,000 | 2,500 | Once/task | Supporting patch |
| Documentation polish | 1,500 | 800 | Once/task | Documentarian pass |
| Scoring | 2,000 | 400 | Once/task | Judge evaluation |
| Commit message | 500 | 250 | Once/task | Competition format |
| Memory retrieval | 400 | 100 | 1/task | Win-rate lookup |
| Coordination event | 800 | 300 | 1/task | Scoreboard entry |
| **TOTAL (typical task)** | **18,600** | **11,150** | -- | Includes parallel agents |

## Unique Insight

From five years of competitive excavation, we have data that nobody else has: what makes a team fast without being sloppy. The answer is not talent. It is calibration. The best teams know exactly how much documentation is "enough" — not so little that they lose points, not so much that they lose time. They have internalized the scoring formula.

Our agents are calibrated the same way. The judge's scoring function is not a post-hoc check — it is the objective function that shapes every agent's behavior. The field agents know the rubric. They optimize for the score. When the rubric changes (because the project's priorities change), the agents' behavior changes automatically.

You cannot improve what you do not measure. We measure everything.

---

*"Score it or it didn't happen."*
