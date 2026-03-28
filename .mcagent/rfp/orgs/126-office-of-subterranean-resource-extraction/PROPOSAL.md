# PROPOSAL.md — Office of Subterranean Resource Extraction Compliance

**Filing Reference:** OSREC-RFP-2026-BUTAI-003
**Classification:** PUBLIC
**Stamped:** Director Mukherjee, 2026-03-28

---

## Section 1: Executive Summary (ref: OSREC Proposal Standard 1.1)

The Office of Subterranean Resource Extraction Compliance proposes to implement the `but-ai` plugin as a permit review system for autonomous code changes. Every agent action is a permit application. Every approval is a stamp. Every memory entry is a filed geological survey. The Office's 47 years of experience reviewing complex, multi-stakeholder extraction proposals translates directly to the review of complex, multi-file code changes.

---

## Section 2: Technical Proposal

### 2.1 Requirement 1: PATH-Based Plugin Architecture

The `but-ai` binary shall be installed to a location discoverable via PATH, following the Office's Executable Placement Standard (analogous to OSREC Form OSREC-EXEC-1). The plugin registers with the `but` CLI through a manifest file specifying supported subcommands. The Office proposes a manifest format modeled on permit application cover sheets: a structured declaration of capabilities, version, and required permissions.

Provider selection occurs at invocation time via `--provider` flag or `BUT_AI_PROVIDER` environment variable. The Office insists on explicit provider declaration — implicit defaults are "unsigned permits" and the Office does not process unsigned permits.

### 2.2 Requirement 2: Provider-Agnostic AI

The Office proposes a provider abstraction layer modeled on its own multi-jurisdiction compliance framework. Just as the Office reviews permits under federal, state, and international standards simultaneously, the provider layer normalizes tool-calling semantics across OpenAI, Anthropic, Ollama, and LMStudio behind a unified interface.

Each provider adapter implements a Compliance Interface: `submit_prompt()`, `receive_completion()`, `verify_tool_call()`, and `report_token_usage()`. The adapter handles provider-specific serialization. The Office does not care which jurisdiction (provider) the response originates from, only that it meets compliance standards.

### 2.3 Requirement 3: But Agent (INDEX.patch + COMMIT.msg)

The Office treats each agent task as a permit application lifecycle:

1. **Filing** — Clerk Pratt registers the task, assigns a filing number, retrieves relevant memory
2. **Survey** — Deputy Director Okafor reads the codebase (seismic survey equivalent)
3. **Extraction Plan** — Okafor produces INDEX.patch (the extraction plan) and COMMIT.msg (the environmental impact statement)
4. **Compliance Audit** — Inspector Zhao audits the patch against the Office's compliance checklist
5. **Stamp** — Director Mukherjee approves or rejects with findings

The INDEX.patch is a unified diff. The COMMIT.msg includes: the filing number, a summary of changes, regulatory cross-references, and a geological classification of the change type (SEDIMENTARY: routine addition, IGNEOUS: new feature creation, METAMORPHIC: refactoring of existing structure, EROSION: deletion).

### 2.4 Requirement 4: Polyrepo PR Coordination

The Office has extensive experience coordinating permits across multiple jurisdictions. The proposed cross-repo coordination uses PR comments as inter-office memoranda, formatted with filing numbers and cross-references. A dependency between repositories is modeled as a "joint permit application" — changes in repo A that require changes in repo B are filed together with Form OSREC-JOINT-7.

Forge-agnostic coordination is achieved through the same adapter pattern used for providers. GitHub, GitLab, and Gitea implement a Forge Compliance Interface: `file_memorandum()`, `retrieve_memoranda()`, `stamp_approval()`.

### 2.5 Requirement 5: Agent Memory in Git Branches

Memory entries are stored in `refs/osrec/memory/<filing-number>` as Git blobs. Each entry is a filed document with:

- **Filing number:** `OSREC-MEM-YYYY-NNNNN`
- **Geological classification:** SEDIMENTARY / IGNEOUS / METAMORPHIC / FOSSIL
- **Weight:** Size in bytes (the Office insists on calling this "weight")
- **Cross-references:** Links to related filings
- **Retention schedule:** TTL based on classification (FOSSIL entries are permanent; SEDIMENTARY entries expire after 30 days)

Clerk Pratt maintains the filing index — a JSON tree object at `refs/osrec/memory/INDEX` that maps filing numbers to blob SHAs. Retrieval is by filing number (exact), by classification (filtered), or by cross-reference traversal (graph walk).

The Office's unique insight: **memory entries should be auditable in the same way permit applications are auditable.** Every access to a memory entry is logged. Every modification creates a new filing (the original is never altered — it is supplemented). The audit trail of memory access is itself a memory, filed under classification ADMINISTRATIVE.

### 2.6 Requirement 6: Signed Commits via OpenWallet

The Office considers unsigned commits to be "unstamped permits" — legally void. All commits are signed via OpenWallet DID-based keys. Director Mukherjee is the sole signing authority. The signing workflow:

1. Okafor produces the patch
2. Zhao audits and certifies
3. Mukherjee reviews Zhao's certification
4. Mukherjee signs via OpenWallet key bound to her DID
5. The signed commit includes the filing number in the trailer

Key rotation follows the Office's existing credential rotation schedule (quarterly, with emergency rotation within 4 hours of suspected compromise).

---

## Section 3: Token Budget

| Agent | Input | Output | Total | Role |
|-------|-------|--------|-------|------|
| Director Mukherjee | 7,500 | 2,800 | 10,300 | Approval, signing |
| Deputy Director Okafor | 9,200 | 6,500 | 15,700 | Patch generation |
| Clerk Pratt | 5,000 | 2,000 | 7,000 | Memory, filing |
| Inspector Zhao | 6,800 | 3,200 | 10,000 | Compliance audit |
| **Team Total** | **28,500** | **14,500** | **43,000** | |

Overhead (inter-agent memoranda): ~4,000 tokens per task.
**Total per task: ~47,000 tokens.**

---

## Section 4: Unique Insight

**The permit-as-patch isomorphism.** The Office has discovered that mining permit review and code patch review are structurally identical problems. Both involve: (a) a proposal to modify a complex, layered system; (b) a review process that must verify the proposal will not cause collapse; (c) a permanent record of the decision; (d) a signature attesting to the reviewer's judgment. The Office has been solving this problem for 47 years. The only difference is that code collapses produce stack traces, not sinkholes. The review process is the same.

---

*CERTIFICATION: This proposal prepared per OSREC Proposal Standard 1.1. Filing reference: OSREC-RFP-2026-BUTAI-003. Weight: 0.006 kg (digital). Stamped: Director Mukherjee, 2026-03-28.*
