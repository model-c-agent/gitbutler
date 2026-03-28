# StageCraft Pro

**"Jira for theater. But, like, good."**

---

## The Pitch

StageCraft Pro is a SaaS startup based in Brooklyn, New York, that builds production management software for theater companies. Automated cue sheets. Digital prop tracking. Real-time schedule updates pushed to every department head's phone. Basically, the tools that every other industry has had for twenty years, built for an industry that still runs on spreadsheets, binders, and the stage manager's memory.

The company was founded in 2022 by Maya Chen, a former stage manager who had spent ten years managing productions Off-Broadway and on national tours. Maya's breaking point was a production of *Into the Woods* where the prop tracking spreadsheet — a Google Sheet with 340 rows, color-coded by department, maintained by three people with different color-coding philosophies — crashed during tech week. Not "the software crashed." The spreadsheet became so large and so many people were editing it simultaneously that Google Sheets produced a conflict it could not resolve. Props disappeared from the list. Duplicates appeared. The swords for Act II ended up backstage left instead of backstage right because someone resolved the conflict by accepting both versions.

Maya spent the intermission of that show writing user stories on her phone.

She recruited Raj Patel, a full-stack engineer she had dated briefly in college (they are now business partners and emphatically not dating), and they built the first version of StageCraft Pro in six months. The MVP was a digital cue sheet that synchronized in real time across all department heads' devices. It was ugly. It worked. Three Off-Broadway companies signed up in the first month.

StageCraft Pro now serves 180 theater companies, from community theaters to regional houses to two Broadway productions that Maya is not yet allowed to name publicly. The product has grown from cue sheets to a full production management platform: scheduling, prop tracking, costume tracking, budget management, and — as of last month — automated rehearsal report generation.

## Philosophy

### 1. Stage Managers Know Best

The product is designed by stage managers, for stage managers. Every feature request goes through Maya, who evaluates it against a single criterion: "Would this have saved me time on a show?" If yes, it ships. If no, it goes on the backlog. If the answer is "a director would use this but a stage manager wouldn't," Maya kills it. Directors have plenty of tools. Stage managers have binders.

### 2. Ship Fast, Iterate Faster

StageCraft Pro deploys daily. Sometimes twice. Maya runs the company the way she runs a tech rehearsal: make a decision, try it, adjust immediately if it doesn't work. Raj, who has a more cautious engineering temperament, has learned to match her speed but insists on automated tests for anything that touches the cue sheet (because a cue sheet bug during a live show is a career-ending event for a stage manager, and therefore for StageCraft Pro).

### 3. Theater Is Underfunded, So Software Must Be Cheap

Theater companies do not have enterprise software budgets. StageCraft Pro's pricing starts at $29/month for community theaters and scales to $199/month for Broadway-scale productions. This means the company must be capital-efficient. Every engineering decision is evaluated against unit economics. "Can we build this without adding another AWS service?" is Raj's mantra.

## Internal Tensions

**Growth versus focus.** Investors want StageCraft Pro to expand into adjacent markets: concerts, corporate events, film production. Maya resists. "We are a theater company. Theater people trust us because we are theater people. The moment we become 'event management software,' we lose that trust." Raj is more open to expansion — the engineering platform would support it with minimal changes — but defers to Maya on product direction. The argument surfaces at every board meeting and is always tabled.

## Achievement

In March 2026, StageCraft Pro was featured in *American Theatre Magazine* as one of five startups "transforming theater production." Maya's quote in the article: "We are not transforming theater production. We are replacing the spreadsheet. The spreadsheet was transforming theater production. Into chaos." The article drove 60 new signups in a week — the company's best acquisition month.

## Team Overview

| Name | Role | Background |
|------|------|------------|
| Maya Chen | CEO / Product Lead | 10 years stage management, product obsessive |
| Raj Patel | CTO / Engineering Lead | Full-stack, ex-Stripe, unit economics nerd |
| Zoe Kim | Frontend Engineer | React, real-time sync, ex-Figma |
| Andre Morrison | DevOps & Memory Systems | Infrastructure, data pipelines, ex-Datadog |

---

*StageCraft Pro operates from a co-working space in DUMBO, Brooklyn. Maya's desk has a prop sword from the Into the Woods incident. It is labeled "NEVER FORGET" in gaffer tape. Raj's desk has a monitor showing the cue sheet sync latency dashboard. It has not exceeded 200ms in fourteen months. He checks it every morning anyway.*
