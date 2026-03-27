# Agent: s14 — Add but-wasi-host crate for sandboxed WASI execution via wasmtime

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Focus

- Study mcagent-wasi at https://github.com/willemneal/mcagent for sandbox patterns
- Study Wasmtime embedding examples for WASI component model
- Key decisions to make and document:
  - Component model vs core wasm
  - Preopened directory permissions (repo rw, config ro)
  - Environment variable passthrough policy
  - AOT compilation and caching strategy
- Document Wasmtime API choices and perf measurements in MEMORY.md

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
