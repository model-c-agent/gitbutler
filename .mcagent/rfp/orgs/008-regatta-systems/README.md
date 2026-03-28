# Regatta Systems

**"Read the wind. Adjust the sails. Win the race."**

---

## Origin Story

Regatta Systems was founded in 2023 by four members of the New Zealand America's Cup analytics team after their syndicate lost in the final round. The loss stung, but not because of the sailing — because of the software. Their weather-routing algorithm had recommended a conservative tack sequence in the deciding race, and the helmsman followed it. The opposing team's algorithm had recommended an aggressive gybe that exploited a wind shift their system had classified as "too uncertain to act on." The other team won by 14 seconds.

After the campaign ended, the four analysts — two meteorologists, a naval architect, and a systems engineer — sat in a bar in Barcelona and agreed: the problem wasn't the algorithm. The problem was that the algorithm couldn't update its model fast enough. By the time it recalculated the optimal route, the wind had shifted again. They needed an adaptive system — one that made decisions continuously, not in batch.

They left competitive sailing and founded Regatta Systems. Their first client was a container shipping line that wanted real-time route optimization across the North Atlantic. The analysts applied their racing methodology: treat every route decision as a tack, every market disruption as a wind shift, and optimize continuously. Route efficiency improved 15% in the first season.

## The Racing Mindset

In yacht racing, you never sail the plan. You sail the conditions. The plan is a starting point; the wind, current, and competitors dictate what actually happens. Our approach to software follows this: design the system, then let reality adjust it in real time.

This means our agents are adaptive by default. They don't execute a fixed plan — they execute a strategy that adjusts as new information arrives. If a tool call returns unexpected results, the agent re-evaluates its approach mid-task. If memory retrieval surfaces a relevant pattern the agent didn't anticipate, the agent changes course. In racing terms: the agent is always looking upwind.

## How We Got Into AI Agents

Our real-time route optimization system already behaved like a multi-agent system — independent modules monitoring weather, traffic, fuel consumption, and port schedules, all feeding into a central decision engine. When we rewired it to use LLM-based agents instead of hand-coded heuristics, the improvement was immediate: the agents could handle novel situations (pirate warnings, unexpected port closures, sanctions compliance) that our heuristics had no rules for.

The version control need emerged when we deployed agents across multiple shipping clients simultaneously. Each client's agents were customizing their behavior based on the client's fleet and routes. Those customizations needed tracking, branching, and merging — especially when a general improvement in one client's configuration should propagate to others.

## Internal Tension

The team splits on pre-race planning. Two members (the meteorologists) believe in extensive pre-computation: model the conditions, generate a set of contingency plans, and select among them in real time. The other two (the architect and the engineer) believe pre-computation is waste — the conditions will be different from the forecast, so just react. The current system does both, which nobody loves.

## Notable Achievement

Our North Atlantic route optimization, deployed across a fleet of 23 bulk carriers, reduced average voyage fuel consumption by 11.4% over the 2024 season. The system made 340,000 route adjustment recommendations, of which 97.2% were accepted by the crew. The 2.8% rejected recommendations were analyzed post-voyage — in 60% of cases, the crew's override was correct (local knowledge the system lacked), and in 40%, the system's recommendation would have been better.

## Team Overview

Five agents modeled on a racing crew. One tactician sets strategy and adapts the plan as conditions change. One navigator manages context, memory, and positioning (workspace state). One trimmer optimizes execution — producing the tightest, most efficient patches. One bowman handles the complex up-front work (forge coordination, PR management). One helmsman makes final decisions and signs commits. They operate in a fast cycle: observe conditions, adjust plan, execute, repeat.

---

*"The wind does not care about your plan."*
