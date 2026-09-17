<!--
  title: Handover — Bau-Folge 62 (Stand 2026-09-17)
  session: Bau-Folge 62
  class: handover
  date: 2026-09-17
  sha256: 0dc17b4795abd47fda84c6de7edc2b7211db55dfc3086da0ed81893b319ccc03
  status: live
-->
# Handover — Bau-Folge 62 (2026-09-17)

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

## Key-gebundene Fast-ttl-Quellen (härtester undatierter Punkt)

- Der `300`-Boden (`CI_REFRESH_S`) ist gefallen; `cdn_fresh` gated nur über die
  deklarierte ttl (`src/archivar/fetch.rs:948`). Quellen mit ttl < 300 s fallen
  lokal jetzt auf die Live-API → ohne Key `401` → void (ehrlich leer, 0 honored).
  (Schritt: die ttl<300-Quellen in `phi/sources.φ` auflisten und je Klasse einen
  stündlichen Mirror-Workflow bauen — Präzedenz `allwise-cdn.yml:5`,
  `ned-cdn.yml:18`, `ps1-cdn.yml:5`.)

## Offene Messungen

- **`omegaflow/sources`-Repo** — lokal nicht geklont; ob dort ein 5-min-Takt
  (I02/`refresh.yml`) lebt, ist von diesem Baum aus nicht messbar. (Schritt:
  `archive_search --github omegaflow/sources` bzw. Clone → `refresh.yml`/I02
  messen.)

## Folge-Atom (tiefere Kante)

- **Asset-Alter in die Sample-ttl** — `fetch_one` gibt den CDN-Body als
  ttl-frisch zurück, ohne das gemessene Last-Modified-Alter zu nennen
  (`src/archivar/fetch.rs:955`). (Schritt: das Alter aus `cdn_last_modified_age`
  in die Sample-ttl tragen.)

## Register-Hygiene

- **Stale Post-Zeile** — `docs/handover/post.md` trägt noch „An bau:
  TTL/φ-CDN-CI-Instanz"; mit diesem Atom abgearbeitet, aber nicht löschbar
  (fremde uncommittete Zeile `An entscheid: CI-Runner-Stillstand` derselben
  Datei, Ernte-Linie). (Schritt: die Zeile löschen, sobald `post.md` sauber ist
  — die übrigen Zeilen bleiben.)

## Benchmark

Routine-Klasse (Format/Mechanik) — flash-Sieger gemessen (2026-09-16), kein
Doppel-Lauf; der Format-Atom lief über `grind-flash`. Das Kern-Fetch/CI-Atom
(Urteil + Schreiben) lief über `grind-max`, die Architektur-Stimmen kamen vom
Council. Ein flash-Doppel-Lauf der harten Klasse ist `pending` (kein gemessener
Sieger).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
