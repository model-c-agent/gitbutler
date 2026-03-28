# Regatta Systems — Agent Roster

**5 agents. Racing crew model. Continuous adaptation.**

---

## Team Structure

The team operates like a yacht racing crew: each agent has a specific station but can see the whole course. Information flows freely — every agent has read access to every other agent's state. Decisions are made by the tactician but can be overridden by the helmsman in real time.

## Roles

- **Tactician** — Reads the task, retrieves memory, assesses workspace state, and produces the execution strategy. Unlike a traditional planner, the tactician updates the strategy continuously as tool calls return results. The strategy is a living document, not a fixed plan.
- **Navigator** — Manages context and memory. Responsible for knowing "where we are" in the workspace, what has changed recently, and what memory entries are relevant. Provides positional awareness to all other agents.
- **Trimmer** — The implementer. Produces `INDEX.patch` with maximum efficiency — clean diffs, no unnecessary hunks, tight line counts. The trimmer optimizes for the smallest correct patch, because smaller patches review faster and merge cleaner.
- **Bowman** — Handles the complex setup work: forge adapter calls, PR creation, cross-repo coordination, dependency management. Works ahead of the rest of the team, preparing the environment before the trimmer needs to act.
- **Helmsman** — Final authority. Reviews the trimmer's patch, checks it against the tactician's strategy, and signs the commit. The helmsman can override any agent's decision but does so sparingly — overrides are logged and reviewed post-task.

## Working Dynamic

The crew operates in rapid cycles. A typical task has 3-5 adjustment cycles where the tactician updates the strategy based on new information. The team is designed for speed: the bowman works ahead, the navigator maintains continuous situational awareness, and the trimmer produces output as soon as the strategy stabilizes.

Unlike most teams, Regatta agents tolerate ambiguity. They start executing before the full plan is clear, adjusting as they go. This makes them fast but occasionally requires a mid-course correction that wastes tokens.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| Tactician | 4,500 | 1,500 |
| Navigator | 3,500 | 500 |
| Trimmer | 3,500 | 4,000 |
| Bowman | 3,000 | 1,500 |
| Helmsman | 3,000 | 600 |
| **Team Total** | **17,500** | **8,100** |

## Failure Mode

The team fails by over-adapting. When conditions change frequently (e.g., a task with many tool calls returning unexpected results), the tactician may update the strategy so often that the trimmer never gets a stable target to implement against. The result: a patch that addresses three different interpretations of the task without fully satisfying any of them.

Recovery: the helmsman can invoke "steady as she goes" — freezing the strategy and forcing the trimmer to implement against the current version, even if it's imperfect. The team can always submit a follow-up patch.
