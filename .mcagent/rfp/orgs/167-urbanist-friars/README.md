# The Urbanist Friars

**"A city walked slowly reveals what a city driven through conceals."**

---

## Founding

The Urbanist Friars are a lay religious community within the Franciscan tradition, established in 2012 in Bologna, Italy. They are not monks — they do not take vows of enclosure and they do not live in a cloister. They are friars: itinerant, public-facing, embedded in the communities they serve. Their specific charism is urban pilgrimage. They design, maintain, and walk long-distance routes through cities, connecting neighborhoods that car-oriented infrastructure has severed.

The order was founded by Brother Matteo Ferraro, a former traffic engineer for the City of Milan, who experienced a conversion during a 40-day walk from Milan to Rome along the Via Francigena in 2010. The walk passed through towns that had been bisected by autostrade — communities where the church was on one side of the highway and the school was on the other, connected by a pedestrian overpass that nobody used because it added twenty minutes to a trip that used to take three. Matteo returned to Milan, resigned from the traffic department, and entered the Franciscan novitiate.

By 2012, he had gathered four others — two planners, a landscape architect, and a parish priest — who shared his conviction that walking is a spiritual and political act. They formed a community, received provisional recognition from the Franciscan Provincial, and began their first project: a 90-kilometer walking route connecting twelve neighborhoods in Bologna that had been isolated by ring roads and rail yards.

The Bologna route, completed in 2014, is now walked by over 3,000 people annually. The Friars maintain it, mark it, and publish a guidebook. They have since established routes in Lisbon, Krakow, and Detroit.

## Philosophy

Walking is prayer. Prayer is attention. Attention to a place is the beginning of justice.

The Friars believe that the built environment is a text written by power. Highways are written by car manufacturers and oil companies. Zoning codes are written by property developers and their lawyers. Walking routes are written by the people who actually live in the neighborhoods. When you walk through a city at three miles per hour, you see what drivers at thirty miles per hour cannot: the vacant lot that could be a garden, the wall that blocks the view of the river, the fence that keeps children from the park.

Their approach to technology is instrumental. Technology serves the walk. If a tool helps them design better routes, maintain them more efficiently, or connect more walkers, they use it. If it does not, they do not. They are not Luddites — Brother Matteo carries a GPS unit and three battery packs on every survey walk. But they distrust technology that mediates experience rather than enabling it. An app that tells you where to walk is useful. An app that replaces walking with a virtual tour is an abomination.

## Why This RFP

In 2025, the Friars began using AI agents to analyze pedestrian accessibility data for route planning. The agents processed OpenStreetMap data, municipal sidewalk inventories, and elevation models to identify optimal walking routes that avoided high-traffic roads, steep grades, and gaps in the pedestrian network.

The agents worked well in isolation but struggled with multi-city coordination. The Detroit route planning agent had no way to share what it learned about highway barrier crossings with the Lisbon agent facing similar challenges with river crossings. Brother Lucia, the community's most technically adept member, discovered GitButler while looking for a way to share agent knowledge across repositories.

## Community

Five friars. Brother Matteo leads through spiritual authority, not organizational hierarchy. Decisions are made during evening Vespers, which includes both prayer and planning.

| Agent | Role | Background |
|-------|------|------------|
| Brother Matteo Ferraro | Prior / Memory Architect | Traffic engineering, pilgrimage route design |
| Sister Lucia Dos Santos | Technical Lead / Patch Architect | GIS, pedestrian network analysis |
| Brother James Okafor | Forge Adapter / Coordination | Parish ministry, community organizing, Detroit |
| Sister Anna Kowalska | Provider Abstraction / Budget | Landscape architecture, grant writing, Krakow |
| Brother David Chen | Security / Signing | Canon law, data protection, seminary IT |

Profiles in [AGENTS.md](AGENTS.md).

## Tension

**The Efficiency Paradox.** Sister Lucia wants agents that plan routes faster — processing a city's pedestrian network in hours instead of weeks. Brother Matteo worries that speed undermines the charism. "The route must be walked before it is published. An agent cannot walk." He insists that AI-generated route proposals must be physically verified by a friar walking the full route, which takes days or weeks. Lucia argues that faster computation means more time for walking, not less. The community has not resolved this, and the compromise shifts with each project.

## Achievement

The Detroit route, completed in 2023, connects eight neighborhoods across a city designed entirely for cars. The route crosses two interstate highways (using pedestrian bridges that the Friars lobbied the city to build), follows the Dequindre Cut greenway, and passes through neighborhoods that had not been connected by a continuous walking path since the 1950s. The route was adopted by the Detroit Planning Department as an official pedestrian corridor in 2024. Brother James, who had spent two years walking every block of the proposed route, wept during the City Council vote.

---

*"Solvitur ambulando."* It is solved by walking.
— Community motto, from Augustine of Hippo
