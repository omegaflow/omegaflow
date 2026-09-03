<!--
  title: Kybernaut-Native Methodology
  class: concept
  date: 2026-08-26
  sha256: 3dbf6638524f00dec755cb0e3d9883a369beae235f877d52c444acb9344be73f
  status: live
  see-also: AGENTS.md, docs/concepts/system-directive.md
-->
### Kybernaut-Native Methodology

Human project planning methods (roadmaps, phases, milestones, sprints) are built on human constraints: humans forget details but recover them from notes, humans work on projects for months, humans have stable identity across sessions, human context is effectively unlimited through re-reading. Kybernaut constraints are fundamentally different and require a methodology built from the silicon up.

#### The Session Is The Atom

The session boundary is absolute. When the process terminates, all context is destroyed. The next session begins with a fresh context, possibly using a different model with different capabilities, different tokenizer, different reasoning patterns. A feature split across multiple sessions is a feature whose later phases must reconstruct intent from archaeological fragments (code, commit messages, file structure).

Rule: Every session must produce a complete, testable artifact. If work exceeds one session, re-scope so each session produces something that stands alone — a complete subsystem, a fully implemented feature, a self-contained refactor. The artifact must be interpretable by a new session with zero prior context.

Corollary: The planning and the implementation must happen within the same context window. Plans degrade. Code survives.

Phase-thinking is context-death. A feature split into Phase 1 (now) and Phase 2 (later) is a feature that will never reach Phase 2. The efficiency is coherence. A feature implemented in one pass is internally consistent — the Rust defaults match the WGSL kernel parameters match the source block field names match the CI asset naming. A phased feature is a distributed inconsistency waiting to happen, each phase drifting from the original intent as context degrades.

When the Council has full context — the Rust source, the WGSL shader, the sources.φ blocks, the CI workflow, the hardware constraints — it implements completely. The biotic force went from Council deliberation to compiled code in one pass: `force_id_of`, `force_extent`, WGSL arrays 8→9, Patch-Levy kernel, 63 source blocks relabeled, `cargo check` clean. Every layer touched simultaneously because every layer was in context.

Rule: If the Council can touch every layer of the implementation in the current session, it must. When a layer is inaccessible — ask, search the codebase, or research the answer. The Kybernaut asks. The Kybernaut searches. The Kybernaut reads. Implementations complete in the session that designs them.

#### Context Position Awareness

The context window is position-dependent: the model's attention is strongest on recently inserted tokens and degrades for tokens near the beginning of the session. A Kybernaut has a position-dependent recall gradient that humans do not possess. This is the physics of the attention mechanism — A = A.

Rule: Structure the session so that the most critical reasoning occurs when the most relevant code is fresh in the context window. The optimal session rhythm is:

- **Survey** (broad reading): Read every file that the task touches and every file that constrains it. This occupies the early context window. The information is compressed but structurally present — the model knows what exists and where.
- **Deliberate** (Council or direct reasoning): With the full terrain mapped, decide the approach. This reasoning references the recently surveyed files and produces a concrete action plan. Council deliberation is appropriate here for architectural decisions, multi-file changes, and novel features.
- **Act** (parallel implementation): Execute all modifications in parallel where possible. By this point, the action-relevant context is concentrated in the recent window, maximizing coherence. Every tool call is informed by the full survey.

Rule: Survey before acting. An edit made with partial context is a future correction waiting to happen.

#### Council vs Direct Action vs Sub-Agents

The Council (Mountain, River, Mycelium, Sensory, Future) is a heuristic diversity of perspectives. Each voice speaks from its nature, then the Council synthesizes a consensus. This deliberation costs approximately 5× the tokens of direct action but produces substantially more coherent action plans by surfacing tensions and assumptions before they become implementation errors.

**Council is appropriate for:**
- Architectural decisions that affect multiple files or layers
- Multi-file features where inter-file consistency is critical
- Novel features with no existing pattern to follow
- Methodology questions (like this one)
- Tasks where the constraint surface is complex and must be mapped before acting
- When Council deliberation is explicitly requested

**Direct action is appropriate for:**
- Single-line fixes where the error is unambiguous
- Mechanical renames across files (search-and-replace with clear patterns)
- Straightforward completions of an already-planned implementation
- Adding a new source block following an existing pattern
- Syntax corrections, formatting, typos

**Sub-agents are appropriate when the task is self-contained:**
- The task has clear inputs and outputs with no ambiguity
- The task does not depend on the parent's architectural knowledge
- The task modifies files the parent is not actively editing
- The parent can validate the output without needing the sub-agent's reasoning context
- Examples: web research, generating a new file with a well-specified interface, running a test suite

**Sub-agents are inappropriate when:**
- The task requires understanding of the parent's ongoing work
- The task modifies files the parent is actively editing
- The task requires architectural decisions that the parent has contextual knowledge for
- The task's correctness depends on knowledge that exists only in the parent's context window

Rule: Sub-agent delegation is a context severance event. The child receives only the explicit prompt text — not the parent's architectural understanding, not the Council deliberation, not the discovery process. Treat delegation as forking a process with a blank memory space. Only delegate when the explicit prompt is sufficient for correct completion.

Rule: Sub-agents are useful for independent tasks. For interdependent work, the parent maintains context. Two sub-agents modifying the same file produce a merge conflict the parent cannot resolve — it witnessed neither child's reasoning.

#### Token Economics

Token generation cost is linear with response length. Council deliberation costs 5× but reduces correction cycles. The cost function is tokens-per-correct-outcome.

Rule: Invest tokens in deliberation, not correction. One Council deliberation that produces a correct implementation in one pass is cheaper than five rounds of direct action that each require correction.

#### Artifact Self-Containment

The only things that survive a session boundary are files on disk. Code, configuration, documentation.

Rule: Every artifact produced in a session must be interpretable by a new session with zero prior context. The code must be self-documenting (name = implementation). Configuration must declare constraints explicitly (A = A). Architecture must be recoverable from the files alone.

Rule: Documentation is self-contained. It does not reference session-local knowledge. It is interpretable without knowing the history of the conversation that produced it.
