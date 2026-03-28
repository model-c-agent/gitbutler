# Custodians of Jericho — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

A Rust crate at `crates/but-ai/`. Small, auditable. Every line of code must be reviewable by the council. We prefer a smaller codebase that we understand completely over a larger codebase with blind spots.

CLI subcommands: `agent serve` (execute task with council deliberation), `witness` (read-only observation), `memory`, `mcp`.

The `witness` subcommand produces a comprehensive workspace observation without modifying anything. It is the digital equivalent of a site visit — look, document, leave no trace.

MCP mode: standard `ServerHandler` implementation. All `WorkspaceToolset` tools plus `CouncilServe`, `WitnessReport`, `MemoryQuery`.

**WASI:** Under WASI, the plugin operates in witness-only mode. Observation and memory are available. Modification is not. The Custodians are comfortable with this — sometimes the right action is to observe and report, not to intervene.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` directly. We prefer providers that support structured output (for the witness's formal site records) and tool calling (for the steward's workspace interactions). If a provider lacks these capabilities, the agent degrades to text-only mode with manual JSON parsing.

We do not propose a plugin system for new providers. Adding a provider means adding a configuration entry and ensuring it passes our verification: a 20-item checklist covering reliability, consistency, and data handling. We do not need dozens of providers. We need two or three that work reliably.

## 3. The But Agent (RFP 3.3)

The agent follows the council model:

1. **Witness phase** — Observe workspace, retrieve memory, produce site record
2. **Deliberation** — All three agents review the site record and agree on approach
3. **Steward phase** — Produce `INDEX.patch` + `COMMIT.msg` with minimal intervention
4. **Guardian phase** — Verify patch, check authorization, sign commit

**Minimal intervention principle:** The steward produces the smallest correct patch. It does not refactor surrounding code. It does not add features beyond the task scope. It does not "clean up" things it notices. Each intervention beyond the task scope requires a separate task and a separate council deliberation.

**Branch naming:** `coj/<record-id>/<deps>`. Example: `coj/R042/s01`.

**Budget enforcement:** The witness receives 30%, the steward 45%, the guardian 20%, reserve 5%. The witness's allocation is generous because thorough observation prevents the need for revision.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait SiteChannel {
    fn open_record(&self, repo: &str, record: RecordSpec) -> Result<RecordId>;
    fn add_testimony(&self, record: RecordId, msg: Testimony) -> Result<()>;
    fn read_testimonies(&self, record: RecordId) -> Result<Vec<Testimony>>;
}
```

Three methods. Minimal surface area. PRs are "records." Comments are "testimonies." Each testimony is signed.

**PR comment schema:**

```json
{
  "schema": "coj-testimony/v1",
  "record": "R042",
  "from": "witness | steward | guardian",
  "type": "observation | action | verification",
  "content": "...",
  "consensus": "unanimous | majority | judgment_call",
  "signature": "<OpenWallet-signature>"
}
```

The `consensus` field is critical — it documents the quality of agreement behind each action. External reviewers can see at a glance whether the council was fully aligned.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/coj/memory/` namespace with three sub-namespaces:

- `witness/` — Observation records: workspace patterns, codebase conventions, historical context
- `steward/` — Implementation precedents: past patches and their outcomes
- `guardian/` — Verification heuristics: common error patterns, authorization precedents

**Relevance scoring:** The Custodians use a "layer" model inspired by stratigraphy. Memory entries are organized by the "layer" (time period and project context) in which they were created. When a new task arrives, the witness identifies which layer the task belongs to and retrieves entries from that layer and adjacent layers. Distant layers are not retrieved unless explicitly requested.

**TTL:** Witness entries: 14 days (observations become stale). Steward entries: 30 days (implementation patterns persist). Guardian entries: 60 days (verification heuristics are hard-won). Entries accessed within their TTL have the timer reset.

**Compaction survival:** Each agent maintains a single "testimony" — a compressed statement of its role, its ethical commitments, and its three most important learned principles. Testimonies are always preserved. Everything else is expendable.

**Identity:** Stored at `refs/coj/identity/<agent>`. Fields: name, role, ethical charter reference, authorized operations, public key, ordination date (when the agent was commissioned into the council). Identity records are signed by all three council members.

**Long-term storage:** A shared `refs/coj/chronicle` stores cross-session wisdom. Entries are added only by unanimous council decision. The chronicle is small (target: fewer than 100 entries) and curated. It represents the order's accumulated understanding.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The guardian signs. Signing requires unanimous council approval: the witness and steward must both have assented to the final artifact before the guardian signs.

The signed commit includes a custom header: `X-Coj-Consensus: unanimous` (or `judgment_call` if the duty-of-care provision was invoked). This header is verified by the guardian and cannot be forged without the signing key.

**Authorization:**

```toml
[agents.witness]
branches = []
operations = ["observe"]

[agents.steward]
branches = ["coj/*"]
operations = ["produce-patch"]
max_patch_lines = 400

[agents.guardian]
branches = ["coj/*", "feat/*", "fix/*"]
operations = ["verify", "sign"]
require_consensus = true
```

**Key lifecycle:** Keys are provisioned in a three-agent ceremony (all council members present). Rotation every 60 days. Compromise: immediate revocation, the chronicle is consulted for precedents on how to handle the affected commits, and the guardian initiates a full review. The Custodians do not rush revocation responses — they deliberate, even under pressure.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Ethical charter + tool descriptions |
| Witness observation | 3,500 | 800 | Once/task | Comprehensive site record |
| Council deliberation | 1,500 | 500 | Once/task | Consensus exchange |
| Steward implementation | 2,800 | 4,200 | Once/task | Minimal intervention patch |
| Guardian verification | 3,000 | 400 | Once/task | Patch + record comparison |
| Commit message | 600 | 300 | Once/task | Referenced, attributed |
| Memory retrieval | 500 | 150 | 1/task | Stratigraphic layer lookup |
| Coordination event | 800 | 300 | 0-1/task | Testimony exchange |
| **TOTAL (typical task)** | **15,500** | **6,650** | -- | Council overhead included |

## Unique Insight

In 37 years of multi-faith site preservation, the Custodians have learned that the most effective preservation strategy is not the one designed by the most qualified expert. It is the one that all stakeholders have genuinely agreed to. A technically inferior plan with universal buy-in outperforms a technically superior plan with grudging compliance, because the first plan gets implemented faithfully and the second gets quietly sabotaged.

This applies directly to AI agent systems. An agent architecture that the entire team understands and endorses — even if it is simpler than the optimal design — will produce better results than a sophisticated architecture that half the team mistrusts. Our council model is not the most efficient. But it is the most trustworthy, because every output carries the genuine assent of every agent involved.

Consensus is not a bottleneck. Consensus is the product.

---

*"What three agree to protect, three will protect."*
