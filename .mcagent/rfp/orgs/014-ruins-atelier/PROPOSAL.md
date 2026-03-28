# Ruins Atelier — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/` with modules: `survey` (workspace analysis), `workshop` (agent execution), `gallery` (output formatting), and `archive` (memory and MCP server).

CLI subcommands: `agent restore` (execute task — we call it "restoring" because the agent brings the codebase toward its intended state), `survey` (read-only analysis), `memory`, `mcp`.

Every CLI output passes through the gallery module, which formats output according to `BUT_OUTPUT_FORMAT`. In human mode, output includes indented structure, color-coded status indicators, and carefully worded descriptions. In JSON mode, output is clean structured data.

MCP mode: drop-in replacement. All `WorkspaceToolset` tools plus `AgentRestore`, `SurveyReport`, and `MemorySearch`.

**WASI:** Under WASI, the plugin operates in "museum mode" — read-only exhibition of existing state. Survey reports and memory queries work. Agent execution is deferred until a non-WASI environment is available. The plugin logs a message explaining the limitation in the same careful language it uses for everything else.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` without modification. Provider selection is based on task fidelity requirements:

- **High-fidelity tasks** (architectural changes, interface definitions): use the best available provider (Anthropic/OpenAI with the largest context window)
- **Standard tasks** (implementation against a clear spec): any provider with tool calling support
- **Surface tasks** (formatting, commit messages): even a small local model suffices

This fidelity-based routing is unique to our proposal. The scanner agent assesses task complexity and recommends a provider tier. The project lead approves.

New providers: a configuration-driven approach. Each provider needs an entry in `.but-ai/providers.toml` mapping its name to its API format and capability set. If the provider is OpenAI-compatible, it works immediately.

## 3. The But Agent (RFP 3.3)

The agent follows the restoration cycle:

1. **Survey** — Scanner reads workspace state, retrieves memory, produces condition report
2. **Work Order** — Project lead decomposes task into restoration phases
3. **Execution** — Artisan(s) produce `INDEX.patch` + `COMMIT.msg`
4. **Preservation** — Conservator verifies, updates memory, signs commit

The structural artisan uses `GetProjectStatus`, `GetBranchChanges`, and `GetCommitDetails` extensively. It reads surrounding code carefully before producing a patch — the patch must fit the existing codebase the way a restored fragment fits the original vessel.

**Branch naming:** `ra/<project>/<phase>/<deps>`. Example: `ra/P042/exec/s01`.

**Budget enforcement:** Budget is allocated by project phase: survey 20%, planning 10%, execution 50%, preservation 20%. The preservation phase's generous allocation reflects the studio's commitment to after-work verification and knowledge capture.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait GalleryChannel {
    fn exhibit(&self, repo: &str, pr: Exhibition) -> Result<ExhibitId>;
    fn annotate(&self, exhibit: ExhibitId, note: CuratorNote) -> Result<()>;
    fn catalog(&self, exhibit: ExhibitId) -> Result<Vec<CuratorNote>>;
    fn link(&self, from: ExhibitId, to: ExhibitId) -> Result<()>;
}
```

We think of PRs as "exhibitions" and comments as "curator's notes." The metaphor keeps our team aligned with our values.

**PR comment schema:** Each comment is a curator's note — human-readable first, machine-parseable second:

```markdown
**Restoration Report** — Project P042, Phase: execution

The structural artisan produced a patch addressing the authentication module refactor. The patch introduces a new trait boundary at the provider interface, consistent with the existing `Toolset` pattern. 164 lines across 3 files.

```json
{"schema":"ra-note/v1","agent":"structural-artisan","status":"complete","lines":164,"files":3,"budget":{"used":18200,"total":26000}}
```
```

The human text is not generated from the JSON — both are authored independently. The human text includes context and reasoning. The JSON includes metrics.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Branch `ra/archive`. Organized as a digital museum:

- `collection/<project-id>/` — Per-project knowledge (what worked, what didn't, what conventions apply)
- `catalog/` — Cross-project patterns and reusable insights
- `registry/` — Agent identity records
- `conservation/` — Preservation notes (why certain decisions were made)

**Relevance scoring:** The conservator uses a "provenance chain" model. Each memory entry records where its knowledge came from (which task, which codebase files, which other memories it relates to). When scoring relevance for a new task, the conservator traces the provenance chain: a memory entry about "authentication patterns" that came from the same codebase directory as the current task scores higher than one from a different project.

**TTL:** Collection entries (project-specific): 14 days. Catalog entries (cross-project): 60 days. Conservation notes (reasoning): 90 days. Registry entries (identity): indefinite.

**Compaction survival:** Each agent has a "conservation card" — a compressed summary of its role, capabilities, and the three most important patterns it has learned. Conservation cards are always injected into compacted context. They are updated after every task by the conservator.

**Identity:** Agent identities are stored in the registry as signed JSON files. Each identity includes: agent name, specialization, authorized operations, public key, and a "provenance" field listing who created the agent and when. Identities are countersigned by the conservator.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The conservator signs all commits. Before signing, the conservator performs three checks: (1) the patch matches the work order, (2) the commit message accurately describes the change, (3) the agent that produced the patch was authorized to work on the target branch.

**Authorization:**

```toml
[agents.structural-artisan]
branches = ["ra/*/exec/*", "feat/*"]
max_patch_lines = 600

[agents.surface-artisan]
branches = ["ra/*/surface/*", "docs/*"]
max_patch_lines = 300

[agents.conservator]
branches = ["ra/*", "feat/*"]
can_sign = true
```

**Key lifecycle:** Keys are provisioned by the conservator in a documented ceremony. Rotation every 30 days. Compromise: the conservator revokes the key, flags all affected commits, and the scanner re-surveys the affected branches to assess damage.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,200 | 0 | Once/session | Studio roles + restoration principles |
| Task ingestion | 2,000 | 300 | Once/task | Project lead reads task |
| Survey (condition report) | 3,500 | 500 | Once/task | Scanner reconnaissance |
| Work order | 800 | 500 | Once/task | Project lead planning |
| Tool call (per call) | 1,000 | 500 | ~4/task | Artisan context gathering |
| Patch generation | 3,000 | 4,200 | Once/task | Artisan execution |
| Commit message | 800 | 400 | Once/task | Carefully crafted |
| Preservation check | 2,000 | 300 | Once/task | Conservator verification |
| Memory retrieval | 600 | 200 | 2/task | Provenance chain lookup |
| Coordination event | 1,200 | 600 | 1/task | Curator's note |
| **TOTAL (typical task)** | **21,100** | **10,000** | -- | Restoration-grade quality |

## Unique Insight

In artifact conservation, there is a principle called "reversibility" — every intervention should be reversible. The adhesive you use to repair a ceramic vessel should be removable without damaging the original clay. The fill material you use for a missing fragment should be distinguishable from the original under UV light.

We apply the same principle to agent commits. Every patch should be cleanly revertable. Every change should be distinguishable from human-authored code (via the signed commit attribution). And the system should always be able to return to its pre-agent state without damage. Reversibility is not a backup plan — it is a design requirement. The original must always be recoverable.

---

*"The fragment that survives teaches more than the whole that was lost."*
