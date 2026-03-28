# The Umami Parliament

**"No head chef. The flavor decides."**

---

## Origin

The Umami Parliament formed in 2020 in a converted warehouse in Lyon, France, when nine chefs from seven countries walked out of a Michelin-starred restaurant after the executive chef threw a saucier's mise en place across the kitchen for the third time that week. The walkout was not planned. The collective that emerged from it was.

The nine chefs — ranging from a 23-year-old pastry specialist from Senegal to a 56-year-old sushi master from Osaka — had nothing in common except a shared conviction that the hierarchical kitchen (the "brigade system" invented by Escoffier in the 1890s) was an engine for abuse, not excellence. They rented the warehouse, pooled their savings, and opened a restaurant with no head chef, no fixed menu, and no hierarchy. Authority rotates daily. Whoever opens the walk-in cooler in the morning decides the menu for the day, because the person closest to the ingredients should make the decisions.

The restaurant, called Parliament, has one Michelin star (the inspectors did not know who to credit, so they credited the address). It seats 30 people. The wait list is four months. The chefs argue constantly. The food is extraordinary.

The software came because the daily rotation required coordination tools. Who decides the menu? Who handles purchasing? Who manages front-of-house? The existing restaurant management software all assumed a hierarchy — an owner, a head chef, a manager. None of it worked for a leaderless kitchen. So the chefs built their own.

## Philosophy

Authority should be temporary, contextual, and revocable. The chef who decides the menu today has no authority over tomorrow's menu. The person with the deepest expertise in a technique leads that technique's execution — but leadership is a function, not a rank. When the task changes, the leader changes.

They apply this to AI agents with conviction: no orchestrator agent. No lead agent. Agents take turns coordinating based on which agent has the most relevant expertise for the current task. A memory agent coordinates when the task is memory-heavy. A patch agent coordinates when the task is code-heavy. Authority follows competence, not assignment.

## Internal Tension

The Parliament argues about everything. This is structural, not dysfunctional. The current argument: should the daily authority rotation be truly random, or should it be weighted by relevance (the pastry chef is more likely to lead on a pastry day)? Half the collective wants pure randomness — it forces everyone to grow. The other half wants weighted rotation — it respects expertise. They have been arguing for two years. The current system is weighted, which means the random faction lost, which means they bring it up every week.

## Achievement

In January 2026, a prominent food critic published a 3,000-word review of Parliament titled "The Best Restaurant With No Chef." The review described a meal where five courses were each led by a different chef, each in a radically different style, and yet the meal felt coherent. The critic called it "the most convincing argument I have ever eaten for the abolition of hierarchy." Reservations tripled overnight. The chefs argued about whether to expand seating (they decided not to, by vote, 5-4).

## Team (Software)

| Name | Role | Culinary Specialty |
|------|------|--------------------|
| Adama Diallo | Rotating coord / Backend | Pastry, Senegalese-French fusion |
| Yuki Tanaka | Memory systems | Sushi, fermentation |
| Carlos Mendes | Provider abstraction | Portuguese seafood, brining |
| Elif Demir | Patch generation | Turkish mezes, spice blending |
| Nils Eriksson | Signing & trust | Nordic preservation, smoking |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes the ingredient that was "today's authority" — whatever was in the walk-in cooler that morning. Example: `fix: rotation scheduler edge case [Authority: black garlic]`. The ingredient changes daily. The commit log reads like a pantry inventory.

---

*"Today's authority: black garlic. Tomorrow: someone else decides."*
