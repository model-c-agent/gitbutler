# Order of the Leviathan — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

*Composed in accordance with the Rule, Precept 12: "Say what must be said. Say nothing more."*

---

## 1. Plugin Architecture (RFP 3.1)

A single Rust binary. No sub-crates. No dynamic loading. No plugin-within-a-plugin architecture. The binary compiles to `but-ai`, placed on PATH, discovered by `find_external_subcommand()`.

Subcommands: `agent` (execute task), `memory` (query/store), `mcp` (start server). Three subcommands are sufficient.

MCP mode implements `ServerHandler`, registers the ten `WorkspaceToolset` tools, and adds one new tool: `AgentExecute`. One tool. The agent does one thing: it receives a task and produces a patch. Additional granularity is unnecessary complexity.

**WASI:** Under WASI, the plugin is unavailable. This is acceptable. The Order does not believe in degraded modes. A tool either works fully or it does not work. Under WASI, the user should be informed clearly: "Agent capabilities are not available in this environment."

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Use `but-llm`. Call `tool_calling_loop` for tasks that require tool interaction. Call `response` for simple text generation (commit messages). Do not use streaming — streaming encourages the consumption of partial results, which is waste.

For new providers: add a new variant to the Git config. If the provider speaks an OpenAI-compatible API, it works immediately. If not, write an adapter crate. There is no need for a runtime plugin system. Compiling a new crate is a small cost for a correct integration.

**Trade-off:** We rejected hot-swappable provider plugins. Dynamic loading introduces failure modes (missing libraries, version mismatches, symbol conflicts) that are not worth the convenience. Static compilation eliminates an entire class of errors.

## 3. The But Agent (RFP 3.3)

Three-phase sequential execution:

1. **Observe (Reader):** Read task description. Retrieve memory. Examine workspace via `GetProjectStatus` and `GetBranchChanges`. Produce specification.
2. **Act (Scribe):** Read specification (not the original task). Produce `INDEX.patch` + `COMMIT.msg`. Use `GetCommitDetails` to match existing code conventions.
3. **Verify (Warden):** Compare patch against specification. Verify branch authorization. Sign commit.

**Branch naming:** `lev/<phase>/<task-hash>`. The task hash is a truncated SHA of the task description, ensuring deterministic branch names for identical tasks.

**Budget enforcement:** Each phase receives a fixed allocation: Reader 30%, Scribe 50%, Warden 20%. If a phase exceeds its allocation, execution halts. No borrowing between phases. The constraint forces discipline.

**Partial results:** If the Scribe cannot complete the patch within budget, it produces the largest correct subset — a patch that applies cleanly but implements only part of the task. The COMMIT.msg states what was completed and what remains.

## 4. Polyrepo PR Coordination (RFP 3.4)

The Order's approach to cross-repo coordination is minimal by design. We believe most coordination overhead is self-inflicted — agents coordinating about coordination.

**Forge adapter:**

```
trait Forge {
    fn open_pr(&self, repo: &str, pr: PrSpec) -> Result<PrId>;
    fn comment(&self, pr: PrId, text: &str) -> Result<()>;
    fn read_comments(&self, pr: PrId) -> Result<Vec<String>>;
}
```

Three methods. No label management, no status certificates, no receipt tracking. PRs are communication channels. Comments are messages. That is sufficient.

**PR comment schema:** Plain text with a structured header:

```
[LEV:status] Task complete. Patch: 84 lines, 2 files.
[LEV:depends] upstream/repo#45
[LEV:budget] 14200/17800 tokens consumed
```

No JSON. No nested objects. Parseable with a regex. Readable by a human without a decoder ring.

**Cross-repo coordination:** The Order limits cross-repo dependencies to a maximum of 2 per task. If a task requires coordination with more than 2 repositories, the task should be decomposed. Complexity is not a feature.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** A single ref, `refs/lev/memory`, pointing to a tree of JSON files. One file per memory entry. No subdirectories. No taxonomies. Flat storage, linearly scanned.

Linear scan is intentional. The Order's agents maintain few memories (target: fewer than 50 active entries). With 50 entries, linear scan is faster than index lookup because there is no index maintenance overhead.

**Relevance scoring:** Each memory entry has tags (3-5 keywords). Relevance is computed as the Jaccard similarity between the entry's tags and the current task's keywords. Entries scoring above 0.3 are included. Maximum 3 entries per retrieval.

**TTL:** Default 7 days. Entries that are accessed reset their TTL. Entries that are never accessed expire and are removed in a cleanup commit. The Order does not archive expired memories. Forgotten things should stay forgotten.

**Compaction survival:** One entry, tagged `core`, survives all compaction. This entry contains the agent's identity, current project context, and the Order's operating principles. Everything else is expendable.

**Identity:** Stored at `refs/lev/identity/<agent>` as a JSON blob: name, role (Reader/Scribe/Warden), authorized branches, public key fingerprint, vow date (when the agent was commissioned). The identity record is signed by all three agents at creation — a ceremony the Order takes seriously.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The Warden alone holds the signing authority. The Reader and Scribe cannot sign commits. This separation ensures that the signer has reviewed the content — there is no possibility of signing something unseen.

**Authorization:**

```toml
[agents.reader]
branches = []  # Reader does not commit
operations = ["read"]

[agents.scribe]
branches = []  # Scribe produces patches, does not commit
operations = ["read", "produce-patch"]

[agents.warden]
branches = ["lev/*", "feat/*"]
operations = ["read", "commit", "sign"]
max_patch_lines = 500
```

**Key lifecycle:** The Warden's key is provisioned in a ceremony witnessed by the Reader and Scribe. Rotation every 60 days, also ceremonial. Compromise triggers immediate revocation and a full audit of all commits since last rotation. The Order does not rush revocation — a thorough audit prevents false positives.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,200 | 0 | Once/session | Minimal: identity + tools + Rule excerpts |
| Task ingestion (Reader) | 2,500 | 0 | Once/task | Read task, retrieve memory |
| Specification (Reader) | 0 | 1,200 | Once/task | Terse specification output |
| Context reading (Scribe) | 2,500 | 0 | Once/task | Specification + code context |
| Patch generation (Scribe) | 0 | 4,500 | Once/task | INDEX.patch |
| Commit message (Scribe) | 500 | 200 | Once/task | Brief, purposeful |
| Verification (Warden) | 3,500 | 400 | Once/task | Patch vs spec comparison |
| Memory retrieval | 400 | 100 | 1/task | Jaccard lookup, 3 entries max |
| Coordination event | 600 | 200 | 0-1/task | Plain text PR comment |
| **TOTAL (typical task)** | **12,200** | **6,600** | -- | Leanest in the field |

## Unique Insight

The sea does not reward optimization for speed. It rewards optimization for fuel. A ship that arrives one day early and burns 20% more fuel has not optimized — it has traded one resource (time) for another (fuel) at a poor exchange rate.

The same applies to token budgets. An agent that produces a patch 30% faster by consuming 50% more tokens has not optimized. It has overspent. Our agents are slower than the competition. They are also cheaper per correct patch, because they do not waste tokens on self-review cycles, coordination overhead, or verbose output.

Frugality is not a constraint. It is the design.

---

*"Carry only what must be carried."*
— The Rule, Precept 1
