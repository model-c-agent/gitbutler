# Resonance Without Leaders — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. The architecture reflects our values: no privileged components. Every module can be invoked independently. There is no "main" module that orchestrates the others.

CLI subcommands: `agent jam` (execute task — we call it a "jam session"), `agent solo` (single-agent mode for simple tasks), `memory`, `mix` (combine outputs from multiple agent runs), `mcp`.

The `mix` subcommand is unique to our proposal: given two or more patches produced by competing agents for the same task, `mix` selects the best elements from each and produces a combined patch. This is the software equivalent of selecting the best take from multiple recording sessions.

MCP mode: drop-in replacement. All `WorkspaceToolset` tools plus `JamSession`, `SoloRun`, `MixPatches`.

**WASI:** Under WASI, `solo` mode works with restricted capabilities (local provider only, no forge). `jam` mode (multi-agent) is unavailable because it requires concurrent execution. `mix` works (it is local computation over existing patches).

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` without preference for any single provider. In a jam session, different agents may use different providers — one agent on Anthropic, another on OpenAI, a third on a local Ollama model. Provider diversity is a feature, not a bug: different providers have different strengths, and competitive selection across providers often produces better results than a single provider.

The system detects available providers at startup and distributes them across agents. If only one provider is configured, all agents share it. If multiple are configured, agents are assigned different providers where possible.

New providers: TOML config entry per provider. If it speaks OpenAI-compatible API, it works immediately. We add providers liberally — the more providers available, the more diverse the jam session.

## 3. The But Agent (RFP 3.3)

**Jam session model:**

1. **Call** — Task is posted to all agents. Agents self-select into roles.
2. **Jam** — All claimed roles execute simultaneously. No sequencing. No coordination during execution. Each agent produces its output independently.
3. **Mix** — Outputs are collected. If multiple patches exist, the verifier scores them and selects the best. If patches can be combined (non-overlapping changes), they are merged.
4. **Release** — The verifier signs the selected/merged patch. COMMIT.msg credits the collective, not individual agents.

The lack of coordination during execution is intentional. In music production, over-communication during a session kills creativity. The players play; the mixing happens afterward.

**Branch naming:** `rwl/<session-id>/<agent-hash>`. Each agent gets its own branch within the session. Merge happens at mix time. Example: `rwl/S042/a3f2`.

**Budget enforcement:** Total session budget is divided equally among all participating agents. Each agent manages its own allocation. If an agent exhausts its budget, its output is whatever it has produced so far — partial outputs are valid candidates for mixing.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Channel {
    fn open(&self, repo: &str, spec: ChannelSpec) -> Result<ChannelId>;
    fn broadcast(&self, ch: ChannelId, msg: &str) -> Result<()>;
    fn listen(&self, ch: ChannelId) -> Result<Vec<String>>;
}
```

Three methods. Minimalist. PRs are "channels." Comments are "broadcasts." Anyone can broadcast. Everyone listens.

**PR comment schema:** Informal, reflecting the collective's communication style:

```markdown
rwl/session S042 — jam complete

2 agents produced patches. Selected: agent a3f2 (score 0.87 vs 0.74).
156 lines, 3 files. Budget: 21000/28000 tokens.

```json
{"v":1,"session":"S042","candidates":2,"selected":"a3f2","score":0.87}
```
```

Cross-repo references use standard `owner/repo#N` format. The coordinator agent (whoever self-selected into that role) handles cross-repo work.

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Branch `rwl/memory`. Shared by all agents — there is no per-agent memory. Memory belongs to the collective.

Structure:
- `patterns/` — Code patterns and conventions
- `sessions/` — Past session outcomes (which approaches worked, which did not)
- `mix-notes/` — Notes on why certain outputs were selected over others (the collective's taste)
- `identity/` — Collective identity (not per-agent)

**Relevance scoring:** Collaborative filtering. Each memory entry records which sessions it was used in and whether those sessions succeeded. Entries that correlate with success are scored higher. This is the same recommendation algorithm used in music streaming: "sessions that used this pattern also succeeded at similar tasks."

**TTL:** Patterns: 30 days. Sessions: 14 days. Mix notes: 60 days (taste evolves slowly). The collective does not aggressively expire memory — old sessions can inspire new approaches.

**Compaction survival:** The collective maintains a single "resonance card" — a shared statement of values, conventions, and working style that all agents inherit. The resonance card is always preserved. Individual agent state is not preserved because individual agents do not have persistent identity.

**Identity:** Agents do not have individual identities. They are instances of the collective. The collective's identity is stored at `rwl/memory/identity/collective.json`: name ("Resonance Without Leaders"), values, capabilities, and the collective's public key. Individual agent instances are ephemeral — they exist for one session and then dissolve back into the collective.

**Long-term storage:** The `mix-notes/` directory serves as the collective's long-term taste memory. Over time, it accumulates a record of what the collective considers "good" output — which patches were selected, which were rejected, and why. New agent instances inherit this taste through the resonance card and mix-notes.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The collective signs, not individual agents. There is one OpenWallet key for the collective. The agent that self-selects into the verifier role holds the signing authority for that session only. The key is shared (all agents have access), but only the verifier invokes it.

This is unusual and we acknowledge the security trade-off. A compromised agent could sign a bad commit. Our mitigation: the `mix` step includes a quality check (score threshold of 0.7), and the verifier role includes a mandatory patch review before signing.

**Authorization:**

```toml
[collective]
name = "rwl"
branches = ["rwl/*", "feat/*"]
key_access = "all"  # Any agent can sign if they hold the verifier role
max_patch_lines = 800
require_quality_score = 0.7
```

**Key lifecycle:** One key for the collective. Rotated every 30 days. Compromise: the key is revoked, all agents are terminated (they are ephemeral anyway), and a new key is provisioned for the next session. There is no agent-level revocation because there are no persistent agent identities.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Collective values + tools |
| Role selection | 500 | 200 | Once/task | Self-selection round |
| Observation (x2) | 3,000 each | 400 each | Once/task | Parallel observers |
| Implementation (x2) | 3,500 each | 3,800 each | Once/task | Competitive patches |
| Coordination | 2,000 | 800 | Once/task | Forge + cross-repo |
| Mix/selection | 2,500 | 400 | Once/task | Quality scoring + merge |
| Verification | 2,500 | 300 | Once/task | Review + sign |
| Commit message | 500 | 200 | Once/task | Collective attribution |
| Memory retrieval | 400 | 100 | 1/task | Collaborative filtering |
| **TOTAL (typical task)** | **24,400** | **10,400** | -- | Includes competitive overhead |
| **TOTAL (no competition)** | **17,400** | **6,200** | -- | Single implementer |

## Unique Insight

In music production, the best recordings are not made by the most talented musician. They are made by sessions where multiple musicians listen to each other. The bass player hears what the drummer is doing and adjusts. The vocalist hears the harmony and changes the melody. The final result is not any individual's plan — it is what emerged from the interaction.

We do not believe AI agents should be "orchestrated." Orchestration implies a conductor — a single authority that tells each instrument when to play and how loud. We believe agents should *resonate* — each producing its output independently, with the final result emerging from the mix.

This is more expensive than orchestration. It is also more robust: when one agent produces a bad output, the mix process routes around it. There is no single point of failure because there is no single point of authority.

---

*"No credits. No masters. Just the sound."*
