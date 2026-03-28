# Department of Comprehensive Zoning Review — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

`but-ai` as a PATH-discoverable binary. Implemented in Rust as `crates/but-ai/`. The binary must be installable on department workstations running Windows 10 without administrator privileges (Raymond's non-negotiable constraint).

CLI subcommands: `agent` (execute task), `memory` (query precedent database), `status` (agent health, budget, pending approvals), `mcp` (MCP server mode for IDE integration, though the department currently uses Notepad++ for text editing).

Environment variables: `BUT_WORKSPACE_DIR`, `BUT_OUTPUT_FORMAT`, `BUT_JSON`. Default output is human-readable memo format matching departmental standards. JSON output available via `BUT_JSON=1` for machine consumption.

WASI fallback: `but-ai-core` linked statically into `but` binary. Reduced mode — no plugin discovery, no subprocess execution. The department does not currently use WASI but Margaret insists on fallback paths for all critical systems ("What happens when the internet goes down?" is her standard question).

We add a `but ai audit` subcommand that generates a compliance report for all agent actions in the current session — who did what, when, citing which authority. This is required by department policy for any automated system that produces public records.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

Provider abstraction wrapping `but-llm`. The department currently has budget approval for one provider (OpenAI, via an existing city-wide enterprise agreement). Our abstraction layer supports additional providers but realistically, the department will use whichever provider the city's IT department has negotiated a contract with.

Provider selection: single-provider mode. All tasks go to the contracted provider. If the provider is unavailable, tasks queue until it recovers. Raymond refuses to implement automatic failover to an unauthorized provider because the city's data governance policy prohibits sending municipal data to unapproved vendors.

Capability detection: we check for tool calling support. If absent, we fall back to a prompt-based workflow where tool parameters are extracted from free text. This fallback exists because the city's enterprise agreement may change providers annually, and we cannot guarantee the new provider supports tool calling.

## 3. The But Agent (RFP 3.3)

Agent loop: **receive** (read task assignment, identify applicable zoning code sections) -> **research** (query memory for precedent, cross-reference code sections) -> **draft** (generate INDEX.patch with mandatory zoning references) -> **review** (internal review by Dolores, authorization by Margaret) -> **finalize** (produce COMMIT.msg with case number, code references, and records classification).

Every INDEX.patch includes a `Zoning-Ref:` trailer in the COMMIT.msg referencing the applicable code section. Patches without this trailer are rejected by Margaret's authorization gate. This is not configurable.

Branch naming: `dczr/<case-number>/s<NN>` with dot-encoded dependencies. The case number links the branch to the department's case tracking system (the FileMaker Pro database, which Raymond has reluctantly connected via a REST API wrapper he built in 2023).

Budget enforcement: conservative. Devon is allocated 70% of the task budget for drafting. If drafting exceeds allocation, the task is paused and escalated to Margaret for a decision: continue with additional budget, or produce a partial draft tagged `INCOMPLETE — MANUAL COMPLETION REQUIRED`.

Abstract mode: when budget is insufficient for full patch generation, the agent produces a structured memo describing the intended change in the department's standard memo format. This memo is a valid work product that a human planner can execute manually.

## 4. Polyrepo PR Coordination (RFP 3.4)

Forge adapter:

```
trait ForgeAdapter {
    fn create_pr(&self, repo: &RepoRef, spec: &PrSpec) -> Result<PrId>;
    fn post_memo(&self, pr: &PrId, memo: &DepartmentMemo) -> Result<CommentId>;
    fn list_memos(&self, pr: &PrId) -> Result<Vec<DepartmentMemo>>;
    fn get_status(&self, pr: &PrId) -> Result<PrStatus>;
    fn cross_reference(&self, ref_str: &str) -> Result<PrId>;
}
```

PR comments use the department's memo format. A `DepartmentMemo` has fields: `subject`, `reference` (zoning code section), `from`, `date`, `body`, `action_required`, `classification` (Routine/Policy/Discretionary), `signature`.

GitHub-only implementation. Raymond will consider adding GitLab support when the department's GitHub enterprise license is renewed in FY2027.

Cross-repo coordination: PR-based with memo-format comments. Dependency tracking via PR labels matching the department's case numbering system.

## 5. Agent Memory and Identity (RFP 3.5)

Memory stored under `refs/dczr/memory/<agent>/` as Git blobs. The memory system mirrors the department's case file structure.

Each memory entry is a JSON object with: `case_number`, `district`, `request_type` (variance, conditional use, rezoning), `decision` (approved, denied, withdrawn), `rationale`, `code_sections` (array of referenced sections), `date`, `appeal_status`, `ttl`.

Retrieval: structured query by district, request type, and code section. "Has a height variance been granted in the C-4 district?" returns matching case files sorted by date. No semantic search — Dolores insists that precedent retrieval must be deterministic. The same query must always return the same results.

Relevance: no decay model. Government records do not expire based on relevance — they expire based on retention schedules. Entries are retained for the period specified by Ohio's public records retention schedule (typically 5 years for routine records, permanent for policy decisions).

Compaction survival: all entries classified as "Policy" survive compaction permanently. "Routine" entries survive for 2 years. "Discretionary" entries survive for 5 years.

Identity: stored at `refs/dczr/identity/<agent>`. Includes: name, title, department, authorization level (Director, Senior, Junior, Support), case assignment list, and OpenWallet key fingerprint. Authorization levels map directly to the department's organizational chart.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Every commit signed. Signing flow includes records classification.

Flow: INDEX.patch + COMMIT.msg produced -> Dolores reviews for precedent accuracy -> Margaret authorizes -> Patricia classifies (Routine/Policy/Discretionary) -> commit object constructed with classification trailer -> Ed25519 signature via OpenWallet -> signed commit finalized.

Authorization policy:

```toml
[agents.devon]
branches = ["dczr/*/draft"]
max_patch_lines = 400
requires_authorization = "margaret"

[agents.dolores]
branches = ["dczr/*/review"]
max_patch_lines = 100

[agents.margaret]
branches = ["dczr/*"]
max_patch_lines = 50
```

Key rotation: 90-day cycle (aligned with the city's quarterly IT audit schedule). Compromise revocation triggers a formal incident report filed with the city's IT security office.

## 7. Token Budget

| Component | Input | Output | Frequency | Notes |
|-----------|-------|--------|-----------|-------|
| System prompt | 3,500 | 0 | Once/session | Identity, tools, department procedures |
| Task receipt | 2,000 | 300 | Once/task | Case assignment, applicable code sections |
| Precedent research | 2,500 | 400 | 2/task | Case file queries, deterministic retrieval |
| Planning | 1,000 | 500 | Once/task | Code cross-referencing, scope assessment |
| Tool calls (per call) | 1,000 | 500 | ~4/task | Parameter + result processing |
| Patch drafting | 3,500 | 5,000 | Once/task | Context + diff with zoning references |
| Commit message | 800 | 400 | Once/task | Case number, code refs, classification |
| Review cycle | 1,500 | 300 | 1/task | Dolores's precedent check |
| Authorization | 500 | 200 | 1/task | Margaret's sign-off |
| **TOTAL (typical task)** | **20,300** | **10,100** | -- | 200-line, 3 files, 1 case |

## Unique Insight

The department has processed 23,000 zoning cases in 38 years. Every case followed the same procedure. That procedure has never produced a decision that was reversed for procedural error. The insight is unfashionable but true: the most reliable systems are the ones that resist innovation. Not because innovation is bad, but because untested changes in high-stakes systems produce failures that are expensive to reverse. Our proposal is deliberately conservative. It introduces AI tooling within the existing procedural framework rather than replacing it. The department's procedures are the test suite. Any agent output that does not conform to procedure is a failing test, regardless of how technically impressive it is.

---

*"The procedure has been followed. The record reflects this."*
