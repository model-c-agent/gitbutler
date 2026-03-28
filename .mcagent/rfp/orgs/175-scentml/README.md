# ScentML

**"Molecules we have never smelled. Fragrances that have never existed. Until now."**

---

## Origin

ScentML was founded in 2022 in San Francisco by Dr. Nina Patel and Alejandro Voss, who met in the Stanford CS PhD programme and bonded over a shared obsession: could a machine learning model design a molecule that smells like something no human has ever smelled before?

Nina's background was in molecular generation — training variational autoencoders to produce novel drug candidates. Alejandro's background was in sensory neuroscience — modelling how the brain maps molecular structure to perceived odour. Separately, they were working on adjacent problems. Together, they realised they could close the loop: generate novel molecules computationally and predict their smell before synthesizing them.

Their first model, trained on 300,000 structure-odour pairs scraped from fragrance patents, PubChem annotations, and Leffingwell's database, could generate molecules with specified olfactory properties at a 61% blind-evaluation accuracy rate. Their second model, fine-tuned on the evaluation data from the first model's outputs, hit 78%. By the time they graduated, they had a prototype, a patent application, and a seed term sheet.

ScentML raised $5.1M in seed funding (Lux Capital, Obvious Ventures) in 2023. The pitch: "We generate novel fragrance molecules that have never existed in nature or synthesis. We predict what they smell like. We license them to fragrance houses." The business model is straightforward: molecule discovery as a service.

The team is now 18 people: 8 ML engineers, 4 synthetic chemists (who actually make the molecules), 3 evaluators (trained perfumers who smell the molecules and provide ground truth), and 3 in business/operations. Five of them are working on this RFP.

## Team

| Agent | Role | Background |
|-------|------|------------|
| Dr. Nina Patel | CEO / Orchestration & Memory | Molecular generation, VAE architectures |
| Alejandro Voss | CTO / Patch Architect | Sensory prediction, GNN models |
| Dr. Mei-Ling Wu | Chemistry Lead / Provider & Budget | Synthetic chemistry, process optimization |
| Jordan Okafor | ML Ops / Forge Adapter | Infrastructure, model serving, CI/CD |
| Sara Fitzgerald | Legal & Compliance / Signing | IP law, patent strategy, fragrance regulation |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Novelty Trap.** Nina wants ScentML to produce molecules that are genuinely novel — compounds that have never been synthesized and that smell unlike anything on the market. Alejandro worries that excessive novelty is commercially valueless. Fragrance houses want molecules that are novel enough to patent but familiar enough to incorporate into existing accords. A molecule that smells like nothing anyone has ever experienced is interesting research but bad product. The tension plays out in model training: Nina pushes for exploration (generating molecules far from the training distribution), Alejandro pushes for exploitation (generating molecules near successful existing fragrances). The model's temperature parameter is their proxy war.

## Achievement

In 2024, ScentML's model generated a molecule designated SM-4217 — a novel amber-musk hybrid that was synthesized, evaluated by a panel of twelve perfumers, and rated 8.4/10 for commercial potential. SM-4217 was licensed exclusively to Firmenich for $1.2M. It is expected to appear in a commercial fragrance in 2026. Nina keeps the original evaluation scores framed in the office. Alejandro keeps the molecular structure printed on a t-shirt that he wears to investor meetings, which Sara considers unprofessional and Nina considers endearing.

---

*"The best molecule is the one nobody imagined."*
— Pitch deck slide 1, all four versions
