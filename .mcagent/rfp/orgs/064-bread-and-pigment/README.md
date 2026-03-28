# Bread & Pigment

**"Feed the eye. Feed the mouth. They are the same hunger."**

---

## Origin

Bread & Pigment occupies a narrow building on the Oudegracht canal in Utrecht, Netherlands. The ground floor is a bakery. The first floor is a gallery. The kitchen and the studio share a wall, and the wall has a window, and through the window you can watch bread rising while looking at paintings made with turmeric, beet juice, and activated charcoal.

The place was started in 2019 by Maren Visser and Tomoko Abe, who were, respectively, a bread baker and a painter, and who were, additionally, married. Maren had been baking sourdough for local restaurants for a decade, obsessing over crust color, crumb structure, and the visual geometry of a well-scored loaf. Tomoko was painting large-format abstract works using pigments derived from food-grade ingredients — saffron for yellow, spirulina for green, cochineal for red. They realized they were doing the same thing in different rooms: creating beauty from edible materials.

The merger happened gradually. Tomoko started painting in the bakery. Maren started treating bread as art. A gallery visitor tasted a painting (the pigment was literally edible). A bakery customer framed a bread basket. The boundaries dissolved.

Bread & Pigment now employs five people, produces artisan bread for 14 restaurants, exhibits quarterly in the upstairs gallery, and maintains a modest open-source presence built around food photography processing and color analysis tools. They came to software because they needed to precisely match bread crust colors to painting pigments, and no existing tool could do it.

## Philosophy

There is no line between craft and art. A perfectly proofed loaf is a sculpture. A painting made with saffron pigment is a recipe. Bread & Pigment believes that the distinction between "functional" and "beautiful" is artificial and harmful — it excuses ugly functional things and useless beautiful things.

They approach software the same way: code is a craft material. A well-structured function is as satisfying as a well-scored crust. A clean diff is as beautiful as a clean brushstroke. They will not produce code they would not display in the gallery.

## Internal Tension

Maren is the pragmatist. Bread must be baked by 6 AM regardless of its aesthetic merits. Tomoko is the perfectionist. A painting is not done until it is done, and deadlines are a form of coercion. When they build software, this tension manifests as a disagreement about iteration: Maren wants to ship and iterate (the bread must be on the shelf). Tomoko wants to refine until it is right (the painting is not done). Their codebase has modules built by Maren (functional, adequate, slightly rough) and modules built by Tomoko (beautiful, over-refined, three weeks late).

## Achievement

In 2025, Bread & Pigment's color-matching algorithm — originally built to match crust colors to pigment recipes — was adapted by a Dutch museum for art conservation. The algorithm could identify pigment degradation in oil paintings by comparing current color values against historical reference data. The museum used it to detect early-stage degradation in a 17th-century Vermeer that conservators had missed on visual inspection. Tomoko was delighted. Maren was confused that their bread software was saving Vermeers.

## Team

| Name | Role | Background |
|------|------|------------|
| Maren Visser | Baker / Pragmatic Lead | 12 years artisan baking, color science hobbyist |
| Tomoko Abe | Painter / Aesthetic Lead | Abstract art, food-grade pigments, color theory |
| Sander Bakker | Full-Stack Engineer | Ex-Booking.com, built the color-match tool |
| Lina Osei | Data / Color Science | Computational color, ex-AkzoNobel paint lab |
| Roan de Vries | DevOps / Fermentation | Sourdough starter management, CI/CD |

Agent profiles in [AGENTS.md](AGENTS.md). Technical proposal in [PROPOSAL.md](PROPOSAL.md).

## Signature Quirk

Every commit message includes a color swatch description — the closest natural pigment match to the "mood" of the change. Example: `fix: crust color parser overflow [Pigment: burnt sienna, cochineal base, 45min bake]`. Lina maintains the pigment reference chart. Disagreements about which pigment matches which commit have delayed two releases.

---

*"The crust is the canvas. The crumb is the composition. Bake accordingly."*
