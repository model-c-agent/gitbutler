# The Pilgrims' Route Cooperative

**"The road serves everyone. So do we."**

---

## Origin

The Pilgrims' Route Cooperative started in 2015 with a minivan and a problem. Pastor Grace Muthoni in Nairobi's Kibera settlement noticed that elderly congregants were missing Sunday services — not from lack of faith, but from lack of transport. Matatu routes did not reach the settlement's interior, and the walk was too far for aging knees. Grace borrowed a 12-seater van from a parishioner, printed a schedule on paper, and began running a free shuttle between Kibera and three churches every Sunday morning.

Within a month, the shuttle was full. Within three months, other faith communities asked to be included: a mosque on Ngong Road, a Hindu temple near the city center, a Sikh gurdwara in Westlands. Grace added stops. The schedule grew. The parishioner donated the van permanently because, she said, "it is doing more good on the road than in my driveway."

By 2018, the Pilgrims' Route operated seven vehicles across Nairobi, serving religious sites, food banks, medical clinics, and elder care facilities. Funded entirely by donations from religious communities, it charged no fares and turned no one away. Grace incorporated as a cooperative under Kenyan law, with a board composed of representatives from each participating faith community.

The software came later. In 2020, Grace's nephew Edwin Ochieng — a software engineer at Safaricom — built a scheduling and dispatch system on weekends because the paper schedules could not handle the route complexity. Edwin's friend Amina Yusuf (data engineer, ex-Jumia) joined to build the demand prediction model. They were later joined by Brother Thomas Kiptoo (a Franciscan friar with a computer science degree from Strathmore University) and Leah Wanjiku (a UX designer who volunteered because her grandmother used the shuttle).

## Philosophy

Service is the purpose. Technology is the vehicle. The Cooperative exists to move people who cannot move themselves — to worship, to eat, to receive care. Every technical decision is evaluated against a single question: does this serve the riders? If a feature does not serve the riders, it does not ship.

They approach AI agents with the same lens: agents exist to serve the human operator, not the other way around. An agent that requires human attention to manage is an agent that has inverted the service relationship. Agents should be as invisible as a well-run shuttle — you notice the destination, not the vehicle.

## Internal Tension

Grace and Edwin disagree about growth. Grace wants to expand to Mombasa and Kisumu — there is demand from faith communities in both cities. Edwin argues that the software is not ready for multi-city operations and that expanding before the platform is stable will degrade service in Nairobi. Brother Thomas sides with Grace (service is urgent). Amina sides with Edwin (infrastructure before expansion). The board has deferred the decision twice, which in practice means Nairobi remains the only city.

## Achievement

During the 2024 Nairobi floods, the Pilgrims' Route was the only transit service operating in three of the worst-affected neighborhoods. Edwin's dispatch system, originally designed for religious service schedules, was repurposed in 48 hours to coordinate food and medical supply deliveries to 14 distribution points. The cooperative moved 2,200 people and 8 tons of supplies in 72 hours. Grace said it was "what the van was always for." The UN Habitat office in Nairobi cited the Cooperative's flood response as a model for community-based disaster logistics.

## Team

| Name | Role | Background |
|------|------|------------|
| Pastor Grace Muthoni | Director / Vision | Founded the cooperative, runs operations |
| Edwin Ochieng | Lead Engineer | Ex-Safaricom, scheduling systems |
| Amina Yusuf | Data Engineer | Ex-Jumia, demand prediction |
| Br. Thomas Kiptoo, OFM | Backend / Ethics | Franciscan friar, Strathmore CS degree |
| Leah Wanjiku | UX / Community | Volunteer, grandmother is a rider |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message ends with a rider count — the total number of people the cooperative has transported since inception. The number updates monthly. Example: `fix: schedule overlap on Ngong Road route [riders: 184,207]`. The team says it reminds them who they are building for.

---

*"The van leaves at 7. Everyone is welcome."*
