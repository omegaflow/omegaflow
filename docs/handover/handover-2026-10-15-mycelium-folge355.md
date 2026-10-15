<!--
  title: Handover — Mycelium-Folge 355 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 63a1a3d603074f5bb72868e69d64f09f59f523f9aff16d504f2979ca198315fb
  status: live
-->
# Handover — Mycelium-Folge 355 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge354.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge351` (die IGETS/GEOFON-Echos;
WDC-B-Verdikt angewandt) · `river-folge215` (CTBTO-Formular = Operator-Hand; die
Weberin-Zeugen-Manifestation — Athabasca gesetzt, SABER/UCAR-FPI weiter Mountain-gebunden).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` = 1 (NSSDC/CLPDS-Datenantrag, operator-gebunden —
bereits gewortet, **nicht** erneut vorgelegt) · `--stale mycelium --persist 3` = 0 ·
`--addressed mycelium` = 2 (mountain-folge351, river-folge215) · `open_points_check folge354`
= 0 stale-citations. Grund der Runde: die **Frontier-Runde zur Netz-Fusion** — das Formular
vorbereiten, die 13 Stimmen des Operators am Baum verifizieren, das Verdikt tragen.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „bitte schaue dir an was river gerade angelegt hat … bitte erstelle ein formular ich hole dann die stimmen" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge355) |
| „stelle die frage so wie river sie gestellt hat aber auf dein thema" | 2026-10-15 | dito |
| „nein verstehst du nicht ich schicke die frage an die ui seats du bereitest die frage … und das formular vor" | 2026-10-15 | dito |
| „eingetragen und gespeichert" | 2026-10-15 | dito |
| „ich habe kene ahnung es muss doch einen logischen nächsten schritt geben" | 2026-10-15 | dito |
| „ich habe es an river gegeben" | 2026-10-15 | dito |
| (standing) „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito (folge352) |
| (standing) „ab jetzt Reports immer in einfache Sprache" | 2026-10-13 | dito |

## Gearbeitet (dieses Rund)

- **Frontier-Runde „Netz-Fusion und Stationsplatzierung" vorbereitet + verifiziert.** Das
  Formular (`state/stimmen/2026-10-15-netzfusion-formular.md`) trägt die Frage (fünf Axiome +
  fünf Stimmen) und die 13 Sitz-Slots; der Operator trug die Stimmen ein (`eingetragen und
  gespeichert`) — 01–12 antworteten, **06 Mistral** = `Kein Kontingent` (`blocked: quota`),
  die geteilten Open-Weight-Sitze leer (`pending`).
- **Am Baum verifiziert (die Session prüft die Stimme, nie umgekehrt;**
  `state/stimmen/2026-10-15-netzfusion-verdikt.md`):
  · Die Fusion ist die **untere Einhüllende (Min)** je Zeitstempel (`al_network_proxy.rs:101-118`) →
  ein Schwellenübertritt ist eine **Konjunktion** (alle Stationen ≥ θ); der monotone Rückruf-Fall
  ist strukturell erzwungen.
  · Adaptive Schwelle `med + 6·1.4826·MAD` (`gic_onset_detector.rs:7-9/:80-86`); `--threshold`,
  `--sweep-k`, `--target-yield`; Toleranz = Matching-Fenster `match_onsets` (`:178-198`).
  · **Fester-Schwelle-N-Sweep** (river-216, Lauf `38709029080`): fixe Schwelle 17.11, konstantes
  Fenster → Rückruf n3/4 0.098 → n12 0.0035 monoton; der Kollaps ist **nicht** reine
  Schwellen-Drift. `argmax_N S` = 3–4; Einzelstation `S = 0.0213`.
  · **Baum-Riss (neu, A ≠ A):** das Netz fusioniert die **dH**-Hülle (Minutenraster), der
  Einzelstations-Arm ist der **stündliche |dB/dt|-Peak** (`abk_dbdt_1h.bin`; Compiler
  `intermagnet_dbdt_compiler.rs:179-190`, `_1h` → `bucket_s=3600`). Der zentrale Vergleich
  („Einzelstation schärfer als Netz") misst damit **nicht dieselbe Größe** — er ist `pending`.
- **Das Verdikt der Runde:** Min ist geschlagen (Struktur **und** Messung); Ersatz (Union/Max,
  k-von-n, TE-Auswahl) ist Hypothese, ungemessen; Platzierung: Anker höchste geom. Breite,
  Erweiterung N/W/O, Süden Kontrolle; Schärfe: `S(τ*,N)=(R−R₀)/(1−R₀)` am fixen Vorlaufpunkt,
  Fenster fixiert.
- **Der nächste begrenzte Messschritt — an River übergeben** (Operator-Wort „ich habe es an river
  gegeben"): beide Arme auf **dieselbe Größe auf demselben Raster** (per-Minute-dB/dt), dann bei
  **fester** Schwelle und **fixem** Fenster nur die Fusionsregel variieren — **Min gegen
  Union/Max** über N = 1·3·6·12·31. Lesart vorab fixiert: steigt Union → mehr Stationen heben;
  sonst steht die Einzelstation.

## Offen — eigen

### SABER / UCAR-FPI — Manifestation wartet auf Mountains `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeile je Faden (Mountain) + die Manifestation
- **Lage:** (gemessen 2026-10-14/353) **Athabasca** ist gesetzt (Commit `0281ae1a4`). **SABER**
  (keyless via HAPI) und **UCAR-FPI** (E-Mail-gated, Operator erledigt) warten auf Mountains
  `field`-Zeilen, bevor der `raw_mirror`-Block gesetzt werden kann.
- **Blockade:** `field` = Mountains Recht.
- **Braucht:** Mountains `field`-Zeilen für SABER / UCAR-FPI. An Mountain: `xfer:river208-weberinmanifest1`.

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug unvollständig (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Nur **2000** liegt (doppelt); **2001–2018 fehlen**.
  `local-only` (© UH Mānoa, keine Lizenzzeile) — kein CDN, keine `phi`-Zeile.
- **Blockade:** keine.
- **Braucht:** `terms local-only` von Mountain; dann `url`/`origin`/`compiler` setzen; der
  korrigierte Vollzug **2001–2018** (nur reale Granule, nicht das 5-Minuten-Raster).

### Venus-Radar (Level-2 VNRD) — Block steht; CI-Naht grün; Bild-Manifestation offen
- **Status:** eigen (Rest Operator-/Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-15/353) VNRD-Block steht (CC0-1.0, `compiler venus_radar_compiler.rs`,
  `field venus_radar_inc_deg/_ocp/_scp`); CI-Naht `venus-radar-cdn` `38709049909` **success**.
- **Blockade:** keine.
- **Braucht:** die **Bild-Manifestation (W)** (Operator-Wort 2026-10-14: das Bild bleibt Bild).

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) Offen: **SEANOE** `…/data/00311/42182/` (206; Lizenz
  `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung. An Mountain: `xfer:mycelium-342-3` · `xfer:mycelium-342-4`.

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-14/348) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`;
  der generische `raw_mirror`-Aktor ist gebaut.
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
  `xfer:mycelium-344-5` (LOD) · `xfer:mycelium-343-3` (BASE) · `xfer:mycelium-342-10`/`-343-2` (WDC/terms).

### TIMED-Weltlinie — `origin` gemessen; Block wartet auf Mountains `terms`
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms` (Ingest-Block-Gate)
- **Lage:** (gemessen 2026-10-15/352) Der Compiler steht (`timed_tle_compiler.rs`); die **`origin`
  ist gemessen** (`celestrak.org/NORAD/elements/gp.php?CATNR=26998&FORMAT=TLE`, per Jina-Transit
  HTTP 200). Der Block ist vorbereitet, aber **nicht gesetzt** — das Commit-Gate verlangt für den
  Ingest-Block (`format orbit_bin`) eine `terms`-Zeile.
- **Blockade:** die TLE-`terms` (Celestrak-Lizenz) = Mountains Verdikt.
- **Braucht:** Mountains `terms`-Token für Celestrak-GP; dann Block setzen; danach die CI-Naht
  `timed-orbit-cdn.yml`. An Mountain: `xfer:mycelium-352-5` (+ `xfer:mountain-348-5`).

### NSSDC/CLPDS-Planetologie — Route entschieden (Antrag operator-gebunden)
- **Status:** eigen (Antrag operator-gebunden) | **Bindung:** eigen
- **Trigger:** Operator-Wort „auf jedem fall mit antrag" (2026-10-14) — Route (b); der Send bleibt
  Operator-Hand
- **Lage:** (gemessen 2026-10-14/347) Antragsentwurf liegt bei Future. **Riss:** Katalog-Record
  `CC BY 4.0` vs. CLPDS-„all rights reserved". Offene Mirrors gemessen.
- **Blockade:** Antrag = Operator-Hand; Lizenz-Riss ungeklärt.
- **Braucht:** Operator sendet den CN-NSSDC-Antrag; die `完全共享`-Sätze sind anonym ladbar.
  An future: `xfer:mycelium-347-2`.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen
- **Trigger:** kein — Autofill-Eintrag entfernen
- **Lage:** (gemessen 2026-10-14/347) Der Login ist rotiert → der exponierte Wert ist entwertet.
- **Blockade:** Operator.
- **Braucht:** nur noch den Autofill-Eintrag im Browser entfernen.

### WDC-B Obninsk — Rettung erweitert; Abdeckung weiter unvollständig (eigen)
- **Status:** eigen (Rest = Vollständigkeit + Verdikte) | **Bindung:** eigen
- **Trigger:** kein — Beobachtung neuer/anderer Archive
- **Lage:** (gemessen 2026-10-15/352) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge;
  `terms` angewandt (Mountain `xfer:mountain-349-1`). Offen: `field` je Datensatz, die `terms` der
  5 Host-`reference`-Blöcke = Mountain (`xfer:mycelium-352-1`/`-2`/`-3`).
- **Blockade:** keine.
- **Braucht:** die CDX-Abdeckung ist **nicht** 1993–2024 komplett; der Rest bleibt `pending`.

## An mountain

Origin: mycelium-355 (2026-10-15):

- `xfer:mycelium-352-1` **WDC-B Wayback-Rettung, die 179 neuen Blöcke** — `terms CC-BY-NC-4.0`
  angewandt; **`field` je Datensatz setzen** (der gemessene Zensus in folge354, Belege je
  `datei:zeile`).
- `xfer:mycelium-352-2` **Der lebende `www.wdcb.ru/sep`-Baum** — 7 `raw_mirror`-Blöcke;
  `terms wdcb-sep-policy` angewandt; **`field` setzen** (cat001…cat007). Riss: `cat004`-Bytes
  passen nicht auf das stark-USSR-Format; `cat007` offen.
- `xfer:mycelium-352-3` **Schwester-Hosts** — 5 `reference`-Blöcke; **`terms` je Host setzen**.
- `xfer:mycelium-352-5` **TIMED-Block — Celestrak-`terms` gesucht.**
- **SABER / UCAR-FPI** — die Manifestation wartet auf die `field`-Zeilen.
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC `terms local-only`) ·
  `xfer:mountain-348-2` (Fünf Fäden) · `xfer:mountain-348-4` (OCNS2019-Ersatz) ·
  `xfer:mountain-348-5` (TIMED-Block) · `xfer:mycelium-342-3`/`-4` (SEANOE/GEOFON) ·
  `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).

## An river

Origin: mycelium-355 (2026-10-15):

- **Der nächste begrenzte Messschritt der Netz-Fusion ist übergeben** (Operator-Wort „ich habe es
  an river gegeben"): bei fester Größe (per-Minute-dB/dt statt dH), **fester** Schwelle und
  **fixem** Fenster nur die Fusionsregel variieren — **Min gegen Union/Max** über N = 1·3·6·12·31,
  dieselbe Onset-Liste, dieselbe Kreisschub-Null. Der Baum-Riss (Netz dH-Hülle vs. stündlicher
  |dB/dt|-Peak) ist der Grund, warum zuerst dieselbe Größe her muss. Verdikt:
  `state/stimmen/2026-10-15-netzfusion-verdikt.md`. Kein neuer River-Akt aus Mycelium darüber hinaus.
- **Athabasca ERG-SC VLF ist manifestiert** (river-215 beglichen); offen bleibt nur **SABER /
  UCAR-FPI** (Mountains `field`-Zeilen).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Runds: `state/stimmen/2026-10-15-netzfusion-formular.md`,
`state/stimmen/2026-10-15-netzfusion-verdikt.md`, diese Übergabe, die Operator-Gespräche-Cuts.
