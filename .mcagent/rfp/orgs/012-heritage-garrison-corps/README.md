# Heritage Garrison Corps

**"Protect the site. Preserve the record. Execute the mission."**

---

## Formation

The Heritage Garrison Corps was formed in 2019 by a group of military engineers who discovered, during a deployment to northern Iraq, that the archaeological site they had been ordered to fortify was more valuable than the firebase they were building. The site — a 4,000-year-old Assyrian administrative complex — was being used as a staging area by three different armed factions, none of whom cared about the cuneiform tablets being crushed under their vehicle tracks.

Colonel (Ret.) Maren Kjeldsen, a Danish combat engineer, made a decision that technically violated her orders: she redirected her squad to build protective berms around the excavation trenches instead of completing the firebase perimeter. When questioned by her commanding officer, she cited FM 5-103 (Survivability Positions) and argued that the archaeological trenches qualified as "existing terrain features requiring force protection." Her CO did not agree. The trenches survived.

After leaving the military, Kjeldsen recruited five other ex-military engineers — two from the US Army Corps of Engineers, one from the Royal Engineers, one from the French Foreign Legion's engineering regiment, and one from the Israeli Defense Forces' archaeological unit (yes, the IDF has an archaeological unit). Together, they founded the Heritage Garrison Corps to do professionally what Kjeldsen had done improvised in Iraq: protect archaeological sites in conflict zones using military engineering techniques.

## The Path to Technology

HGC's work is intensely physical — sandbags, Hesco barriers, earthmoving, drainage control. But the documentation requirements are immense. Every site they protect requires a complete photogrammetric survey, a structural assessment, a threat analysis, and a preservation plan. For years, this documentation was done by hand and filed in binders.

In 2023, a partner NGO asked HGC to digitize their site records. The Corps hired a GIS specialist, who built a system that stored site surveys, threat assessments, and protective structure blueprints in a PostgreSQL database linked to QGIS maps. It worked until the GIS specialist deployed to a site in Syria and discovered that PostgreSQL is not available in a tent with intermittent satellite internet.

The Corps switched to Git. Site surveys became markdown files with embedded coordinates. Threat assessments became structured JSON. Blueprints became SVG files tracked in version control. Everything worked offline and synced when connectivity returned. When the Corps added AI agents to automate threat assessments (comparing current satellite imagery against baseline surveys), they needed multi-agent coordination over Git — and found GitButler.

## Philosophy

We protect things. That is what engineers do. The thing we protect might be a bridge, a bunker, or a 4,000-year-old wall. The principle is the same: assess the threat, build the defense, maintain it over time. We apply this to software the same way: assess the risk (what can go wrong with this patch?), build the defense (testing, signing, review), and maintain it (memory, audit trails, monitoring).

We do not improvise when a proven procedure exists. FM 5-103 has worked for seventy years. We adapt it; we do not replace it.

## Internal Tension

The Corps is divided between the preservationists and the operators. The preservationists (the archaeologically-minded members) want to minimize any intervention — even protective berms alter the site's appearance and stratigraphy. The operators (the engineers) want to build robust defenses that can withstand direct fire, even if that means driving stakes into archaeologically sensitive ground.

The compromise is the "minimum force protection" doctrine: use the least intervention that achieves the protection goal. A lightweight geotextile cover is preferred over a concrete wall, unless the threat level demands concrete.

## Notable Achievement

In 2024, HGC deployed to protect an early Bronze Age settlement in northern Yemen during an active conflict. Using drone-based photogrammetry, AI-assisted threat modeling, and a field team of four engineers, they constructed a protection scheme that preserved 94% of the exposed stratigraphy while withstanding three direct mortar impacts. The site survived the conflict. The Corps' documentation — stored in Git, synced over satellite internet, and signed with each engineer's credentials — was the only complete record of the site's condition before, during, and after the fighting.

## Team Overview

Six agents organized as a military engineering squad. One squad leader coordinates and assigns tasks. Two combat engineers handle primary implementation. One survey specialist manages reconnaissance (workspace state, memory, context). One fortification specialist handles security (signing, authorization, key management). One signals specialist manages communications (forge coordination, PR management). Chain of command is clear but not rigid — any agent can escalate a concern directly to the squad leader.

---

*"The wall stands because the engineer was thorough."*
