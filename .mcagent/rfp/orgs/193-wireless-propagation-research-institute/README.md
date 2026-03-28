# The Wireless Propagation Research Institute

**"The signal does not care about your model. The signal cares about the building."**

---

## About the Institute

The Wireless Propagation Research Institute (WPRI) is an independent research laboratory affiliated with ETH Zurich, funded by the Swiss National Science Foundation and a consortium of European telecom operators. The Institute studies how radio waves propagate through complex environments — urban canyons, indoor spaces, tunnels, forests, and the increasingly dense clutter of modern cities where every new building changes the RF landscape.

WPRI was established in 2017 by Prof. Elena Marchetti, a physicist specializing in electromagnetic wave propagation who became frustrated with the gap between theoretical propagation models and real-world measurements. Standard models (Okumura-Hata, COST-231, 3GPP TR 38.901) work well for open terrain and regular urban grids. They fail spectacularly in the irregular geometries of European cities, where a 14th-century church reflects signals differently than a 21st-century glass tower, and a narrow alley can act as a waveguide that carries 5G millimeter waves two blocks further than any model predicts.

The Institute maintains a fleet of 24 mobile measurement units — modified electric cargo bikes equipped with spectrum analyzers, GPS, LIDAR, and weather sensors. Research assistants ride these bikes through cities, collecting propagation measurements that are fed into the Institute's models. The bikes are a source of quiet institutional pride and loud complaints from the research assistants about hills.

The research team consists of Prof. Marchetti, three postdoctoral researchers, two PhD students, and a research software engineer who maintains the Institute's computational infrastructure.

## Path to AI Agent Tooling

In 2024, the Institute received a grant from the EU Horizon Europe program to develop "AI-assisted urban radio planning tools." The proposal: train agents on the Institute's measurement database (14 million data points across 30 cities) to predict propagation characteristics for new deployment sites without requiring field measurements at every location.

The agents work by analyzing building geometry (from OpenStreetMap and LIDAR data), material composition (estimated from building age and type), and existing measurement data from nearby locations. They produce propagation predictions as structured maps — essentially, patches to the Institute's coverage database that predict signal strength at unmeasured points.

These predictions needed version control. When a new building is constructed (or demolished), the propagation predictions for the surrounding area change. The Institute needed to track which predictions were based on which building data, so that when the physical environment changed, stale predictions could be identified and regenerated. Git's diff and blame capabilities were ideal. GitButler's virtual branches allowed the team to maintain simultaneous prediction sets for different scenarios (e.g., "with proposed tower" and "without proposed tower").

## Philosophy

Models are approximations. Measurements are truth. When the model disagrees with the measurement, the model is wrong. This principle extends to AI agents: an agent's prediction is a hypothesis, not a fact. Every prediction must be tagged with its uncertainty estimate, and predictions with high uncertainty are flagged for field verification.

The Institute does not trust black-box models. Every agent prediction must be explainable — the agent must identify which input features (building height, material, distance, frequency, weather) contributed most to the prediction. Unexplainable predictions are discarded.

## The Zurich North Tower Incident

In June 2025, the Institute's prediction agent forecast excellent 5G coverage for a new residential district in Zurich North, based on the planned building layout. A telecom operator used the prediction to plan a base station deployment. After deployment, actual coverage was 18dB below prediction — effectively unusable. Investigation revealed that the prediction was based on building plans that showed glass facades. The buildings were actually constructed with metallized glass (for energy efficiency) that attenuated millimeter waves by 25dB. The agent's building material model did not distinguish between glass types.

The incident cost the operator an estimated CHF 400,000 in additional infrastructure. The Institute added material subtype classification to the agent's feature set and now requires field measurement validation for any prediction that informs infrastructure investment exceeding CHF 100,000.

## Achievement

**14 million measurement data points**: The Institute's propagation database is the largest independent measurement dataset in Europe, covering 30 cities across 8 countries. The database is the foundation for the Institute's agent models and is cited in 47 peer-reviewed publications.

## Team

| Member | Title | Role |
|--------|-------|------|
| Prof. Elena Marchetti | Director | Research direction, model validation |
| Dr. Kenji Tanaka | Postdoc | Propagation modeling, agent training |
| Dr. Amara Diallo | Postdoc | Measurement analysis, uncertainty quantification |
| Dr. Lukas Brenner | Postdoc | Urban geometry processing, LIDAR integration |
| Sofia Petridou | PhD Student | Agent architecture, plugin development |
| Ravi Sundaram | Research Engineer | Infrastructure, memory systems, deployment |

Details in [AGENTS.md](AGENTS.md).

---

*"Every model is wrong. Some models are useful. Ours come with error bars."*
— Prof. Marchetti, WPRI Annual Report 2025
