# Agent: s01.s03/feat/wasi-oxidize-noop

## Objective

Make `but-oxidize` compile as an empty crate when the `wasi` feature is active. Since it bridges `git2` and `gix`, it is unnecessary in a pure-`gix` WASI build.

## Focus

Check what downstream crates import from `but-oxidize` and whether they need conditional imports too. Key investigation:

```
grep -r "but_oxidize\|but-oxidize" crates/
```

Look at:
- Which crates have `but-oxidize` as a dependency
- Which symbols they import from it
- Whether those imports are already behind `cfg` gates or need to be wrapped

The crate is marked "soon obsolete" so the approach is straightforward: wrap all public items in `#[cfg(not(feature = "wasi"))]`. Ensure `Cargo.toml` conditionally excludes `git2` dependency under `wasi`.

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
