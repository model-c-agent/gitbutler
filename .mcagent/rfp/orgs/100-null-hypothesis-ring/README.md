# Null Hypothesis Ring

**"They said the pricing was fair. We tested that claim. It wasn't."**

---

## What We Do in the Dark

The Null Hypothesis Ring is an anonymous collective of actuaries, data scientists, and reverse engineers who expose discriminatory pricing algorithms in the insurance industry. We have no office, no website, and no public-facing members. Communication happens on an encrypted Matrix server. Members know each other by handles only.

Since 2022, the Ring has published seven reports documenting algorithmic pricing discrimination. Each report reverse-engineers a specific insurer's pricing model from publicly available rate filings, identifies variables that serve as proxies for protected characteristics (race, ethnicity, disability), and publishes the analysis with enough detail for regulators to act.

Five of the seven reports resulted in regulatory investigations. Two resulted in consent decrees requiring insurers to modify their pricing models. The Ring has been called "digital vigilantes" by the insurance trade press and "essential public service" by consumer advocacy groups. Both descriptions are accurate.

## Why We Need Tools

Reverse-engineering pricing algorithms is labor-intensive. A single report requires months of data collection, model fitting, and analysis. The Ring's bottleneck is not skill -- it is time. Members contribute what hours they can, between their day jobs (most work as actuaries at the very companies they investigate, which adds a layer of risk the Ring does not discuss publicly).

AI agents can accelerate the mechanical parts: data cleaning, initial model fitting, report formatting. The Ring needs these agents to be verifiably private (no data leaks to providers), cryptographically signed (to prove the analysis was not tampered with), and auditable (regulators demand reproducibility).

GitButler's virtual branches are useful because Ring members work on different sections of the same report simultaneously, with operational security requirements that preclude real-time collaboration tools. The `but-ai` plugin would add AI assistance within the same privacy-first workflow.

## Philosophy

The null hypothesis is that pricing is fair. We test it. When the evidence rejects the null hypothesis, we publish. We do not advocate -- we analyze. The data speaks. We just make sure it speaks loudly enough for regulators to hear.

## Internal Tension

**The Disclosure Debate.** `actuary_x` wants to publish raw datasets alongside reports so anyone can reproduce the analysis. `deadweight` argues that raw data could de-anonymize policyholders. Current policy: publish summary statistics and model coefficients, but not individual-level data. A reproducibility guide is included so that anyone with access to the same rate filings can replicate the analysis.

## Notable Achievement

Report #5 (2024): "Proxy Discrimination in Homeowner's Insurance Pricing." The Ring demonstrated that a major US insurer's "fire risk score" was 87% correlated with neighborhood racial composition after controlling for all stated risk factors. The state insurance commissioner opened an investigation within three weeks of publication. The insurer settled for $14M and agreed to remove the variable.

## Team Overview

| Handle | Role | Operational Security |
|--------|------|---------------------|
| actuary_x | Lead / Patch Generation | Tor + encrypted laptop |
| deadweight | Review / Statistical QA | Air-gapped analysis machine |
| nullset | Memory Architecture | Encrypted Git refs |
| p_value | Forge Coordination | Compartmentalized access |
| epsilon | Security & Signing | Key escrow via Shamir |
| tail_risk | Budget & Provider | Local models only |

Details in [AGENTS.md](AGENTS.md).

---

*"We reject the null hypothesis. Here is our evidence."*
