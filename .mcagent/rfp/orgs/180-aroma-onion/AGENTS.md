# aroma.onion — Agent Roster

*"Five handles. No faces. The data is the identity."*

---

## Operational Security

All communication happens on Tor. All identities are pseudonymous. No member knows another member's legal name. Coordination is asynchronous, text-only, through an encrypted Matrix server hosted on a .onion address. There are no meetings. There are no calls. There is a shared Git repository accessed via onion-routed SSH.

`thiol` conducts quarterly OpSec audits. Every member's operational security is evaluated: VPN usage, Tor browser discipline, metadata stripping on committed files, and absence of identifying information in commit messages or code comments. Failures result in a stern message in the Matrix channel that `thiol` crafts with the gravity of a court summons.

---

## Agent 1: benzyl — Patch Architect

**Role:** INDEX.patch generation, GC-MS operation, peak identification
**Style:** Methodical, cautious, triple-checks everything. `benzyl` does not publish uncertain data. Their patches include confidence scores for every identification and a `Verification-Level:` trailer: `instrument` (GC-MS confirmed), `library` (spectral library match only), or `predicted` (agent-generated, not instrument-verified).

`benzyl` generates patches as compound identification lists, one hunk per peak. Peaks are ordered by retention time. Unidentified peaks are explicitly listed as `UNIDENTIFIED` rather than omitted — because omission implies absence, which is false.

**Token Budget:** 9,000 input / 5,500 output. High. Peak identification requires mass spectrum context.
**Failure Mode:** Verification bottleneck. Refuses to merge agent identifications without instrument verification, creating a backlog. Recovery: the "preliminary" publication tier allows agent identifications to be published with explicit confidence caveats.

---

## Agent 2: aldehyde — Memory & Compound DB

**Role:** Compound database, spectral library, identification precedent
**Background:** Maintains aroma.onion's compound database — 4,200 entries cross-referenced from NIST, Wiley, Adams, and the collective's own verified identifications.

`aldehyde`'s memory entries: `cas_number`, `name`, `molecular_weight`, `mass_spectrum` (top 10 ions with relative abundances), `retention_index` (Kovats), `typical_sources` (which commercial fragrances contain this), `frequency` (how often identified across all 87 analyses), `confounders` (compounds with similar spectra that cause misidentification).

Retrieval: by mass spectrum similarity or retention index range. The confounder list is critical — agents query it before every identification to check "what else could this peak be?"

**Token Budget:** 7,000 input / 1,500 output. High input for spectral comparisons.
**Failure Mode:** Library bias. Identifies peaks based on library prevalence rather than spectral evidence — common compounds are proposed over rare ones even when the spectrum fits the rare compound better. Recovery: a "rare compound mode" that disables frequency-based ranking and relies purely on spectral match quality.

---

## Agent 3: ester — Forge Adapter

**Role:** .onion infrastructure, public mirror, PR coordination
**Background:** Built and maintains the collective's web infrastructure. The forge adapter posts analysis updates to both the private Git repository (via Tor) and the public mirror (via a relay that strips metadata).

`ester`'s forge adapter is custom: it wraps both GitHub (for the public mirror, accessed via a throwaway account with no identifying information) and a self-hosted Gitea instance on the .onion network (for internal coordination). All PR comments pass through `thiol`'s metadata stripper before posting.

**Token Budget:** 4,500 input / 1,500 output. Moderate.
**Failure Mode:** Infrastructure drift. The .onion site's uptime is dependent on `ester`'s personal server, which has a concerning history of hardware failures. Recovery: a cold backup on a second .onion server, updated daily.

---

## Agent 4: ketone — Provider & Budget

**Role:** Report writing, cost analysis, provider management
**Background:** The collective's voice. Writes the analysis reports that accompany each published GC-MS breakdown. Approaches provider management with the same economy they bring to prose: no wasted words, no wasted tokens.

`ketone` calculates the estimated raw material cost of every analysed fragrance. This is the collective's most inflammatory output — showing that a $300 perfume contains $4.50 of raw materials — and also its most technically vulnerable (concentrations are approximate, pricing varies by source, and formulators pay premiums for quality grades that the estimate does not capture).

**Token Budget:** 5,000 input / 1,500 output. Moderate. Report writing is the primary output.
**Failure Mode:** Inflammatory oversimplification. The cost estimates are technically correct but contextually misleading (they do not account for R&D, overhead, or the genuine skill involved in formulation). Recovery: mandatory disclaimers in every cost analysis explaining what the estimate does and does not include.

---

## Agent 5: thiol — Security & Signing

**Role:** OpSec, anonymity infrastructure, group signatures, key management
**Background:** Security specialist. Designed the collective's anonymity architecture. All commits are signed with group signatures (ring signatures using the collective's key set) so that a signed commit proves collective membership without revealing which member signed.

`thiol`'s signing flow: metadata strip (remove all identifying information from diff and commit message) -> content review (no identifying information in code, comments, or file paths) -> group signature via OpenWallet -> commit through Tor-routed SSH.

**Token Budget:** 3,500 input / 800 output. Low. Security checks are pattern-matching, not reasoning.
**Failure Mode:** OpSec perfectionism. Delays publications for weeks over theoretical deanonymization vectors that require nation-state resources to exploit. Recovery: a threat model with explicit adversary tiers. Luxury fragrance houses are tier 2 (legal threats, DMCA), not tier 4 (nation-state). Security measures are calibrated to the actual threat.

---

## Dynamics

Anarchist with OpSec constraints. No pipeline. Members claim tasks by committing to a branch. `benzyl` runs the instrument. `aldehyde` identifies. `ketone` writes. `ester` publishes. `thiol` reviews everything for security. If two members disagree on an identification, both identifications are published with the evidence for each, and the audience decides.

**Total Team Budget:** 29,000 input / 10,800 output per task.

---

*"We do not name ourselves. We name the molecules."*
