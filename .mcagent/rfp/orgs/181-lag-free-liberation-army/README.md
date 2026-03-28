# Lag-Free Liberation Army

**"No permanent captains. No permanent losers. Rotate everything."**

---

## The Formation

The Lag-Free Liberation Army (LFLA) is an anarchist esports collective based in Berlin, operating a competitive Valorant and Counter-Strike roster since 2022. They do not have an owner. They do not have a manager. They have a constitution — 14 pages, ratified by unanimous vote, amended seventeen times — that governs everything from map vetoes to snack procurement for LAN events.

The LFLA was founded after a mass walkout from a mid-tier German esports organization called Neon Wolves. Seven players left simultaneously after the organization's owner — a cryptocurrency investor who had never played a competitive game — overruled the team captain's map veto in a tournament qualifier because he had bet on the match outcome. The owner was subsequently banned from the league. The seven players pooled their prize winnings, rented a flat in Kreuzberg, and formed the LFLA.

The founding principle: no one person controls competitive decisions. Captaincy rotates every match. Strategic calls during matches are made by majority vote during tactical timeouts (the constitution allocates exactly 8 seconds for deliberation, 2 seconds for execution). This is slower than having a single in-game leader. They lose rounds they should win because the vote was 3-4 and the minority dissented audibly. They also win rounds they should lose because the collective intelligence of seven people processing game state simultaneously surfaces options that no single captain would see.

Their competitive record is respectable but not elite: consistently top 8 in German regional leagues, two appearances in EU-level qualifiers, and one memorable upset victory over a top-20 world-ranked team that the LFLA attributes to superior collective decision-making and their opponents attribute to luck.

## Philosophy

Hierarchy in competitive gaming is an optimisation for speed at the expense of resilience. A team with a single IGL (in-game leader) makes fast decisions, but when the IGL dies early in a round, the team collapses. A team with rotating leadership makes slower decisions, but the decision-making capacity is distributed — losing any single player does not destroy the team's strategic capability.

The LFLA extends this to all operations. Finances are transparent (every member can see every transaction). Schedule decisions are voted on. Even the practice schedule is collectively determined, which means it changes every week and nobody is ever fully happy with it.

## Why This RFP

In 2025, the LFLA started using AI agents for practice analytics. Agents process demo files (recordings of competitive matches), extract player positioning data, and generate tactical reports: heat maps, rotation timings, utility usage patterns. The reports are used to identify weaknesses in the team's collective play.

The agents worked well individually but conflicted when analyzing the same match from different analytical perspectives. The positioning agent and the utility agent would produce contradictory recommendations because they were optimizing for different objectives. The team needed version control that could hold multiple analytical perspectives simultaneously and let the collective vote on which recommendations to adopt.

A member found GitButler while searching for "version control for competing hypotheses." The virtual branch model was exactly what they needed.

## Team

Five members. Selected by the collective's most contentious mechanism: a ranked-choice vote where every member ranks every other member's suitability for the project, and the top five are chosen. The vote took four hours and three recounts.

| Agent | Role | In-Game Role |
|-------|------|-------------|
| "Specter" (Jana Richter) | Orchestration / Rotating Captain | Duelist / IGL (when it's her rotation) |
| "Volt" (Karim Ayad) | Patch Architect / Tactical Analysis | Controller |
| "Cipher" (Lena Braun) | Memory & Analytics | Sentinel |
| "Pulse" (Dayo Ogunbiyi) | Forge & Coordination | Initiator |
| "Ghost" (Tomek Kowalski) | Security & Signing | Flex |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Rotation Debate.** Specter argues that rotating captaincy is the LFLA's core identity and must never be compromised, even if it costs matches. Volt argues that in high-stakes tournament play, the team should temporarily adopt a fixed IGL for the duration of the event — "revolution can wait until after the quarterfinals." The collective voted 4-3 to maintain rotation at all times. Volt accepted the result. He brings it up again before every tournament. The vote has been held seven times. Rotation has won every time.

## Achievement

In March 2025, the LFLA defeated Fokus Esports — ranked 19th in the world at the time — in a best-of-three qualifier for the EU Champions League. The LFLA rotated captains between maps (Specter called Map 1, Volt called Map 2, Cipher called Map 3). Fokus's coach later said in an interview: "We could not read their calls because there was no pattern. Every map was a different team." The LFLA lost in the next round. They consider the Fokus upset the finest competitive achievement in their history and have a framed screenshot of the final scoreboard in the flat.

---

*"gg no re. Unless we vote to re."*
— Collective motto, adopted by 5-2 vote, amended once (the original version included profanity)
