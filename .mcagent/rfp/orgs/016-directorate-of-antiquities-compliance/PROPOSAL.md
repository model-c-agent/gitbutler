# Directorate of Antiquities Compliance — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**
**Reference Number:** DAC-2026-TECH-RFP-001
**Prepared by:** Technical Compliance Division

---

## 1. Plugin Architecture (RFP 3.1)

The Directorate proposes a Rust binary crate at `crates/but-ai/`, structured in accordance with the existing workspace conventions. The crate shall comprise four modules, corresponding to the four agents: `compliance` (specification), `inspection` (validation), `records` (memory and archival), and `certification` (signing and authorization).

CLI subcommands: `agent permit` (submit a task for processing — we use "permit" because no work proceeds without one), `agent inspect` (validate an existing patch), `records query` (search memory), `records archive` (store memory), `mcp`.

The `inspect` subcommand is available standalone: given a patch file, it runs the Directorate's compliance checks and produces an inspection report. This allows external agents or human developers to validate their work against our standards.

MCP mode: `ServerHandler` implementation, backward-compatible. All `WorkspaceToolset` tools plus `PermitTask`, `InspectPatch`, `QueryRecords`.

**WASI:** The Directorate proposes "restricted operations" mode under WASI. The plugin announces: "Operating under restricted environment. Inspection and records operations available. Permit processing requires standard environment." Clear. Unambiguous.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

The Directorate shall use the `but-llm` crate without modification, as stipulated. Provider certification follows Directorate standard procedure:

- Each provider is tested against a 45-item conformance checklist
- Providers that pass all items are classified "Certified"
- Providers that pass 40-44 items are classified "Provisional"
- Providers below 40 items are classified "Non-compliant" and excluded

Current classifications: OpenAI (Certified), Anthropic (Certified), Ollama (Provisional — structured output inconsistency on smaller models), LMStudio (Provisional — variable quality by model).

New providers: submit a certification request (a TOML config entry), pass the conformance checklist, receive classification. The checklist is shipped with the plugin and can be run via `but ai provider certify <name>`.

## 3. The But Agent (RFP 3.3)

The agent follows the Directorate's permit process:

1. **Application** — Compliance officer receives task, produces formal work order with scope, standards, and acceptance criteria
2. **Survey** — Records clerk gathers context (workspace state, memory, branch topology) and prepares a context package
3. **Implementation** — Inspector produces `INDEX.patch` + `COMMIT.msg` in accordance with the work order
4. **Inspection** — Inspector validates own output against the work order (self-inspection with documented checklist)
5. **Certification** — Certifier reviews the full chain: work order → context → patch → inspection report → signs

**Branch naming:** `dac/<permit-number>/<deps>`. Example: `dac/P2026-0012/s01.s02`.

**Budget enforcement:** Budget is allocated by the compliance officer in the work order:
- Application + survey: 25%
- Implementation: 45%
- Inspection: 15%
- Certification: 10%
- Contingency: 5%

The contingency allocation is the Directorate's innovation: a small reserve for unexpected costs (additional tool calls, expanded context requirements). If the contingency is not used, it is returned to the budget pool.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait ComplianceChannel {
    fn file_permit(&self, repo: &str, pr: PermitApplication) -> Result<PermitId>;
    fn submit_report(&self, permit: PermitId, report: InspectionReport) -> Result<()>;
    fn retrieve_reports(&self, permit: PermitId) -> Result<Vec<InspectionReport>>;
    fn register_dependency(&self, from: PermitId, to: PermitId) -> Result<()>;
    fn issue_certification(&self, permit: PermitId, cert: Certification) -> Result<()>;
}
```

Five methods. GitHub reference implementation. Each method produces a documented artifact.

**PR comment schema:** Formal inspection report format:

```json
{
  "directorate_schema": "dac-report/v1",
  "permit_number": "P2026-0012",
  "report_type": "inspection | certification | status | dependency",
  "agent": "<role>",
  "timestamp": "<ISO-8601>",
  "findings": [ ... ],
  "recommendation": "approve | revise | reject",
  "signature": "<OpenWallet-signature>"
}
```

Every report includes findings (even if the finding is "no issues detected") and a recommendation. The certifier's final approval references all prior reports.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/dac/records/` namespace, organized as a filing system:

- `permits/` — Past work orders and their outcomes
- `inspections/` — Past inspection reports
- `standards/` — Standing compliance rules (infrequently updated)
- `registry/` — Agent identity and certification records

**Relevance scoring:** The records clerk uses a precedent-based system. When a new task arrives, the clerk searches past permits with similar scope and standards. Relevance is scored by: (1) scope overlap (40%), (2) standard overlap (30%), (3) outcome success (20%), (4) recency (10%). Top 3 precedents are injected into context.

**TTL:** Permits and inspections: 30 days (recent precedents are most useful). Standards: indefinite (standing rules do not expire). Registry: indefinite.

**Compaction survival:** Each agent maintains a "credential" — a compact document containing its role, its authority, and the three most relevant standing standards for the current project. Credentials are always preserved during compaction.

**Identity:** Agent identities are "certification records" in the registry: name, role, certified capabilities, authorized operations, public key fingerprint, certification date, and certifying authority (the certifier agent signs all identity records).

## 6. Signed Commits via OpenWallet (RFP 3.6)

The certifier is the sole signing authority. Signing requires: (1) a valid work order from the compliance officer, (2) an inspection report from the inspector with "approve" recommendation, (3) a complete records entry from the clerk.

**Authorization:** Policy stored in `.but-ai/dac-policy.toml`:

```toml
[agents.compliance-officer]
operations = ["specify", "create-branch"]
branches = ["dac/spec/*"]

[agents.inspector]
operations = ["implement", "inspect"]
branches = ["dac/*"]

[agents.certifier]
operations = ["sign", "certify"]
branches = ["dac/*", "feat/*"]
require_inspection_report = true
```

**Key lifecycle:** Keys issued through a formal certification ceremony documented in the records. Rotation every 60 days (the Directorate prefers longer cycles with thorough documentation). Compromise handling: the certifier revokes the key, the records clerk flags all affected commits, and the inspector re-inspects each flagged commit.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,800 | 0 | Once/session | Includes standing standards |
| Work order | 2,000 | 2,000 | Once/task | Formal specification |
| Context survey | 3,000 | 400 | Once/task | Records clerk preparation |
| Tool call (per call) | 1,100 | 500 | ~4/task | Data gathering |
| Implementation | 3,000 | 4,000 | Once/task | Inspector produces patch |
| Inspection report | 2,500 | 800 | Once/task | Formal validation |
| Commit message | 700 | 400 | Once/task | Referenced format |
| Certification | 1,500 | 300 | Once/task | Final review and signing |
| Memory retrieval | 600 | 200 | 2/task | Precedent search |
| Coordination event | 1,200 | 500 | 1/task | Formal report |
| **TOTAL (typical task)** | **23,800** | **12,100** | -- | Full compliance cycle |

## Unique Insight

In 70 years of archaeological compliance, the Directorate has learned that the most destructive excavation is not the one conducted by a careless archaeologist — it is the one conducted by a careful archaeologist working from an incorrect permit. When the authorization does not match the reality, the most meticulous work produces the most irrecoverable damage.

The same applies to AI agents. An agent working from a correct specification produces correct patches, even if the agent is imperfect. An agent working from an incorrect specification produces confidently wrong patches that are harder to detect and harder to revert than obviously broken ones.

Our investment in specification quality — the compliance officer's detailed work orders — is not overhead. It is the primary defense against the most dangerous failure mode: doing the wrong thing perfectly.

---

*"Authorization precedes action. Always."*

**Filed:** DAC Records, Reference DAC-2026-TECH-RFP-001-SUBMISSION
