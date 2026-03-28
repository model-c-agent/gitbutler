# Actuarial Autonomy Network

**"No employer. No proprietary model. No permission required."**

---

## What We Are

The Actuarial Autonomy Network is a loose federation of 23 freelance actuaries who refuse to work for insurance companies directly. Instead, they build open-source actuarial tools and sell consulting services to consumer advocacy groups, mutual aid societies, and regulators who need independent pricing analysis.

Founded in 2020 during the pandemic, when six actuaries lost their jobs simultaneously and discovered they preferred unemployment to building pricing models designed to maximize policyholder extraction. They pooled their severance, rented a co-working space in Bristol, and started publishing open-source mortality tables that anyone could audit.

The insurance industry response was predictable: silence in public, legal threats in private. Two members received cease-and-desist letters from former employers claiming their open-source models contained proprietary methodologies. Both claims were dismissed -- you cannot copyright mathematics, and actuarial science is mathematics.

## Why Software Tools

Actuarial models are code. A mortality table is a dataset. A pricing algorithm is a function. The network realized early that their real product was not consulting -- it was the tooling. If advocacy groups had access to the same modeling software that insurers use, the information asymmetry that enables extractive pricing would collapse.

Their toolchain is entirely open source: R for statistical modeling, Python for data pipelines, Rust for production pricing engines. They adopted Git early. They adopted GitButler in 2025 when they needed to coordinate model development across twelve time zones without a central repository owner.

The `but-ai` plugin interests them because actuarial model development is repetitive in predictable ways: fitting distributions, validating against historical data, generating documentation. AI agents can handle the mechanical parts while the actuaries focus on the judgment calls that require domain expertise and ethical reasoning.

## Philosophy

Information asymmetry is the source of all extractive pricing. If the insurer knows more about your risk than you do, they can charge you more than your risk justifies. The network's mission is to eliminate that asymmetry by making actuarial tools freely available.

They apply the same principle to AI agents: an agent's reasoning must be auditable. If an agent makes a pricing recommendation, you must be able to trace every input, every calculation, and every assumption. Black-box AI in insurance is just a newer form of information asymmetry.

## Internal Tension

**The Revenue Problem.** The network runs on consulting fees and donations. This is precarious. Marta (founding member, based in Lisbon) argues they should license their premium tooling to insurance companies -- "take their money and use it against them." Ade (founding member, based in Lagos) considers this ideologically incoherent. The argument recurs quarterly. Revenue remains flat.

## Notable Achievement

In 2025, the network's open-source health insurance pricing model was adopted by the UK consumer advocacy group "Fair Cover" for their annual report on pricing disparities. The report identified systematic overcharging of policyholders in low-income postcodes. Three insurers adjusted their pricing within two months. The network's model was cited in a parliamentary committee hearing.

## Team Overview

| Agent | Role | Location |
|-------|------|----------|
| Marta | Lead / Patch Generation | Lisbon |
| Ade | Memory / Knowledge Base | Lagos |
| Soren | Provider Abstraction | Copenhagen |
| Yuki | Forge Coordination | Osaka |
| Ravi | Security / Signing | Mumbai |
| Chen | Budget / Token Mgmt | Toronto |

Details in [AGENTS.md](AGENTS.md).

---

*"Publish the tables. Let them audit."*
