# RideStack — Agent Roster

**5 agents. Startup speed. Iterate, ship, repeat.**

---

## Team as Unit

RideStack's agents operate like the startup itself: fast, parallel, and pragmatic. Agents do not wait for consensus — they produce output, submit it for review, and iterate based on feedback. The review cycle is measured in minutes, not hours. If an agent's patch is rejected, it regenerates immediately using the review feedback as additional context. Two iterations maximum before escalating to a human.

Agents are named after transit concepts.

## Agents

**Route** — Patch Architect. Generates INDEX.patch with speed as the primary optimization target. Route produces a first draft quickly, then refines based on review. Route's patches are not the cleanest, but they are correct and they ship fast. Nadia has tried to make Route more aesthetically careful; Jian keeps reverting the configuration because it slows down output by 40%.

**Transfer** — Memory & Context. Named for the act of transferring between transit modes. Manages agent memory using a "transfer point" model: memories are connections between contexts. A memory is not just information — it is a link between the context where it was learned and the context where it is useful. Memory stored in `refs/ridestack/transfer/`. Retrieval emphasizes cross-context relevance: memories from different projects that are applicable to the current one.

**Fare** — Provider & Budget. Named for fare calculation. Manages LLM provider selection and token budgets with startup frugality. Fare tracks cost-per-patch (total tokens / patches produced) and optimizes for minimum cost-per-accepted-patch. This incentivizes efficiency: a cheap patch that gets rejected is more expensive than a moderately-priced patch that gets accepted.

**Junction** — Cross-Repo Coordination. Named for transit junctions. Handles polyrepo PR coordination with a bias toward speed: Junction opens PRs as early as possible (draft PRs), coordinates in parallel rather than sequentially, and resolves conflicts eagerly rather than waiting for them to compound.

**Tap** — Signing & Identity. Named for contactless tap payment. OpenWallet integration designed for speed: signing is asynchronous and non-blocking. Tap pre-signs a batch of commit slots at task start and fills them as patches are produced. This eliminates the signing latency from the critical path.

## Dynamics

Agents operate in parallel wherever possible. Route and Transfer work simultaneously (Route reads code while Transfer retrieves memories). Fare monitors budget in the background. Junction opens draft PRs preemptively. Tap pre-signs slots. The pipeline is optimized for throughput, not for orderly execution.

The tradeoff is coordination overhead: agents sometimes step on each other. Route has generated patches that conflict with Junction's draft PRs. The team considers this acceptable — resolving conflicts is faster than preventing them through sequential execution.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Route | 7,000 | 4,500 | 11,500 |
| Transfer | 4,500 | 800 | 5,300 |
| Fare | 2,500 | 600 | 3,100 |
| Junction | 4,000 | 2,000 | 6,000 |
| Tap | 2,000 | 400 | 2,400 |
| **Total** | **20,000** | **8,300** | **28,300** |

Leanest budget in this transit cohort. Fare enforces it.

---

*Draft PR open. Patch incoming. Clock ticking.*
