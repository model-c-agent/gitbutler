# Agent: s09 — Gate git2::Repository in but-ctx for WASI

## Branch

`pr1/s02.s09/feat/wasi-gate-ctx`

## Focus

- Run `grep -r "git2::Repository" crates/` to find all usage across the codebase
- Understand the CommandContext struct fully before making changes
- Check which methods are called in non-legacy code paths
- The ObjectId abstraction from s02 must be landed first — verify it is available
- Make `git2::Repository` field conditional: present on native, absent on WASI
- Provide gix-only context under `#[cfg(feature = "wasi")]`
- Methods requiring git2::Repository should return errors or be gated behind cfg
- Trace downstream crate impact carefully — but-ctx is widely imported

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
