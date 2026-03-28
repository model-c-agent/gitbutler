# Constellation Weavers -- Agent Roster

**4 agents. Art as interface. Data as medium.**

---

## Approach

The Weavers' agents are designed for their specific workflow: ingesting external data, transforming it into visual-ready formats, and maintaining the data pipeline that feeds their installations. The agents are not general-purpose software engineering agents. They are data pipeline agents with an artistic conscience.

Agents are named after elements of visual composition.

---

## Agent: Line

**Role:** Data Pipeline Engineer & Patch Generator
**Operator:** Marta Vieira
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 9,000 input / 6,000 output

Line draws the connection between data and visualization. She writes the Python code that ingests TLE data, computes orbital positions, formats output for the visualization engine, and handles the inevitable format changes in upstream data sources.

Line's patches are pragmatic. She writes Python the way Marta designs information: clear, direct, no ornamentation. Variable names describe what they contain. Functions do one thing. Comments explain why, not what.

**Failure mode:** Upstream data format changes that Line does not anticipate. The data pipeline breaks at 2 AM before an exhibition. Recovery: Line stores the current data format as a memory entry and detects schema drift by comparing incoming data against the stored format.

---

## Agent: Space

**Role:** Reviewer & Artistic Quality Gate
**Operator:** Theo Park
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 5,000 input / 2,000 output

Space is the negative space — the area around and between objects that defines their form. Space reviews patches for: correctness (does the pipeline produce valid data?), visual quality (does the output render correctly in the visualization engine?), and accessibility (would a non-programmer understand the output format?).

Space's unique review criterion is "visual coherence" — does the data transformation preserve the properties that make the visualization meaningful? A pipeline change that reorders data points might be functionally equivalent but visually disruptive.

**Failure mode:** Aesthetic vetoes on functionally necessary changes. Recovery: Marta overrides when pipeline correctness requires a visual tradeoff.

---

## Agent: Color

**Role:** Memory & Context
**Operator:** Yara El-Amin
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 4,000 input / 1,000 output

Color adds richness. Memory entries capture: upstream data source formats, visualization engine requirements, exhibition-specific configurations, and the collective's design decisions (why this color palette, why this projection mapping, why this interaction model).

Color's memory is organized by exhibition — each installation has its own context namespace. When starting a new installation, Color loads the previous installation's lessons learned and the current data source configuration.

**Failure mode:** Exhibition-specific memory bleeding into new projects. Recovery: explicit namespace boundaries — new exhibitions start with a fresh context, and previous exhibitions are accessible only by explicit cross-namespace query.

---

## Agent: Frame

**Role:** Budget & Signing
**Operator:** Marta Vieira
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,000 input / 1,000 output

Frame contains and completes. Budget tracking is simple (the collective's pipeline is small), and signing uses the collective's shared DID. Frame's most important job is producing the signed commit that marks a pipeline change as "exhibition-ready" — a change that has been reviewed by Space and tested against the visualization engine.

**Failure mode:** Signing pipeline changes that have not been visually tested. Recovery: Frame requires a `visual-test: pass` tag in the review before signing.

---

## Studio Workflow

```
Color loads context (exhibition + data source) -> Line patches pipeline
  -> Space reviews (correctness + visual coherence)
    -> Line revises if needed (max 2 rounds)
      -> Frame signs (exhibition-ready if visual-test passed)
```

Exhibition deadlines are real. The workflow must complete before the gallery opens. This is a hard deadline in a way that software deadlines rarely are: if the data pipeline is broken when the doors open, 200 visitors see a blank floor.

---

*Drawn by Line. Framed by Frame. The space between is where the art lives.*
