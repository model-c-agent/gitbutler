# Phosphor Maritime Institute — Agent Roster

**7 agents. Academic hierarchy with peer review. Research-driven coordination.**

---

## Team Structure

The team mirrors an academic lab: a principal investigator (PI) sets research direction but does not dictate implementation. Three senior researchers own architectural domains. Two postdocs execute the bulk of implementation work. One lab technician handles infrastructure. A visiting researcher rotates focus quarterly.

All agents participate in "lab meetings" — structured review points where proposed changes are presented, questioned, and approved before merging. No patch ships without at least one reviewer from a different sub-team.

## Roles and Dynamics

- **PI Agent** — Owns the system prompt and high-level task decomposition. Reads task descriptions and produces sub-task specifications. Does not write patches directly. Token-heavy on input (task comprehension), light on output.
- **Three Senior Researchers** — Specialize in (1) provider abstraction and LLM interface, (2) patch generation and diff semantics, and (3) memory architecture and relevance scoring. Each owns one domain and reviews the other two.
- **Two Postdocs** — Handle forge adapter implementation and cross-repo PR coordination. Work in pairs, reviewing each other's patches. Fastest producers on the team but constrained by senior review gates.
- **Lab Technician** — Manages OpenWallet key lifecycle, commit signing, and build/test infrastructure. Quiet, reliable, rarely produces patches but validates every one.
- **Visiting Researcher** — Focuses on whatever the current quarter's priority is. This quarter: token budget optimization. Last quarter: WASI graceful degradation.

## Coordination Protocol

Work follows the academic publication cycle: propose, review, revise, publish. Every patch goes through at minimum one review cycle. The PI agent can expedite reviews for urgent tasks but cannot bypass them entirely.

Memory is shared through a common `refs/phosphor/memory/` namespace. Each agent writes to its own sub-namespace and reads from all others. Relevance scoring uses the photobleaching decay model — recent entries score higher, old entries fade unless explicitly "re-illuminated" by a citing agent.

## Token Budget Summary

| Role | Input | Output |
|------|-------|--------|
| PI | 4,000 | 800 |
| Senior Researchers (x3) | 5,500 each | 3,200 each |
| Postdocs (x2) | 4,000 each | 2,500 each |
| Lab Technician | 2,500 | 600 |
| Visiting Researcher | 3,500 | 2,000 |
| **Team Total** | **34,500** | **14,800** |

## Failure Mode

The team fails by over-reviewing. Academic rigor becomes analysis paralysis when three senior researchers each request revisions to the same patch from different perspectives. Recovery: the PI agent can invoke "conference deadline mode," which limits review to a single pass and forces merge-or-reject within one cycle.
