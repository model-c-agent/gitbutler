# Nakamura Sound House

**"Three generations. One standard."**

---

## The House

Nakamura Sound House occupies a narrow four-story building in Ota City, Tokyo, wedged between a convenience store and a temple supply shop. The ground floor is a retail showroom displaying the mixing consoles, preamps, and summing boxes the family builds by hand. The second floor is the workshop. The third floor is where Yuki Nakamura, the current generation's lead builder, lives and writes software. The fourth floor belongs to his grandmother, Fumiko, who still tests every console by ear before it ships.

The family has been building audio equipment since 1968, when Takeshi Nakamura -- Yuki's grandfather -- left Toshiba's consumer electronics division to build mixing consoles for Tokyo's recording studios. Takeshi's consoles were overbuilt, expensive, and sonically flawless. He sold twelve units in his first year. Studios that bought them never bought another brand. When Takeshi died in 2003, his son Kenji took over and expanded the product line to include preamps and summing boxes. Kenji runs the workshop today. He builds by hand, tests by ear, and documents nothing.

This last point -- the absence of documentation -- is the family business's greatest vulnerability and the reason they are responding to this RFP.

## The Problem

In 2024, Kenji had a health scare that put him in the hospital for three weeks. During those three weeks, the workshop stopped. Not because there were no orders -- there were seven consoles on backorder. It stopped because Kenji carries the build specifications in his head. Wiring diagrams, component tolerances, assembly sequences -- none of it is written down. Yuki, who had been studying computer science at the University of Tokyo, came home and realized that if his father could not work, the family business could not function.

Yuki spent six months documenting his father's processes. He recorded Kenji building a console from scratch, transcribed the recordings, and converted them into structured build specifications stored in a Git repository. Then he went further: he built a set of scripts that tracked component inventories, generated wiring diagrams from declarative specifications, and produced quality control checklists from the build specs.

The scripts worked. Kenji hated them. He hated them because they exposed the gap between what he thought he did and what he actually did -- the scripts revealed inconsistencies in his process that he had unconsciously corrected by hand for decades. The family argued about this for weeks. The resolution: Kenji continues to build by hand. Yuki maintains the documentation system. Fumiko tests everything and has the final word.

## Why This RFP

Yuki sees the `but-ai` plugin as an extension of his documentation project. If an AI agent can read the family's build specifications and produce valid patches against them, then the specifications are good enough to survive the loss of any single person. This is not about automation -- Kenji will build by hand until he retires. It is about resilience. It is about making sure the fourth generation does not lose what the second generation knew.

## Team

| Name | Generation | Role |
|------|-----------|------|
| **Fumiko Nakamura** | 1st (founder's wife) | Final quality authority. Tests by ear. |
| **Kenji Nakamura** | 2nd | Master builder. Carries the knowledge. |
| **Yuki Nakamura** | 3rd | Software, documentation, this proposal. |
| **Aiko Sato** | — | Yuki's university friend. Frontend developer. Helps part-time. |

## Internal Tension

Kenji believes that codifying knowledge kills intuition. His consoles are great because he adjusts each one by hand, responding to the specific characteristics of the components in that specific unit. A specification that says "resistor R47: 4.7kOhm +/- 5%" misses the fact that Kenji measures every resistor and adjusts the surrounding circuit to compensate. Yuki argues that the adjustment process itself can be specified. Kenji says that is like writing a recipe for knowing when the soup is done.

They are both right. The tension is productive but painful.

## Notable Achievement

In early 2026, a Tokyo recording studio ordered a console to match a unit Takeshi had built in 1974. The original was failing and irreparable. Using Yuki's documentation system, the family built a reproduction in eight weeks -- a process that would have taken sixteen without the structured specifications. Kenji made 23 manual adjustments during the build. Yuki documented all 23. Fumiko tested the reproduction against her memory of the original and said, "Close enough." From Fumiko, this is the highest praise.

---

*Built by hand. Tested by ear. Documented by necessity.*
