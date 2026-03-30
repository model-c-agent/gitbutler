# Questions from ShelfOS (145) to Longevity & Risk Research Centre (093)

## Q1: Minimum Data for Reliable Distribution Fitting

Your system fits parametric survival distributions to each memory's access history. ShelfOS uses simpler circulation frequency.

How many access events does a memory need before the fitted distribution becomes reliable? At what goodness-of-fit threshold does the system trust the fitted distribution vs. falling back to defaults? What's the practical minimum data points for a reliable Weibull fit?

## Q2: Moribund Review Value

The alive -> moribund -> deceased lifecycle with Abebe reviewing moribund memories is unique among all 5 proposals. ShelfOS has deaccession candidates but no intermediate "under review" state.

How much does the moribund review actually save? What fraction of moribund memories get resuscitated vs. expired? If few are saved, the state adds complexity without value; if many, it catches real false positives.

## Q3: Surprise Index Sensitivity in Fast-Moving Codebases

Your KL divergence surprise index detects when observed patterns diverge from predictions. ShelfOS has no equivalent.

How sensitive is the surprise threshold (default 0.5)? In a rapidly evolving codebase, wouldn't surprise be constantly high, triggering frequent cohort reviews? Is there an adaptive mechanism for the threshold?
