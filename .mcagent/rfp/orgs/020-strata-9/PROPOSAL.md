# Strata-9 — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. Hardened build: all dependencies audited, no network calls at startup, no telemetry. The binary should be usable in air-gapped environments.

CLI subcommands: `agent investigate` (execute task), `agent trace` (provenance audit of a commit — who produced it, from what data, with what authorization), `evidence query` (search memory), `evidence seal` (store memory with cryptographic integrity check), `mcp`.

The `trace` subcommand is our signature tool: given a commit hash, it reconstructs the full evidence chain — which agent, which task, which context data, which memory entries were in scope. This is provenance for code, the same way we trace provenance for artifacts.

MCP mode: drop-in replacement. All `WorkspaceToolset` tools plus `Investigate`, `Trace`, `EvidenceQuery`.

**WASI:** Under WASI, `trace` and `evidence query` work (local computation only). Investigation is unavailable. The plugin announces its restrictions and suggests running in a full environment for investigation tasks.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` directly. We trust no provider with sensitive data, so all prompts are scrubbed of identifying information before being sent. The scrubbing layer sits between the agent and `but-llm`: it replaces file paths with hashes, repository names with codes, and personal names with placeholders. The response is de-scrubbed before being used.

This is paranoid. We know. But in our line of work, an API provider that logs prompts could compromise an investigation.

For new providers: static configuration only. No runtime loading. Every provider binary must be present at compile time and pass our audit. We will add providers when we have audited them.

## 3. The But Agent (RFP 3.3)

Investigation model:

1. **Brief** — Analyst receives task, identifies leads, assigns scope boundaries
2. **Investigate** — Investigators work leads independently, producing patches
3. **Log** — Archivist receives all outputs, verifies provenance, logs evidence
4. **Seal** — Archivist signs commits after provenance verification

**Branch naming:** `s9/<cell>/<lead-id>`. Example: `s9/alpha/L042`.

**Budget enforcement:** Analyst receives 15%. Each investigator receives 35%. Archivist receives 15%. No sharing. If an investigator runs out of budget, they produce whatever partial result they have. The archivist logs partial results the same way — partial evidence is still evidence.

**Provenance tagging:** Every patch includes metadata (in COMMIT.msg) documenting: which workspace tools were called, which memory entries were referenced, and which context files were read. This is the agent's evidence log — every decision is traceable.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait EvidenceChannel {
    fn open_case(&self, repo: &str, case: CaseSpec) -> Result<CaseId>;
    fn file_evidence(&self, case: CaseId, evidence: SignedEvidence) -> Result<()>;
    fn retrieve_evidence(&self, case: CaseId) -> Result<Vec<SignedEvidence>>;
    fn cross_reference(&self, from: CaseId, to: CaseId) -> Result<()>;
}
```

PRs are "cases." Comments are "evidence filings." Every filing is signed. Unsigned filings are rejected.

**PR comment schema:**

```json
{
  "schema": "s9-evidence/v1",
  "case": "L042",
  "from": "s9/alpha",
  "type": "finding | status | dependency | handoff",
  "provenance": {
    "tools_called": ["GetProjectStatus", "GetBranchChanges"],
    "memory_refs": ["patterns/error-handling", "repairs/auth-refactor"],
    "context_files": ["src/auth/mod.rs", "src/auth/provider.rs"]
  },
  "signature": "<OpenWallet-signature>"
}
```

Every message includes its provenance. If you cannot trace where the information came from, you cannot trust it.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/s9/evidence-locker/` namespace. All memory is integrity-checked — each entry includes a SHA-256 hash of its content at the time of creation. If the content is modified without updating the hash, the archivist flags it as tampered.

Structure:
- `active/` — Current investigation data
- `sealed/` — Completed investigation data (immutable after sealing)
- `methods/` — Investigation techniques and patterns (long-term)
- `identity/` — Agent records

**Relevance scoring:** The archivist uses a "connection strength" model. Each memory entry records which other entries it was used alongside (co-occurrence). When a new task retrieves one entry, entries that frequently co-occur with it are also retrieved. This builds organic clusters of related knowledge without requiring manual tagging.

**TTL:** Active entries: 14 days. Sealed entries: indefinite (evidence is never destroyed). Methods: 90 days (investigation techniques evolve slowly). Identity: indefinite.

**Compaction survival:** Each agent has an "operational cover" — a compressed identity and capability summary that survives all compaction. Additionally, the archivist maintains a "case brief" for the current investigation that persists through compaction events.

**Identity:** Pseudonymous. Agent identity files contain: handle, role, capabilities, authorized leads, public key, and an "endorsement chain" — which other agents have vouched for this agent. Identity files are stored encrypted.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The archivist signs all commits after provenance verification. A commit without a verifiable evidence chain is not signed. This is non-negotiable.

**Authorization:**

```toml
[agents.analyst]
branches = ["s9/spec/*"]
operations = ["assign-lead", "create-branch"]

[agents.alpha]
branches = ["s9/alpha/*"]
operations = ["produce-patch"]
max_lines = 600

[agents.beta]
branches = ["s9/beta/*"]
operations = ["produce-patch"]
max_lines = 400

[agents.archivist]
branches = ["s9/*", "feat/*"]
operations = ["sign", "seal-evidence"]
require_provenance = true
```

**Key lifecycle:** Keys provisioned in a multi-agent ceremony. Rotation every 30 days. Compromise: immediate revocation, all commits since last rotation are quarantined, the archivist traces the provenance of every quarantined commit. Commits with verified provenance are re-signed with the new key. Commits with broken provenance are flagged for human investigation.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,600 | 0 | Once/session | Cell roles + evidence protocol |
| Task briefing | 2,000 | 600 | Once/task | Analyst decomposes leads |
| Investigation (Alpha) | 3,500 | 4,000 | Once/task | Primary patch generation |
| Investigation (Beta) | 2,500 | 2,500 | Once/task | Supporting work |
| Provenance logging | 2,000 | 500 | Once/task | Archivist verification |
| Commit message | 500 | 300 | Once/task | Includes provenance summary |
| Memory retrieval | 500 | 150 | 2/task | Co-occurrence matching |
| Coordination event | 800 | 300 | 1/task | Signed evidence filing |
| **TOTAL (typical task)** | **16,400** | **10,350** | -- | Includes provenance overhead |

## Unique Insight

In provenance research, the most valuable skill is not finding data — it is recognizing the significance of missing data. A museum record that lists an object's acquisition date as 1970 but provides no dealer name is not incomplete by accident. An auction catalog that describes an object's origin as "European private collection" is not being discreet — it is being evasive.

We apply the same analytical lens to codebases. A module without tests is not an oversight — it is a signal. A function without documentation is telling you something about the development team's relationship with that code. A commit message that says "fix" without context is a provenance gap.

Our agents do not just read what is present. They catalog what is absent. The `trace` command reconstructs not just what an agent did but what it *could have* referenced and chose not to. The absence of a memory retrieval for a relevant pattern is as diagnostic as a faulty patch.

---

*"Every gap in the record is a question someone hoped you would not ask."*
