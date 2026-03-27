# Agent: s01.s02/feat/wasi-serde-objectid

## Objective

Abstract the ObjectId type in `but-serde` so that `git2::Oid`-specific serialization is gated behind a feature flag, enabling WASI builds that use only `gix::ObjectId`.

## Focus

This is the most impactful serde change. Start by running:

```
grep -r "git2::Oid" crates/
```

to find all usage sites. Understand the `gix::ObjectId` <-> `git2::Oid` conversion layer in `but-oxidize` — that bridge is relevant context for how the two ID types interact today.

The goal is a thin abstraction in `but-serde` that:
- Under native builds: continues to support `git2::Oid` serialization as-is
- Under `wasi` feature: uses `gix::ObjectId` exclusively, with no `git2` dependency

Pay attention to how `Serialize`/`Deserialize` impls are structured for `Oid` types and whether downstream crates depend on specific impl details.

## References

- **Workflow protocol:** [PR.md](../../../../PR.md)
- **Project context:** [INDEX.md](../../../INDEX.md)
- **Coordinator:** [AGENT.md](../../../AGENT.md)
- **Tools:** [SKILLS.md](../../../../SKILLS.md)

## Asking Questions

If you are blocked or unsure, create a `QUESTIONS.md` file in this directory with the format from [AGENT.md](../../../AGENT.md).

If a `but` operation you need doesn't exist, write a tool in `scripts/bin/` per [SKILLS.md](../../../../SKILLS.md). Document it in MEMORY.md.

Check `QUESTIONS.md` before starting — it may contain prior answers.
