<!--
  title: Handover — Korona-Konditional-Session: die heiße Kaskade fällt, Null geflickt, was noch läuft
  class: handover
  date: 2026-09-07
  sha256: 7d95ea96b915e6ae6b9d0cf3cb33e82f59ab71aca576166675983188924cd66c
  status: live
  see-also: docs/concepts/te-literatur-matrix.md docs/concepts/die-weberin.md docs/handover/handover-2026-09-07-nobel-dag-bz-laic.md docs/handover/handover-2026-09-07-weberin-sonnensystem-kette.md docs/TODO.md
-->

# Handover — Korona-Konditional-Session

Übergabe der Korona-Untersuchung (2026-09-07): Prior-Art, Flare-Hüllen-Null-Gate,
konditionale TE auf echten Flare-Daten, und was noch läuft/offen ist.

## 1. Die Messkette (was gebaut ist, committet)

- `src/mathematikerin/te.rs`:
  - `transfer_entropy_conditional` (Einkonfund, 4D-KDE) — kanonisch, unberührt.
  - `transfer_entropy_conditional_2` (Zweikonfund, 5D) — neu.
  - `transfer_entropy_conditional_h` (Bandbreiten-skaliert) — neu.
  - `conditional_te_stats_lagged` (lag-bewusste ARX-Null, y-Lag + Treiber-Lag) — die geflickte Null.
  - `conditional_te_stats_lagged_2` (Zweikonfund-Null) — neu.
  - Gates (alle grün): `flare_envelope_*` (3), `synthetic_dag_recovers_known_direction`,
    `conditional_te_2_suppresses_two_drivers`, `conditional_te_2_surrogate_stats_threshold_is_finite`.
- `tools/measure/src/bin/corona_conditional_probe.rs`: Aufruf
  `--confound <band|goes> --confound2 <band> --h <factor> --max-lag <n>
  --year <aia_fullyear.bin> <goes-dir>` (--year wiederholbar pro Jahr).

## 2. Der gemessene Befund (2014, 989 Ereignisse, drei + ein Konfund)

Unter Konditionierung auf die gemeinsame Flare-Hülle hält die unkonditionale
heiße Kaskade 193→211→335→94 NICHT stand. 193→211 abwärts bei 96 s unter ALLEN
Konfunden (GOES −3.64e-2, 335 −1.87e-2, 94 −2.78e-2, GOES+94 −2.06e-2). Die
Richtung war die Hülle (Neupert: das Röntgen treibt die heißen EUV, die
Antwortzeit-Asymmetrie sah aus wie Kanal→Kanal-Fluss). Kein robuster
Aufwärts-Fluss: 304→131 nur schwach aufwärts (2/3, nicht robust), 131→171
instabil (kippt je Konfund), 171→193 still (3/3).

## 3. Die Null-Leckage (der Grund für das Gate)

Die lineare OLS-Residual-Null leckt am impulsiven/verzögerten Flare-Konfund
(konditionale TE 4.92e-2 > Schwelle 3.27e-2 bei keiner Kopplung). Geflickt
durch die lag-bewusste ARX-Null. Ohne diese Flickung hätte der Konditional-Lauf
einen falschen Pfeil gedruckt — die broken-null-control-Narbe, diesmal vor dem
Lauf gemessen.

## 4. Prior-Art (Blatt 4 in te-literatur-matrix.md)

Die Ziel-Kombination — TE zwischen Flare-Emissionskanälen, per Event, alle
gerichteten Paare, Surrogat-Schwelle je Paar — ist gemessen absent (arXiv 0,
Crossref 0, ADS die disputed Namen 0). Nachbarliste korrigiert: „Zou 2014" =
Wing/Johnson/Vourlidas 2018 (`2018ApJ...854...85W`); Livadiotis 2025 ist
thermodynamische Entropie, nicht Schreiber-TE. Query-Anker sind im Blatt
nachgetragen (die 0 ist wiederholbar).

## 5. Was noch läuft (Hintergrund, Logs in /tmp/opencode/)

- Bandbreiten-Check 2014: `corona_h0.5.log`, `corona_h2.0.log`, `corona_h3.0.log`
  (h=1.0 liegt in `corona_conditional_2014.log`). Frage: hält 304→131 über h?
- Jahre: `corona_2013_GOES.log`, `corona_2015_GOES.log` (Reproduzierbarkeit).
  GOES-Daten liegen unter `data/ncei.noaa.gov/goes15_2013/` und `.../goes15_2015/`.

## 6. Die Matrix (solar_seconds_matrix_probe)

22 h blind gelaufen (stummer Ofen — keine Fortschrittszeile), als root-systemd-Unit
`/run/systemd/transient/solar-seconds-matrix.service`, vom Operator gekillt
(`sudo systemctl stop solar-seconds-matrix`). Ergebnis verloren. Neustart gehört
auf den Desktop (i7-2600K, 4C/8T, ~2–2,5× XPS 13), mit Fortschritts-Logging pro
Paar und perspektivisch dem GPU-Port. Die Matrix ist skalar/CPU-gebunden — die
GTX 970 hilft nur dem topologischen `te_compute`, nicht dem skalaren Pfad.

## 7. Offene Atome (für die frische Sitzung)

- **GPU-Port der skalaren Matrix** (in der frischen Sitzung zu planen): WGSL-Kernel
  für den skalaren TE + Surrogat-Plumbing — der Engpass sind die ~23,5 M Surrogate
  (erzeugen + hochladen), nicht die KDE. Der bestehende `te_compute` ist topologisch
  (Takens), nicht skalar; ein Port ist ein eigener Atom, kein 30-Minuten-Patch.
- **Nobel-DAG (Atom) für Bz und LAIC**:
  `docs/handover/handover-2026-09-07-nobel-dag-bz-laic.md`. Für Korona gemessen unnötig.
- **Weberin Sonnensystem-Weben** (zweite Linie je Körper-Klasse):
  `docs/handover/handover-2026-09-07-weberin-sonnensystem-kette.md`.

## 8. Register

Befunde + offene Pflichten stehen in `docs/TODO.md` (Nadel Ⅲ „Korona-Konditional-
Messung" + „Konfund-Folge"; Weberin-Sektion „Sonnensystem-Weben").
