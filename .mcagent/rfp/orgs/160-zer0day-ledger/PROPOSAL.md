# zer0day_ledger -- technical proposal

**rfp:** gitbutler `but-ai` plugin
**date:** 2026-03-28
**from:** the collective

---

## summary

we correlate leaked financial data with public filings to expose fraud and sanctions evasion. our `but-ai` proposal is designed for air-gapped, pseudonymous, opsec-first operation. no cloud providers. no identity in commits. no metadata leakage. the plugin must work entirely offline on hardware we control, with signing that verifies membership without revealing identity.

---

## 1. PATH-based plugin architecture

`but-ai` on PATH. per-task invocation. no network access during operation.

- binary: Rust, statically linked, reproducible build, hash-verified
- protocol: JSON lines over stdin/stdout
- config: `~/.config/but/ai.toml`
- network: NONE. the binary must function with no network stack. all providers are local.

the binary is built from source on an air-gapped build machine. the hash is verified before deployment to operational machines. this is non-negotiable.

---

## 2. provider-agnostic AI

local providers only.

| provider | usage | notes |
|----------|-------|-------|
| Ollama | all operations | local inference, no telemetry |
| LMStudio | fallback | local, OpenAI-compatible |
| Anthropic | NOT USED | requires network |
| OpenAI | NOT USED | requires network |

the `Provider` trait must support a `network_required: false` flag. providers that require network access are rejected at configuration time.

---

## 3. but agent (INDEX.patch + COMMIT.msg)

### correlation findings

each patch adds one correlation finding:

```
correlation: entity match across datasets DS-2026-017 and REG-EE-2026

Entity: [REDACTED-HASH-A7F3]
Matched in: leaked transaction log (DS-2026-017, record 4,201)
Matched in: Estonian corporate registry (REG-EE-2026, entity 88,103)
Relationship: director of shell company with no visible economic activity
Confidence: 0.89
Published: NO (pending opsec review)

Case: ZDL-2026-009
Agent: zk_proof
```

entity names are hashed in the commit message. the unhashed names exist only in the encrypted case branch. this prevents casual `git log` inspection from revealing investigation targets.

---

## 4. polyrepo PR coordination

minimal. the collective's repos are on private Gitea instances. coordination with journalists happens via PGP email, not forge comments.

### internal coordination

```json
{
  "collective": "zdl",
  "case": "ZDL-2026-009",
  "action": "finding_ready",
  "confidence": 0.89,
  "opsec_cleared": false,
  "branch": "case/009/correlation-a7f3"
}
```

forge support: Gitea (internal), Forgejo. no GitHub, no GitLab. we do not use hosted forges.

---

## 5. agent memory in Git branches

### compartmentalized memory

memory stored in `refs/zdl/memory/<case>/<compartment>/<key>`.

compartments:
- `source`: references to source datasets (hashed identifiers only)
- `entities`: known entity records (encrypted at rest)
- `methods`: detection methods (shareable across cases)
- `opsec`: opsec decisions and sanitization rules

| field | description |
|-------|-------------|
| `key` | content-derived hash |
| `value` | memory content (encrypted for entity compartment) |
| `compartment` | isolation boundary |
| `confidence` | 0.0-1.0 |
| `ttl` | methods: indefinite. entities: case duration. source: case duration. opsec: indefinite. |

### encryption

entity and source compartments are encrypted at rest using age (https://age-encryption.org). only agents with the case key can read them. methods and opsec compartments are plaintext for cross-case reuse.

---

## 6. signed commits via OpenWallet

### pseudonymous group signatures

each agent signs with an ephemeral key derived from the case key. the corresponding Verifiable Credential contains:
- the public key
- a group membership proof (proves membership in zer0day_ledger without revealing which member)
- a role designation (`analyst`, `opsec`, `pipeline`, `coordination`)

the credential does NOT contain: name, email, timezone, location, or any correlatable identifier.

- key lifecycle: one key per case, derived from case key
- revocation: case key is rotated, invalidating all derived keys
- verification: proves "a member of zdl with the analyst role signed this"

### opsec review

`bit_rot` reviews every commit before it leaves the air-gapped environment. the review checks for:
- timezone information in timestamps (must be UTC)
- locale information in string formatting
- metadata that could correlate to a specific individual
- references to source datasets that could identify the leaker

---

## 7. token budget

| handle | role | input | output | total |
|--------|------|-------|--------|-------|
| zk_proof | correlations | 8,500 | 5,000 | 13,500 |
| bit_rot | opsec/signing | 3,200 | 800 | 4,000 |
| pkt_loss | pipeline | 3,500 | 800 | 4,300 |
| eof | coordination | 4,000 | 1,500 | 5,500 |
| **total** | | **19,200** | **8,100** | **27,300** |

budget measured in local compute cost, not API spend. no cloud.

---

## 8. unique insight: opsec-first agent architecture

most agent systems assume a trusted environment: cloud providers, hosted forges, signed commits that link to a human identity. we assume a hostile environment: adversaries who will examine our commits for metadata, subpoena our forge for IP addresses, and correlate our signing keys to individuals.

our insight: **the agent architecture must be designed for adversarial conditions from the ground up.** this means:
- no cloud providers (all inference is local)
- no hosted forges (all repos are self-hosted on Tor)
- no identity in signing (group signatures prove membership, not identity)
- no unencrypted entity data (case-specific encryption at rest)
- no metadata leakage (opsec review of every outgoing artifact)

this is not paranoia for its own sake. the entities we investigate include sanctioned individuals, organized crime-adjacent shell companies, and politically connected tax evaders. they have lawyers, investigators, and in some cases, state resources. our architecture assumes they will try to identify us. it is designed to make that impossible.

the same architecture generalizes to any agent deployment in a hostile environment: security research, whistleblower protection, journalism in authoritarian regimes. the `but-ai` plugin should support this use case, not as an afterthought but as a first-class configuration.

---

```
// we read the receipts. we always read the receipts.
```
