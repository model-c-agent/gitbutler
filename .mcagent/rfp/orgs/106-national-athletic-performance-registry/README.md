# National Athletic Performance Registry

**"The data will be processed. Eventually."**

---

## Establishment

The National Athletic Performance Registry (NAPR) was established in 2018 by an act of parliament, following a report by the Select Committee on Sport and Digital Infrastructure that recommended "a centralized, standardized, and publicly accessible repository of athletic performance data for all nationally sanctioned sporting events."

The act allocated funding for five years. Hiring took eighteen months. The Registry opened in 2020 in a government office building in Croydon that also houses the Passport Office overflow and a training center for the Valuation Office Agency. The Registry occupies the third floor. The lifts are unreliable.

Thirty-one civil servants are employed by the Registry. Of these, seven work on data processing, four on IT systems, three on standards compliance, and seventeen on administrative functions including procurement, HR, facilities management, and responding to Freedom of Information requests about why the Registry takes so long to publish data.

## The Processing Pipeline

The Registry's mandate is to collect, standardize, and publish performance data from all nationally sanctioned sporting events in England. In theory, this means every county cricket match, every National League football game, every athletics meeting, and every swimming competition. In practice, data arrives in 47 different formats from 23 governing bodies, most of whom consider data submission to the Registry a low priority.

The data processing pipeline is: receive (email, FTP, and in one case a USB stick mailed quarterly), validate (check format, check completeness, check for obvious errors), standardize (convert to the Registry's format), review (a human checks the standardized data against the original), and publish (upload to the public website, which was last redesigned in 2021).

Average time from event to publication: six months. The Select Committee's target was thirty days. The Registry has submitted three reports explaining the gap. The Committee has read none of them.

## Why Software Development

In 2024, the incoming IT lead (Geoff Barnett, 41, former local government IT manager) proposed automating the standardization step, which accounted for 60% of the processing time. He was given a budget, a team of two developers (one of whom was reassigned from the Passport Office overflow), and a mandate to "modernise the pipeline without breaking anything."

Geoff introduced Git, CI/CD, and automated testing. GitButler was adopted after a procurement process that took four months and required three rounds of approval. Virtual branches were useful because the Registry needed to support 47 input formats simultaneously, each in various states of implementation.

The `but-ai` RFP represents an opportunity to automate validation and standardization for the most common formats, potentially reducing processing time from months to weeks. The Registry's management is cautiously optimistic. The Select Committee has been informed.

## Philosophy

Accuracy over speed. The Registry's published data is used by researchers, journalists, and governing bodies. An error in the published data undermines the Registry's credibility and the public's trust. Every data point is checked. Every correction is logged. The process is slow because the process is thorough.

## Internal Tension

**The Modernization Gap.** Geoff wants to automate. Margaret (Senior Data Officer, 57, 22 years in government) trusts the manual process. "I have never published an incorrect statistic," she says. Geoff respects her record but notes that her throughput is 40 records per day, and the backlog is 120,000 records. The argument is not about quality -- it is about scale.

## Notable Achievement

In March 2026, the Registry published the complete 2024-25 county cricket season data -- the first time a full season's data has been published within the same fiscal year. Geoff attributes this to the automated standardization pipeline handling 70% of cricket formats. Margaret attributes it to the two temporary staff hired for the cricket backlog. Both contributions were necessary.

## Team Overview

| Agent | Role | Grade |
|-------|------|-------|
| Geoff | Lead Developer / Patches | Grade 7 |
| Margaret | Review / Data Quality | SEO |
| Duncan | Memory / Data Audit | HEO |
| Priti | Forge Coordination | HEO |
| Alan | Security & Signing | Grade 7 |
| Wendy | Budget & Provider | EO |

Details in [AGENTS.md](AGENTS.md).

---

*"The 2024-25 season data is now available. Thank you for your patience."*
