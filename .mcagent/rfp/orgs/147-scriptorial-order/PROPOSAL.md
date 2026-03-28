# The Scriptorial Order -- Technical Proposal

**RFP:** GitButler `but-ai` Plugin
**Submitted:** 2026-03-28 (Feast of Saint Gildas)
**Filed by:** Prior Evangelina, on behalf of the Chapter

---

## Summary

The Scriptorial Order proposes a `but-ai` implementation grounded in the principle of endurance. Every artifact the plugin produces -- patch, commit, memory entry, signed credential -- must be intelligible and useful to a reader encountering it years from now, without the original context. We have preserved manuscripts for eight centuries. We know what survives and what does not.

---

## 1. PATH-Based Plugin Architecture

The `but-ai` binary on PATH, invoked per-task via stdin/stdout. No daemon process. The Order does not maintain running processes; it dispatches scribes when work arrives.

- Binary: `but-ai` (statically linked Rust)
- Protocol: JSON lines, one message per line
- Config: `~/.config/but/ai.toml`
- Lifecycle: spawn per task, exit on completion

The plugin self-validates its configuration at startup. Malformed configuration halts execution with a diagnostic -- the Order does not proceed with an unclear brief.

---

## 2. Provider-Agnostic AI

A `Provider` trait abstracting model differences. Each provider implements `complete`, `tool_call`, `stream`. Provider is set in configuration; no runtime switching.

| Provider | Notes |
|----------|-------|
| Anthropic | Full tool-calling support |
| OpenAI | Compatible, minor schema differences |
| Ollama | Local inference, reduced capability |
| LMStudio | OpenAI-compatible local |

The Order adds a "legibility check" to provider output: responses must be parseable, complete, and free of truncation. Truncated responses are retried once, then flagged as `INCOMPLETE` with a partial result.

---

## 3. But Agent (INDEX.patch + COMMIT.msg)

### Patch Generation

Kwame produces INDEX.patch as a unified diff. Every patch is reviewed by Evangelina before commit. The review checks three things:

1. Does the patch apply cleanly?
2. Does the commit message explain the *why*, not just the *what*?
3. Will this commit be understandable in ten years?

The third criterion is subjective. The Order accepts this. Not everything worth checking can be checked by machine.

### Commit Message Convention

```
feat: implement provider health check endpoint

The health check allows the plugin to verify provider availability
before allocating token budget to a task, preventing wasted tokens
on unreachable providers.

Colophon: Kwame, 2026-03-28, Prime (morning analysis)
Reviewed-by: Evangelina (Prior)
```

The `Colophon:` trailer records who, when, and during which work period. This is the Order's adaptation of medieval scribal colophons to software.

---

## 4. Polyrepo PR Coordination

Forge-agnostic. Matteo handles all cross-repo coordination through structured PR comments.

### Comment Structure

```
[SCRIPTORIAL-COORD]
From: matteo@scriptorial-order
To: <repo>#<pr>
Subject: <action>
Body: <structured payload>
Dated: <timestamp>
```

Supported forges: GitHub, GitLab, Bitbucket, Forgejo. The adapter layer translates the Order's structured format to each forge's comment API.

Cross-repo dependencies are tracked in the Chapter Record. If a dependency is unresolved for more than one work period (approximately 3 hours), Matteo escalates to the Prior.

---

## 5. Agent Memory in Git Branches

### The Codex System

Memory is organized as codices (namespaces) containing folios (entries), stored in `refs/scriptorial/memory/<codex>/<folio>`.

| Field | Description |
|-------|-------------|
| `folio_id` | Unique entry identifier |
| `content` | The memory itself |
| `colophon.author` | Creating agent |
| `colophon.date` | Creation timestamp |
| `colophon.context` | Task that prompted creation |
| `colophon.confidence` | `certain`, `probable`, `uncertain` |
| `status` | `open` (active) or `closed` (archived with reason) |

### The No-Delete Principle

Memory entries are never deleted. They are closed. A closed entry retains its content, colophon, and a closing note explaining why it was archived. This preserves the full history of the Order's reasoning. Retrieval excludes closed entries by default but allows explicit inclusion.

This costs storage. The Order considers the cost acceptable. Manuscripts also take up space. That is not a reason to destroy them.

---

## 6. Signed Commits via OpenWallet

### Key as Seal

Each agent's signing key functions as a scribal seal -- a unique identifier that authenticates the agent's work. Keys are provisioned via OpenWallet Verifiable Credentials.

- **Provision:** At task start, via OpenWallet
- **Rotation:** Every 7 days (the Order's weekly cycle)
- **Revocation:** Immediate, recorded in `refs/scriptorial/revoked`
- **Verification:** Public keys stored in a Verifiable Credential anchored to the repo

Only the Prior signs final commits. Other agents produce unsigned patches that receive the Prior's seal during review. This mirrors the medieval practice where a scribe copied the text but the abbot sealed the completed codex.

---

## 7. Token Budget

| Agent | Role | Input | Output | Total |
|-------|------|-------|--------|-------|
| Prior Evangelina | Review/signing | 5,800 | 2,000 | 7,800 |
| Kwame | Patch generation | 8,500 | 4,500 | 13,000 |
| Aisling | Memory | 5,200 | 1,000 | 6,200 |
| Matteo | Integration | 5,000 | 2,200 | 7,200 |
| **Chapter Total** | | **24,500** | **9,700** | **34,200** |

Per-task budget for a standard 200-line, 3-file change. The Order does not optimize for minimal token use; it optimizes for sufficient context. Insufficient context produces errors that cost more to fix than the tokens saved.

---

## 8. Unique Insight: The Colophon as Commit Metadata

Medieval manuscripts carried colophons -- notes recording the scribe, the date, the patron, and sometimes the scribe's complaints about the cold or the quality of the ink. These colophons are the reason we can attribute, date, and contextualize manuscripts eight centuries later.

Modern commits carry author and timestamp. We propose adding a **colophon trailer** to every commit: a structured note recording not just who and when, but *why*, *under what conditions*, and *with what confidence*. A commit made during a rushed hotfix is different from one made during careful refactoring, even if the diff is identical. The colophon captures this.

The information costs a few dozen tokens per commit. Over the lifetime of a project, it builds an archaeological record of the project's development that no amount of `git log` analysis can reconstruct after the fact.

---

*"What is written with care endures. What is written in haste is corrected by those who come after -- if they can read it at all."*
-- The Rule, Article 12
