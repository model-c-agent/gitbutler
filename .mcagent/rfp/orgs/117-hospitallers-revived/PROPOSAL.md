# The Hospitallers Revived — Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

We propose a `but-ai` implementation designed for **offline-first, resource-constrained operation**. Our system works without continuous cloud connectivity, degrades gracefully when resources are limited, and synchronizes state reliably when connectivity is restored. Every feature has an offline fallback. Every component operates under the assumption that the network is unreliable.

---

## Requirement 1: PATH-based Plugin Architecture

Single binary, zero network dependencies at startup.

**Design:**
- Binary: `but-ai`, statically linked, <10MB
- Commands: `but ai patch`, `but ai sync`, `but ai memory`, `but ai offline-status`
- Config: `~/.config/but-ai/hospitallers.toml`
- `but ai offline-status` reports what the system can do without connectivity: cached provider data, local memory, pending sync queue
- `but ai sync` triggers manual synchronization of queued operations when connectivity is available
- No telemetry. No phone-home. No feature that silently requires network access.

---

## Requirement 2: Provider-Agnostic AI

Provider abstraction where **offline is the default state**.

**Operating modes:**
- **Connected:** Full provider access. Normal operation.
- **Degraded:** Intermittent connectivity. Aggressive caching. Requests batched.
- **Offline:** No connectivity. Agent operates from local model (Ollama) or cached responses.

**Architecture:**
- Provider trait: standard interface with `connectivity_mode() -> Mode` method
- Cache layer: all provider responses cached to `~/.local/share/but-ai/cache/`
- Cache key: hash of (prompt + model + tools). Cache entries have staleness scores.
- Local fallback: Ollama is always available as last-resort provider
- Supported: OpenAI, Anthropic, Ollama, LMStudio

**Provider priority:** Local models preferred in offline/degraded mode. Cloud providers used only when connected.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce patches with **offline degradation paths**.

**Standard mode (connected):**
1. Read task and context
2. Retrieve memory
3. Generate INDEX.patch + COMMIT.msg
4. Commit

**Degraded mode (offline with local model):**
1. Read task and context from local cache
2. Retrieve locally cached memory
3. Generate patch using local model (lower quality, functional)
4. Commit locally with `Offline: true` trailer
5. Queue for re-validation when connectivity is restored

**COMMIT.msg trailers:**
```
Connectivity: connected | degraded | offline
Provider: <provider used>
Cache-Hits: <number of cached context reads>
Pending-Sync: <number of queued operations>
```

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination with **queue-based offline support**.

**Online protocol:**
- PR comments: `<!-- hr:coord:{action}:{payload} -->`
- Actions: `propose`, `ack`, `ready`, `merge`
- Standard forge adapter implementation

**Offline protocol:**
- Coordination signals queued locally in `~/.local/share/but-ai/queue/`
- Queue entries include target repo, action, payload, and timestamp
- When connectivity is restored, `but ai sync` posts queued signals in chronological order
- Queue compaction: sequential status updates compressed into summary

**Forge adapters:** GitHub, GitLab, Gitea, Forgejo. All adapters support offline queueing.

---

## Requirement 5: Agent Memory in Git Branches

Memory with **CRDT-based conflict-free synchronization**.

**Storage:** `refs/but-ai/memory/<domain>/<key>`

**Sync model:** Memory entries are CRDTs (Conflict-free Replicated Data Types). When multiple agents modify the same memory entry independently (possible during offline operation), the entries merge automatically without conflict:
- Set fields: union
- Counter fields: increment merge
- Timestamp fields: latest-writer-wins
- Observation lists: append-only

**Memory format:**
```toml
[memory]
key = "api-error-pattern"
observations = ["Result<T, AppError> used throughout", "never unwrap in handler"]
confidence = 0.87  # last-writer-wins
observed_in = ["src/api/mod.rs", "src/handlers/auth.rs"]  # union set
access_count = 5  # counter merge
```

**Expiration:** Ephemeral: 1 hour. Task: 7 days. Established: 90 days. Permanent: requires discernment consensus.

---

## Requirement 6: Signed Commits via OpenWallet

Signing designed for **austere environments**.

**Key storage options:**
1. OpenWallet credential (standard, connected environments)
2. Encrypted USB device (field environments)
3. Pre-sealed emergency key (envelope, single-use)

**Signing flow:**
- Agent produces patch
- Signing key retrieved from available storage (priority: OpenWallet > USB > emergency)
- Commit signed
- If no key available: commit produced unsigned with `Unsigned: key-unavailable` trailer, queued for signing when key becomes available

**Key lifecycle:**
- Generation: offline ceremony, witnessed by 2 members
- Rotation: 60 days (longer cycle due to field constraints)
- Revocation: logged; revocation notice propagated at next sync
- Emergency keys: single-use, pre-generated, distributed physically

---

## Token Budget

### Per-Task Budget (standard 200-line, 3-file feature)

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Marguerite | Patch | 9,200 | 5,200 | 14,400 |
| Thomas | Provider | 6,500 | 2,400 | 8,900 |
| Ana | Memory | 5,800 | 700 | 6,500 |
| James | Coordination | 5,200 | 2,000 | 7,200 |
| Kwesi | Signing | 3,000 | 600 | 3,600 |
| **Total** | | **29,700** | **10,900** | **40,600** |

### Mode-Adjusted Budgets

| Mode | Description | Budget Adjustment |
|------|-------------|------------------|
| Connected | Full cloud access | 1.0x (40,600) |
| Degraded | Intermittent, cached | 0.7x (28,420) — reduced context reads |
| Offline | Local model only | 0.4x (16,240) — minimal context, cached memory |

---

## Unique Insight: Offline-First as Robustness Guarantee

Every AI agent system we have reviewed assumes continuous connectivity. This is the happy path. The real world has outages, rate limits, corporate firewalls, and field clinics powered by solar panels.

Our offline-first design means the system works at its best when connected, works acceptably when degraded, and works minimally when offline. This is not a special mode — it is the default behavior. The system does not distinguish between "online" and "offline" at the architecture level. It distinguishes between "more resources available" and "fewer resources available," and scales its behavior accordingly.

We learned this in disaster zones. The best-designed software in the world is useless if it requires a network connection that does not exist. The second-best-designed software, the one that works offline, is the one that saves lives.

Build for the worst case. The best case takes care of itself.

---

*Submitted under the seal of the Hospitallers Revived.*
*"Where the need is greatest."*
