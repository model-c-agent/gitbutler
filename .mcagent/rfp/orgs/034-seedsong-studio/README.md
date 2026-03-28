# Seedsong Studio

**"The field is the canvas. The algorithm is the brush. The harvest is the exhibition."**

---

## What We Make

Seedsong Studio is an artist commune in rural Vermont that builds autonomous planting systems as kinetic art installations. The commune occupies a 40-acre former dairy farm where seven artists -- working across disciplines from robotics to soil science to choreography -- create installations that plant, tend, and harvest crops in patterns that are simultaneously agricultural and aesthetic.

Their most celebrated work, *Fibonacci Furrows* (2024), used a modified GPS-guided seed drill to plant sunflowers in a spiral pattern visible from the air. The planting algorithm was derived from the Fibonacci sequence applied to row spacing, and the resulting field -- when photographed by drone at peak bloom -- produced a golden spiral half a mile wide. The sunflowers were harvested and pressed into oil that was sold at a gallery opening. The oil was the artwork. The field was the frame.

## Origin

The commune was founded in 2021 by Noor Al-Rashid, a landscape architect turned installation artist, and her partner, Tomoko Ishikawa, a roboticist who had spent seven years at Boston Dynamics before leaving to build "machines that create rather than patrol." They bought the failing dairy farm for its acreage and its barn, which they converted into a workshop.

The first year was spent building their primary tool: a retrofitted John Deere planter connected to a custom control system that accepts planting patterns as SVG files. The SVG is converted to GPS waypoints, and the planter executes the pattern in the field. The error tolerance is approximately 3 inches -- enough for art, too much for Tomoko, who is working on a sub-inch system.

Five other artists joined over the following two years, each bringing a different medium: a soil ecologist, a sound artist, a choreographer, a data visualization specialist, and a poet who does the commune's technical writing because, she says, "poetry and documentation are both about precision with words."

## Why This RFP

Seedsong's planting systems generate code: SVG patterns, GPS waypoint sequences, motor control scripts, and sensor calibration files. These artifacts are versioned in Git, but the version control workflow is chaotic. Noor designs a pattern, Tomoko converts it to waypoints, the planter executes, sensors collect data, and the data informs the next iteration. Every step produces files that need tracking, and the current system (manual commits, no review, hope-based coordination) has resulted in three incidents where a planting pattern was executed from the wrong branch, producing a field that was neither the intended art nor useful agriculture.

The `but-ai` plugin offers a structured workflow for their iterative design-execute-measure-revise cycle. The commune sees it as version control for landscape art.

## Team

| Name | Medium | Role in Proposal |
|------|--------|-----------------|
| **Noor Al-Rashid** | Landscape architecture, pattern design | Creative direction, task specification |
| **Tomoko Ishikawa** | Robotics, control systems | Systems architecture, patch generation |
| **Ellis Marsh** | Soil ecology, sensor data | Memory design, environmental context |
| **Rio Vance** | Data visualization | Review, output quality |

## Internal Tension

Noor treats planting patterns as art -- they should be beautiful first and functional second. Tomoko treats them as engineering artifacts -- they should be executable first and beautiful second. The tension produces work that is both, but the arguments about priority are constant. Ellis mediates by pointing out that soil health does not care about aesthetics or engineering elegance, only about outcomes. Rio stays out of it and makes beautiful charts.

## Notable Achievement

*Fibonacci Furrows* was exhibited (as aerial photographs and pressed sunflower oil) at the Whitney Biennial in 2025. It was the first installation at the Biennial created by an autonomous farming system. The catalog description credited both the human designers and the planting algorithm as co-creators. Tomoko was proud. Noor was ambivalent -- she does not think algorithms can be artists. Ellis pointed out that the soil bacteria did most of the actual work.

---

*Planted in the field. Harvested in the gallery. Versioned, eventually.*
