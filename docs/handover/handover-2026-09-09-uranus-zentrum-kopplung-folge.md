<!--
  title: Handover — Uranus-Zentrum-Kopplung-Folge: (b) Versionen-Differenz, Wobble-Periode, (c) Neptun-Komposition + Riß und die Trägerjahre gebaut; die Neptun-Astrometrie-Kopplung ist die eine offene Linie (Source-Port)
  class: handover
  date: 2026-09-09
  sha256: a88e0a479c97fe708505ed9a9d4d78229ca3ef87717ae8d40c4f4306a0f6e3c2
  status: live
  see-also: docs/handover/handover-2026-09-09-uranus-zentrum-kopplung.md docs/befund/befund-2026-09-09-uranus-zentrum-versionenstruktur.md docs/befund/befund-2026-09-09-uranus-wobble-periode.md docs/befund/befund-2026-09-09-neptun-zentrum-riss.md docs/befund/befund-2026-09-09-uranus-traegerjahre.md docs/TODO.md
-->

# Handover — Uranus-Zentrum-Kopplung-Folge

Übergabe für die nächste Sitzung. Die empfangende Sitzung hat die Übergabe
`handover-2026-09-09-uranus-zentrum-kopplung.md` konsumiert: vier Atome gebaut
und gemessen, eines (die Neptun-Astrometrie-Kopplung) bleibt als Source-Port
offen.

## 0. Die abgebende Sitzung ist abgeschlossen

Commits: `62096a3` (b Versionen-Differenz), `c481b35` (Wobble-Periode),
`dd394f2` (c Neptun-Komposition), `d97345f` (Astrometrie-Erreichbarkeit),
`5363949` (c Neptun-Riß), `d0480a4` (Trägerjahre). Offen bleibt nur der
Source-Port in §3. Alle Befunde tragen den gesetzten sha256, alle Proben
laufen `cargo check` 0 Fehler / 0 Warnungen und passieren `commit_check`.

## 1. Was gemessen steht

- **(b) Versionen-Differenz** — per-Punkt-Vektor (ΔRA·cosδ, ΔDec):
  \|Δ\| Mittel 39.6/44.6/47.9 mas, säkulares Wachstum (de441−epm 9.9 → 74.8 mas
  von 1992 bis 2011, Maximum 2011 — die Eisriesen-Kluft wächst mit dem
  Extrapolations-Abstand). Kalibrier-Gate exakt.
- **Wobble-Periode** — km-groß (24.77 km Mittel / 44.27 km max, J2000+365 d),
  die Uranus-Mond-Umläufe (Oberon 13.46 d / 20.3 km dominant, Titania 8.71 d,
  Umbriel 4.14 d, Ariel 2.52 d); die „1,4-d-Signatur" (Miranda) ist die
  schwächste. Die Vorlage notierte km als „m" — im TODO korrigiert.
- **(c) Neptun-Komposition** — `horizons_compiler --neptune-c-spk` (DE441-
  Baryzentrum + nep097xl 899−8 → `ephemeris_neptune_c.bin`), verifiziert
  (Triton-Wobble 74.1 km, Reproduktions-RMS 51 m).
- **(c) Neptun-Riß** — die drei Häuser am Zentrum: \|Δ\| Mittel 589/3227/3208 km,
  Minimum in den 1980ern (Voyager-2-Anker 1989), bis 9571 km in den 2020ern.
  Kalibrier-Gate exakt.
- **Trägerjahre** — die vier de441-Trägerjahre tragen sub-σ-Margen
  (0.1–4.0 mas gegen ⟨σ⟩ = 82.5 mas) mit gestreuten Richtungen: sub-σ-
  Kreuzungspunkte der drei Residuen-Kurven, kein großer physikalischer Grund.

## 2. Der Infrastruktur-Stand

- Neue Proben (`tools/measure/src/bin/`): `uranus_center_versionenstruktur_probe`,
  `uranus_wobble_period_probe`, `neptune_center_check_probe`,
  `neptune_center_rift_probe`, `uranus_carrier_years_probe`.
- `horizons_compiler` trägt `--neptune-c-spk`; eigener Workflow
  `neptune-c-spk-cdn.yml` (nie kernel-flatten). `phi/sources.φ` trägt
  nep097.bsp + nep097xl-899.bsp + ephemeris_neptune_c.bin.
- **Folge-Sitzung:** den CI-Lauf von `neptune-c-spk-cdn.yml` verifizieren —
  `ephemeris_neptune_c.bin` muß auf dem CDN 200 liefern; bei rotem Lauf den
  Schritt lesen.

## 3. Die eine offene Linie — Neptun-Astrometrie-Kopplung (Source-Port)

Die Reduktion gegen das Zentrum fehlt Neptun, weil keine saubere
Planetenzentrum-Tabelle vorliegt. Erreichbarkeit gemessen (2026-09-09):
- Camargo 2015 (J/A+A/582/A8) ist Uranus-only („Astrometry of the main
  satellites of Uranus", kein `neptu_j`).
- GeoAzur-Basis (`astrogeo/observations/base/`, Neptun 1753–1995) erreichbar
  (200), aber FK4/Boss-GC-Rahmen — Präzession nach J2000 nötig.
- Yunnan Neptun+Triton 2020–2024 (Icarus 437:16625) liegt nicht in VizieR
  (ScienceDirect).

Die Arbeit läuft über `docs/SOURCE_PORT.md` (Self-carrying-Protokoll, Grind).
Ziel: eine Neptun-Planetenzentrum-Tabelle ernten + registrieren + gegen
`ephemeris_neptune_c.bin` reduzieren (Spiegel von `--center`), Kalibrier-Gate,
Befund. Der Riß-Probe (`neptune_center_rift_probe`) bleibt Modell-gegen-Modell,
bis die Tabelle steht.

## 4. Arbeitsregeln für die bauende Sitzung

- **Geteiltes Repo:** die Parallel-Session committet weiter (derzeit Archivar:
  ephemeris.rs/motion.rs/tests.rs) — nur eigene Dateien stagen, Hunks der
  Parallel-Session nie, `.git/index.lock` kurz abwarten. Eigene Commits über
  privaten Index (`GIT_INDEX_FILE`) und danach `git reset HEAD -- <eigene
  Pfade>` — sonst bleibt der geteilte Index gegen HEAD veraltet.
- **Commit-Gate:** `cargo check` null Fehler/null Warnungen; `commit_check`
  blockt `unwrap_or(0.0)`, `unwrap_or(0)`, `unwrap_or_default(`,
  `unwrap_or_else(`, `#[derive(Default)]`, Deutsch-in-Code, verbotene Wörter.
- **Kalibrier-Gate:** injizierte Fälle exakt zurückgewonnen; die physikalische
  Form der Signatur separat geprüft.
- **0-Kanon:** absent bleibt absent; ein gemessen freigegebener Pfad ist
  `descoped` mit Befund.
- **A = A:** Konsistenz ist ein Zwirn, keine Bestätigung.
