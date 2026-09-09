<!--
  title: Handover — TODO-Tod: die Register aufgelöst, das Handover ist das Register
  class: handover
  date: 2026-09-09
  sha256: 99078a293b69fdb2b19fe58ec8f0ef66d930b54b73de69be708a52eee19551e9
  status: live
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/handover/handover-thematisch-te-atom-4.md docs/handover/handover-thematisch-membran-sonde.md docs/handover/handover-thematisch-mechanische-reste.md docs/concepts/docs-naming.md
-->
# Handover — TODO-Tod: die Register aufgelöst, das Handover ist das Register

Übergabe für die nächste Sitzung. Diese Sitzung hat die Doppelbuchführung
aufgelöst: das 3492-Zeilen-Register `docs/TODO.md` ist gestorben, die Klasse
`docs/auftrag/` (53 Dateien) ist aufgelöst, die 44 datierten Handover sind
gemessen und archiviert. Was offen blieb, tragen vier thematische Handover —
das Handover ist das Register, Git ist die Historie.

## Vollzogen

- **TODO gestorben** — `docs/TODO.md` gelöscht. Erledigtes trägt Git; offene
  Punkte sind in die thematischen Handover gewandert; nichts ist still
  gefallen.
- **Aufträge aufgelöst** — die 13 geschlossenen Aufträge und die 40 offenen
  ruhen flach in `docs/auftrag/archiv/`; die offenen Pflichten tragen die
  thematischen Handover.
- **44 Handover archiviert** — gemessen (bearbeitet/unbearbeitet) und flach
  nach `docs/handover/archiv/` verschoben (status: archived). Die unbearbeitete
  offene Arbeit ist in die vier thematischen Handover eingegangen.
- **Vier thematische Handover stehen** als lebendes Register:
  `handover-thematisch-tiefenphasen-flotte.md`, `-te-atom-4.md`,
  `-membran-sonde.md`, `-mechanische-reste.md` (`class: handover`,
  `status: live`, kein Datum im Namen).
- **Maschinen auf das Handover-Register umgehängt** — `commit_gate::learn_register`
  liest `docs/handover`, `docs/befund`, `docs/blatt` statt TODO; das
  Drift-Handover-Template lehrt das thematische Handover; `friction.rs`
  (tote TODO-/status-Präfixe entfernt); `register_verify`-Default = die vier
  thematischen Handover; `bz_blatt_probe`-Diagnose benennt das thematische
  Handover; `claim_verify` stillgelegt (kein Code-/Workflow-Aufruf, gemessen).
- **Regel steht** — AGENTS.md: „A commit is a checkmark. The handover is the
  register; git is the history. … moves the handover it consumed into
  docs/handover/archiv/ …"; die flachen Archiv-Ordner
  `docs/{handover,auftrag,befund,blatt}/archiv/`; das Session-Protokoll.
  `docs-naming.md` trägt jetzt das stehende thematische Handover; die
  no-archive-move-Zeile ist umgeschrieben. `SOURCE_PORT.md` + `NOTICE` folgen.

## Der parallele Faden

Der Baum trug fremde Uncommitted-Arbeit (Tiefenphasen-Polarität, TE-Betriebs-
punkt-Sweep) — auf Operator-Wort in diese Sitzung eingefaltet und committet.

## vo-tap / uvor

`ivoa/uvor` existiert (HTTP 200, gemessen). Der Seed `tools/vo-tap` (BSD-3-Clause,
Copyright Johannes Tyroller) steht; der Push ist ein Operator-Wort, kein Code.

## Offen für die Folge-Session

Die vier thematischen Handover lesen — sie tragen das Offene vollständig.
