# /dev/drape -- technical proposal

**rfp response: `but-ai` plugin for gitbutler**

---

## summary

open source everything. our plugin is AGPL. our patterns are AGPL. our agent memory is content-addressed and auditable. we don't trust black boxes. we build glass boxes.

---

## requirement 1: PATH-based plugin architecture

`but-ai` binary installed via `cargo binstall` or built from source (`cargo build --release`). the binary is a single static executable. no dynamically linked providers, no plugin-within-a-plugin architecture.

**our addition:** the binary embeds its own git hash in the `--version` output. you can always trace a `but-ai` binary back to the exact commit that built it. reproducible builds via nix flake.

---

## requirement 2: provider-agnostic AI

trait-based provider abstraction. we implement `Completer` for openai, anthropic, ollama, lmstudio. our default is ollama because we don't trust cloud providers with our users' code.

**provider selection:** explicit, not automatic. the user chooses their provider in `but-ai.toml`. we don't route traffic to providers the user didn't configure. if no provider is configured, the plugin runs in offline mode: no AI features, but all non-AI commands still work.

**token counting:** provider-native tokenizers. we vendor tiktoken for openai and use anthropic's published token counting API. for local models, we approximate using the model's declared context window and a conservative tokens-per-character ratio.

---

## requirement 3: but agent (INDEX.patch + COMMIT.msg)

agents produce patches. patches are diffs. diffs are auditable. this is the only acceptable workflow.

**patch requirements:**
- unified diff format
- applies cleanly to HEAD via `git apply --check`
- includes only files within the declared task scope
- COMMIT.msg follows conventional commits
- COMMIT.msg includes `Scanned-by:` trailer with the content hash of the context used to generate the patch (so you can verify what the agent "saw")

---

## requirement 4: polyrepo PR coordination (forge-agnostic)

`ForgeAdapter` trait. github and gitlab implementations shipped; bitbucket community-contributed.

**coordination:** PR comments with machine-readable headers (`<!-- but-ai:coord:v1 -->`). human-readable section below. cross-repo links are explicit: each coordination comment names the remote repo, branch, and PR number.

**our constraint:** no webhooks. webhooks require server infrastructure we don't have. all coordination is poll-based. agents check for updates on a configurable interval (default: 60s).

---

## requirement 5: agent memory in git branches

content-addressed memory. every entry is keyed by the SHA-256 of its value. same content = same key = no duplicates.

**storage:** `refs/drape/mem/<handle>/` namespace. entries are gzipped JSON blobs.

**schema:**
```json
{
  "key": "<sha256 of value>",
  "value": "...",
  "tags": ["pattern-grading", "v3"],
  "scan_hash": "<hash of input context>",
  "created": "2026-03-28T23:00:00Z",
  "ttl": 2592000
}
```

**retrieval:** embedding-based similarity search over memory values. top 5 returned. content-addressing means the same memory is never retrieved twice under different keys.

**gc:** `but-ai gc` prunes expired entries. runs automatically before each task. aggressive by default because disk is cheap but context windows are not.

---

## requirement 6: signed commits via OpenWallet

every commit signed. zh0st manages the trust root. each contributor's agent gets a key via OpenWallet issuance.

**our policy:**
- keys rotate every 60 days (shorter than the 90-day default because we're paranoid)
- revocation is immediate and public (published to the `#drape-security` channel)
- verification is offline-capable: the trust registry is mirrored locally
- unsigned commits are rejected at merge time via a pre-merge hook

---

## token budget

| handle | input | output | total |
|--------|-------|--------|-------|
| zh0st | 3,800 | 1,000 | 4,800 |
| patchwerk | 7,800 | 4,200 | 12,000 |
| seam_ripper | 4,200 | 1,400 | 5,600 |
| nullstitch | 5,200 | 700 | 5,900 |
| bobbin | 5,800 | 2,200 | 8,000 |
| selvage | 3,000 | 600 | 3,600 |
| grainline | 2,800 | 500 | 3,300 |
| **total** | **32,600** | **10,600** | **43,200** |

---

## unique insight: content-addressed memory prevents hallucination drift

most agent memory systems use opaque keys (UUIDs or human-assigned names). this means two different memory entries can describe the same thing with different wording, and the system doesn't know they're duplicates. over time, the agent accumulates contradictory memories about the same concept, leading to inconsistent behavior.

content-addressing solves this. if two agents learn the same thing, the SHA-256 of the value is identical, so the entries merge automatically. if they learn slightly different things about the same concept, the hashes differ, and both entries coexist — but the system can detect near-duplicates via embedding similarity and flag them for human review.

in our testing with `scan2pattern`, content-addressed memory reduced contradictory patch outputs by 24% compared to UUID-keyed memory. the system stays consistent because it cannot accumulate invisible duplicates.

---

*`git push --force-with-lease origin freedom`*
