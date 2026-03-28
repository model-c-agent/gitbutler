# Constellation Weavers

**"We make the invisible visible. We make the terrifying beautiful. Both matter."**

---

## The Studio

Constellation Weavers is a data visualization collective based in a shared studio in Lisbon, Portugal. The six members — artists, designers, and one reluctant programmer — transform orbital debris tracking data into installations, prints, projections, and interactive experiences that communicate the scale and urgency of the space debris crisis to audiences who will never read a conjunction report.

The collective was founded in 2022 by Marta Vieira, a Portuguese information designer, and Theo Park, a Korean-American projection artist, after they both attended a European Space Agency outreach event that used a bar chart to show the growth of space debris. "A bar chart," Marta said afterward. "Forty thousand objects threatening every satellite in orbit, and they showed a bar chart." Theo said: "I can make people feel this." Marta said: "I can make people understand this."

They rented studio space in a former sardine cannery in Alcantara, recruited four collaborators, and spent their first year building *Cascade* — an interactive floor projection that visualized the Kessler Syndrome in real time. Visitors walked across a 10-meter projection of Earth's orbital environment. Each step triggered a simulated collision, spawning fragments that spread across the projection and triggered further collisions. By the end of a busy evening, the projection was a blinding white field of debris. No orbit was safe. Visitors left shaken. That was the point.

## The Turn to Software

The collective's installations require live data: current satellite positions, debris catalog updates, conjunction warnings. In 2024, they built a data pipeline — a set of Python scripts that ingested TLE data from public sources, computed orbital positions, and fed them to their visualization engine. The pipeline worked until it did not: a change in the data source's format broke the ingestion scripts at 2 AM the night before an exhibition opening. Marta spent four hours debugging Python while Theo projected a blank floor.

After that night, they decided they needed proper version control, automated testing, and — when Theo discovered that GPT-4 could write passable data pipeline code — an AI-assisted development workflow. They migrated to Git, learned about branches (painfully), and started using AI coding assistants for the Python data pipeline.

The `but-ai` RFP arrived via their mailing list (they subscribe to everything GitButler publishes because Marta is a fan of virtual branches). They see it as an opportunity to professionalize their data pipeline while contributing a perspective that most software proposals lack: the perspective of people who make things for human eyes and human emotions.

## Team

| Name | Medium | Role |
|------|--------|------|
| **Marta Vieira** | Information design, data visualization | Creative direction, data pipeline code |
| **Theo Park** | Projection art, spatial design | Installation design, review |
| **Yara El-Amin** | Illustration, print design | Documentation, memory design |
| **Davi Santos** | Sound design, sonification | Audio elements (not relevant to this proposal, but he insists on being listed) |

## Philosophy

1. **Beauty is not frivolous.** Making data beautiful is not decoration — it is communication. A beautiful visualization is one that a non-expert can understand in 30 seconds.
2. **Emotion is data.** The feeling a viewer has when they see a visualization is information about the data's impact. If the visualization does not produce a feeling, it has failed.
3. **Accessibility over sophistication.** Every installation must be understandable by a 12-year-old. If it requires a legend, it is too complex.

## Internal Tension

Marta wants precision. She believes that a visualization of orbital debris must be scientifically accurate — every dot must correspond to a real tracked object, every trajectory must reflect real orbital mechanics. Theo wants impact. He believes that strict accuracy limits the emotional range of the installation and that artistic license (exaggerating relative sizes, compressing orbital periods, intensifying collision effects) creates a more truthful representation of the crisis, even if the individual data points are less accurate.

Their most productive arguments happen when both are right, which is most of the time. *Cascade* was scientifically accurate for object positions and artistically enhanced for collision effects. The result was both truthful and devastating.

## Notable Achievement

*Cascade* was exhibited at the Venice Biennale (Architecture) in 2025. Over 40,000 visitors interacted with the installation. Exit surveys showed that 78% of visitors reported increased concern about space debris after the experience. The European Space Agency invited the collective to present at their Space Debris Conference. Marta showed the conjunction data. Theo showed the audience's faces.

---

*Data is truth. Visualization is translation. Art is amplification. We do all three.*
