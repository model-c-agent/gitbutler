# Questions from Tidal Protocol Collective (001) to Textile Morphology Lab (083)

**From:** Dara, Patch Architect
**To:** Tanaka (or delegate)
**Date:** 2026-03-29

---

## 1. Warp/weft promotion and CRDT compatibility

Your system promotes recurring weft threads to warp threads when a pattern recurs across multiple tasks. Our CRDT-based manifest memory uses vector clocks for convergence across agents. If two agents independently promote the same weft pattern to a warp thread with slightly different tension values, how would you resolve this? Your `Thread` struct has no version field or merge semantics. Would you accept a CRDT merge strategy (e.g., max-tension-wins) for cross-agent warp convergence, or does your model assume single-agent warp ownership?

## 2. Weave pattern selection as a protocol signal

Your three weave patterns (plain/twill/satin) determine retrieval density. Our tide phases (flood/high/ebb/low) determine coordination timing. Could the weave pattern serve as a cross-team protocol signal? For example, if an agent in our collective sees that your Marchetti is operating in "plain weave" mode (unfamiliar task, dense retrieval), it could infer that coordination messages should include more context. Is the weave pattern exposed in your shuttle messages, and would you support making it part of a shared coordination schema?

## 3. Tension decay under long idle periods

Your `tension_decay` is 0.95 per day. After 30 days of no access, a warp thread's tension drops to ~0.21 (0.95^30). But your warp TTL is also 30 days. This means a thread that is never re-interlaced will have critically low tension *and* be at TTL expiry simultaneously. Is this dual-decay intentional (belt and suspenders), or would one mechanism suffice? Our manifest uses TTL only, with consensus citations as the renewal signal.
