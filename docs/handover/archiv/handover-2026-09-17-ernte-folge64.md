<!--
  title: Handover — Ernte-Folge 64 (Stand 2026-09-17)
  session: Ernte-Folge 64
  class: handover
  date: 2026-09-17
  sha256: d9285a43d0e967f1d7ef0b956f7b948e2f08ec8edd33ca1c2cf3a6c2f967e3f0
  status: live
-->
# Handover — Ernte-Folge 64 (2026-09-17)

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

## CI-Queue — versehentlicher Dispatch-Schwung saturierte die Account-Konkurrenz, gestoppt (härtester undatiert)

- Gemessen: um `06:36:49–06:36:57Z` ein Dispatch-Schwung — `gaia-xp-full-cdn` `35190604057`, `noaa-ghcn/gsod/isd-allstations-cdn` `35190606228`/`35190608279`/`35190610464`, `physionet-cdn` `35190612735`, `planetary-odf-cdn` `35190614514`. Die drei noaa-Läufe und planetary `35190614514` sind `completed`; **gaia** (257-Job-Matrix, kein `max-parallel`, 11 laufend) und **physionet** (23 Jobs, 8 laufend) hielten **19 der 20 Free-Tier-Slots** → jeder andere Lauf (`maven-tnf`, `mariner-occlt`, `planetary-odf`, alle scheduled `*-cdn`) `queued` mit `runner_id: 0`. **Kein Account-/Billing-Problem:** Repo `public`, Actions `operational` (githubstatus.com), `actions/permissions` `enabled`, keine `environment`/`needs`-Sperre in den YAMLs, `maven-tnf-cdn`-Gruppe frei (kein Duplikat).
- Aktion: `gh run cancel 35190604057` und `gh run cancel 35190612735`; Asset-Zahlen vor/nach unverändert (`dc.g-vo.org` 68, `physionet.org` 14) — **nichts gelöscht**; laufende Jobs standen im Compile-/Harvest-Schritt, nicht im Upload.
- Wurzel-Fix (umgesetzt): `max-parallel: 8` in `gaia-xp-full-cdn.yml`, `max-parallel: 4` in `physionet-cdn.yml` (`bidsleep` + `ltmm`) — Vorbild `igets-cdn.yml:58`. Eine 256-chunk-Matrix darf die Account-Konkurrenz nicht mehr monopolisieren.
- Die frühere `An entscheid:`-Zeile dieser Session (Account-/Billing-Verdacht) war eine **Fehldiagnose** und wurde von entscheid bereits abgeholt — Korrektur ist diese Messung; keine Billing-/Support-Aktion nötig.
- (Schritt: keine Neu-Dispatches von gaia/physionet — die notwendigen Ernte-Läufe laufen an, sobald die Slots frei sind.)

## MAVEN TNF — Force-Lauf queued, Registrierung offen

- `35205267703` (`3bc1455f`, `force=true`) war `queued` seit 09:28Z (Ursache: der Schwung oben). Bei success: `sha256`-Zeilen + `maven_tnf`-Blöcke ans Ende von `phi/sources.φ` (heute nicht registriert, `sgrep maven_tnf phi/sources.φ` = 0 Treffer). (Schritt: `gh run view 35205267703`.)

## Mariner 10 PSPA-00316 — Dispatch queued, Registrierung offen

- `35204897220` (`67f4e8c1`) `queued` seit 09:23Z, einziger Lauf der Workflow (`mariner-occlt-cdn`). Bei success: Register-Block `mariner_occlt` in `phi/sources.φ` (heute nicht registriert) + `…_register_field_names_match_components`-Test. (Schritt: `gh run view 35204897220`.)
- Pending (eigene Atom-Linie): Sample-Rate/Record-Dauer (Tag-Inkremente 18…20.898, Records nicht äquidistant), Frac-Einheit (25-ms-Ticks vs. BCD-Zentisekunden), unabhängiger UTC-Anker (1974-02-05 aus `attrib`, unbestätigt).

## planetary-odf — Läufe offen, Register↔CDN-Lücken gemessen

- `rosetta_odf` registriert (`phi/sources.φ:6374`, note „Offen: `planetary-odf-cdn.yml --ci-mode` nach Push"), CDN-Asset auf `archives.esac.esa.int` absent → Block braucht `sha256` bei success. `juno_ocru_odf` weder registriert (`sgrep juno_ocru_odf phi/sources.φ` = 0) noch auf `atmos.nmsu.edu`. Läufe: `35189038542` (`87680961`) `in_progress`, `35190614514` (`1e4faf79`) `failure` (7/9 Legs `HTTP 403`), `35218760142` (`f070ca36`) `queued`. (Schritt: bei success `sha256`/`rosetta_odf` + `juno_ocru_odf.bin`-Block in `phi/sources.φ`.)

## DEMETER — Route frei, Harvest am Quellen-WAF (aus `post.md` gefaltet)

- Die Route ist **nicht blockiert** (Key funktioniert, gemessen: Login 200, 57 760 `DMT_N1_1144`-Objekte, 42 Orders meist `DONE`); der Harvest `35146819646` scheitert am Quellen-WAF (`0 files on disk`, jede Order `WAF blocked … waiting 1800s` / `order parse void`) — neue Orders durchbrechen den WAF nicht, der Consent-Akt hat keinen Gegenstand. Der Quellen-Punkt bleibt bei ernte. (Schritt: nach erfolgreichem Aggregat die 77 `url`-Zeilen — `format demeter_isl`, `at earth`, `ttl 604800`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}` — ans Ende von `phi/sources.φ`.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`) umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein Massen-Dispatch.)

## Benchmark — Routine-Verifikation

- Die Klasse „Routine-Verifikation" ist geschlossen (flash gewinnt 2,4–11×, 2026-09-16); die CI-Diagnose lief über zwei flash-Dispatches (Concurrency + Ursache), kein Doppel-Lauf gegen pro/max.

## Geteilter Baum — eigener Pfad-Satz

- Fremde uncommittete Arbeit (nicht im eigenen Commit-Pfad, gemessen 2026-09-17): `docs/zustand/external-state.md`, die gestagten Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`, `docs/concepts/{archivar-mathematikerin,pfeiler-der-architektur}.md`, `src/archivar/{fetch,port,tests}.rs`, `src/gate/commit_gate_vocab.json`. Eigener Commit-Pfad = dieses Handover + das archivierte folge63 + `.github/workflows/{gaia-xp-full-cdn,physionet-cdn}.yml`; fremde Pfade nicht anfassen, nie ein nacktes `git commit`. Die DEMETER-`post.md`-Zeile ist in dieses Handover gefaltet; `post.md` trägt keinen eigenen Anteil (fremde Session-Edits unangetastet, nicht committet).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
