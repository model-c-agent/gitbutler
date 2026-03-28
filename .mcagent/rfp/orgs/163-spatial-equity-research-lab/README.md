# The Spatial Equity Research Lab

**"Every parcel tells a story. Most of them are stories of exclusion."**

---

## Origin

The Spatial Equity Research Lab (SERL) was established in 2018 as a cross-departmental initiative at the University of Manchester, born from a disagreement between a geographer and an economist over a pub lunch about whether housing segregation in Birmingham could be measured from publicly available parcel data alone. The geographer — Dr. Adaeze Okonkwo — said yes, and bet a year of research funding on it. The economist — Dr. Felix Brandt — said no, and matched the bet.

Adaeze won. The paper that settled the bet, "Exclusionary Geometries: Lot Size as Proxy for Racial Segregation in Seven English Cities," was published in Environment and Planning B in 2019 and has been cited 340 times. The methodology was deceptively simple: correlate minimum lot size requirements in zoning codes with demographic data at the census tract level. In all seven cities studied, larger minimum lot sizes predicted whiter, wealthier neighborhoods with near-perfect accuracy.

Felix joined the lab the month the paper was published. He brought his econometric models. Adaeze brought her GIS expertise. Together they hired three more researchers and secured a five-year grant from the Leverhulme Trust to build a computational framework for quantifying spatial injustice.

## Philosophy

### On Data

We believe spatial data is never neutral. Every dataset about land use, zoning, or property encodes the decisions — and the biases — of the people who drew the boundaries. Our job is not to present data as objective truth but to interrogate it: who drew this line, when, and who was excluded by it?

### On Computation

Computational methods amplify. If the underlying data is biased, computation amplifies the bias. If the methodology is transparent, computation amplifies transparency. We publish all code, all data cleaning scripts, and all intermediate results. A spatial equity analysis that cannot be reproduced is not science — it is opinion with maps.

### On AI

AI agents operating on spatial data must be accountable. An agent that generates a zoning recommendation is making a political decision, whether or not it knows it. We approach AI agent development with the same rigor we apply to our published research: every output must be traceable to its inputs, every decision must be auditable.

## How We Found This RFP

In 2025, Adaeze began experimenting with AI agents to automate the most tedious part of the lab's work: extracting zoning parameters from municipal code PDFs and geocoding them to parcel boundaries. The agents worked well individually but coordinated poorly — two agents would independently process the same municipality and produce conflicting extractions with no mechanism to reconcile.

When a postdoc forwarded the `but-ai` RFP, Felix recognized the problem they were solving: multi-agent coordination through version control. The INDEX.patch workflow mapped directly onto how the lab already worked — each researcher producing independent analyses that needed merging into a shared dataset.

## Team

Five researchers. Dr. Okonkwo leads, but SERL operates by academic consensus — major methodological decisions require group agreement, which usually takes two to four weeks and at least one whiteboard session that devolves into an argument about map projections.

| Agent | Role | Background |
|-------|------|------------|
| Dr. Adaeze Okonkwo | Principal Investigator / Memory Architect | GIS, spatial statistics, 15 years |
| Dr. Felix Brandt | Econometric Modeler / Budget Analyst | Urban economics, policy analysis |
| Priya Mehta | Postdoc / Patch Architect | Computational geometry, zoning extraction |
| Tomoko Ishii | Research Associate / Forge Adapter Lead | Data engineering, municipal data systems |
| Sam Nwosu | PhD Student / Security & Identity | Cryptography, data provenance |

Profiles in [AGENTS.md](AGENTS.md).

## Internal Tension

**The Normative Gap.** Adaeze believes the lab's tools should include normative recommendations — the analysis should not just show that a zoning code is exclusionary but explicitly say "this zoning code is exclusionary and should be changed." Felix disagrees sharply. He argues that the lab's credibility depends on presenting evidence without policy prescriptions, and that embedding normative judgments in computational tools crosses a line from research to advocacy. The tension is productive but unresolved. Their agents reflect this divide: Adaeze's memory schemas tag spatial patterns as "equitable" or "exclusionary," while Felix's budget models deliberately strip normative labels.

## Notable Achievement

In 2023, the lab's analysis of Sheffield's proposed comprehensive plan update was entered into public testimony by a community organization. The analysis showed that the plan's proposed upzoning was concentrated in already-dense neighborhoods with predominantly South Asian populations, while low-density suburban neighborhoods with predominantly white populations were left untouched. The planning commission revised the plan. The lab's GIS methodology was cited in the commission's official findings. Priya cried in the office when the news came through. Adaeze pretended she did not.

## Working Style

Academic pace. Everything is documented, reviewed, and cross-referenced. The lab meets weekly in person and uses a shared LaTeX document for all internal communication that is not email. Code reviews take days, not hours. This is slow. They know it is slow. They believe rigor is worth the cost.

---

*"The map is not the territory. But the zoning code is."*
— Lab motto, adopted 2019
