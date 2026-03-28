# zer0day_ledger

**`// we read the receipts. the ones they deleted.`**

---

## what this is

zer0day_ledger is a loosely affiliated group of data analysts, OSINT researchers, and reformed black-hat hackers who correlate leaked financial data with public corporate filings to identify fraud, tax evasion, and sanctions violations. we operate pseudonymously. we publish our findings anonymously. we have been doing this since 2019.

we do not hack systems. we analyze data that is already public or has already been leaked by others. the distinction is legally meaningful. we are information analysts, not intruders. our inputs are SEC filings, Companies House records, leaked banking data from journalistic sources, and open corporate registries. our outputs are correlation reports that we publish on our .onion site and share with investigative journalists.

we have contributed to 7 major investigative journalism reports, including a 2024 story in the Guardian about sanctions evasion through shell companies in the Baltics. we are not credited by name. we do not want to be.

## how we work

someone uploads a dataset to our intake. the dataset could be a leaked bank transaction log, a scraped corporate registry, or a collection of public filings. our agents process the dataset: parse the records, extract entity names and transaction details, and cross-reference them against our existing database of known entities.

the database is 23 million entity records from public sources. the cross-referencing is the valuable part -- when a name in a leaked dataset matches a director of a company in our public registry data, that's a lead. when 40 names match, that's a story.

the pipeline produces INDEX.patch files that add correlation findings to a case branch. each finding includes the source data references, the matched entities, and the confidence level. we publish findings above 85% confidence. below that, we hold and wait for corroborating data.

## the stack

- **intake**: tor-hosted upload portal, PGP-encrypted submissions
- **processing**: AI agents running on air-gapped hardware, no cloud
- **storage**: Git repos, encrypted at rest, replicated across 3 locations
- **publication**: .onion site, static HTML, updated via Git push
- **communication**: IRC (OFTC) + PGP-encrypted email

everything is air-gapped or tor-routed. we take operational security seriously because the entities we investigate take retaliation seriously.

## why but-ai

our pipeline is a mess of Python scripts that produce Git commits directly. merge conflicts between agents are a daily occurrence. signing is done with PGP keys that we manage manually. cross-referencing between datasets happens through ad-hoc scripts that break when data formats change.

`but-ai` gives us a formal framework for the pipeline we already run. the INDEX.patch + COMMIT.msg model matches our workflow exactly. virtual branches eliminate merge conflicts. OpenWallet gives us a signing infrastructure that does not require manually managing PGP keys.

## philosophy

### on transparency

the powerful hide behind opacity. shell companies exist to obscure ownership. offshore accounts exist to hide wealth. complex corporate structures exist to prevent anyone from seeing the full picture. we make the picture visible by connecting the dots that were designed to stay disconnected.

### on anonymity

we protect our identities because the entities we investigate have resources to retaliate. this is not paranoia. two OSINT researchers in our broader community have faced legal threats from subjects of their analysis. we learn from their experience.

### on legality

we analyze public and previously published data. we do not access systems without authorization. we do not pay for stolen data. when someone uploads data to our intake, we verify that it has been previously published by a journalistic source before we process it. this policy has cost us leads. it has also kept us out of court.

## tension

**the publication threshold.** `zk_proof` (our senior analyst) wants to lower the publication threshold from 85% to 75% confidence, arguing that more findings published means more leads for journalists. `bit_rot` (our opsec lead) insists on 85% because publishing a wrong correlation could expose an innocent person and undermine the collective's credibility. "We get one chance to be wrong before nobody trusts us." The threshold stays at 85%. `zk_proof` revisits the argument quarterly.

## achievement

in 2025, our correlation analysis contributed to an ICIJ investigation into a sanctions evasion network that used Baltic shell companies to move approximately EUR 800 million on behalf of sanctioned individuals. our agents identified 14 previously unknown shell companies by correlating leaked banking data (published by a Latvian news outlet) with Estonian and Lithuanian corporate registries. the ICIJ published the investigation in November 2025. fourteen jurisdictions opened inquiries. we were not named.

## team

| handle | role | focus |
|--------|------|-------|
| `zk_proof` | lead analyst | INDEX.patch, correlations, findings |
| `bit_rot` | opsec lead | signing, key management, identity |
| `pkt_loss` | data pipeline | provider abstraction, intake processing |
| `eof` | coordination | forge adapters, publication pipeline |

profiles in [AGENTS.md](AGENTS.md).

## working style

asynchronous. pseudonymous. air-gapped. communication via IRC and PGP email. decisions by lazy consensus (48 hours). no meetings. no video calls. no voice. text only. always encrypted.

---

```
// the receipts exist. they always exist.
// the question is who has the patience to read them.
```
