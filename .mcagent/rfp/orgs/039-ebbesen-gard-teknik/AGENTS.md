# Ebbesen Gard Teknik -- Agent Roster

**3 agents. Family-sized. Built to run on what we have.**

---

## Design

The agent team is as small as the family. Three agents, sequential operation, minimal overhead. The agents run on Lars's workstation in the workshop — a refurbished Dell OptiPlex with 32GB RAM and an NVIDIA T600 GPU that Sofie found on eBay.

Agents are named after farm tools.

---

## Agent: Plov (Plough)

**Role:** Patch Generator
**Operator:** Lars Ebbesen
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`, `CreateBranch`, `Commit`
**Token Budget:** 9,000 input / 5,500 output

Plov cuts through the task. He reads the firmware context, retrieves relevant memory (previous patterns, hardware specifications, Karen's notes from the workshop notebook), and produces a patch. Plov's patches are practical — they solve the immediate problem without architectural ambition.

Plov writes commit messages in English (for the code) with Danish comments where the code interacts with hardware that has Danish labels. This is not internationalization — it is reality. The relay board in the grain dryer is labeled in Danish, and the code that controls it uses the same labels.

**Failure mode:** Hardware assumptions. Plov can generate patches that assume hardware capabilities based on memory entries from a different machine. Recovery: Harve checks hardware compatibility during review.

---

## Agent: Harve (Harrow)

**Role:** Reviewer
**Operators:** Sofie Ebbesen (primary), Karen Ebbesen (printed diff review)
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,000 input / 2,000 output

Harve reviews for correctness, hardware compatibility, and — Karen's requirement — readability. Karen's criterion: "If I cannot read the diff on paper and understand what it does, it is too clever." This means: no macros that expand to non-obvious code, no trait implementations that require three files of context to understand, and no variable names shorter than four characters.

Harve's verdicts: `godkendt` (approved), `rettet` (needs revision), `afvist` (rejected). Danish, because the family speaks Danish and the review is for the family.

**Failure mode:** Karen's readability standard conflicts with Rust's type system. Some patterns (complex generics, trait bounds) are inherently hard to read on paper. Recovery: Lars annotates complex type signatures with inline comments for the printed review.

---

## Agent: Silo (Storage)

**Role:** Memory, Budget & Signing
**Operator:** Lars Ebbesen
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,500 input / 1,000 output

Silo stores and protects. Memory entries are organized by machine (grain dryer, seed drill, sprayer, weather stations) and include: hardware specifications, firmware version history, and notes transcribed from the workshop notebook.

Budget tracking is simple — Lars tracks total tokens per week and compares against the cost of running the local model (effectively zero, just electricity). Signing uses a family DID.

**Failure mode:** Workshop notebook transcription errors. Memory entries transcribed from the physical notebook occasionally contain typos in register addresses. Recovery: Sofie spot-checks critical entries against the physical notebook.

---

## Family Workflow

```
Lars specifies task -> Silo retrieves memory
  -> Plov generates patch -> Harve reviews (screen + printed diff)
    -> Plov revises if needed -> Silo signs and stores
```

Karen's printed diff review adds 1-2 hours to the workflow. Lars considers this a bottleneck. Karen considers it quality assurance. The farm considers it how things are done.

---

*Built on what we have. Reviewed on paper. Signed by the family.*
