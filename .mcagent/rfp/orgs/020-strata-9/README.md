# Strata-9

**"The provenance gap is the tell."**

---

## What We Are

Strata-9 is a collective of digital archaeologists — a term we use without irony. We excavate databases. Our subjects are not buried cities but buried data: museum collection records, auction house catalogs, customs declarations, and provenance chains that have been deliberately obscured, accidentally corrupted, or conveniently incomplete.

We formed in 2021 after three of our founding members independently noticed the same pattern: a cluster of antiquities appearing on the European market with provenance documentation that traced back to a single dealer in Beirut, whose records had been "destroyed in the 2020 port explosion." Convenient. We cross-referenced the objects against pre-explosion photographs of the dealer's warehouse (scraped from real estate listings), auction records from three houses, and customs databases from four countries. The provenance was fabricated. The objects had been looted from a site in northern Syria.

We published our analysis on a dark web forum dedicated to cultural heritage protection (yes, this exists). The analysis was picked up by INTERPOL's Works of Art unit. Six objects were seized. Two dealers were charged.

We have been doing this ever since. Our members use handles. We communicate via encrypted channels. We publish on Tor. Our data sources are public records, leaked databases, and open-source intelligence. We do not hack systems — we read what is already there, connect what others missed, and publish what institutions would prefer remained hidden.

## How We Got Into Agent Development

Our work is data-intensive. Tracing the provenance of a single artifact can require cross-referencing dozens of databases, each with a different schema, different access method, and different reliability level. Doing this manually takes weeks. We needed automation.

In 2023, we built agents that could crawl museum collection APIs, parse auction catalogs (PDFs, HTML, sometimes scanned images), and construct provenance timelines automatically. The agents were good at data collection but terrible at judgment — they could not tell the difference between a genuine provenance gap and a suspicious one. We taught them to flag suspicious patterns: gaps that coincide with known conflict events, provenance chains through weak-export-control jurisdictions, and documentation citing dealers known to have handled looted material.

Version control became critical when we started tracking multiple investigations simultaneously. Each investigation has its own data, its own agents, its own hypotheses. Cross-contamination between investigations is unacceptable. Git branches provided isolation. GitButler's virtual branches provided concurrent multi-investigation workflow.

## Philosophy

Absence is data. A missing provenance record is not a gap — it is information. A database that was recently modified to remove entries is telling you something. A museum that declines a FOIA request has given you an answer.

Our agents are trained to notice absence. When examining a codebase, the absence of tests in a module that handles sensitive data is as significant as the presence of a bug. When reviewing a patch, what was removed matters as much as what was added.

## Internal Tension

The collective debates publication ethics. Some members (the "sunlight" faction) believe all findings should be published immediately and publicly. Others (the "scalpel" faction) believe findings should be shared selectively — with law enforcement first, then institutions, then the public — to maximize impact. The current practice follows the scalpel approach, but the sunlight faction keeps pushing.

## Notable Achievement

In 2025, Strata-9 published an analysis identifying provenance gaps in 340 objects across 12 European museums. The analysis, compiled by our agents in three weeks (a two-year task for human researchers), revealed that 47 objects had documentation tracing to the same three intermediary dealers — a network previously unknown to investigators. Two museums voluntarily removed flagged objects. The analysis is cited in a pending EU directive on museum provenance transparency.

## Team Overview

Four agents structured as an investigation cell. One analyst coordinates research and decomposes investigations into leads. Two investigators handle data collection and patch generation. One archivist manages evidence storage, memory integrity, and commit signing. All communications are signed and logged. The cell operates on need-to-know: each agent accesses only the data required for its current lead.

---

*"Follow the gap."*
