<!--
  title: Docs — Benennung & Versionierung
  class: concept
  date: 2026-09-10
  sha256: 02eb4a527dd5ab81c86cf1eeeb53d036730aeab2d2e6eaf6e85693b9e7d98029
  status: live
  see-also: AGENTS.md
-->
## Docs — Benennung & Versionierung (docs/)

Classes (folder = purpose, prefix = kind, kebab-case, ASCII, no spaces/umlauts):

- `docs/handover/handover-YYYY-MM-DD-<slug>.md` — a handover (Übergabe) and a
  session plan are one kind of document: written by the closing session, read
  by the next session to continue, consumed into code/register/commits.
  Immutable. The date is the document's own date, never invented. Open points
  stay named in the handover; the session that works them off takes them up.
  Same-day handovers with similar slugs may coexist (in the archive or live) —
  the name carries the day and the line, and the next session reads the one it
  needs.
- `docs/surveys/survey-YYYY-MM-DD-<slug>.md` — a dated finding/snapshot;
  `survey-<slug>.md` — a standing survey (evolving, no date in the name).
- `docs/plans/ref-<slug>.md` — a standing reference list.
- `docs/auftrag/auftrag-<slug>.md` — an Untersuchungsauftrag (`class:
  auftrag`): the research order a gate (e.g. `livefeed_gate`) issues for a
  new research line. It is the *only* gate output that is versioned, and it is
  versioned under `docs/auftrag/`, never loose in `docs/` root — a loose
  `docs/auftrag-*.md` is drift. The repo-root `AUFTRAG.md` is the
  transient, unwritten-form order; the versioned `docs/auftrag/` copy is the
  canonical one. One order per file, dated by its own date.
- `docs/befund/` — Alt-Bestand only. Befunde are abolished (operator word,
  2026-09-10): an implementable finding is built, not filed; a real
  measurement result is a handover line, not a document. No new Befund
  (`class: befund` struck). The migration deleted all 111 files on
  2026-09-10 — every deleted file stays resolvable via git history.
- `docs/blatt/blatt-<slug>.md` — an Ein-Blatt sheet (`class: sheet`): a
  single-sheet causal-arrow pre-registration or screening verdict (the
  `blatt-papier` discipline). Sheets are not papers (a sheet is a
  pre-registration / one-sheet verdict, `class: sheet`; a paper is a
  self-contained publishable measurement, `class: paper`). A `blatt-*.md`
  loose in `docs/` root is drift — it belongs in `docs/blatt/`.
- `docs/concepts/<kebab>.md` — concept docs. The filename is kebab-case; the
  concept's proper name in prose stays UPPER_SNAKE (e.g. file
  `sources-v2-spec.md`, prose `SOURCES_V2_SPEC §1` — like `rfc-2616.md` ↔
  `RFC 2616`).
- `docs/paper/<kebab>.md` — publishable measurements (papers and Ein-Blatt
  verdicts): self-contained, one measured verdict per paper, `class: paper`.
- `docs/reference/` — external reference material only, in its native format (no header).

Versioning is git-only: no `vN`, `_ancestral`, or hash in the name — the
commit SHA addresses every state; a milestone is marked via `version:` in the
header. True historical snapshots that must coexist move to
`archive-root/`, never version-suffixed in place.

Every prose doc (handover/survey/ref/concept/paper/auftrag/blatt) opens with a header block; the
`sha256` covers the body **without** the header (`sed '/^<!--/,/^-->/d' <f> |
sha256sum`), so two local copies are compared in one command:

    <!--
      title: …
      class: handover | survey | ref | concept | paper | auftrag | sheet
      date: YYYY-MM-DD
      version: <n>          (milestone only)
      sha256: <hex>
      status: live | consumed | archived | done
      see-also: …
    -->

The receiving session consumes a handover into code/register/commits; when
the new handover stands, the session moves the handover it consumed into
`docs/handover/archiv/`. Closed documents rest in the flat archive folders
`docs/{handover,auftrag,blatt}/archiv/` (befund is struck — the Alt-Bestand
folder carries no archive of its own).
Raw consultation transcripts (arena/foreign-model chats) are archived to
`archive-root/arena/` — their distilled findings live in
the standing concept docs.
