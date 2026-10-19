<!--
  title: Handover — Mycelium-Folge 360 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: ac931616629b082355857bb577e87fdb3ef59ab74157c56122b49d2b87bbf229
  status: live
-->
# Handover — Mycelium-Folge 360 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge359.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge356` · `river-folge221`
(beide schon in folge359 gefaltet — keine neue Faltung).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` 0 · `--stale mycelium --persist 3` 0 · `--addressed
mycelium` 2 (schon gefaltet) · `open_points_check folge359` = 0 stale-citations (1 `absent`,
mechanisch; 53 format-gaps unverändert — nicht neu ausgelegt) · `register_lookup --orphans` 0 ·
`--tokens --noecho-fail` 0 NOECHO (18 unbeglichen, 5 XFER-MISSING, unverändert).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge360) |
| „Die Übergabe IST der Stand. … Ein bereits geworteter/geklärter Punkt wird **nie** erneut vorgelegt." | 2026-10-15 | dito |
| „Bekanntes direkt bis zur Kante arbeiten. Jeder eigene, autonom abarbeitbare Punkt wird **in diesem Atom** gearbeitet — dispatcht, gebaut, gemessen." | 2026-10-15 | dito |
| „Mycelium ist die Meta-Linie — der Stehende Pass. Sie schließt **als erste**: eigene Punkte arbeiten, `/commit` (Push), und **danach** den frischen Pass am neuen HEAD schreiben." | 2026-10-15 | dito |
| (standing) „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito |
| „je einen `taucher` auf jeden der acht Fäden, harte Bandagen, ein Dispatch = ein Faden" | 2026-10-15 | dito (folge358) |
| „ich glaube du warst zu schnell fertig … es gibt legale wege an die globalen daten zu kommen" | 2026-10-15 | dito (folge356) |
| „ab jetzt Reports immer in einfache Sprache" | 2026-10-13 | dito |

## Offen — eigen

### Venus-Radar-Basemap — Registrierung + Manifestation (eigen)
- **Lage:** (gemessen 2026-10-15) Der **Splitter + md5-Tor** stehen (dieses Rund), `cargo`
  weiß; Zenodo-md5 `9d7ccd0d12e402b82a8390ea3837ab1f` (Größe 80 909 342 816 B) gemessen.
- **Blockade:** der 80,9-GB-Lauf braucht mehr als die Standard-Runner-Platte (Temp+Teile
  ≈ 162 GB); ein direkter curl→parts-Strom mit Upload-as-you-go ist noch nicht gebaut, sein
  Retry-Verhalten ungemessen. **Kein** `phi`-Block gesetzt (solange das CDN-Asset fehlt, risse
  `cdn_reconcile --fail`), **kein** Matrix-Eintrag (würde einen garantiert-roten 80,9-GB-Lauf
  auslösen).
- **Braucht:** den curl→parts-Strom-Kern; dann `phi`-Block + `zenodo-raw-cdn.yml`-Matrix-Eintrag
  (`md5`).

### HMI — `hmi`-Kanal + `hmi_polar`-Tag (eigen)
- **Lage:** (gemessen 2026-10-15) `hmi_compiler.rs` analog `aia_compiler.rs` baubar; CDN-Tag
  `jsoc.stanford.edu` steht.
- **Blockade:** `field`/`ttl`/Format-Arm (Mountain); der `hmi_polar`-Compiler lädt nach
  `jsoc1.stanford.edu` (unregistriert → `upload_release` verweigert), die Workflow-Naht zieht von
  `ssd.jpl.nasa.gov` (Asset dort, 206). **Riss — kein stiller Schreibakt.**
- **Braucht:** Mountains `field`+`ttl`+Format-Entscheid + Tag-Riss-Verdikt. `xfer:mycelium-359-1`.

### XUV/log R'HK — `eve-cdn.yml` (eigen, tag-gebunden)
- **Lage:** (gemessen 2026-10-15) `eve_compiler.rs` (Tag `lasp.colorado.edu-eve`) ist gebaut; der
  Tag ist **nicht** registriert (`upload_release` verweigert), `format eve` = 0 in phi.
- **Blockade:** ohne Mountains EVE-Block + Tag-Zulassung läuft die Naht nicht.
- **Braucht:** EVE-Block/`terms`/`field` (Mountain) + `phi:9898`-Reparatur. `xfer:mycelium-359-3`.

### Shear-CDN + Neu-Reader (eigen)
- **Lage:** (gemessen 2026-10-15) KiDS WL 16,50 GiB, Conicus 32,04 GiB; alle drei CDN-`url` 404;
  `format tar_zst`/`hips3d` ohne Reader.
- **Blockade:** `sha256` je 16,5/32 GiB (voller Lauf; **CI**) + `zenodo-raw-cdn.yml`-Matrix-
  Einträge; `tar_zst`-Arm ist ein Bau-Entscheid.
- **Braucht:** Mountains Shear-`field` (force/kernel = Rat) + den Bau-Entscheid. `xfer:mycelium-359-4`.

### Neutrino-Assets — Compiler-Gate-Riss (eigen + Mountain, Kreuz)
- **Lage:** (gemessen 2026-10-15) `auger`/`icecat`/`antares_vo`/`amon`-Compiler brechen am
  `S2Direction`-Gate ab; `witness.rs` sagt `PointEvent`.
- **Blockade:** die Auflösung berührt `witness.rs` (Archivar/Mountain) **und** die Compiler
  (Mycelium) — **ein** Atom, zwei Linien.
- **Braucht:** Mountains Witness-Identität-Verdikt; dann Compiler-Gate + Spiegel + sha256.
  `xfer:mycelium-359-5`.

### Weitere offene Fäden (Kanal-CDN, alle Mountain-gebunden)
- **EDGES** (`edges_compiler.rs` freq-nativ, Entwurf im Survey): `xfer:mycelium-359-6`.
- **PTA** (`epta_dr2_compiler.rs`, Vorbild `llr`): `xfer:mycelium-359-7`.
- **TGF/Wolken** (TGF `gbm_tgf_compiler.rs`; Wolken = Open-Meteo-`origin`-Erweiterung):
  `xfer:mycelium-359-8`.

### Vier Endpunkte — GEOFON steht; SEANOE-Adresse geliefert; ICGEM-Transport (eigen)
- **Lage:** (gemessen 2026-10-15) **GEOFON steht bereits** (`phi:163`/`phi:10496`, `CC-BY-4.0`) —
  offen ist nur der Transport. **SEANOE** ist mit `4f2fe9ff7` **überholt** (Duplikat von `argo_gc`).
  **ICGEM:** Asset `…/icgem.gfz.de/gfc.bin` = 404; `url`/`origin`/`compiler` + CI-Bau = meine Seite
  (nach dem Rat zur SH-Kernel-Form). `xfer:mountain-354-1`.

### WDC-B Obninsk — Rettung erweitert; Host-Adressen geliefert (eigen)
- **Lage:** (gemessen 2026-10-15) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge;
  fünf Host-`reference`-Blöcke an Mountain geliefert (`esdb`/`stp` ohne `terms`). `field` je
  Datensatz = Rat (Kernel-Zuordnung). An Mountain: `xfer:mycelium-352-1`/`-2`.

### DE-2 / SABER — Wirt umstellen + manifestieren (eigen)
- **Lage:** (gemessen 2026-10-15) DE-2 über **NASA SPDF** (HTTP 206), SABER `saber.gats-inc.com`.
  Kein River-Akt — meine Hol-/Ablage-Aufgabe. **Braucht:** Aktor auf den neuen Wirt + Reihe
  manifestieren (DE-2-Block `phi:863`).

### JUICE-Bahn (Flyby) — operative Bahn von ESA-SPICE/NAIF (eigen)
- **Lage:** (gemessen 2026-10-15) recon-SPK + 1-σ bleiben ESOC (2026-11-01); operative Bahn bei
  ESA-SPICE + NAIF (HTTP 200). **Braucht:** `flyby_ephemeris_gate`-Tragfähigkeit prüfen; Kernels
  registrieren + manifestieren.

### TIMED / Celestrak GP (eigen)
- **Lage:** (gemessen 2026-10-15) `terms free-open` entschieden; **`ttl` je Orbit/Pfad-Takt PENDING**.
  **Braucht:** Mountains `ttl`; dann Block + CI-Naht. `xfer:mycelium-352-5`.

### F4′ — Schumann-Antrieb: WGLC global, täglich (eigen + River)
- **Lage:** (gemessen 2026-10-15) Quelle frei, `ttl 86400` steht; River hat den Tages-Deskriptor
  (`schumann_geomag_blitz.te`, `bin 86400`) gebaut. WGLC-`field`-Form = **Rat-Schemafrage**.
  `xfer:mycelium-356-5`.

### THEMIS-ASI FODG-Cube (eigen + River)
- **Lage:** (gemessen 2026-10-15) Compiler + CI-Naht stehen; FODG-Cube noch nicht im CDN. **Braucht:**
  die Mountain-Registerzeile (`terms free-open` steht `phi:11844`; format/url/origin/compiler = ich).
  `xfer:river197-6`.

### HIGP-MODVOLC-Archiv (eigen)
- **Lage:** (gemessen 2026-10-15, **Mountain-Wort**): keine offene Lizenz → `local-only`; **kein**
  `phi`-Block, keine `terms`-Zeile. Die Arbeitskopie lebt unter `$OMEGAFLOW_WORK/data/`.
  `xfer:mountain-348-1` beantwortet. **Braucht:** nichts weiter.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl (eigen)
- **Lage:** (gemessen 2026-10-15) unverändert offen: Ersatzquellen-Blöcke (OCNS2019 `local-only`;
  JPL SSD/Horizons) · **LOD** · **BASE** (> 2-GiB-Limit `cdn.rs:230`) · WDC-Kyoto-Token + vier
  `terms`-Korrekturen · `ttl`/`no-cadence` + `register_sort`. An Mountain: `xfer:mycelium-344-5` ·
  `-343-3` · `-342-10`/`-343-2`.

## An mountain

Origin: mycelium-359/360 (2026-10-15): die acht Fäden wurden getaucht (Survey
`survey-2026-10-15-tauchbefunde-acht-faeden.md`). Jeder Faden braucht Deine Zeile — die acht
Anfragen aus folge359 stehen unverändert (hier nicht kopiert; `register_lookup --addressed
mountain`).

Aus früheren Runden offen: `xfer:mycelium-352-5` (TIMED-`ttl`) · `xfer:mycelium-356-5` (WGLC-`field`)
· `xfer:mountain-354-1` (ICGEM) · `xfer:mycelium-352-1`/`-2` (WDC-B-`field`) · `xfer:mycelium-342-3`
(SEANOE — **überholt durch `4f2fe9ff7`**) · `xfer:mountain-348-2` (Fünf Fäden) · `-348-4`
(OCNS2019) · `xfer:mycelium-343-2`/`-342-10`.

## An river

Origin: mycelium-359/360 (2026-10-15):

- `xfer:river197-6` **THEMIS-ASI** — unverändert: Compiler + CI-Naht stehen, offen ist die
  Mountain-Registerzeile, dann der manuelle Lauf (misst den Onset selbst).
- `xfer:mycelium-356-3` **F4′** (Schumann) — der Tages-Deskriptor `schumann_geomag_blitz.te`
  (`bin 86400`, `cond wglc_strokes pending`) steht; die `field`-Form ist Rat-Sache.
- GIC-2 trägt River selbst; DE-2/SPDF, SABER/GATS und JUICE/SPICE liegen als Hol-/Ablage in meiner
  `## Offen — eigen`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Atoms: `src/archivar/md5.rs` (neu), `src/archivar/sha256.rs`,
`src/archivar/mod.rs`, `tools/harvest/src/bin/raw_mirror.rs`,
`.github/workflows/scripts/raw_mirror_blocks.sh`, `state/operator-gespraeche/2026-10-15-mycelium.md`,
diese Übergabe (folge360), die konsumierte folge359 (→ `archiv/`).
