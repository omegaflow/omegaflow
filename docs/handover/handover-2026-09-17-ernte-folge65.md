<!--
  title: Handover — Ernte-Folge 65 (Stand 2026-09-17)
  session: Ernte-Folge 65
  class: handover
  date: 2026-09-17
  sha256: e110bedaaaf09f9888f82702d260ccd3ad4f31a959964953273c12d92e567de3
  status: live
-->
# Handover — Ernte-Folge 65 (2026-09-17)

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

## DEMETER — Route entschieden, Code gefixt, Runner-Install bei entscheid (härtester undatiert)

- **Gemessen:** Der Order-POST `POST https://regards.cnes.fr/api/v1/rs-order/user/orders`
  von der Residential-IP der Operator-Maschine → `201 Created`/JSON; derselbe Code und
  curl-UA vom GitHub-Runner (`ubuntu-latest`) → `<title>Request Rejected</title>`
  (F5 ASM), über 4 h stabil. **IP-basiert, kein UA-/Token-/Rate-Problem.** Der CI-Lauf
  `35146819646` (`demeter-cdn`) war `success` bei `0 files on disk` — falsches Grün;
  die `WafBlocked`-Klassifikation im Harvester war eine Fehldeutung jedes Nicht-JSON.
- **Rat (2026-09-17):** Route C — self-hosted Runner, Label `demeter-residential`, trägt
  `demeter-cdn.yml`; `demeter-aggregate-cdn.yml` bleibt auf `ubuntu-latest` (geteilte
  Cache-Wurzel). A (lokaler `--ci-mode`-Upload) fällt an der Writer-Konvention, B/D am
  fehlenden Transferkanal.
- **Umgesetzt (Code):** `tools/harvest/src/bin/demeter_harvest.rs` — `fetch_http` reicht
  Status+Body durch (`HttpReply`); `CreateError {WafBlocked, Unauthorized, HttpStatus(u16),
  UnparsableOk, CreateFailed}`; `WafBlocked` nur beim gemessenen `Request Rejected`; 401 →
  sofortiges Re-Login; Token-Refresh (`TOKEN_REFRESH_SECS=2700`) an der Verwendungsstelle
  (vor jedem `create_order` und vor der Status-Schleife); 0 Dateien + unvollständiges Ledger →
  roter Exit (kein falsches Grün); `login void`/`catalog void` → rot.
  `.github/workflows/demeter-cdn.yml` — `runs-on: demeter-residential`, `timeout-minutes: 540`,
  Cron entfernt (keine stehende Dritt-Schreibung), `--budget 28800`. `cargo check -p omegaflow-harvest`
  0 Fehler/0 Warnungen.
- **Offen:** (1) Runner-Install auf der Operator-Maschine — Operator-Wort ad hoc (eigene
  Domäne, nicht `entscheid`). Runner repo-gebunden, Label `demeter-residential`, eigener
  Nutzer, systemd, `timeout-minutes: 540`, nie in einem `pull_request`-Workflow; gemessen:
  kein Workflow außer `demeter-cdn.yml` zielt darauf, kein nacktes `self-hosted` im Baum.
  (2) Danach `gh workflow run demeter-cdn.yml`; Ergebnis beim nächsten Planungs-Pass
  (`ci_manage view <id>`), kein Poll. (3) Nach erfolgreichem Aggregat: die 77 `url`-Zeilen
  (`format demeter_isl`, `at earth`, `ttl 604800`, Felder
  `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`) + sha256 ans Ende von
  `phi/sources.φ` — heute bewusst nicht (kein Asset, 0-Kanon).
- (Schritt: Operator installiert den Runner; dann Dispatch `gh workflow run demeter-cdn.yml`.)

## CI-Queue — Root-Fix steht, keine Neu-Dispatches bis Slots frei

- `max-parallel: 8` in `gaia-xp-full-cdn.yml`, `max-parallel: 4` in `physionet-cdn.yml`
  (Vorbild `igets-cdn.yml`) sind gesetzt. (Schritt: Slot-Lage bei Bedarf über `ci_manage list`;
  keine Neu-Dispatches von gaia/physionet, bis die laufenden Jobs frei sind.)

## MAVEN TNF — Lauf queued, Registrierung offen

- `35205267703` (`3bc1455f`, `force=true`). Bei success: `sha256`-Zeilen + `maven_tnf`-Block
  ans Ende von `phi/sources.φ` (`sgrep maven_tnf phi/sources.φ` = 0 Treffer).
  (Schritt: `gh run view 35205267703`.)

## Mariner 10 PSPA-00316 — Lauf queued, Registrierung offen

- `35204897220` (`67f4e8c1`), einziger Lauf der Workflow `mariner-occlt-cdn`. Bei success:
  Register-Block `mariner_occlt` + `…_register_field_names_match_components`-Test.
  Pending (eigene Atom-Linie): Sample-Rate/Record-Dauer (Tag-Inkremente 18…20.898, nicht
  äquidistant), Frac-Einheit (25-ms-Ticks vs. BCD-Zentisekunden), unabhängiger UTC-Anker
  (1974-02-05 aus `attrib`, unbestätigt). (Schritt: `gh run view 35204897220`.)

## planetary-odf — Läufe offen, Register↔CDN-Lücken gemessen

- `rosetta_odf` registriert (`phi/sources.φ:6374`), CDN-Asset auf `archives.esac.esa.int`
  absent → Block braucht `sha256` bei success. `juno_ocru_odf` weder registriert
  (`sgrep juno_ocru_odf phi/sources.φ` = 0) noch auf `atmos.nmsu.edu`. Läufe: `35189038542`
  (`87680961`) in_progress, `35190614514` (`1e4faf79`) failure (7/9 Legs `HTTP 403`),
  `35218760142` (`f070ca36`) queued. (Schritt: bei success `sha256`/`rosetta_odf` +
  `juno_ocru_odf.bin`-Block in `phi/sources.φ`.)

## CDN-Register-Schuld — 20 registrierte Assets fehlen am CDN (gemessen 2026-09-17)

- Gemessen gegen `omegaflow/sources` (207 Releases, 5532 Assets): von 365 registrierten
  CDN-`url`-Zeilen (76 Netloc-Tags) fehlen 20 Assets; daneben 588 direkte Nicht-CDN-URLs
  (115 Hosts, per Konstruktion nicht manifestiert). Kein Phantom — 15 echte
  Nachmanifestationen, 4 überholte Registrierungen, 1 gemessen-ungebaute Kette.
- **Nachmanifestation (15, Workflow dispatch):** `rosetta_odf` (`planetary-odf-cdn`),
  `cosmic_ro_temp` (`cosmic-cdn`), `noe4_deimos`/`noe4_phobos` (`noe4-cdn`), `ned.json`
  (`ned-cdn`, läuft automatisch bis alle `ned_part_*` da sind), `swot_l2_lr_ssh` (`swot-cdn`),
  `gedi_l2a` (`gedi-cdn`), `icesat2_atl03` (`icesat2-cdn`), `hess_dl3`/`magic_dl3`
  (`dl3-skymap-cdn`, `--telescope hess|magic`), `vlass_tap_component`/`vlass_tap_source`
  (`vlass-tap-cdn`), `maxi_J0006+202` (`maxi-cdn`), `isc_bulletin` (`isc-cdn`),
  `uscrn_hourly` (`uscrn-cdn`).
- **Registrierung bereinigen (4, `phi/sources.φ`):** `xp_pilot_p6144` (Z.6474–6478, ersetzt
  durch `ssd.jpl.nasa.gov/xp_spectra.bin` + `xp_full_*`); `ghcn_d_USW00094728_2025-03`
  (Z.6639–6647, ersetzt durch die Volljahr-Form); `mpcobs.bin` (Z.1841–1844, ersetzt durch
  `mpcobs-unnobs-*`); `superdarn.ca/superdarn_fitacf` (Z.7451–7455, verwaiste Doppelreg. —
  der lebende Eintrag ist `zenodo.org/superdarn_fitacf.bin`, Z.7548).
- **Pending (1):** `las_oahu_bathy_1999` — Granule messbar, aber kein Netloc-Workflow
  (`las-cdn.yml` nur USGS) und der MLLW→Ellipsoid-GTX-Reader ungebaut. (Schritt: GTX-Reader
  + noaa-nos-LAS-Workflow bauen.)
- (Schritt: die 15 Workflows dispatchen; die 4 Zeilen bereinigen; kein Massen-Dispatch ohne
  freie Slots.)

## CDN-Idempotenz-Gates — Nachweis über nächste Läufe

- 47 `*-cdn.yml` auf das aia-cdn-Muster (`id: idempotence` + `present`-Output + `if:`)
  umgestellt; erster Nachweis argo-bgc `35154746956` `present=true` + übersprungener
  Compile-Step. (Schritt: der nächste reguläre Lauf je Workflow zeigt dasselbe; kein
  Massen-Dispatch.)

## Benchmark

- DEMETER-Diagnose lief über drei flash-Dispatches (WAF-Rohantwort, CI-Log + Fluss,
  `cargo check`) — alle korrekt und vollständig, kein pro/max nötig. Ein vierter
  `grind-pro`-Dispatch (Harvester-Fix) lieferte einen leeren Report und **änderte nichts**;
  der Fix lief dann in `build` selbst. Der `council`-Dispatch trug das Routen-Urteil.
  Klasse „Routine-Verifikation" bleibt geschlossen (flash 2,4–11×).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/demeter_harvest.rs`,
  `.github/workflows/demeter-cdn.yml`, `docs/handover/handover-2026-09-17-ernte-folge65.md`
  (+ archiviertes `folge64`). Fremde uncommittete Arbeit (nicht anfassen):
  `.opencode/command/{bau,entscheid,ernte,forschung,start}.md`, `docs/zustand/external-state.md`,
  `docs/handover/post.md` (fremde Session-Änderung), die gestagten Renames
  `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
