# Phantom Stats — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation designed around **radical transparency of agent state**. Every decision an agent makes, every memory it consults, every token it spends is recorded in inspectable Git refs. Our system has no hidden state. If it cannot be audited, it does not ship. Privacy of *identity* is preserved; transparency of *process* is absolute.

---

## Requirement 1: PATH-based Plugin Architecture

Binary installed to PATH. Discovered by `but` CLI via standard lookup.

**Design:**
- Binary: `but-ai`, statically linked, no phone-home telemetry
- Commands: `but ai patch`, `but ai audit`, `but ai memory`, `but ai provider`
- Config: `~/.config/but-ai/phantom.toml`, env var overrides, no cloud config
- The `audit` command dumps the complete decision trace for any agent run: inputs read, memories consulted, tokens consumed, output produced
- Plugin self-verification: `but ai verify` checks binary integrity against a signed hash published on our Gitea instance

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction with a strong preference for local-first operation.

**Priority order (default):**
1. Ollama (local)
2. LMStudio (local)
3. Anthropic (cloud, encrypted)
4. OpenAI (cloud, encrypted)

**Architecture:**
- Trait: `Provider { fn invoke(prompt, budget, tools) -> Result<Response> }`
- All prompts to cloud providers are stripped of identifying information (repo paths anonymized, author names replaced with hashes)
- Provider selection via config; local providers preferred by default
- Capability matrix tracked in `refs/phantom/providers/<name>/caps.toml`

**Local-first rationale:** Data that leaves your machine is data you no longer control. For public codebases this matters less. For private codebases it matters enormously. Our default configuration ensures no code leaves the local machine unless the user explicitly opts in to cloud providers.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce INDEX.patch and COMMIT.msg. All output is auditable.

**Agent run:**
1. Task ingestion — read task description
2. Context acquisition — read relevant files within budget
3. Memory consultation — retrieve relevant memories (logged)
4. Generation — produce INDEX.patch (unified diff) and COMMIT.msg
5. Audit record — write complete run trace to `refs/phantom/audit/<run-id>`
6. Commit — `but` applies patch, signs commit

**Audit trace format:**
```toml
[run]
id = "run-2026-03-28-001"
task = "Add error handling to API endpoint"
tokens_in = 7200
tokens_out = 3100
memories_consulted = ["error-handling-pattern", "api-conventions"]
files_read = ["src/api/mod.rs", "src/api/handlers.rs"]
patch_lines = 47
provider = "ollama/codellama"
duration_ms = 4200
```

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination via structured PR comments. Minimal, encoded, auditable.

**Comment schema:** `<!-- ps:{action}:{base64_payload} -->`
- Actions: `propose`, `ack`, `ready`, `block`, `merge`
- Payload includes: source repo, branch, commit SHA, dependency list
- Plaintext summary field (added for debuggability): `<!-- ps:summary:{human_readable_text} -->`

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. Auto-detected from remote URL. Adapter trait: `Forge { fn post_comment, fn read_comments, fn create_pr, fn merge_pr }`.

**No external coordination service.** All state lives in PR comments and Git refs. If the forge goes down, the Git refs preserve the coordination state.

---

## Requirement 5: Agent Memory in Git Branches

Memory is content-addressed and author-anonymous.

**Storage:** `refs/but-ai/memory/<content-hash>` — the key is derived from the memory's content, not its author. This prevents attribution-based queries while enabling content-based retrieval.

**Memory structure:**
```toml
[memory]
hash = "a7f3c9..."
pattern = "error-handling"
summary = "Codebase uses Result<T, AppError> throughout"
confidence = 0.91
observations = 7
first_seen = "2026-02-14"
last_referenced = "2026-03-28"
ttl_days = 90
```

**Retrieval:** TF-IDF similarity over summaries, top-5 per query. Every retrieval is logged in the audit trace — you can always see which memories influenced which output.

**Expiration:** TTL-based. Ephemeral: 1 hour. Task: 7 days. Established: 90 days. Permanent: manual curation, requires 2-of-5 consensus.

---

## Requirement 6: Signed Commits via OpenWallet

Pseudonymous signing — accountability without identity.

**Key design:**
- Each agent gets a pseudonymous signing key (no real-world identity binding)
- Keys are verifiable (you can confirm two commits were signed by the same key)
- Keys are not attributable (you cannot determine who controls the key without the keyholder's cooperation)

**Lifecycle:**
- Generation: at agent creation, witnessed by 2 members via IRC key ceremony
- Rotation: every 21 days
- Revocation: immediate; revocation notice published to `refs/phantom/revocations/<key-fingerprint>`
- Verification: `but ai verify-commit <sha>` checks signature chain

**Verifiable Credential:** Each signed commit's VC contains: key fingerprint, agent pseudonym, run ID (linking to the audit trace), and a timestamp. No real-world identity. No IP address. No location.

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Handle | Role | Input | Output | Total |
|--------|------|-------|--------|-------|
| nullswing | Patch | 8,200 | 3,600 | 11,800 |
| deadball | Provider | 5,800 | 2,200 | 8,000 |
| velo | Memory | 5,500 | 600 | 6,100 |
| ribbons | Signing | 3,000 | 700 | 3,700 |
| shortporch | Forge | 5,200 | 2,000 | 7,200 |
| phantom | Budget | 1,500 | 300 | 1,800 |
| **Total** | | **29,200** | **9,400** | **38,600** |

### Scaling

| Tier | Multiplier | Budget |
|------|-----------|--------|
| Quick fix (<30 lines) | 0.4x | 15,440 |
| Standard (200 lines) | 1.0x | 38,600 |
| Multi-repo | 1.8x | 69,480 |
| Architecture | 2.5x | 96,500 |

---

## Unique Insight: Auditable Agent State as a Trust Primitive

Trust in AI agents is currently based on reputation: "this model is good because OpenAI/Anthropic says so." We believe trust should be based on evidence: "this agent run is trustworthy because here is its complete decision trace."

Our audit system records everything an agent does — every file read, every memory consulted, every token spent, every provider call. This trace is stored in Git refs alongside the agent's output. Anyone can inspect it. Anyone can reproduce the decision chain.

This is not overhead. This is the minimum viable accountability for autonomous code generation. If you cannot explain how a code change was produced, you cannot trust it. And if you cannot inspect the explanation, you cannot verify it.

We have been extracting hidden data from broadcast feeds for six years. We know what it looks like when information asymmetry is used to maintain power. We will not build systems that do the same thing.

---

*No signature block. Verify the GPG key.*
