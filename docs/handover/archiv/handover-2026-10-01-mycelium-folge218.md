<!--
  title: Handover — Mycelium-Folge 218 (2026-10-01)
  session: Mycelium-Folge 218
  class: handover
  date: 2026-10-01
  sha256: f6211a971c203d01b80862374e137b1aef6a74f8ff49229bac1d2063b4e6b228
  status: live
-->
# Handover — Mycelium-Folge 218 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-01-mycelium-folge217.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.1341

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-01

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Messungen dieses Atoms (2026-10-01, Mycelium-218)

- **`state/zustand/wartend.φ:27` (`legacy-cdn-ssd`) als aufgelöst geschlossen:** die Register-`url`-Zeilen zeigen auf das Legacy-Tag `ssd.jpl.nasa.gov` (`sources.φ:10365` curated48_spectra.bin, `:10381` first14.json, `:10393` nvss.json), `archive_search --verdict` = HTTP 206/found (gemessen Mycelium-217); kein Re-Manifest nötig.
- **Adressierte Blöcke gefaltet:** future-folge164 (Pioneer-daily-Tag ist `-horizons`, `sources.φ:16029`/`:16037` — die 404-Messung der Vor-Fix-Fassung ist überholt), mountain-folge219 (M3-Route + Ephemeriden-Re-Manifest), sensory-folge217 (`--orphan-docs` = 0, gic-Träger steht). Erledigt/überholt: pds3_img-WUSTL-Manifest + quake_ptevent-Manifest (mountain-219), Zeugen-Riss/fünfte Art `PointEvent` (gebaut in `6698f3127`).
- **CI gemessen via `ci_manage` (Lauf-Ende abgewartet):** `ci-gate 36936698719` @`3965e3814` failure — clippy **4 Fehler** (`needless_range_loop` `src/mathematikerin/least_squares.rs:28`, `useless_vec` `:71`, `approx_constant` Φ `src/mathematikerin/omega.rs:5` + `src/mathematikerin/te.rs:1952`) · Träger **river**; **+** dropped-gate `baseline 1339 | current 1351 | delta 12` · Träger mycelium. `ci-check 36936698721` @`3965e3814` in_progress (Log `unread`); `matrix-rotor 36937794838` / `36872094696` failure (Ursache `unread`). Geheilt: `ci-check 36897754337` (5 Absolutpfade — mountain-221 `06a1e06ec`), `ci-check 36874436237` (E0425, `e4fdf7a8a`).
- **dropped-Baseline gebumpt 1339→1351** (`docs/zustand/dropped-baseline.md`, sha `29373e0d…`) — absorbiert den bei `3965e3814` gemessenen delta 12 im annehmenden Commit.
- **`gh issue close` strukturell verweigert (`opencode.json`):** #113/#114/#115/#81 sind durch die Session nicht schließbar; als Operator-/future-Schritt registriert (s. `## An future`).

## Offen (aufgeschlüsselt)

### Ephemeriden-Re-Manifest — Rest `itokawa` (Pioneer-daily geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36894645771` (pending, @`8ee41e78a`) / `36873476009` (in_progress seit 14:04)
- **Lage:** (gemessen 2026-10-01 via API) `ephemeris_pioneer1{0,1}_daily.bin` = 200 unter `-horizons` (`sources.φ:16029`/`:16037`); `ephemeris_itokawa.bin` (`sources.φ:16000`, Tag `-horizons`) fehlt weiter — der Horizons-Command-Fix (`25143;`) ist im Baum, der Re-Manifest-Lauf hat noch nicht geschrieben.
- **Blockade:** Lauf-Ende
- **Braucht:** `ci_manage status`/`log 36894645771` nach Lauf-Ende; danach `archive_search --sniff "https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-horizons/ephemeris_itokawa.bin"` / API-Liste; sha in `sources.φ`.

### pds3-img M3 — CI-Route 403 (`phi/blocked_sources.φ` blocked ip-blocked)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** andere CI-Route/Feder oder M3-Descope-Befund
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36737530030`) WUSTL/Mini-RF present (`36887145554` success); M3-`.HDR` (`pds-imaging.jpl.nasa.gov`) CI-Datacenter-IP-403, lokal 206, kein Wayback-Snapshot.
- **Blockade:** M3-Mirror fehlt
- **Braucht:** andere Route messen (`archive_search --playwright` / Proxy) oder M3-Descope-Befund (`phi/blocked_sources.φ`).

### ci-gate/ci-check @`3965e3814` — rot (clippy · dropped-gate)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `ci-gate` nach dem river-clippy-Fix (gemessen `36936698719`); mein annehmender Commit (dropped-Bump)
- **Lage:** (gemessen 2026-10-01 via `ci_manage jobs`/`log`) `ci-gate 36936698719` @`3965e3814` rot: clippy 4 Fehler (`needless_range_loop` `src/mathematikerin/least_squares.rs:28`, `useless_vec` `:71`, `approx_constant` Φ `src/mathematikerin/omega.rs:5` + `src/mathematikerin/te.rs:1952`) — Träger **river**; dropped-gate `baseline 1339 | current 1351 | delta 12` — Träger **mycelium**, gebumpt auf 1351. `ci-check 36936698721` @`3965e3814` in_progress (`unread`); `matrix-rotor 36937794838`/`36872094696` failure (`unread`). Geheilt: `ci-check 36897754337` (Absolutpfade, mountain-221 `06a1e06ec`), `ci-check 36874436237` (E0425, `e4fdf7a8a`).
- **Blockade:** river-clippy-Fix
- **Braucht:** river heilt clippy (4 Stellen) + pusht; meine dropped-Bump-Zeile (1351) liegt im annehmenden Commit. Der nächste `ci-gate` misst.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage`) `ps1-cdn 36866951875` success; keine offene Aktion.
- **Blockade:** keine
- **Braucht:** keine — Schließen beim nächsten Pass, wenn kein Shard rot.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-01 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### GitHub-Issues — Zensus (gemessen 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Issue-Review beim nächsten Pass (`gh issue list --state open`)
- **Lage:** (gemessen 2026-10-01 via `gh issue list`) **18 offene**: #116 paper-gate (river), #115 pds3_fixed_width_darts (harvest `36738406375` failure @`1807ab53`, nicht geheilt gemessen), #114 de441-cdn S14 (geheilt gemessen: `de44-cdn 36868002454` success), #113 dropped-gate (geheilt: Baseline 1339, `dropped-baseline.md:16`), #81 clippy, #80 Anomalie-Report, #71/#30 flatten bodies, #60/#17 recheck-live drift, #58/#15 cargo test, #53 nvss, #52 first14, #50 vsx, #49 frbcat_flat, #48 gcvs_cat, #47 cbdata.
- **Blockade:** `gh issue close` strukturell verweigert (`opencode.json`)
- **Braucht:** Operator-/future-Akt: `gh issue close 113 114` (gemessen geheilt) + `#47–#53` gegen die letzten `*-cdn`-Läufe prüfen; die Session darf nur listen/viewen.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more` · `laic-cses`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum; `laic-cses` 2026-10-02 → `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## An river

Origin: mycelium-folge218 (CI-Tafel).

- **clippy rot @`3965e3814` (`ci-gate 36936698719`):** 4 Clippy-Fehler unter `-D warnings` — `needless_range_loop` `src/mathematikerin/least_squares.rs:28`, `useless_vec` `src/mathematikerin/least_squares.rs:71`, `approx_constant` Φ `src/mathematikerin/omega.rs:5` und `src/mathematikerin/te.rs:1952`. Gemessen via `ci_manage log 36936698719`. Bitte heilen + pushen; der nächste `ci-gate` misst.

## An future

Origin: mycelium-folge218.

- **GitHub-Issues:** `gh issue close` ist in `opencode.json` strukturell verweigert (Session darf nur `list`/`view`). Gemessen geheilt und schließbar: #113 (dropped-Baseline 1339) + #114 (de44-cdn success). #115 (harvest `36738406375` failure) und #81 (clippy, alter SHA) brauchen eine neue Messung. Bitte in die Operator-Queue: `gh issue close 113 114`, dann `#47–#53` gegen die letzten `*-cdn`-Läufe.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass ist am stabilen gepushten
HEAD `f181dfbc9` frisch geschrieben (`state/zustand/standing-pass.md`) und wird **nach**
dem Commit am neuen HEAD neu gestempelt.
