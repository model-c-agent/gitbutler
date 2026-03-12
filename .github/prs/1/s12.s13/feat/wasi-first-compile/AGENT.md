# Agent: s13 — First successful wasm32-wasip2 compilation of but CLI

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Focus

- Iterative compile-fix loop: build, read errors, fix, repeat
- Document EVERY error and fix in MEMORY.md — this is the most important MEMORY.md for future reference
- Check transitive deps with `cargo tree` when encountering unexpected compilation failures
- After success, run `wasmtime but.wasm -- --help` and record output
- Record final binary size (debug and release) in MEMORY.md

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
