# Admiralty Bureau of Freight Standards — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**
**Proposal Number:** ABFS-2026-RFP-001
**Classification:** Technical Submission, Unrestricted

---

## Preamble

This proposal has been prepared in accordance with Bureau Standard Practice 47 (Preparation of Technical Submissions to External Bodies) and approved by the Committee under Resolution ABFS-C-2026-0043. All technical claims herein are supported by the Bureau's internal testing records, available upon formal written request (Form 12-A, Request for Disclosure of Internal Technical Documentation).

## 1. Plugin Architecture (RFP 3.1)

The Bureau proposes a Rust binary crate, `crates/but-ai/`, structured in strict accordance with the existing `but` workspace conventions. The crate shall expose two operational modes:

1. **CLI mode** — Subcommands: `agent propose` (submit task for committee deliberation), `agent status` (query committee status), `memory query`, `memory archive`, `audit trail`, and `mcp`.
2. **MCP server mode** — Implements `ServerHandler` via `rmcp`. Registers all ten `WorkspaceToolset` tools plus Bureau-specific tools: `ProposeTask`, `QueryAudit`, `MemoryRetrieve`.

All subcommands produce output conforming to Bureau Output Standard 3 (BOS-3): structured JSON when `BUT_JSON=1`, tabular human-readable text otherwise. Error messages include a Bureau error code, a human-readable explanation, and a reference to the applicable standard.

**WASI compatibility:** The Bureau proposes a "reduced capability certificate" model. Under WASI, the plugin issues a formal notification at startup listing capabilities that are unavailable (plugin discovery, fork/exec, forge API access). The remaining capabilities operate normally. The notification is logged and retrievable via `but ai audit`.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

The Bureau shall use the `but-llm` crate without modification, as mandated by the RFP. Provider selection follows Git config precedence rules. The committee has reviewed all four existing providers and certified them as "fit for Bureau use" under the following conditions:

- OpenAI and Anthropic: certified for all operations
- Ollama: certified for patch generation and memory queries; not certified for structured output (insufficient reliability in Bureau testing)
- LMStudio: certified with advisory — local model quality varies; Bureau recommends minimum 70B parameter models for patch generation

**New provider registration:** The Bureau proposes a provider certification process. A new provider must pass a conformance test suite (60 tests covering tool calling, structured output, streaming, and error handling) before it is registered. Registration is via a `[but-ai.providers.<name>]` section in Git config.

## 3. The But Agent (RFP 3.3)

The agent operates under the Bureau's Deliberative Execution Model:

1. **Receipt** — Task is received and assigned a Proposal Number
2. **Specification** — Policy agent produces a formal specification of the required changes
3. **Implementation** — Full-stack agent produces `INDEX.patch` + `COMMIT.msg`
4. **Inspection** — Inspection agent validates patch against specification
5. **Approval** — Committee votes. Unanimous approval required
6. **Archive** — All artifacts (specification, patch, inspection report, vote record) are archived

**Branch naming:** `abfs/<proposal-number>/<dependency-chain>`. Example: `abfs/P2026-0043/s01.s02`.

**Budget enforcement:** The Bureau allocates budget by phase. If specification consumes more than 30% of total budget, the chair convenes an emergency review to determine whether the task should proceed or be returned as "insufficiently scoped for available resources."

**Partial results:** If budget is exhausted mid-implementation, the agent produces a formal "Incomplete Work Notice" containing: what was completed, what remains, estimated additional budget required, and a recommendation for next steps. This notice is committed as the COMMIT.msg with a `PARTIAL` prefix.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

The Bureau defines a `ForgeComplianceAdapter` trait:

```
trait ForgeComplianceAdapter {
    fn submit_pr(&self, spec: &PrSpecification) -> Result<PrRegistration>;
    fn file_comment(&self, pr: &PrRegistration, msg: &OfficialCommunication) -> Result<CommentReceipt>;
    fn retrieve_communications(&self, pr: &PrRegistration) -> Result<Vec<OfficialCommunication>>;
    fn register_dependency(&self, from: &PrRegistration, to: &PrRegistration) -> Result<DependencyRecord>;
    fn certify_status(&self, pr: &PrRegistration) -> Result<StatusCertificate>;
}
```

Reference implementation: GitHub REST API. Each method returns a typed receipt that serves as proof of action for the audit trail.

**PR comment schema:** All agent communications are formatted as Bureau Official Communications:

```json
{
  "bureau_schema": "abfs-comm/v1",
  "proposal_number": "P2026-0043",
  "communication_type": "inspection_report | status_certificate | dependency_declaration | task_assignment",
  "from_agent": "<agent-name>",
  "timestamp": "<ISO-8601>",
  "content": { ... },
  "signature": "<OpenWallet-signature>",
  "filing_reference": "<archive-ref>"
}
```

Every communication is filed in the archive. The archivist maintains an index.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** The Bureau maintains a `refs/abfs/archive/` namespace. Memory is organized by classification:

- `refs/abfs/archive/decisions/` — Architectural and procedural decisions
- `refs/abfs/archive/inspections/` — Inspection reports and validation results
- `refs/abfs/archive/precedents/` — Past task outcomes used for future reference
- `refs/abfs/archive/identity/` — Agent certification records

Each entry is a JSON document stored as a Git blob, indexed by a tree object at the classification root.

**Relevance scoring:** The Bureau uses precedent-based retrieval. When a new task arrives, the policy agent searches the archive for tasks with similar classifications (using the Bureau's 312-category taxonomy adapted for software). Matching precedents are ranked by (1) classification match depth, (2) recency, (3) outcome success. Top 3 precedents are injected into context.

**TTL and retention:** The Bureau does not believe in automatic deletion. Memory entries are "retired" — moved from active index to cold storage — after their TTL expires, but they remain in the Git object store. A retired entry can be "recalled from retirement" if a future task matches its classification.

**Identity:** Agent identity is a "Certificate of Competency" stored at `refs/abfs/archive/identity/<agent-name>`. The certificate includes: name, role, competency scope (which task categories the agent is certified to handle), authorized branches, public key, and issuance date. Certificates are renewed annually.

## 6. Signed Commits via OpenWallet (RFP 3.6)

Commit signing is treated as a Bureau ceremony. The signing flow:

1. Agent produces `INDEX.patch` + `COMMIT.msg`
2. Inspection agent verifies patch against specification
3. Committee issues formal approval (recorded in archive)
4. Archivist invokes OpenWallet signing with the committee's collective key
5. The signed commit includes the Proposal Number in its metadata

**Authorization:** Policy is codified in Bureau Standard Practice 88 (Agent Authorization for Repository Operations):

```toml
[authorization.policy-agent]
branches = ["abfs/spec/*"]
operations = ["commit", "create-branch"]
max_patch_lines = 0  # Policy agent does not produce patches

[authorization.inspection-agent]
branches = ["abfs/*"]
operations = ["read", "comment"]
max_patch_lines = 0  # Inspection agent does not produce patches

[authorization.committee]
branches = ["abfs/*", "feat/*"]
operations = ["commit", "create-branch", "merge"]
max_patch_lines = 1000
```

**Key lifecycle:** Keys are provisioned through a formal ceremony requiring all four agents present. Rotation occurs every 90 days (the Bureau prefers longer rotation cycles with thorough ceremony over frequent automated rotation). Compromise response follows Bureau Incident Procedure 14: immediate key revocation, notification to all dependent repositories, and a formal investigation documented in the archive.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 4,000 | 0 | Once/session | Includes Bureau standards references |
| Task ingestion | 2,500 | 300 | Once/task | Classification and Proposal Number assignment |
| Specification drafting | 2,000 | 2,500 | Once/task | Policy agent formal specification |
| Planning | 1,000 | 500 | Once/task | Committee deliberation |
| Tool call (per call) | 1,200 | 500 | ~4/task | Inspection and data gathering |
| Patch generation | 3,000 | 4,000 | Once/task | Implementation against specification |
| Inspection | 2,500 | 800 | Once/task | Validation report |
| Commit message | 800 | 400 | Once/task | Formal, referenced format |
| Memory retrieval | 800 | 200 | 2/task | Precedent search |
| Coordination event | 1,500 | 600 | 1/task | Official communication |
| Archive filing | 500 | 500 | Once/task | Archivist recording |
| **TOTAL (typical task)** | **24,600** | **13,300** | -- | Includes deliberation overhead |

The Bureau acknowledges this is higher than most proposals. The Bureau does not apologise for thoroughness.

## Unique Insight

In 77 years of cataloguing failures, the Bureau has observed that 68% of all maritime logistics failures trace back to a single root cause: an assumption that was never written down. A container was stowed incorrectly because someone assumed the manifest was in metric tons when it was in short tons. A ship was misrouted because someone assumed the port code "HAM" meant Hamburg when it meant Hamilton.

The same pattern applies to AI agents. When an agent "hallucinates," it is often because an assumption in its context was never made explicit. Our approach — formal specification before implementation, inspection after implementation — exists to surface assumptions. The specification forces the agent to state its understanding of the task. The inspection verifies that the implementation matches the stated understanding. The overhead is real. The alternative — unwritten assumptions — is more expensive.

---

*"There is always a form for that. If there is not, we shall create one."*

**Filed:** ABFS Archive, Reference ABFS-2026-RFP-001-SUBMISSION
**Classification:** Technical Proposal, Open Distribution
