# Heritage Garrison Corps — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust binary crate at `crates/but-ai/`. Structured as three modules: `command` (CLI dispatch), `agent` (mission execution engine), `mcp` (server mode). The binary is statically linked — it must be deployable to environments without package managers (the Corps deploys to field conditions; the binary must work on a bare Linux install).

CLI subcommands: `agent deploy` (execute task), `agent recon` (read-only workspace assessment), `agent sitrep` (status report), `memory`, `audit`, `mcp`.

The `recon` subcommand runs the survey phase without executing any changes — it produces a structured report of the workspace state, relevant memory, and recommended approach. Useful for pre-flight checks before committing to a full mission.

MCP mode: drop-in replacement for the existing MCP server. All ten `WorkspaceToolset` tools plus `AgentDeploy`, `AgentRecon`, and `SituationReport`.

**WASI:** Under WASI, the plugin operates in "garrison mode" — defensive posture only. Read-only operations (recon, memory queries, audit) work. Write operations (agent execution, forge coordination) are disabled. The plugin announces its posture at startup.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` as the sole backend. Provider selection follows an operational readiness assessment: at startup, the survey specialist tests each configured provider with a diagnostic prompt. Providers that respond correctly within 5 seconds are marked "operational." Providers that fail are marked "non-operational" and excluded. The agent uses the highest-priority operational provider.

This mirrors the Corps' approach to equipment: test everything before the mission. Do not discover that your radio is broken mid-operation.

New providers: a Rust trait `ProviderAdapter` with three methods: `test`, `send`, `receive`. Implementation requires one file and one test. The provider must pass the 60-test conformance suite before deployment.

## 3. The But Agent (RFP 3.3)

The agent follows the Military Decision Making Process (MDMP), adapted:

1. **Receipt of Mission** — Squad leader reads task, issues warning order
2. **Mission Analysis (Recon)** — Survey specialist gathers context, produces site survey
3. **Course of Action Development** — Squad leader produces operations order (implementation plan)
4. **Execution** — Heavy engineer produces `INDEX.patch`; light engineer validates
5. **After Action** — Fortification specialist verifies provenance and signs

**Branch naming:** `hgc/<mission-id>/<phase>/<deps>`. Example: `hgc/M042/exec/s01.s03`.

**Budget enforcement:** Budget is allocated by phase using military logistics ratios:
- Recon: 20%
- Planning: 10%
- Execution: 50%
- Validation + signing: 15%
- Reserve: 5%

The reserve is sacred — it ensures the agent always has enough tokens to produce at least a partial patch and a commit message, even if execution runs over budget.

**Partial results:** If execution budget is exhausted, the heavy engineer produces a "field expedient" patch — the largest correct subset of the intended change, with a COMMIT.msg documenting what was completed and what remains as follow-on tasks.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait CommsChannel {
    fn establish(&self, repo: &str, topic: &str) -> Result<ChannelId>;
    fn transmit(&self, ch: ChannelId, msg: FieldMessage) -> Result<()>;
    fn receive(&self, ch: ChannelId) -> Result<Vec<FieldMessage>>;
    fn mark_status(&self, ch: ChannelId, status: MissionStatus) -> Result<()>;
}
```

The signals specialist handles all forge operations. No other agent touches the forge API directly.

**PR comment schema:** Formatted as a military situation report (SITREP):

```
HGC SITREP — M042 — 2026-03-28T14:00Z
FROM: hgc/heavy-eng
STATUS: MISSION COMPLETE
PATCH: 164 lines / 3 files
BUDGET: 21400/28000 tokens
DEPS: upstream/core#89 (VERIFIED)
NEXT: Follow-on task for memory indexing
```

Human-readable. Machine-parseable via line-prefix matching. The schema mirrors NATO SITREP format.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/hgc/field-records/` namespace. Organized by classification:

- `intel/` — Gathered intelligence (workspace patterns, codebase conventions, known issues)
- `orders/` — Past operations orders and their outcomes
- `lessons/` — After-action lessons learned
- `identity/` — Agent service records

**Relevance scoring:** The Corps uses a "threat proximity" model. Each memory entry is tagged with the contexts in which it was useful (which files, which domains, which patterns). When a new task arrives, the survey specialist computes overlap between the task's context and each memory entry's tags. Entries with >40% overlap are retrieved. Maximum 4 entries per retrieval.

**TTL:** Intel: 7 days (codebase changes quickly). Orders: 30 days (architectural patterns persist). Lessons: 90 days (hard-won knowledge is worth keeping). Identity: indefinite.

**Compaction survival:** Each agent maintains a "field manual page" — a single memory entry containing its role definition, tool list, and the three most important lessons learned. Field manual pages survive all compaction events.

**Identity:** Agent identities are "service records" stored at `refs/hgc/field-records/identity/<agent>`. Include: designation, rank (role), qualifications, authorized operations, public key, commissioning date, and commanding officer (which agent authorized this agent's creation).

## 6. Signed Commits via OpenWallet (RFP 3.6)

The fortification specialist is the sole signing authority. Signing requires successful validation by the light engineer — the fortification specialist will not sign an unverified patch.

**Authorization:** Policy modeled on military access control:

```toml
[clearance.heavy-engineer]
zones = ["hgc/exec/*", "feat/*"]
operations = ["produce-patch"]
max_lines = 800

[clearance.fortification-specialist]
zones = ["hgc/*", "feat/*"]
operations = ["sign", "verify"]

[clearance.squad-leader]
zones = ["hgc/*"]
operations = ["create-branch", "assign-task"]
```

**Key lifecycle:** Keys are "issued" at agent commissioning in a ceremony involving the squad leader and fortification specialist. Rotation every 45 days (synchronized with the Corps' standard key rotation schedule). Compromise response follows the Corps' "compromised position" drill: immediate revocation, all commits since last rotation are flagged for re-verification, and the fortification specialist conducts a full audit.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 3,200 | 0 | Once/session | Squad roles + tool descriptions + doctrine |
| Mission receipt | 1,800 | 200 | Once/task | Warning order |
| Reconnaissance | 3,500 | 500 | Once/task | Site survey |
| Planning | 1,000 | 600 | Once/task | Operations order |
| Tool call (per call) | 1,100 | 500 | ~4/task | Context gathering |
| Patch generation | 3,000 | 4,200 | Once/task | Heavy engineer execution |
| Validation | 2,500 | 600 | Once/task | Light engineer verification |
| Commit message | 600 | 300 | Once/task | SITREP format |
| Memory retrieval | 500 | 150 | 2/task | Threat proximity lookup |
| Coordination event | 1,000 | 500 | 1/task | Signals transmission |
| **TOTAL (typical task)** | **22,600** | **10,050** | -- | Full MDMP cycle |

## Unique Insight

In field engineering, the most dangerous moment is not when the threat arrives — it is when the threat has passed and the team relaxes. We call it "post-contact complacency." After a successful defense, engineers stop checking their work because the pressure is off.

The same pattern appears in agent systems. After a successful patch — tests pass, review approved, commit signed — the system relaxes. Memory cleanup is deferred. Audit trails are not reviewed. Budget accounting is approximated. Over time, these small neglects accumulate into systemic failures: bloated memory, broken provenance chains, budget overruns.

Our agents do not relax after success. The after-action review is mandatory, not optional. Every completed task triggers a memory cleanup, a provenance check, and a budget reconciliation. The 5% reserve budget is partially allocated to this post-mission maintenance. The wall stands because the engineer maintained it.

---

*"No site is expendable. No record is optional."*
