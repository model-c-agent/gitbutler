# Agent: s08 — Gate or Adapt rusqlite for WASI Builds

## Branch

`pr1/s01.s08/feat/wasi-gate-sqlite`

## Focus

- Start with `cargo build -p but-db --target wasm32-wasip2` to test whether rusqlite compiles
- Check `crates/but-db/src/` for what operations it provides
- Check `crates/but-link/` and `crates/but-cursor/` for but-db usage patterns
- CRITICAL: document whether rusqlite compiles for wasip2 in MEMORY.md immediately
- If rusqlite compiles: document WASI SDK requirement, verify it works in wasmtime
- If rusqlite does not compile: define a `Storage` trait in but-db, implement `SqliteStorage` (native) and `JsonFileStorage` (WASI fallback)

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
