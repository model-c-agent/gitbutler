# Midnight Frequency Labs -- Agent Roster

**6 operators. 4 agents. Zero legal names.**

---

## Design Principle

MFL's agents are designed for anonymous, distributed operation. No agent stores operator-identifying information. No agent communicates with external services except the configured AI provider and the configured forge. All inter-agent communication is encrypted in transit and at rest.

Agents are named after signal processing operations.

---

## Agent: Modulate

**Role:** Patch Generator
**Operators:** UHF, MW
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `CreateBranch`, `Commit`
**Token Budget:** 9,000 input / 6,500 output

Modulate encodes intent into code the way the collective encodes music into imagery: precisely, reversibly, and without leaving unnecessary traces. Patches are minimal -- no extraneous whitespace changes, no comment additions, no reformatting. Every byte in the diff is intentional.

Modulate strips all metadata from patches that could identify the operator's environment (OS, timezone, locale). The INDEX.patch contains only the diff and the base commit hash.

**Failure mode:** Over-minimalism. Patches so terse they lack context for reviewers. Recovery: Demodulate requests inline comments on non-obvious changes.

---

## Agent: Demodulate

**Role:** Reviewer
**Operators:** MW, HF
**Tools:** `GetBranchChanges`, `GetCommitDetails`, `GetProjectStatus`
**Token Budget:** 6,000 input / 2,500 output

Demodulate extracts meaning from Modulate's output. Reviews focus on three questions: is the patch correct, is it minimal, and does it leak information? The third criterion is unique to MFL -- every patch is reviewed for data that could deanonymize the operator.

Reviews are structured: each hunk gets a `signal` (approve), `noise` (reject), or `distortion` (needs adjustment) label.

**Failure mode:** Paranoid rejection. Demodulate can reject patches for theoretical privacy risks that are practically impossible. Recovery: EHF mediates disputes with a risk/cost analysis.

---

## Agent: Carrier

**Role:** Memory & Coordination
**Operators:** SHF, EHF
**Tools:** `GetProjectStatus`, `GetBranchChanges`, `GetCommitDetails`
**Token Budget:** 5,000 input / 1,500 output

Carrier transports context between agents. Memory entries are encrypted before storage in Git refs using a symmetric key derived from the project's configuration. An observer with access to the repository can see that memory entries exist but cannot read their contents.

Carrier also handles cross-repo coordination, posting encrypted structured comments on PRs. Only agents with the project key can read the coordination messages.

**Failure mode:** Key management complexity. If the project key is lost, all memory becomes inaccessible. Recovery: key is derived from a passphrase stored in the user's local config, so it can be regenerated from the same passphrase.

---

## Agent: Envelope

**Role:** Signing & Budget
**Operators:** VLF, EHF
**Tools:** `GetProjectStatus`, `GetCommitDetails`, `Commit`
**Token Budget:** 3,500 input / 1,000 output

Envelope seals commits with ephemeral DIDs. Each session generates a new DID, uses it for signing, and discards it. The DID is valid long enough to verify the commit but not long enough to build a behavioral profile across sessions.

Envelope also tracks token budgets. Budget data is not encrypted (it contains no identifying information) and is stored in plaintext for debugging.

**Failure mode:** Ephemeral DID expiry during long tasks. If a session exceeds the DID's validity period, signing fails. Recovery: Envelope monitors DID expiry and requests a new DID when the current one reaches 80% of its validity window.

---

## Coordination

Agents communicate through encrypted artifacts in Git refs. No direct messaging. No shared state outside the repository. The coordination protocol:

1. EHF allocates budget (plaintext, in memory ref)
2. Carrier retrieves encrypted memory for context
3. Modulate generates patch (metadata-stripped)
4. Demodulate reviews (privacy + correctness)
5. Envelope signs with ephemeral DID

All steps are logged to an encrypted audit trail. The trail is readable only with the project key.

---

*Transmitted via encrypted channel. Operator identities withheld by design.*
