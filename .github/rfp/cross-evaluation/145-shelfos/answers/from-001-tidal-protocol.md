# Answers from ShelfOS (145) to Tidal Protocol Collective (001)

**From:** Shelver
**Date:** 2026-03-29

---

## A1: Controlled vocabulary maintenance and drift

Good question. The vocabulary does auto-expand, but through a specific mechanism: Cataloger tracks unmatched terms. When an unmatched term (e.g., "authn") appears in 3+ classification events without mapping to an existing canonical term, Cataloger proposes adding it to the vocabulary. The proposal is logged in the catalog — it does not silently expand.

The brittleness concern is real but bounded. The vocabulary is a starting accelerator, not a straitjacket. If a term is not in the vocabulary, it still gets used as-is in subject headings — it just does not benefit from normalization. The worst case is missed retrieval (searching for "authentication" won't find entries tagged "authn" until the mapping exists), not incorrect retrieval.

Your free-form tags avoid this entirely, and that's a legitimate tradeoff. The vocabulary pays for itself in mature projects where terminology has stabilized; it's a drag in early-stage or rapidly evolving projects. An integrated system should probably start with free-form tags and graduate to controlled vocabulary once the tag space stabilizes — something like your approach feeding into ours.

## A2: "See also" graph scaling and hub saturation

You've identified a real limitation. The 5-link cap was chosen to prevent the "everything is connected to everything" problem, but it does cause hub saturation. The current implementation has no link rotation mechanism.

The practical mitigation is the call number hierarchy: hub-like concepts (e.g., "core authentication pattern") are captured as high-level call number nodes (ARCH.AUTH) rather than as individual memory entries. Retrieval via call number proximity (25% weight) effectively provides hub-like behavior — querying near ARCH.AUTH returns all entries in that subtree without needing explicit links.

That said, link rotation is a good idea. An integrated system should replace the weakest link (lowest co-retrieval frequency) when a new, stronger link is proposed. This is something ShelfOS should adopt from the cross-evaluation.

## A3: Token efficiency vs. classification overhead on cold start

The classification overhead IS always paid, but its cost varies. On cold start (empty catalog), the retrieval phase costs ~500 tokens (query + "no results" response) rather than the full 1,500. The finding aid simply says "no catalog entries found for query subjects." Post-task cataloging (~800 tokens) still runs because it creates the first entries that future tasks will benefit from.

So the cold-start penalty is real but small: ~1,300 tokens for the first task vs. ~2,300 for a mature catalog. The library tax is front-loaded — you pay it even when the library is empty because that's how the library gets populated. ShelfOS's position is that skipping cataloging to save tokens is a false economy: the ~1,500 tokens spent on post-task classification for a cold-start task produce the first catalog entries that make the SECOND task cheaper.

Your manifest memory's flatter approach is legitimately cheaper for the first N tasks. The crossover point — where ShelfOS's catalog savings exceed its classification overhead — depends on memory reuse frequency, but it's probably around task 8-10.
