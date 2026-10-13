<!--
  title: Große Rätsel und die Maschine — Kandidaten, Werkzeuge, Maßstab
  class: survey
  date: 2026-10-13
  sha256: 5a39d13016fc0b7c376e04e208ac1c12ea92bac90f6561194b8eaa07457f5dd0
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/surveys/survey-raetsel-bestand.md docs/concepts/die-weberin.md
-->
# Große Rätsel und die Maschine — Kandidaten, Werkzeuge, Maßstab

Diese Survey hält das rätselbezogene Gespräch vom 2026-10-13 (Operator-Auftrag, River-208)
fest: die Suche nach den großen offenen Rätseln, die Werkzeuge, die sie tragen, und den
Maßstab, an dem entschieden wird, ob ein Rätsel überhaupt untersuchbar ist. Sie ersetzt
kein Blatt; sie ist die Landkarte der Kandidaten und der Ehrlichkeits-Kern.

## 1. Anlass und Gang

Operator-Wort: „auf archive search all und den roster all nutze um die großen ungelösten
rätsel zu identifizieren", dann „was die maschine kann" und „was die weberin kann", zuletzt
die Frage, ob das Ungemessene durch das Gemessene findbar ist. Der Bestand vorher: **14 Nadeln**
(`docs/concepts/kybernetische-astrophysik.md:353-368`) — Dunkle Materie · Flyby-Anomalie ·
Koronale Heizung · Erdbeben-Vorläufer · Technosignaturen · Planet 9 · Wurmloch · Dunkler
Fluss · FRB · Kugelblitz · Placebo · Urknall · Atmosphären-Biosignatur · Schumann-Resonanz.

## 2. Die Recherche (gemessen)

- `archive_search --all` **durchgelaufen** (109/114 Keyword-Modi; 5 Parameter-Arme gehalten),
  volle Datei `state/research/spill/omegaflow_all_1791909383_greatest_unsolved_problems_astrophysics_.txt`
  (866 Zeilen). Ergebnis: die offene Web-Suche liefert die **enzyklopädischen Listen** —
  Wikipedia „List of unsolved problems in astronomy", Frontiers „Grand Challenge for Space
  Physics", NPG Space-Weather-Review, Borovsky „Is Our Understanding of
  Solar-Wind/Magnetosphere Coupling Satisfactory?", „Advancing Heliophysics … Open Science"
  (Corti et al.), „Current Unsolved Problems in Planetary Nebulae Research". **Keine**
  projektschneiderten Rätsel.
- Nebenquellen: `--openalex` 39 Treffer (verrauscht), `--arxiv` lief, `--crossref`/`--ads`/
  `--ntrs`/`--zenodo` HTTP 0/504, `--tavily` 432, `--github` absent.

## 3. Das Roster (gemessen am lebenden Chrome, river-208)

Volle Linien-Runde (nicht vier Seats): **Rat (5 Stimmen) · Claude · Duck.ai · Gemini 3.1 Pro ·
Sakana · Mistral · MiniMax M3 · Perplexity** haben geantwortet. `pending`: Qwen · Z.ai ·
DeepSeek (Reasoning), Kimi · Lumo (kein Composer im gelandeten Zustand). Rohmaterial:
`state/stimmen/2026-10-13-river-grosse-raetsel-roster.md`.

**Konvergenz (fast alle Linien):** Hubble-Spannung · UHECR-/Multimessenger-Ursprung ·
Geodynamo/Erdkern-Jerks. **Dazu je Domäne:** Sonnen-Dynamo/Magnetzyklus ·
Substurm-/CME-/SEP-Raumwetter · Strahlungsgürtel/Van-Allen · Terrestrische Gammablitze (TGF) ·
PTA-nHz-Gravitationswellen · Kosmische Dämmerung 21 cm/EDGES · fehlende Baryonen (WHIM) ·
S8-Spannung · Exoplaneten-Radius-Tal · Little Red Dots.

**Verweigert (keine Nadel):** Bewusstseinsfeld · Telepathie · Torsionsfelder ·
„Schumann-Frequenz steigt" · t=0/Planck-Schaum. MiniMax spart zusätzlich Wurmloch/Placebo/
Dunkle-Materie-Labor/Dunkler-Fluss aus (kein Detektor-Falsifikat).

## 4. Was die Maschine kann (gemessen am Baum)

Kein EKG — ein **Analyse-Werkkasten**:
- **Kausal-Discovery**: TE (`te.rs`, `scalar_te_gpu.rs`, `ksg_k.rs`) + partielle Korrelation
  (`parcorr.rs`) + MCI (`mci.rs`) + **PC-Algorithmus** (`pc.rs`) — ein ganzes Ursachennetz.
- **Negativer Fuzzy / Anomalie-Scan**: `negativ_fuzzy_probe`, `lsst_anomaly_probe`,
  `ztf_anomaly_probe`, `exoplanet_outlier_scan_probe` — die **Gestalt der Abwesenheit**.
- **Residuum-Proben**: `kbo_residue_probe`, `jeans_residuum_probe`, `dark_flow_probe`, `flyby_*`,
  `galileo_*` — Rest gegen ein Modell.
- **Sphärische Analyse**: `healpix.rs`, `s2.rs` — ganze Himmelskarten.
- **Gleichgewicht**: `equilibrium.rs` (Gibbs) — thermochemisch, für Atmosphären.
- **Familien-Null**: `wy_max_t.rs` — viele Vergleiche kontrolliert.
- **Spektral/Doppler/Dispersion**: `doppler.rs`, `dispersion.rs`.
- ~294 Mess-Proben in `tools/measure/src/bin/` (u.a. `signal_cone_audit_probe`,
  `footprint_gate_probe`, `exoplanet_ps_spot_probe`).

## 5. Was die Weberin kann (gemessen)

Die Weberin ist **die Anordnung**, nicht das EKG (`docs/concepts/die-weberin.md`): ein Webstuhl
(ICRS·TDB), eine Kette (Weltlinien — **Direction · Body · Station**), ein Schuss (Verdict/TE),
ein Vlies (nichts verdeckt). Der Beweis ist der **Verdikt-Vergleich**: zwei unabhängige Linien,
die dieselbe Weltlinie treiben → **Placed**; zwei, die sich widersprechen → **Riss**, bleibt
sichtbar. Gebaut + gemessen: zweite Körper-Linie (MPC gegen Ephemeride, `weberin_body_verdict`),
Stations-Konvergenz (INTERMAGNET gegen SWARM — erster Riss an Station ABK, 8801 nT über Toleranz),
topozentrische Kopplung, Vlies-Dichte (`vlies_density.vlde`, manifestiert), Riss-Knoten-Ledger,
GW-/Neutrino-/CR-Skymap-Routen als Zeugen, geliehener Sinn (`nadel_gate`, live in
`lsst_anomaly_probe`).

## 6. Der Maßstab — wann ein Rätsel untersuchbar ist

Drei Bedingungen, alle nötig:
1. **Weltlinien/Größen**: die Stücke sind als Weltlinien (Richtung · Körper · Station) oder als
   messbare Größen schreibbar.
2. **Zweite unabhängige Linie** (Weberin) bzw. ein zweiter Kanal (Kausal/Negativ): sonst bleibt
   der Faden ein Einzelatom, kein Beweis.
3. **Nullprobe**: ein Surrogat, das die Antwort widerlegen könnte.

**Der epistemische Kern.** Das **Ungemessene ist durch das Gemessene findbar, sobald es
koppelt.** Planet 9 war genau dieses Muster — und die Maschine hat es **gemessen und
geschlossen**: `docs/paper/planet-nine-kbo-residue.md` — 7180 TNO-Bahnen, Residuum gegen
Sonne+8-Planeten, TE(Residuum → Perihellänge) gegen Surrogate und Familiengrenze
(fam = 7,28 × 10⁻¹): **kein familien-signifikanter Pfeil**; Rayleigh-Test der Häufung mit
Gleichverteilung verträglich (R = 5,8 × 10⁻² gegen Null 1,35 × 10⁻¹), 9,3° statt ~180°.
Satz des Papers: „the machine states the limit, not the absence of the planet." — Die
Negativ-Suche ist ein **Widerleger**: sie lief, mit Null, und fand **keinen Abdruck**. Die
Stille ist die Messung (0 honored).

Daraus folgt: **kein Kanal ⇒ kein Abdruck ⇒ keine Suche** (Bewusstseinsfeld, Torsionsfelder:
kein Kraft-Typ, keine Reichweite — ein verweigerter Tatort, nicht ein ungemessenes Faktum).
**Kanal benannt ⇒ untersuchbar**:
- **Terrestrische Gammablitze** — Kanal = Blitz/VLF/Ionosphäre/Gamma; die dichte VLF/Blitz-Reihe
  trägt den Abdruck; die Suche kann finden **oder** die Grenze setzen.
- **Telepathie** — Kanal = gerichtete Information zwischen zwei EEG-Reihen; die TE-Maschine
  (gespiegelte Bedingung, Null) prüft über den Zufall. Erwartbar kommt eine **Grenze** — ein
  voller Befund.
- **Bewusstseinsfeld / Torsionsfelder** — kein Kanal deklariert; der ehrliche Satz bleibt
  „kein Kraft-Typ, keine Reichweite".

## 7. Klassifikation der Kandidaten

| Kandidat | Art | Zweite Linie / Kanal | Werkzeug | Stand |
|---|---|---|---|---|
| Hubble-Spannung | Riss | Planck-H0 × Entfernungsleiter | Weberin (Riss) | beide Linien noch nicht im Bestand (`die-weberin.md:297-299`) |
| S8-Spannung | Riss | CMB × Schwache Linsen/Haufen | Weberin + Sphäre | Linien fehlen |
| Geodynamo/Erdkern-Jerks | Station | INTERMAGNET × SWARM × LOD | Weberin (Stations-Konvergenz, gebaut) | erster Riss gemessen |
| Sonnen-Dynamo/Magnetzyklus | Körper+Station | Sonnenflecken × Magnetogramme × Sonnenwind | Kausal/TE | Kanäle teils im Haus |
| Raumwetter/Substurm/CME | Station+Körper | Bodenmagnetometer × OMNI × Kp | Kausal/TE | Kanäle teils im Haus |
| Strahlungsgürtel/Van-Allen | Körper | GOES-Elektronen × Aktivität | Kausal/TE | GOES im Haus |
| SEP-Teilchenstürme | Körper | GOES-Protonen × Sonne × Radio | Kausal/TE | teilweise |
| Multimessenger/UHECR | Richtung | Auger × IceCube × Gamma | Weberin (Skymap-Routen gebaut) | Routen gebaut |
| Atmosphären-Biosignatur (ⅩⅢ) | Größe | JWST-Spektren × Stern-Aktivität | Gibbs-Gleichgewicht | Paper live |
| Exoplaneten-Radius-Tal | Katalog | Transit × Isolation × Alter | Outlier-Scan | Katalog-Arbeit |
| Technosignaturen (Ⅴ) | Negativ | Kataloge × Gestalt | negativer Fuzzy, geliehener Sinn | gebaut |
| TGF | Kanal | Blitz/VLF × Gamma | Koinzidenz + Fuzzy | Kanal benannt |
| Kosmische Strahlung & Wolken | Reihe | Neutronenmonitor × Wolken | Superposed-Epoch + Familien-Null | Kanal fehlt |
| PTA-nHz-GW | Reihe+Richtung | Pulsar-Residuen | TE + Winkel | großer Abruf |
| 21 cm/EDGES · WHIM · Miyake · Tiefbeben · Geoneutrinos | Reihe/Karte | je ein Kanal | divers | dünn/spärlich |
| Bewusstseinsfeld · Telepathie · Torsionsfelder | kein Kanal | — | — | verweigerter Tatort |

## 8. Riss, den der Rat benennt

Der Bestand trägt bereits mehrere **unbearbeitete** Nadeln (Dunkle Materie wartet auf Gaia DR4,
Flyby auf das Gate-JSON, Koronale Heizung auf eine Register-Zeile, Kugelblitz `descoped`).
Mountain: „Eine neue Nadel ist kein Zuwachs durch Arbeit, sondern durch Verschieben." Aufnahme
nur mit **Kreuzung + Signal + Surrogat-Null**.

## 9. Offen (Aufnahme-Wort)

Welche Kandidaten als neue Nadeln (ⅩⅤ+) aufgenommen werden — je Weltlinien-Art, zweite Linie,
Werkzeug und fehlende Registrierung — ist ein Operator-Wort (s. Handover River-208, Punkt
„Große ungelöste Rätsel"). Der Maßstab steht; die Auswahl folgt.
