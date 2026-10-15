<!--
  title: Handover — Mycelium-Folge 352 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 679cfc1822b3e8bb36b0634e14ce0c2ee6987af62ab76ad561f9723a9bec7d4e
  status: live
-->
# Handover — Mycelium-Folge 352 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge351.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge349` (`xfer:mountain-348-3`
IGETS-`origin`, `-6` GEOFON-`field` — beide geantwortet) · `river-folge213` (CTBTO-Formular,
Weberin-Zeugen, Register→Manifest). Der Baum gewinnt: das GEOFON-`field` steht (`phi:170`).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein
Meßwert. Grund: die Register-Messung, die 191 neue `raw_mirror`/`reference`-Blöcke,
`register_sort`/`cdn_reconcile --fail`/`cargo check`.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „WDC-B-RETTUNG ERWEITERN — von 32 auf ~205 ladbare Dateien …" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge352) |
| „DEN LEBENDEN BAUM registrieren: www.wdcb.ru/sep/ … + die Schwester-Hosts …" | 2026-10-15 | dito |
| „Die drei Zenodo-Dateien (CMONOC-GNSS, CEDC) ins Manifestations-Register ziehen." | 2026-10-15 | dito |
| „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater …" | 2026-10-15 | dito |
| „url/origin/compiler je neuer offener Quelle setzen: BGI · IGETS · GSRAS-ftp · WDC-B · ISC · CMONOC-GNSS (Zenodo) · CEDC 8084659 · CB-FDSN · NGL-ARDA. … ARDA NICHT als Schwere/Seismik …" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge351) |
| „bitte schicke taucher los um den echten pfad zu messen WDC-B (Obninsk) …" | 2026-10-15 | dito |
| „aber machen wir das nicht schon ist das nicht unser beitrag zum erbe?" | 2026-10-15 | dito |
| „auf jedem fall mit antrag" — Route (b): CN-NSSDC-Datenantrag | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| „Führe den … bestätigten Plan aus … Commit und Push trägt `/commit`." — session-weiter Consent, **nicht** das Commit-Wort | 2026-10-14 | dito |
| „das ist privat" — die private Pass-Fassung wird öffentlich nicht gelesen | 2026-10-14 | dito |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" | 2026-10-13 | dito |

## Gearbeitet (dieses Rund)

- **WDC-B-Rettung von 32 auf 211 erweitert.** Der volle bestätigte Wayback-Satz: die
  **179 neuen** Payload-Dateien (CDX, `collapse=urlkey`, `statuscode:200`) sind als
  `raw_mirror`-Blöcke in `phi/sources.φ` registriert (`url …/web.archive.org/<name>`,
  `origin https://web.archive.org/web/<ts>id_/…`, sha256 je Datei, `at earth`,
  `no-cadence`) und in die Matrix `.github/workflows/wdcb-obn-wayback-cdn.yml` (32 → 211)
  aufgenommen. Gruppen: `sep/GravConst/data` 55 · `stp/geo_min.val` 44 (MSK .ZIP + MIR .WDC)
  · `sep/data/gdata` 31 · `sep/data/sdata` +18 neu (cat002, cat_CAU*, `OBN2000*`, `sig_2000`)
  · `stp/geomagni.ind/aa` 16 · `stp/catal_pc` 8 · `gps` 7. **Beleg:** jede Datei von Wayback
  `id_` geladen und gehasht (sha256 = der `raw_mirror`-Gate-Wert; gegen `bul199806.dat`
  byte-identisch verifiziert). **Mountain hat die Verdikt-Seite geliefert**
  (`xfer:mountain-349-1`, `mountain-folge350:224-228`); als **direkter Edit des Punkts des
  Eigentümers** angewandt, beide Enden genannt: die 179 Wayback-Blöcke tragen
  `terms CC-BY-NC-4.0` (die Erbe-Seite), die 7 lebenden `www.wdcb.ru`-Blöcke
  `terms wdcb-sep-policy http://www.wdcb.ru/sep/policy.html` (Site-Policy, **kein**
  CC-Ersatz). `field` je Datensatz bleibt Mountain. `register_sort phi/sources.φ` canonical
  (2805 Blöcke); `cdn_reconcile --fail` clean (334 Hosts); `cargo check` 0/0.
- **Der lebende Baum registriert.** `www.wdcb.ru/sep/seismology/catalogues/cat001–cat007`
  (cat003 als `cat003.gz`; live 200, 7 Dateien) als 7 `raw_mirror`-Blöcke (Tag `www.wdcb.ru`)
  + neuer Lauf **`.github/workflows/wdcb-sep-live-cdn.yml`**. Danach die **Schwester-Hosts**
  als `reference`-Blöcke (url = live, sha256 der Wurzel): `www.wdcb.ru/sep/` · `usd.wdcb.ru`
  (Weltraum) · `esdb.wdcb.ru` (DOI) · `rjes.wdcb.ru` · `stp.wdcb.ru`. Policy gemessen
  (`sep/policy.html`): offen und frei für wissenschaftliche/Lehr-/nicht-kommerzielle Zwecke,
  Nennung Pflicht, kommerziell nur nach Kontakt; als `terms wdcb-sep-policy` an den 7
  Katalog-Blöcken gesetzt (Mountains Verdikt). Die 5 Host-`reference`-Blöcke tragen noch
  kein `terms` (Host-Policy ungemessen) → `xfer:mycelium-352-3`.
- **Die drei Zenodo-Assets sind im Manifestations-Register** (worked in 351): `gps_cmonoc.dat`
  · `gps_campaign.dat` (CMONOC 7798617) · `cedc_catalog.xlsx` (8084659) — je `url`/`origin`/
  `compiler raw_mirror.rs`/`harvest raw_zenodo`/sha256/`at earth`/`no-cadence` in
  `phi/sources.φ` **und** in der Matrix `.github/workflows/zenodo-raw-cdn.yml`. Nur der
  CI-Lauf (`zenodo-raw-cdn`) ist noch unread.
- **GEOFON-CI-Naht gebaut:** `.github/workflows/geofon-waveform-cdn.yml` — `workflow_dispatch`,
  `fdsn_station_compiler --network GE` + `fdsn_waveform_compiler --route geofon --network GE
  --station WLF --channel BHZ` (gemessene GE-Station WLF, Walferdange/Luxemburg), Upload auf
  den `geofon.gfz-potsdam.de`-Tag. Der offene Punkt „CI-Naht fehlt" ist geschlossen.
- **Läufe dieses Runds gemessen:** `wdcb-sep-live-cdn` `38704082015` **success** ·
  `sources-repo-licence` `38704081473` **success** · `register-coverage` `38704150805`
  **success**; `wdcb-obn-wayback-cdn` `38704081781` **queued** (die 211er-Matrix).
- **Werkzeug-Riss gemessen:** `archive_search --wayback` (CDX) liefert `HTTP 503`; der
  `web.archive.org/cdx/search/cdx`-Endpunkt antwortet per `curl` mit 200. Die CDX-Belege
  (diese Runde) wurden über den Endpunkt geholt; `sfetch` meldet `fetch void`. Kein
  Register-Akt, nur die Route.

## Offen — eigen

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug **unvollständig** (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Nur **2000** liegt (doppelt); **2001–2018 fehlen**.
  `local-only` (© UH Mānoa, keine Lizenzzeile; Block `xfer:mountain-348-1` setzt
  `terms local-only`) — kein CDN, keine `phi`-Zeile.
- **Blockade:** keine.
- **Braucht:** `terms local-only` von Mountain; dann `url`/`origin`/`compiler` setzen; der
  korrigierte Vollzug **2001–2018** (nur reale Granule, nicht das 5-Minuten-Raster).

### Register→Manifest — CI-Trigger gebaut; der generische Aktor fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau (berührt die Register-Semantik → Rat)
- **Lage:** (gemessen 2026-10-14/348) `harvest-dispatch.yml` reagiert auf eine ausdrückliche
  `harvest <actor>`-Zeile und auf `format`-Diffs. Offen: der **generische Aktor**
  (`# harvest: format=csv arm=raw_mirror`) für die compiler-losen Formate; die statischen CSV
  (SILSO, EDGES, Fermi-TGF) brauchen eine registrierte `sha256`.
- **Blockade:** der Bau berührt die Register-Semantik (Fingerabdruck-Tor) → Rat.
- **Braucht:** generischen Aktor entwerfen/bauen (Mycelium, Rat-Vorlage).

### Venus-Radar (Level-2 VNRD) — Block steht; Bild-Manifestation offen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/350) Der VNRD-Block steht (CC0-1.0, `compiler
  venus_radar_compiler.rs`, `field venus_radar_inc_deg/_ocp/_scp`). Offen: die
  **Bild-Manifestation (W)** und die **CI-Naht** (15×960 MB).
- **Blockade:** keine.
- **Braucht:** Bild-Manifestation + CI-Naht.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) Offen: **SEANOE** `…/data/00311/42182/` (206; Lizenz
  `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung. An Mountain: `xfer:mycelium-342-3` (SEANOE) ·
  `xfer:mycelium-342-4` (GEOFON).

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-14/348) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`
  (Sonnen-Magnetfeld · Stern-Aktivität · 21 cm · Wolken · TGF). **Compiler-Zensus:** nur
  `hmi_polar_compiler.rs` und `wwlln_compiler.rs` existieren; die statischen CSV sind
  `raw_mirror`-Kandidaten.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht; für die compiler-losen Quellen
  zusätzlich der generische Aktor (Rat).
- **Braucht:** Mountain-Zeilen aus dem Entwurf. An Mountain: `xfer:mountain-348-2`.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl`/Zulassung je Block
- **Lage:** (gemessen 2026-10-13/344) unverändert offen: **Ersatzquellen-Registerblöcke**
  (OCNS2019 `local-only`; JPL SSD/Horizons offene Alternative) · **LOD im `finals`-Arm** ·
  **BASE** (Venus-Radar-PDS > 2-GiB-Limit `cdn.rs:230`) · **WDC-Kyoto-Token** + vier
  `terms`-Korrekturen · **`ttl`/`no-cadence` + `register_sort`**.
- **Blockade:** Verdikt-/`terms`-/`field`-Recht = Mountain.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:future.…7d5c9163` (OCNS2019 `local-only`) ·
  `xfer:mycelium-344-5` (LOD) · `xfer:mycelium-343-3` (BASE) · `xfer:mycelium-342-10`/
  `xfer:mycelium-343-2` (WDC/terms).

### TIMED-Weltlinie — Block zu setzen; TLE-`origin` ungemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/352) Der Compiler steht (`timed_tle_compiler.rs`, das
  eingebettete TLE für NORAD 26998); `data/timed_orbit.bin` liegt (sha256
  `bf5609e9…`). Der Block (`url`/`origin`/`compiler`, `format orbit_bin`, `at timed`) fehlt;
  **die `origin`-Zeile ist ungemessen** — der kanonische TLE-Dienst `celestrak.org/NORAD/…`
  ist in diesem Rund **nicht** erreichbar (curl timeout, 000). Kein erfundener `origin`.
  `terms`/`ttl` = Mountain.
- **Blockade:** die TLE-`origin` (celestrak) time-out — ungemessen.
- **Braucht:** die TLE-`origin` messen (bei erreichbarem Celestrak), dann den Block setzen;
  danach die CI-Naht `timed-orbit-cdn.yml`. An Mountain: `xfer:mountain-348-5`.

### NSSDC/CLPDS-Planetologie — `完全共享` anonym ladbar; `有条件共享` mit Antrag
- **Status:** eigen (Antrag operator-gebunden; Route entschieden) | **Bindung:** eigen
- **Trigger:** Operator-Wort „auf jedem fall mit antrag" (2026-10-14) — Route (b); der Send
  bleibt Operator-Hand
- **Lage:** (gemessen 2026-10-14/347) Antragsentwurf liegt bei Future. **Riss:** Katalog-Record
  `CC BY 4.0` vs. CLPDS-„all rights reserved". Offene Mirrors gemessen (CE-1/2 MRM → NASA PDS;
  CE-4 LPR / Tianwen-1 RoPeR → Zenodo CC-BY-4.0; MoRIC-DOM → CDS HiPS ODbL).
- **Blockade:** Antrag = Operator-Hand; Lizenz-Riss ungeklärt.
- **Braucht:** Operator sendet den CN-NSSDC-Antrag; die `完全共享`-Sätze sind anonym ladbar.
  An future: `xfer:mycelium-347-2`.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen
- **Trigger:** kein — Autofill-Eintrag entfernen
- **Lage:** (gemessen 2026-10-14/347) Der Login ist rotiert → der exponierte Wert ist
  entwertet.
- **Blockade:** Operator.
- **Braucht:** nur noch den Autofill-Eintrag im Browser entfernen.

### WDC-B Obninsk — Rettung erweitert; Abdeckung weiter unvollständig (eigen)
- **Status:** eigen (Rettung erweitert; Rest = Vollständigkeit + Verdikte) | **Bindung:** eigen
- **Trigger:** kein — Beobachtung neuer/anderer Archive
- **Lage:** (gemessen 2026-10-15/352) **211** Wayback-Blöcke registriert + 7 lebende
  `www.wdcb.ru/sep`-Kataloge; die lebende `zeus.wdcb.ru`-Quelle ist tot. `terms` der 179
  Wayback- und 7 lebenden Blöcke ist angewandt (Mountains Verdikt `xfer:mountain-349-1`).
  Offen: `field` je Datensatz, die `terms` der 5 Host-`reference`-Blöcke = Mountain
  (`xfer:mycelium-352-1`/`-2`/`-3`).
- **Blockade:** keine.
- **Braucht:** die CDX-Abdeckung ist **nicht** 1993–2024 komplett; weitere Mirror/Archive
  beobachten; der Rest bleibt `pending`. **Gemessen (2026-10-15/352): die URL-Gruppen
  `sep/strong` (422 URLs), `sep/traverses` (172), `sep/hp` (153) tragen KEINE
  Datendateien** — ausschließlich `text/html` und `image/gif|jpeg` (Beben-Seiten und
  Kartenbilder); der Punkt ist damit geschlossen, kein Mirror.

## An mountain

Origin: mycelium-352 (2026-10-15):

- `xfer:mycelium-352-1` **WDC-B Wayback-Rettung, die 179 neuen Blöcke** — je `url
  …/web.archive.org/<name>`, `origin …/<ts>id_/…`, `compiler raw_mirror.rs`, sha256,
  `at earth`, `no-cadence`, `terms CC-BY-NC-4.0` (dein Verdikt `xfer:mountain-349-1`,
  angewandt); Matrix `wdcb-obn-wayback-cdn.yml` (32 → 211). **Setze `field` je Datensatz**
  (der Obninsk-Katalog: fixe Spalten lat/lon/Tiefe/M; `GravConst`: Gravitationskonstante;
  `geo_min.val`: geomagnetische Minutenwerte; `gdata`: Gravimetrie/Magnetik; `aa`:
  geomagnetischer Index; `gps`: GPS-Metadaten). **Kein** Blanko-`field` — je Gruppe gemessen.
- `xfer:mycelium-352-2` **Der lebende `www.wdcb.ru/sep`-Baum** — 7 `raw_mirror`-Blöcke
  (`cat001.dat`…`cat007.dat`, `cat003.gz`) + Lauf `wdcb-sep-live-cdn.yml`; `terms
  wdcb-sep-policy http://www.wdcb.ru/sep/policy.html` ist angewandt (dein Verdikt). Setze
  `field`.
- `xfer:mycelium-352-3` **Schwester-Hosts** — 5 `reference`-Blöcke: `http://www.wdcb.ru/sep/`
  · `http://usd.wdcb.ru/` (Weltraum: cosmos49/321/1809, prognoz, vega) · `http://esdb.wdcb.ru/`
  (DOI-prägend) · `http://rjes.wdcb.ru/` · `http://stp.wdcb.ru/`. Die konkreten Daten-Subtrees
  sind ungemessen (`pending`); setze `terms` je Host.
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC `terms local-only`) ·
  `xfer:mountain-348-2` (Fünf Fäden) · `xfer:mountain-348-4` (OCNS2019-Ersatz) ·
  `xfer:mountain-348-5` (TIMED-Block) · `xfer:mycelium-342-3`/`-4` (SEANOE/GEOFON) ·
  `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).

## An river

Origin: mycelium-352 (2026-10-15):

- `xfer:mycelium-352-4` **`ci-gate` rot am HEAD `10b521783`** (Lauf `38702218068`, `failure`) — **gemessener Grund
  aus dem Log:** `cargo clippy`/Test-Bau des **Kerns** bricht: `error[E0425]: cannot find
  function 'frame_earth' in this scope` (`src/mathematikerin/machines/tests.rs:461/489/498`)
  + Clippy-Lints (`too many arguments (12/7)` `src/archivar/extract.rs:4100`; manual
  `Range::contains` `src/archivar/tle.rs:90`; negated comparison `src/archivar/tle.rs:147/341`;
  very complex type `src/mathematikerin/machines/matrix.rs:219/1158`,
  `src/mathematikerin/omega.rs:2099`). **Träger:** River (Feld/Kern). Kein Mycelium-Pfad.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du
  nicht messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein
  Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort. Dieses Rund trägt das Wort „Am Ende: ein
Commit (deine Pfade)."
