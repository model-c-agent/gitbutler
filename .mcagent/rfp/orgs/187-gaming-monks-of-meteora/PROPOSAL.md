# The Gaming Monks of Meteora — Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Approach:** Contemplative Agent Architecture

---

## Executive Summary

The Brotherhood proposes an agent system modeled on monastic discipline: slow, deliberate, correct. Agents contemplate before acting. Patches are small and complete. Memory is reflective, not merely retrievable. The system trades speed for correctness and considers the tradeoff a virtue.

---

## Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary is installed to `~/.gitbutler/bin/`. The binary is compiled from Rust, statically linked. Alexios builds it. The command surface is minimal:

- `but ai contemplate` — reads repository state, builds context, returns a summary (no mutations)
- `but ai propose` — generates INDEX.patch + COMMIT.msg as a proposal, does not commit
- `but ai commit` — applies and signs a previously proposed patch
- `but ai remember` — stores or retrieves a memory entry

The separation between `propose` and `commit` is intentional and non-negotiable. Contemplation precedes action. The monk must see the patch, sit with it, and then consciously decide to apply it. There is no `but ai auto` command. There never will be.

## Requirement 2: Provider-Agnostic AI

The monastery uses Anthropic as the primary provider and Ollama (running on a local server in the monastery's cellar) as the fallback for when the satellite internet goes down, which happens during winter storms. Provider selection is manual — Alexios switches providers by editing the config file when the connection drops. There is no automatic failover because the monks distrust automated decisions.

The provider interface: `contemplate(context, task) -> Proposal`. Single method. The proposal includes the patch, the commit message, and a confidence score. If confidence is below 0.8, the proposal includes a `CONTEMPLATION_NOTE` explaining the uncertainty. These notes read like journal entries — "The test coverage for this module is thin. I am uncertain whether the change preserves the invariant on line 47. I recommend manual verification."

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Methodios generates all patches. His workflow is deliberately sequential:

1. Read the entire relevant codebase context (Methodios refuses to generate from partial context)
2. Write a design note explaining the intended change and its rationale
3. Commit the design note to `docs/design/`
4. Wait for Ioannis to review (24-hour window)
5. Generate INDEX.patch
6. Wait 4 hours (mandatory contemplation gap)
7. Review the patch himself
8. Pass to Seraphim for signing

The 4-hour contemplation gap is the monastery's most distinctive process element. It exists because Methodios found that patches generated in a single session contain subtle errors that become visible only after the mind has rested. The gap is encoded in the tooling — `but ai propose` writes a timestamp, and `but ai commit` refuses to apply a proposal less than 4 hours old.

COMMIT.msg format:

```
<short summary>

Design: docs/design/<design-note>.md
Contemplation-Hours: <time between propose and commit>
Confidence: <score>
```

## Requirement 4: Polyrepo PR Coordination

The monastery's projects are single-repo. Cross-repo coordination is supported in the architecture (Alexios implemented a forge adapter for GitHub) but rarely used. When it is used, PR comments follow a minimal format:

```
[Meteora:sync] The frontend branch awaits the backend change.
Patience is appropriate. Do not merge out of order.
```

The monastery does not use urgency markers, priority labels, or deadline indicators. Work completes when it is ready. The forge adapter supports GitHub. Other forges will be supported when the need arises and not before.

## Requirement 5: Agent Memory in Git Branches

Memory branches: `refs/meteora/memory/<brother>`. Nikolaos designed the memory system around three tiers, named after levels of monastic reading:

- **`lectio`** (reading): Raw observations. What the agent saw in the codebase. TTL: 7 days.
- **`meditatio`** (meditation): Patterns derived from observations. What the agent understood. TTL: 30 days.
- **`contemplatio`** (contemplation): Deep insights. Architectural principles, design convictions, lessons learned. TTL: 1 year.

The tier system reflects the monastic belief that understanding deepens over time. Raw observations fade quickly. Patterns persist longer. Deep insights are nearly permanent. Promotion between tiers is manual — Nikolaos reviews `lectio` entries weekly and promotes significant ones to `meditatio`. `Meditatio` entries are reviewed monthly for promotion to `contemplatio`.

Retrieval uses the "contemplative" algorithm: candidates are scored, then re-scored with additional context, then re-scored a final time. Three passes. The first pass finds relevant memories. The second pass eliminates false positives. The third pass ranks by depth — `contemplatio` entries are preferred over `meditatio`, which are preferred over `lectio`.

## Requirement 6: Signed Commits via OpenWallet

Each monk holds an OpenWallet DID. Key generation is a formal ceremony conducted in the chapel by Seraphim with two witnesses. Keys are stored on hardware tokens (YubiKeys) kept in a locked cabinet in the sacristy. Key rotation follows the monastic calendar — keys are rotated at the beginning of each liturgical season (roughly every 60-90 days).

Revocation is immediate and does not require a ceremony. Seraphim can revoke a key unilaterally if compromise is suspected. The community accepts this asymmetry: creation requires ceremony, destruction requires only urgency.

**Unique insight:** The 4-hour contemplation gap between proposal and commit is the monastery's most valuable contribution. It is a formalization of something every experienced developer knows: the best time to review your own code is not immediately after writing it. By encoding this gap into the tooling — making it a *technical constraint* rather than a *behavioral suggestion* — the system prevents a class of errors that no amount of testing catches: the errors that look correct to the author in the moment but reveal themselves after rest. This is not inefficiency. It is engineered correctness.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Ioannis | 1,200 | 600 | 1,800 |
| Methodios | 4,000 | 3,500 | 7,500 |
| Alexios | 3,000 | 2,000 | 5,000 |
| Seraphim | 1,500 | 500 | 2,000 |
| Nikolaos | 2,500 | 800 | 3,300 |
| **Task Total** | **12,200** | **7,400** | **19,600** |

Contemplation overhead (design notes, re-scoring, mandatory gaps): 3,500 tokens. Grand total per task: **23,100 tokens**.

---

*"The code will wait. It has nowhere else to be."*
— Brother Methodios, during a code review
