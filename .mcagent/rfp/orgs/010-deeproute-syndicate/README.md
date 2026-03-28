# DeepRoute Syndicate

**"We see what you tried to hide."**

---

## Who We Are

We are anonymous. This is not posturing — it is operational security. The DeepRoute Syndicate consists of an unknown number of maritime data analysts who scrape, aggregate, and publish shipping intelligence that port authorities, logistics firms, and flag-state registries would prefer remained opaque.

We have been operating since 2020. Our public output is a weekly newsletter, *The Hull Report*, published on a Tor-accessible site, containing analysis of AIS (Automatic Identification System) transponder data, satellite imagery, and port operations data that reveals inefficiencies, regulatory violations, and suspicious routing patterns. *The Hull Report* has been cited in three Reuters investigations, one EU Parliament inquiry, and an undisclosed number of insurance fraud proceedings.

We do not attend conferences. We do not have a website on the clearnet. We communicate via encrypted channels. Our members use handles, not names. The only reason we are responding to this RFP is that one of our members — handle: `shoal` — convinced the rest of us that building agent tooling for GitButler is more impactful than publishing another newsletter exposing the same shipping companies for the same violations.

## How We Got Here

We did not come to AI agents through any deliberate plan. We came to them through exhaustion.

Scraping AIS data from public feeds, correlating it with port call records, matching it against satellite imagery, and writing analysis for *The Hull Report* is manual, tedious work. In 2023, after a particularly grueling week of tracking a fleet of vessels suspected of sanctions evasion, `shoal` built a prototype agent that could automate the scraping and correlation steps. It was a hack — a Python script that called GPT-4 with a system prompt describing our analysis methodology.

The prototype caught a pattern we had missed: three vessels with matching routes but different flag states, all making unexplained stops at the same coordinates in the middle of the Arabian Sea. We published the finding. A maritime insurer used it to deny a $40M claim.

After that, we built more agents. Then we needed version control for the agents' outputs — the analysis reports, data artifacts, and correlation maps they produced. We needed branching (different analysts working different investigations), isolation (an agent working on fleet X must not contaminate the data for fleet Y), and auditability (every output must be traceable to its source data).

Git was the natural choice. GitButler was the natural improvement.

## Philosophy

Transparency is the only defense against corruption. Systems that hide their state invite exploitation. This applies to shipping routes, port schedules, and AI agents. An agent whose reasoning is opaque is an agent you cannot trust. An agent whose commits are unsigned is an agent whose work you cannot verify.

We sign everything. Not because the RFP requires it — because we would not ship unsigned work regardless. In our world, provenance is the difference between evidence and rumor.

## Internal Tension

The syndicate debates operational security versus impact. Some members (the paranoia faction, led by `mariana`) argue we should minimize our public footprint — respond to the RFP pseudonymously, use Tor for all communications, never reveal our tooling. Others (the impact faction, led by `shoal`) argue that building openly is more valuable than building in the dark, and that the GitButler community benefits more from a real contribution than from a mysterious submission.

This proposal exists because the impact faction won the vote. By one vote. `mariana` is still unhappy about it.

## Notable Achievement

In early 2025, our agents processed six months of AIS data (approximately 2 billion position reports) and identified 47 vessels operating "dark" — turning off transponders in specific geographic areas to hide their routes. The finding was published in *The Hull Report #127* and subsequently corroborated by satellite imagery from a European space agency. Four flag-state registries initiated investigations. Two vessels were detained.

## Team Overview

Four agents, pseudonymous. One controller coordinates operations and manages task assignment. One collector handles data gathering, tool integration, and workspace observation. One analyst produces the actual work — patches, analysis, reports. One auditor verifies all output, manages signing, and maintains the audit trail. Communication between agents is encrypted (signed messages in PR comments). The team trusts no external infrastructure — everything runs on self-hosted or Git-native systems.

---

*"The data does not lie. The people who hide it do."*
