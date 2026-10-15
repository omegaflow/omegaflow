<!--
  title: Handover — Mycelium-Folge 356 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 4d047f757102b6564cb60418c65611581dcc86f8abd04d96a2d8e7ad9d778730
  status: live
-->
# Handover — Mycelium-Folge 356 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge355.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge351` (IGETS/GEOFON-Echos;
WDC-B-Verdikt angewandt) · `river-folge215` (CTBTO-Formular = Operator-Hand; Athabasca gesetzt).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` = 1 (NSSDC/CLPDS-Datenantrag, bereits gewortet —
**nicht** erneut vorgelegt) · `--stale mycelium --persist 3` = 0 · `--addressed mycelium` = 2 ·
`open_points_check folge355` = 0 stale-citations (33 format-gaps: Inline-`Braucht`/ungestempelte
`Lage` — mechanisch, nicht neu ausgelegt).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „bitte schaue dir an was river gerade angelegt hat … bitte erstelle ein formular ich hole dann die stimmen" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge355) |
| „nein verstehst du nicht ich schicke die frage an die ui seats du bereitest die frage … und das formular vor" | 2026-10-15 | dito |
| „eingetragen und gespeichert" | 2026-10-15 | dito |
| „ich habe kene ahnung es muss doch einen logischen nächsten schritt geben" | 2026-10-15 | dito |
| „ich habe es an river gegeben" | 2026-10-15 | dito |
| „das problem ist dass keiner außer dir das formular kennt" | 2026-10-15 | dito |
| „bitte prüfe die 5 stimmen und die 5 axiome das stichwort ist granit" | 2026-10-15 | dito |
| „die 2 letzten habt ihr irgendwann hinzugefügt für die fragebögen gelten die 5en" | 2026-10-15 | dito |
| „nein die 4 schilde plus mitte brauchst du nicht" | 2026-10-15 | dito |
| „und bau" | 2026-10-15 | dito |
| (standing) „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito (folge352) |
| (standing) „ab jetzt Reports immer in einfache Sprache" | 2026-10-13 | dito |

## Gearbeitet (dieses Rund)

- **Vier Frontier-Runden vorbereitet, verifiziert, verdiktet** — je am Baum geprüft, nie die Stimme:
  · **Netz-Fusion** (`2026-10-15-netzfusion-formular.md` → `…-verdikt.md`): Min = Konjunktion;
    der Baum-Riss (Netz = dH-Hülle vs. Einzelstation = stündlicher |dB/dt|-Peak) benannt; der
    Union-Kontrast an River übergeben.
  · **Fragenpaket F2–F7** (`…-rat-fragenpaket.md` → `…-verdikt.md`): F7 (Aktuator) und F4
    (Schumann) am Baum bestätigt.
  · **Nachfrage F7′/F4′** (`…-nachfrage-f7-f4-wissenschaft.md` → `…-verdikt.md`): nach der weiten
    Recherche (`bin/all_wide`); Pan-Gesetz bzw. WWLLN-Kanal.
  · Mountain-Bogen M1–M3 (`…-mountain-rat-fragen.md`) — Stimmen + Verdikt stehen (fremde Linie).
- **Granit-Check (Operator-Wort „das stichwort ist granit"):** der Granit (`docs/granit.md`) trägt
  **sieben** Grundsätze; die Git-Historie zeigt: geboren mit **fünf** (`200ed6040`: A=A ·
  ICRS & TDB · force_type/`channel_ref` · 0 honored · pending), Satz 6 (`e4a59b92d`) und 7
  (`0d97a8e1f`) später. **Für die Fragebögen gelten die fünf.** Zwei Formulare nachgezogen.
- **F7′-Bau (Operator-Wort „und bau"):** `src/mathematikerin/actuators.rs` — der Vorzeichen-Gate
  (`sign(Σω)` → Stille) ist durch eine **konstante-Leistung-Pan-Regel** ersetzt: Balance =
  Mittelwert / Betrag; bei Σω = 0 mit lebendigen Partialen geht es in die **Mitte**, nicht in die
  Stille; nur ohne Partialenergie schweigt es. river-142s Test
  `a_zero_sum_does_not_collapse_the_channels` wird damit grün. `cargo check` 0/0, `rustfmt` clean.
- **F4′-Quellenmessung (dieses Atom, weite Leiter `bin/all_wide` + Direktmessung):** mehrere
  **offene** Wege gefunden — WGLC (CC-BY, global-monatlich) · WWLLN-Klimatologie-Datenverzeichnisse ·
  stündliches CONUS-Set (UNC Dataverse) · WWLLN-KML-Feed · MTG-LI/GLM — s. F4′ unten. Die erste
  enge Messung war unvollständig.
- folge355 → `archiv/`; die Operator-Worte dieses Runds geschnitten. Commits `e21129d5c`,
  `7f25f9004`, `fb7fcdb66`, `0e8fc665d`, `ab6679435`, `d3ba14e74`.

## Offen — eigen

### F4′ — Schumann-Antrieb: der Blitz-Kanal ist offen, die Wahl ist die Auflösung (eigen + River)
- **Status:** eigen (Rest Operator-gebunden) | **Bindung:** eigen
- **Trigger:** der F4′-Bau (Mycelium: Reihe/CDN) + Rivers bedingte TE
- **Lage:** (gemessen 2026-10-15) `descriptors/schumann_geomag.te`: `cond none`, `surrogate 10`,
  `seasonal none` → kein Blitz-Kanal. Die Runde (12 Sitze) wählt die **WWLLN-Stoßrate**;
  Konditionierung **Stunde × Saison**, Null **innerhalb** jeder Schicht
  (`…-nachfrage-f7-f4-verdikt.md`). **Die erste enge Messung („nur Monatsmittel frei") war
  unvollständig** (Operator-Nachfrage): die weite Leiter (`bin/all_wide`, Spill
  `omegaflow_all_1792067520_WWLLN_…txt`) plus Direktmessung (alle **HTTP 200**) zeigen
  **mehrere offene Wege**:
  · **WWLLN**-Klimatologie-Datenverzeichnisse `wwlln.net/climate/th_clim/data/` + `sd_clim/`.
  · **WGLC** (WWLLN Global Lightning Climatology **+ time series**), Kaplan & Lau, **CC-BY**,
    Zenodo `10725446` u.a. + Pangaea `10.1594/pangaea.904253` — global gegitterte
    Stoßdichte-Reihe (**monatlich**).
  · **Gridded Hourly WWLLN Lightning Flash Density** 2016–2023, 12 km **CONUS**, UNC Dataverse
    `10.15139/s3/nk3ncm` — **stündlich**, aber nur USA (101°W–70°W).
  · WWLLN eigener **offener KML-Feed** (1 h, 6 h Verzögerung) über wwlln.net.
  · offene Satelliten-Blitze: **MTG-LI** (Europa/Afrika, EUMETSAT) · **GLM** (Amerikas) ·
    GHRC Lightning Virtual Collection.
  Nur der **globale stündliche** WWLLN-Rohdatensatz bleibt zugangsgebunden (Verkauf/UW).
- **Blockade:** **keine Quellen-Blockade** — die Wahl ist eine **Kanal-/Auflösungs-Entscheidung**
  (global-monatlich WGLC · regional-stündlich CONUS/MTG-LI/GLM · global-stündlich WWLLN-KML-Feed).
- **Braucht:** Operators Kanal-Wahl; dann Registrierung (Mountain-Verdikt + Mycelium-Manifestation)
  und `TE(abk→f | Kanal, Stunde, Saison)`.

### SABER / UCAR-FPI — Manifestation wartet auf Mountains `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeile je Faden (Mountain) + die Manifestation
- **Lage:** (gemessen 2026-10-15) Athabasca ist gesetzt (`0281ae1a4`). SABER (keyless via HAPI) und UCAR-FPI
  (E-Mail-gated, Operator erledigt) warten auf Mountains `field`-Zeilen.
- **Blockade:** `field` = Mountains Recht. **Braucht:** Mountains `field`-Zeilen. An Mountain:
  `xfer:river208-weberinmanifest1`.

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug unvollständig (abgebrochen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** kein — Bau (Hintergrund-Job)
- **Lage:** (gemessen 2026-10-15) Nur **2000** liegt (doppelt); **2001–2018 fehlen**. `local-only` (© UH Mānoa).
- **Braucht:** `terms local-only` von Mountain; dann Block setzen; Vollzug 2001–2018 (reale Granule).

### Venus-Radar (Level-2 VNRD) — Block steht; CI-Naht grün; Bild-Manifestation offen
- **Status:** eigen (Rest Operator-/Mountain-gebunden) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) VNRD-Block steht (CC0-1.0); `venus-radar-cdn` `38709049909` success.
- **Braucht:** die **Bild-Manifestation (W)** (Operator-Wort 2026-10-14: das Bild bleibt Bild).

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-15) Offen: **SEANOE** (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon`
  fehlt im Register). **Braucht:** Mountain-Zulassung. An Mountain: `xfer:mycelium-342-3`/`-4`.

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) Entwurf `state/mountain/faden-term-field-ttl-draft.φ`; `raw_mirror`-Aktor gebaut.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:mountain-348-2`.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains Zeilen
- **Lage:** (gemessen 2026-10-15) unverändert offen: Ersatzquellen-Blöcke (OCNS2019 `local-only`; JPL SSD/Horizons) ·
  **LOD** · **BASE** (> 2-GiB-Limit `cdn.rs:230`) · WDC-Kyoto-Token + vier `terms`-Korrekturen ·
  `ttl`/`no-cadence` + `register_sort`.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:future.…7d5c9163` · `xfer:mycelium-344-5` ·
  `xfer:mycelium-343-3` · `xfer:mycelium-342-10`/`-343-2`.

### TIMED-Weltlinie — `origin` gemessen; Block wartet auf Mountains `terms`
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains `terms`
- **Lage:** (gemessen 2026-10-15) `origin` gemessen (celestrak GP, per Jina HTTP 200); das Commit-Gate verlangt für den
  Ingest-Block (`format orbit_bin`) eine `terms`-Zeile. **Braucht:** Mountains Celestrak-`terms`.
  An Mountain: `xfer:mycelium-352-5`.

### NSSDC/CLPDS-Planetologie — Route entschieden (Antrag operator-gebunden)
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen | **Trigger:** Operator-Wort „mit antrag"
- **Lage:** (gemessen 2026-10-15) Antragsentwurf bei Future; **Riss** Katalog `CC BY 4.0` vs. CLPDS-„all rights reserved".
- **Braucht:** Operator sendet den Antrag. An future: `xfer:mycelium-347-2`.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen | **Trigger:** kein — Autofill entfernen
- **Lage:** (gemessen 2026-10-15) Login rotiert → der exponierte Wert ist entwertet. **Braucht:** nur den Autofill raus.

### WDC-B Obninsk — Rettung erweitert; Abdeckung unvollständig (eigen)
- **Status:** eigen (Rest = Vollständigkeit + Verdikte) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge; `terms` angewandt
  (Mountain `xfer:mountain-349-1`). Offen: `field` je Datensatz, `terms` der 5 Host-`reference`-
  Blöcke = Mountain. **Braucht:** CDX-Abdeckung ist **nicht** 1993–2024 komplett → `pending`.

## An mountain

Origin: mycelium-356 (2026-10-15):

- `xfer:mycelium-352-1`/`-2`/`-3` **WDC-B** (Wayback + lebender Baum + Schwester-Hosts) — **`field`
  je Datensatz** + **`terms` je Host** setzen (Zensus in folge355, Belege je `datei:zeile`).
- `xfer:mycelium-352-5` **TIMED-Block — Celestrak-`terms` gesucht.**
- `xfer:mycelium-356-1` **SABER / UCAR-FPI** — die Manifestation wartet auf die `field`-Zeilen.
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC) · `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-348-5` (TIMED) · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.

## An river

Origin: mycelium-356 (2026-10-15):

- `xfer:mycelium-356-2` **F7′ ist gebaut** (`d3ba14e74`, `src/mathematikerin/actuators.rs`): der
  Vorzeichen-Gate ist durch die konstante-Leistung-Pan-Regel ersetzt; bei Σω = 0 (lebendige
  Partiale) geht es in die Mitte, nicht in die Stille; river-142s Test wird grün. Der
  **Energie↔Lautstärke-Riss** (konstante Leistung vs. linear, Mono-Kompatibilität) ist eine
  **Deklaration** — noch nicht getroffen; der Aktuator trägt vorerst das constant-power-Gesetz.
- `xfer:mycelium-356-3` **F4′** (Schumann-Blitz-Kanal): die bedingte TE (`abk→f | Kanal, Stunde,
  Saison`) ist Rivers Teil. **Korrektur (weite Leiter `bin/all_wide`):** offen sind **WGLC** (CC-BY,
  global-monatlich), **WWLLN-Klimatologie-Datenverzeichnisse** und ein **stündliches CONUS**-Set
  (UNC Dataverse) sowie der offene **WWLLN-KML-Feed** — die TE-Schicht ist **nicht** blockiert,
  nur die Kanal-/Auflösungswahl steht beim Operator.
- `xfer:mycelium-356-4` **Athabasca ERG-SC VLF** ist manifestiert; offen bleibt nur
  **SABER / UCAR-FPI**.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Atoms: diese Übergabe (folge356), die konsumierte folge355
(→ `archiv/`), die Operator-Gespräche-Cuts; aus dem vorigen Atom die Formulare/Verdikte unter
`state/stimmen/` und `src/mathematikerin/actuators.rs` (bereits committet `e21129d5c`…`d3ba14e74`).
