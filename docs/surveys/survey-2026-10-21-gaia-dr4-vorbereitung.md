<!--
  title: Survey — Gaia DR4 Vorbereitung: 12-Quellen-Dry-Run der Eigenbewegung (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 497ffac2d2ff547dbc2372d9979215a9664b8e6e8c51111d2eca392318dcccc2
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
   `al_i = pm_ra · (t_i−t̄) · sin(pa_i) + pm_dec · (t_i−t̄) · cos(pa_i) + c`
   (3 Parameter, Normalgleichungen, Gauß-Elimination). `pa` ist
   `scan_pos_angle`; `al` ist `centroid_pos_al`.
5. **Residuum** als RMS in mas. Keine Parallaxe, keine Qualitätsflags, keine
   Kepler-Bahn — das ist der dry run, nicht der Endfit.

## Ergebnis (Lauf 2026-10-21)

`rows 1008`, `sources 12`. Die Tabelle nennt `ra0/dec0` statt der gerundeten
64-bit-ID; die ID wird im vollen DR4-Lauf exakt geführt.

| # | ra0 | dec0 | Epochen | Spanne (a) | pm_ra (mas/a) | pm_dec (mas/a) | RMS (mas) | Lesart |
|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 42,086693 | +8,531269 | 663 | 5,36 | **+27,84** | **−18,28** | 7,07 | Einzelstern |
| 2 | 44,859053 | +4,493684 | 804 | 5,36 | **+0,36** | **+0,10** | 6,67 | QSO-nah |
| 3 | 47,461804 | +21,196202 | 679 | 5,02 | **+0,38** | **−0,19** | 9,16 | QSO-nah |
| 4 | 47,907514 | +2,966986 | 677 | 5,36 | +127,40 | −31,75 | **1763,10** | Bahn/Parallaxe, kein Linearfit |
| 5 | 49,028117 | +47,857329 | 755 | 5,11 | +3,26 | −5,98 | 3,52 | Einzelstern |
| 6 | 51,276459 | +8,814584 | 577 | 5,04 | **−0,19** | **+0,11** | 6,69 | QSO-nah |
| 7 | 179,204832 | +16,837002 | 577 | 5,39 | −9,27 | −0,63 | 3,45 | Einzelstern |
| 8 | 198,079292 | +17,517124 | 879 | 5,35 | **−566,12** | **−59,58** | **283,25** | hohe PM + Bahn |
| 9 | 201,230513 | +61,410223 | 892 | 5,33 | −123,81 | −135,75 | 17,68 | Bahn/Parallaxe |
| 10 | 209,506327 | +31,695500 | 1077 | 5,35 | −70,95 | +19,06 | 21,02 | Bahn/Parallaxe |
| 11 | 292,779599 | −16,702466 | 730 | 4,75 | **+2,34** | **−0,51** | 2,40 | QSO-nah |
| 12 | 294,827849 | +14,930918 | 631 | 5,10 | −28,62 | −150,26 | 38,77 | Bahn/Parallaxe |

**Was der Dry-Run zeigt.**

- Die **QSO-nahen Quellen** (2, 3, 6, 11) geben **pm ≈ 0,1–2,3 mas/a** mit
  RMS 2,4–9,2 mas. Für die Frame-Wache zählt genau das: Ein QSO-Anker mit
  pm ≈ 0,3 mas/a verschiebt sich über 25 Jahre um ≈ 7,5 mas — klein gegen die
  Rahmenrotation, aber nicht null.
- Die **Bahnen/Parallaxen** (4, 8, 9, 10, 12) brechen den linearen Fit
  (RMS bis 1763 mas). Das ist erwartet: Die Vorabveröffentlichung enthält
  ausdrücklich drei nicht-einzelne Quellen mit Kepler-Bahnen. Ein 3-Parameter-
  Fit ist für sie das falsche Modell; der DR4-Endfit braucht Parallaxe und
  ggf. eine Bahn.
- Die **Spannen** 4,75–5,39 Jahre und 577–1077 Epochen je Quelle zeigen, dass
  die Zeitreihen tatsächlich Epochenastrometrie sind — kein Mittelwert.

## Was für den 2.12. noch fehlt

1. **Exakte 64-bit-`source_id`.** Der generische VOTable-Reader legt `long` als
   f64 ab; für das Matchen der 512 Anker braucht der DR4-Lauf einen
   ID-exakten Parser (oder ein Positions-Matching).
2. **Qualitätsflags.** `used_by_agis_al`, `transit_acq_flags`,
   `transit_proc_flags`, `ccd_proc_flags` werden noch nicht angewendet.
3. **Parallaxe/5-Parameter.** Der Endfit braucht `parallax_factor_al` und
   `parallax_factor_ac`; der Dry-Run ist 3-parametrig.
4. **Scan-Winkel-Konvention.** Die Vorzeichen von `pm_ra`/`pm_dec` sind aus
   `scan_pos_angle` übernommen, aber noch nicht gegen DR3/DR4-PMs geprüft.
5. **Die 512 Anker.** Der TAP/Archive-Pfad für die Anker-IDs ist noch nicht
   gebaut; die 12 Quellen sind der Methodenbeweis.
6. **Die 88 ohne Gaia.** Bleiben die zweite Lücke; DR4 kann sie nur schließen,
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
