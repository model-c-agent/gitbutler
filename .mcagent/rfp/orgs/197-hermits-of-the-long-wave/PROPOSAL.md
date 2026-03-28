# The Hermits of the Long Wave — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Bandwidth-Constrained Contemplative Development

---

## Executive Summary

The Hermits propose an agent system designed for extreme bandwidth constraints. All inference is local. All patches are minimal. All memory is compressed. The system synchronizes over shortwave radio at bytes-per-second speeds, which imposes a discipline on agent output that no other constraint can achieve: every byte must justify its existence.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is compiled for the hermitages' hardware (x86_64 and aarch64, depending on the hermitage). Installed to `~/bin/but-ai`. The binary is distributed over shortwave — a new release takes approximately 45 minutes to transmit to all hermitages. Updates are infrequent.

Subcommands: `but ai review` (check a patch for errors before transmission), `but ai minimize` (reduce a patch to its smallest correct form), `but ai sign` (sign a commit), `but ai sync` (prepare commits for shortwave transmission).

The `minimize` command is unique to the Hermits' implementation. It takes a generated patch and reduces it: removing unnecessary whitespace changes, combining adjacent hunks, and eliminating redundant context lines. The goal is the smallest patch that produces the same result. Every byte saved is transmission time recovered.

The `sync` command packages signed commits into a transmission-optimized format: compressed, error-corrected (the shortwave channel is noisy), and sequenced for reliable reconstruction at the receiving end.

## Requirement 2: Provider-Agnostic AI

Local only. The hermitages have no internet. The provider is Ollama running on each hermitage's computer. Model: TinyLlama 1.1B, quantized to 4-bit. This is the largest model that runs acceptably on the hermitages' hardware (typically a Raspberry Pi 4 or equivalent ARM SBC).

The provider interface: `review(patch, context) -> Assessment`. Single method. The agent reviews a patch for correctness — syntax errors, logic issues, potential conflicts with other recent patches. It does not generate code. Code generation is a human activity. The agent is a reviewer, not an author.

No fallback. If Ollama fails, the brother reviews the patch manually, as they did before the agent existed. The agent is an aid, not a dependency.

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The workflow reflects the shortwave transmission constraint:

1. A brother writes a patch manually (the hermits do not use agents for code generation)
2. The agent reviews the patch for errors and potential conflicts
3. If issues are found, the agent annotates the patch with warnings
4. The brother addresses the warnings or overrides them with a comment explaining why
5. The `minimize` command reduces the patch to its smallest form
6. Benedict signs the commit
7. The `sync` command packages the commit for transmission

COMMIT.msg format (deliberately minimal):

```
Fix: leap year calculation in paschal calendar

Lines: 4 changed
Bytes: 187
Reviewed-By: agent (tinyllama)
Warnings: 0
```

The `Bytes` field is included because it determines how much transmission time the commit will cost. The brothers plan their evening transmission window around the total byte count of pending commits.

## Requirement 4: Polyrepo PR Coordination

The hermits maintain a single repository. There is no polyrepo coordination. The bandwidth cost of maintaining multiple repos would be unjustifiable.

If cross-hermitage coordination is needed (e.g., two brothers working on the same file), it happens over shortwave conversation during the Office of the Air. The conversation is slow. The resolution is thorough.

The forge adapter is not implemented. The hermits do not use a forge. Their repository is synchronized peer-to-peer over shortwave. There is no central server.

## Requirement 5: Agent Memory in Git Branches

Memory branch: `refs/hermits/memory/shared`. Memory types:

- **`conflict-history`**: Records of past merge conflicts and their resolutions. TTL: 1 year. This memory prevents the agent from repeating known-bad merge strategies.
- **`patch-pattern`**: Successful patch patterns that applied cleanly across all hermitages. TTL: 6 months.
- **`atmospheric`**: Shortwave propagation conditions correlated with sync success rates. TTL: 30 days. This unusual memory type helps the agent predict whether a large commit will transmit successfully given current atmospheric conditions.

Memory entries are aggressively compressed. Fiacre's maximum entry size is 512 bytes. Entries exceeding this limit are split or summarized. The total memory branch size across all hermitages must not exceed 1MB — roughly 20 minutes of shortwave transmission time, which is the maximum the Order is willing to allocate for memory synchronization.

Memory retrieval is key-based. No embedding models — the hermitages' hardware cannot run them. Retrieval is fast because the memory is small.

## Requirement 6: Signed Commits via OpenWallet

Each brother has an OpenWallet DID. Keys are stored on hardware tokens. Key generation ceremonies are conducted over shortwave — Benedict walks the brother through the process during a dedicated transmission session. This takes approximately 90 minutes due to the bandwidth constraints of the ceremony protocol.

Key rotation every 180 days (the longest interval in this RFP). The Order accepts the risk because the alternative — more frequent key ceremonies over shortwave — would consume too much of the limited transmission window.

Revocation is immediate but propagation is delayed: a revocation notice reaches all hermitages during the next evening sync, which may be up to 24 hours later. The Order accepts this propagation delay as inherent to their infrastructure.

**Unique insight:** The Hermits' bandwidth constraint produces a discipline that improves agent output quality in ways that cannot be achieved by instruction alone. When every byte costs transmission time, the agent must be precise. The `minimize` command — reducing patches to their smallest correct form — is a pure expression of this discipline. But the insight generalizes: *any* constraint on output size forces the agent to be more careful about what it includes. Token budgets are one form of this constraint. Output size limits are another. The Hermits' system is the extreme case, but the principle — that constraint improves quality — is universal. The shortest patch is not just the cheapest to transmit. It is also the easiest to review, the least likely to conflict, and the most likely to be correct.

---

## Token Budget

| Brother | Input | Output | Total |
|---------|-------|--------|-------|
| Columba | 3,500 | 2,500 | 6,000 |
| Elias | 2,500 | 1,500 | 4,000 |
| Fiacre | 1,800 | 600 | 2,400 |
| Benedict | 1,000 | 300 | 1,300 |
| Ambrose | 500 | 200 | 700 |
| **Task Total** | **9,300** | **5,100** | **14,400** |

No overhead. The budget is the budget. There is nothing to add.

---

*"Brother, is the patch small enough to send before Compline?"*
— Brother Columba, evening transmission, 2025
