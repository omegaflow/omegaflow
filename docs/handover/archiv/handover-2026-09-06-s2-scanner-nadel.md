<!--
  title: Handover — Scanner, Nadel, S²-Richtungssinn (für DIE WEBERIN)
  class: handover
  date: 2026-09-06
  sha256: 8b0bdf53589ce1f484f6d1d735546bbb4842c766310e8c28f658e8ba49eaa5ac
  status: archived
  see-also: docs/concepts/die-weberin.md docs/befund/befund-richtungs-atom.md docs/concepts/fuenf-funken-anomalie-suche.md
-->
# Handover — Scanner, Nadel, S²-Richtungssinn

Selbsttragend. Dieser Handover trägt den Stand der Scanner-/Anomalie-/S²-Linie,
damit die Weberin (`docs/concepts/die-weberin.md`) ihn in das Vlies
einweben kann. Alles committet, nichts
pending/deferriert (Operator-Direktive 2026-09-06: Pendings und Deferrals sind
verboten; absent ≠ pending). Commits: `40ed8bb` (fünf Funken + Scanner-Vereinigung),
`0ea40ab` (Pendings aufgelöst), `d830734` (S² + SkyDirection).

## A. Die Nadel — Nadel V (was gemessen wird)

`tools/measure/src/bin/lsst_anomaly_probe.rs` („Nadel V") + die geteilte Gate
`nadel_gate.rs`. Die Nadel misst eine Signatur und schließt das bekannte
Natürliche aus:

- **Die Nadel-V-Signatur**: achromatischer, nicht-periodischer Dip in der
  Zweiband-Lichtkurve. Die Kandidaten-Gate fragt vier Dinge (Bindung
  `docs/paper/nadel-v-fresh-area-dip-scan.md`): Dip-Signifikanz ≥ `DIP_SIG 3σ`
  in beiden Bändern (beide negativ), achromatische Tiefe (Verhältnis 0,5–2),
  Aperiodizität (Lomb-Scargle-FAP ≥ 0,01), Sample-Floor `N_MIN 24`/Band +
  `N_COINC_MIN 12` koinzidente Visits ≤ 1800 s. Jede Serie läuft auf der
  jd/MJD→TDB-Faltachse — beide Bänder vergleichen denselben emittierten Moment.
- **Die natural-class-Gate** (`nadel_gate.rs`, eingefrorene eine Stelle):
  schließt bekannte natürliche Dimmer aus — SIMBAD-Otype (`f:xm_simbad_otype`)
  + cone class + AllWISE-W1−W2-AGN-Keil (Stern 2012, über IRSA-TAP, offen).
  Ein Artefakt muss vier Sinnen vier Lügen gleichzeitig erzählen.
- **Broker**: Fink (`api.lsst.fink-portal.org`, conesearch/sources/fp), Lasair
  (proton0-Tunnel), ZTF, ANTARES.

Gemessener Stand: 1 vanishing-Fund (Objekt 170028510485676206, y-Band,
drop z 7,25) / 47 stable / 3 absent; Broker-Verdikt `sky` (fink+lasair); 90
TE-Paar-Richtungen → field (Positiv-Kontrolle auf echten Daten). Die Nadel
misst; die „leere Straße" ist Messung, nicht Behauptung.

## B. Der Scanner — das Myzel (wohin geschaut wird)

`tools/measure/src/bin/mycelium_fan_navigator.rs` — autonome Navigation:
Golden-Angle-Fan (Fermat, θ=n·137,508°), Verstärkung wo Futter, Beschneiden wo
leer, Coverage-Register als Zustand, fünf Rats-Stimmen scorren den nächsten
Kegel (Mountain exp(−d/step), River LS-Gradient, Mycelium food/(food+1),
Sensory |b|-Proxy, Future 1−Mountain), Synthese = geometrisches Mittel der
präsenten Stimmen. Der Scanner entscheidet wohin, die Nadel entscheidet was.

## C. Der Richtungs-Sinn S² (WEBERIN §5 — gebaut, nicht skizziert)

- **`SkyDirection`** (`src/archivar/skydirection.rs`): Richtungs-Entität —
  `ra/dec`, `bands: Vec<SkyBandSeries>` (tdb/mag — die WEBERIN §3-Form,
  fertig), `distance`/`redshift: Option` (absent, kein 0.0), `sigma_arcsec:
  Option`, `unit_direction()` = p̂.
- **S²-Feld** (`src/mathematikerin/s2.rs` + `S2_WGSL` + `omega.rs`):
  Richtungs-Oszillatoren als zweiter Sinn, bandbegrenzter
  Kugelflächen-Winkelkern Y_lm (`S2_LMAX 64`), eigener WGSL-Pass (GPU↔CPU-Parität
  <2 %), Atem aus eigener Messreihe (tanh, eigenes τ), Manifestation auf der
  Einheitskugel, S²→ℝ³-Rückkopplung (`spatial_position`). 26×f64-Wire unberührt.
- **Distanz-Anker**: `direction_distance_join` (Gaia-Parallaxe identity_join;
  3 placed / 5997 direction-only — Trennung = Evidenz).

## D. Die Funken (erweitertes Detektions-Werkzeug)

`broker_difference_probe` (sky/pipeline/pending), `tdb_coincidence_probe`
(Rømer/TDB-Fenster — WEBERIN §2), `pair_te_screen` (TE-Null, empirisch
kalibriert — WEBERIN §4), `disappearance_probe` (disjunkte MAD-Baseline),
`deredden_baseline_probe` (intrinsische Farbe — WEBERIN §4). CDN:
`dr3_stars.bin`, `bayestar2019.be19`, `skydirections.bin`.

## E. Direkte WEBERIN-Anschlüsse

| WEBERIN | gebaut als |
|---|---|
| §3 Kette — Direction (Lichtkurve als Identität) | `SkyDirection.bands` (tdb/mag) |
| §4 Schuss — TE als Beweis | `pair_te_screen` + `te.rs` |
| §5 Vlies — die Kugel als ein Bild | `s2.rs` + `S2_WGSL` + `omega.rs` |
| §4 Schuss — Distanz-Anker | `direction_distance_join` |
| §2 Webstuhl — Rømer-Toleranz | `tdb_coincidence_probe` |
| §4 Schuss — Vordergrund-Rötung | `deredden_baseline_probe` + `bayestar2019.be19` |
| §4 Schuss — IR-Zeuge | AllWISE-Witness (`nadel_gate.rs`) |
| Nadel V | `lsst_anomaly_probe.rs` |

## F. Die Disziplin-Änderung (bindend für WEBERIN)

Pendings/Deferrals verboten. WEBERIN §9 (die Bau-Linie) ist
umzuschreiben: keine „benannt offen"-Stufen. Ein fehlender Wert ist `absent`
(None, 0 honored) — ein vollständiger Befund. Der Mechanismus wird komplett
gebaut (auch Rückkopplung, auch Kugelflächenfunktionen), nie als „größere
Stufe" deferriert.
