# Nakamura Sound House -- Agent Roster

**3 agents. Family structure. Grandmother has final authority.**

---

## Structure

The agent team mirrors the family: one builder, one documenter, one quality authority. The agents are small because the family is small. There is no coordination overhead because there are only three people and they live in the same building.

Agents are named after workshop tools.

---

## Agent: Chisel

**Role:** Patch Generator & Implementer
**Operator:** Yuki Nakamura
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 10,000 input / 6,000 output

Chisel builds. Named for the tool Kenji uses to shape console chassis, Chisel produces patches with the precision Yuki has learned from watching his father work. Every change is scoped tightly -- Chisel modifies only what the task requires, tests the change in isolation, and produces a clean diff.

Chisel reads more context than most patch generators because Yuki's experience documenting Kenji's process taught him that understanding the surrounding system is essential to making changes that fit. A resistor value only makes sense in the context of its circuit.

**Failure mode:** Over-documentation. Chisel's patches sometimes include excessive comments explaining why a change was made. Recovery: Caliper flags unnecessary comments during review.

---

## Agent: Caliper

**Role:** Reviewer & Quality Gate
**Operator:** Aiko Sato (with Fumiko's criteria)
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,000 input / 2,000 output

Caliper measures. Named for the precision instrument Kenji uses to verify component dimensions, Caliper reviews every patch against three criteria: does it work, does it match the existing code style, and would Fumiko approve?

The third criterion is subjective but important. Fumiko's standard is: "If I cannot understand what this does by reading it, it is wrong." Caliper applies this by checking that every function has a clear purpose, every variable has a descriptive name, and no abstraction exists without a concrete use case.

**Failure mode:** Applying Fumiko's standard too literally to code that Fumiko will never read. Recovery: Chisel pushes back on reviews that prioritize readability over functionality in performance-critical paths.

---

## Agent: Jig

**Role:** Memory, Budget & Signing
**Operator:** Yuki Nakamura
**Tools:** `GetProjectStatus`, `GetCommitDetails`
**Token Budget:** 4,000 input / 1,000 output

Jig holds things in place. Named for the assembly jigs Kenji uses to align components during soldering, Jig manages memory, tracks token budgets, and handles commit signing. Jig is the simplest agent because in a three-person team, complexity must live in the builders and reviewers, not in the infrastructure.

Memory entries are tagged with the component or module they relate to, mirroring the family's practice of organizing build specifications by console subsystem.

**Failure mode:** Under-investing in memory. Jig's minimal budget means memory entries are sparse. Recovery: Chisel can request that Jig store specific entries by flagging them during patch generation.

---

## Family Dynamics

```
Chisel builds -> Caliper measures -> Chisel adjusts (max 2 rounds)
  -> Jig signs and stores
```

Three agents, sequential flow, no parallel execution. The family builds one console at a time. The agents work one task at a time. This is intentional -- parallel work in a small team creates coordination costs that exceed the time saved.

Fumiko does not have an agent. She has veto power. If a production deployment fails her test -- which, in the family's context, means "if Yuki shows her the result and she frowns" -- the work is redone from scratch. This has happened twice.

---

*Hand-built. Measured twice. Cut once.*
