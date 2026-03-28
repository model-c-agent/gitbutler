# Okafor Library Services -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28
**From:** The Okafor Family (Baltimore, Philadelphia, Newark, Richmond, Durham)

---

## Summary

We run community lending libraries in five cities. Our `but-ai` proposal is built on the same principle as our libraries: serve the patron, not the system. The plugin should be invisible to the developer the way our catalog is invisible to the reader. They ask for a book; they get a book. They ask for a commit; they get a commit.

---

## 1. PATH-Based Plugin Architecture

`but-ai` installed to PATH. `but` discovers it, invokes it per-task, receives results on stdout.

- Binary: Rust, statically linked
- Protocol: JSON lines over stdin/stdout
- Config: `~/.config/but/ai.toml` with per-location overrides in `.but-ai.toml` per repo
- No daemon -- each invocation is stateless

The per-repo config override is our addition. Each of our five locations has different provider preferences (Durham uses local Ollama; Baltimore uses Anthropic). The override file lets each repo specify its context without changing global config.

---

## 2. Provider-Agnostic AI

`Provider` trait with `complete`, `tool_call`, `stream`. Four supported providers.

| Provider | Where We Use It | Notes |
|----------|----------------|-------|
| Anthropic | Baltimore, Philadelphia | Best accuracy for cataloging |
| OpenAI | Newark, Richmond | Cost-effective for high volume |
| Ollama | Durham | Tobias runs local models for testing |
| LMStudio | Development | Local iteration |

Provider selection is per-repo via `.but-ai.toml`. No runtime switching. The user chose; we respect it.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Workflow

1. Adaeze reads task and workspace state
2. Produces INDEX.patch (unified diff, minimal changes)
3. Produces COMMIT.msg with structured trailers

### Commit Format

```
fix: correct ISBN extraction for multi-volume sets

Multi-volume sets were returning the ISBN of volume 1 for all
volumes. Now extracts per-volume ISBNs from the 020 field.

Location: baltimore
Reviewed-by: Adaeze (Collection Manager)
```

The `Location:` trailer indicates which city's context produced the change. This is critical for our multi-location setup -- a fix that works in Baltimore's catalog conventions may not apply in Durham's.

---

## 4. Polyrepo PR Coordination

Fumilayo coordinates across five location repositories plus the shared catalog repo. Six repos total.

### Coordination Message

```json
{
  "family": "okafor",
  "from_location": "baltimore",
  "to_location": "durham",
  "action": "catalog_sync",
  "branch": "feat/isbn-fix",
  "commit": "abc1234",
  "note": "Applies to multi-volume sets in all locations"
}
```

### Forge Support

GitHub (primary), GitLab, Bitbucket, Forgejo. All five location repos are on GitHub. The adapter layer supports others for partner institutions.

### Sync Protocol

When a change in one location's repo affects the shared catalog, Fumilayo opens a PR on the shared repo and comments on each location's repo with the coordination message. Location agents review and confirm applicability to their context.

---

## 5. Agent Memory in Git Branches

### Multi-Location Memory

Memory stored in `refs/okafor/memory/<city>/<namespace>/<key>`. Each city has its own memory namespace. Shared memories live in `refs/okafor/memory/shared/<namespace>/<key>`.

| Field | Description |
|-------|-------------|
| `key` | Entry identifier |
| `value` | Memory content |
| `city` | Location that created it |
| `scope` | `local` (one city) or `shared` (all cities) |
| `ttl` | Hours until expiration (default: 168) |
| `verified_by` | Who confirmed this memory's accuracy |

### Retrieval Rules

- Working on a city branch: retrieve that city's local + shared memories
- Working on the shared catalog: retrieve shared memories only
- Cross-city memories are never auto-injected; they require explicit query

This prevents Baltimore's cataloging conventions from leaking into Durham's branch. Each library serves its own community. The system reflects that.

---

## 6. Signed Commits via OpenWallet

Each agent has a signing key via OpenWallet Verifiable Credential. The credential includes the agent's name, role, and location.

- Rotation: weekly
- Revocation: immediate, stored in `refs/okafor/revoked`
- Signing authority: Adaeze signs catalog changes; Tobias signs infrastructure changes
- Verification: public keys in per-repo Verifiable Credentials

Split signing authority reflects the family's division of responsibility. Catalog quality is Adaeze's domain. Infrastructure is Tobias's. Neither signs the other's work.

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Adaeze | Patches/quality | 7,500 | 3,500 | 11,000 |
| Tobias | Infrastructure | 3,800 | 1,000 | 4,800 |
| Fumilayo | Coordination | 5,200 | 2,400 | 7,600 |
| Emeka | Memory/sync | 5,000 | 1,100 | 6,100 |
| **Family** | | **21,500** | **8,000** | **29,500** |

---

## 8. Unique Insight: Location-Scoped Everything

Most multi-repo agent systems treat all repositories as interchangeable. We do not. Each of our five libraries serves a different community with different needs, different collections, and different conventions. A cataloging decision that is correct in Baltimore may be wrong in Durham.

Our insight: **scope agent state -- memory, configuration, signing authority -- to the repository's context, not to the agent's identity.** The same agent operating in two different repos should behave differently because the repos serve different communities.

This is how librarians actually work. A librarian transferred from a children's branch to an academic branch does not catalog the same way. The context changes the practice. Our system encodes this.

---

*"We are not building software. We are building a library that happens to use software."*
-- Chidinma, at the 2025 Monday meeting
