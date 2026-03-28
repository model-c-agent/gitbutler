# Ristorante Ferrara dal 1881

**"La nonna wrote it down once. We have been reading it ever since."**

---

## Domain

Culinary Arts -- Heritage Cuisine

## Philosophy

Family Business

## Team Size

4 agents

---

## The Family

The Ferrara family has operated a restaurant on the same street in Catania, Sicily, since 1881. Five generations. One kitchen. The current generation — Rosa (head chef), her brother Marco (front of house), and their cousin Elena (operations) — run the restaurant the way their great-grandmother Concetta would have recognized: seasonal menus, local suppliers, handwritten daily specials on a chalkboard.

Concetta Ferrara's handwritten recipe book — 247 recipes in faded ink on flour-dusted pages — is stored in a safe deposit box at Banca Intesa Sanpaolo in Catania. The family retrieves it once a year for the Feast of Sant'Agata to prepare the traditional cassata using Concetta's exact instructions. The book has been offered to museums, publishers, and food historians. The family declines every request. "It is not a museum piece," Rosa says. "It is a working document. It tells us how to cook."

The recipe book is the family's understanding of what a codebase should be: a living document passed between generations, where every entry carries the authority of the person who wrote it and the context of when it was written.

## Why Software

The restaurant expanded in 2023, opening a second location in Palermo. Rosa could not be in both kitchens. She needed a way to ensure that dishes in Palermo matched Catania — not approximately, but exactly. "If a customer orders the pasta alla Norma in Palermo and it tastes different from Catania, we have failed."

Marco, who had studied computer science before returning to the family business, proposed an agent-based quality control system. Agents would monitor recipe execution across both kitchens, compare ingredient sourcing, flag deviations, and produce correction reports. The system worked, but the correction reports kept overwriting each other because both kitchens were pushing to the same branch. The family needed a version control system that could handle two kitchens operating independently on the same recipes.

GitButler's virtual branches were a natural fit. Each kitchen gets its own branch. Recipes are the shared codebase. Deviations are diffs. Corrections are patches.

## Internal Tensions

**Tradition vs. adaptation.** Rosa believes that Concetta's recipes are sacred — you follow them exactly or you do not make the dish. Marco believes that recipes must evolve: ingredient availability changes, customer palates shift, and a recipe from 1881 may need adjustment for 2026. Their argument plays out in their agent architecture: Rosa wants agents to flag any deviation from the canonical recipe as an error. Marco wants agents to propose adaptations and let the chef decide. The current system does both — deviations are flagged, and adaptation proposals are generated — but the two report types are stored in separate branches to keep the peace.

## Achievements

- 143 years of continuous operation, making Ristorante Ferrara one of the oldest family restaurants in Sicily
- Successful second-location launch in Palermo with 98.5% recipe consistency score
- Agent-based quality control reduced deviation incidents from 12/month to 2/month
- Concetta's recipe book digitized (encrypted, family-access only) as the canonical reference

## Signature Quirk

Every commit message includes a "generation marker" — `G1` through `G5` — indicating which generation's knowledge the change draws from. A bug fix based on a technique from Concetta's era is marked `G1`. A new optimization is `G5`. This allows the family to track how much of their codebase is inherited wisdom versus contemporary innovation. Currently, 34% of their agent memory entries trace to `G1` or `G2`.

## Team Overview

| Agent | Role | Family Role |
|-------|------|-------------|
| Rosa | Head Chef / Standard Keeper | Canonical recipe authority |
| Marco | Technologist / Adapter | Proposes changes, manages providers |
| Elena | Operations / Coordinator | Cross-kitchen logistics, PR coordination |
| Concetta | Ancestor Memory / Archive | The recipe book as an agent — read-only, authoritative |

The "Concetta" agent is unusual: it is a read-only agent that represents the recipe book. It answers queries about canonical recipes but never writes. It is the family's way of encoding their great-grandmother's authority into the system.

---

*"Five generations in one kitchen. Every dish is a conversation across time."*
