<!--
  title: Handover — Ernte-Folge 62 (Stand 2026-09-17)
  session: Ernte-Folge 62
  class: handover
  date: 2026-09-17
  sha256: 44b0cf1665828b8987508e547129ec82621dad69999b014bad39dec81d6b529e
  status: live
-->
# Handover — Ernte-Folge 62 (2026-09-17)

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

## MAVEN TNF — Force-Lauf steht; 8 Shards am 09:49Z vom CDN verschwunden (härtester undatiert)

- Fix `c39deab8` (TNF-Zeit am CHDO-Typ-Offset, DT16/DT17 bei 44) ist in HEAD; Force-Lauf `35205267703` (headSha `3bc1455f`, `force=true`) `pending`. Der Lauf `35188036501` hatte um 06:38Z 8 Shards `maven_tnf_t*.bin` hochgeladen (`upload_release` gibt nur bei `gh release upload`-Erfolg true, Log gemessen), aber das Release `pds-ppi.igpp.ucla.edu` trägt sie nicht mehr: `updated_at 2026-09-17T09:49:14Z`, kein `maven_tnf`-Asset, kein Lösch-Trace im Baum (`sgrep 'delete-asset' .` findet nur die geplante folge61-Zeile). (Schritt: nach success des Force-Laufs die `sha256`-Zeilen + `maven_tnf`-Blöcke ans Ende von `phi/sources.φ`; die Löschung am 09:49Z zuerst klären — nächste Messung `gh api repos/omegaflow/sources/releases/tags/pds-ppi.igpp.ucla.edu --jq '.updated_at, [.assets[]|{name,created_at}]'`.)

## Mariner 10 PSPA-00316 — Dispatch steht (queued), Registrierung offen

- Lauf `35204897220` (headSha `67f4e8c1`) `queued`; Compiler + Parser + Workflow gebaut. (Schritt: bei success Register-Block `mariner_occlt` in `phi/sources.φ` + `…_register_field_names_match_components`-Test.)
- Pending (eigene Atom-Linie): Sample-Rate/Record-Dauer (Tag-Inkremente 18…20.898, Records nicht äquidistant), Frac-Einheit (25-ms-Ticks vs. BCD-Zentisekunden), unabhängiger UTC-Anker (1974-02-05 aus `attrib`, unbestätigt).

## planetary-odf (rosetta/mro/juno_ocru) — 7/9 Legs an GH-API-403 gescheitert

- `mro_odf`-Registrierung geradegezogen (diese Session): `phi/sources.φ:6712` zeigte auf das auf CDN nicht existente `mro_odf.bin`; jetzt 14 Shard-Blöcke `mro_odf_t*.bin` (Muster wie `odyssey_odf`/`voyager_odr`), `(ttl asc, url asc)`-Ordnung eingehalten. `cdn_reconcile` (`tools/register`) misst Register↔CDN, blockt aber nicht — die Lücke war messbar, nicht schließend.
- Lauf `35190614514` (headSha `1e4faf79`) `failure`: 7/9 Matrix-Legs starben an `HTTP 403: API rate limit exceeded` beim `ensure release` (Schreib-Token `OMEGAFLOW_TOKEN`), `rosetta_odf` `cancelled`; Lauf `35189038542` (headSha `87680961`, enthält den mro-Speicher-Fix) `in_progress`. (Schritt: bei success `sha256`/`rosetta_odf`-Block + `juno_ocru_odf.bin`-Block in `phi/sources.φ`; das Schreib-Token-Budget ist der Blocker — `An entscheid:`-Zeile in `post.md`, committet `68f576bb`.)

## DEMETER — Harvest 0 Dateien (Quellen-WAF)

- Harvest `35146819646` success = Budget-Stopp: Log `0 files on disk, 0 orders done`; jede Order `WAF blocked … waiting 1800s` / `order parse void`. (Schritt: neue Orders = Dritt-Akt → Consent über entscheid; `An entscheid:`-Zeile in `post.md`, committet `68f576bb`.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Benchmark — Routine-Verifikation

- TNF-Anomalie und planetary-odf-Diagnose an `grind-flash` delegiert (2 Dispatches); die Klasse „Routine-Verifikation" ist geschlossen (flash gewinnt 2,4–11×, 2026-09-16) — kein Doppel-Lauf gegen pro/max.

## Geteilter Baum — eigener Pfad-Satz

- Fremde uncommittete Arbeit (nicht im eigenen Commit-Pfad, gemessen 2026-09-17): `docs/zustand/external-state.md` (` M`, trägt den veralteten CI-Status bei `bc9d6a0b`); gestagte Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`. (Schritt: eigener Commit-Pfad = `phi/sources.φ` (mro_odf-Shards) + dieses Handover + das archivierte folge61; `post.md` ist clean, die entscheid-Zeilen sind in `68f576bb` committet; `external-state.md` nicht anfassen — sein CI-Status-Eintrag ist fällig für seinen Eigner.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
