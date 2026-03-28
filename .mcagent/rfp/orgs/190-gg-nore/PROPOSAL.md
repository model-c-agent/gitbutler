# /gg/noRe — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Evidence-Chain Agent System (Local-Only)

---

## Executive Summary

/gg/noRe proposes a fully local, zero-cloud agent system designed for legally sensitive reverse engineering analysis. All inference runs on local hardware. All commits are pseudonymously signed. The evidence chain — from binary offset to published finding — is cryptographically verifiable without revealing analyst identity.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is compiled from Rust and installed to `~/.local/bin/` (we do not use `~/.gitbutler/bin/` because our systems already have custom PATH configurations and we prefer XDG-adjacent conventions). The binary is statically linked with no network capability at the binary level — network calls are stripped at compile time via feature flags. This is not security theater. Our analysis artifacts include decompiled proprietary code. The binary physically cannot exfiltrate data.

Subcommands: `but ai annotate` (generate annotations for decompiled source), `but ai link` (establish evidence chains between annotations and binary offsets), `but ai sign` (pseudonymous commit signing), `but ai evidence` (verify evidence chain integrity).

The `evidence` subcommand is our core differentiator. It traverses the commit history and verifies that every annotation cites a valid binary offset and that the cited code has not changed since the annotation was written. If the game client is updated and the binary changes, annotations against the old binary are flagged as `STALE`.

## Requirement 2: Provider-Agnostic AI

Local only. No cloud. No exceptions.

The provider layer supports: Ollama (primary), llama.cpp (fallback), and a custom GGUF loader for models that neither Ollama nor llama.cpp handle cleanly. The provider interface: `analyze(decompiled_context) -> Annotations`. Single method. We do not need `embed` or `health_check`. We need analysis.

Model selection: Codestral for code annotation (best decompiled-code comprehension in our benchmarks), Mistral for natural language summary. Both run on consumer GPUs. Our minimum hardware spec is an RTX 3090 or equivalent. Members who cannot afford a 3090 use race_condition's remote analysis server (self-hosted, VPN-only, no internet routing).

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The annotation pipeline:

1. heapspray's triage agent scans decompiled output and identifies matchmaking-relevant code paths
2. The agent generates INDEX.patch adding structured annotations:

```diff
+ // [gg:annotate] offset=0x4A2F3C build=v3.2.1-4847
+ // Matchmaking weight calculation. Player `frustration_score`
+ // (float, range 0.0-1.0) is factored into opponent selection.
+ // Higher frustration correlates with easier opponent assignment.
+ // Evidence: cross-reference with network capture showing
+ // `mm_weight` field in matchmaking request packet.
```

3. COMMIT.msg includes full evidence citation:

```
Annotate: frustration-weighted matchmaking in opponent selection

Binary: game-client-v3.2.1-4847
Offset: 0x4A2F3C-0x4A3012
Function: mm_calculate_opponent_weight
Evidence-Type: static-analysis + network-correlation
Network-Capture: captures/2026-03-15/match-request.pcap
Confidence: high (two independent evidence sources)
```

4. nullref reviews. If approved, sigreturn signs.

## Requirement 4: Polyrepo PR Coordination

Multi-game analyses span separate repos (one per game title). Cross-repo coordination tracks evidence dependencies:

```
[gg:evidence-link] game-a-analysis#12 → game-b-analysis#7
Both titles use identical matchmaking weight function.
Binary offset differs but decompiled logic is structurally
identical (see diff in game-a-analysis#12 comment 3).
Shared EOMM implementation confirmed.
```

The forge adapter targets Forgejo (our self-hosted instance). GitHub adapter exists for publishing redacted summaries to our public repository. We never push unredacted analysis to GitHub.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/gg/memory/evidence`. Memory types:

- **`offset-map`**: Maps binary offsets to function names and annotations. TTL: until the binary is superseded by a new build. When a new build is detected, all offset-map entries are flagged `STALE` and must be re-verified.
- **`protocol-signature`**: Network protocol structures identified by packetsniff. TTL: until protocol version changes.
- **`cross-reference`**: Links between annotations in different repos/games. TTL: 1 year. Cross-references are the most valuable memory type — they connect findings across titles and reveal shared infrastructure.

Memory retrieval is key-based: `<game-title>:<build-hash>:<offset>`. Deterministic. No embedding search — we need exact references, not approximate matches. When you cite evidence in a legal context, "approximately this function" is not sufficient.

## Requirement 6: Signed Commits via OpenWallet

Pseudonymous signing. Each member has an OpenWallet DID that is not linked to any real-world identity. The DID proves that the same analyst produced multiple annotations across time, which establishes credibility without revealing identity.

sigreturn's key management: keys are generated on air-gapped hardware, stored on hardware tokens, and never touch a networked machine. Key rotation every 60 days. Revocation requires two members to co-sign the revocation notice (threshold signature, preventing a single compromised member from revoking others' keys).

The signing chain includes a `Pseudonym-Attestation` header: a statement that the signer is a member of /gg/noRe in good standing, signed by at least two other members' DIDs. This provides collective attestation without individual identification.

**Unique insight:** /gg/noRe's evidence chain system solves a problem that most agent architectures ignore: *citation integrity over time*. When a game client updates, every annotation that references the old binary becomes potentially invalid. The `STALE` detection mechanism — comparing stored binary hashes against current builds and flagging affected annotations — ensures that the analysis repository never contains outdated evidence presented as current. This is critical for legal credibility but equally valuable for any codebase where the underlying source changes frequently. The same mechanism could flag agent-generated patches that reference code that has been modified since the patch was generated.

---

## Token Budget

| Handle | Input | Output | Total |
|--------|-------|--------|-------|
| nullref | 3,000 | 1,500 | 4,500 |
| packetsniff | 2,500 | 1,500 | 4,000 |
| sigreturn | 1,500 | 500 | 2,000 |
| heapspray | 4,000 | 4,000 | 8,000 |
| race_condition | 2,000 | 800 | 2,800 |
| **Task Total** | **13,000** | **8,300** | **21,300** |

All tokens consumed locally. Cloud cost: $0. Hardware cost: amortized across members' existing GPU rigs. Grandmother would approve.

---

*"The binary doesn't lie. The marketing does."*
