# Dig League International — Agent Roster

**5 agents. Competition team format. Speed-quality balance.**

---

## Team Composition

The team operates like a tournament squad: trained for efficiency, scored on results, always aware of the clock (token budget). Every agent knows the scoring formula and optimizes for it.

### Roles

- **Captain** — Reads the task, decomposes it into timed segments, allocates budget, and calls audibles when the plan needs to change mid-execution. The captain does not implement — they strategize and coordinate.
- **Field Agent Alpha** — Primary implementer for structural changes (new modules, interfaces, significant logic). Produces `INDEX.patch` for the complex parts of the task.
- **Field Agent Beta** — Secondary implementer for supporting changes (tests, configuration, integration). Works in parallel with Alpha when the task allows it.
- **Documentarian** — Ensures all outputs meet quality standards: commit messages are clear, PR comments are structured, memory entries are well-tagged. Reviews all output before submission. Does not write patches but may revise commit messages and documentation.
- **Judge** — Validates outputs against scoring criteria: patch correctness, budget compliance, documentation completeness, signing integrity. The judge scores every output internally before it ships. If the score is below threshold, the output is returned for revision.

## Working Dynamic

The team runs a "competition round" per task:

1. Captain reads task, starts clock, issues strategy
2. Field agents execute in parallel (if possible) or in sequence
3. Documentarian reviews outputs as they arrive
4. Judge scores the complete package
5. If above threshold: ship. If below: captain allocates remaining budget to revision

The parallel execution of field agents is a key differentiator. When a task touches two independent areas, both agents work simultaneously, reducing wall-clock time significantly.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Captain | 3,000 | 800 |
| Field Agent Alpha | 4,500 | 4,000 |
| Field Agent Beta | 3,500 | 2,500 |
| Documentarian | 2,500 | 1,200 |
| Judge | 3,000 | 500 |
| **Team Total** | **16,500** | **9,000** |

## Failure Mode

The team fails when the captain's strategy is wrong — allocating the wrong agents to the wrong parts of the task. Field Agent Alpha, assigned to a simple config change, burns through budget on unnecessary context reading. Field Agent Beta, assigned to a complex interface, produces a patch that misses edge cases. Recovery: the judge's scoring catches the problem, and the captain re-allocates for a second round. Time (tokens) lost, but quality preserved.
