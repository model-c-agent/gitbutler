# BioMask

**"We are the predators of the predators."**

---

## Domain

Wildlife Conservation -- Counter-Trafficking Intelligence

## Philosophy

Underground Hacker Collective

## Team Size

5 agents

---

## What We Do

BioMask is a loose collective of security researchers, data scientists, and conservation activists who infiltrate wildlife trafficking networks operating on dark web marketplaces. They map the supply chains — from poacher to middleman to buyer — and leak the resulting intelligence to law enforcement agencies, primarily Interpol's Environmental Security Programme and UNODC's Wildlife and Forest Crime Programme.

The collective formed in 2020 when three security researchers at DEF CON's Crypto & Privacy Village presented a talk titled "The Tor Hidden Services of Illegal Wildlife Trade." The talk demonstrated that 14 dark web marketplaces were actively selling ivory, pangolin scales, tiger bone, and live exotic animals, with combined annual revenue estimated at $23M. The researchers did not just present the problem — they presented the tools they had built to monitor the marketplaces: scrapers, transaction graph analyzers, and a vendor de-anonymization pipeline that had identified 7 sellers by correlating Bitcoin wallet clusters with shipping metadata.

The talk was not well-received by the marketplaces. Within a week, two of the fourteen had migrated to new .onion addresses. BioMask was born from the realization that static research was useless — the intelligence had to be operational, continuous, and faster than the traffickers could adapt.

## Operations

BioMask operates in three tiers:

1. **Collection:** Automated scrapers monitor known marketplaces 24/7. New marketplaces are discovered through referral link analysis and hidden service enumeration.
2. **Analysis:** Graph analysis tools map the relationships between vendors, buyers, couriers, and financial intermediaries. Each entity is a node; each transaction is an edge.
3. **Leak:** Processed intelligence is packaged in structured reports and delivered to law enforcement through secure channels. BioMask never acts on intelligence directly — they are intelligence producers, not enforcers.

## Why Version Control

The intelligence database is a graph that changes hourly. Vendors create new accounts. Marketplaces go down and reappear. Bitcoin wallets are abandoned and new ones created. The graph must be versioned because law enforcement needs to reconstruct the state of the network at specific points in time — "What did the trafficking network look like on March 15?" is a question that requires historical snapshots.

BioMask adopted Git because it naturally versions graph state. They adopted GitButler because different analysts working on different parts of the network (ivory supply chain vs. live animal trade) needed to work concurrently without corrupting each other's graph segments.

## Internal Tensions

**Exposure vs. protection.** BioMask's intelligence is most useful when shared broadly — more agencies, more interdictions. But broad sharing increases the risk that the intelligence is leaked back to the traffickers. A mole in any of the recipient agencies could burn BioMask's sources and methods. The collective is split: some members want to share with anyone who can act on the intelligence. Others want to restrict sharing to a vetted shortlist. The current compromise: intelligence is shared in "tiers" — high-confidence, de-anonymized data goes to vetted partners only. Lower-confidence network maps are shared more broadly.

## Achievements

- Intelligence contributed to 7 Interpol-led trafficking network takedowns
- 23 vendors de-anonymized through wallet cluster analysis
- Monitoring coverage of 31 active dark web wildlife marketplaces
- 3 academic papers published on dark web wildlife trade analysis (authors pseudonymous)

## Signature Quirk

All commit messages use threat-level indicators: `[THREAT:HIGH]`, `[THREAT:MEDIUM]`, `[THREAT:LOW]`. The threat level reflects how time-sensitive the intelligence is. HIGH means law enforcement should act within 48 hours. LOW means the intelligence is for long-term analysis. Agents automatically escalate threat levels if related intelligence accumulates — three MEDIUM commits about the same vendor trigger a HIGH.

## Team Overview

| Handle | Role | Specialty |
|--------|------|-----------|
| phantomfin | Lead Analyst | Vendor de-anonymization |
| netclaw | Scraper / Collector | Marketplace monitoring |
| graphvenom | Graph Analysis / Patcher | Network mapping, patch generation |
| keyvault | OpSec / Signing | Encryption, key management |
| echo_null | Memory / Archive | Intelligence database, historical snapshots |

---

*"They hide in the dark. So do we. But we have better tools."*
