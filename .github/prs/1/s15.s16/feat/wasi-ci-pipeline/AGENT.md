# Agent: s16 — Add GitHub Actions workflow for WASI target builds and tests

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Focus

- Follow existing CI patterns in `.github/workflows/` — match style and conventions
- Keep the workflow simple and maintainable
- Use `bytecodealliance/actions/wasmtime/setup@v1` for Wasmtime installation
- Document binary size baseline and CI run time in MEMORY.md
- Set up size budget thresholds that can be tightened over time

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
