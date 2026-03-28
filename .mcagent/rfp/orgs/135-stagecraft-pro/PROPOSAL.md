# PROPOSAL.md — StageCraft Pro

**"We built Jira for theater. We can build agents for Git."**

---

## Summary

StageCraft Pro proposes to build the `but-ai` plugin the same way we build production management software: fast, lean, opinionated, and tested against real users. No over-engineering. No unnecessary abstractions. Ship the feature, measure the result, iterate tomorrow.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

`but-ai` is a binary on PATH. Manifest is TOML. Discovery is standard. We do not need to make this complicated. The plugin registers subcommands, the CLI discovers them, the user runs them. We will spend our token budget on the hard problems, not the solved ones.

Config lives in `.but/ai.toml`. Provider, model, and budget preferences are set once and respected everywhere. Override via flags for one-off usage: `--provider anthropic --model claude-3-5-sonnet`.

### Requirement 2: Provider-Agnostic AI

We built real-time sync for 180 theater companies across three different WebSocket implementations. Provider abstraction is the same pattern at a smaller scale. The `Provider` trait: `complete(messages, tools) -> Response`, `stream(messages, tools) -> Stream<Chunk>`, `usage(response) -> TokenUsage`. Four adapters: OpenAI, Anthropic, Ollama, LMStudio.

We add streaming support because latency matters. An agent that produces output progressively (streaming) gives the user feedback immediately. An agent that waits for a complete response before showing anything feels broken. We learned this from cue sheet sync: the user needs to see something moving within 200ms.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agent workflow is a sprint cycle:

1. **Ticket** — Maya writes acceptance criteria from the task description
2. **Context** — Andre retrieves memory, Raj reads the relevant files
3. **Build** — Raj produces INDEX.patch + COMMIT.msg
4. **Sync** — Zoe coordinates cross-repo state
5. **QA** — Maya reviews against acceptance criteria
6. **Ship** — Signed commit, PR created

Raj's patches follow Conventional Commits (`feat:`, `fix:`, `refactor:`). The COMMIT.msg is structured: type, scope, one-line description, optional body with rationale. No essays. The commit message should be readable in `git log --oneline`.

### Requirement 4: Polyrepo PR Coordination

Zoe's domain. Cross-repo coordination uses sync events — the same primitive we use for real-time cue sheet synchronization:

```json
{
  "event": "sync",
  "repo": "backend",
  "state": "patch-ready",
  "version": "abc123",
  "depends_on": [{"repo": "auth", "version": "def456"}],
  "timestamp": "2026-03-28T14:30:00Z"
}
```

Sync events are posted as PR comments. Each repo's agent reads the events on its dependent repos and blocks until dependencies are satisfied. Forge-agnostic: we implement a `Forge` trait with `post_event()`, `read_events()`, `resolve_dependency()`. Three adapters (GitHub, GitLab, Gitea). Same interface.

### Requirement 5: Agent Memory in Git Branches

Andre designed the memory system for speed. Memory lives in `refs/stagecraft/memory/` as Git blobs.

```json
{
  "key": "auth-jwt-refresh",
  "value": "JWT refresh uses sliding window with 24h cap",
  "tags": ["auth", "jwt", "security"],
  "relevance": 0.88,
  "written_by": "raj",
  "written_at": "2026-03-28T14:30:00Z",
  "ttl_days": 30,
  "reads": 4
}
```

**Key design decisions:**
- Relevance scored at write time, not read time. Read-time scoring costs tokens. Write-time scoring costs nothing at retrieval.
- `reads` counter tracks how often a memory is retrieved. High-read memories are kept longer (TTL extends on access). Zero-read memories expire on schedule.
- Tags enable fast filtering. Retrieval is: filter by tags, sort by relevance, take top N.

**Unique memory scheme:** StageCraft Pro uses **cue-sheet memory** — memories are ordered into a sequence that mirrors the production workflow. Retrieving memory for a task first returns "pre-show" memories (project setup, architecture decisions), then "Act I" memories (the specific subsystem being modified), then "intermission" memories (known issues, tech debt notes). This sequential structure helps agents build context in the same order a stage manager builds a show.

### Requirement 6: Signed Commits via OpenWallet

Raj signs commits via OpenWallet DID key. The signing step is integrated into the deploy pipeline — no commit ships without a signature, the same way no cue sheet update ships without Maya's approval. Key management is handled by Andre as part of infrastructure: quarterly rotation, automated alerting on approaching expiration, emergency rotation via CLI command.

We keep signing simple. One key, one signer, deterministic process. Multi-sig is a nice idea but adds latency and complexity that our users (theater companies with limited budgets) cannot afford.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Maya Chen | 6,000 | 2,000 | 8,000 | Product, review |
| Raj Patel | 9,000 | 7,000 | 16,000 | Patch generation |
| Zoe Kim | 6,500 | 3,000 | 9,500 | Coordination |
| Andre Morrison | 5,500 | 1,500 | 7,000 | Memory, infra |
| **Team Total** | **27,000** | **13,500** | **40,500** | |

Overhead: ~2,500 tokens (sprint coordination, acceptance criteria).
**Total per task: ~43,000 tokens.**

---

## Unique Insight

**Score relevance at write time, not read time.** Every other proposal we have read computes memory relevance during retrieval — comparing the current task against all stored memories using cosine similarity or semantic search. This costs tokens at read time, scales poorly as memory grows, and introduces latency into the agent's critical path. StageCraft Pro computes relevance at write time: when a memory is stored, the writing agent assigns a relevance score based on the current context. At read time, retrieval is a simple filter-and-sort — zero LLM calls, zero token cost, sub-millisecond latency. The tradeoff: write-time relevance is less precise than read-time relevance (the context at write time may differ from the context at read time). We accept this tradeoff because speed matters more than precision for memory retrieval, and a fast approximate answer beats a slow precise one every time.

---

*"Ship it. Then iterate."*
