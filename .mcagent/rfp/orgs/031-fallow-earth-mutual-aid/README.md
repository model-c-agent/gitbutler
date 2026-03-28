# Fallow Earth Mutual Aid

**"No one owns the soil. Everyone feeds the network."**

---

## Who We Are

Fallow Earth Mutual Aid is a decentralized network of farmers, mechanics, and firmware engineers who share open-source tractor firmware, sensor configurations, and automation scripts. The network has no headquarters, no board, and no formal membership. You join by contributing. You leave by stopping. There are approximately 200 active contributors across 14 countries, though the number fluctuates because nobody counts.

The network coalesced in 2020 during the Right to Repair movement, when farmers in Iowa, Saskatchewan, and Bavaria discovered they were independently reverse-engineering John Deere's tractor firmware to fix equipment they legally owned but could not legally modify. Someone created a shared Git repository. Someone else set up a Matrix channel. A third person wrote a README that said "push what you fix, pull what you need, help when you can." That README is still the closest thing the network has to a charter.

## Why Software Agents

In 2024, a contributor named Hazel Otieno (a Kenyan farmer who also holds a CS degree from the University of Nairobi) proposed using AI agents to translate firmware patches between tractor architectures. The problem: a fix developed for a John Deere 8R in Iowa does not apply directly to a Fendt 942 in Bavaria. The hardware is different, the bus protocols are different, and the memory maps are different. But the underlying bug -- a sensor calibration drift, a timing issue in the hydraulic controller -- is often architecturally identical.

Hazel built a prototype agent that read a firmware patch for one platform, extracted the conceptual fix, and generated an equivalent patch for a different platform. The agent worked for simple patches and failed spectacularly for complex ones. But the concept proved that cross-platform firmware translation was possible, and the network wanted to scale it.

The `but-ai` RFP appeared at the right time. The network needs version control that understands agent workflows, cross-repository coordination (each tractor platform has its own repo), and memory that persists between translation sessions. Five contributors volunteered for the proposal. Nobody was assigned.

## Contributors to This Proposal

| Handle | Location | Focus |
|--------|----------|-------|
| **Hazel** | Kenya | Agent architecture, cross-platform translation |
| **Bjorn** | Sweden | Memory systems, Git internals |
| **Marta** | Poland | Forge abstraction, PR coordination |
| **Diego** | Argentina | Token budgets, resource management |
| **Cam** | Canada | Signing, identity, security |

## Philosophy

1. **Mutual aid, not charity.** We do not help farmers. We are farmers who help each other.
2. **Forks are freedom.** Anyone can fork the firmware, the tools, or this proposal. No permission required.
3. **Working code over governance.** We do not vote on designs. We commit code. If it works, it stays. If it breaks, it gets reverted.

## Internal Tension

The network argues about licensing. Hazel and Bjorn want all firmware patches released under GPLv3, ensuring that modifications remain open. Cam wants a more permissive license (MIT or Apache) to encourage adoption by commercial repair shops that will not touch GPL code. Diego does not care about licenses and wishes everyone would stop talking about them. Marta proposed a dual-license model. Nobody has agreed.

The current practice: contributors choose their own license per patch. The repository has 14 different licenses. It is a mess. It works.

## Notable Achievement

In 2025, a firmware patch originally developed by a contributor in Saskatchewan for a John Deere 8R was translated by Hazel's prototype agent to run on a Massey Ferguson 8S in Kenya. The translated patch fixed a hydraulic valve timing issue that had caused crop loss for two seasons. The Kenyan farmer who applied the patch -- a network member named Joseph -- reported a 15% increase in planting efficiency. This was the first successful cross-continental, cross-platform firmware translation in the network's history.

---

*No one signed this. Everyone contributed. Push what you fix.*
