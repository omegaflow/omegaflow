<!--
  title: Die positive Maske — Treiber hinzufügen statt Rauschen abziehen
  class: concept
  date: 2026-09-12
  sha256: 382321f6aef574a02f489861237934b45192d7726b0d9182001785f5209850f8
  status: live
  see-also: docs/concepts/die-akteure-im-boden-und-wasser.md
-->

# DIE POSITIVE MASKE — Treiber hinzufügen statt Rauschen abziehen

## Der Kern

Die negative Maske zieht Rauschen ab: Filter, Masken, Null-Abschätzung. Die
positive Maske fügt physikalischen Kontext hinzu: einen Treiber nach dem anderen
bedingen und messen, ob der Riss kleiner wird. Beides ist Messung — die negative
entfernt, die positive bindet.

## Das Instrument

Zwei Instrumente, gestuft nach der Ereigniszahl:

- **n ≈ 16 — σ-Reduktion mit Permutations-Null** (der Tiefenphasen-Riss):
  `tools/measure/src/bin/depth_phase_driver_scatter_probe.rs` +
  `tools/measure/src/driver_scatter.rs`. σ₀ = Stichproben-Standardabweichung
  der Flotten-Tiefenoffsets, frisch aus depthphase.rs gemessen; pro Treiber
  einzeln eine OLS-Regression des Offsets auf den Treiberwert, σ₁ =
  Rest-Standardabweichung, ρ = σ₁/σ₀. Die Null: ≥ 1000 Permutationen der
  Treiber↔Ereignis-Paarung; der Reduktionsanspruch trägt nur, wenn das wahre ρ
  jenseits von mean ± 2σ der ρ_null-Verteilung liegt.
- **n ≥ 32 — bedingte TE**: `src/mathematikerin/te.rs`
  (`transfer_entropy_conditional`, `_2`, `_stats_lagged*`, residuale
  Surrogate). Der Boden ist 32, eine benannte Zweierpotenz. Sie fährt bereits
  auf anderen Linien: die Tibet-Sturzflut (der geteilte Treiber Luftdruck löst
  das Scheinsignal auf), `corona_conditional_probe`,
  `galileo_floor_external_te`. Das Fahrzeug ist CPU + CI — kein GPU-Pfad: die
  Proben sind klein und offline; der GPU-Pfad (`te_compute`) gehört der
  Live-Topologie der Membran, nicht diesen Messungen.

## Das Protokoll (simpelst)

1. Der Archivar holt einen Treiber.
2. Die Mathematikerin bedingt das Echo auf ihn.
3. Die Weberin misst den Riss vorher/nachher.

Riss kleiner → der Faden war ein echter Treiber. Riss gleich → er war
irrelevant. Keine Deutung, nur Messung. Kein Anspruch, nur der Riss.

## Die Treiber-Kandidaten (billig zuerst)

- **Gestalt** — GLO-30-DEM (Compiler steht, CDN-Asset gemessen fertig
  2026-09-14, HTTP 200) und gebco-Bathymetrie: die Oberflächenform als Zeuge.
- **Magnetfeld** — INTERMAGNET/Swarm/GOCE/CryoSat (in `sources.φ` registriert):
  Stellvertreter-Kandidat für die Mantel-/Kerntemperatur (heißes Gestein leitet
  langsamer). Ein Kandidat, keine Behauptung.
- **Plattentektonik** — Slab-Geometrie (USGS Slab2): nicht registriert,
  Ernte-Kandidat.
- **3D-Geschwindigkeitsmodelle** (Tomografie) — nicht registriert, heavy Fetch
  (CI).

## Der Audit — wo wir noch nicht so exakt sind (Stand 2026-09-12)

- Echo-Tiefe: σ 19 km (Ereignisse) / 36 km (Stationen) dominiert das
  ±10-km-Gate; ak135 ist 1D (kein 3D-Modell registriert); der Stationsterm ist
  offen (II.KIV +5,69 s); pP-Mehrdeutigkeit bei Δ≈30°.
- M9.1-Picker: die Streuung bleibt — der USGS-Mww-Zentroid-Nachfolger ist
  gebaut, die Verdrahtung in die Flotte offen.
- Ephemeriden: geschlossen (2026-09-13). Der `body_fixed_to_icrs`-Matrixpfad
  trägt die IAU-Produktform Rz(90+α)·Rx(90−δ)·Rz(W) mit dem Pol aus Spalte 2
  (`src/archivar/motion.rs`, Fix `0f79b1ce`; de441 mars Anker Δ 0,0 km, die
  Vor-Fix-6045,3-km-Lesung ist zu). Erdmitte 0,0 m von de440; die 42,5 km /
  −90,5 s / +10,1 s-Drift war ein veraltetes lokales 19-MB-Erdbin (ttl-Cache),
  kein de441-Defekt; die 2024-Kanon-Δ 793,3 km war eine Suchfeld-Grenze
  (longitude 90° statt 180°), jetzt 8,4 km. Gate (Punkt < 15 km, Magnitude
  < 0,002) hält (`docs/paper/eclipse-clock-worldlines.md`).
- Galileo-ODF: Format 1 vs 2 ungemessen (kein lokales galileo_odf.bin).
