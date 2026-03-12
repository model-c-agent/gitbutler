# Agent: s01.s04/feat/wasi-gate-tui

## Objective

Gate all TUI and terminal dependencies behind a `tui` feature flag so WASI builds exclude them entirely.

## Focus

The TUI module is cleanly separated, but there are edge cases to investigate:

1. **`colored` usage outside `tui/`** — check if `colored` is used for non-TUI output (e.g., log formatting, error messages). If so, those call sites need `#[cfg(feature = "tui")]` guards or an alternative.

2. **`atty`/`is-terminal` checks** — terminal detection is often used in output formatting. Check:
   ```
   grep -r "atty\|is_terminal\|IsTerminal" crates/but/src/
   ```

3. **`but-link` TUI deps** — verify which TUI-related deps `but-link` pulls in and gate them.

4. **Pager** — `pager.rs` likely spawns an external process (`less`, `more`). Under WASI this is a no-op regardless, but gate it cleanly.

Ensure JSON/structured output paths remain functional without the TUI feature.

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
