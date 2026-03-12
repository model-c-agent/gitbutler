# Agent: s06 — Gate Networking Dependencies for WASI

## Branch

`pr1/s01.s06/feat/wasi-gate-networking`

## Focus

- Run `cargo tree -p but --no-default-features -i reqwest` to find all paths pulling in reqwest
- Run `grep -r "reqwest" crates/*/Cargo.toml` to find direct dependencies
- Check if `gix` pulls networking transitively
- Check if `but-forge` is a hard or optional dependency of `but`
- Gate `reqwest` behind `not(feature = "wasi")` in but-github, but-gitlab, but-update, but-llm
- Gate `posthog-rs` in the but CLI crate
- Gate `ssh2` if present in the dependency chain
- Gate network ops in but-forge/but-forge-storage

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
