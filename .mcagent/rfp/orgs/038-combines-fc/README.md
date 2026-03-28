# Combines FC

**"First to the finish line. Every acre. Every season."**

---

## The League

Combines FC is a competitive harvesting team based in Wichita, Kansas, that races autonomous combine harvesters in the North American Autonomous Harvest League (NAAHL). The league, founded in 2023, pits teams of autonomous combines against each other on standardized test fields: 100-acre parcels of wheat, timed from first cut to last grain in the hopper. The fastest team with less than 2% grain loss wins.

Combines FC was founded by retired NASCAR pit crew chief Darnell "DT" Thompson and agricultural engineer Yolanda Reyes. DT brought the racing mentality: preparation, precision, and the belief that every tenth of a second matters. Yolanda brought the farming knowledge: crop conditions, machine capabilities, and the understanding that grain is not a checkered flag — you cannot go faster by damaging the product.

The team has competed in every NAAHL season since 2023. They placed 3rd in the inaugural season, 2nd in 2024, and 1st in 2025, setting a record of 4 hours 12 minutes for a 100-acre wheat harvest with 0.8% grain loss. DT framed the trophy. Yolanda framed the grain loss report.

## The Pit Crew

The team operates like a racing pit crew: every member has a specialized role, every action is rehearsed, and race-day execution is the product of months of preparation. The "off-season" (November through April) is spent tuning algorithms, calibrating sensors, and running simulations.

The combines themselves are modified Case IH 9250s with custom autonomy packages: LIDAR for obstacle detection, GPS-RTK for sub-inch positioning, and a custom harvest optimization algorithm that adjusts cutting height, ground speed, and header position in real time based on crop density sensors.

## Why This RFP

The team's software stack generates hundreds of configuration changes per season: combine parameters, field maps, harvest strategies, sensor calibrations. These are currently managed through a Git repository with a branching strategy that DT designed to mirror pit stop workflows. The `but-ai` plugin would automate the routine configuration changes (sensor recalibration after field changes, parameter adjustments for crop conditions) and free the engineers to focus on race strategy.

## Team

| Name | Pit Role | Technical Focus |
|------|----------|----------------|
| **Darnell "DT" Thompson** | Crew Chief | Strategy, coordination |
| **Yolanda Reyes** | Lead Engineer | Harvest algorithms, patch generation |
| **Kwame Asante** | Telemetry | Data analysis, memory systems |
| **Jin Park** | Systems | Infrastructure, signing, provider config |

## Internal Tension

DT optimizes for speed. Yolanda optimizes for grain quality. These goals conflict directly: faster ground speed means more grain loss. The team's competitive edge comes from Yolanda's algorithm that finds the optimal speed for each section of the field based on crop density — but DT always wants to push it faster. Their argument after every race is the same: DT says they could have been 10 minutes faster. Yolanda says they could have lost 0.3% less grain. The data always supports both of them.

## Notable Achievement

At the 2025 NAAHL Championship in Salina, Kansas, Combines FC set the all-time record. The winning margin was 22 minutes over the second-place team, with the lowest grain loss in the field. The key was Yolanda's adaptive speed algorithm, which reduced ground speed by 15% in high-density sections and increased it by 20% in low-density sections — a strategy no other team had attempted because it requires real-time crop density sensing with <100ms latency. The trophy is in DT's garage. The algorithm is in the Git repository.

---

*First to finish. Least to lose. That is the Combines FC way.*
