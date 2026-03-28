# Scent Workers' Council

**"No trade secrets. No master perfumers. Only shared formulas and collective noses."**

---

## Formation

The Scent Workers' Council was founded in 2020 in Grasse, France — the historical capital of perfumery — by six perfumers who were fired, bought out, or quietly pushed from major fragrance houses for the same reason: they wanted to publish their formulas.

In commercial perfumery, formulas are proprietary. A perfumer who creates a fragrance for a major brand does not own the formula — the brand does. The formula is locked in a vault (literally — IFF, Givaudan, and Firmenich all maintain physical formula vaults). The perfumer cannot publish it, teach from it, or use it at another company. This system produces beautiful fragrances and miserable perfumers.

The founding six — all trained at ISIPCA, the perfumery school in Versailles — decided to build an alternative. They formed a cooperative under French law (SCOP), pooled their savings, rented a lab in Grasse, and started making perfume with one rule: every formula is published. Every ingredient, every proportion, every process step. Published on their website, licensed under Creative Commons BY-SA.

The industry called them insane. Two years later, they have 23 members, a catalog of 140 published formulas, and a small but devoted customer base of indie perfumers who use their open formulas as starting points for their own work.

## Philosophy

Perfumery is a craft, not a secret. The mystification of fragrance creation — the "nose" as genius, the formula as trade secret, the brand as arbiter of taste — serves capital, not creativity. When formulas are open, perfumers learn faster, iterate more freely, and produce more diverse work. The Council believes that open-source perfumery will produce better fragrances than closed perfumery, for the same reason that open-source software produces better tools: more contributors, faster iteration, broader perspective.

## Why This RFP

In 2025, the Council began using AI agents to accelerate formula iteration. The traditional perfumery workflow is: compose a trial formula, compound it (mix the ingredients), evaluate it (smell it, wait for dry-down, smell again), and adjust. This cycle takes days because physical compounding and evaluation cannot be skipped.

The agents accelerated the composition phase. Given a brief ("warm, woody, with a citrus opening and a musky base"), an agent generates candidate formulas based on the Council's open formula database. Multiple agents generate different interpretations of the same brief — one agent might emphasize sandalwood, another vetiver, another oud. The resulting formulas are independent proposals that need version control.

The Council discovered GitButler when a member searched for "version control for structured data that is not code." The virtual branch model mapped perfectly onto their workflow: multiple independent formula proposals, maintained in parallel, evaluated through physical testing, with the best merged into the catalog.

## Team

Five members. Elected by the Council's annual assembly. The delegation rotates annually.

| Agent | Role | Specialty |
|-------|------|-----------|
| Inès Dufour | Patch Architect / Formula Generation | Top notes, citrus accords |
| Rashid Benali | Memory Architect | Ingredient database, olfactory profiles |
| Yelena Sorokina | Forge Adapter / Coordination | Supplier relations, cross-lab coordination |
| Kofi Mensah | Provider & Budget | Cost optimization, raw material pricing |
| Camille Laurent | Security & Signing | Formula provenance, CC licensing, signing |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Purity Debate.** Rashid insists that the Council's formulas should use only natural ingredients — essential oils, absolutes, tinctures. Inès argues that synthetic aroma chemicals are essential for modern perfumery and that excluding them is a form of elitism (naturals are expensive; synthetics make perfumery accessible). The Council voted 14-9 in favor of allowing synthetics, but Rashid publishes a "naturals-only" variant of every formula he contributes to, which Inès considers passive-aggressive and Rashid considers principled.

## Achievement

In 2024, the Council's formula for "Terre Ouverte" (Open Earth) — a vetiver-and-fig composition published under CC BY-SA — was independently compounded by 47 perfumers in 12 countries, each producing their own variation. The collective body of work was exhibited at the Musee International de la Parfumerie in Grasse. It was the first museum exhibition of open-source perfumery. The curator called it "a radical act of generosity." Inès cried during the opening. Rashid pointed out that seven of the variations used synthetic vetiver and left the reception early.

---

*"The formula is free. The nose is yours."*
— Council charter, Article 1
