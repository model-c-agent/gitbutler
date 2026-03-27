# Agent: s12 — Configure minimal gix feature set for WASI compilation

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Focus

- Run `cargo tree --target wasm32-wasip2 -p but --no-default-features --features wasi` to audit the full dependency tree
- Check gix Cargo.toml for feature descriptions — understand what each feature pulls in
- Try `default-features = false` with incremental additions to find the minimal viable set
- Disable `parallel` (threading), gate `credentials` (shell-out), verify `sha1` and `zlib-rs` are pure Rust
- Document the final feature matrix in MEMORY.md with rationale for each inclusion/exclusion

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
