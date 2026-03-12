# Agent: s07 — Gate Platform-Specific Dependencies for WASI

## Branch

`pr1/s01.s07/feat/wasi-gate-platform`

## Focus

- Investigate how `but-secret` is used downstream — trace all callers
- Check if `gix::credentials` shells out to system credential helpers
- Design env-var naming scheme for secrets carefully (e.g., `GITBUTLER_SECRET_<NAME>`)
- Gate `keyring` in but-secret and provide env-var auth fallback for WASI
- Gate `notify` in but-settings (inotify C bindings won't compile for WASI)
- Gate `dirs` in but-path (no home directory concept in WASI)
- Gate `machine-uid` in but/but-update
- Gate `open` in but-api (no browser to open in WASI)

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
