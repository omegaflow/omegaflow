<!--
  title: Handover — disjunkte Linien Folge: Register-Schluss, Provenienz-Notiz, Step-5 gemessen
  class: handover
  date: 2026-09-09
  sha256: 8672e46ddb665781db3f734add096155d3a4860c7cc341ed5ad15f161716160f
  status: archived
  see-also: docs/handover/archiv/handover-2026-09-09-disjunkte-linien-dispatch.md,
            docs/handover/handover-2026-09-09-mechanische-reste.md
-->
# Handover — disjunkte Linien Folge: Register-Schluss, Provenienz-Notiz, Step-5 gemessen

Fortsetzung der disjunkten Register-Linie. Diese Sitzung schloss die offenen
Register-Punkte des Vorgängers und maß die Step-5-Klasse der sauberen Datenbank.
Die TE-/Tiefenphasen-Linie lief parallel weiter; ihre Dateien blieben unberührt.

## Geschlossen (trägt Git)

- **Register-Schlüsse der disjunkten Linie** — Galileos Blatt und die
  Bz-/omni2-Manifestation sind von der TE-Linie geschlossen (`9aa9350`,
  `befund-galileo-nsurr-20.md` done; sources.φ:1917 + CDN 200 + der
  `nobel_probe_bz`-Fallback) — registriert, nichts gebaut. `ledger.φ` (lokales
  Register, gitignored): die parser-gap-Sektion trägt den Schluss (`d41cc87`),
  die Bucket-Dispositionen je Dataset bleiben eine Register-Frage (Litmus:
  url-line / Compiler-Lease / Konsument).
  `noaa_nodd_inventory.φ` steht unter `phi/pipeline/catalog/` (101 Datasets).
- **Provenienz-Notiz-Muster** — gebaut: `docs/specs/provenienz-notiz.md` (drei
  benannte Zeilen — Frage-Ursprung / Operator-Überstimmungen / benannte
  verworfene Pfade; „zehn Zeilen" = Menge, keine erfundene Kategorie; jede
  ungemessene Zeile = `pending`). `das-eine-instrument.md` §5 ist die erste
  Instanz. Schließt „Provenienz-Notiz-Muster steht aus".
- **R2 verifiziert pending** — kein Fabrik-Fix: `number_audit.rs` trägt den
  Grund in der R2-Ausgabe und im Test
  `z_section_counts_are_not_a_double_count_and_r2_stays_pending`; lauf-log und
  Register stimmen überein. Die Z-Klasse braucht die Archiv-Zählung als
  Grundwahrheit — lokal zählbar (archive-root + `/home/johannes/backup/archive/`),
  kein Operator-Wort.

## Gemessen (Step-5, Rat gehört 2026-09-09)

- **14 `repo_tag`-Releases** (disponiert): alle tragen `mirror_*`-Assets mit
  sha256-Digest; 12 Quell-Repos leben (Rebuild-Quelle = das Repo, Inhalt per
  Digest gepinnt), `Bowserinator/PeriodicTable-JSON` repo-tot (Release-Tag
  schon 404), `cristianst85/GeoNuclearData` 0 Assets. Der destruktive Schnitt
  bleibt ein benannter Folgeschritt: je Asset Byte-Vergleich CDN-Digest ↔
  Repo-Raw, einzeln, nie ein Blindwurf.
- **3 `dataset_host`-Leases** (behalten): physionet.org + spdf.gsfc.nasa.gov
  tragen sources.φ-url-lines (bidsleep / wind_orbit / wind_waves);
  sentinel1euwest.blob.core.windows.net ist Compiler-Lease (`s1_sar_compiler`
  + `s1-sar-cdn.yml`) ohne url-line.

## An die nächste Sitzung

- **docs-reference-verteilung** — bleibt ein eigenes Atom (reine Bewegung +
  Referenz-Rewiring; die gemessene Verteilungs-Karte steht in
  `mechanische-reste`). Nicht halb ausgeführt.
- **Step-5-Folge** — je `mirror_*`-Asset der 12 lebenden repo_tag-Releases den
  Byte-Vergleich CDN-Digest ↔ Repo-Raw messen, dann einzeln schneiden.
- **matrixmachine 769-Suite** — die fremdfreie HEAD-Messung läuft in `ci-check.yml`
  (`cargo test --release` je push, der `src/**`/`phi/**`/`tools/register/**`/`docs/**`/`Cargo.toml`
  berührt; kein branches-Filter); die Urkunden-Zeile gegen den letzten Lauf aktualisieren.
- **abfluss-trishuli** — der Abfluss-Pfeil bleibt `pending`; Entsperrung = das
  archivierte externe CSV des 08-27-Zugs — gemessen lokal:
  `/home/johannes/backup/archive/data/opencode-tmp-2026-09-01/worktree-aufraeum/dhm_bhotekoshi_stage_1h.csv`
  (+ `dhm_bhotekoshi_stage.csv`); kein Operator-Wort.
- **NOAA-NODD-Bucket-Dispositionen** je Dataset — Register-Frage (Litmus:
  url-line / Compiler-Lease / Konsument), kein Operator-Wort; bleibt offen.

## Nicht angefasst (Parallellinie)

- Uncommittet/staged der fremden Sitzung: `tools/measure/src/depthphase.rs`,
  `tools/measure/src/bin/depth_phase_fleet_probe.rs` (M),
  `docs/befund/befund-bz-laic-nsurr100.md`, `docs/befund/befund-tscaling.md`,
  `docs/befund/befund-2026-09-09-zonen-flotte-pick-hebel.md`,
  `docs/handover/handover-2026-09-09-zonen-flotte-pick-hebel.md` (untracked),
  der gestagte Umzug `zonen-flotte-folge.md → archiv/`. Fremde Live-Handovers
  (te-galileo-nsurr-folge, de441-reverifikation-pending) unberührt.
