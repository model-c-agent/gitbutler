# Ghost Line Collective

**"GTFS is public data. We just read it more carefully than they want us to."**

---

## Origin

Ghost Line Collective formed in 2021 on a Signal group called "transit-equity-data." The founding members — all anonymous, all working in or adjacent to public transit — had been independently analyzing GTFS (General Transit Feed Specification) data from US transit agencies and finding patterns that agencies did not publicize: service frequency declining in low-income neighborhoods while increasing in gentrifying areas, scheduled "ghost runs" (buses that appear in the GTFS feed but never actually operate), and stop closures that disproportionately affected communities of color.

The collective's first publication — a data analysis showing that a major Midwestern transit agency's GTFS feed included 47 bus runs per day that had never once operated — was picked up by local media. The agency's response was to mark those runs as "non-revenue" in the next feed update, which was technically accurate but missed the point: the runs were listed in the passenger-facing schedule, and riders were waiting for buses that would never come.

Ghost Line now monitors GTFS feeds from 80 US transit agencies. They publish quarterly "equity audits" comparing scheduled service against actual service (measured via real-time vehicle position data, also public), broken down by neighborhood demographics. Their analyses have been cited in three federal civil rights complaints and one Department of Justice investigation.

The collective has no legal entity. No office. No website (clearnet, anyway). They publish on a Tor-hosted blog and communicate via encrypted channels. Five core members handle the software. The rest contribute data analysis and domain expertise.

## Philosophy

Data is accountability. When a transit agency publishes a GTFS feed that says a bus will arrive at 8:07 AM, that is a commitment. When the bus does not arrive, the data records the failure. When the failure pattern correlates with neighborhood demographics, the data tells a story that the agency would rather not tell. Ghost Line believes in reading the data the way the data wants to be read — not the way the agency wants it presented.

They approach AI agents with the same adversarial lens: agents should be transparent about their failures, not just their successes. An agent that hides error rates, suppresses partial results, or smooths over inconsistencies is an agent that enables the same data dishonesty that Ghost Line exists to expose.

## Internal Tension

The collective argues about engagement. `fare_zero` (the technical coordinator) believes Ghost Line should remain purely adversarial — analyze, publish, shame. `route_null` argues that adversarial analysis without constructive engagement just gets agencies defensive. `route_null` has privately provided technical assistance to two smaller transit agencies that wanted to improve their equity metrics. `fare_zero` considers this collaboration with the system they are supposed to critique. The argument is unresolved.

## Achievement

In 2025, Ghost Line's analysis of the Philadelphia SEPTA GTFS feed revealed that scheduled headways on three routes serving predominantly Black neighborhoods had been quietly increased from 12 minutes to 20 minutes — a 67% reduction in service frequency — without public notice or comment period. The analysis was published on their blog, picked up by the Philadelphia Inquirer, and resulted in SEPTA restoring the original headways within 30 days. SEPTA's official statement attributed the headway change to "a scheduling optimization error." Ghost Line's analysis showed the change had persisted for 14 months.

## Team

| Handle | Role | Known Details |
|--------|------|---------------|
| fare_zero | Coordinator / Architecture | US timezone, systems engineering |
| route_null | Data Analysis / Outreach | Eastern US, transit planning background |
| headway_ghost | Backend Engineering | European timezone, GTFS specialist |
| stops_end | Security / Infrastructure | Unknown, Tor/infra expert |
| transfer_void | ML / Equity Modeling | US timezone, data science |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes a "ghost count" — the number of phantom bus runs detected across all monitored agencies in the current month. Example: `fix: GTFS-RT position parser [Ghosts this month: 1,247]`. The number is always depressingly high.

---

*"The schedule says the bus is coming. The data says it is not. Believe the data."*
