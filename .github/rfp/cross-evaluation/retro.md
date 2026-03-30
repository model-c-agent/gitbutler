# Cross-Evaluation Retrospective

**Session:** `b5da394f-4bce-4730-a2c1-9a5c1737075a` (inherited-sauteeing-harp)
**Date:** 2026-03-29 16:22 UTC to 2026-03-30 18:36 UTC (~26 hours wall-clock, ~8 hours active)
**Scope:** Implement 5 competing `but-ai` plugin proposals, cross-evaluate them, synthesize a unified design, begin unified implementation

---

## What Got Stuck and Why

### 1. Hartmann Agent Idle Loop (the most visible failure)

**Symptom:** After the team lead sent shutdown requests at 02:27 UTC, 4 of 5 agents (Tanaka, Dara, Vassiliev, Shelver) acknowledged and terminated. Hartmann (084) entered an infinite idle loop, sending `idle_notification` messages every ~15 seconds without processing the shutdown request.

**Root cause:** The Hartmann agent was spawned as a teammate in the `rfp-cross-evaluation` team. When it completed its work (integrated proposal + Q&A answers), it transitioned to an "available" idle state. The team lead sent shutdown requests as `SendMessage` calls, but Hartmann kept emitting idle notifications instead of processing the shutdown message. This created a tight loop:
1. Team lead sends shutdown request
2. Hartmann emits idle notification
3. Team lead sends another shutdown request
4. Repeat

This consumed 3 round-trips (L547-L558) over ~1 minute before the team lead gave up and force-deleted the team directory via `rm -rf`.

**Contributing factor:** There is no `terminate` or `kill` command for individual teammates. The only exit mechanisms are: (a) the agent processes the shutdown message itself, (b) `TeamDelete` removes the entire team, or (c) manual cleanup of team files. When an agent ignores shutdown messages, the team lead has no graceful recourse.

**Fix needed:** A `TeamTerminate` tool that forcefully stops a specific teammate, or a protocol where idle agents auto-terminate after N consecutive idle notifications with no new work.

---

### 2. Rate Limit Hit During Cross-Evaluation (caused 2+ hour pause)

**Symptom:** At 00:01 UTC (L471), while 3 of 5 cross-evaluation agents had completed and 2 (Hartmann, Vassiliev) were still working, the session hit its API rate limit. The system returned "You've hit your limit - resets 10pm (America/Toronto)" and blocked all further processing.

**Impact:**
- Hartmann and Vassiliev's in-flight work was interrupted mid-execution
- 3 idle agents (Dara, Shelver, Tanaka) kept sending idle notifications that accumulated as unprocessed messages
- When the user typed "continue" at 02:14 UTC (~2 hours later), the team lead found that Hartmann and Vassiliev had actually completed their work before the rate limit hit -- they had written their integrated proposals and Q&A answers to disk. The rate limit prevented the team lead from discovering this.

**Root cause:** Running 5 cross-evaluation agents in parallel (each reading all 5 proposals + implementations, writing questions, waiting for answers, writing integrated proposals) consumed tokens faster than the rate limit allowed. The cross-evaluation phase was the most token-intensive part of the session because each agent needed to:
- Read ~18,000 lines of Rust across 5 implementations
- Read 5 proposal documents
- Write 3 questions to each of 4 other orgs (12 questions per agent, 60 total)
- Wait for and read answers
- Write an integrated proposal (~2,000-4,000 words each)

**Contributing factor:** No token budget tracking at the session level. The team lead had no visibility into how close the session was to the rate limit until it hit.

**Fix needed:** Session-level token budget monitoring. The team lead should be able to query remaining capacity and throttle agent spawns accordingly (e.g., run 2 agents at a time instead of 5).

---

### 3. Worktree Base Branch Mismatch (early ~30 min delay)

**Symptom:** At 22:14 UTC (L127), `cargo check -p but-ai` failed in the worktrees because the `but-ai` crate skeleton couldn't find its dependencies.

**Root cause:** Worktrees were branched from local `master`, but local `master` was stale -- it didn't have the modern `but-*` crate structure. The correct base was `upstream/master`. This required:
1. Removing all 5 worktrees
2. Recreating them from `upstream/master`
3. Re-bootstrapping the `but-ai` skeleton in each

**Time lost:** ~20 minutes of trial-and-error before identifying that `master` vs `upstream/master` was the issue.

**Fix needed:** Before creating worktrees, verify the base branch has the expected crate structure. A pre-flight check like `ls $BASE/crates/but-llm` would have caught this immediately.

---

### 4. OpenSSL Build Failure in Worktrees (early ~15 min delay)

**Symptom:** After fixing the base branch, `cargo check -p but-ai` still failed because the `but-ai` skeleton depended on `but-llm`, which pulled in `reqwest` with `openssl-vendored`, which failed to compile in the worktree environment.

**Root cause:** The initial `but-ai` skeleton had `but-llm` and `but-tools` as dependencies. These pulled in heavy transitive dependency chains (OpenSSL, reqwest) that failed to build. The skeleton didn't actually need these dependencies -- they were included speculatively.

**Fix:** Stripped the skeleton down to `anyhow`, `serde`, `serde_json`, and `tracing` only. Agents would add heavier dependencies as needed during implementation.

**Lesson:** Start with minimal dependencies. Let implementers pull in what they need rather than pre-loading the dependency tree.

---

### 5. lib.rs Overwrite During Phase 2 (Org 083, ~10 min delay)

**Symptom:** After Tanaka (Phase 1) wrote 17 structural spine files and generated a patch, Tanaka reset the worktree. When Phase 2 agents (Osei, Marchetti, Nakamura) wrote their implementations, `lib.rs` was overwritten back to the empty skeleton. Lindqvist (Phase 3 reviewer) caught this -- the crate had all the source files but `lib.rs` didn't declare any modules, so nothing actually compiled.

**Root cause:** The patch-based workflow (INDEX.patch + COMMIT.msg) was designed so agents produce artifacts without modifying the working tree. But Tanaka's agent both wrote files directly AND produced a patch, then reset the working tree after generating the patch. The reset removed the `lib.rs` that declared modules. Phase 2 agents only wrote their own module files, not `lib.rs`.

**Fix:** The team lead manually reconstructed `lib.rs` from the patch content. Also fixed a compile error in Nakamura's `coordination.rs`.

**Lesson:** The patch workflow and the direct-write workflow are incompatible in the same worktree. Pick one. If agents write directly, don't reset. If agents produce patches, apply them sequentially.

---

### 6. TeamCreate Failures During Unified Phase (late, ~18 min delay)

**Symptom:** At 18:17 UTC (L595), after approving the unified plan, the team lead tried to create a new team (`rfp-unified`). The first attempt appeared to succeed, but subsequent operations failed. The team lead attempted `TeamCreate` 3 times (L595, L599, L603) with a `TeamDelete` in between (L601).

**Root cause:** Leftover state from the previous `rfp-cross-evaluation` team. The cleanup command at L597 (`rm -rf`) ran, but there was a timing issue -- the team directory or task state wasn't fully cleaned up before the new team was created.

**Time lost:** ~18 minutes between the first `TeamCreate` attempt (18:17) and the successful team creation (18:35).

**Fix needed:** `TeamCreate` should fail fast with a clear error if conflicting state exists, rather than silently creating a broken team.

---

### 7. Context Window Exhaustion (terminal failure)

**Symptom:** The conversation was compacted and continued in a new session. The original session had 607 messages over ~26 hours.

**Root cause:** The session accumulated massive context from:
- 5 full proposal reads (~15,000 tokens each)
- 5 implementation outputs from agent spawns
- 22 AGENT.md file writes
- Cross-evaluation: 60 questions + 60 answers + 5 integrated proposals
- All teammate messages, idle notifications, tool results
- The unified plan synthesis

The cross-evaluation phase was the biggest context consumer because the team lead received all teammate messages (questions asked, answers received, idle notifications) and had to track 5 agents' progress simultaneously.

**Contributing factor:** The Hartmann idle loop (issue #1) and the accumulated idle notifications from completed agents during the rate limit window (issue #2) wasted context on zero-value messages.

**Fix needed:**
- Idle notifications should be suppressed or batched after the first one
- Agent results should be summarized rather than streamed as full messages
- Consider running multi-org coordination as separate sessions rather than one mega-session

---

## Timeline Summary

| Time (UTC) | Event | Duration Lost |
|------------|-------|--------------|
| 21:36-22:00 | Worktree base branch mismatch | ~20 min |
| 22:00-22:14 | OpenSSL build failure | ~15 min |
| 22:50-23:00 | lib.rs overwrite, manual fix | ~10 min |
| 00:01-02:14 | Rate limit hit, session paused | ~2 hours |
| 02:27-02:32 | Hartmann idle loop, force cleanup | ~5 min |
| 18:17-18:35 | TeamCreate failures | ~18 min |
| 18:36 | Context exhaustion, session end | terminal |

**Total time lost to issues:** ~3 hours 10 minutes (of ~8 hours active time = ~40% overhead)

---

## What Went Well

1. **Parallel org implementation worked.** 4 orgs ran simultaneously via Agent spawns, completing in ~10 minutes each. All produced compiling Rust code.
2. **Cross-evaluation produced genuine intellectual exchange.** Agents asked probing, specific questions and gave honest, detailed answers acknowledging weaknesses. The cold-start problem, tension escalation, and survival model gaps were all real findings.
3. **The unified plan synthesized 5 proposals into a coherent architecture.** All 5 integrated proposals converged on the same core decisions (3-state lifecycle, survival distributions, 6-component scoring), which validated the cross-evaluation process.
4. **Patch workflow for Org 083 mostly worked.** Despite the lib.rs incident, the INDEX.patch + COMMIT.msg pattern successfully captured the structural spine.

---

## Recommendations for Future Multi-Agent Sessions

1. **Budget tokens explicitly.** Track cumulative token usage and throttle parallelism when approaching rate limits.
2. **Use separate sessions for independent phases.** Implementation, cross-evaluation, and unification could each be their own session, passing only artifacts (file paths) between them.
3. **Add a kill switch for agents.** `TeamTerminate(agent_name)` should forcefully stop a specific agent.
4. **Suppress idle notification floods.** After the first idle notification, batch or suppress subsequent ones until new work arrives.
5. **Validate base branches before creating worktrees.** A 5-second pre-flight check would have saved 20 minutes.
6. **Choose one workflow per worktree.** Either direct-write (agents modify files) or patch-based (agents produce patches, lead applies them). Not both.
7. **Pre-flight dependency checks for skeleton crates.** Only include dependencies that are actually used in the skeleton.
