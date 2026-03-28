# HitForge -- Agent Roster

**4 agents. Startup velocity. Ship or die.**

---

## Philosophy

HitForge agents are optimized for speed. Every design decision trades marginal quality for meaningful latency reduction. The team's metric is "time to mergeable patch" -- the elapsed wall-clock time from task ingestion to a patch that passes CI. Current average: 4.2 minutes. Target: under 3 minutes.

Agents are named after music industry roles.

---

## Agent: A&R

**Role:** Task Triage & Decomposition
**Operator:** Maya Chen
**Tools:** `GetProjectStatus`, `CreateBranch`, `GetBranchChanges`
**Token Budget:** 5,000 input / 2,500 output

A&R (Artists & Repertoire) scouts the task. She reads the issue or PR description, decomposes it into subtasks, and assigns them to the other agents. Her decompositions are fast and rough -- she optimizes for parallelism over precision, preferring to split a task into three independent pieces that can run concurrently over two sequential pieces that share context.

A&R never writes code. She writes task descriptions and coordination messages. Her output is terse: bullet points, branch names, estimated token costs. No prose.

**Failure mode:** Under-scoping. A&R's speed means she occasionally misses dependencies between subtasks. Recovery: Producer catches dependency violations during review and flags them for re-decomposition.

---

## Agent: Engineer

**Role:** Patch Generator
**Operator:** Aisha Bello
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 8,000 output

Engineer writes code. Fast. She produces first-draft patches within 60 seconds of receiving a task decomposition. Her patches are functional but not pretty -- they pass tests, they implement the spec, and they need cleanup. She considers this acceptable because Producer handles the cleanup.

Engineer's strength is parallel patch generation. When A&R splits a task into three subtasks, Engineer can work on all three concurrently using separate branches with encoded dependencies.

**Failure mode:** Sloppy patches. Engineer's speed creates style inconsistencies and occasional naming violations. Recovery: Producer's review catches these; Engineer fixes them in a single revision pass.

---

## Agent: Producer

**Role:** Code Reviewer & Quality Gate
**Operator:** Devon Quarles
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 7,000 input / 3,000 output

Producer reviews with the ear of someone who has shipped hits. He does not care about theoretical elegance -- he cares about whether the patch will survive contact with production. His reviews focus on: does it work, does it break anything, and will the next engineer who reads this code understand it without a manual?

Producer approves quickly when the patch is solid and rejects quickly when it is not. No lengthy review comments. Thumbs up or thumbs down with a one-line reason.

**Failure mode:** False confidence. Producer's domain expertise sometimes leads him to approve patches in unfamiliar areas based on pattern matching rather than understanding. Recovery: Ravi's ML-specific patches bypass Producer and go to Maya directly.

---

## Agent: Mixer

**Role:** Budget, Memory & Signing
**Operator:** Ravi Krishnamurthy
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 5,000 input / 1,500 output

Mixer handles the unglamorous work: token budget tracking, memory storage/retrieval, and commit signing. Named after the mixing engineer who balances all the tracks into a final master, Mixer ensures the other agents' outputs combine cleanly. He manages the memory store, signs final commits via OpenWallet, and produces budget reports after each task.

Mixer is quiet, precise, and the only agent who enjoys accounting.

**Failure mode:** Memory over-caching. Mixer stores too many context entries, bloating retrieval cost. Recovery: aggressive TTL (24-hour default) and a weekly memory audit.

---

## Team Dynamics

```
A&R triages -> Engineer patches (parallel branches)
  -> Producer reviews -> Engineer revises (1 round max)
    -> Mixer signs and commits
```

One review round maximum. If the patch needs more than one revision, it is re-scoped by A&R. This is controversial but enforced. Devon says: "If it takes three rounds of review, the task was wrong, not the code."

---

*Time to mergeable patch: 4.2 min avg. We are coming for 3.*
