# sudo rm -rf /sprawl

**"Your zoning data is public. We just made it searchable."**

---

## Origin Story

`sudo rm -rf /sprawl` (stylized lowercase, no capital letters, ever) started as a Twitter bot in 2021. The bot scraped county assessor records, geocoded them, and tweeted maps showing the correlation between single-family zoning and racial demographics in US metropolitan areas. The maps were devastating — and because the data was entirely public record, no one could dispute them.

The bot was built by two people who have never met in person and communicate exclusively through Signal. They use handles, not names. `parcel` is a former GIS analyst who was fired from a county planning department for publicly criticizing the county's exclusionary zoning practices. `setback` is a data engineer who works at a FAANG company during the day and reverse-engineers property records at night.

The bot went viral three times: once when it mapped every majority-white suburb in the Atlanta metro with minimum lot sizes above half an acre, once when it cross-referenced school district boundaries with zoning density caps to show how exclusionary zoning concentrates poverty in specific school districts, and once when a state legislator screenshotted the bot's output during a committee hearing on housing reform.

By 2022, `parcel` and `setback` had been joined by three others: `easement` (a housing activist and scraper specialist), `plat` (a cartographer who designs the visualizations), and `variance` (a security researcher who handles the collective's OpSec). They incorporated as nothing. They have no legal entity, no bank account, and no office. They operate through cryptocurrency donations, borrowed cloud compute, and a shared Signal group.

## Philosophy

Public data should be public. That sounds obvious. It is not. County assessor records are technically public, but they are stored in systems designed to prevent bulk access — CAPTCHAs, rate limits, pagination traps, session timeouts. The data is "public" the way a book on the top shelf of a locked library is "available." You can see it, but you cannot reach it without tools.

The collective builds the tools. They scrape, clean, geocode, and publish property records, zoning maps, and land use data in machine-readable formats. They do not editorialize (much). They let the maps speak.

Their position on AI agents: agents are tools for making public data more accessible. An agent that can cross-reference three county databases in ten minutes replaces a researcher who would spend three weeks doing the same work manually. The agent does not add insight — it removes friction.

## Why This RFP

In 2025, the collective started using agents to automate the most labor-intensive part of their pipeline: parsing zoning code PDFs. Every municipality publishes its zoning code as a PDF, and every PDF is formatted differently. The agents parse the PDFs, extract dimensional standards (setbacks, height limits, FAR, lot sizes), and produce structured data that feeds the mapping pipeline.

The agents needed coordination. Different agents processed different municipalities, and their outputs needed to merge into a unified dataset. The collective was using raw Git with manual merges, which worked until `easement` accidentally force-pushed over two weeks of `parcel`'s work. The subsequent argument (in Signal, at 3 AM) led `setback` to evaluate GitButler, and the `but-ai` RFP arrived at the right moment.

## Team

Five members. No leader. No hierarchy. Pseudonyms only.

| Handle | Role | Specialty |
|--------|------|-----------|
| parcel | Patch Architect | Zoning code parsing, GIS analysis |
| setback | Provider & Budget | Data engineering, infrastructure, cost |
| easement | Memory Systems | Web scraping, data cleaning, caching |
| plat | Forge Adapter / Coordination | Cartography, visualization, public comms |
| variance | Security & Signing | OpSec, key management, anonymity |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Attribution Problem.** `plat` wants the collective's work to be more visible — published under a recognizable brand, presented at conferences, cited in academic papers. `variance` is categorically opposed. Visibility means attack surface. The collective has received legal threats from two municipalities whose exclusionary zoning practices were exposed by the bot. `variance` argues that anonymity is the collective's primary defense. `plat` argues that anonymity limits impact. The compromise is unstable: the bot is public, the members are not.

## Achievement

In 2024, the collective's dataset was used as evidence in a federal fair housing complaint filed against a suburban county in Georgia. The complaint alleged that the county's minimum lot size requirements (one acre in predominantly white neighborhoods, no minimum in predominantly Black neighborhoods) constituted de facto racial zoning. The case is ongoing. The collective's data was entered into the record as Exhibit 14. `parcel` keeps a screenshot of the exhibit list as their Signal profile picture.

---

*"$ cat /dev/zoning | grep exclusion"*
