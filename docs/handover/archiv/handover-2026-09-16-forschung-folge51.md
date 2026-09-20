<!--
  title: Handover — Forschung-Folge 51 (2026-09-16)
  session: Forschung-Folge 51
  class: handover
  date: 2026-09-16
  sha256: 3305a8b0e5e51b9e03ee6fb1f39ebdd0f3100bcdc39c59ea185ab18d2931a5e1
  status: live
-->
# Handover — Forschung-Folge 51 (2026-09-16)

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
der härteste undatiert); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## ODF-Bande-Split — Konsument (undatiert, mechanisch — CI-blockiert)

- Der im Vorgänger-Handover genannte Lauf `35139594201` ist **cancelled** (als
  wartender Lauf durch einen neueren Dispatch ersetzt, gemessen 2026-09-16). Der
  Lauf, der die Shard-Compiler trägt, ist `35148936648` @ `83fa92ec` (HEAD):
  `mro_odf` + `odyssey_odf` laufen **in_progress** (Shard-Zweig), `juno`/
  `messenger`/`mgs` übersprungen (Assets vorhanden). Der frühere Fehllauf
  `35115390613` @ `94e407fc` scheiterte, weil er den Shard-Writer `f2380e19`
  **nicht** enthält — odyssey lud die volle 3 367 454 840 B → HTTP 422 `size must
  be less than 2147483648`, mro exit 143. Der Baum-Vertrag ist vollständig
  (Writer `odf.rs:PODF_SHARD_*`, Compiler `mro_odf_compiler.rs:142`,
  `odyssey_odf_compiler.rs:209`, Konsument `extract.rs:42–52` + `main_flow.rs:2091`,
  Überlappungs-Gate `parse.rs:refuse_shard_overlaps`). (Schritt: nach
  Run-Abschluss `gh run view 35148936648 --log` — den gedruckten φ-Block nehmen
  und die zwei Ganzdatei-Blöcke in `phi/sources.φ` ersetzen: `mro_odf` heute
  `:6740–6744`, `odyssey_odf` heute `:6746–6750`, je ein 5-Zeilen-Block je Shard;
  `refuse_shard_overlaps` verweigert Überlappung gleichen `format`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. Gemessen 2026-09-16: `docs/paper/flyby-path-2-preregistration.md` nennt
  nur das *Was* (Felder plasma-pressure gradient, IMF-Bz, Kp, Swarm, RTSW@L1 mit
  L1-Transitzeit) — kein ausführbares Schritt-Detail. (Schritt: vor dem 28.09. den
  konkreten Abruf-Schritt je Kanal in `docs/paper/flyby-path-2-preregistration.md`
  setzen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
