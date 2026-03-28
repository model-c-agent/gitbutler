# DeepRoute Syndicate — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate. Statically linked. No dynamic dependencies. The binary should be portable — copy it to a machine, put it on PATH, and it works. No installers, no setup wizards, no configuration files required for basic operation.

CLI subcommands: `agent exec` (run task), `agent audit` (verify a commit's provenance chain), `memory`, `mcp`. The `audit` subcommand is unique to our proposal — it takes a commit hash and traces the full chain of custody: which agent produced it, what specification it was built against, what data informed it, and who signed it.

MCP mode: `ServerHandler` with all ten `WorkspaceToolset` tools plus `AgentExec`, `ProvenanceAudit`, and `MemoryQuery`.

**WASI:** We treat WASI as a sandboxed execution environment. Under WASI, the agent operates in "airgapped" mode: no network access, no forge coordination, no external provider calls. Only local operations (memory query, workspace state, patch generation against local context) work. If a local LLM provider (Ollama/LMStudio) is accessible via WASI HTTP, the agent can still generate patches.

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` with one principle: always prefer local providers. The syndicate does not trust cloud providers with operational data. Our default provider order: Ollama (local) → LMStudio (local) → Anthropic → OpenAI. Users can override this, but our default reflects our values.

For new providers: a trait `ProviderShim` with `send_prompt`, `send_tool_call`, and `receive_stream` methods. Each shim is a separate Rust file in `src/providers/`. Adding a provider means implementing three methods and adding a match arm. No runtime plugin loading — we do not load code we have not audited.

**Provider verification:** Before using any provider, the agent runs a canary test: a trivial prompt that verifies the provider returns coherent output. This catches misconfigured endpoints, expired API keys, and providers that are nominally alive but returning garbage. The canary costs 50 input + 20 output tokens. Cheap insurance.

## 3. The But Agent (RFP 3.3)

The agent runs a compartmentalized pipeline:

1. **Controller** decomposes task, produces specification
2. **Collector** gathers workspace state and memory, produces dossier
3. **Analyst** receives spec + dossier, produces `INDEX.patch` + `COMMIT.msg`
4. **Auditor** verifies patch against spec, traces provenance, signs commit

Each stage is isolated. The analyst cannot call forge APIs. The collector cannot produce patches. This prevents a compromised or hallucinating agent from taking actions outside its scope.

**Branch naming:** `dr/<cell>/<task-hash>`. "Cell" is the agent's compartment. Example: `dr/anls/a1b2c3d4`.

**Budget enforcement:** Each compartment has a hard budget ceiling. Controller: 15%. Collector: 25%. Analyst: 45%. Auditor: 15%. No agent can borrow from another's allocation. If the analyst runs out of budget, it produces a partial patch — it does not ask the auditor to donate tokens.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait ForgeChannel {
    fn open(&self, repo: &str, pr_spec: PrSpec) -> Result<ChannelId>;
    fn send(&self, ch: ChannelId, msg: SignedMessage) -> Result<()>;
    fn recv(&self, ch: ChannelId) -> Result<Vec<SignedMessage>>;
}
```

Three methods. All messages are signed. Unsigned messages are discarded without logging (to avoid leaking information about what was rejected).

**PR comment schema:** Compact, encrypted-header format:

```json
{"v":1,"from":"dr/anls","sig":"<base64>","payload":{"type":"status","state":"done","lines":98,"budget_pct":67}}
```

No human-readable wrapper. Our agents talk to machines. If a human wants to read agent messages, they can use `but ai audit <pr-id>` to decode and display them.

**Cross-repo coordination:** The controller maintains a "link chart" — a JSON manifest in the memory branch listing all cross-repo references and their trust levels. Trust levels: `verified` (we've confirmed the referenced PR exists and is valid), `claimed` (another agent says it exists; we haven't verified), `stale` (last verified more than 24 hours ago).

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** `refs/dr/vault/` namespace. Memory entries are encrypted at rest using a symmetric key derived from the repository's OpenWallet credentials. An attacker who gains read access to the Git refs sees encrypted blobs, not plaintext memory.

Structure:
- `refs/dr/vault/active/` — Current memory entries
- `refs/dr/vault/cold/` — Expired entries moved to cold storage (not deleted — the syndicate keeps receipts)
- `refs/dr/vault/identity/` — Agent identity records

**Relevance scoring:** Each memory entry has a "heat" score: `heat = access_count * recency_factor`. Recency factor decays as `1 / (1 + days_since_last_access)`. Hot entries (heat > threshold) are retrieved. Cold entries are archived. The threshold adapts: if retrieval returns too many entries, threshold increases. If too few, it decreases.

**TTL:** Active entries: 14 days. Cold storage: indefinite (but not retrieved unless explicitly requested). The syndicate never deletes data — it just makes it harder to access.

**Compaction survival:** Each agent has an "operational brief" — a compressed summary of its role, current task context, and critical knowledge. Briefs are regenerated at each compaction event. The brief is the first thing injected into fresh context.

**Identity:** Agent identity includes: handle (pseudonym), role, compartment restrictions, authorized branches, public key, and a "chain of trust" field listing which other agents vouched for this agent at creation. Identity records are stored encrypted.

## 6. Signed Commits via OpenWallet (RFP 3.6)

The auditor is the sole signer. Signing requires successful provenance verification — the auditor traces the patch back through the dossier to the original task specification before signing. A commit that cannot be traced is not signed.

**Authorization:**

```toml
[agents.ctrl]
branches = []
can_sign = false

[agents.coll]
branches = []
can_sign = false

[agents.anls]
branches = ["dr/anls/*"]
can_produce_patch = true
can_sign = false

[agents.audt]
branches = ["dr/*", "feat/*"]
can_sign = true
require_provenance = true
```

**Key lifecycle:** Keys are provisioned in a multi-agent ceremony: the controller generates the key pair, the auditor verifies the generation, and all four agents sign the identity record. Rotation every 21 days. Compromise response: immediate revocation, all commits since last known-good signed state are quarantined, and a full provenance audit is triggered for every quarantined commit.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,500 | 0 | Once/session | Compartment roles + tools |
| Task decomposition (ctrl) | 2,000 | 800 | Once/task | Specification |
| Data collection (coll) | 3,500 | 400 | Once/task | Dossier assembly |
| Patch generation (anls) | 3,000 | 4,500 | Once/task | Core implementation |
| Provenance audit (audt) | 3,000 | 400 | Once/task | Trace + verify |
| Commit message | 500 | 200 | Once/task | Terse, factual |
| Memory retrieval | 500 | 100 | 1/task | Encrypted lookup |
| Coordination event | 800 | 300 | 1/task | Signed message exchange |
| Canary test | 50 | 20 | Once/session | Provider verification |
| **TOTAL (typical task)** | **15,850** | **6,720** | -- | Compartmented execution |

## Unique Insight

From years of tracking vessels that turn off their transponders to hide their routes, we learned the most important lesson in data security: the absence of evidence is itself evidence. A vessel that goes dark for 6 hours in a specific sea area is telling you something by not telling you anything.

The same applies to agent commits. An agent that skips memory retrieval, that produces a patch without reading the specification, that signs a commit without provenance verification — the absence of these steps is a signal. Our system does not just check for the presence of correct behavior. It checks for the absence of expected behavior. A missing step is flagged as suspiciously as an incorrect step.

---

*"Trust no feed. Verify every position."*
