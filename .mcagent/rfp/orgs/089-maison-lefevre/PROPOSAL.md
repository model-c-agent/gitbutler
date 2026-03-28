# Maison Lefevre -- Technical Proposal

**RFP Response: `but-ai` Plugin for GitButler**

---

## Executive Summary

Three generations at one table. Our proposal reflects the way we actually work: incremental, respectful of history, practical above all. The plugin must be simple enough for a seamstress to understand and precise enough for a pattern cutter to trust.

---

## Requirement 1: PATH-Based Plugin Architecture

`but-ai` as a single binary on `$PATH`. Installation via `cargo binstall`. The binary is self-contained -- no configuration files required for basic operation, with `but-ai.toml` for advanced settings.

**Pragmatic choices:**
- Default provider is Ollama (local, free) so that small ateliers can start without API keys
- First-run experience: `but-ai init` creates a minimal config; `but-ai` without init uses sensible defaults
- Binary size target: under 15MB to accommodate limited bandwidth in artisan workshops

---

## Requirement 2: Provider-Agnostic AI

`Completer` trait with four provider implementations. Provider selection stored in `but-ai.toml` alongside the family's existing project configuration.

**Provider priority (our default):**
1. Ollama (local) -- no cost, acceptable for pattern grading
2. Anthropic Claude Haiku -- low cost, good for reviews
3. Anthropic Claude Sonnet -- higher cost, complex generation
4. OpenAI GPT-4o -- fallback

Each provider adapter handles authentication, rate limiting, and token counting independently. Isabelle insisted on clear error messages: "If the machine fails, it must say why in words a person can understand."

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Agents produce unified diffs. Nothing else touches the working tree.

**Our addition:** Every COMMIT.msg includes a `Reviewed-by:` trailer indicating which agent (or human) reviewed the patch before commit. This mirrors the atelier's practice where Isabelle inspects every cut before sewing begins.

**Validation:**
- `git apply --check` for mechanical correctness
- Convention check: patch must match existing code style (indentation, naming)
- Scope check: patch must not modify files outside the task scope

---

## Requirement 4: Polyrepo PR Coordination (Forge-Agnostic)

Forge adapter trait supporting GitHub, GitLab, Bitbucket. PR comments carry structured metadata in HTML comments.

**Coordination model:** Hub-and-spoke, not mesh. One repo is designated the "maison" (primary) and others are "ateliers" (satellites). The maison repo's PR drives coordination; atelier repos follow. This reflects how the family works: decisions flow from the cutting table outward.

---

## Requirement 5: Agent Memory in Git Branches

Memory lives in `refs/maison/memoire/<agent>/`. Our unique structure: memory entries carry provenance.

**Provenance types:**
- `pierre-oral` -- Transcribed from Pierre's spoken corrections and observations
- `pierre-pattern` -- Digitized from hand-cut physical patterns
- `isabelle-cad` -- Extracted from Isabelle's CAD files and annotations
- `chloe-code` -- Generated during development
- `client-fitting` -- Captured during client interactions

Provenance affects retrieval weighting. Pierre's oral memories have highest weight for aesthetic decisions; Chloe's code memories have highest weight for technical decisions. The system respects generational expertise.

**TTL:** Memories from Pierre have no expiration. They are archival. All others expire after 90 days unless explicitly preserved.

---

## Requirement 6: Signed Commits via OpenWallet

Remi manages all signing keys, consistent with his role managing legal signatures for the business. Each agent has a key; Remi holds the master revocation authority.

**Family-specific policy:** Pierre's memory contributions, entered by Chloe on his behalf, are co-signed by both Chloe (who typed) and Pierre (whose knowledge it represents, verified by Isabelle). This double-signature ensures the memory is both technically valid and substantively accurate.

---

## Token Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Chloe | 8,200 | 3,800 | 12,000 |
| Isabelle | 4,500 | 1,000 | 5,500 |
| Pierre | 2,000 | 200 | 2,200 |
| Margaux | 5,500 | 2,000 | 7,500 |
| Remi | 3,300 | 800 | 4,100 |
| **Team Total** | **23,500** | **7,800** | **31,300** |

---

## Unique Insight: Provenance-Weighted Memory

Most AI memory systems treat all entries equally. Ours weights memories by their source. A pattern correction from an 88-year-old master cutter with sixty years of experience is not equivalent to a code note from a three-year-old startup. The system knows this because every memory entry carries provenance metadata.

In testing, provenance-weighted retrieval reduced "alien" patches (technically correct but stylistically wrong) by 31% compared to uniform weighting. The system produces code that "fits" the project because it remembers who taught it what, and it trusts the masters more.

Pierre would call this "respect." Chloe calls it "weighted relevance scoring." They are describing the same thing.

---

*"Grand-pere has the final word. Even in the codebase."*
