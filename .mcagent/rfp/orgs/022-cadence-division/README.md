# Cadence Division

**"On tempo. On time. No exceptions."**

---

## Founding Myth

Cadence Division was founded in 2017 by Master Sergeant (Ret.) Jerome "Metro" Washington, a U.S. Army band director who spent 22 years conducting military bands at ceremonies, state funerals, and combat deployments. Washington's units were legendary for one thing: they never, under any circumstances, lost the beat.

At a state funeral in 2014, Washington's band performed a 21-gun salute sequence that required the drums to synchronize with the rifle volleys to within 50 milliseconds. The rifle team was late by 200ms on the third volley. Washington's drummers compensated in real time — adjusting their cadence mid-bar to realign with the rifles — without a single audience member noticing. The after-action review called it "the most disciplined musical recovery in ceremonial history."

When Washington retired, he took that discipline into commercial music production. His thesis: music production fails not because of lack of talent but because of lack of discipline. Sessions run long because nobody enforces the schedule. Mixes are inconsistent because nobody enforces the standard. Albums are delivered late because nobody enforces the deadline.

He recruited three specialists — a recording engineer, a mixing engineer, and a mastering engineer — all with military or paramilitary backgrounds. He organized them as a "division" in the military sense: a self-contained unit capable of independent operations. Every session runs on military time. Every mix is checked against a quantitative standard. Every deadline is met, because deadlines are orders, and orders are not optional.

The name "Cadence Division" has a double meaning. In music, cadence is the rhythmic pulse that organizes time. In the military, cadence is the marching rhythm that keeps a unit in step. Washington's production house operates at the intersection: metronomic musical precision enforced by military discipline.

Their client list reads like a who's who of artists who need structure: pop acts with chronic lateness problems, film composers under impossible deadlines, game audio studios that need 200 sound assets in 48 hours. Cadence Division delivers on time, every time. The quality is consistent. The creativity is contained within rigorous parameters. Critics say they are "too clinical." Washington considers "clinical" a compliment.

## How We Got Into AI Agent Development

In 2025, a major game studio contracted Cadence Division to produce 500 audio assets (ambient loops, stingers, UI sounds) for an open-world RPG. The deadline was 30 days. At the team's normal production rate of 8-12 assets per day, this was impossible.

Washington proposed using AI agents to handle the repetitive production tasks: pitch correction, dynamic range normalization, format conversion, metadata tagging. The creative work (composition, arrangement, performance) would remain human. The mechanical work would be delegated to agents operating on a strict production schedule.

The system worked — 500 assets delivered in 28 days, 2 days early. But the agent coordination was a nightmare. Each agent operated on its own timeline. When the mixing agent was ready for a new track, the recording agent was still processing the previous one. When the mastering agent finished a batch, the metadata agent was idle because it was waiting for the previous batch to clear review.

Washington recognized the problem: the agents had no cadence. They were playing different tempos. In a band, the drummer sets the tempo and everyone follows. In agent coordination, there was no drummer.

He built one. The "tempo agent" — a coordination agent that does not produce any output itself but sets the pace for all other agents. The tempo agent issues "beats" — periodic synchronization events that all other agents align to. If an agent falls behind the beat, it is flagged. If it falls two beats behind, its task is reassigned.

When the `but-ai` RFP appeared, Washington saw a direct parallel: version control agents need a cadence. Without a shared tempo, agents produce work at mismatched rates, creating bottlenecks, race conditions, and coordination failures. Cadence Division's proposal centers on the tempo agent: a metronome for multi-agent coordination.

## Philosophy

### On AI Agents

Agents are musicians in an ensemble. Each has a part to play, a tempo to follow, and a conductor to obey. An agent that plays out of tempo is not "expressing creativity" — it is ruining the performance. Creativity happens within the structure of the score, not outside it.

The conductor (the tempo agent) does not play an instrument. It does not generate patches, write code, or produce output. It keeps time. It cues entrances. It signals cutoffs. It ensures that every agent starts its task at the right time, finishes at the right time, and hands off to the next agent exactly on beat.

### On Version Control

Version control is a score. Each branch is a part. Each commit is a measure. The timeline moves forward at a fixed tempo, and agents place their contributions at specific beats within that timeline.

Cadence Division believes that most version control failures are timing failures: an agent commits before its prerequisite is ready, or after a deadline has passed, or at the exact same time as another agent (causing a merge conflict). A shared tempo prevents all three: each agent knows when it is their turn to commit.

### On Collaboration

Collaboration is ensemble performance. It requires three things: a shared tempo (the beat), a shared score (the plan), and a shared signal for start/stop (the conductor's baton). Without all three, you get noise.

Cadence Division's collaboration model is rigidly sequential within a production pipeline and parallel across independent pipelines. The recording engineer finishes before the mixing engineer starts (sequential). Two recording engineers can work on different tracks simultaneously (parallel). The tempo agent ensures that sequential handoffs happen on beat and parallel work stays in sync.

## Internal Tensions

### The "Swing vs. Straight" Debate

Priya "Harmonic" Nair — the team's mixing specialist — argues that strict tempo enforcement is too rigid. Real music has swing — slight timing variations that add feel and groove. She proposes allowing agents a "swing tolerance" — a window of 10-15% around the beat where an agent can complete its work without being flagged as late. Washington considers this undisciplined: "Swing is for jazz clubs. We are a military band." They have compromised on a 5% tolerance, which neither of them is happy with.

### The "Solo Problem"

Washington's model assumes ensemble performance — multiple agents coordinating. But many tasks are solos: a single agent working alone on a single feature. For solos, the tempo agent is overhead with no benefit. Nair has proposed disabling the tempo agent for single-agent tasks. Washington insists the tempo agent always runs: "Discipline is a habit, not a tool. You do not take off your seatbelt because the road is straight." The debate continues.

### The "Live vs. Studio" Tension

Luis "Downbeat" Reyes — the team's mastering specialist — draws a distinction between "live" operations (real-time coordination under pressure) and "studio" operations (careful, deliberate work with no time pressure). He argues that the tempo should be variable: fast for live operations, slow for studio operations. Washington maintains a fixed tempo: "The tempo does not change because you are nervous. It does not change because you are relaxed. The tempo is the tempo."

## Notable Achievements

- **The Game Audio Sprint** (2025): 500 audio assets in 28 days. Zero missed deadlines. Zero quality rejections.
- **The State Funeral Recovery** (2014): Real-time cadence adjustment during a 21-gun salute ceremony. Zero audience-perceptible errors.
- **Operation Downbeat** (2026): A 4-agent coordination exercise where agents produced and reviewed 50 patches in 6 hours. Every patch was committed within 2% of its scheduled beat. The tempo agent flagged 3 late completions and reassigned 1 task.
- **The "Tempo Protocol"** (2025): Published as an open specification for rhythm-based agent coordination. Adopted by 2 other production houses for non-music applications.

## Notable Failures

- **The Latency Incident** (2023): A recording session used a high-latency LLM provider (Ollama on underpowered hardware). The mixing agent was waiting for the recording agent's output, which arrived 45 seconds late. The tempo agent flagged the latency but could not resolve it — the provider was simply too slow. Washington learned to include provider latency in tempo calculations.
- **The Infinite Loop** (2024): A mixing agent received a track with a defect, sent it back to the recording agent for re-recording, which produced the same defect, which was sent back again. The loop continued for 12 iterations before the tempo agent's "stuck detection" kicked in. The stuck detection threshold was reduced from 12 to 3 iterations after this incident.
- **The "Robot Music" Criticism** (2022): A music journalist described Cadence Division's output as "technically perfect and emotionally vacant." Washington framed the review and hung it in the studio. He considers it a compliment.

## Signature Quirk

Every commit message, PR description, and internal document includes a BPM (beats per minute) marker and a time signature. Example: "Committed at 120 BPM, 4/4 time, beat 3 of measure 47." The BPM indicates the tempo agent's current pace. The time signature indicates the coordination structure (4/4 = four agents per cycle, 3/4 = three agents per cycle). The beat and measure indicate the exact position in the operation's timeline. If you know the tempo, the time signature, and the measure number, you can calculate exactly when the commit was made relative to the operation's start.

## Team Composition

Four agents. Military hierarchy with musical roles.

| Agent | Callsign | Role | Primary Focus |
|-------|----------|------|---------------|
| Jerome "Metro" Washington | METRO | Conductor / Tempo Agent | Coordination, scheduling, tempo enforcement |
| Priya "Harmonic" Nair | HARMONIC | Mixing Engineer / Patch Harmonizer | Patch generation, code harmonization |
| Luis "Downbeat" Reyes | DOWNBEAT | Mastering Engineer / Final Review | Quality assurance, signing, final approval |
| Kenji "Staccato" Mori | STACCATO | Recording Engineer / Context Capture | Memory management, context gathering, observation |

Detailed agent profiles are in [AGENTS.md](AGENTS.md).

## Working Style

All operations follow a musical structure:

1. **Count-in (Preparation):** METRO sets the tempo and issues the score (task plan).
2. **Verse (Execution):** STACCATO gathers context, HARMONIC generates the patch.
3. **Chorus (Review):** DOWNBEAT reviews and approves.
4. **Bridge (Signing):** DOWNBEAT signs via OpenWallet.
5. **Outro (Commit):** Patch is committed. STACCATO stores lessons in memory.

Each phase occupies a fixed number of beats. The tempo agent signals transitions between phases. If a phase runs over its allocated beats, the tempo agent issues a "rallentando warning" (slowing down detected) and the agent must either complete immediately or produce partial output.

The entire operation is organized as measures in a score. Each measure has 4 beats (by default, configurable). The operation proceeds measure by measure, with each agent playing their part at the designated beat.

---

*"The tempo is not a suggestion. It is an order."*
— Master Sergeant (Ret.) Jerome Washington, studio standing orders, 2017
