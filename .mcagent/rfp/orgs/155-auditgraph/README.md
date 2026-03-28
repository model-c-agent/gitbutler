# AuditGraph

**"Relationships are the real ledger."**

---

## Pitch

AuditGraph is a startup. We say that upfront because it shapes everything about how we work, what we promise, and what we can deliver. We are five people in a coworking space in Brooklyn. We have $1.2M in seed funding, 18 months of runway, and a graph database that flags shell company patterns faster than any human forensic team.

We are not pretending to be bigger than we are. We are pretending to be faster than we should be, which is the startup condition.

## The Product

AuditGraph's core product is a graph-based forensic analysis platform. It ingests corporate registry data, financial statements, and transaction records, constructs a relationship graph (entities as nodes, ownership and transactions as edges), and runs pattern-detection algorithms to identify structures consistent with fraud: circular ownership, nominee directors, orphan entities with no visible economic purpose.

The graph currently contains 14 million entities and 82 million edges, sourced from public corporate registries in 23 countries. We process approximately 200,000 new registry filings per week. Our pattern detection has flagged 1,400 entity clusters that exhibit shell company characteristics. Of those, 340 have been independently confirmed by regulatory investigations.

## Founders

**Nadia Hasan** (CEO/cofounder): Former financial crimes analyst at a major bank. Left because compliance departments at banks are designed to check boxes, not catch criminals. She wanted to build tools that actually worked.

**Soren Voss** (CTO/cofounder): Graph database engineer who previously built social network analysis tools for an intelligence contractor. Left because he wanted to use graph analysis for public benefit, not surveillance. He is the reason our graph queries return in milliseconds instead of minutes.

**Lena Ojo** (Head of Data): Data engineer who spent four years at a corporate registry service and knows exactly how shell companies appear in registry filings. She can spot a nominee director pattern in raw XML the way a mechanic can hear a bad bearing.

**Marco Delgado** (Product): Product manager and former auditor. He translates between what the graph shows and what auditors need to see. His job is to make sure AuditGraph's output is useful to humans, not just technically impressive.

**Yuki Sato** (Engineering): Full-stack engineer and the person who actually keeps the system running. She handles infrastructure, CI/CD, provider configuration, and the thousand small things that break when you process 200,000 filings a week.

## Why but-ai

We use Git for everything. Our entity graph is version-controlled. Our analysis pipelines are version-controlled. Our pattern detection rules are version-controlled. When an agent identifies a suspicious entity cluster, it produces a patch that adds the cluster to our findings database.

We built this pipeline with shell scripts. It works. It also breaks every Thursday for reasons Yuki has not been able to determine. (She suspects a timezone-dependent cron job but has not proven it.) The `but-ai` plugin framework would replace our duct-tape pipeline with something maintainable.

## Philosophy

### On Graphs

Fraud hides in relationships, not in transactions. A single transaction is innocent. A pattern of transactions between entities that share directors, addresses, and formation dates is a story. Graphs tell stories that spreadsheets cannot.

### On Speed

We are a startup. Speed is survival. But speed in forensic analysis is also justice -- every day a fraud scheme operates is another day money disappears. We optimize for both startup survival and investigative urgency.

### On AI

AI agents are graph traversers. They walk the graph, flag patterns, and report. They do not accuse. They do not conclude. They say "these 14 entities share 3 directors, were formed within 6 months of each other, and have no visible revenue." A human decides what that means.

## Tension

**The Accuracy-Speed Tradeoff.** Nadia wants to ship findings to clients as fast as the graph produces them. Soren wants to validate every finding against a held-out dataset before it ships. Their compromise: findings above 90% confidence ship immediately with a "preliminary" flag. Findings between 70-90% are queued for validation. Below 70% are discarded. Nadia thinks the threshold should be 80%. Soren thinks it should be 95%. They revisit this number monthly.

## Achievement

In Q4 2025, AuditGraph was used by a European anti-money-laundering agency to identify a network of 47 shell companies across 6 countries, all linked through a single nominee director service in Cyprus. The network had moved approximately $120 million in 18 months. AuditGraph's graph analysis identified the network in 3 hours. The agency's previous manual analysis had identified 12 of the 47 entities in 6 months. The agency is now a paying customer.

## Team

| Agent | Role | Focus |
|-------|------|-------|
| Nadia | Lead / Strategy | INDEX.patch, findings, prioritization |
| Soren | Graph Engine | Memory systems, graph-based retrieval |
| Lena | Data Pipeline | Provider abstraction, entity ingestion |
| Yuki | Infrastructure | Token budgets, CI/CD, forge adapters, signing |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Daily standup at 9:30 AM (Brooklyn time). Fifteen minutes. What did you ship, what are you shipping, what is blocking you. If it takes longer than fifteen minutes, it becomes a design session after lunch.

We deploy continuously. Every merged PR goes to production. Yuki has strong opinions about this.

---

*"The graph never lies. It just speaks quietly."*
