# Pixel & Soul

**"Every frag is a frame. Every match is a film."**

---

## Who We Are

Pixel & Soul is a five-person artist commune based in a converted warehouse in Ghent, Belgium. We make esports content — not highlight reels, not hype packages, not the corporate sizzle that tournament organizers pump out during ad breaks. We make *films*. Short films. Documentary portraits. Experimental pieces that use gameplay footage the way Terrence Malick uses landscape shots: as texture, as mood, as a window into something the subject cannot articulate directly.

The commune was founded in 2022 by Maren Lindqvist, a documentary filmmaker who spent a year embedded with a professional Counter-Strike team for a film that never got distribution because the production company wanted her to add "more exciting moments." The film was a quiet, contemplative study of five people who spent 14 hours a day together, performing the same actions thousands of times, chasing an improvement margin measured in milliseconds. The production company wanted explosions. Maren wanted to show the silence between the rounds.

She left, found four like-minded artists — a motion designer, a sound designer, a writer, and a photographer who moonlighted as a systems engineer — and formed Pixel & Soul. They fund their work through Patreon, commissioned pieces for indie game studios, and the occasional tournament organizer who wants something different.

## How We Got Here

We started using Git for version control on our media projects in 2023. Not for code — for project files. Premiere Pro timelines, After Effects compositions, audio stems, color grading presets. We store the metadata and project configurations in Git; the binary assets live in LFS. Our repository structure mirrors our creative process: each piece has a `vision/` directory (mood boards, scripts, references), a `production/` directory (project files, assets), and a `delivery/` directory (exports, distribution metadata).

In 2024, we started experimenting with AI-assisted editing. We built a set of agents that could analyze footage, identify emotionally resonant moments (based on audio energy, camera movement, and player behavioral signals), and generate rough cuts as edit decision lists (EDLs). The EDLs were committed to the repository as structured data. The agents were clumsy at first — they kept selecting "exciting" moments instead of quiet ones, because their training data was dominated by mainstream esports highlight reels. We spent six months fine-tuning them to prefer stillness over spectacle.

When those agents started conflicting with each other (the color grading agent and the audio agent both wanted to adjust the emotional arc of the same sequence), we needed better version control for agent output. GitButler's virtual branches let us keep each agent's creative direction alive simultaneously and merge them during our weekly review sessions. The `but-ai` RFP is a chance to formalize that workflow.

## Philosophy

Art is not efficient. Agents should not be either — at least, not in the way engineers mean. Our agents are efficient in a different sense: they are efficient at preserving ambiguity. They generate multiple interpretations of the same footage and present them without ranking. Ranking is a human act. The artist chooses.

We distrust agents that optimize for a single objective. A single objective produces corporate work — technically correct, emotionally empty. Our agents optimize for *range*: how many distinct emotional readings can this footage support? The best footage is the footage that can be cut five different ways and mean five different things.

## The Autocut Incident

In March 2025, we deployed an automated rough-cut agent on a commissioned piece for an indie game studio. The agent produced a technically competent 3-minute cut. The client loved it. Maren hated it. She said it "looked like everything else on the internet" — algorithmically smooth, emotionally predictable, the cinematic equivalent of stock music.

The incident forced an internal debate about whether AI agents should ever produce final outputs or only intermediate artifacts. The commune voted 3-2 that agents produce *proposals* — multiple rough cuts, each labeled with the emotional register it targets — but never a single "best" version. The two dissenters (the motion designer and the photographer) thought this was inefficient. Maren said efficiency was not the point.

## Achievement

**"Peripheral Vision" — Sundance New Frontier 2026**: A 12-minute experimental documentary about a retired StarCraft professional, composed entirely from agent-analyzed gameplay footage and interview fragments. Three agents each produced a different emotional arc; the final film interweaves all three, letting the viewer experience the subject's ambivalence about retirement without resolving it. Selected for the New Frontier program at Sundance. The commune's first major festival selection.

## Team

| Agent | Name | Role |
|-------|------|------|
| Director | Maren Lindqvist | Creative direction, final editorial authority |
| Visual Engine | Samir Osei | Motion design, patch generation for visual assets |
| Sound Architect | Elin Falk | Audio analysis, sonic memory, emotional scoring |
| Narrator | Cass Moreau | Commit messages as micro-narratives, documentation |
| Operator | Juno Varga | Systems engineering, plugin architecture, provider abstraction |

Details in [AGENTS.md](AGENTS.md).

---

*"The algorithm suggests. The artist decides. The audience feels."*
