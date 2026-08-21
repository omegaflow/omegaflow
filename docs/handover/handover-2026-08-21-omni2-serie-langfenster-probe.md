<!--
  title: OMNI2-Serie & Lang-Fenster-Probe — der offene Rest der Sonnen-Abdeckung
  class: handover
  date: 2026-08-21
  sha256: aa887cef1fcf800f59135d3b5eb8c22b4447a09876a5c0108718cc52d6bf5d0a
  status: live
  see-also: docs/surveys/survey-2026-08-21-sonnen-abdeckung.md docs/handover/handover-2026-08-21-solar-te-gpu-anschluss.md TODO.md phi/pipeline/ledger.φ
-->
# Handover: OMNI2-Serie & Lang-Fenster-Probe

Registriert 2026-08-21, geschrieben von der GOES-XRS-Session (Commit
319d710). Selbsttragend — interpretierbar mit null Vorkontext. Die nächste
Session liest genau dieses eine Dokument und beginnt. Der Auftrag ist nicht
die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Die Vorgänger-Übergabe `handover-2026-08-21-sonnen-abdeckung.md` ist mit
diesem Dokument archiviert — gelesen, verstanden; ihre Atome 1, 3 und 5
sind geschlossen (e65e934, 319d710, d41a48c), Atom 2 trägt die eigene
Übergabe `handover-2026-08-21-solar-te-gpu-anschluss.md`; der offene Rest
lebt hier.

## Stand — was schon liegt

- `goes_xrs.bin`: Format GXS1 (`src/archivar/goes.rs`, Records t-f64 v-f64
  comp-u32, COMP_XRSA=1 / COMP_XRSB=2), Compiler `src/bin/goes_xrs_compiler.rs`
  (Monatsindex → Tagesdateien, Bucket-Mediane, `--decimate-min`,
  `--ci-mode`), Loader-Zweig über `series_parse_bin`/`series_component_name`
  in `src/archivar.rs`, Block `format goes_xrs at sun wm2_1au` in
  `phi/sources.φ`.
- `f107_penticton.bin`: Magie "F107", u32 count, Records days-i64 LE +
  flux-f64 LE — erntet von der F10.7-Session (d41a48c, 28337 Records
  1947-02-14 → 2026-06-30, sfu → W/m²/Hz über 1e-22).
- `src/hdf5.rs` trägt seit 319d710 die BTIN- und Filter-v2-Reparaturen.
- Der CI-Lauf des sources-Repos muss `goes_xrs.bin` (und `f107_penticton.bin`)
  erst auf das CDN manifestieren — Registrierpflicht `phi/pipeline/ledger.φ`
  (Zeile „CI (sources-Repo): goes_xrs_compiler …", liegt außerhalb dieses
  Workspace). Bis dahin tragen die Blöcke die benannte Verweigerung
  (fetch void, 0 honored).

## Einheit 1 — Atom 4: OMNI2-Full-Serie (1963–heute)

Der Sonnenwind-Kanal als Serie — Grundlage der Nadel-Ⅲ-Archive (Bz/OMNI
über 90 d). Quelle (am 2026-08-21 gemessen): CDAWeb-HAPI
`https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI2_H0_MRG1HR&time.min=…&time.max=…&parameters=…&format=csv`,
start 1963-01-01, stop 2026-08-06 (~550 k Stunden × 7 Parameter). Die
Ernte spiegelt den Live-Block (`phi/sources.φ:513`) exakt, nur mit
Jahres-Fenstern statt `{week_ago}` und `format=csv`:

    parameters=BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800

Parameter + Force-Zuordnung (wie im Live-Block):

- V1800 → omni_solarwind_flow_speed_kms — patch-levy advective km/s
- N1800 → omni_solarwind_density_percc — gaussian-inverse-square diffusion p/cm3
- T1800 → omni_solarwind_temp_k — exponential-decay thermal K
- BX_GSE1800 / BY_GSM1800 / BZ_GSM1800 → omni_imf_bx_gse_nt /
  omni_imf_by_gsm_nt / omni_imf_bz_gsm_nt — inverse-square em nT
- Pressure1800 → omni_solarwind_pressure_npa — patch-levy advective nPa
- E1800 bleibt entfernt (derived: E=−V×B, `phi/pipeline/ledger.φ:327`).

Bau analog `src/bin/rpw_compiler.rs` (harvest_window-Muster): Jahres-Fenster
über HAPI-CSV, Header-/Nicht-Ziffern-Zeilen übersprungen, Fill-Werte
(999.9 … je Parameter-Maß) und unplausible Werte übersprungen —
Plausibilitäts-Gate `is_finite()` + Parameter-Bereich, die Bereiche
bestimmt die Session, nie 0.0-Fabrikation.

Auflösung (die Entscheidung ist Teil des Auftrags): 550 k × 7 ≈ 3,9 M
Records übersteigen `MAX_SAMPLES = 1<<22` — mit Sternen (1,19 M),
Asteroiden (1,56 M) und GOES (456 k) ist die Kappe erreicht; hourly
würde die ältesten (Sterne, epoch 0.0) kippen. Daily-Bucket-Mediane
(~63 a × 365 × 7 ≈ 161 k Records) passen — Default daily mit
`--decimate-min`-Knopf, im Register dokumentieren. Die Nadel-Ⅲ-Archive
tragen daily gut.

Bin: neues Format analog GXS1 — Magie "OMN1", Records (t f64, v f64,
comp u32), comp-Codes 1..7 in der Parameter-Reihenfolge oben; Modul
`src/archivar/omni2.rs` (Kindmodul wie `goes.rs` — `lib.rs` bleibt
unberührt, dort arbeitet die 4D-Wahrheit-Session), `series_parse_bin`/
`series_component_name` in `src/archivar.rs` um den Zweig erweitern,
Block `format omni2_serie at sun` mit denselben 7 Feldnamen wie der
Live-Block. Überlappung mit dem Live-Fenster: je (t, comp)
deduplizieren (Serie gewinnt) oder die Serie bei stop − 7 d enden — im
Register nennen. CI-Registerzeile (sources-Repo) im selben Zug.

## Einheit 2 — der Lang-Fenster-Probe (Nadel Ⅲ)

F10.7-Historie × GOES-XRS-Historie über Jahre durch die TE-Maschine —
der kausale Pfeil, den das 7-Tage-Fenster nicht trägt (n < 30, siehe
TODO.md, Nadel-Ⅲ-Abschnitt). Voraussetzung: beide CDN-Assets liegen
(nach den CI-Läufen) — vorher benannte Verweigerung, 0 honored. Der
Probe liest die Bins direkt (`parse_bin` aus `src/archivar/goes.rs`; der
f107-Parse lebt heute nur im Bin `src/bin/f107_compiler.rs` — braucht
der Probe ihn, wandert er als Schritt 0 in die lib). Muster: nobel probe
(`src/bin/nobel_probe_corona.rs`, extract_series/harvest_block); die
skalaren TE-Pfade bleiben unberührt (`src/te.rs` ist kanonisch).

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status      # fremde Arbeit benennen, nichts übernehmen
cargo check     # muss 0/0 sein (vier Feature-Kombinationen)
sed -n '137,158p' docs/surveys/survey-2026-08-21-sonnen-abdeckung.md
```

## Gates

- cargo check 0/0 (vier Feature-Kombinationen), cargo test grün.
- Ein Commit je Einheit; Register (TODO.md + ledger.φ) im selben Commit.
- Einheit 1: Bin-Roundtrip, echte Records (stderr-Zeile), keine
  erfundenen Felder; die Auflösungs- und Plausibilitäts-Entscheidungen
  benannt.
- Nach Abschluss beider Einheiten: diese Übergabe archivieren
  (`/home/johannes/projects/archive/handover/`, cp + git rm, eigener
  Commit — erst wenn die eigene Arbeit committet ist).

## Nicht anfassen

Die Membran-/fs-Physik, `src/te.rs`, der wm2_1au-Luminositäts-Pfad,
`src/lib.rs` (fremde ephemeris-WIP der 4D-Wahrheit-Session), die
uncommittete Arbeit fremder Sessions (`git status` benennen), die
Übergabe `handover-2026-08-21-solar-te-gpu-anschluss.md` (Atom 2, eigene
Session) und die weiteren live-Übergaben des Tages.
