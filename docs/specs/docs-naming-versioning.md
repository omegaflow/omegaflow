<!--
  title: Docs — Naming & Versioning
  class: concept
  date: 2026-08-26
  sha256: 231c6e4b707897d0a59eec994296d1e43342e93a19113fda74c099da69dff982
  status: live
  see-also: AGENTS.md
-->
## Docs — Naming & Versioning (docs/)

Classes (folder = purpose, prefix = kind, kebab-case, ASCII, no spaces/umlauts):

- `docs/handover/handover-YYYY-MM-DD-<slug>.md` — a handover (Übergabe) and a
  session plan are one kind of document: written by the closing session, read
  by exactly one receiving session, consumed into code/register/commits.
  Immutable. The date is the document's own date, never invented.
- `docs/surveys/survey-YYYY-MM-DD-<slug>.md` — a dated finding/snapshot;
  `survey-<slug>.md` — a standing survey (evolving, no date in the name).
- `docs/plans/ref-<slug>.md` — a standing reference list.
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

Every prose doc (handover/survey/ref/concept/paper) opens with a header block; the
`sha256` covers the body **without** the header (`sed '/^<!--/,/^-->/d' <f> |
sha256sum`), so two local copies are compared in one command:

    <!--
      title: …
      class: handover | survey | ref | concept | paper | auftrag | sheet
      date: YYYY-MM-DD
      version: <n>          (milestone only)
      sha256: <hex>
      status: live | consumed | archived
      see-also: …
    -->

The receiving session archives a consumed handover to
`archive-root/handover/` — **only after its own work is
committed**, never before: git is the safety net against crashes and rogue
sessions. The archive commit (`cp` + `git rm`) is the checkmark that the
handover was read and understood. A consumed-but-unarchived handover is a
register debt; an archived-but-uncommitted one is a violation. Raw
consultation transcripts (arena/foreign-model chats) are archived to
`archive-root/arena/` — their distilled findings live in
the standing concept docs.
