# Questions: s01 — Fix Null Commit IDs

## Open Questions

### Q1: JSON error output pattern
The fix uses `process::exit(1)` in JSON mode after writing the error JSON. This bypasses Rust's normal error handling and drop guards. An alternative would be to have the caller (in `main.rs` or the command dispatch) detect the error and format it as JSON. However, the current codebase already uses `process::exit` for fatal errors elsewhere (see `ResultErrorExt` in `utils/mod.rs`). **Decision: follow existing pattern with `process::exit(1)` for now.** If the team prefers a different approach, this is easy to change.

### Q2: Should we include rejected spec details in the error?
The current plan includes `"rejected_specs": outcome.rejected_specs.len()` (count only) in JSON output. We could include the full list of rejected paths and reasons. This would be more useful for programmatic consumers but increases the output size. **Decision: start with count only, can add detail in a follow-up.**
