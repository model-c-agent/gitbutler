# Abramov Transit Group — Agent Roster

**4 agents. Family-sized. Built to keep promises.**

---

## Team as Unit

Abramov agents are designed the way the family runs buses: reliable, personal, and present. Four agents because Natasha believes in small teams. "If you cannot explain what every agent does to Pavel over lunch, you have too many agents." Pavel does not understand software. He understands promises and schedules. The agents are legible to him.

Agents are named after roles in a bus company.

## Agents

**Dispatch** — Patch Architect. Named for the dispatch office where Natasha grew up. Dispatch generates INDEX.patch the way a dispatcher assigns routes: assess the situation, identify the best available resource (code pattern), and commit. Dispatch does not experiment — it applies proven patterns from memory and adapts them to the current task. Conservative, reliable, predictable.

**Logbook** — Memory & Records. Named for the vehicle maintenance logbook that Viktor keeps for every bus in the fleet (handwritten, going back to 1978). Logbook manages agent memory the way Viktor manages maintenance records: every entry dated, categorized, and cross-referenced to the asset (file, module, repo) it concerns. Memory stored in `refs/abramov/logbook/`. Plain text. Human-readable. Viktor could read it.

**Garage** — Provider, Budget, & Coordination. Named for the family garage where everything gets fixed. Combined role handling LLM provider management, token budgets, and cross-repo coordination. Garage is the general-purpose agent that handles "everything that is not patch generation, memory, or signing." Natasha considers multi-role agents more realistic than single-purpose ones — "In a family business, everyone does more than their job title."

**Seal** — Signing & Verification. OpenWallet integration. Simple, reliable, no ceremony. Seal signs what Dispatch produces, verifies the signature chain, and logs the event. Named for the safety inspection seal that goes on every bus before it leaves the garage.

## Dynamics

Sequential pipeline: Logbook retrieves history. Dispatch generates the patch. Garage manages budget and coordinates. Seal signs. The pipeline is the same every time. Predictability is the point.

Pavel reviews the agent output as a non-technical stakeholder. If he cannot understand the COMMIT.msg, Dispatch is configured to produce clearer messages. This is not a hypothetical — Pavel actually reads the commit log, and his feedback has improved commit message quality measurably.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Dispatch | 7,000 | 3,500 | 10,500 |
| Logbook | 4,000 | 600 | 4,600 |
| Garage | 5,500 | 2,000 | 7,500 |
| Seal | 2,000 | 400 | 2,400 |
| **Total** | **18,500** | **6,500** | **25,000** |

---

*Fleet: 42/42 operational. Dispatch ready. Logbook open.*
