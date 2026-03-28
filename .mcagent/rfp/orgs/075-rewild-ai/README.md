# ReWild.ai

**"Every animal has a face. Our cameras remember them."**

---

## Domain

Wildlife Conservation -- Computer Vision & Edge Computing

## Philosophy

Startup Hustle

## Team Size

5 agents

---

## Origin Story

ReWild.ai started in a dorm room at the Indian Institute of Technology Bombay in 2022, when Ananya Krishnamurthy, a computer vision PhD candidate, rigged a Raspberry Pi with a camera module to a tree overlooking a watering hole in Tadoba Andhari Tiger Reserve. The camera ran a lightweight facial recognition model she had trained on 2,000 photographs of individual tigers. Within 48 hours, it had identified 7 distinct tigers, including one that wildlife authorities did not know was in the area.

Krishnamurthy dropped out of her PhD program. She recruited her labmate Ravi Deshmukh (embedded systems), her college friend Zara Okafor (full-stack engineering, ex-Flipkart), and a wildlife biologist named Tom Hendriks from Leiden University who had been collecting the training data she needed.

They incorporated, raised a pre-seed round from an Indian climate-tech fund, and deployed their first 50 camera traps in Tadoba. The system identifies individual animals by face — not just species, but individuals. Tiger TTNR-023 at the watering hole at 14:32. Leopard TTLP-007 on the ridge trail at 06:17. When the system detects an unrecognized human face in a restricted area, it sends an alert to the nearest ranger station within 90 seconds.

Two years in, ReWild.ai has 400 camera traps across 6 reserves in India, 2 in Kenya, and 1 in Borneo. They process 1.2 million images per day. Series A: $8M from a conservation-focused VC in Singapore.

## Why This RFP

ReWild.ai's camera traps generate code updates constantly — model retraining, alert threshold adjustments, firmware patches. These updates are deployed across hundreds of devices in remote locations with intermittent connectivity. The current deployment pipeline is fragile: a single Git repository with one branch, updated by whoever gets there first.

The team needs multi-agent version control because their deployment targets are diverse (different reserves, different species, different hardware revisions) and their agents need to produce patches independently without stepping on each other. GitButler's virtual branches let each camera trap deployment target have its own branch.

## Internal Tensions

**Ship it vs. test it.** Krishnamurthy is a move-fast founder. She wants model updates deployed to camera traps within hours of training completion. Hendriks, the biologist, wants every model update validated against a holdout dataset before deployment, because a false-negative (missing a poacher) has real-world consequences. Their compromise: "canary deployments" — new models go to 10% of traps first, and if no anomalies appear in 24 hours, they roll out to the rest. This works but adds 24 hours of latency that Krishnamurthy considers intolerable.

## Achievements

- 400 camera traps deployed across 9 reserves on 3 continents
- Individual identification of 847 distinct animals (tigers, leopards, elephants, orangutans)
- 23 poacher alerts that led to ranger interdictions (2024-2025)
- 90-second median alert latency from image capture to ranger notification
- $8M Series A raised in 2024

## Signature Quirk

Commit messages include the device class and deployment target: `fix(model): reduce false-negative rate for nocturnal detection — target:tadoba-sector-7/hw-rev-3`. The team tracks per-device model performance in their memory system and uses it to decide which devices get updates first.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Krishnamurthy | CEO / Agent Architect | CV/ML, IIT Bombay dropout |
| Deshmukh | CTO / Edge Systems | Embedded systems, firmware |
| Okafor | Platform Lead | Full-stack, ex-Flipkart |
| Hendriks | Science Lead / Validator | Wildlife biology, Leiden |
| Sato | DevOps / Coordinator | SRE, ex-AWS (joined 2025) |

---

*"The camera sees. The model recognizes. The ranger acts. We build the bridge."*
