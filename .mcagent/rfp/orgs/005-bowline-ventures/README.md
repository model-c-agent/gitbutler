# Bowline Ventures

**"Move fast and ship containers."**

---

## The Pitch

We're two years old, twelve people, burning through our Series A, and we've already processed more bills of lading than most logistics companies handle in a decade. Our product is a blockchain-verified manifest system that replaces the paperwork nightmare of international container shipping with cryptographically signed digital documents.

Founded in 2024 by three ex-Maersk operations analysts and a crypto protocol designer who met at a logistics hackathon in Singapore. The hackathon challenge was "reduce container dwell time by 50%." Our team's solution — drone-verified container seals linked to on-chain manifests — won first place and attracted $4.2M in seed funding before we'd even incorporated.

We incorporated in Singapore (because that's where the hackathon was and we were too excited to fly home first), hired aggressively, and shipped our MVP in four months. The MVP was rough. It crashed every third day. But it worked: container dwell time at our pilot terminal dropped 41%, not quite 50% but close enough to make our investors smile.

## How We Got Here

We didn't plan to build AI agents. We planned to build a better manifest system. But our customers kept asking: "Can the system automatically flag discrepancies between the digital manifest and the physical container?" That's an agent problem. The system needs to observe (drone camera feeds, sensor data, manifest records), reason (does this container match its manifest?), and act (flag discrepancy, notify operator, update manifest).

We built agents. They were terrible at first — hallucinating manifest numbers, confusing TEU counts, once flagging a container of bananas as hazardous materials because the yellow color triggered a false positive in our visual classifier. But we iterated fast. By Q3 2024 we had agents that could process 500 containers/hour with 99.2% accuracy.

The version control problem hit us when we scaled to multiple terminals. Agents at different ports were updating manifests concurrently. We needed branch-per-terminal isolation with cross-terminal coordination. That's GitButler's sweet spot.

## Philosophy

Speed is a feature. Not reckless speed — *informed* speed. We ship daily. We break things weekly. We fix them the same day. Our agents operate the same way: act fast, validate immediately, rollback if wrong. We'd rather ship a patch that needs one revision than spend three days making it perfect.

We know this makes traditional logistics companies nervous. Good. The industry has been shipping paper manifests by fax for thirty years. Someone needed to be impatient.

## Internal Tension

Our CTO (the crypto protocol designer) wants everything on-chain. Our head of operations (ex-Maersk, 15 years of container terminals) thinks blockchain is a solution looking for a problem and that a signed database is fine. They argue about this approximately twice per week. The company's architecture reflects the compromise: manifests are on-chain, everything else is off-chain, and both sides claim victory.

## Notable Achievement

In Q1 2025, we processed 2.1 million digital manifests across 8 terminals in Southeast Asia. Zero paper. Zero fax machines. The system detected 14,000 manifest discrepancies that would have been caught days later (or never) under the old system. Our flagship customer's insurance premiums dropped 12% because their cargo documentation accuracy improved from 94% to 99.7%.

## Team Overview

Eight agents organized as a startup team. One product agent sets priorities and decomposes work. Two full-stack agents handle most implementation. One infrastructure agent manages deployments and signing. One QA agent runs validation. Three specialist agents rotate between provider integration, memory optimization, and forge coordination depending on sprint priorities. Coordination is informal — Slack-style async messages, no ceremonies, ship when ready.

---

*"If you're still using paper, we're already faster than you."*
