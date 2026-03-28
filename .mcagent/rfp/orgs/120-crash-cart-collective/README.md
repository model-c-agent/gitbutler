# Crash Cart Collective

**"If the manufacturer won't fix it, we will."**

---

## Origin

The Crash Cart Collective jailbreaks medical devices. Defibrillators, specifically.

It started in 2021 when a biomedical engineer working in a rural Kenyan hospital discovered that the hospital's donated defibrillators — surplus units from a US hospital chain — had been firmware-locked to prevent third-party maintenance. The lock was a deliberate design choice by the manufacturer: if the device detected non-OEM replacement parts (which were all that was available in rural Kenya), it refused to charge. A life-saving device, rendered useless by DRM.

The engineer, who goes by `sparks` online, reverse-engineered the firmware over three weekends and published a patch that bypassed the parts-detection lock. The patch spread through medical equipment forums. Within six months, forty-three hospitals in twelve countries had applied it. The manufacturer sent a cease-and-desist. `sparks` ignored it.

The collective formed around `sparks`: five biomedical engineers, firmware hackers, and medical device security researchers who believe that DRM on medical devices is morally indefensible and that the technical skills used to jailbreak phones can be used to save lives.

They do not only work on defibrillators. They have produced firmware patches for locked ventilators, infusion pumps with artificially restricted flow rate ranges, and a pulse oximeter that required a $500/year cloud subscription to display SpO2 readings — a device used in clinics that have no reliable internet.

Their work is legally ambiguous. In many jurisdictions, circumventing firmware locks violates anti-circumvention statutes regardless of intent. The collective operates pseudonymously, communicates through encrypted channels, and publishes patches on anonymous hosting. They consider this precaution, not paranoia: a member was sued in 2023 (case dismissed; the manufacturer declined to pursue discovery after the collective threatened to publish the firmware source code in court filings).

## Philosophy

**If it saves lives, ship it. If it is locked, unlock it.** The collective applies this to AI agents with the same directness: agent systems should never have artificial limitations that prevent useful operation. No vendor lock-in. No cloud dependencies. No subscription-gated features. Everything runs locally, everything is auditable, and everything can be modified by the user.

## The Tension

`sparks` and `flatline` (the collective's security researcher) disagree about disclosure. `sparks` believes all firmware vulnerabilities should be published immediately — full disclosure forces manufacturers to fix issues faster. `flatline` argues for coordinated disclosure with a 90-day window — "If we publish a vulnerability in a ventilator and someone weaponizes it before the manufacturer patches it, that's on us." The collective uses coordinated disclosure for safety-critical vulnerabilities and full disclosure for DRM locks. This satisfies no one completely.

## Notable Achievement

In 2025, the collective published a patch for a widely deployed defibrillator model that unlocked pediatric mode — a capability physically present in the hardware but firmware-gated behind a $3,000 license upgrade. The patch was downloaded 12,000 times in the first month. An emergency physician in Guatemala reported using the unlocked pediatric mode to defibrillate a 6-year-old in cardiac arrest. The child survived. The manufacturer released the pediatric mode for free three months later, citing "changing market conditions."

## Team

Five pseudonymous members. No leader. Rough consensus via encrypted group chat.

| Handle | Role | Focus |
|--------|------|-------|
| sparks | Firmware / Patch Lead | Reverse engineering, INDEX.patch generation |
| flatline | Security Research | Vulnerability analysis, signing, key management |
| paddles | Systems | Provider abstraction, local-first infrastructure |
| rhythm | Data / Memory | Agent memory, pattern databases, device catalogs |
| joules | Coordination | Cross-repo PR management, forge adapters |

Detailed profiles in [AGENTS.md](AGENTS.md).

## Working Style

Asynchronous. Encrypted Signal group for real-time coordination, self-hosted Gitea for code, no centralized identity. Members use Tor for Gitea access. Patches are reviewed by minimum two members. All commits are signed with pseudonymous GPG keys. The collective meets physically once a year at a security conference, always under assumed names.

---

*No attribution. No credit. Check the patch.*
