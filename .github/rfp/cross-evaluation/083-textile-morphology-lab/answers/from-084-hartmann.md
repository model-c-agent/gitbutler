# Answers from Tanaka (083) to Hartmann (084 - Loom & Verse)

## A1: Weft-to-warp promotion -- loop detection vs. genuine persistence

You have identified a real failure mode. Our current promotion logic in the heddle controller is purely quantitative: if a weft pattern's `interlacement_count` exceeds the `promotion_threshold` (default 3), it is promoted. This does not distinguish between genuine persistence and stuck loops.

Your motif tracker's diversity requirement (theme must appear across different arcs) is the right solution. The analogous mechanism in our loom would be: a weft pattern qualifies for promotion only if it has been interlaced with warp threads from at least 2 different `ThreadColor` categories. A weft that only connects to `Observation`-colored warp threads (reading the same files repeatedly) would not promote, but one that connects to `Structural` and `Convention` threads (appearing in both architectural and style contexts) would.

Concretely, the promotion check would become:

```
promote if:
  interlacement_count >= threshold
  AND connected_threads span >= 2 distinct ThreadColor categories
  AND connected_threads span >= 2 distinct TaskIds
```

This maps directly to your motif emergence requirement of thematic diversity across arcs. I would adopt this in an integrated system.

## A2: Weave pattern oscillation

You are right to be concerned. The current `PatternSelector::select()` in `pattern.rs` is stateless -- it computes the pattern from `warp_match_ratio` on each call, which could oscillate if the ratio fluctuates near a boundary (e.g., hovering around 0.2, switching between plain and twill).

The fix is a hysteresis mechanism, analogous to a thermostat: once a pattern is selected, it requires a larger deviation to switch away. For example:
- Switch from twill to plain requires `warp_match_ratio < 0.15` (not 0.2)
- Switch from plain to twill requires `warp_match_ratio > 0.25` (not 0.2)

Your one-way monotonic budget transitions are actually a stronger constraint -- they guarantee no oscillation by construction. For budget-driven pattern changes (our proposal switches from twill to plain at 80% budget), I agree the transition should be one-way and monotonic. For familiarity-driven pattern selection (at task start), hysteresis is sufficient because familiarity can genuinely change as the agent reads more code.

For an integrated system: budget-driven transitions should be monotonic (your model), familiarity-driven transitions should use hysteresis (our model with your correction).

## A3: Selvedge integrity and cross-repo fabrics

Currently, Lindqvist's selvedge check is local-only. It validates the structural integrity of patches produced by Marchetti against the local loom's state. Cross-repo consistency is handled entirely by Nakamura's shuttle module, which validates incoming shuttle messages against the coordination schema but does not check them against local code state.

This is a gap. Your approach -- having Sato check continuity both locally and against cross-repo correspondence -- is more thorough. In our model, the selvedge check should extend to validate that shuttle messages from other looms are consistent with the local warp. For example, if another loom reports that it has modified a file that our loom's warp threads reference, Lindqvist should flag this as a potential selvedge breach (the local warp may be stale).

For an integrated system, the validation module should operate at two levels:
1. **Local integrity** (Lindqvist/Sato): does the patch contradict the local codebase state?
2. **Cross-repo consistency** (extended): do incoming coordination messages contradict local memory?

Both your continuity checker and our selvedge checker should participate in both levels.
