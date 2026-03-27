# Agent: s15 — Add WASI integration test harness and smoke tests

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Focus

- Research test patterns for WASI/wasmtime — programmatic module loading in tests
- Set up test repos with `gix` (not git CLI) for deterministic, hermetic tests
- Compare native and WASI JSON output for consistency — flag any divergences
- Document perf baselines in MEMORY.md (startup time, command execution time)
- Ensure all tests are deterministic and CI-friendly (no flaky timing deps)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
