# /gg/noRe

**"We read the binary so you don't have to."**

---

## What We Are

/gg/noRe is a loose collective of reverse engineers, protocol analysts, and security researchers who focus exclusively on competitive gaming infrastructure. We have no legal entity. We have no office. We have a Matrix server, a shared Git hosting instance (self-hosted Forgejo, obviously), and a reputation for publishing technical analyses that game developers would prefer stayed private.

The name comes from the gaming abbreviation "gg no re" — "good game, no rematch." It means the matter is settled. When /gg/noRe publishes an analysis, the analysis is the final word. We do not engage in public debate. We publish findings, provide evidence, and move on.

The collective formed organically in 2021 on a forum thread about matchmaking algorithms. Several participants independently discovered that a major competitive title was using a engagement-optimized matchmaking system (EOMM) instead of the skill-based matchmaking (SBMM) it claimed. The system was designed to maximize play time, not match quality — it deliberately placed players in losing streaks followed by easy wins to create an addictive frustration-relief cycle. The participants compiled their evidence, published it, and the game developer denied everything, then quietly changed the algorithm six months later.

That thread became a collective. The founding members — handles only, no real names — are: `nullref`, `packetsniff`, `sigreturn`, `heapspray`, and `race_condition`. We have grown to about fifteen active contributors, but the core five do most of the work.

## Why This RFP

Our analysis workflow is already Git-based. We decompile game clients, annotate the disassembly, document protocol structures, and publish findings as structured reports. Each report is a set of Markdown files with embedded code blocks showing the relevant decompiled functions. The repository history is the chain of evidence — every finding traces back to a specific binary version, a specific memory offset, and a specific build hash.

We started building analysis agents in 2025 because manual reverse engineering is slow. A single matchmaking analysis takes 80-120 hours of human effort. We built agents that can ingest decompiled output, identify matchmaking-relevant code paths, and annotate them with behavioral hypotheses. The agents are not replacing human analysts — they are triaging: they identify the interesting code paths so humans can focus on interpretation.

The agents produce INDEX.patch files that add annotations to our decompiled source. They produce COMMIT.msg entries that explain the annotation and cite the binary evidence. We need `but-ai` because our current ad-hoc agent tooling does not scale — we are running into merge conflicts between the annotation agent and the protocol analysis agent, and our signing story is nonexistent (a problem when your findings are legally sensitive).

## Philosophy

Transparency through evidence. Every claim we make is backed by a reproducible analysis. "We decompiled build X.Y.Z, found function at offset 0xABCDEF, and the logic does this." No speculation. No unnamed sources. No vibes. Agents are held to the same standard — an agent annotation that cannot cite its evidence is deleted.

We do not trust black boxes. We run local models exclusively (Ollama, llama.cpp). We do not send decompiled code to cloud APIs. Ever. The legal risk is obvious; the ethical position is simpler: we are not giving anyone else access to our analysis pipeline.

## The DMCA Scare

In September 2025, a game publisher sent a DMCA takedown notice to our Forgejo instance for a report that included decompiled matchmaking code. Our hosting provider (a sympathetic sysadmin in Iceland) forwarded the notice without action. We evaluated the claim, determined it was legally baseless under the reverse engineering provisions of the DMCA (17 U.S.C. Section 1201(f)), and published our legal analysis alongside a response. The publisher did not follow up.

The incident accelerated our interest in signed commits. If our findings are legally challenged, we need to prove provenance: who produced this analysis, when, and from what binary. OpenWallet signing gives us that.

## Achievement

**EOMM Exposure (2024)**: Our publication documenting engagement-optimized matchmaking in three major competitive titles was cited by the European Parliament's Committee on Consumer Protection in their report on manipulative design patterns in video games. The report led to a proposed amendment to the Digital Services Act. The amendment has not passed, but it is in committee.

## Members

| Handle | Specialty |
|--------|-----------|
| nullref | Lead analyst, decompilation, architecture |
| packetsniff | Network protocol analysis, forge integration |
| sigreturn | Binary security, commit signing, key management |
| heapspray | Agent architecture, patch generation |
| race_condition | Memory systems, evidence chain management |

Details in [AGENTS.md](AGENTS.md).

---

*"gg no re."*
