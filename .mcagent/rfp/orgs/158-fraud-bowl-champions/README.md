# Fraud Bowl Champions

**"We train on fake data so we can find the real thing."**

---

## The Team

The Fraud Bowl Champions are a competitive forensic accounting team that competes in fraud detection challenges -- timed events where teams analyze synthetic financial datasets to identify embedded fraud patterns. Think of it as CTF (Capture the Flag) for accountants, except instead of exploiting vulnerabilities, you are finding where the money went.

The team was formed in 2021 by four forensic accountants who met at the inaugural Fraud Bowl, an annual competition organized by the Association of Certified Fraud Examiners (ACFE). They placed second that year. They won in 2022, 2023, and 2025. They placed third in 2024 because Rajan had food poisoning and could not compete, and the team refuses to compete with a substitute.

The members: Coach Lin (team captain, strategy), Rajan Patel (speed analyst), Amara Diop (pattern specialist), and Devon Park (tooling and automation). They call themselves a "sports team" without irony. They train weekly, review tape (competition recordings), and run drills on synthetic datasets. Rajan has a warm-up routine that involves mentally computing Benford distributions for random number sets.

## The Competition Circuit

The Fraud Bowl circuit includes three major annual events:

1. **The Fraud Bowl** (ACFE, Austin, TX): The original. Teams of four analyze a synthetic corporate dataset with 10-15 embedded fraud schemes. 4-hour time limit. Scoring based on correctly identified schemes (precision) and schemes not missed (recall).

2. **The European Forensic Challenge** (CILEA, Brussels): Similar format, but datasets include European regulatory structures (GDPR implications, multi-currency, EU VAT schemes).

3. **The Asia-Pacific Forensic Data Challenge** (ISACA, Singapore): Focus on technology-enabled fraud: cryptocurrency, DeFi exploits, digital banking.

The Champions compete in all three. Their combined record: 8 wins, 3 podium finishes, 1 DNS (Devon's visa was denied for Singapore in 2023).

## From Competition to Software

Devon built the team's competitive tooling: scripts that automated the tedious parts of fraud detection (data loading, preliminary statistical tests, timeline visualization) so the team could focus on the analytical parts. After their 2023 Fraud Bowl win, several competing teams asked Devon to share the tools. She open-sourced them.

The tools evolved into an AI-assisted pipeline: agents that ran Benford tests, flagged round-number transactions, detected duplicate invoices, and generated preliminary investigation timelines. The pipeline consumed synthetic competition data and produced structured findings that the team could review and refine during competition.

When the `but-ai` RFP appeared, Devon saw it as a framework for the pipeline she had already built. The team agreed to respond, treating the RFP itself as a competition: "We have 51 files to produce. Let's set a time."

## Philosophy

### On Practice

You get good at fraud detection the same way you get good at anything: deliberate practice on realistic scenarios. Our synthetic datasets are designed to be indistinguishable from real financial data with real fraud embedded in it. When we find a scheme in competition, the cognitive pathway is identical to finding one in a real investigation. Practice transfers.

### On Speed

Competition rewards speed. But speed in fraud detection is not recklessness -- it is prepared expertise executing efficiently. We are fast because we have seen every common fraud pattern hundreds of times in training. An unusual pattern slows us down. A familiar pattern takes seconds.

### On AI

AI agents are training partners. They run the drills we don't have time for. They process the datasets we don't have bandwidth for. They do not replace the team -- they make the team's preparation more efficient.

## Tension

**The Automation Ceiling.** Devon believes AI agents can handle 70% of fraud detection in competition settings -- the standard patterns, the obvious anomalies, the statistical red flags. Coach Lin disagrees: "Competitions are designed to test the edge cases. The 70% the AI handles is the easy part. We win on the 30% that requires human insight." Devon's counter: "If the AI handles the 70%, we have 4 hours to focus on the 30% instead of spending 3 hours on setup." They compromise by using AI for pre-screening and reserving human analysis for findings that require judgment.

## Achievement

At the 2025 Fraud Bowl, the Champions identified 14 of 15 embedded fraud schemes in 3 hours and 12 minutes, using their AI-assisted pipeline for initial screening and human analysis for confirmation. The 15th scheme -- a complex trade-based money laundering pattern -- was missed because the team ran out of time during manual analysis. Their score of 14/15 (93.3% recall) with zero false positives set a competition record. The previous record was 12/15.

## Team

| Agent | Role | Focus |
|-------|------|-------|
| Coach Lin | Captain / Strategist | INDEX.patch, task prioritization, review |
| Rajan | Speed Analyst | Fast pattern detection, memory retrieval |
| Amara | Pattern Specialist | Complex scheme analysis, forge coordination |
| Devon | Tooling | Provider abstraction, automation, signing, infrastructure |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

The team operates in "rounds" -- timed analysis sprints modeled on competition format. Each round has a fixed duration and a clear objective. Between rounds, a 2-minute debrief: what did we find, what did we miss, where should we look next.

Communication during rounds is competition-terse: "BL1 hit" (Benford first-digit anomaly), "RN 9999" (round number just below threshold), "DUP vendor" (duplicate vendor detected).

---

*"We have seen every fraud pattern in the book. Now we are writing the book."*
