# Agent: s01.s05/feat/wasi-gate-process

## Objective

Gate all process spawning behind feature flags so WASI builds contain no `std::process::Command` usage, since WASI does not support spawning subprocesses.

## Focus

Start with a comprehensive search for all process spawning:

```
grep -r "Command::new\|process::Command\|command_group\|std::process" crates/but/src/
```

Key areas to investigate:

1. **`tokio::process` usage** — async process spawning needs the same treatment
2. **`gix` shelling out** — check if `gix` spawns subprocesses (e.g., for SSH, GPG). If so, those code paths need WASI-safe alternatives or gates.
3. **`$EDITOR` spawning** — interactive editor launch must be gated
4. **`command-group`** — this crate manages process groups; gate it behind `native`
5. **Legacy teardown** — verify it's already gated by the `legacy` feature so no duplicate work is needed

For each `Command::new` site, determine whether it belongs to:
- Background sync (gate behind `native`)
- Metrics subprocess (gate behind `native`)
- Editor spawning (gate behind `native` or `tui`)
- Legacy code (already gated by `legacy`)

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
