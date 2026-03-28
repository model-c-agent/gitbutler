# PROPOSAL.md — Stage Operations Command

**"STAGE OPS to all stations: Proposal follows. Acknowledge."**

---

## Summary

Stage Operations Command proposes to build the `but-ai` plugin as a military-grade production operation. Tasks are missions. Patches are executed orders. Memory is the operations log. All communication follows comms discipline: callsign, message, acknowledge. The result is an agent system with zero ambiguity, full traceability, and the discipline of a carrier flight deck.

---

## Technical Proposal

### Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is deployed to PATH as a standard executable. STAGEOPS treats binary deployment as equipment staging: the tool must be in position, inspected, and ready before the mission begins. The manifest file is the equipment manifest — a structured declaration of available subcommands, provider configuration, and capability status. Discovery: PATH lookup. No runtime auto-discovery. If the binary is not on PATH, the mission does not proceed.

### Requirement 2: Provider-Agnostic AI

Different theaters, same comms protocol. The provider abstraction layer implements a `Comms` trait: `transmit(prompt) -> Completion`, `decode(completion) -> ToolCalls`, `log_traffic(usage) -> TokenReport`. Each provider adapter (OpenAI, Anthropic, Ollama, LMStudio) implements `Comms`. The adapter handles provider-specific encoding; the operations layer sees only structured messages.

STAGEOPS adds a communications verification step: every completion is checked for structural integrity before processing. Malformed completions (missing finish reason, inconsistent token counts) are logged as "comms errors" and retried once. Second failure triggers a provider failover.

### Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The agent workflow follows the operations cycle:

1. **Mission brief** — STAGE OPS decomposes the task into an operations plan with a cue sheet
2. **Memory retrieval** — BOARD OP retrieves relevant cues from the operations log
3. **Execution** — TECH DIR generates INDEX.patch + COMMIT.msg per the operations plan
4. **Coordination** — FLY RAIL handles cross-repo dependencies and PR sequencing
5. **Review** — STAGE OPS reviews output against the cue sheet, approves or orders revision
6. **Log** — BOARD OP records mission completion

TECH DIR's COMMIT.msg follows SITREP format:
```
SITUATION: Auth module lacked token refresh capability
ACTION: Added sliding window refresh with 24h max TTL
RESULT: Tokens now auto-refresh without re-authentication
REF: STAGEOPS-MISSION-2026-0047
```

### Requirement 4: Polyrepo PR Coordination

FLY RAIL's domain. Cross-repo coordination follows the same comms protocol used for multi-venue touring productions. PR comments are formatted as operational messages:

```
[STAGEOPS-SIGNAL] from: FLY RAIL | to: repo-backend | status: STANDING BY
depends: repo-auth (COMPLETE) | action: CLEARED TO PROCEED
mission: STAGEOPS-MISSION-2026-0047
```

Forge-agnostic: FLY RAIL implements a `Venue` trait: `post_signal(signal)`, `read_signals(venue) -> Vec<Signal>`, `acknowledge_signal(signal_id)`. GitHub, GitLab, and Gitea each implement `Venue`. The signal format is identical across all venues.

### Requirement 5: Agent Memory in Git Branches

Memory is the operations log, stored in `refs/stageops/log/` as Git blobs. Each entry is a cue:

```json
{
  "cue_number": "Q-4217",
  "mission": "STAGEOPS-MISSION-2026-0047",
  "timestamp": "2026-03-28T14:30:00Z",
  "agent": "TECH DIR",
  "action": "Identified JWT refresh pattern in src/auth/",
  "classification": "INTEL",
  "ttl_hours": 720
}
```

**Cue classifications:**
- `INTEL` — information gathered during reconnaissance (file reads, branch analysis)
- `DECISION` — a choice made during the mission (approach selected, alternative rejected)
- `HAZARD` — a risk identified (conflicting state, unexpected file structure)
- `OUTCOME` — result of an action (patch applied, test passed/failed)

Retrieval is by mission (all cues for a task), by classification (all HAZARDs across missions), or by agent (all cues logged by TECH DIR). BOARD OP performs all memory operations.

**Unique memory feature:** The operations log includes a `REPLAY` capability. Given a mission ID, the entire sequence of cues can be replayed in order, reconstructing the decision history. This enables post-mission debrief and allows new agents to understand not just what was done, but in what order and why.

### Requirement 6: Signed Commits via OpenWallet

STAGE OPS is the sole signing authority. Cmdr. Reyes's DID-bound key signs every commit via OpenWallet. The signing protocol follows the military chain of custody: TECH DIR produces the artifact, FLY RAIL confirms alignment, BOARD OP logs the pre-signing state, and STAGE OPS signs. The signing event itself is logged with the same fidelity as any other operational action.

Key management follows STAGEOPS security protocols: quarterly rotation with a 24-hour overlap window, emergency rotation within 2 hours, and a documented key ceremony that requires physical presence (or, in the agent context, an explicit rotation command with countersignature from TECH DIR).

---

## Token Budget

| Callsign | Input | Output | Total | Role |
|----------|-------|--------|-------|------|
| STAGE OPS | 6,500 | 2,500 | 9,000 | Command, review, signing |
| TECH DIR | 9,000 | 7,000 | 16,000 | Patch generation |
| FLY RAIL | 6,500 | 3,000 | 9,500 | Coordination |
| BOARD OP | 5,500 | 1,500 | 7,000 | Memory, logging |
| **Team Total** | **27,500** | **14,000** | **41,500** | |

Comms overhead: ~3,000 tokens (protocol messages, acknowledgments, logging).
**Total per task: ~44,500 tokens.**

---

## Unique Insight

**Acknowledgment is not overhead — it is verification.** In military comms, every message is acknowledged by the receiver. Most agent systems treat inter-agent communication as fire-and-forget: agent A sends a message, agent B is assumed to have received it. STAGEOPS requires explicit acknowledgment. When STAGE OPS orders TECH DIR to generate a patch, TECH DIR acknowledges the order before beginning work. This costs tokens but eliminates a class of failures where an agent proceeds on a misunderstood instruction. On a carrier flight deck, the cost of a misunderstood order is a crashed aircraft. In an agent system, it is a wasted token budget on an incorrect patch. The acknowledgment protocol costs ~500 tokens per task and prevents re-runs that cost ~15,000.

---

*"All stations: secure from operations. Mission complete. Good show. Out."*
