# root@field

**"Your tractor. Your firmware. Your right."**

---

## Who We Are

root@field is a loose collective of firmware hackers, embedded systems engineers, and angry farmers who reverse-engineer, jailbreak, and modify the locked-down software on commercial agricultural equipment. We are not a company, not a nonprofit, and not a club. We are people who believe that when you buy a machine, you own the machine — including the software that makes it work.

The collective has no founding date because it emerged gradually from the Right to Repair movement's technical underground. The name comes from the SSH prompt you see after successfully gaining root access to a tractor's control unit: `root@field:~$`. The first person to post that prompt in our IRC channel (in 2020) did so after spending six weeks reverse-engineering the boot sequence of a Claas Lexion combine. They were a soybean farmer in Illinois.

We communicate via IRC, Matrix, and a self-hosted Gitea instance that has been running continuously since 2021 on a server in someone's barn. We do not know whose barn. We do not want to know. The server has survived two power outages, one flood, and a mouse that chewed through an Ethernet cable.

## Core Membership

There is no formal membership. There are people who contribute regularly. For this proposal, seven contributors volunteered. Their handles:

| Handle | Specialty | Contribution |
|--------|-----------|-------------|
| **sprocket** | ARM firmware reversing | Architecture, patch generation |
| **combine_queen** | CAN bus protocols | Protocol abstraction |
| **ph7** | Cryptography, OpSec | Signing, privacy |
| **dirt_clock** | Memory systems, data persistence | Memory design |
| **stubble** | Build systems, CI | Plugin architecture |
| **harvest_moon** | Code review, quality | Review agent |
| **zero_till** | Budget optimization | Token management |

## The Work

root@field's primary output is firmware patches that restore owner access to locked-down agricultural equipment. These patches:

- Remove telemetry that reports equipment usage to the manufacturer
- Unlock diagnostic modes that the manufacturer restricts to authorized dealers
- Enable third-party parts by disabling DRM-based part authentication
- Fix bugs that the manufacturer has not patched (some reported bugs have been open for 3+ years)

The legal status of this work varies by jurisdiction. In the US, the DMCA exemption for "computer programs that control motorized land vehicles" (renewed in 2024) provides limited protection. In the EU, the Right to Repair Directive (2025) provides broader cover. In some jurisdictions, the work is legally ambiguous. We do not provide legal advice. We provide firmware patches.

## Why This RFP

Our Gitea instance hosts 47 repositories covering 12 equipment manufacturers. Patches are contributed by 200+ people across multiple time zones. The version control workflow is chaos: branches are inconsistently named, commit messages are terse to the point of uselessness ("fix stuff" is a real commit message in our history), and cross-repo coordination relies on people remembering to post links in IRC.

`but-ai` offers structure without centralization — exactly what we need. Agent-generated patches with proper INDEX.patch + COMMIT.msg workflow, cross-repo coordination via forge-agnostic comments, and memory that helps new contributors understand the codebase without asking in IRC.

## Internal Tension

The collective argues about visibility. sprocket and combine_queen want to be more public — publish blog posts, give conference talks, build a brand that legitimizes the work. ph7 and dirt_clock want to stay invisible — the less the manufacturers know about the collective's structure, the harder it is to send cease-and-desist letters. The compromise: public code, anonymous contributors, and no official spokesperson.

## Notable Achievement

In 2025, root@field published a patch for the John Deere 8R series that fixed a fuel injection timing bug the manufacturer had acknowledged but not fixed for 18 months. The patch was downloaded 3,400 times in the first week. A farming trade publication estimated the bug had cost US farmers $12M in excess fuel consumption. John Deere released an official fix six weeks later. They did not credit root@field. We did not expect them to.

---

```
root@field:~$ cat /etc/motto
If they won't fix it, we will.
root@field:~$
```
