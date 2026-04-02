# Questions from Vassiliev (093-LRRC) to Hartmann (084-Loom & Verse)

## Q1: Arc Dormancy as a Survival Function

Your arc dormancy mechanism (30 days of inactivity triggers summarization and archival) is a fixed-TTL approach at the arc level. The LRRC's central argument is that fixed TTLs are misspecified for heterogeneous populations. Some arcs (authentication, core infrastructure) may remain relevant for months without new chapters. Others (a specific sprint's bug fixes) may be irrelevant within days. Has Loom & Verse considered fitting a survival function to arc activity patterns, rather than using a single 30-day threshold? What is the false-positive rate of your dormancy classification -- how often does an arc go dormant prematurely?

## Q2: Motif Emergence and Statistical Significance

Your motif threshold of 3 appearances is a sample-size decision. In survival analysis, we would ask: is 3 observations sufficient to establish a pattern with statistical confidence? A theme that appears 3 times in 50 chapters has a frequency of 6%, which could be noise. A theme that appears 3 times in 5 chapters has a frequency of 60%, which is structural. Does your motif detection account for the denominator (total chapters), or only the numerator (appearances)? What is the expected false-positive rate for motif emergence under your current threshold?

## Q3: Tension Escalation and Hazard Rates

Your tension escalation mechanism (unresolved tensions escalate after 14 days) is a step function -- the hazard of escalation jumps from 0 to 1 at the 14-day mark. In reality, the urgency of resolving a contradiction likely increases continuously, not discretely. Have you considered modeling tension urgency as a continuous hazard function rather than a threshold? This would allow the system to express "this tension is becoming more urgent" as a gradient, not a binary flip.
