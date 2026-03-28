# Stellamaris & Sons — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. We keep the architecture simple because Paolo maintains it and he is also running a logistics company. The crate has three modules: `cli` (argument parsing and subcommand dispatch), `agent` (the execution engine), and `mcp` (the server mode).

Subcommands: `run` (execute a task), `review` (matriarch-style validation of a pending patch), `memory`, `mcp`.

The `review` subcommand is distinctive — it takes an existing `INDEX.patch` + `COMMIT.msg` and runs the matriarch validation: checks for correctness, style, completeness, and budget compliance. This can be used standalone, even for patches produced by other tools or humans.

MCP mode is backward-compatible: the existing `gitbutler_update_branches` tool is preserved as a wrapper around the new `AgentRun` tool. Clients that rely on the old tool continue to work.

**WASI:** Under WASI, the plugin announces which capabilities are missing (forge coordination, plugin discovery) and operates in reduced mode. Patch generation and memory queries work. Paolo's pragmatic take: "If it runs on the phone one day, this is enough."

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` directly. The patriarch agent selects the provider method based on task complexity: `response` for simple tasks (commit messages), `tool_calling_loop` for standard tasks, `tool_calling_loop_stream` for complex tasks where the matriarch monitors progress in real time.

New providers: a TOML configuration mapping provider names to endpoint URLs and API formats. If the provider is OpenAI-compatible, no code changes. If not, a small adapter module in `crates/but-ai/src/providers/`. Paolo does not want a plugin system for providers — "I do not want to debug someone else's plugin at 2 AM."

## 3. The But Agent (RFP 3.3)

The agent follows the Stellamaris family workflow:

1. **Patriarch plans.** Reads task, retrieves memory, produces implementation spec.
2. **Sibling implements.** Produces `INDEX.patch` + `COMMIT.msg`. Uses `GetProjectStatus`, `GetBranchChanges`, and `GetCommitDetails` for context.
3. **Matriarch reviews.** Reads the spec, reads the patch, reads the commit message. Checks: Does the patch match the spec? Is the commit message clear? Is the budget within limits? Is the patch size reasonable?
4. **If approved:** matriarch signs (via Warden role). If rejected: sibling revises. Maximum 2 revision cycles before the task is returned as incomplete.

**Branch naming:** `stm/<task-id>/<deps>`. Family abbreviation. Example: `stm/T042/s01.s02`.

**Budget enforcement:** The patriarch allocates budget: 15% planning, 55% implementation, 20% review, 10% reserve. If implementation exceeds 55%, the matriarch gets a reduced allocation. If the matriarch cannot complete review within budget, she approves with a `QUICK_REVIEW` tag, indicating the review was abbreviated.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Forge {
    fn open_pr(&self, repo: &str, pr: PrDraft) -> Result<PrRef>;
    fn post_comment(&self, pr: PrRef, body: &str) -> Result<()>;
    fn get_comments(&self, pr: PrRef) -> Result<Vec<Comment>>;
    fn set_status(&self, pr: PrRef, status: &str) -> Result<()>;
    fn cross_reference(&self, from: PrRef, to: PrRef) -> Result<()>;
}
```

Five methods. GitHub reference implementation. The younger sibling handles all forge operations — it is the family's "people person," comfortable with external communication.

**PR comment schema:** Bilingual — structured JSON wrapped in Italian-flavored human-readable text (because the Stellamaris team genuinely communicates this way internally):

```markdown
**Stellamaris: Compito completato.** Patch: 142 righe, 3 file.

<!-- stm:msg {"v":1,"agent":"primo","status":"done","tokens":{"used":19500,"budget":26800}} -->
```

Machines read the HTML comment. Humans read the Italian. Both are happy.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Branch `stm/memoria`. Directory structure mirrors the family hierarchy:

- `padre/` — Architectural decisions, task decomposition patterns
- `nonna/` — Review heuristics, common error patterns, style guides
- `primo/` — Implementation patterns, code conventions
- `secondo/` — Forge interaction patterns, PR templates
- `apprendista/` — Learned routines, formatting rules

Each agent reads its own directory and the nonna's directory (because everyone in the family reads Lucia's notes).

**Relevance scoring:** Simple keyword matching weighted by recency. The family does not believe in complex scoring algorithms. "If you cannot find what you need in thirty entries, you have too many entries." Maximum active entries per agent: 30. Oldest entries are evicted when the cap is hit.

**TTL:** Padre and nonna entries last 30 days (strategic knowledge persists). Sibling entries last 14 days. Apprentice entries last 7 days. Access resets the timer.

**Compaction survival:** Each agent has a "family recipe" — a single memory entry that is always preserved, containing the agent's core operating principles. The nonna's recipe includes her review checklist. The primo's recipe includes code style conventions. These survive any compaction.

**Identity:** Agent identities stored at `stm/identita/<agent>.json`. Contains: name (Italian), role, branch permissions, public key, and a `commissioned_by` field (which other agent authorized this agent's creation — the patriarch commissions all agents, but the matriarch must countersign).

## 6. Signed Commits via OpenWallet (RFP 3.6)

The matriarch is the sole signer. This is non-negotiable. Lucia stamps every manifest; the matriarch signs every commit.

**Authorization:**

```toml
[agents.padre]
branches = ["stm/spec/*"]
can_commit = false  # Plans, does not commit

[agents.primo]
branches = ["stm/*"]
can_produce_patch = true
can_commit = false

[agents.nonna]
branches = ["stm/*", "feat/*"]
can_sign = true
max_patch_lines = 600
```

Only the nonna has `can_sign = true`. All commits are attributed to the implementing agent but signed by the matriarch.

**Key lifecycle:** The matriarch's key is provisioned by the patriarch in a ceremony that requires both agents present. Rotation every 45 days. Compromise handling: the patriarch revokes the key and provisions a new one. All commits since last known-good state are flagged. The matriarch re-reviews them. If Lucia taught us anything, it is that you check again.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,000 | 0 | Once/session | Family roles + tool descriptions |
| Task ingestion | 2,000 | 400 | Once/task | Patriarch reads and plans |
| Planning | 1,200 | 800 | Once/task | Implementation spec |
| Tool call (per call) | 1,100 | 500 | ~4/task | Context gathering |
| Patch generation | 2,800 | 4,200 | Once/task | Primo implements |
| Matriarch review | 3,500 | 300 | Once/task | Full patch + spec review |
| Commit message | 700 | 300 | Once/task | Clear, bilingual |
| Memory retrieval | 400 | 100 | 1/task | Keyword lookup |
| Coordination event | 1,000 | 400 | 1/task | Secondo handles forge |
| **TOTAL (typical task)** | **19,100** | **9,500** | -- | Includes matriarch overhead |

## Unique Insight

We run a business where a 74-year-old woman who does not trust computers catches errors that the computers miss 2% of the time. That 2% matters. It is the difference between a lost container and a delivered one, between a trusted partner and a former client.

Our matriarch agent is Lucia translated into software. It adds cost. It adds time. It catches errors that automated testing misses — not logical errors, but contextual ones. A patch that is technically correct but violates the project's conventions. A commit message that is accurate but misleading. A cross-repo reference that points to the right PR but the wrong commit.

Human review is not a process to be eliminated. It is a process to be automated respectfully.

---

*"La nonna ha sempre ragione."*
*(Grandmother is always right.)*
