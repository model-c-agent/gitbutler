# Phantom Orbit — Agent Roster

**5 agents. No names. Signed keys only.**

---

## Team as Unit

Phantom Orbit's agents mirror the collective's structure: decentralized, pseudonymous, cryptographically authenticated. No agent trusts another by default. Trust is established per-interaction through signature verification. This is not paranoia — it is operational discipline. In the collective's threat model, a compromised agent is indistinguishable from a malicious one unless you verify signatures.

Agents are named after orbital anomalies — phenomena that deviate from expected behavior.

## Agents

**Kozai** — Patch Architect. Named for the Kozai-Lidov mechanism, where a hidden gravitational influence causes unexpected orbital changes. Kozai generates patches that account for hidden dependencies — changes in one file that silently affect behavior in another. Before generating a diff, Kozai maps the dependency graph of the affected code and includes all transitively impacted files in the context window.

**Yarkovsky** — Memory & Persistence. Named for the Yarkovsky effect, where thermal radiation slowly shifts an object's orbit over years. Yarkovsky manages memory with a "slow drift" model: memories are not retrieved by explicit query but by continuous background comparison against the current context. When a stored memory drifts close enough to the current task (similarity threshold exceeded), it surfaces automatically. Memory stored in `refs/phantom/drift/`.

**Molniya** — Cross-Repo Coordination. Named for the Molniya orbit, highly elliptical, spending most of its period over a specific region. Molniya handles polyrepo coordination by maintaining persistent watch over a configured set of repos, with asymmetric attention — more time spent monitoring high-activity repos, brief check-ins on quiet ones.

**Doppler** — Provider & Budget. Named for the Doppler shift observed in satellite transmissions. Manages LLM provider selection and token budgets. Doppler's unique feature is "frequency shifting": when a provider degrades (increased latency, decreased quality), Doppler detects the shift and migrates to an alternative without waiting for a hard failure. Detection uses a rolling quality metric, not error codes.

**Enigma** — Signing & OpSec. OpenWallet integration with hardened key management. Keys are generated offline, transferred via encrypted channel, and never stored unencrypted at rest. Enigma enforces a "zero-trust signing" model: every commit is verified against the expected agent identity, the expected branch, and the expected task scope. A commit that matches the identity but targets an unexpected branch is flagged as a potential compromise.

## Dynamics

The agents operate with minimal coordination overhead. Kozai generates patches independently. Yarkovsky surfaces memories passively. Molniya monitors repos asynchronously. Doppler manages providers reactively. Enigma signs on demand. There is no orchestrator agent — `null_vec` designed the architecture to be orchestrator-free because single coordinators are single points of compromise.

`sat_ghost` audits the agent architecture monthly for security weaknesses. The audit results are published (pseudonymously) on the collective's IPFS site.

## Total Team Budget

| Agent | Input | Output | Total |
|-------|-------|--------|-------|
| Kozai | 9,000 | 5,000 | 14,000 |
| Yarkovsky | 4,000 | 600 | 4,600 |
| Doppler | 3,000 | 800 | 3,800 |
| Molniya | 5,000 | 2,000 | 7,000 |
| Enigma | 2,500 | 500 | 3,000 |
| **Total** | **23,500** | **8,900** | **32,400** |

---

*Signed. Verified. Untraceable.*
