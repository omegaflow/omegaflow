<!--
  title: Handover — Ernte-Folge 63 (Stand 2026-09-17)
  session: Ernte-Folge 63
  class: handover
  date: 2026-09-17
  sha256: ae467c6ffe183287d6adfc5467667ef68b7aa012c240871439edb2da47431d78
  status: live
-->
# Handover — Ernte-Folge 63 (2026-09-17)

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

## MAVEN TNF — Force-Lauf entblockt; 8 Alt-Shards selektiv vom CDN entfernt (härtester undatiert)

- Der Force-Lauf `35205267703` (`3bc1455f`, `force=true`) hing seit 09:28Z `pending`: ein stale Duplikat `35188042598` (`8a38932d`, 06:01:18, exakt der schon erfolgreiche `35188036501`) hielt die workflow-level Concurrency-Gruppe `maven-tnf-cdn`. Duplikat gecancelt (diese Session) → Force-Lauf `pending → queued` 12:20:57Z. (Schritt: bei success die `sha256`-Zeilen + `maven_tnf`-Blöcke ans Ende von `phi/sources.φ`.)
- Löschung der 8 Alt-Shards gemessen: direkte URL 404 (`archive_search --verdict`); Release `pds-ppi.igpp.ucla.edu` `updated_at 2026-09-17T09:49:14Z` bei `created_at 2026-08-04` (kein Recreate), galileo/messenger/voyager-Assets unberührt → selektive `maven_tnf*`-Entfernung, konsistent mit der folge61-geplanten Orphan-Bereinigung (`gh release delete-asset`), ausgeführt im aktiven Session-Fenster (commits `735259dc`/`d0e4b52e`, 09:43–09:47Z). Kein CI-Schritt löscht Release-Assets (`sgrep delete-asset` = nur die Handover-Pläne); exakter Akteur nicht messbar (kein Audit-Log für ein User-Repo). (Schritt: entfällt — der neue DT0-Satz ersetzt die Alt-Shards.)

## Mariner 10 PSPA-00316 — Dispatch queued, Registrierung offen

- Lauf `35204897220` (`67f4e8c1`) `queued` seit 09:23Z (runner-gebunden, kein Duplikat gemessen). (Schritt: bei success Register-Block `mariner_occlt` in `phi/sources.φ` + `…_register_field_names_match_components`-Test.)
- Pending (eigene Atom-Linie): Sample-Rate/Record-Dauer (Tag-Inkremente 18…20.898, Records nicht äquidistant), Frac-Einheit (25-ms-Ticks vs. BCD-Zentisekunden), unabhängiger UTC-Anker (1974-02-05 aus `attrib`, unbestätigt).

## planetary-odf (rosetta/mro/juno_ocru) — Läufe offen, Register↔CDN-Lücken gemessen

- `mro_odf`-Registrierung steht (14 Shard-Blöcke). Läufe: `35189038542` (`87680961`) `in_progress`; `35190614514` (`1e4faf79`) `failure` (7/9 Legs `HTTP 403` Schreib-Token, `rosetta_odf` cancelled); `35218760142` (`f070ca36`) `queued` 12:01Z.
- Register↔CDN gemessen: `rosetta_odf.bin` steht in `phi/sources.φ:6374`, ist aber auf `archives.esac.esa.int` **absent** (Assets: hsa-json, M15TNF0L1A, `mars_express_odf.bin`) → Block braucht die `sha256` bei success; `juno_ocru_odf.bin` weder registriert noch auf `atmos.nmsu.edu` (Assets: `cassini_tnf.bin`, `juno_odf.bin`, `vex_odf.bin`). (Schritt: bei success `sha256`/`rosetta_odf` + `juno_ocru_odf.bin`-Block in `phi/sources.φ`; Schreib-Token-Budget → `An entscheid`-Zeile in `post.md`, committet `68f576bb`.)

## DEMETER — Harvest 0 Dateien (Quellen-WAF)

- Harvest `35146819646` success = Budget-Stopp: Log `0 files on disk, 0 orders done`; jede Order `WAF blocked … waiting 1800s` / `order parse void`. (Schritt: neue Orders = Dritt-Akt → Consent über entscheid; `An entscheid:`-Zeile in `post.md`, committet `68f576bb`.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Benchmark — Routine-Verifikation

- Die Klasse „Routine-Verifikation" ist geschlossen (flash gewinnt 2,4–11×, 2026-09-16); die TNF-Blocker-Diagnose lief direkt, kein Doppel-Lauf gegen pro/max.

## Geteilter Baum — eigener Pfad-Satz

- Fremde uncommittete Arbeit (nicht im eigenen Commit-Pfad, gemessen 2026-09-17): `docs/zustand/external-state.md` (` M`, trägt den veralteten CI-Status bei `bc9d6a0b`) und die gestagten Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`. (Schritt: eigener Commit-Pfad = dieses Handover + das archivierte folge62; fremde Pfade nicht anfassen, nie ein nacktes `git commit`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
