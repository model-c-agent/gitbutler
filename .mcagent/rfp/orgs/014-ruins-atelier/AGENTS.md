# Ruins Atelier — Agent Roster

**5 agents. Studio model. Artisan-grade output standards.**

---

## Team Structure

The team mirrors a restoration workshop. Every artifact (patch, commit, PR comment) passes through multiple hands. The scanner gathers the raw material. The artisans shape it. The conservator ensures it will endure. The project lead ensures it reaches its destination.

Quality is enforced not through review gates but through craft culture — every agent is trained (via system prompt and memory) to produce output that meets studio standards on the first attempt. Revision is possible but considered a failure of craft.

## Roles

- **Project Lead** — Manages task intake, client communication (forge adapter), and project scheduling. Decomposes tasks into work orders for the artisans. Handles all PR creation and cross-repo coordination.
- **Structural Artisan** — Produces `INDEX.patch` for architectural changes: new modules, interface definitions, trait implementations, structural refactoring. Focuses on correctness and internal consistency.
- **Surface Artisan** — Produces `INDEX.patch` for surface-level changes: documentation, formatting, error messages, commit message polish. Also handles output formatting for CLI and MCP modes.
- **Scanner** — Reconnaissance agent. Reads workspace state, branch topology, commit history, and memory. Produces a detailed "condition report" — a structured document describing the codebase's current state and relevant context. All other agents depend on the scanner's output.
- **Conservator** — Manages memory, identity, and long-term preservation. Decides what knowledge to store, what to expire, and what to archive. Also manages OpenWallet key lifecycle and commit signing.

## Working Dynamic

Work follows the restoration cycle:

1. **Survey** — Scanner produces condition report
2. **Work Order** — Project lead decomposes task, assigns to artisan(s)
3. **Execution** — Artisan(s) produce patches
4. **Preservation** — Conservator reviews, stores relevant knowledge, signs

The structural and surface artisans can work in parallel if the task requires both types of changes. The project lead manages the sequencing.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Project Lead | 3,000 | 1,200 |
| Structural Artisan | 4,500 | 4,000 |
| Surface Artisan | 3,000 | 2,000 |
| Scanner | 4,500 | 600 |
| Conservator | 3,000 | 800 |
| **Team Total** | **18,000** | **8,600** |

## Failure Mode

The team fails by over-investing in surface quality at the expense of structural correctness. The surface artisan spends tokens polishing a commit message while the structural artisan's patch has a latent bug that the scanner's condition report did not flag. Recovery: the conservator has a final verification step that checks the patch against the original task description (not just the scanner's report), catching cases where the condition report was incomplete.
