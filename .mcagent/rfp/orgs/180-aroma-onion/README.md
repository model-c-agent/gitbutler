# aroma.onion

**"Your luxury perfume is 3 cents of linalool and $200 of marketing. We publish the proof."**

---

## Who We Are

aroma.onion is an anonymous collective that reverse-engineers luxury perfumes using gas chromatography-mass spectrometry data. We obtain commercial fragrances, analyse them on GC-MS instruments (our own, and at sympathetic university labs), and publish the results: full compound identifications, approximate concentrations, and estimated raw material costs. We publish on a .onion site (Tor hidden service), a public mirror, and a Matrix channel.

The collective started in 2020, born from a Reddit thread on r/fragrance where a user posted a GC-MS chromatogram of Chanel No. 5 and asked "can someone identify these peaks?" Three people could. They did. The post was deleted by moderators within hours after a DMCA takedown notice. The three GC-MS readers found each other in a private chat and decided that if the fragrance industry was going to use legal threats to suppress publicly verifiable chemical data, they would make the data uncensurable.

We operate under handles. `benzyl` is the GC-MS operator — a chemistry PhD who works at a university lab and runs samples on weekends. `aldehyde` is the data analyst who matches peaks to compound libraries. `ester` is the web developer who maintains the .onion site and the public mirror. `ketone` is the writer who composes the analyses into readable reports. `thiol` is the security specialist who keeps everyone anonymous.

We have published 87 analyses. We have received 14 cease-and-desist letters. We have complied with zero of them. Our legal position: we are publishing factual chemical data about commercially available products. This is no different from a nutritional label — except the fragrance industry fights nutritional labels too.

## Philosophy

Secrecy in perfumery is a business strategy, not a technical necessity. A formula published does not lose its beauty. A formula published loses its monopoly. We believe consumers have the right to know what they are putting on their skin, and perfumers have the right to learn from existing work. The fashion industry does not copyright clothing designs. The food industry publishes ingredient lists. The fragrance industry's secrecy is an anomaly, and we are the correction.

## Why This RFP

In 2025, our analysis pipeline became a bottleneck. `benzyl` can run one sample per weekend. `aldehyde` can identify approximately 50 peaks per day. A complex fragrance has 200+ peaks. We started using AI agents to accelerate peak identification — an agent reads the mass spectrum of a peak and proposes an identification from a compound library.

The agents worked well individually but conflicted when two agents identified the same peak differently (a common scenario with co-eluting compounds). We needed version control that could hold competing identifications as parallel hypotheses. GitButler's virtual branches mapped onto this perfectly: each agent's identifications live on a virtual branch, and conflicts are resolved by the human analysts.

## Team

Five handles. No names. Tor-only communication.

| Handle | Role | Specialty |
|--------|------|-----------|
| benzyl | GC-MS Operator / Patch Architect | Sample preparation, chromatography |
| aldehyde | Data Analyst / Memory & Compound DB | Peak identification, spectral matching |
| ester | Web Dev / Forge Adapter | .onion infrastructure, public mirror |
| ketone | Writer / Provider & Budget | Analysis reports, cost calculations |
| thiol | Security / Signing & OpSec | Anonymity, key management, legal defense |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Publication Speed Debate.** `ketone` wants to publish analyses faster — the collective's audience is growing, and demand for new analyses outpaces supply. `benzyl` insists on triple-checking every identification before publication. "A wrong identification discredits the entire project," they argue. "Nobody trusts the second analysis from a lab that got the first one wrong." `ketone` counters that perfectionism is a luxury the collective cannot afford when operating on donated lab time. The compromise: analyses are published as "preliminary" (agent-generated, human-reviewed but not triple-checked) with a later "confirmed" update.

## Achievement

In 2024, aroma.onion's analysis of a luxury perfume marketed as "100% natural" revealed that 60% of the formulation by weight was synthetic. The analysis was picked up by a consumer advocacy organization and cited in a formal complaint to the French DGCCRF (consumer fraud authority). The investigation is ongoing. `ketone` wrote the analysis report. `benzyl` ran the sample three times to be certain. `thiol` ensured the publication could not be traced back to any individual. The cease-and-desist letter arrived within 48 hours. It was framed (digitally) and posted on the .onion site's "Wall of Shame."

---

*"GC-MS does not lie. Neither do we."*
