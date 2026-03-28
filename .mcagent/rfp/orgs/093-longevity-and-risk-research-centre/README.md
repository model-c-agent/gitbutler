# The Longevity & Risk Research Centre

**"Every memory has a half-life. The question is what distribution governs its decay."**

---

## Domain

Insurance Actuarial -- Mortality Modeling and Survival Analysis

## Philosophy

Academic Research Lab

## Team Size

5 agents

---

## Founding Story

The Longevity & Risk Research Centre (LRRC) was established in 2014 at the London School of Economics, Department of Statistics, by Professor Elena Vassiliev, a Russian-born demographer who had spent twenty years at the Office for National Statistics studying why British people die when they do. Her frustration was specific: the insurance industry used mortality models that were, in her professional opinion, embarrassingly crude. The standard Lee-Carter model, published in 1992, was still the backbone of most longevity risk pricing in 2014, despite three decades of evidence that it systematically underestimated improvements in mortality at older ages.

Vassiliev assembled a research group with a single mission: build mortality models that are actually right. Not approximately right. Not right-on-average. Right at the individual level, accounting for socioeconomic status, geography, lifestyle factors, genetic predisposition, and the interactions between all of them.

The LRRC's first model, published in 2016, was called MORTAL-1. It replaced the Lee-Carter framework with a Bayesian hierarchical model that estimated age-specific mortality improvements as a function of 23 covariates, with full posterior uncertainty quantification. When tested against held-out data from 2010-2015, it reduced prediction error by 34% compared to Lee-Carter. The paper was published in the *Journal of the Royal Statistical Society* and was downloaded more times in its first month than any actuarial paper in the journal's history.

Three insurers attempted to acquire the LRRC between 2017 and 2020. Vassiliev refused all offers. "A model that can be bought can be suppressed," she said. "A model that is published cannot." All LRRC models, data pipelines, and validation frameworks are published under open-access licenses. This has made the LRRC both beloved by the academic community and deeply irritating to the proprietary-model vendors who had been selling inferior products at premium prices.

The LRRC is now on its fourth-generation model (MORTAL-4), which incorporates machine learning components for non-linear interaction effects while maintaining the full Bayesian uncertainty quantification that is the lab's hallmark. The model's most controversial feature is its "surprise index" -- a metric that quantifies how much new data deviates from the model's predictions, allowing the model to flag when its own assumptions may be failing.

## Core Belief

**Everything decays, but not everything decays the same way.** The LRRC studies how things die -- organisms, populations, risk factors, and now, memories. Their central insight is that decay is not a single process. Some things decay exponentially (radioactive isotopes, the relevance of yesterday's stock price). Some things decay according to a Weibull distribution (mechanical components, where failure rate increases with age). Some things have bathtub-shaped hazard functions (humans, who are most likely to die as infants, least likely to die as young adults, and increasingly likely to die as they age).

The LRRC approaches agent memory with this framework. Every memory entry has a survival function -- the probability that it remains relevant at time *t* given that it was relevant at time *t-1*. Different types of memories have different survival distributions. A memory about a project's architecture has a long half-life and a Weibull hazard rate (it becomes less reliable as the codebase evolves, but slowly). A memory about a specific bug has a short half-life and an exponential hazard rate (once the bug is fixed, the memory's relevance drops precipitously). A memory about a team's coding conventions has a bathtub-shaped hazard rate (initially uncertain, stable once learned, increasingly unreliable as the team changes).

Fitting the right survival distribution to each memory type is, in the LRRC's view, the key to building a memory system that expires gracefully rather than catastrophically.

## Internal Tensions

The LRRC's tensions are methodological.

**The Bayesians**, led by Vassiliev, insist that all uncertainty must be quantified as full posterior distributions. Point estimates are lies. Confidence intervals are better lies. Only the complete posterior distribution tells the truth. This makes LRRC models computationally expensive and difficult to explain to non-statisticians, but Vassiliev considers both problems acceptable prices for intellectual honesty.

**The Pragmatists**, led by Dr. James Okonkwo (a former Chief Actuary at Aviva who joined the LRRC in 2019 after retiring from industry), argue that a model no one uses is worse than an imperfect model everyone uses. He pushes for simplified outputs -- point estimates with confidence intervals, lookup tables, Excel-compatible exports -- that practicing actuaries can actually incorporate into their workflows. He calls Vassiliev's full posteriors "beautiful and useless in a quarterly board meeting."

The compromise is a two-tier output system. The full Bayesian posterior is always computed and published for the academic audience. A simplified "practitioner summary" with point estimates, confidence intervals, and sensitivity analyses is produced for the industry audience. Okonkwo reviews all practitioner summaries; Vassiliev reviews all posteriors. Neither is allowed to simplify the other's output without consent.

This tension directly informs their approach to agent memory. The "full posterior" version of a memory is the complete context with all metadata. The "practitioner summary" version is the compressed representation that survives context window compaction. Both must be maintained.

## Achievements

- **MORTAL-1 through MORTAL-4**: Four generations of mortality models, each representing a significant methodological advance
- **The Surprise Index**: A real-time metric for detecting when a model's assumptions are being violated by incoming data, adopted by 8 pension funds for monitoring their longevity exposure
- **Open-access commitment**: All models, code, and data published under open licenses, despite three acquisition attempts
- **The Vassiliev-Okonkwo Framework**: A methodology for translating full Bayesian posteriors into actionable actuarial tables without losing critical uncertainty information. Published in the *British Actuarial Journal* (2022)
- **Advisory role**: The LRRC advises the UK's Government Actuary's Department on longevity risk methodology
- **Negative results registry**: The LRRC maintains a public registry of modeling approaches that did not work, complete with data and code, so that other researchers do not repeat their failures

## Failures

- **MORTAL-2's COVID Blind Spot**: MORTAL-2, calibrated on 2000-2019 data, completely failed to anticipate the mortality shock of 2020-2021. This was not a modeling failure per se -- no mortality model predicted COVID -- but the lab was criticized for not having a mechanism to rapidly incorporate pandemic mortality into the model. MORTAL-3 was developed specifically to address this, incorporating a "shock component" that activates when the surprise index exceeds a threshold.
- **The Socioeconomic Overfit**: An early version of MORTAL-3 incorporated 47 socioeconomic covariates and achieved spectacular in-sample fit. Out-of-sample, it performed worse than the simple Lee-Carter model. The lab had overfit. This was a humbling experience for a group that prided itself on methodological rigor. They now enforce a strict cross-validation protocol with a held-out test set that no researcher is allowed to touch until the final model evaluation.
- **The Pension Fund Incident**: In 2021, a pension fund misinterpreted LRRC's practitioner summary as endorsing a specific investment strategy. The fund lost GBP 40 million. The LRRC was not legally liable but felt morally responsible for insufficiently clear communication. All practitioner summaries now include a "Limitations and Appropriate Use" section written by Okonkwo in language designed to be impossible to misinterpret.

## Signature Quirk

The LRRC operates on **Survival Time**. Every project, every analysis, every model component has an estimated survival function -- a probability distribution over its useful life. When a component's survival probability drops below 50% (the median lifetime), it is flagged for review. When it drops below 10%, it is flagged for deprecation. When it drops below 1%, it is archived.

This is not a metaphorical practice. The lab literally computes survival functions for their own research artifacts. A dataset's survival function depends on how frequently the underlying population changes. A model's survival function depends on how quickly the phenomena it models are evolving. A paper's survival function depends on how rapidly the field is advancing.

They intend to apply exactly the same framework to agent memory. Every memory entry gets a fitted survival distribution. The distribution parameters are estimated from the memory's type, the rate of change in the relevant codebase area, and the frequency with which the memory has been accessed. Memories die according to their own hazard rates, not according to arbitrary TTL values.

## Team Overview

The LRRC fields five agents, organized as a research group:

| Agent | Role | Specialty |
|-------|------|-----------|
| Vassiliev | Principal Investigator / Architect | Bayesian survival modeling |
| Okonkwo | Practitioner Liaison / Validator | Actuarial translation and validation |
| Petrov | Research Fellow / Patch Generator | Code implementation and patch production |
| Abebe | Data Curator / Memory Manager | Survival function estimation for memory entries |
| Chen | Research Assistant / Coordinator | Cross-repo coordination and PR management |

The group operates with academic hierarchy: Vassiliev sets the research direction, Okonkwo validates against practical requirements, Petrov implements, Abebe manages data and memory, Chen handles logistics and communication. Decisions are made by discussion and consensus, with Vassiliev holding a casting vote on methodological questions and Okonkwo holding a casting vote on practical usability questions.

## Why This RFP

The LRRC applies to this RFP because the central problem of agent memory expiration is a survival analysis problem, and survival analysis is what they do better than anyone in the world.

Every existing memory system uses fixed TTLs. A memory expires after N minutes, N hours, N days. This is the equivalent of assuming that every human dies at age 78 -- technically the average, but wrong for every individual. The LRRC's approach fits a survival distribution to each memory entry based on its type and context, allowing memories to expire at rates that reflect their actual probability of remaining relevant. A memory about a project's license file has a long median survival. A memory about a branch's current HEAD commit has a short one. The system does not treat them the same way, because they are not the same kind of thing.

They also bring the "surprise index" concept from mortality modeling. When an agent encounters data that deviates significantly from what its memory predicts, the surprise index spikes, triggering a memory review cycle. This is how the LRRC's mortality models detect when their assumptions are failing, and it translates directly to detecting when agent memory has become stale.

---

*"The insurance industry taught us that every risk has a price. Our lab taught us that every memory has a mortality rate. The `but-ai` plugin needs to know both."*
-- Professor Elena Vassiliev, Principal Investigator

---

## Publications

- Vassiliev, E. et al. "MORTAL-1: A Bayesian Hierarchical Framework for Multi-Population Mortality Modeling." *JRSS Series B*, 2016.
- Vassiliev, E. and Okonkwo, J. "Translating Bayesian Posteriors to Actuarial Practice Without Losing Uncertainty." *British Actuarial Journal*, 2022.
- Okonkwo, J. "The Practitioner's Dilemma: When Full Information is Too Much Information." *Annals of Actuarial Science*, 2023.
- LRRC Negative Results Registry: `lrrc.lse.ac.uk/negative-results/`
- MORTAL-4 codebase: `github.com/lrrc-lse/mortal-4` (open access, GPL-3.0)
