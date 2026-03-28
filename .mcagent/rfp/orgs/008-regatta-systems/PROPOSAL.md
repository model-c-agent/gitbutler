# Regatta Systems — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust binary crate at `crates/but-ai/`. Lean build, fast startup. In yacht racing, every second at the start counts — same principle applies to CLI tool latency.

CLI subcommands: `agent run`, `agent adapt` (re-evaluate strategy mid-task), `memory`, `status`, `mcp`. The `adapt` subcommand is unique to our architecture — it allows an external actor (human or orchestrator) to inject new information mid-task, triggering a strategy update without restarting.

MCP mode: `ServerHandler` implementation, all `WorkspaceToolset` tools plus `AgentRun`, `AgentAdapt`, and `ConditionReport` (returns current strategy state and remaining budget).

**WASI:** The plugin operates in "drifting" mode under WASI — it can perform read-only operations (memory queries, workspace status) but cannot execute agent tasks or coordinate via forge APIs. This mirrors a yacht under bare poles: making way but not racing.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm`'s `tool_calling_loop_stream` as our primary interface. Streaming is essential for our adaptive model — the tactician begins processing results as they arrive, not after the full response completes. This enables mid-generation strategy adjustments.

Provider capability is detected at startup: does this provider support streaming? Tool calling? Structured output? The agent adjusts its execution model based on available capabilities. With a full-featured provider (Anthropic, OpenAI), the agent runs its adaptive model. With a limited provider (some Ollama models), the agent falls back to a fixed-plan execution that doesn't require streaming.

New providers: we propose a capability manifest — a small JSON file per provider that declares supported features. Adding a provider means writing the manifest and (if needed) an API adapter. No recompilation of `but-ai`.

## 3. The But Agent (RFP 3.3)

The agent runs an **adaptive execution loop**:

```
while budget_remaining > RESERVE:
    conditions = observe(workspace, memory, tool_results)
    strategy = tactician.evaluate(conditions, current_strategy)
    if strategy.changed:
        log("Course correction: {}", strategy.delta)
    action = strategy.next_action()
    result = execute(action)
    update(conditions, result)

patch = trimmer.produce(strategy.final_state)
commit_msg = format_commit(strategy, patch)
helmsman.sign(patch, commit_msg)
```

The loop continues until either the task is complete or the budget reserve is hit. The reserve (10% of total budget) is held for patch generation and signing — the agent always has enough tokens to produce a final artifact.

**Branch naming:** `reg/<leg>/<mark>/<deps>`. Racing terminology: `leg` is the task phase, `mark` is the task ID, `deps` are dependencies. Example: `reg/upwind/M042/s01.s03`.

**Progress reporting:** Each iteration of the loop emits a structured progress event:

```json
{"leg": "upwind", "mark": "M042", "tack": 3, "budget_pct": 72, "strategy_stable": true}
```

This feeds into both terminal display (for humans) and JSON output (for orchestrators).

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait ForgePort {
    fn open_channel(&self, repo: &str, spec: ChannelSpec) -> Result<Channel>;
    fn signal(&self, ch: &Channel, msg: Signal) -> Result<()>;
    fn receive(&self, ch: &Channel) -> Result<Vec<Signal>>;
    fn mark(&self, ch: &Channel, label: &str) -> Result<()>;
}
```

We think of PRs as "channels" and comments as "signals" — maritime radio terminology. The abstraction keeps it simple. GitHub implementation maps channels to PRs, signals to comments.

**PR comment schema:**

```
[REG:signal] mark=M042 from=reg/trimmer tack=3
state: complete
patch: 156 lines / 3 files
budget: 18400/25600
depends: upstream/infra#78
```

Human-readable, machine-parseable, minimal. One signal per PR comment. No threading — each signal stands alone.

**Cross-repo coordination:** The bowman agent handles all cross-repo work. It pre-fetches dependency states before the trimmer begins work, reducing mid-task blocking. Dependencies are tracked as marks on a course chart (a JSON file in the memory branch listing all known cross-repo references and their states).

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/reg/memory/` namespace. Memory entries are stored as JSON blobs organized by "leg" (task category):

- `refs/reg/memory/conditions/` — Environmental observations (codebase patterns, conventions, known issues)
- `refs/reg/memory/tactics/` — Successful strategies worth repeating
- `refs/reg/memory/marks/` — Task-specific context that may be relevant to follow-up tasks

**Relevance scoring:** We use a "wind vector" model. Each memory entry has a direction (tags indicating domain relevance) and magnitude (recency-weighted access count). The current task has a "heading" (its keywords and domain). Memories whose direction aligns with the heading (cosine similarity > 0.4) are retrieved, ranked by magnitude.

**TTL:** Conditions expire in 7 days (the codebase changes fast). Tactics expire in 30 days. Marks expire in 3 days. TTLs are reset when a memory is accessed.

**Compaction survival:** The navigator maintains a "chart" — a compact summary of the workspace's essential state (branch topology, recent changes, known conventions). The chart is always injected into compacted context. It is the minimum viable navigational awareness.

**Identity:** Each agent's identity is a JSON file at `refs/reg/identity/<role>`. Contains: role name, station (which tools it uses), qualifications (task categories it can handle), and public key fingerprint.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Only the helmsman signs. The helmsman reviews the patch and strategy before signing. This is not a rubber stamp — the helmsman can (and does) reject patches that the trimmer produced under an unstable strategy.

**Authorization:** Policy stored in `.but-ai/crew-manifest.toml`:

```toml
[crew.tactician]
branches = []  # Does not commit

[crew.trimmer]
branches = []  # Produces patches, does not commit

[crew.helmsman]
branches = ["reg/*", "feat/*", "fix/*"]
max_patch_lines = 800
require_stable_strategy = true
```

The `require_stable_strategy` flag means the helmsman will not sign a commit if the strategy changed in the last iteration of the execution loop.

**Key lifecycle:** Provisioned at the start of each "campaign" (project engagement). Rotated between campaigns. Compromise revocation follows the racing protest process: the helmsman files a protest (revocation request), all affected commits are flagged for review, and a post-campaign audit determines which commits need re-signing.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Racing crew roles + tool descriptions |
| Task ingestion | 2,000 | 300 | Once/task | Read task, assess conditions |
| Strategy formulation | 1,500 | 800 | 1-3/task | Adaptive — may update mid-task |
| Tool call (per call) | 1,000 | 500 | ~5/task | Bowman pre-fetch + trimmer data gathering |
| Patch generation | 2,800 | 4,000 | Once/task | Trimmer output, optimized for size |
| Commit message | 600 | 250 | Once/task | Concise, includes strategy reference |
| Memory retrieval | 500 | 150 | 2/task | Wind vector matching |
| Coordination event | 1,000 | 400 | 1/task | Signal exchange |
| **TOTAL (typical task)** | **17,400** | **9,400** | -- | With 2 strategy updates |

## Unique Insight

In racing, the boats that win are not always the fastest. They are the ones that respond to wind shifts first. A 5-degree wind shift that you detect 30 seconds before your competitor is worth more than 2 knots of boat speed.

The same is true for AI agents. An agent that detects a change in task conditions (new test failures, updated dependencies, conflicting changes on another branch) and adjusts its strategy 30 seconds before running into the problem is more effective than an agent that runs 2x faster on a fixed plan and hits the problem head-on.

Adaptation is not a backup plan. Adaptation is the plan.

---

*"The fleet that tacks first wins the leg."*
