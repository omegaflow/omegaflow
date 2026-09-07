<!--
  title: Survey — Scan: /tmp/opencode (was ist kanonisch zu behalten)
  class: survey
  date: 2026-09-07
  sha256: 03b89a467a6f2abc2f482ba9d87230139a4f2cb4e591af934967009751496ab7
  status: live
  see-also: docs/concepts/docs-naming.md docs/concepts/archivar-mathematikerin.md docs/SOURCE_PORT.md
-->
# Survey — Scan: /tmp/opencode (was ist kanonisch zu behalten)

Datierter Snapshot der Erkundungsfläche `/tmp/opencode` (Arbeits-Sandkasten der
Harvest-/Probe-/Befund-Sitzungen 2026-09-06/07). Frage: welche geharvesteten
Dateien und Tools gehören dauerhaft kanonisch ins Repo oder als dauerhafte
Rust-Werkzeuge?

## 1. Befund in einem Satz

Der kanonische Code dieser Sitzungen steht bereits im Repo (Compiler in
`tools/harvest/src/bin`, Probes in `tools/measure/src/bin`, Module in
`src/archivar`, Befunde in `docs/befund/`). Fast alles unter `/tmp/opencode` ist
transienter Probe-Stdout, Scratch-Code, Roh-Download oder Arbeitskopie — und
gehört nach omegaflow-Konvention nicht ins Git (fertige Datensätze leben als
CDN-Assets + `phi/sources.φ`-Registrierung; `phi/pipeline/` ist gitignored).
Die dauerhaften Restposten sind wenige Register-/Fixture-/Ein-Lib-Entscheidungen,
keine Datei-Verschiebung.

## 2. Gemessen: die kanonischen Orte

- Fertige Datensätze: CDN `github.com/omegaflow/sources` (release-Tag = netloc),
  registriert als `url`-Zeile in `phi/sources.φ`; lokale Arbeitskopien gitignored
  unter `data/<netloc>/<datei>`.
- Zwischenstufen: `phi/pipeline/` (gitignored): `queue/`, `stage/`, `park/`,
  `research/agent_output/`.
- Registers (versioniert): `phi/sources.φ`, `phi/dead_sources.φ`,
  `phi/blocked_sources.φ`; Arbeitszustand: `phi/pipeline/ledger.φ`, `index.φ`.
- Befunde `docs/befund/`, Ein-Blatt `docs/blatt/`, Papers `docs/paper/`, datierte
  Snapshot-Surveys `docs/surveys/survey-YYYY-MM-DD-*.md`.
- `data/` = gitignored lokale Arbeitskopie finaler Datensätze, gegliedert
  `data/<netloc>/<datei>`; `cache/` = Archivar-Cache
  (`cache_root()`, `OMEGAFLOW_STATE/archivar_cache` sonst `cache/`).
- `archive-root` (`/home/johannes/backup/archive-root/`) = externes Alt-Archiv
  (`archeology/` pre-CDN + `phi-research/`), kein Ablageort aktueller Harvests.

## 2a. Ablage-Ort der zu behaltenden Daten (Operator-Wort 2026-09-07)

Zu behaltende geharvestete Daten gehören nach `data/` (finale Datensätze als
`data/<netloc>/<datei>`, gitignored) bzw. `cache/` (Archivar-Cache), dauerhaft
registriert über CDN-Asset + `url`-Zeile in `phi/sources.φ`. `archive-root`
ist ausschließlich der Alt-/Forschungsbestand und trägt keine der hier
behaltenen Dateien.

## 3. Rust-Proben: nichts unverändert übernehmen

Alle `.rs`-Proben sind funktional bereits im Repo gemessen; keine ist ein
Dauer-Tool-Kandidat ohne Twin. Die überlebensfähigen Funktionen sind adoptiert:
`archive_search`, `riss_knoten_probe`, `zip_range_extract`,
`galileo_h1_receiver_regression`, `motion.rs::chebyshev_evaluate`/`bpc.rs`.

Der einzige Neu-Bau aus diesem Scan: eine geteilte Statistik-Funktion. Die
zweiseitige t-p (`t_two_p`, Incomplete-Beta `gammln`/`betacf`/`betai`) war
dupliziert in `galileo_floor_basis_drift.rs` (voll) und `galileo_h1_receiver_regression.rs`
(`gammln`). Sie lebt jetzt einmal in `tools/measure/src/stats.rs`
(`omegaflow_measure::stats`, `pub gammln`, `pub t_two_p`), beide Probes rufen sie
über `use omegaflow_measure::stats::…`; Referenzwerte als `#[cfg(test)]`
(Tafel-kritische t, df 1/2/5/6/30/40; df=1 Cauchy-Grenzfall). Messung:
`betacf` konvergiert stabil, trägt aber bei a=b=0,5, x=0,5 eine intrinsische
Unschärfe von ~2,5e-3 — geerbte committete Algorithmik, unverändert gelassen,
damit Messungen reproduzierbar bleiben; die Test-Toleranzen spiegeln die echte
Genauigkeit (keine unhaltbare "exakt"-Behauptung).

## 4. Fixtures: keine übernommen

Geprüft und verworfen (A = A, keine verwaisten Dateien):
- Repo-Tests nutzen ausschließlich In-Memory-Fixtures (`&'static str` in
  `src/archivar/tests.rs`, Arithmetik in `skymap.rs`), keine externen
  `.sky1`/`.nc`/`.bin`-Fixtures.
- `test_skymap.csv/.sky1`, `lsst_*_test_*.bin`: regenerative Probe-Outputs
  (die Probes holen reale Lichtkurven und schreiben in `tmp/`) — kein Consumer.
- `nodd/daily.nc` (NOAA-NRS-HDF5), `2024_IS52…nc` (BGR): Origin-Downloads, die
  der jeweilige Compiler per curl selbst holt.

## 5. Subdirs / Top-Level-Cluster: Wegwerf bzw. extern

| Cluster | Domäne | Befund |
|---|---|---|
| `skymap/` (627 .amon, 109 PAO*, assets) | Astroteilchen-Transienten + GW-Skymaps | Roh-Korpus der blocked-Quellen; kompiliert `.amn1/.pao1/.sky1/.s2e1` = CI-Artefakte; Registrierungs-Notizen bereits in `phi/blocked_sources.φ` (datierte gemessene Notes) |
| `argo/`, `wf/`, `velo_check/` (HFRadar/Argo) | Ozeanmessung | Sondierungs-/Download-Surface, refetchbar |
| `nodd/` | NOAA NODD | `daily.nc` = Quell-Kandidat (→ §7); Probe-Binaries Build-Abfall |
| `ncprobe/` | SuperDARN-FITACF | Zenodo-Zip → inflate → NC-Inspektion; FITACF-Compiler existiert im Repo (`superdarn_fitacf_compiler`) |
| `fink/`, `leads_fix/`, `wire/`, `ncprobe/target` | Broker/Register/Perl-Migration/Build | Scratch/überholt (Python/Perl nie ins Repo) |
| Referenzliteratur (`gal125.pdf`, `asmar`, `morabito`, `tdf_unpack.pdf`, …) | Galileo/Radio-Science | externes Material → `docs/reference/` (native Formate, kein Header); nicht nach `data/`/`cache/` (kein Messwert) |
| `galileo_*_report/stdout.txt`, `*_body.md`, `blatt_new.md` | Probe-Outputs/Entwürfe | finale Fassungen in `docs/befund/` |
| Logs, `ant_*.out`, `f/g/p/q/z.out`, Test-I/O | transient | Wegwerf |
| `sources_new.φ` | Register-Kopie | byte-identisch mit committetem `phi/sources.φ` → löschen |

## 6. Pending-Quellen aus dem Scan

- NOAA-NODD passive-bioacoustic (NRS `sound_level_metrics`, `daily.nc`):
  unregistriert, kein eigener Compiler → Register-Eintrag in `docs/TODO.md`
  (2026-09-07).
- SuperDARN-FITACF: Compiler existiert; Quelle in `phi/sources.φ` nicht unter
  den Scan-Keywords gemessen — nicht als pending behauptet.
- Transiente blocked-Quellen: in `phi/blocked_sources.φ` gemessen und notiert
  ("positions-pending, kein Oszillator"); keine neue Notiz nötig.

## 7. Was bleibt zu tun

- Keine Datei-Übernahme aus `/tmp/opencode` ins Git; zu behaltende Daten nach
  `data/`/`cache/` (§2a), dauerhaft registriert via CDN + `phi/sources.φ`.
- Roh-Korpora der blocked-Quellen (falls aufbewahrt): kompilierte Assets als
  CDN-Assets registrieren; Rohdaten nach `data/<netloc>/`.
- Quell-Entscheid NOAA-NRS (→ eigener `tools/harvest`-Compiler) — `pending`.
- Scratch-/Literaturdateien nach Sicherung nach `data/`/`docs/reference/` oder
  Löschen, sobald der Operator entscheidet.
