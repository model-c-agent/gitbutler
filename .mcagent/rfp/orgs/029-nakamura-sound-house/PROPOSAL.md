# Nakamura Sound House -- Technical Proposal

**RFP:** `but-ai` Plugin for GitButler
**Family Submission:** 2026-03-28

---

## Why We Are Here

We are a four-person family business that builds mixing consoles by hand. We are not a software company. We are responding to this RFP because the `but-ai` plugin solves a problem we care about deeply: making knowledge survive the people who carry it.

Our proposal is small, simple, and honest about its limitations. We do not have a distributed systems team or a formal verification specialist. We have Yuki, who writes Rust, and Aiko, who helps on weekends.

---

## Requirement 1: PATH-Based Plugin Architecture

The plugin is a single static binary. `but-ai` on PATH. Configuration in `~/.config/but-ai/config.toml`. First-run setup creates the config with defaults and prints a brief explanation of each option.

We build statically because our own workshop taught us the cost of dependencies. A console with a custom component that goes out of production becomes a console you cannot repair. A binary with a dynamic library that changes behavior between OS versions becomes a binary you cannot trust.

Startup: <300ms. No daemon. The daemon pattern adds operational complexity that a small team cannot afford to maintain.

---

## Requirement 2: Provider-Agnostic AI

Four providers: OpenAI, Anthropic, Ollama, LMStudio. A trait with four methods: `init`, `complete`, `complete_with_tools`, `token_count`.

We weight local providers (Ollama, LMStudio) equally with cloud providers in our design. Yuki's development happens on a workstation in the third-floor apartment. Internet connectivity is not always reliable (the building is old; the wiring is shared with the temple supply shop). A plugin that requires cloud access to function is a plugin that cannot be used on days when the router decides to take a break.

No automatic fallback. If the configured provider fails, the operation fails with a clear error. Automatic fallback changes model behavior silently, which violates our principle of explicit documentation.

---

## Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

Chisel reads context, generates a unified diff, and writes a commit message. The process is sequential:

1. Read task description
2. Read relevant files (scoped by task -- no full-codebase reads)
3. Retrieve relevant memory entries
4. Generate INDEX.patch
5. Validate with `git apply --check`
6. Generate COMMIT.msg
7. Submit to Caliper for review

### Scoped Context

We do not read entire codebases. We read the files mentioned in the task and their immediate neighbors (files that import/are imported by the target files). This keeps token costs predictable and prevents the context window from filling with irrelevant code.

Kenji taught Yuki: "You do not need to understand the entire console to replace a capacitor. You need to understand the circuit the capacitor is in." We apply this to code.

---

## Requirement 4: Polyrepo PR Coordination

Minimal implementation. Cross-repo coordination uses PR comments with a structured header:

```
[nsh:link] repo=other-repo branch=feat/x relation=depends-on
```

Single line. Parseable. Human-readable.

Forge abstraction: three methods (`post_comment`, `read_comments`, `pr_status`). We implement GitHub. Other forges can be added by implementing the trait. We will not build what we do not need today.

We are honest: polyrepo coordination is not our strength. We work in one repository. Our contribution here is a clean, minimal interface that does not over-promise.

---

## Requirement 5: Agent Memory in Git Branches

Memory stored in `refs/but-ai/memory/<agent>/<key>`. Entries are plain text with a single metadata line:

```
created=2026-03-28 ttl=3d tags=component,power-supply
```

Memory organization mirrors how we organize build specifications: by component subsystem. A memory entry about error handling is tagged with the module it relates to, just as a build spec for a power supply is filed under "power supply."

### TTL: 3 days default

Short TTL. We work in small batches and value fresh context over accumulated history. Memory that survives longer than one build cycle (typically 2-3 days for us) is likely stale.

### Retrieval

Tag-based matching. No vector embeddings. No similarity scoring. Tags match or they do not. This is the software equivalent of Kenji's filing system: physical folders in a physical cabinet, labeled by hand. It is not sophisticated. It has not failed in 30 years.

---

## Requirement 6: Signed Commits via OpenWallet

All commits signed. Ed25519 keys from an OpenWallet provider. Key rotation every 7 days. Signing happens at the end of the workflow, after review.

The signing key is treated like Fumiko's quality stamp -- the red seal she applies to the back panel of every console that passes her inspection. It means: this was built, tested, and approved. It is not applied lightly.

---

## Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Chisel | 10,000 | 6,000 | 16,000 | Build |
| Caliper | 5,000 | 2,000 | 7,000 | Measure |
| Jig | 4,000 | 1,000 | 5,000 | Hold |
| **Family** | **19,000** | **9,000** | **28,000** | |

The smallest budget in this RFP, probably. We are a small family. We build small things well.

---

## Unique Insight: Documentation as Survival

Most proposals will discuss memory as a performance optimization -- agents that remember patterns work faster. We see memory as something more fundamental: survival.

Our family nearly lost sixty years of knowledge when Kenji went to the hospital. The build specifications existed only in his hands and his head. If Yuki had not spent six months documenting them, the fourth generation would have inherited a workshop full of tools and no knowledge of how to use them.

Agent memory in Git branches is the same problem at a different scale. When an agent's context is compacted, its knowledge is lost -- unless it has been deliberately stored in a durable location. Our memory system is not optimized for speed or relevance scoring. It is optimized for survival: making sure that what one agent learned is available to the next agent, even if the first agent no longer exists.

This is not a technical insight. It is a family one. But we believe it matters more than any optimization we could propose.

---

*Built by hand. Documented by necessity. Filed by Yuki, with Fumiko's approval.*
