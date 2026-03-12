# Agent: s10 — Gate but-core Worktree/Checkout Module for WASI

## Branch

`pr1/s09.s10/feat/wasi-gate-core-checkout`

## Focus

- Small, focused change — gate the checkout module only
- Check scope of git2 in but-core — only checkout uses it
- Verify but-ctx gating from s09 is compatible with this change
- Gate `src/worktree/checkout/` behind `#[cfg(not(feature = "wasi"))]`
- Provide a stub that returns an error for WASI ("checkout not available in WASI")
- Gate the git2 dependency in but-core's Cargo.toml

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
