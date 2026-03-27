# Agent: s11 — Configure Single-Threaded Tokio Runtime for WASI

## Branch

`pr1/s05.s11/feat/wasi-tokio-singlethread`

## Focus

- Run `grep -r "block_in_place" crates/` — these PANIC on current_thread runtime
- Run `grep -r "spawn_blocking" crates/` — these need review for WASI compatibility
- Check workspace tokio features across all Cargo.toml files
- Verify `tokio-util` compatibility with single-threaded runtime
- Configure tokio features: native gets `rt-multi-thread`, `io-std`, `process`, `fs`, `net`, `time`; WASI gets `rt`, `io-std`, `fs`, `time` only
- Set up conditional `#[tokio::main(flavor = "current_thread")]` in main.rs
- Check `parking_lot` threading features — may need gating

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
