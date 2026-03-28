# grep_the_stacks -- technical proposal

**rfp:** gitbutler `but-ai` plugin
**date:** 2026-03-28
**from:** the collective

---

## summary

we already run AI agents that produce INDEX.patch + COMMIT.msg for an 11-million-entry catalog distributed across 14 mirrors. our system works but it's held together with shell scripts and hope. this proposal describes rebuilding our existing pipeline on top of `but-ai`, replacing duct tape with a formal plugin architecture.

we are not theorizing. we are describing what we already do, expressed in a new framework.

---

## 1. PATH-based plugin architecture

`but-ai` on PATH. we already use PATH-based tools for our pipeline (`foil`, `grep`, `libcat-index`). adding `but-ai` is consistent with our existing workflow.

- binary: Rust, statically linked, reproducible build
- protocol: JSON lines over stdin/stdout
- config: `~/.config/but/ai.toml` with mirror-specific overrides
- lifecycle: spawn per-task, exit on completion
- reproducibility: binary is built from a pinned Cargo.lock with hash verification

we care about reproducible builds because our mirrors need to trust the binary they're running. every mirror operator builds from source and verifies the hash. no prebuilt binaries.

---

## 2. provider-agnostic AI

minimal `Provider` trait. prompt in, completion out.

| provider | usage | notes |
|----------|-------|-------|
| Anthropic | primary mirrors | best metadata extraction |
| OpenAI | secondary mirrors | backup provider |
| Ollama | development | local testing, no API cost |
| LMStudio | development | local, OpenAI-compatible |

we don't use streaming for catalog work. entries are short. we send a prompt containing the paper's abstract and bibliographic data, and receive a structured catalog entry. one round trip. no tool calling for the common case -- tool calling is reserved for complex entries that need CrossRef verification.

---

## 3. but agent (INDEX.patch + COMMIT.msg)

this is our existing workflow, formalized.

### current pipeline

1. new paper arrives (uploaded by a contributor or scraped)
2. agent extracts metadata: title, authors, DOI, abstract, subject terms
3. agent generates catalog entry as unified diff against current catalog
4. agent writes COMMIT.msg with metadata summary
5. `null_ptr` reviews entries below 95% confidence
6. entry is committed

### what changes with but-ai

the agent stops writing directly to the catalog branch and instead produces INDEX.patch + COMMIT.msg as artifacts that `but` validates and applies. this eliminates the merge conflicts that currently plague our multi-agent pipeline.

### commit format

```
catalog: add 10.1234/example.2026.001

Title: Example Paper on Example Topics
Authors: 3
Subjects: computer-science, information-retrieval
Confidence: 0.94
Source: crossref-verified
```

no conventional commits prefix beyond `catalog:`. we have one type of change. we don't need a taxonomy.

---

## 4. polyrepo PR coordination

14 mirrors. each runs its own Forgejo instance. catalog updates propagate from primary to secondaries.

### propagation protocol

```json
{
  "from": "primary",
  "to": ["mirror-02", "mirror-03", "..."],
  "patch_hash": "sha256:abc123...",
  "entries": 47,
  "action": "catalog_sync"
}
```

`chmod` posts the coordination message to each mirror's repo as a PR comment. the mirror's local agent verifies the patch hash, applies it, and responds with `ack` or `nack`. no centralized orchestration. each mirror decides independently whether to accept.

### forge support

Forgejo (primary -- 12 mirrors), Gitea (1 mirror), GitHub (1 mirror for public-facing metadata). the adapter layer handles all four. we contributed the Forgejo adapter.

---

## 5. agent memory in Git branches

### content-addressed memory

memory stored in `refs/gts/memory/<hash>` where the hash is derived from the memory content. this makes deduplication automatic -- identical memories at different mirrors resolve to the same ref.

| field | description |
|-------|-------------|
| `hash` | content-derived identifier |
| `value` | memory content |
| `scope` | `local` (one mirror) or `global` (all mirrors) |
| `confidence` | 0.0-1.0, same scale as catalog entries |
| `ttl` | hours (default: 72 for local, 168 for global) |

### retrieval

top-3 by relevance. no injection of entries below 0.7 confidence. global memories are preferred over local when both match.

### mirror sync

global memory entries propagate with catalog syncs. local memories stay on their mirror. this prevents one mirror's edge cases from polluting another's context.

---

## 6. signed commits via OpenWallet

### pseudonymous verification

the challenge: verify commits without identifying committers.

our approach: each agent holds a Verifiable Credential containing:
- a public key
- a role (`metadata`, `infrastructure`, `forge`, `identity`)
- a collective membership proof (signed by the collective's group key)

the credential does NOT contain a name, email, or any correlatable identifier. verification confirms: this commit was signed by a member of grep_the_stacks with the `metadata` role. it does not confirm which member.

- key rotation: every 48 hours
- revocation: immediate, stored in `refs/gts/revoked`
- group key: rotated monthly by `sudo`

---

## 7. token budget

| handle | role | input | output | total |
|--------|------|-------|--------|-------|
| null_ptr | catalog patches | 8,000 | 4,500 | 12,500 |
| rm_rf | infrastructure | 3,200 | 800 | 4,000 |
| chmod | forge/mirrors | 4,800 | 2,000 | 6,800 |
| sudo | signing | 3,000 | 700 | 3,700 |
| **total** | | **19,000** | **8,000** | **27,000** |

we run on donated API credits. the budget is tight because it has to be. every token is accountable.

---

## 8. unique insight: content-addressed memory deduplication

most agent memory systems use arbitrary keys. we use content-derived hashes. this means:

- identical memories are automatically deduplicated across mirrors
- memory sync reduces to a set difference operation (which hashes do I have that you don't?)
- memory integrity is verifiable -- if the content changes, the hash changes, and the entry is flagged as tampered

this is not novel computer science. it is how Git objects work. it is how IPFS works. it is how our paper repository works. we are applying the same principle to agent memory because it solves the distributed consistency problem that plagues multi-node agent deployments.

when 14 mirrors independently discover the same pattern, they should store it once, not fourteen times. content addressing makes this automatic.

---

```
// we didn't invent this. we just applied it where nobody else thought to look.
```
