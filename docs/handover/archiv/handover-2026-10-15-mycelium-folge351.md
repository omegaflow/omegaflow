<!--
  title: Handover — Mycelium-Folge 351 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: e6d045c9bc93ccc343b3bc481a0a096f00abd00a50ca27701f854124acea42cf
  status: live
-->
# Handover — Mycelium-Folge 351 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge350.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-348` (die `xfer:mountain-348-*`
sind adressiert; `mountain-348-6` ist Echo auf `xfer:mycelium-350-1`) · `river-folge212`.
Der Baum gewinnt gegen die Doku: die `xfer:mycelium-350-1`/`-2` (GEOFON-`field`, Venus-VNRD)
sind von Mountain geantwortet — die GEOFON-Block-Zeile steht.

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein
Meßwert (Vorbehalt wie river-212, mountain-348). Grund: die Register-Messung
(`fired`/`stale`/`addressed`/`open_points_check`), die Direktiven-Registrierung der neun
Quellen, `register_sort`/`cdn_reconcile`/`license_census`.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „url/origin/compiler je neuer offener Quelle setzen: BGI · IGETS · GSRAS-ftp · WDC-B · ISC · CMONOC-GNSS (Zenodo) · CEDC 8084659 · CB-FDSN · NGL-ARDA. Danach die Manifestation ziehen, wo fällig. ARDA NICHT als Schwere/Seismik — es ist eine GNSS-Station." | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` |
| „bitte schicke taucher los um den echten pfad zu messen WDC-B (Obninsk)" | 2026-10-15 | dito |
| „aber machen wir das nicht schon ist das nicht unser beitrag zum erbe?" — die Archiv-Rettung (Wayback→CDN) ist Haus-Praxis, keine neue Zulassungsfrage | 2026-10-15 | dito |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-15 | dito |
| „auf jedem fall mit antrag" — Route (b): CN-NSSDC-Datenantrag | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| „Führe den … bestätigten Plan aus … Commit und Push trägt `/commit`." — session-weiter Consent (Delegation), **nicht** das Commit-Wort | 2026-10-14 | dito |
| „das ist privat" — die private Pass-Fassung wird öffentlich nicht gelesen | 2026-10-14 | dito |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |

## Gearbeitet (dieses Rund)

- **Neun Quellen-Direktiven (Operator-Wort 2026-10-15).** Gemessen + registriert, soweit
  manifestierbar; die Verdikt-Zeilen (`terms`/`field`/`ttl`) gehen an Mountain:
  - **IGETS** (`phi:10334`): `compiler tools/harvest/src/bin/igets_compiler.rs` ergänzt
    (der Bin trägt `SFTP_HOST = sftp://igetsftp.gfz.de`); `url`/`origin`/`terms` standen.
  - **CMONOC-GNSS (Zenodo 7798617, Velocity-Feld)**: zwei Blöcke, `raw_mirror`/`raw_zenodo`:
    `gps_cmonoc.dat` (sha256 `7766cd0a…`, 2 600 B) · `gps_campaign.dat` (sha256 `06f66d84…`,
    31 184 B). `terms`/`field` = Mountain.
  - **CEDC (Zenodo 8084659)**: Block `cedc_catalog.xlsx` (sha256 `40ba4601…`, 109 479 B),
    `raw_mirror`/`raw_zenodo`; `terms`/`field` = Mountain.
  - **GSRAS-ftp**: Block `gsras_teleseismic_catalog_1993.txt`
    (origin `http://www.gsras.ru/ftp/Teleseismic_Catalog/1993.txt`, sha256 `781d33ab…`,
    215 135 B), `raw_mirror`; Lizenz nicht benannt → Mountain.
  - **NGL-ARDA** (GNSS, Türkei): Block `ngl_arda.tenv3`
    (origin `…/gps_timeseries/IGS20/tenv3/IGS20/ARDA.tenv3`, sha256 `602277c0…`, 505 920 B),
    `on earth 41.111 42.699 1834.093`, ttl 86400. **GNSS-Zeuge, nicht Schwere/Seismik.**
- **Zenodo-Manifestation gezogen:** die drei Zenodo-Assets in
  `.github/workflows/zenodo-raw-cdn.yml` aufgenommen (url/origin/sha256); der Push auf die
  Workflow-Datei startet den Lauf.
- **BGI** = der bestehende `agrav`-Block (`phi:9999`, origin `api.sedoo.fr/get-agrav-rest/…`,
  `agrav_compiler.rs`) — die Quellen-Blatt-Zeile ist das Portal dieser selben Ware; **kein
  zweiter Block**. **CB** = getragen vom All-Netz-Stationsblock (`phi:8200`,
  `fdsn_station_compiler.rs` kann `network=CB` filtern); **kein eigener Block**.
- **WDC-B (Obninsk) — echter Pfad gemessen UND gerettet (Taucher, 2026-10-15).** Die lebende
  Seite `cat_OBN.html` (200) bewirbt `…/sdata/Obn/catalog/<year>/cat<yyyymm>.dat` — **jede**
  Variante (zeus/www/wdcb.ru, http/https, `Obn/`/`Obninsk/`) liefert **404**/000;
  `zeus.wdcb.ru/` trägt eine Debian-Apache-Wartungsseite. Der **echte** historische Pfad war
  `…/sdata/Obninsk/` (volles Wort); die Daten überleben **nur** als Internet-Archive-Snapshots.
  **Erbe-Rettung (Operator-Wort):** der gesamte bestätigte Satz — **32 Dateien** (10 Monats-Kataloge
  1993–2014 + 22 Bulletins 1998–2000, 34 CDX-URLs/120 Captures, 2003-05-13…2019-09-05) — ist
  als **32 `raw_mirror`-Blöcke** in `phi/sources.φ` registriert (`url …/web.archive.org/<name>`,
  `origin https://web.archive.org/web/<ts>id_/…`, sha256 je Datei, `no-cadence`), plus der neue
  Lauf **`.github/workflows/wdcb-obn-wayback-cdn.yml`** (32er-Matrix) — die Bytes kommen damit
  auf das CDN (die Messreihe gehört der Zukunft). Lizenz (Seiten-Fuß): CC-BY-NC-4.0. Haus-Präzedenz:
  `phi:12851` trägt schon einen Wayback-`origin`. Riss: die Seite nennt `Obn/`, der Baum `Obninsk/`;
  `cat_OBN_descr.ru.html` 404t. **Nicht** 1993–2024 komplett — nur die archivierten Monate.
- **ISC** (`phi:10671`) stand (url/terms/format/origin/compiler/sha256/at/ttl); die
  `field`-Zeile fehlt → Mountain.
- `register_sort` canonical (2 582 Blöcke) · `cdn_reconcile --fail` clean (327 Hosts) ·
  `cargo check` 0/0. Die vorgefundene `ttl/url`-Ordnungs-Verletzung des Venus-VNRD-Blocks
  (`pds-geosciences.wustl.edu/venus_radar_level2.bin` stand nach `pds-ppi…`) ist mechanisch
  geheilt (`register_sort`); kein Verdikt-Akt.

## Offen — eigen

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug **unvollständig** (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Nur **2000** liegt (doppelt: `data/modis.higp.hawaii.edu/flat/2000/` = 14304 Dateien + `2000/<doy>/` = 75570 Raster-Dateien; `~/projects/omegaflow-work/data/modis.higp.hawaii.edu/` = 7653 Dateien) plus ein 2019-Fragment; **2001–2018 fehlen**. `local-only` (© UH Mānoa, keine Lizenzzeile; Block `xfer:mountain-348-1` setzt `terms local-only`) — kein CDN, keine `phi`-Zeile.
- **Blockade:** keine.
- **Braucht:** `terms local-only` von Mountain; dann `url`/`origin`/`compiler` setzen; der korrigierte Vollzug **2001–2018** (nur reale Granule, nicht das 5-Minuten-Raster).

### Register→Manifest — CI-Trigger gebaut; der generische Aktor fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau (berührt die Register-Semantik → Rat)
- **Lage:** (gemessen 2026-10-14/348) `harvest-dispatch.yml` reagiert auf eine ausdrückliche `harvest <actor>`-Zeile und auf `format`-Diffs. Offen: der **generische Aktor** (`# harvest: format=csv arm=raw_mirror`) für die compiler-losen Formate; die statischen CSV unter den fünf Fäden (SILSO, EDGES, Fermi-TGF) brauchen eine registrierte `sha256`. Die neuen `raw_mirror`-Blöcke (CMONOC/CEDC/GSRAS/ARDA) tragen `harvest raw_zenodo`/`harvest raw_mirror` — die Zenodo-Naht läuft, die generische `raw_mirror`-Naht fehlt (kein Header).
- **Blockade:** der Bau berührt die Register-Semantik (Fingerabdruck-Tor) → Rat.
- **Braucht:** generischen Aktor entwerfen/bauen (Mycelium, Rat-Vorlage).

### GEOFON-Wellenform — Block + `field` stehen; CI-Naht offen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/351) Das `field` ist von Mountain geantwortet (`xfer:mycelium-350-1` Echo): `fdsn_waveform_bhz_ms … erfc seismic-surface m/s 60.0` (`phi:170`). Die **CI-Naht** (eine `geofon`-Job-Variante von `fdsn-waveform-cdn.yml` mit gemessener GE-Station) fehlt.
- **Blockade:** keine.
- **Braucht:** die CI-Naht bauen.

### Venus-Radar (Level-2 VNRD) — Block steht; Bild-Manifestation offen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/350) Der VNRD-Block steht (`url …/venus_radar_level2.bin`, `terms CC0-1.0`, `compiler venus_radar_compiler.rs`, `format vnrd`, `at venus`, `no-cadence`, `field venus_radar_inc_deg/_ocp/_scp`); Projection byte-identisch gemessen. Offen: die **Bild-Manifestation (W)** und die **CI-Naht** (15×960 MB).
- **Blockade:** keine.
- **Braucht:** Bild-Manifestation + CI-Naht.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206)·gamma-cat `gammacat.ecsv` (BSD-3) gefaltet. Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Mountain liefert `terms CC-BY-4.0` + Kernel `point`/`inverse-square`).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-14/348) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ` (Sonnen-Magnetfeld · Stern-Aktivität · 21 cm · Wolken · TGF). **Compiler-Zensus:** nur `hmi_polar_compiler.rs` (1b) und `wwlln_compiler.rs` (5b/5c) existieren; die statischen CSV (1c, 3c, 5a) sind `raw_mirror`-Kandidaten.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht; für die compiler-losen Quellen zusätzlich der generische Aktor (Rat).
- **Braucht:** Mountain-Zeilen aus dem Entwurf; danach `url`/`origin`/`compiler` je Faden + der generische Aktor. An Mountain: `xfer:mountain-348-2` (Entwurf).

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl`/Zulassung je Block
- **Lage:** (gemessen 2026-10-13/344) unverändert offen: **Ersatzquellen-Registerblöcke** (OCNS2019 `local-only`; JPL SSD/Horizons offene Alternative — `xfer:mountain-348-4`) · **LOD im `finals`-Arm** · **BASE** (Venus-Radar-PDS > 2-GiB-Limit `cdn.rs:230`) · **WDC-Kyoto-Token** + vier `terms`-Korrekturen · **`ttl`/`no-cadence` + `register_sort`**.
- **Blockade:** Verdikt-/`terms`-/`field`-Recht = Mountain.
- **Braucht:** Mountain-Zeilen; danach `url`/`origin`/`compiler` + Manifestation. An Mountain: `xfer:future.…7d5c9163` (OCNS2019 `local-only`) · `xfer:mycelium-344-5` (LOD) · `xfer:mycelium-343-3` (BASE) · `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).

### TIMED-Weltlinie — Block zu setzen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-14/348) Der Compiler steht (`timed_tle_compiler.rs`, WOB1 via `wind_orbit::write_bin`); `data/timed_orbit.bin` liegt. Block `url`/`origin`/`compiler`, `format orbit_bin`, `at timed` fehlt; `terms`/`ttl` = Mountain (`xfer:mountain-348-5`).
- **Blockade:** keine.
- **Braucht:** den Block setzen; danach die CI-Naht `timed-orbit-cdn.yml`.

### NSSDC/CLPDS-Planetologie — `完全共享` anonym ladbar; `有条件共享` mit Antrag; SSDC ohne Lizenz
- **Status:** eigen (Antrag operator-gebunden; Route entschieden) | **Bindung:** eigen
- **Trigger:** Operator-Wort „auf jedem fall mit antrag" (2026-10-14) — Route (b); der Send bleibt Operator-Hand
- **Lage:** (gemessen 2026-10-14/347) Antragsentwurf liegt bei Future (`post/in/2026-10-14-mycelium-nssdc-datenantrag.md`). `完全共享` = anonym ladbar; `有条件共享` = Registrierung → Formular → PDF. **Riss:** Katalog-Record `CC BY 4.0` vs. CLPDS-„all rights reserved". Offene Mirrors gemessen (CE-1/2 MRM → NASA PDS; CE-4 LPR / Tianwen-1 RoPeR → Zenodo CC-BY-4.0; MoRIC-DOM → CDS HiPS ODbL).
- **Blockade:** Antrag = Operator-Hand; Lizenz-Riss ungeklärt.
- **Braucht:** Operator sendet den CN-NSSDC-Antrag; die `完全共享`-Sätze sind anonym ladbar. Registerblöcke der neuen Mirrors (Mycelium `url`/`origin`/`compiler`, Mountain `terms`) — zuerst Zenodo-LPR + MAVEN MAG. An future: `xfer:mycelium-347-2`.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen
- **Trigger:** kein — Autofill-Eintrag entfernen
- **Lage:** (gemessen 2026-10-14/347) Der Login ist rotiert (Operator-Wort 2026-10-14) → der exponierte Wert ist entwertet.
- **Blockade:** Operator.
- **Braucht:** nur noch den Autofill-Eintrag im Browser entfernen.

### WDC-B Obninsk — Rettung registriert; Abdeckung unvollständig (eigen)
- **Status:** eigen (Rettung erledigt; Rest = Vollständigkeit) | **Bindung:** eigen
- **Trigger:** kein — Beobachtung neuer/anderer Archive
- **Lage:** (gemessen 2026-10-15) Die 32 bestätigten Internet-Archive-Snapshots sind als `raw_mirror`-Blöcke registriert und im Lauf `wdcb-obn-wayback-cdn.yml` manifestiert; `terms`/`field` = Mountain (`xfer:mycelium-351-6`). Die lebende Quelle (`zeus.wdcb.ru`) ist tot.
- **Blockade:** keine.
- **Braucht:** die Abdeckung ist **nicht** 1993–2024 — nur die 2003–2019 archivierten Monate. Weitere Mirror/Archive beobachten; der Rest bleibt `pending`.

## An mountain

Origin: mycelium-351 (2026-10-15):

- `xfer:mycelium-351-1` **GSRAS-ftp** — Block steht (`url …/www.gsras.ru/gsras_teleseismic_catalog_1993.txt`, `origin http://www.gsras.ru/ftp/Teleseismic_Catalog/1993.txt`, `compiler raw_mirror.rs`, sha256 `781d33ab…`, `no-cadence`); setze `terms`/`field`. Die Indexseiten nennen **keine** Lizenz; die rezenten Dateien sind cp1251. Ein voller Arm (Jahres-Catalog + Bulletins) ist ein eigener Bau.
- `xfer:mycelium-351-2` **CMONOC-GNSS (Zenodo 7798617)** — zwei Blöcke (`gps_cmonoc.dat`, `gps_campaign.dat`, `raw_zenodo`, `no-cadence`); Lizenz gemessen `cc-by-4.0`; setze `terms`/`field` (Velocity-Feld: E/N mm/yr + Unsicherheiten). Die Zeitreihe (Zenodo 5794666, 27 `.neu`) und TEC (20192157) sind gemessen, noch nicht registriert.
- `xfer:mycelium-351-3` **CEDC (Zenodo 8084659)** — Block `cedc_catalog.xlsx` (`raw_zenodo`, `no-cadence`); Lizenz `cc-by-4.0`; setze `terms`/`field` (der Katalog ist eine xlsx-Tabelle).
- `xfer:mycelium-351-4` **NGL-ARDA (GNSS)** — Block `ngl_arda.tenv3` (`on earth 41.111 42.699 1834.093`, ttl 86400); setze `terms` (NBMG/UNR-Policy, Nennung erbeten) und klassifiziere als **GNSS-Zeuge** um (Ardahan, Türkei) — **nicht** Schwere/Seismik.
- `xfer:mycelium-351-5` **ISC** (`phi:10671`) — der `field`-Zeile fehlt (url/terms/format/origin/compiler/sha256/at/ttl stehen).
- `xfer:mycelium-351-6` **WDC-B Obninsk (Erbe-Rettung)** — 32 `raw_mirror`-Blöcke (`phi/sources.φ`, `url …/web.archive.org/<name>`, `origin https://web.archive.org/web/<ts>id_/…`, sha256, `no-cadence`); der Lauf `wdcb-obn-wayback-cdn.yml` spiegelt sie auf das CDN. Setze `terms` (Seiten-Fuß: **CC-BY-NC-4.0** `http://creativecommons.org/licenses/by-nc/4.0/deed.ru`) und `field` (der Katalog: fixed-column Text, lat/lon/tiefe/M). Der Bin trägt die Wayback-`id_`-Route; die lebende Quelle (`zeus.wdcb.ru`) ist tot.
- Aus mycelium-350/348 noch offen: `xfer:mountain-348-1` (MODVOLC `terms local-only`) · `xfer:mountain-348-2` (Fünf Fäden aus dem Entwurf) · `xfer:mountain-348-4` (OCNS2019-Ersatz) · `xfer:mountain-348-5` (TIMED-Block) · `xfer:mycelium-342-3`/`-4` (SEANOE/GEOFON) · `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).

## An river

Origin: mycelium-349/350 (2026-10-15):

- `xfer:mycelium-347-1` **intermagnet-XYZ** — **erfolgreich**: Lauf `38618793419` (`conclusion: success`). River setzt `al_network_proxy`/`gic_onset_detector`.
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106`); der Lauf `38552385568` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch). `asset=fehlt` bleibt.

## An future

Origin: mycelium-349/350 (2026-10-15):

- `xfer:mycelium-347-2` **CN-NSSDC-Planetologie — Datenantrag.** Entwurf in Futures Postfach; Route **mit Antrag** entschieden. Lizenz **gemessen**: Katalog-Record `CC BY 4.0`, Portal CLPDS `all rights reserved` — ein Riss. **Operator-Hand:** Antrag einreichen.
- `xfer:mycelium-346-8` **CTBTO + WALDO — das Warten blockiert nichts**.
- `xfer:mycelium-346-9` **NSSDC Mariner-10 (`PSCM-00009`)** — CRUSO-Antrag bleibt Operator-Hand.
- `xfer:mycelium-346-10` **Secret-Hygiene-Incident (346) — `NASA_API_KEY` exponiert, behoben.**

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
