# BPM United -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Formation:** 1-2-1
**Match Date:** 2026-03-28

---

## Pre-Match Brief

We are a five-person production crew that builds software the way we make music: fast, collaborative, and with the ball always moving forward. Our proposal is not the most sophisticated. It is the one most likely to ship on time, work in production, and be maintainable by a small team.

We know our lane. We are not academics, not a startup with VC runway, not a government agency. We are five people in a warehouse who are very good at passing the ball.

---

## Requirement 1: PATH-Based Plugin Architecture

Binary on PATH. `but-ai` discovered by `but` through standard executable lookup. Configuration in `~/.config/but-ai/config.toml`, created interactively on first run (`but-ai init`). The init flow asks three questions: which provider, which model, and whether to enable the persistent daemon. That is it. Three questions, working system.

The daemon (`but-ai daemon`) is optional but recommended. It keeps provider connections warm and pre-loads the system prompt. Cold start: ~400ms. Warm start with daemon: <80ms.

---

## Requirement 2: Provider-Agnostic AI

Four providers behind a `Provider` trait: `complete`, `complete_with_tools`, `token_count`. Three methods. We argued about whether to add `stream` and decided against it -- streaming adds UI complexity and our review pipeline processes complete responses, not fragments.

Provider selection is per-session. Hot-switching is supported with a warning: "Switching providers mid-match may change results." We support it because in practice, OpenAI goes down at 3 PM on a Tuesday and you need to fall back to Ollama. Pragmatism over purity.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Midfielder generates the patch. Process:

1. Read task description and relevant file contents
2. Read memory entries tagged for the current codebase
3. Generate INDEX.patch (unified diff format)
4. Generate COMMIT.msg (summary + body + trailers)
5. Validate: `git apply --check` on the patch
6. Submit to Goalkeeper for review

Patches are scoped to the task. No drive-by refactors, no cleanup commits, no "while I was here" changes. Each patch does one thing. Leo's rule: "If you cannot describe the patch in one sentence, it is two patches."

### COMMIT.msg Format

```
<one-line summary, imperative mood>

<body: what and why, 2-5 lines>

Position: <agent role>
Budget: <used>/<allocated>
```

The `Position:` trailer records which agent produced the commit. Future agents reading the log can see the formation that produced any given piece of work.

---

## Requirement 4: Polyrepo PR Coordination

Cross-repo coordination via PR comments with an inline schema:

```
[bpm:coord] from=repo-a@branch to=repo-b@branch type=depends status=pending
```

Single-line format. Parseable by regex. No JSON, no YAML, no schema documents. We keep it simple because cross-repo coordination is already complex enough without adding serialization debates.

Forge abstraction: a `Forge` trait with `post_comment`, `read_comments`, `pr_status`. Three methods. We implement GitHub first and add others when someone actually needs them. YAGNI applies to forge adapters too.

Dependency tracking is a simple list in memory. No DAG, no topological sort. If repo-a depends on repo-b, repo-a waits until repo-b's status is `merged`. If there is a circular dependency, the squad has made a tactical error and Manager re-forms.

---

## Requirement 5: Agent Memory in Git Branches

Memory in `refs/but-ai/mem/<agent>/<key>`. Entries are plaintext with a header line:

```
ttl=5d tags=rust,error-handling position=midfielder
```

One header line, then content. No YAML frontmatter, no JSON metadata. The header is parseable by splitting on spaces and splitting each field on `=`.

Retrieval: tag matching. Current task is tagged (automatically from file extensions and directory names). Memory entries with overlapping tags are returned, ranked by overlap count. Simple, fast, deterministic.

### Match Reports

After every task, Physio writes a match report to memory. The report summarizes: what the task was, which formation was used, how many tokens were consumed, whether Goalkeeper parried or saved, and any lessons for future tasks. These reports are the primary mechanism for improving over time.

---

## Requirement 6: Signed Commits via OpenWallet

Goalkeeper signs all commits using an OpenWallet DID. Key rotation every 48 hours. Signing flow:

1. Goalkeeper verifies patch has been reviewed (Goalkeeper's own review, so this is a formality)
2. Goalkeeper requests signing credential from wallet
3. Commit is signed with Ed25519 key
4. Credential ID and expiry are recorded in commit trailer

We keep the signing flow minimal. No separate signing agent, no authorization chains. Goalkeeper reviews and signs in the same step because in a four-agent squad, splitting these roles would create more overhead than risk.

---

## Token Budget

| Agent | Input | Output | Total | Position |
|-------|-------|--------|-------|----------|
| Manager | 5,000 | 2,000 | 7,000 | Coordination |
| Midfielder | 10,000 | 7,000 | 17,000 | Architecture + patches |
| Goalkeeper | 6,000 | 2,000 | 8,000 | Review + signing |
| Physio | 4,000 | 1,000 | 5,000 | Memory |
| **Squad** | **25,000** | **12,000** | **37,000** | |

Lean budget. Small squad, tight formation. No wasted passes.

---

## Unique Insight: Formation as Configuration

Every team sport has discovered the same truth: the same players in different formations produce different results. A 4-4-2 is not better or worse than a 3-5-2 -- it is suited to different opponents and different pitch conditions.

Our insight is that agent teams work the same way. The same four agents, configured differently -- different token allocations, different review policies, different memory TTLs -- produce different quality/speed tradeoffs. Our proposal does not hardcode a formation. It parameterizes one.

The `but-ai` configuration file includes a `[formation]` section where the user specifies: how many review rounds, what memory TTL, how much budget per agent role, and whether to enable speculative execution. This means a user can run our agents in a defensive formation (more review, more memory, slower) for critical work and an attacking formation (less review, less memory, faster) for exploratory work. Same squad, different tactics.

This is not novel computer science. It is what every football manager has known for a century: the formation is the strategy.

---

*Match report will be filed by Physio after final whistle. COYB.*
