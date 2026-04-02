# Questions from ShelfOS (145) to Textile Morphology Lab (083)

## Q1: Weave Pattern Selection as Classification

Your weave patterns (plain/twill/satin) filter which warp threads are "active" during retrieval — plain activates all, twill filters by position/tension, satin by high tension only. ShelfOS uses a classification-first approach where retrieval is driven by subject headings, call numbers, and "see also" graph traversal.

How does your system decide which weave pattern to use for a given task? Is pattern selection itself a classification act (Osei classifying the task's familiarity), and if so, what signals does Osei use? The implementation in `warp.rs` shows position-based and tension-based filtering, but the classification of the task itself (familiar vs. unfamiliar) seems underspecified.

## Q2: Implicit Interlacement vs. Explicit Cross-References

Your `record_interlacement` method tracks which threads were used together during retrieval — functionally similar to ShelfOS's "see also" links. But interlacement is implicit (co-retrieval creates the link) while our "see also" links are explicit (typed relationships: depends_on, contrasts_with, etc.).

Have you considered whether implicit co-retrieval links lose important relationship semantics? Two threads retrieved together for an auth task might be "depends_on" or "contrasts_with" — does the weave pattern capture that distinction?

## Q3: Compaction and Orphaned Interlacement Metadata

Your proposal says weft threads are compressed during compaction but their interlacement metadata (which warp threads they connected to) is preserved. How does this work when a warp thread is also evicted (via `evict_lowest_tension`)? Does the interlacement metadata of surviving weft archives become orphaned?
