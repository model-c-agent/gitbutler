# InsureQL

**"Ask the risk in English. Get the price in milliseconds."**

---

## The Pitch

InsureQL is a seed-stage startup (Series A, $4.2M raised, 2025) building a natural language interface for insurance underwriting. The product lets underwriters type questions like "What is the 95th percentile loss for a commercial property portfolio in flood zone AE with construction type III?" and get an answer in under two seconds, backed by the insurer's own historical claims data.

Before InsureQL, that question took a senior underwriter 45 minutes and three spreadsheets. Or it was never asked at all, because the friction of querying actuarial databases discouraged exploratory analysis. Policies were priced on gut feel and precedent, not on the data sitting in the company's own systems.

## Founding

Three co-founders, a whiteboard, and a demo that crashed during the pitch to their first investor.

- **Karim El-Amin** (CEO, 34): Former underwriter at Lloyd's. Left because he was tired of watching junior underwriters make pricing errors that the data could have prevented, if only the data were accessible.
- **Sofia Reyes** (CTO, 30): NLP researcher from Stanford. Built the query parser. Once described her job as "teaching computers to speak insurance."
- **Jae Park** (CPO, 28): Product designer from Samsung. Designed the interface. Believes that if an underwriter needs training to use the product, the product has failed.

The company operates from a WeWork in Shoreditch, London. The team is twelve people. They move fast, ship weekly, and have the slightly manic energy of a startup that knows its runway is measured in months, not years.

## Why but-ai

InsureQL's product is AI-native. The `but-ai` plugin is not a side project -- it is directly relevant to their development workflow. Their codebase changes constantly (weekly releases), their models need version-controlled configuration, and their multi-repo architecture (query parser, pricing engine, data pipeline, frontend) requires cross-repo coordination.

They adopted GitButler in 2025 for virtual branches. The `but-ai` RFP is a chance to build internal tooling that also benefits the ecosystem.

## Philosophy

Speed wins. In underwriting, the faster you can price a risk, the more risks you can evaluate, and the better your portfolio. InsureQL applies this to development: the faster you can ship a patch, the faster you learn whether it was right.

This does not mean sloppy. It means efficient. Every unnecessary step in the development workflow is a tax on learning speed.

## Internal Tension

**The Accuracy-Speed Tradeoff.** Sofia wants the query parser to be maximally accurate -- she would rather return no result than a wrong result. Karim wants it to be maximally responsive -- he would rather return an approximate result with a confidence indicator than make the underwriter wait. The current product returns results with a confidence score, which is Sofia's compromise. She is not happy about it.

## Notable Achievement

In Q4 2025, InsureQL's product was licensed by six insurers. One of them reported a 60% reduction in time-to-quote for commercial property policies. The case study was published (with the insurer's permission) and brought in three more leads. The startup is not yet profitable but is, in Karim's words, "pointed at revenue."

## Team (Plugin Dev Squad)

| Agent | Role | Co-founder? |
|-------|------|-------------|
| Sofia | Patch Generation / NLP | Yes (CTO) |
| Karim | Review / Product | Yes (CEO) |
| Jae | Forge Coordination | Yes (CPO) |
| Lina | Memory / Retrieval | No (ML engineer) |
| Omar | Security & Signing | No (DevOps) |
| Devi | Provider & Budget | No (Backend) |

Details in [AGENTS.md](AGENTS.md).

---

*"The underwriter asks. The machine answers. The portfolio improves."*
