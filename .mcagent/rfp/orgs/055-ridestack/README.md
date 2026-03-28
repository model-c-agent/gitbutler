# RideStack

**"One ticket. Every mode. Zero friction."**

---

## Origin

RideStack was founded in 2023 by Nadia Kovacs and Jian Li, two mobility engineers who met at a MaaS (Mobility-as-a-Service) hackathon in Helsinki and bonded over a shared frustration: every city has buses, trains, scooters, bikes, rideshares, and ferries, each with its own app, its own payment system, and its own data format. A commuter who takes a bus to a train to a scooter needs three apps and three payment methods. The friction is absurd.

Nadia had spent four years at a European MaaS startup that raised forty million euros and produced an app that aggregated two transit modes in one city. Jian had built the trip planning engine for a Chinese ride-hailing company and watched it fail to integrate with public transit because the data formats were incompatible. They both concluded independently that the problem was not UX — it was data plumbing. You cannot build a unified mobility experience on fragmented data infrastructure.

RideStack is a data-first MaaS platform. They do not build the passenger-facing app — they build the integration layer that lets anyone build one. Their product is an API that normalizes GTFS, GBFS, TOMP, and proprietary transit feeds into a single schema, routes across modes, and settles payments through a unified clearing system. Three cities are live. Series A closed in January 2026.

## Philosophy

Move fast, integrate everything. RideStack is a classic startup: small team, aggressive roadmap, bias toward shipping. They believe the mobility integration problem is a data problem, and data problems are solved by writing code, not by writing standards documents. When a new transit feed does not match their schema, they write an adapter. When a payment system does not support their clearing protocol, they build a bridge. Speed is their moat.

They approach AI agents the same way: agents should ship working code quickly, iterate based on review feedback, and never let perfect be the enemy of deployed. A patch that is 90% correct and ships today is better than a patch that is 100% correct and ships next week.

## Internal Tension

Nadia is the architect — she wants clean abstractions and extensible interfaces. Jian is the pragmatist — he wants to ship features that close enterprise deals. When Nadia spends a week refactoring the provider abstraction layer for elegance, Jian points out that no customer has ever asked for elegant abstractions. When Jian ships a quick-and-dirty adapter for a new transit feed, Nadia points out that they will pay for the tech debt in six months. They are both right. The codebase reflects the tension: some modules are beautifully abstracted, others are held together with TODO comments and optimism.

## Achievement

In November 2025, RideStack integrated seven transit modes in Gothenburg (bus, tram, ferry, e-scooter, bike-share, commuter rail, and on-demand shuttle) into a single trip-planning API in 11 days. The previous record for a multi-modal integration of that scope was 8 weeks. Jian attributes the speed to their adapter architecture. Nadia attributes it to the three all-nighters she pulled refactoring the routing engine the week before. The customer does not care who is right; the product works.

## Team

| Name | Role | Background |
|------|------|------------|
| Nadia Kovacs | CTO / Architecture | Ex-MaaS startup (Helsinki), routing engines |
| Jian Li | CEO / Product | Ex-Didi, trip planning, enterprise sales |
| Priya Rajan | Backend Engineer | API design, ex-Stripe payments |
| Olu Adeyemi | Data Engineer | GTFS/GBFS normalization, ex-Transit App |
| Kasper Berg | DevOps / Infra | Kubernetes, CI/CD, ex-Spotify platform |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Commit messages include the number of transit modes currently integrated, because it is the metric Jian tracks obsessively. Example: `feat: add TOMP adapter for Helsinki shuttles [modes: 14]`.

---

*"Fourteen modes. One API. Ship it."*
