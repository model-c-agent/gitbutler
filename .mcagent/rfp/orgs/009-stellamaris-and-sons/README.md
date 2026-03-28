# Stellamaris & Sons

**"La nonna controlla tutto."** *(Grandmother checks everything.)*

---

## History

The Stellamaris family has been in shipping since 1923, when Nonno Giuseppe opened a chandlery on the waterfront in Naples. He sold rope, paint, and navigational charts to the fishing boats that worked the Bay of Naples. His son, Marco, expanded into cargo brokerage in the 1960s. Marco's daughter, Lucia, transformed the business into a full logistics operation in the 1990s, handling container bookings, customs clearance, and last-mile delivery across Southern Italy.

Today, the business is run by the fourth generation — Lucia's three children: Elena, Paolo, and Gianni. But Lucia, now 74, still comes to the office every morning at 7:00. She sits at the desk overlooking the dockyards that has been in the family since Giuseppe's time, and she reviews every shipping manifest that passes through Stellamaris & Sons. Every single one.

She does not trust computers. She does not trust algorithms. She trusts paper, ink, and her own eyes. When the company adopted its first digital manifest system in 2018, Lucia insisted that every digital manifest be printed, reviewed by her, and stamped with her personal seal before it was considered final. The company has tried to change this process. Lucia has refused. The compromise: Lucia reviews a random 10% sample of all manifests. She finds errors in approximately 2% of the sample. The digital system's error rate is approximately 2%. Nobody mentions this to Lucia.

## The Family's Path to Software

Paolo, the middle child, studied computer science at Politecnico di Milano before returning to Naples to join the family business. He has spent a decade trying to modernize Stellamaris & Sons without alienating his mother or the company's traditional clients, many of whom still phone in orders and expect a human to answer.

Paolo built the company's internal logistics platform himself — a sprawling Rails application that he calls "beautiful" and everyone else calls "Paolo's monster." It works, mostly. When it doesn't, Paolo fixes it, usually at 2 AM, usually while arguing with Elena on the phone about whether the bug would have been caught if they had proper testing.

The AI agent work began in 2024, when a client asked Stellamaris to automate their container booking process. Paolo built a prototype agent that could read booking requests (emailed in Italian, often grammatically creative), extract the relevant details, and produce booking confirmations. The prototype worked well enough that three more clients requested similar automations.

The version control need arose because Paolo was managing five different client customizations of the same agent, on the same machine, with no branching strategy beyond "I hope I remember which directory is which." He discovered GitButler after the third time he accidentally deployed Client A's configuration to Client B.

## Philosophy

We are a family business. We do not have a philosophy. We have Lucia.

Lucia's operating principle is simple: if you cannot explain a decision to her in one sentence, the decision is wrong. This applies to shipping routes, manifest formats, and now, apparently, to AI agent architectures. When Paolo explained token budgets to Lucia, she said: "So it is like fuel. If you use too much, you do not arrive." Paolo has not found a better explanation since.

## Internal Tension

The family argues constantly, loudly, and in Neapolitan dialect. The central tension: Elena wants to scale the business internationally. Paolo wants to rebuild the technical infrastructure first. Gianni wants to diversify into yacht charter (he has wanted this for fifteen years and nobody has agreed yet). Lucia wants everyone to stop arguing and check the manifests.

These arguments happen at Sunday lunch, in the office, on the phone at midnight, and occasionally at client meetings. Clients who have worked with Stellamaris for more than a year consider this normal. New clients are sometimes alarmed.

## Notable Achievement

In 2025, Stellamaris & Sons processed 180,000 container bookings across 14 shipping lines with a staff of 28 people and no enterprise software beyond Paolo's Rails application and the AI agent prototypes. Their error rate was 0.4%, the lowest among comparable-sized Italian logistics firms. Lucia takes credit for this. Paolo's Rails app takes credit for this. The AI agents take credit for this. The truth probably involves all three.

## Team Overview

Five agents organized as a family. One patriarch agent (Paolo's design, though he would never call it that) makes architectural decisions. One matriarch agent (inspired by Lucia) validates everything before it ships. Two sibling agents handle implementation — they work well together but occasionally produce conflicting approaches to the same problem, which the matriarch resolves by rejecting both and making them start over. One apprentice agent handles routine tasks (memory management, formatting, status reporting) and is being "trained" by the others through example.

---

*"Check the manifest. Then check it again."*
— Lucia Stellamaris, to everyone, always
