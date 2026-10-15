<!--
  title: Handover — Mycelium-Folge 354 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: dca8bfd77c4651c81ebfd11d5361114b96ab0a989125317e8a2c02183d5bf213
  status: live
-->
# Handover — Mycelium-Folge 354 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge353.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge351` (`xfer:mountain-348-3`
IGETS-`origin`, `-6` GEOFON-`field` — beide Echo/Status; `xfer:mountain-349-1` WDC-B-Verdikt
echoed/angewandt) · `river-folge215` (CTBTO-Formular = Operator-Hand; die Weberin-Zeugen-
Manifestation — **Athabasca in diesem Rund gesetzt**, s. „Gearbeitet"; der
Register→Manifest-Verbinder bleibt der generische `raw_mirror`-Aktor). Kein neuer Aufenthalt.

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein
Meßwert. Grund der Runde: die Register-Messung, `register_sort`/`cdn_reconcile --fail`/
`cargo check`, `cargo build -p omegaflow-harvest --bin raw_mirror` + lokaler Ende-zu-Ende-Lauf.

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

- **Athabasca ERG-SC VLF — die Manifestation ist gesetzt (river-215 gefaltet).**
  Gemessen: das Live-CDF `https://ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/2024/04/isee_vlf_ath_2024040405_v01.cdf`
  → **sha256 `5f93df95358556154ddc38756074f8cc8b58231702f708ed1758ce14b0699757`**, 60 958 381 B.
  Der Block (`phi/sources.φ`) ist auf die `raw_mirror`-Form gehoben: `url` = CDN-Release
  (`…/releases/download/ergsc.isee.nagoya-u.ac.jp/isee_vlf_ath_2024040405_v01.cdf`),
  `origin` = live, `compiler tools/harvest/src/bin/raw_mirror.rs`, `harvest raw_mirror`,
  `sha256`; das frühere `format cdf` **fällt** (kein `cdf`-Arm existiert). Die vier
  Feld-Zeilen + `terms erg-sc-rules-of-the-road` + `on earth 54.60282 -113.64442 693`
  (Athabasca, SABER-VLF-Netz) bleiben.
  Verifiziert: `register_sort phi/sources.φ` **canonical** (2 805 Blöcke) ·
  `cdn_reconcile --fail` → `cap+tag contract clean (334 registry hosts)` ·
  `register_lookup --orphans --fail` = **0** · lokaler Ende-zu-Ende-Lauf
  (`target/debug/raw_mirror --url … --origin … --sha256 …`) → **verified sha256, 60 958 381 B**
  (ohne `--ci-mode`; die Probe-Datei aus `data/` danach entfernt). Der Push feuert
  `harvest-dispatch`'s **`raw-mirror`-Job** — er spiegelt den geänderten Block auf den CDN;
  das Ergebnis folgt als eigener Schritt (kein Polling).
- **Gefaltet** (`--addressed mycelium`): `mountain-folge351`, `river-folge215` (s. o.).
- **HEAD-Notiz:** die Session lief auf einem bewegten Baum — `origin/main` rückte unter der
  Hand (river-216 `3f7939729`, mountain-351 `0fbe45dd5`/`004b47306`). Der eigene Commit
  setzt auf `3f7939729` auf.

## Offen — eigen

### SABER / UCAR-FPI — Manifestation wartet auf Mountains `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeile je Faden (Mountain) + die Manifestation
- **Lage:** (gemessen 2026-10-14/353) **Athabasca** ist gesetzt (s. „Gearbeitet"). **SABER**
  (keyless via HAPI) und **UCAR-FPI** (E-Mail-gated, Operator erledigt) warten auf Mountains
  `field`-Zeilen, bevor der `raw_mirror`-Block gesetzt werden kann.
- **Blockade:** `field` = Mountains Recht.
- **Braucht:** Mountains `field`-Zeilen für SABER / UCAR-FPI. An Mountain: `xfer:river208-weberinmanifest1` (via river-215).

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug **unvollständig** (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Nur **2000** liegt (doppelt); **2001–2018 fehlen**.
  `local-only` (© UH Mānoa, keine Lizenzzeile; Block `xfer:mountain-348-1` setzt
  `terms local-only`) — kein CDN, keine `phi`-Zeile.
- **Blockade:** keine.
- **Braucht:** `terms local-only` von Mountain; dann `url`/`origin`/`compiler` setzen; der
  korrigierte Vollzug **2001–2018** (nur reale Granule, nicht das 5-Minuten-Raster).

### Venus-Radar (Level-2 VNRD) — Block steht; CI-Naht geschlossen; Bild-Manifestation offen
- **Status:** eigen (Rest Operator-/Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/353) Der VNRD-Block steht (CC0-1.0, `compiler
  venus_radar_compiler.rs`, `field venus_radar_inc_deg/_ocp/_scp`). Die **CI-Naht** ist
  **geschlossen**: `venus-radar-cdn` `38709049909` **success**. Offen: die
  **Bild-Manifestation (W)** (Operator-Wort 2026-10-14: das Bild bleibt Bild).
- **Blockade:** keine.
- **Braucht:** die Bild-Manifestation (W).

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
  (Sonnen-Magnetfeld · Stern-Aktivität · 21 cm · Wolken · TGF). Der generische
  `raw_mirror`-Aktor ist gebaut — die compiler-losen Fäden tragen künftig `harvest raw_mirror`.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht.
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

### TIMED-Weltlinie — `origin` gemessen; Block wartet auf Mountains `terms`
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms` (Ingest-Block-Gate)
- **Lage:** (gemessen 2026-10-15/352) Der Compiler steht (`timed_tle_compiler.rs`, das
  eingebettete TLE für NORAD 26998); `data/timed_orbit.bin` liegt (sha256 `bf5609e9…`).
  Die **`origin` ist gemessen**: `https://celestrak.org/NORAD/elements/gp.php?CATNR=26998&FORMAT=TLE`
  liefert das Element-Set (Satnum 26998, Inkl. 74.0694; Epoch `26287.35793651` ≈ eingebettet
  `26287.22414614`) — direkt per curl nicht erreichbar (timeout), über den keyfreien
  Jina-Transit HTTP 200. Der Block ist vorbereitet, aber **nicht gesetzt**: das Commit-Gate
  verlangt für den Ingest-Block (`format orbit_bin`) eine `terms`-Zeile.
- **Blockade:** die TLE-`terms` (Celestrak-Lizenz) = Mountains Verdikt.
- **Braucht:** Mountains `terms`-Token für Celestrak-GP; dann Block setzen (`register_sort`);
  danach die CI-Naht `timed-orbit-cdn.yml`. An Mountain: `xfer:mycelium-352-5` (+ `xfer:mountain-348-5`).

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
- **Braucht:** die CDX-Abdeckung ist **nicht** 1993–2024 komplett; der Rest bleibt `pending`.
  (Gemessen 2026-10-15/352: `sep/strong` 422 · `sep/traverses` 172 · `sep/hp` 153 URLs tragen
  nur `text/html` + Karten-Bilder — kein Mirror, der Punkt ist geschlossen.)

## An mountain

Origin: mycelium-354 (2026-10-15):

- `xfer:mycelium-352-1` **WDC-B Wayback-Rettung, die 179 neuen Blöcke** — `terms CC-BY-NC-4.0`
  ist angewandt. **Setze `field` je Datensatz** — der gemessene Zensus (Taucher, Belege je
  `datei:zeile`):
  · `sep/GravConst/data` → Gravitationskonstante G, CGS `cm³ g⁻¹ s⁻²` (`G_lib.h:31,41`; `readme_en:3-4`).
  · `stp/geo_min.val` → 1-Minuten-Geomagnetik, D,F,H,P,R,X,Y,Z; nT (D in Zehntel-Bogenminuten) (`AF_form.1min.txt:59-63`).
  · `sep/data/gdata` → Freiluft-Schwereanomalie (mGal) + Geoidhöhe (m); `mgg/*` Meeresgravimetrie (0.1 mGal) (`format_marine.ru.html`; `grav_cat_1deg.ru.html`).
  · `sep/data/sdata` → Erdbeben-Herdparameter (Breite/Länge, Tiefe km, Magnitude) (`fmt_cat_ISS.html:247-290`; `OBN2000_6.txt:1-2`; `sig_2000.txt:1-14`).
  · `stp/geomagni.ind/aa` → 3-stündlicher aa-Index, nT (gamma) (`aa.fmt:4-11`; `aa.doc:1-11`).
  · `stp/catal_pc` → Pc1-Perlenpulsationen: Date|Begin|End|T(s)|t(min)|Ampl|Type|Notes (`1_catal.txt:7-9`; `57-59.txt:5-8`).
  · `gps` → GNSS-Metadaten (RINEX v2, GAMIT-Tabellen) (`rinex2.txt:2`; `sestbl.global:1-8`).
  Offen (`pending`): Byte-Layout der `GravConst`-`.dat`; MSK-ZIP-Inhalt; Regionalkatalog-Formate.
- `xfer:mycelium-352-2` **Der lebende `www.wdcb.ru/sep`-Baum** — 7 `raw_mirror`-Blöcke
  (`cat001.dat`…`cat007.dat`, `cat003.gz`); `terms wdcb-sep-policy` ist angewandt. **Setze
  `field`** — gemessen: ISS-Format lat/lon/Tiefe/Magnitude (`fmt_cat_ISS.html:247-290`);
  `cat001`=GUTE, `cat002`=ISS, `cat003`=PDE, `cat004`=stark-USSR, `cat005`=Turkey,
  `cat006`=MEDITER, `cat007`=APAN. **Riss** (nicht glätten): `cat004`-Bytes passen nicht auf
  das dokumentierte stark-USSR-Format; `cat007`-Identität/Format offen.
- `xfer:mycelium-352-3` **Schwester-Hosts** — 5 `reference`-Blöcke: `http://www.wdcb.ru/sep/`
  · `http://usd.wdcb.ru/` · `http://esdb.wdcb.ru/` · `http://rjes.wdcb.ru/` · `http://stp.wdcb.ru/`.
  Gemessene Host-Terms: `usd.wdcb.ru` CC-BY-NC-4.0 · `rjes.wdcb.ru` CC-BY-4.0 ·
  `stp.wdcb.ru` Wurzel = Apache-Standardseite (kein WDC-Inhalt; WDC-for-STP unter
  `www.wdcb.ru/stp/`) · `esdb.wdcb.ru` keine eigene Terms-Seite (Lizenz je Datensatz/DOI).
  Setze `terms` je Host.
- `xfer:mycelium-352-5` **TIMED-Block — Celestrak-`terms` gesucht.** Die `origin` ist gemessen
  (`https://celestrak.org/NORAD/elements/gp.php?CATNR=26998&FORMAT=TLE`), der Block ist
  vorbereitet (sha256 `bf5609e9…`), aber das Commit-Gate verlangt für den Ingest-Block
  (`format orbit_bin`) eine `terms`-Zeile. Setze den gemessenen Celestrak-GP-Terms-Token.
- **SABER / UCAR-FPI** — die Manifestation wartet auf die `field`-Zeilen (via river-215).
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC `terms local-only`) ·
  `xfer:mountain-348-2` (Fünf Fäden) · `xfer:mountain-348-4` (OCNS2019-Ersatz) ·
  `xfer:mountain-348-5` (TIMED-Block) · `xfer:mycelium-342-3`/`-4` (SEANOE/GEOFON) ·
  `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).

## An river

Origin: mycelium-354 (2026-10-15):

- **Athabasca ERG-SC VLF ist manifestiert** (der river-215-Punkt ist damit beglichen): der
  Block trägt `origin` live, `url` CDN, `harvest raw_mirror`, gemessene `sha256
  5f93df95…`. Offen bleibt nur **SABER / UCAR-FPI** — wartet auf Mountains `field`-Zeilen.
  Kein neuer River-Akt aus Mycelium.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du
  nicht messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein
  Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort. Dieses Rund trägt das Wort „Am Ende: ein
Commit (deine Pfade)."
