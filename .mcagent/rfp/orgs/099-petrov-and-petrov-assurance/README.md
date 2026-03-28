# Petrov & Petrov Assurance

**"We agree on the math. We disagree on everything else."**

---

## The Firm

Petrov & Petrov Assurance is a two-person actuarial consultancy in Sofia, Bulgaria, run by identical twin brothers Nikolai and Alexei Petrov. They are 44 years old. They share an office. They share clients. They share a surname on the door. They do not share opinions on discount rates, mortality table selection, reserve methodology, or the correct temperature for office thermostat settings.

The firm was founded in 2015 after both brothers left PricewaterhouseCoopers on the same day -- a coincidence they each claim was their idea first. They specialize in pension fund valuations and insurance reserve opinions for mid-tier Eastern European insurers. Revenue is stable. Growth is nonexistent, because growth would require hiring a third person, and neither brother trusts anyone else's actuarial judgment.

Their mother, Dr. Ivanka Petrov (retired professor of statistics at Sofia University), still calls every Friday to ask if they have reconciled their positions on the Bulgarian mortality table. They have not.

## How Software Happened

Both brothers are competent programmers -- you cannot be a modern actuary without writing code. But they write different code. Nikolai prefers R. Alexei prefers Python. Their shared codebase is a bilingual abomination that somehow works because both brothers are too stubborn to rewrite the other's modules.

Version control was essential from day one because the brothers overwrite each other's files. Git helped. GitButler helped more, because virtual branches let them work on the same valuation model simultaneously without the daily merge conflicts that had become a source of argument indistinguishable from their other arguments.

The `but-ai` RFP interested both brothers for different reasons. Nikolai wants AI to automate report generation (the boring part). Alexei wants AI to validate his models against Nikolai's models (the argumentative part). Both are valid use cases.

## Philosophy

They agree on exactly one philosophical principle: actuarial work must be reproducible. Given the same data and the same assumptions, two actuaries should produce the same result. When they produce different results (which happens often, because they choose different assumptions), the disagreement is meaningful and must be resolved through analysis, not authority.

This principle extends to AI agents: an agent must produce reproducible output, and when two agents disagree, the disagreement must be explicable.

## Internal Tension

**Everything.** The brothers argue about everything. The productive tension is the argument about discount rates for pension valuations: Nikolai favors a market-consistent approach (use the current yield curve), Alexei favors a best-estimate approach (use a long-term average). Their clients receive both valuations with a note explaining the difference. This is actually their competitive advantage -- clients get two independent opinions for the price of one firm.

## Notable Achievement

In 2025, the firm's dual-valuation approach caught a significant error in a Bulgarian pension fund's reserves. Nikolai's market-consistent valuation showed the fund was solvent; Alexei's best-estimate valuation showed it was underfunded by 12%. The difference was due to an unusually inverted yield curve that flattered the market-consistent result. The fund's board, presented with both numbers, chose to strengthen reserves. Alexei reminded Nikolai of this for three months. Nikolai has not forgiven him.

## Team Overview

| Agent | Role | Brother |
|-------|------|---------|
| Nikolai | Patch Gen (R modules) / Review (Python) | Elder (by 4 minutes) |
| Alexei | Patch Gen (Python modules) / Review (R) | Younger |
| Ivanka-bot | Memory / Arbitration | Mother (automated oracle) |
| Borislav | Forge Coordination | Nephew (part-time) |
| Desislava | Security, Budget, Signing | Office manager |

Details in [AGENTS.md](AGENTS.md).

---

*"Two brothers. Two opinions. One firm. Somehow it works."*
