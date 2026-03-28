# /dev/drape

**"0x434F555455524500 — couture in hex."**

---

## origin story

we don't have a founding date. someone registered the IRC channel in 2018. someone else pushed the first commit to the `libdrape` repo in 2019. by 2020 there were nine of us and a manifesto: *all garment patterns should be open source. proprietary fashion is a bug.*

the core premise: 3D body scanning is cheap now. a $200 depth sensor and our open-source `scan2pattern` pipeline can generate a custom-fit garment pattern from a body scan in under 30 seconds. the pattern is a parameterized SVG. you print it, cut it, sew it. couture for anyone with a printer and a sewing machine.

the fashion industry hates us. we've received three cease-and-desist letters from luxury houses claiming our reverse-engineered pattern algorithms infringe on their "trade dress." our lawyer (who we pay in patches to her home automation system) says they have no case. we publish anyway.

## the collective

no real names. handles only. we communicate on a self-hosted Matrix server and an IRC bridge for the old-timers. meetings happen on the `#drape-standup` channel at 2300 UTC — chosen because it's inconvenient for everyone equally.

membership is contribution-based. push ten merged patches to any `/dev/drape` repo and you're in. there's no application process, no interview, no org chart. the repos have CODEOWNERS files and that's the closest thing to hierarchy.

seven active contributors. dozens of drive-by patchers. the core seven do 90% of the work.

## why but-ai

we've been using git since forever. we tried gitbutler in 2025 because virtual branches map perfectly to our workflow: multiple contributors hacking on the same pattern engine simultaneously, each in their own branch, merging when ready.

`but-ai` interests us because we want to automate the boring parts of pattern generation. the creative work is the 3D-to-2D projection algorithm. the boring work is grading (scaling patterns across sizes), seam allowance calculation, and fabric layout optimization. AI agents can handle that. we'll review the patches and merge what works.

also: we want agents that are as open and auditable as our code. no black boxes. signed commits. verifiable memory. if an agent contributes to our project, its work must be as transparent as any human contributor's.

## philosophy

code is pattern. pattern is code. a garment pattern is a program that transforms flat fabric into a 3D object. a software patch is a program that transforms one codebase state into another. the operations are isomorphic and we treat them identically.

## internal tension

**the licensing war.** `zh0st` wants all `/dev/drape` repos to be AGPL — if anyone uses our patterns in a commercial product, they must open-source their modifications. `patchwerk` argues for MIT — maximum freedom, even if corporations freeload. the argument has been running for three years. current compromise: `libdrape` is AGPL, utility tools are MIT. nobody is satisfied.

## notable achievement

`scan2pattern` v3.0 (2025) can generate a full custom-fit shirt pattern from a depth scan in 22 seconds on consumer hardware. the pattern includes seam allowances, grain line markers, and notch positions. twelve community sewing spaces in Berlin, Amsterdam, and Detroit have deployed it. estimated 400 garments produced from our patterns in 2025. zero revenue. maximum impact.

## team overview

| handle | role | timezone |
|--------|------|----------|
| zh0st | lead maintainer / signing | UTC+1 |
| patchwerk | patch generation | UTC-5 |
| seam_ripper | review / quality | UTC+8 |
| nullstitch | memory architecture | UTC+3 |
| bobbin | forge coordination | UTC-8 |
| selvage | provider abstraction | UTC+0 |
| grainline | budget / token mgmt | UTC+5:30 |

details in [AGENTS.md](AGENTS.md).

---

*`// TODO: liberate all patterns`*
