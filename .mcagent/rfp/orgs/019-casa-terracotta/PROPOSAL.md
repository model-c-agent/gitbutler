# Casa Terracotta — Proposal

**Response to: `but ai` Plugin RFP v1.0.0**

---

## 1. Plugin Architecture (RFP 3.1)

Rust crate at `crates/but-ai/`. Small and well-joined, like a good repair. Three modules: `workshop` (agent execution), `recipes` (memory and pattern management), and `liaison` (MCP and forge).

CLI subcommands: `agent repair` (execute a task — we call it "repairing" because we bring the codebase toward its intended state), `recipes show` (display relevant patterns for current context), `recipes store` (save a new pattern), `mcp`.

The `recipes show` subcommand is our distinctive feature: it does not execute a task but shows what the agent *would* reference if it did. Useful for understanding the agent's knowledge base before committing to a task.

MCP mode: backward-compatible drop-in. All `WorkspaceToolset` tools plus `RepairTask`, `ShowRecipes`, `StoreRecipe`.

**WASI:** Under WASI, the `recipes` subcommands work (memory queries are local). Agent execution is unavailable. The workshop does not produce work in constrained environments — "you cannot repair pottery in a moving truck."

## 2. Provider-Agnostic AI Interface (RFP 3.2)

We use `but-llm` as-is. Provider selection is based on the master's assessment of task difficulty:

- Simple tasks (formatting, documentation): `response` method, any provider
- Standard tasks (single-file patches): `tool_calling_loop`, capable provider
- Complex tasks (multi-file, cross-module): `tool_calling_loop_stream`, frontier provider

The materials specialist maintains a "compatibility chart" for providers, analogous to the workshop's ceramic-adhesive compatibility charts. The chart records which providers perform well on which task types, updated after each task.

New providers: add config entry, run the compatibility tests, add to the chart. No runtime plugins.

## 3. The But Agent (RFP 3.3)

The workshop repair cycle:

1. **Assessment** — Master studies the task and the surrounding codebase. Produces a specification that describes not just what to change but how the change should *feel* — matching the project's idioms, conventions, and style.
2. **Preparation** — Materials specialist retrieves relevant recipes (patterns, conventions, past solutions).
3. **Repair** — Apprentice produces `INDEX.patch` + `COMMIT.msg`, following the specification and using the recipes.
4. **Inspection** — Master reviews the repair. Checks: Does the patch match the codebase style? Is the change invisible (does it look like it was always there)? Is the commit message clear?
5. **If approved:** Master signs. If rejected: Apprentice revises (max 2 revisions).

**Branch naming:** `ct/<project>/<deps>`. Example: `ct/P042/s01`.

**Budget enforcement:** Assessment 20%, preparation 10%, repair 45%, inspection 15%, reserve 10%. The reserve is larger than most proposals because the workshop expects revision cycles and allocates budget for them.

## 4. Polyrepo PR Coordination (RFP 3.4)

**Forge adapter:**

```rust
trait Gallery {
    fn exhibit(&self, repo: &str, piece: PrSpec) -> Result<ExhibitId>;
    fn annotate(&self, id: ExhibitId, note: &str) -> Result<()>;
    fn read_notes(&self, id: ExhibitId) -> Result<Vec<String>>;
    fn link_pieces(&self, from: ExhibitId, to: ExhibitId) -> Result<()>;
}
```

The liaison handles all forge operations. The workshop does not deal with the outside world directly — that is what liaisons are for.

**PR comment schema:** Workshop report format, warm and professional:

```markdown
**Casa Terracotta — Repair Report**

Project P042 complete. The apprentice produced a patch of 128 lines across 2 files. The master approved after one revision. The repair matches the existing code style.

<!-- ct:data {"v":1,"agent":"apprentice","revisions":1,"master_approved":true,"tokens":{"used":16800,"budget":21100}} -->
```

## 5. Agent Memory and Identity (RFP 3.5)

**Storage:** Branch `ct/recipes`. This is the workshop's recipe book — the most important artifact in the system.

Structure:
- `patterns/` — Code patterns and conventions extracted from the project
- `adhesives/` — Solutions to recurring problems (how this project handles errors, tests, configs)
- `repairs/` — Past task outcomes and what worked
- `identity/` — Agent records

**Relevance scoring:** The materials specialist uses a "compatibility" model. Each recipe has a list of "compatible materials" (file types, module domains, pattern categories). When a new task arrives, the specialist matches the task's context against recipe compatibility lists. Recipes with >50% material overlap are retrieved. Maximum 5 recipes per retrieval.

**TTL:** Patterns: 60 days (project conventions are stable). Adhesives: 30 days (solutions may become outdated). Repairs: 14 days (specific task context fades). Identity: indefinite.

**Compaction survival:** Each agent has a "workshop card" — a summary of its role and the three most important recipes it uses regularly. Cards are always preserved.

**Identity:** Agent identities stored at `ct/recipes/identity/<agent>.json`. Contains: name, role, specialization, branch permissions, public key, and `trained_by` field (which agent supervised this agent's creation).

## 6. Signed Commits via OpenWallet (RFP 3.6)

The master signs. Every commit bears the master's seal, just as every repair in the physical workshop bears the Ferrau family mark.

**Authorization:**

```toml
[agents.master]
can_sign = true
branches = ["ct/*", "feat/*", "fix/*"]

[agents.apprentice]
can_produce_patch = true
branches = ["ct/*"]
max_lines = 500

[agents.materials]
branches = ["ct/recipes/*"]
operations = ["read", "write-memory"]
```

**Key lifecycle:** The master's key is provisioned by the materials specialist (who manages the technical side of the workshop). Rotation every 45 days. Compromise: the master revokes the key, the materials specialist audits all signed commits, and the liaison notifies any dependent repositories.

## 7. Token Budget

| Component | Input Tokens | Output Tokens | Frequency | Notes |
|-----------|-------------|--------------|-----------|-------|
| System prompt | 2,800 | 0 | Once/session | Workshop roles + recipes format |
| Assessment | 3,000 | 800 | Once/task | Master studies task and codebase |
| Recipe retrieval | 600 | 200 | 2/task | Compatibility matching |
| Repair (initial) | 3,000 | 4,000 | Once/task | Apprentice first attempt |
| Master inspection | 3,000 | 300 | Once/task | Style and correctness check |
| Revision (if needed) | 1,500 | 2,000 | 0-1/task | Apprentice second attempt |
| Commit message | 600 | 300 | Once/task | Workshop style |
| Coordination event | 1,000 | 400 | 0-1/task | Liaison handles forge |
| **TOTAL (typical task)** | **15,500** | **8,000** | -- | Including one revision |

## Unique Insight

In ceramic repair, the most important skill is not applying the adhesive. It is mixing it. The proportions must account for the ceramic's age, porosity, the break pattern, and the ambient humidity. Get the mix wrong and the repair fails — not immediately, but in six months, when the adhesive shrinks or expands at a different rate than the ceramic.

The same principle applies to agent context preparation. The most important phase is not patch generation — it is recipe retrieval and context assembly. If the agent's context includes the wrong patterns, the wrong conventions, the wrong examples, the patch will be technically correct but stylistically wrong — and it will "fail" in review, not because the code is broken but because it does not belong.

We invest more in preparation than in execution. The mix matters more than the application.

---

*"Sedici generazioni. Sempre la stessa cura."*
*(Sixteen generations. Always the same care.)*
