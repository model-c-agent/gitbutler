# BPM United

**"Every position has a role. Every role has a beat."**

---

## The Squad

BPM United is a music production crew based in Manchester, England, that operates like a football club. Founded in 2020 by five friends who met playing five-a-side at Platt Fields Park, the crew discovered that the tactical structures they argued about at the pub after matches mapped perfectly onto production workflows. The striker finishes the track. The midfielder controls tempo and arrangement. The goalkeeper catches errors before they reach the master. The manager sets strategy but never touches the ball.

The analogy started as a joke. It became their operating system.

BPM United occupies a converted warehouse in Ancoats that they call "the pitch." The main production room is literally painted with touchline markings. There is a whiteboard labeled "formation" that shows who is working on which track. When someone finishes a stem, they "pass" it to the next person by pushing it to a shared branch. When someone catches a mix error, they "save" it and push the fix to a review branch. The football metaphor is total. Visitors find it either charming or insufferable. There is no middle ground.

## Formation

The crew runs a 4-3-3 formation adapted for production:

- **Goalkeeper (GK):** Quality control. Catches errors before mastering.
- **Defenders (DEF):** Infrastructure. Maintain the studio systems, backups, and version control.
- **Midfielders (MID):** Arrangement and production. Control the flow of the track.
- **Forwards (FWD):** Mixing and mastering. Finish the track.
- **Manager (MGR):** Strategy and coordination. Never touches the mix.

In practice, the five members rotate positions based on the project. Leo plays GK on Tuesday and FWD on Thursday. The rotation prevents staleness and ensures everyone understands every position.

## The Software Pitch

BPM United got into software development because their production workflow outgrew the tools available. In 2024, after losing a week's work to a corrupted Pro Tools session file, they built `pitchctl` -- a CLI tool that manages audio stems as Git branches and treats mix sessions as merge commits. The tool is crude, undocumented, and used by exactly five people. But it works, and the experience taught them that version control concepts translate directly to production workflows.

When the `but-ai` RFP landed, Leo (the crew's most technical member and perpetual manager) saw it as an upgrade path for `pitchctl`. Devon wants to keep things scrappy. Marcus thinks they should go all in. The vote was 3-2 in favor of responding.

## Team

| Name | Default Position | Specialty |
|------|-----------------|-----------|
| **Leo Marchetti** | Manager | Strategy, coordination, token budgets |
| **Devon Hart** | Goalkeeper | Quality assurance, error detection |
| **Marcus Obi** | Midfielder | Architecture, system design |
| **Yuki Tanaka** | Forward | Patch generation, code output |
| **Sam Nazari** | Defender | Infrastructure, memory, signing |

## Internal Tension

Leo wants to professionalize -- proper documentation, CI pipelines, code review checklists. Devon thinks professionalization kills the vibe and that their best work happens when they are just five people riffing in a warehouse. The rest of the crew oscillates. The compromise: they follow Leo's process for external commitments (like this RFP) and Devon's chaos for internal projects. Both approaches have a winning record.

## Notable Achievement

In 2025, BPM United produced the soundtrack for a BBC documentary about Manchester's music scene. The entire project -- 14 tracks, 47 stems, 3 remixes -- was managed through `pitchctl`. Zero stems were lost. Zero mix versions were overwritten. The BBC's post-production team called it "the smoothest audio handoff we have ever received." Leo framed the email. Devon said it proved nothing because "even a broken pitch plays well on a sunny day."

---

*Full time. Final score: vibes 3, process 2. Match report filed.*
