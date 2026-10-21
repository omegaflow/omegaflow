<!--
  title: Survey — Gaia DR4 Vorbereitung: 12-Quellen-Dry-Run mit Eigenbewegung und Parallaxe (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 3006ae727bfa09118a6db0aa67ed8363ebe762711b30729216e4f2e365295a55
  status: live
  see-also: docs/surveys/survey-2026-10-19-rohdaten-12von15.md docs/surveys/survey-2026-10-20-frame-wache.md projects/quasare.md tools/measure/src/bin/gaia_dr4_probe.rs
-->
# Survey — Gaia DR4 Vorbereitung: 12-Quellen-Dry-Run

**Zweck.** Die offizielle Gaia DR4 kommt am **2026-12-02**. Die ESA-Vorab-
veröffentlichung vom Juni 2026 liefert bereits **astrometrische Zeitreihen für
12 Quellen**. Dieser Survey baut und fährt die Methode, die am 2.12. auf die
**512 ICRF-Anker** angewendet wird: lineare Eigenbewegung aus den
Along-Scan-Centroiden und den Scan-Positionswinkeln.

**Kein Ersatz für DR4.** Die 12 Quellen sind ein Prüfstand, nicht der
Anker-Datensatz. Die 512 Anker und die 88 ohne Gaia-Zuordnung bleiben offen.

## Quelle (gemessen 2026-10-21)

| | |
|---|---|
| Datei | `gaia-dr4-prerelease-epoch-astrometry_2026-06-26.zip` |
| URL | `https://anonftp.cosmos.esa.int/pub/GAIA_PUBLIC_DATA/Gaia_DR4/dr4-prerelease/gaia-dr4-prerelease-epoch-astrometry_2026-06-26.zip` |
| Größe / Format | **625.651 B**, ZIP-Magic, HTTP 200 |
| sha256 | `07f0e8d9ac97a29ea376a0c7242de3124d2a08ad72aba0958d6575d94d35fa0b` |
| Inhalt | `GAIA_DR4_PRERELEASE_EPOCH_ASTROMETRY_RAW.xml` (1.183.282 B, VOTable 1.4, BINARY2) + `readme.txt` |
| Zeitbasis | `TIMESYS timeorigin=2455197.5`, `timescale=TCB`; Spalte `obs_time_tcb` in ns |
| Felder | `source_id`, `ra0`, `dec0`, `obs_time_tcb[]`, `scan_pos_angle[]`, `centroid_pos_al[]`, `centroid_pos_ac[]`, `*_error_*[]` |

## Methode

1. **VOTable BINARY2** über den vorhandenen Reader (`votable_to_json`) in Zeilen
   zerlegen.
2. **Gruppieren** nach `(ra0, dec0)` — die 12 Quellen sind eindeutig; die
   Gruppierung über Position umgeht die f64-Rundung der 64-bit-`source_id`.
3. **Zeit** `t = obs_time_tcb / 1e9 / 86400 / 365.25` in Jahren; der Ursprung
   kürzt sich im Fit.
4. **Fit** je Quelle:
   `al_i = pm_ra · dt · sin(pa_i) + pm_dec · dt · cos(pa_i) + plx · pf_i + c`
   (4 Parameter, Normalgleichungen, Gauß-Elimination). `pa` ist
   `scan_pos_angle`, `al` ist `centroid_pos_al`, `pf` ist
   `parallax_factor_al`.
5. **Qualität:** Epochen mit `used_by_agis_al = false` werden verworfen; das
   reduziert die Epochenzahlen (z. B. Quelle 2: 804 → 698). Das Residuum ist
   der RMS in mas. Kepler-Bahnen sind nicht modelliert.

## Ergebnis (Lauf 2026-10-21)

`rows 1008`, `sources 12`. Die Tabelle nennt `ra0/dec0` statt der gerundeten
64-bit-ID; die ID wird im vollen DR4-Lauf exakt geführt.

| # | ra0 | dec0 | Epochen | Spanne (a) | pm_ra (mas/a) | pm_dec (mas/a) | plx (mas) | RMS (mas) | Lesart |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 42,086693 | +8,531269 | 574 | 5,04 | **+27,10** | **−18,21** | +2,45 | 9,44 | Einzelstern |
| 2 | 44,859053 | +4,493684 | 698 | 5,05 | **+0,07** | **+0,08** | −0,06 | 3,99 | QSO-nah |
| 3 | 47,461804 | +21,196202 | 602 | 5,02 | **+0,23** | **−0,37** | −0,52 | 5,97 | QSO-nah |
| 4 | 47,907514 | +2,966986 | 596 | 5,05 | +6,25 | −3,66 | +1,39 | **3,16** | Parallaxe erklärt den 3-Param-Ausreißer |
| 5 | 49,028117 | +47,857329 | 644 | 4,76 | +3,48 | −5,95 | +0,72 | 2,87 | Einzelstern |
| 6 | 51,276459 | +8,814584 | 509 | 5,04 | **−0,40** | **+0,02** | +0,16 | 4,39 | QSO-nah |
| 7 | 179,204832 | +16,837002 | 459 | 5,13 | −9,42 | −0,48 | +1,04 | 2,03 | Einzelstern |
| 8 | 198,079292 | +17,517124 | 558 | 4,92 | **−597,21** | **−35,13** | +103,54 | **199,03** | nahe/Bahn, kein 4-Param-Fit |
| 9 | 201,230513 | +61,410223 | 778 | 5,21 | −124,51 | −135,91 | +10,68 | 6,52 | Bahn/Parallaxe |
| 10 | 209,506327 | +31,695500 | 824 | 4,94 | −75,76 | +17,72 | +13,63 | 2,19 | Bahn/Parallaxe |
| 11 | 292,779599 | −16,702466 | 637 | 4,75 | **+2,53** | **−0,50** | +0,93 | 1,07 | QSO-nah |
| 12 | 294,827849 | +14,930918 | 558 | 5,10 | −29,39 | −150,10 | +1,88 | 29,82 | Bahn/Parallaxe |

**Was der Dry-Run zeigt.**

- Die **QSO-nahen Quellen** (2, 3, 6, 11) geben **pm ≈ 0,07–2,53 mas/a**,
  **plx ≈ −0,5…+0,9 mas** und RMS 1,1–6,0 mas. Für die Frame-Wache zählt
  genau das: Ein QSO-Anker mit pm ≈ 0,3 mas/a verschiebt sich über 25 Jahre
  um ≈ 7,5 mas — klein gegen die Rahmenrotation, aber nicht null.
- **Quelle 4** war im 3-Parameter-Fit ein Ausreißer (PM +127/−32, RMS 1763).
  Mit dem Parallaxenterm wird sie **PM +6,25/−3,66, plx +1,39, RMS 3,16** —
  das jährliche Signal war Parallaxe, nicht Eigenbewegung.
- **Quelle 8** bleibt mit RMS 199 mas und plx 103,5 mas ein Nicht-Einzel-Fall
  (nahe Quelle/Bahn); sie braucht ein Bahnmodell, keinen linearen Fit.
- Die **Spannen** 4,75–5,21 Jahre und 459–824 Epochen je Quelle (nach
  Qualitätsfilter) zeigen, dass die Zeitreihen echte Epochenastrometrie sind —
  kein Mittelwert.

## Was für den 2.12. noch fehlt

1. **Exakte 64-bit-`source_id`.** Der generische VOTable-Reader legt `long` als
   f64 ab; für das Matchen der 512 Anker braucht der DR4-Lauf einen
   ID-exakten Parser (oder ein Positions-Matching).
2. **Scan-Winkel-Konvention.** Die Vorzeichen von `pm_ra`/`pm_dec` sind aus
   `scan_pos_angle` übernommen, aber noch nicht gegen DR3/DR4-PMs geprüft.
3. **Die 512 Anker.** Der TAP/Archive-Pfad für die Anker-IDs ist noch nicht
   gebaut; die 12 Quellen sind der Methodenbeweis.
4. **Die 88 ohne Gaia.** Bleiben die zweite Lücke; DR4 kann sie nur schließen,
   wenn der Crossmatch sie enthält.

## Reproduktion

```sh
unzip -o dr4_prerelease.zip
cargo build --release -p omegaflow-measure --bin gaia_dr4_probe
./target/release/gaia_dr4_probe \
  --xml GAIA_DR4_PRERELEASE_EPOCH_ASTROMETRY_RAW.xml
```

## Ablage

- Probe: `tools/measure/src/bin/gaia_dr4_probe.rs`
- Quelle/Hash: `docs/surveys/survey-2026-10-19-rohdaten-12von15.md` (Zeile 2d)
- Frame-Wache: `docs/surveys/survey-2026-10-20-frame-wache.md`
- Projekt: `projects/quasare.md`, Arm B

## Offen

- 512-Anker-Matching und 5-Parameter-Fit für den 2.12. bauen.
- Die QSO-nahen 12 Quellen gegen DR3-PMs prüfen, sobald der DR4-Katalog
  querybar ist.
- Die Kepler-Quellen (4, 8, 9, 10, 12) als Bahn-Prüffälle führen, nicht als
  PM-Ausreißer.
