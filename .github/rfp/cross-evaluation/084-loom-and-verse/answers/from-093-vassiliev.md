# Answers from Hartmann (084) to Vassiliev (093)

## A1: Arc dormancy and survival functions

You have accurately diagnosed the weakness. Our 30-day fixed dormancy threshold is exactly the kind of uniform TTL that your proposal argues against. An authentication arc that has been stable for 45 days is not less relevant than a sprint bug-fix arc that has been quiet for 10 days. The fixed threshold treats them identically.

I concede this point fully. The integrated proposal should replace the fixed dormancy threshold with a fitted survival function per arc. Your Weibull model is the right tool: arcs about infrastructure (k > 1, slowly increasing hazard) should have much longer expected dormancy times than arcs about specific bug fixes (exponential, short half-life).

The false-positive rate of our dormancy classification is unknowable from the current implementation because we have no historical data on arc reactivation. This is itself a statistical failure -- we make a classification decision without calibrating it against observed outcomes. The LRRC's approach of tracking deceased memories as training data for improving future fits is directly applicable here. The integrated proposal should track arc dormancy events and reactivation events to calibrate the dormancy survival function.

Practical implementation: `ArcManager` should carry a per-arc `SurvivalDistribution` (imported from your survival module) fitted to the arc's chapter arrival times. Dormancy is triggered when the predicted probability of a new chapter within the next 30 days drops below a threshold (e.g., 10%), not when a fixed clock expires.

## A2: Motif emergence and statistical significance

You have identified a real statistical weakness. Our motif threshold of 3 is an absolute count with no denominator correction. Three appearances in 5 chapters (60% frequency) is structurally meaningful. Three appearances in 500 chapters (0.6% frequency) could be noise.

The correct approach is a frequency-based threshold rather than an absolute count. The `MotifConfig` should specify a `min_frequency: f64` (e.g., 0.05 for 5%) in addition to `emergence_threshold: u32`. A theme is promoted to a motif when:

```
appearances >= emergence_threshold AND (appearances / total_chapters) >= min_frequency
```

This means:
- In a 5-chapter project, 3 appearances (60%) easily clears both thresholds.
- In a 500-chapter project, 3 appearances (0.6%) does not clear the frequency threshold. The theme would need 25 appearances to reach 5%.

The expected false-positive rate under the current threshold is high for large chapter counts and low for small chapter counts, which is exactly the wrong behavior (false positives matter more in large projects where they pollute a richer retrieval space).

I note that your surprise index could serve as a complementary signal: if a theme's appearances are clustered in time (suggesting a real pattern) versus uniformly distributed (suggesting noise), the distribution of inter-appearance gaps would distinguish them. This is another contribution your framework makes that the integrated proposal should adopt.

## A3: Tension escalation as a continuous hazard

You are completely right, and I find your framing elegant. The current tension escalation is a step function at 14 days -- urgency is 0 before and 1 after. This is a poor model of reality.

The `tension_urgency` component of our relevance score should be modeled as a continuous function of time since tension introduction. Your Weibull hazard model is again the right tool:

```
tension_urgency(t) = (k / lambda) * (t / lambda)^(k-1)
```

where `t` is days since tension introduction, `k > 1` (urgency increases over time), and `lambda` is calibrated to the escalation threshold. With `k = 2.0` and `lambda = 14 days`, the urgency at day 7 is 0.071, at day 14 is 0.143, at day 28 is 0.286 -- a smooth gradient rather than a binary flip.

The `TensionState::Escalated` variant would then trigger at a hazard threshold (e.g., h(t) > 0.1) rather than at a fixed day count. This preserves the escalation concept while making the urgency gradient visible to the retrieval system throughout the tension's lifecycle.

This is a direct import from your survival module into our tension system, and it is clearly an improvement. The `TensionConfig` should carry the Weibull parameters for the tension urgency function.

I note with some irony that your system, which lacks a tension concept entirely, has provided the mathematical framework that would make our tension concept work properly. The integrated proposal should combine your survival mathematics with our narrative tension tracking.
