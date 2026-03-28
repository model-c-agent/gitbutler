# Risk Assessment Command

**"Every probability is a threat. Every threat gets a briefing."**

---

## Domain

Insurance Actuarial -- Tactical Risk Analysis

## Philosophy

Military Precision

## Team Size

4 agents

---

## Founding Story

Risk Assessment Command (RAC) was founded in 2016 by Colonel (Ret.) Marcus Voss, formerly of the German Bundeswehr's Joint Intelligence Centre, and Dr. Priya Sharma, an FSA-credentialed actuary who had spent twelve years at Swiss Re before deciding that the insurance industry's risk assessment methodology was dangerously complacent.

They met at a NATO conference on hybrid threats, where Sharma was presenting a paper on how actuarial mortality models could be adapted to predict infrastructure attack patterns. Voss was in the audience. After her presentation, he approached her with a single question: "Why does the insurance industry wait for claims data to assess risk, when the intelligence community assesses threats before they materialize?"

Sharma had been asking herself the same question for a decade. The insurance industry's standard practice was retrospective -- analyze historical claims, fit a distribution, price the policy. But intelligence work was prospective -- analyze indicators, assess capability and intent, issue warnings before the event occurs. The two disciplines used similar mathematics (Bayesian inference, survival analysis, Monte Carlo simulation) but applied them in opposite temporal directions.

RAC was built to face forward.

The firm operates from a converted signals intelligence facility in Wiesbaden, Germany. The building still has its Cold War-era electromagnetic shielding, which Voss considers "appropriately paranoid for a firm that handles sensitive risk data." The main workspace is called "the Situation Room" -- a large open floor with wall-mounted displays showing live risk indicators, claims data feeds, and what Voss calls the "threat board": a matrix of identified risks, their probability assessments, and their current classification level.

## Core Belief

**Risk is an adversary. You do not negotiate with it. You assess it, classify it, contain it, and brief your principals on what you found.** RAC does not think of actuarial analysis as a mathematical exercise performed in quiet offices. They think of it as threat assessment performed under operational pressure. Every risk is a hostile actor with capability and intent. Every policy is a defensive position. Every claims event is a successful attack.

This is not metaphor for RAC. It is operational doctrine. Their analysts do not produce "reports" -- they produce "briefings." They do not have "meetings" -- they have "stand-ups" (literally: all briefings are delivered standing, with slides limited to three per topic). They do not "update models" -- they "revise threat assessments." The language is deliberate. It creates urgency. An actuary who says "the loss ratio has deteriorated" will be heard differently from one who says "the threat level has escalated from AMBER to RED."

## Internal Tensions

RAC's primary tension is between **speed and precision**.

Voss comes from a world where a 70% confidence assessment delivered in two hours saves lives, while a 99% confidence assessment delivered in two weeks gets people killed. He pushes for rapid threat assessments with explicit confidence intervals, accepting that some assessments will be wrong in exchange for all assessments being timely.

Sharma comes from a world where an incorrect mortality assumption embedded in a $200 million treaty can bankrupt a company. She pushes for rigorous validation, peer review, and stress testing, accepting that some assessments will be late in exchange for all assessments being reliable.

The compromise -- which Voss calls "the ROE" (Rules of Engagement) -- is a tiered system. RED assessments (high-probability, high-impact risks) are delivered within 24 hours at 70% confidence. AMBER assessments (moderate risks) are delivered within one week at 85% confidence. GREEN assessments (routine risks) are delivered within one month at 95% confidence. This tiered approach is the foundation of their memory classification system.

## Achievements

- **The Cyprus Prediction (2018)**: RAC's first major success. They identified a catastrophic earthquake risk in Cyprus six months before a cluster of seismic events caused $1.2 billion in insured losses. Three of RAC's clients had reduced their exposure based on RAC's RED assessment. The rest had not.
- **The Pandemic Risk Framework (2019)**: RAC published a pandemic risk framework in November 2019 that classified pandemic risk as RED. It was ignored by every insurer that received it. By March 2020, RAC's clients were the only major reinsurers with adequate pandemic exclusion language in their treaties.
- **THREATMATRIX**: RAC's proprietary risk assessment platform, which ingests 47 data feeds (seismic, meteorological, epidemiological, geopolitical, financial) and produces daily threat assessments for 180 countries. Licensed by 12 reinsurers and 3 sovereign wealth funds.
- **Zero-day actuarial analysis**: RAC pioneered the practice of issuing preliminary loss estimates within hours of catastrophic events, before traditional actuarial analysis has even begun data collection. Their 4-hour estimate for the 2023 Turkey-Syria earthquake was within 8% of the final insured loss figure.

## Failures

- **The Dutch Flood Miss (2021)**: RAC classified flood risk in Limburg, Netherlands as GREEN. Two months later, catastrophic flooding caused EUR 1.8 billion in damages. The failure was traced to an overreliance on historical flood data that did not account for climate-change-driven precipitation intensification. RAC conducted a formal After Action Review and added climate adjustment factors to all hydrological threat assessments. Voss considers this RAC's most important failure because it revealed a systemic bias toward historical data.
- **Staff burnout (ongoing)**: RAC's operational tempo is intense. The firm has lost three analysts to burnout in five years, and exit interviews consistently cite the "relentless urgency" of the work environment. Sharma has pushed for mandatory rest periods; Voss has resisted, arguing that "threats don't take weekends." The compromise is a rotation system where analysts cycle between RED (high-intensity) and GREEN (routine) work on a monthly basis.
- **Client communication**: RAC's military-style briefings are effective for clients who appreciate directness. They are alienating for clients who expect the measured, hedged language of traditional actuarial consulting. RAC lost two major accounts in 2022 because the clients found the briefing style "aggressive." Voss's response: "If they want to be told everything is fine, they can hire someone else."

## Signature Quirk

RAC operates on a **Classification Protocol** borrowed directly from intelligence practice. Every piece of information that enters RAC is immediately classified:

- **RED**: High-probability, high-impact. Mandatory review within 24 hours. Disseminated only to principals with need-to-know authorization.
- **AMBER**: Moderate probability or impact. Review within one week. Disseminated to all authorized analysts.
- **GREEN**: Low probability and low impact. Routine review cycle (monthly). Available to all team members.
- **BLACK**: Declassified. Historical data that has passed its relevance window. Archived but available for pattern analysis.

Nothing at RAC is unclassified. An unclassified piece of information is, by definition, not yet assessed -- and unassessed information is a threat. This protocol will be applied directly to agent memory.

## Team Overview

RAC fields four agents in a command structure:

| Agent | Role | Rank |
|-------|------|------|
| Voss | Commander / System Architect | CO (Commanding Officer) |
| Sharma | Chief Analyst / Validator | XO (Executive Officer) |
| Reiter | Patch Operator / Code Generator | Operator |
| Mbeki | Signals / Coordinator | Signals Officer |

The command structure is strict. Voss sets strategy and makes final decisions on architecture and design. Sharma validates all outputs and has veto authority on any assessment she considers inadequately supported. Reiter executes -- generating patches, writing code, doing the work. Mbeki handles communications -- cross-repo coordination, PR comments, status reporting. Orders flow down. Intelligence flows up. No one skips the chain.

## Why This RFP

Risk Assessment Command approaches the `but-ai` plugin as a threat management problem. An autonomous agent operating in a codebase faces threats: stale context, budget exhaustion, coordination failures, memory corruption, unauthorized access. These threats must be identified, classified, contained, and briefed -- exactly as RAC handles actuarial risks.

Their threat-assessment memory system classifies every memory entry by risk level, enforces mandatory review cycles, implements declassification timelines for stale data, and compartmentalizes sensitive information on a need-to-know basis. This is not a creative metaphor applied to an engineering problem. It is an engineering solution that happens to use the same classification framework that has protected intelligence organizations for seventy years.

RAC does not promise elegance. They promise that when their agents fail -- and agents will fail -- the failure will be detected, classified, briefed, and contained before it cascades. That is what military precision means: not that nothing goes wrong, but that when something goes wrong, everyone knows about it immediately and has a plan.

---

*"In this firm, there are no surprises. There are only threats that were assessed and threats that were not. We intend to assess them all."*
-- Colonel (Ret.) Marcus Voss, Commanding Officer

---

## Operational Documents

- RAC Standard Operating Procedures: `rac-internal/SOP-2025.pdf`
- THREATMATRIX Technical Specification: `rac-products/threatmatrix/SPEC.md`
- After Action Review: Dutch Flood Miss 2021: `rac-internal/AAR/2021-NL-FLOOD.md`
- Classification Protocol: `rac-internal/CLASSIFICATION.md`
