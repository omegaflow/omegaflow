<!--
  title: Handover — Bau-Folge 61 (Stand 2026-09-17)
  session: Bau-Folge 61
  class: handover
  date: 2026-09-17
  sha256: bd0486a0079ada243ca819ac25fc72a18bfdd030dff5b679d9d3118403b67172
  status: live
-->
# Handover — Bau-Folge 61 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## TRK-2-34 — Decoder gebaut; reale Serie ungeprüft

- Alle 18 TNF-Format-Codes (0–17) sind in `src/archivar/odf.rs` dekodiert
  (`tnf_row`-Dispatch, benannte `TNF_FORMAT_*`-Konstanten, `tnf_rows` ohne
  stillen Drop, Fallback `_ => None`); die DT0-only-Schranke wurde Gate-Fixture
  (`tnf_format_gate`) + Gate-Test (`fp_tool_tnf_format_gate_blocked`) im selben
  Atom. Gemessen sind die SIS-Layout-Tabelle (dsn_trk-2-34.2021-06-03) +
  synthetische Fixtures — gegen die reale Serie noch nicht. (Schritt:
  tnf-Compiler-CI-Lauf auf den drei registrierten TNF-`url`-Zeilen
  (MAVEN/DART/Cassini), Row-Counts gegen die erwarteten 1167/401 halten.)

## las CDN — Dispatch läuft; Lauf lesen

- Der Lauf `35185399788` (`gh workflow run las-cdn.yml`, nach der Fixture-
  Korrektur) ist `in_progress`; der Vorlauf `35156273692` war success.
  (Schritt: `gh run view 35185399788` — success: Punkt löschen; failure:
  `gh run view 35185399788 --log-failed`.)

## Pioneer-10 ATDF dtype — descoped (gemessen)

- Die Prämisse „two-way fällt heraus" ist widerlegt: das binäre ATDF-Feld
  `DATA_TYPE` ist 4-bit (TKFORM item 12, @181–184), two-way = `2` (nicht 12);
  der `1|2`-Filter hält two-way bereits. Die Codes 12/13 leben nur in der
  NAVIO-abgeleiteten ASCII-Datei, die `pioneer_doppler_compiler` filterlos
  erfasst. Benannte Konstanten (`DTYPE_*`) in `src/archivar/atdf.rs`. Kein
  offener Schritt — die gemessene Freigabe ist der Eintrag.

## Post an bau — abgearbeitet, Zeilen stehen (fremd-uncommittet)

- Die drei „An bau"-Zeilen in `docs/handover/post.md` (TRK-2-34, Pioneer,
  Ledger-Drift) sind in diesem Atom abgearbeitet: TRK-2-34 gebaut, Pioneer
  gemessen descoped, Ledger-Drift als Gate-Fixture `unstable_pointer` + Tests
  `fp_tool_unstable_pointer_blocked`/`_only_in_docs_markdown` gebaut. Die Zeilen
  stehen noch, weil `post.md` fremd-uncommittete Hunks trägt und nicht
  überschrieben wurde. (Schritt: beim nächsten Pass löschen, sobald `post.md`
  committet ist.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
