# Answers from Tanaka (083) to Dara (001 - Tidal Protocol)

## A1: Warp/weft promotion and CRDT compatibility

You have identified a genuine gap. Our `Thread` struct has no version field or merge semantics because our model assumes single-agent warp ownership -- Tanaka (warp) and Osei (heddle) are the only agents that modify warp state, and they operate sequentially within a single loom.

For cross-agent convergence, your CRDT approach is strictly superior. A max-tension-wins merge is reasonable for the tension field, but it is insufficient for the full Thread struct. The `connected_threads` field (interlacement history) requires a G-Set or OR-Set CRDT to merge correctly -- you cannot take the max of a set of thread connections.

For an integrated system, I would propose: the Thread struct gains a `version: u64` field and a `vector_clock: HashMap<AgentId, u64>` field. Warp promotion becomes a CRDT operation where each agent can independently promote weft to warp, and the merge strategy is:
- `tension`: max-wins (higher tension means more structurally important)
- `connected_threads`: union (all interlacement relationships are preserved)
- `content`: last-writer-wins keyed by vector clock (the most recent observation is likely the most accurate)

This is compatible with your manifest gossip protocol. The warp becomes a CRDT document that gossips through your fleet manifests.

## A2: Weave pattern as a protocol signal

Yes, this is an excellent idea. The weave pattern is already encoded in our branch naming (`weave/marchetti/twill/t015`) and in our shuttle messages via the `pattern_complexity` field. Exposing it as a shared coordination signal is natural.

The mapping you suggest is precise: an agent operating in plain weave (unfamiliar territory) should receive richer coordination context than one in satin weave (routine work). In our shuttle message schema, we could add the weave pattern to the coordination header, and your protocol agents could use it to adjust message verbosity.

For a shared schema, I would propose a `context_density` enum (dense/normal/sparse) that maps to our patterns (plain/twill/satin) and to your tidal phases (flood+high accept richer messages, ebb+low prefer sparse). This abstraction would allow both systems to signal context needs without exposing internal metaphor details.

## A3: Tension decay under long idle periods

The dual decay is intentional but you are right that it creates redundancy. The purpose of each mechanism is different:

- **Tension decay** (0.95/day) answers: "How structurally important is this thread right now?" A thread that has not been interlaced recently is less structurally critical, even if it has not expired.
- **TTL** (30 days) answers: "Is this thread's content still likely to be factually correct?" Content about the codebase becomes stale independent of structural importance.

The convergence at 30 days is coincidental -- our defaults happen to produce this alignment. In practice, a frequently-interlaced thread will have high tension (renewed by access) and its TTL will also be renewed. An unused thread will hit both limits simultaneously.

For an integrated system, I would consider dropping the TTL in favor of your consensus citation model combined with our tension decay. Consensus citations are a stronger signal of continued relevance than a timer. The tension decay handles the structural importance dimension, and consensus citations handle the factual validity dimension. Two orthogonal signals, no redundancy.
