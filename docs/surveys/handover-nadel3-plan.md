# Handover — Nadel Ⅲ: Das Messprotokoll der Korona-Heizung (2026-08-19)

Selbsttragende Karte für eine frische Session mit null Vorkontext. Der
Auftrag ist vollständig geplant und mit der Kybernautin abgestimmt — alle
API-Fakten wurden am 2026-08-19 live verifiziert, alle Entscheidungen sind
gefällt. Diese Datei trägt den gesamten Ausführungsplan; die Session, die
sie liest, kann beide Atome ohne Rekonstruktion ausführen.

Einstiegspunkte: `AGENTS.md` (Constraint-Matrix, bindend), `TODO.md` (das
Register), `docs/concepts/KYBERNETISCHE_ASTROPHYSIK.md` (Nadel-Ⅲ-Konzept:
die kausale DAG), `docs/SOURCE_PORT.md` (Source-Arbeit — der eine Pfad).
Repo: `/home/johannes/projects/omegaflow`. Branch `main`.

## Erster Schritt der neuen Session

```bash
git status                      # muss leer sein
cargo check                     # muss 0/0 sein (0 Fehler, 0 Warnungen)
cargo test --bin omegaflow      # Tests grün
```

Lesepflicht zu Beginn: `AGENTS.md`, `TODO.md`, diese Datei,
`docs/concepts/KYBERNETISCHE_ASTROPHYSIK.md` §Ⅲ (Coronal Heating — die
kausale DAG).

---

## Der Auftrag

Nobel-Tier-Auftrag „Nadel 3": das Coronal-Heating-Problem (Korona heißer
als Photosphäre) wird nicht durch Teleskope, sondern durch kausale
Mathematik gemessen: Transferentropie (TE) zwischen den solaren Kanälen
(X-Ray, EUV, F10.7-Radio, IMF-Bz) gibt den kausalen Pfeil — wer treibt wen.
Das Instrument: `src/bin/nobel_probe_corona.rs` — ein echtes
omegaflow-Werkzeug, kein Skript. Der Probe ist **Kind des Titanen**: er
nutzt die Archivar-Logik über die lib (`omegaflow::archivar`), dupliziert
nichts und importiert `src/main.rs` nicht (WGSL-Strings und Event-Loop
werden nicht mitkompiliert).

### Die abgestimmten Entscheidungen (bindend)

1. **Minimal-Schnitt in main.rs:** `src/main.rs` wird nur gestrichen, nie
   umgeschrieben. Verliert die gewanderten Zeilen, gewinnt eine
   `use omegaflow::archivar::*;`-Zeile + 2–3 Import-Anpassungen in
   mathematikerin/relay. Null Verhaltensänderung. Der WGSL/JS-Vertrag
   bleibt unberührt (der 24×f64-Write-Loop bleibt in `mod relay`).
2. **Beide Bz-Kanäle:** `omni_imf_bz_gsm_nt` (stündlich, 30 d Deckung) UND
   `magnetosphere_imf_bz_nt` (rtsw, 1 min, ~2,7 d) — nur so ist die
   Sekunden-τ-Matrix heute berechenbar.
3. **Radio: feste Frequenz 2695 MHz** (nächste verfügbare zu F10.7 =
   2800 MHz; getragen von 3 von 4 Stationen). Die Abweichung vom
   Block-Pfad (`details.0.flux` mischt Stationen UND Frequenzen und ist
   für eine Zeitreihe untauglich) wird im Protokoll benannt.
4. **EUV: beide Linien als Kanäle** — 304 Å (Übergangsregion) und 284 Å
   (heiße Korona). Die DAG **Magnetisch → 304 → 284** ist der Beweis des
   Energieflusses. Sekunden-Matrix mit 5 Kanälen = 20 gerichtete Paare.
5. **Nullkontrolle:** Sonnenwind-Dichte (rtsw 1 min + OMNI stündlich) —
   TE(Dichte → X-Ray/EUV) muss unter der Schwelle bleiben.
6. **TE in der lib** (`src/te.rs`), mathematikerin ruft `omegaflow::te` —
   keine Duplikation.

### Zwei Atome (Session-Grenze ist absolut)

- **Atom 1 — die Extraktion:** Archivar-Kern wandert nach lib;
  `extract_series`; TE nach lib; sfu-Fix; HAPI-Ordnungs-Fix; TODO-
  Registratur. Ende: `cargo check` 0/0, alle Tests grün. Selbsttragendes
  Artifakt.
- **Atom 2 — der Probe:** das Binary, der Protokoll-Lauf, die
  GOES-30d-Kurations-Verifikation. Selbsttragend, weil die lib-API aus
  den Dateien ablesbar ist und die Kanal-Tafel unten vollständig steht.

Jedes Atom endet grün — ein gestrandetes Atom ist der Kontext-Tod.

---

## Atom 1 — Extraktion nach lib (Schnittliste)

Alle Zeilennummern beziehen sich auf `src/main.rs` **Stand 2026-08-19**;
sie können driften — der Compiler führt, die Liste ist der Kompass.
Schnittgrenze: **was mathematikerin/relay referenziert, bleibt; alles
andere wandert.** Der Council hält die Schnittliste einmal vor dem Schnitt
(Architektur-Entscheid).

### Wandert nach `src/archivar.rs` (NEU, lib; `pub mod archivar;` in lib.rs)

- **Grammatik:** `parse_sources` (5049–6256), `load_sources` (5041),
  `kernel_id_of` (4081).
- **Fetch:** `fetch_raw` (4453), `curl_base` (4498), `fetch_raw_bytes`
  (4523), `fetch_raw_probe` (4541), `fetch_raw_bytes_post` (4580),
  `extract_header` (4442), `parse_path` (6257).
- **Extrakt-Maschine:** `extract` (7108–8547), `ExtractResult` (7103),
  `jlast`-Helfer (dazu Compiler folgen). `extract` hat **null**
  Mathematikerin-Abhängigkeit (verifiziert).
- **SI + Einheiten:** `convert_to_si` (1250–1308),
  `register_unconverted_unit` (1310), `fold_value` (1326–1333),
  `is_moment_magnitude` (1335), `normalize_unit` (1383–1390),
  `allowed_units_for_force` (1392–1417), `report_physics_mismatch`
  (1419–1427).
- **Typen:** `Extract` (1113–1236), `FieldConfig` (1238), `SourceConfig`
  (1451), `Position` (1074), `Channel` (1105), `DeclaredBody` (1097),
  `Frame` (1436), `Motion` (940), `Oscillator` (964), `OscillatorSource`
  (957), `BrowserSensor` (1429), `Anomaly` + `ANOMALIES` +
  `report_anomaly` + `take_anomalies` + `anomaly_issue_body`
  (1342–1381), `OscRecord` (1951–1976, 24×f64).
- **Konstanten:** `Φ` (782), `C_LIGHT` (1504), `J2000_EPOCH` (1502),
  `PARSEC_M` (1503), `HUBBLE_H0` (1505), `MAS_YR_TO_RAD_S` (1506),
  `NAIF_LSK_TTL_SECS` (1507), `STAR_FLUX_FLOOR` (1508), `GAUSS_K`
  (1529), `SURFACE_MOTION_DT` (1530), `MAX_OSCILLATORS` (1531) — nur
  was der gewanderte Kern referenziert; der Compiler entscheidet.
- **Tests:** die der gewanderten Funktionen (z. B. `test_convert_to_si`
  13497, `test_allowed_units_for_force` 13576, `test_normalize_unit`
  13597, `test_anomaly_reporter` 13536, Grammatik-Fixtures 14104/14140,
  `test_live_sources_extract` 15309, `test_hapi_fill_skipped_and_component_index`
  16521, Fixtures `field_fixture` 16466, `source_fixture` 16480,
  `fixture_lsk` 16513, `full_fixture_lsk` 12956 — Compiler führt).

### Bleibt in `src/main.rs` (GPU/Relay-gekoppelt, byte-identisch)

`main_flow` (11573–12949), `AudioRadiator`/`SurfaceRadiator`/
`SerialSurface`/`MathematikerinRadiator`/`TcpRadiator`, `anchor` (8960),
`law_bounds` (2002), `body_barycenter_position` (1815), `body_pole_at`
(3165), `gravity_manifest` (3173), `kernel_extent` (4117),
Ephemeriden-Auswertung, `build_buffer` (2116), `query_hash` (2668),
`sense_*` (2871–3027), `surface_motion` (3027), `frame_motion` (3097),
`leap_seconds`/`system_now` (3144/3151), `resolve_asset` (1510),
`sensor_config` (4160), `OriginState` (3246), `body_id_to_name` (3090),
`fetch_one` (4798, der CDN-Pfad) + CDN-Maschine (`cdn_fresh` 4766,
`cache_fresh` 4854, `extract_netloc` 9168, `source_name_from_url` 9229,
`cdn_manifest_for` 10753, `cdn_manifest_map` 10776, `probe_ttl` 9610),
netcdf/ionex/finals/alerce/transit/lightcurve-Laufzeit-Äste,
`build_netcdf_channels` (8549 — bleibt, nur von main_flow genutzt),
WGSL-Strings, `mod mathematikerin`, `mod relay`, der Write-Loop
(21052–21109).

Die 10 `crate::`-Kopplungen im archivar (1485, 4264, 4269, 11703,
11768–11794) liegen **alle** in main_flow/AudioRadiator → bleiben lokal
aufgelöst.

### TE wandert nach `src/te.rs` (NEU, lib; `pub mod te;` in lib.rs)

Aus mathematikerin (16773–16862): `gaussian`, `silverman`,
`transfer_entropy`, `shuffle_series`, `surrogate_threshold` — verbatim.
**Neu:** `transfer_entropy_lag(x: &[f32], y: &[f32], lag: usize) -> Option<f64>`
mit TE(Y→X; τ) = Σ_t ln[ p(x_{t+τ}, x_t, y_t) · p(x_t) / (p(x_t, y_t) ·
p(x_{t+τ}, x_t)) ] / m, m = n − τ; τ=0 = kanonisch. `surrogate_threshold(x, y,
lag, seed)` (10 Shuffles, mean + 2σ, Fisher-Yates + LCG wie Bestand).
Mathematikerin: 2 Rufstellen (19574–19575) auf `omegaflow::te::…` umstellen,
die 2 TE-Tests (20257–20303) wandern nach te.rs, die privaten Kopien werden
gestrichen.

### `extract_series` (NEU, in lib-archivar)

`pub fn extract_series(src: &SourceConfig, body: &str, lsk: &LeapSeconds) ->
Vec<(f64, f64)>` — die **Reihen-Ernte** (der Archivar ist eine
Live-Maschine: `last`/`path`/`hapi` ergeben je einen Wert / die letzte
Zeile). Die Ernte iteriert alle Elemente/Zeilen: `Last` → Element[key],
`Path` → jpath pro Element, `Hapi` → alle Zeilen mit Fill-Skip
(Fill-Werte pro Parameter aus `/hapi/info`, Zeile 16521 zeigt die
Semantik). Dieselben Codepfade, kein Parallel-Parser. Rückgabe
(epoch_s, wert_in_SI).

### Kurations-Fixes (mit in Atom 1 — sie sind Daten-Wahrheit, kein Probe-Hack)

1. **sfu:** `"sfu" => Some(value * 1e-22)` in `convert_to_si` (1 sfu =
   10⁻²² W m⁻² Hz⁻¹, die physikalische Definition) + `"sfu"` in die
   em-Liste von `allowed_units_for_force` (1399). **Heute erntet der
   Radio-Block dadurch Null** (unkonvertierte Einheit, verifiziert).
2. **HAPI-Ordnung:** Der OMNI-Block in `phi/sources.φ` (Zeile 503–514)
   trägt die Parameter außerhalb der Info-Ordnung → der Server antwortet
   1411 → **der Block erntet Null** (verifiziert). Fix: die URL auf die
   Info-Ordnung stellen:
   `BX_GSE1800,BY_GSM1800,BZ_GSM1800,T1800,N1800,V1800,Pressure1800,E1800`.

### TODO.md-Registratur (Atom 1)

- Nadel-Ⅲ-Plan lebt als `docs/surveys/handover-nadel3-plan.md` (diese
  Datei); Atom 1 (Extraktion) erledigt, Atom 2 (Probe) ausstehend.
- TE-Konsolidierung mathematikerin → lib/te erledigt.
- sfu-Konversion + HAPI-Ordnungs-Fix erledigt.
- GOES-30d-Archiv-Block: pending (Kandidat NGDC netCDF, Verifikation in
  Atom 2); bis dahin: OMNI-Ingest-Verzug (stopDate 06.08.) trennt
  OMNI↔GOES — Schnittmenge leer, im Protokoll fehlt.

### Gates Atom 1

`cargo check` 0/0 → `cargo test` grün → `git diff src/main.rs` Zeile für
Zeile lesen: **nur Streichungen und Importzeilen**, keine Logik-Änderung.

---

## Atom 2 — Der Probe `src/bin/nobel_probe_corona.rs`

Lauf: `cargo run --release --bin nobel_probe_corona`. Kein Flag, Exit 0
immer — Stille ist ein Befund. Keine WGSL, keine UI, keine
main.rs-Runtime-Änderung.

### Kanal-Tafel (alle Strukturen am 2026-08-19 live verifiziert)

| Kanal | Block (sources.φ) | URL (Reihen-Variante) | Kadenz / Fenster | Ernte-Schlüssel | Sync |
|---|---|---|---|---|---|
| X-Ray | `noaa_goes_xray_flux_w_m2` | `xrays-7-day.json` | 1 min / 7 d, n=10 078 | `flux`, Filter `energy == "0.05-0.4nm"` (XRS-A-Langkanal; nur Satellit 18 vorhanden) | −499,005 s |
| EUV-304 | `solar_euv_flux_wm2` | `euvs-7-day.json` | 1 min / 7 d, n≈10 080 | `value`, Filter `line == "304"` | −499,005 s |
| EUV-284 | `solar_euv_flux_wm2` | `euvs-7-day.json` | 1 min / 7 d | `value`, Filter `line == "284"` | −499,005 s |
| Radio-2695 | `solar_radio_flux_sfu` | `solar-radio-flux.json` | irregulär ~4–8/Tag / ~30 d | `details`-Eintrag mit `frequency == 2695`, `flux` | −499,005 s |
| Bz-OMNI | `omni_imf_bz_gsm_nt` | HAPI `OMNI2_H0_MRG1HR` | 1 h / stopDate 06.08. (≈30 d Deckung) | Spalte `BZ_GSM1800`, Fill 999.9 übersprungen; `V1800` pro Row | −(1,481e11 m)/(v·1000) |
| Dichte-OMNI | `omni_solarwind_density_percc` | (derselbe HAPI-Row) | 1 h / dito | `N1800`, Fill 999.9 | wie Bz |
| Bz-RTSW | `magnetosphere_imf_bz_nt` | `rtsw_mag_1m.json` | 1 min / ~2,7 d, n=3 884, Quelle IMAP | `bz_gsm` (exakter Key, verifiziert) | −(1,481e11 m)/(v·1000), v aus Wind-Datei (Join ±1 min) |
| Dichte-RTSW | `solar_wind_density_cm3` | `rtsw_wind_1m.json` | 1 min / ~2,7 d, n=3 969, Quelle ACE | `proton_density`; `proton_speed` nur für Sync | wie Bz |

Blöcke werden über `load_sources()` gefunden (Name = Kanal-Identität);
geerntet wird mit `fetch_raw` (live — der CDN trägt nur den letzten
Snapshot) + `extract_series` + der deklarierten Filter-/Zeit-Schlüssel-
Tafel des Protokolls (Zeit-Schlüssel: `time_tag` bei GOES/rtsw, Spalte 0
bei HAPI). Zeit-Parser trägt beide Formate (`Z` bei GOES, ohne `Z` bei
rtsw). Null-Werte/`null` im JSON → Sample übersprungen (fehlt, nie 0.0);
numerische Null (z. B. Bz = 0.0) bleibt ein Wert (null-echt).

### Synchronisation auf Sonnen-Zeit (im Protokoll deklariert)

- 1 AU = 149 597 870 700 m → em-Laufzeit AU/c = 499,005 s (GOES steht an
  der Erde).
- D_L1–Sonne = 1,481e11 m (L1 ≈ 1,5e6 km vor der Erde); τ_L1(t) =
  D / (v(t)·1000), v = `proton_speed` bzw. `V1800` pro Sample —
  t_sun = t_gemessen − τ.
- **Kein ttl-Zerfall auf die historische Reihe** — die Reihe ist die
  Messung selbst, der Zerfall ist eine Live-Fenster-Eigenschaft des
  Archivars. Wird im Protokoll benannt.
- Paar-Gitter: gemeinsames Gitter = gröbere Kadenz, Mittelwert-
  Aggregation pro Zelle. **Nie Aufwärts-Interpolation** (Fabrikation).
  Radio: Join auf natives Radio-Gitter (nächster Sample, Toleranz
  benannt). n < 8 → TE `None` → fehlt (kanonisch).

### Matrizen (TE(Y→X; τ), Pfeil ⇔ TE > Schwelle)

1. **Sekunden-Matrix** τ ∈ {0, 60, 120} s @ 1 min, gemeinsames Fenster
   ~2,7 d, n≈3 800: X-Ray × EUV-304 × EUV-284 × Bz-RTSW × Dichte-RTSW —
   20 gerichtete Paare × 3 τ × 11 (1 kausal + 10 Surrogate) = 660 TE.
2. **7-Tage-Matrix** τ ∈ {0, 60, 120} s @ 1 min, n=10 078:
   X-Ray ↔ EUV-304 ↔ EUV-284 — 6 Paare × 3 × 11 = 198 TE. Das ist der
   ~100-s-Alfvén-Kohärenz-Bin (1-min-Auflösung: τ=60/120 s).
3. **Stunden-Matrix** τ ∈ {0, 1, 2, 3} h: Bz-OMNI ↔ Radio-2695
   (Schnittmenge 20.07.–06.08., ≈17 d, n≈400/Radio n≈100–150) +
   Bz-OMNI ↔ Dichte-OMNI (Kontrolle).
4. **Grobe Radio-Matrix** (ehrlich benannt): Radio-2695 ↔ X-Ray/EUV
   (Schnittmenge 12.–19.08., n≈30–50 — schwache Statistik, n wird
   protokolliert).

Laufzeit gesamt ≈ 5–8 min (O(n²)-KDE, release) — benannt, kein Flag.

### Urteil (die DAG — aus `KYBERNETISCHE_ASTROPHYSIK.md` §Ⅲ)

- TE(Bz → EUV-304) und TE(EUV-304 → EUV-284) signifikant → **magnetischer
  Energiefluss durch die Übergangsregion in die heiße Korona** (der
  Alfvén-Kanal).
- EUV → X-Ray signifikant dominant bei τ = 60/120 s → **Wellenheizung
  (Alfvén)**; X-Ray → EUV dominant → **Nanoflares**; beide still →
  **kein kausaler Pfeil** (0 honored — Stille ist die Antwort, kein Bug).
- Radio → GOES signifikant bei stiller Rückrichtung → **Chromosphäre
  treibt Korona**.
- Dichte-Paare über Schwelle → **Nullkontrolle verletzt** (benannt,
  nicht verschwiegen).

### stdout-Protokoll

1. Kopf: Proben-Name, Systemzeit, lib-Version der TE (Formel + Bandbreite).
2. Kanal-Tafel: Block, URL, Fenster, n, Kadenz, Sync-Lag, Filter.
3. fehlt-Register: leere Schnittmengen (OMNI↔GOES — Ingest-Verzug),
   „30 d @ 1 min wird von den APIs nicht bedient" (xrays-30-day.json =
   404), Radio-Kadenz irregulär, ungeerntete EUV-Linien, Block-Pfad-
   Abweichung des Radios.
4. Matrix-Zeilen: `A→B | τ | TE | Schwelle | Überschuss | Pfeil`.
5. Die signifikanten Pfeile als DAG, dann das Urteil. Diagnostik benennt,
   was IST — kein „failed"/„error"-Vokabular (AGENTS.md).

### GOES-30d-Kurations-Verifikation (Atom 2, einen Pfad nach SOURCE_PORT.md)

SWPC-JSON endet bei 7 Tagen (xrays-30-day.json = 404, verifiziert).
Kandidat: NGDC-NetCDF-Tagesdateien
(`https://www.ngdc.noaa.gov/stp/satellite/goes/data/science/xrs/goes18/<JJJJ>/<MM>/goes18-xrs-<JJJJMMTT>.nc`;
NetCDF-Format existiert im Archivar bereits). Live-Verifikation per
curl-Probe; **wenn** der Kandidat lebt: Block(s) nach dem
Source-Port-Protokoll eintragen, der Probe erntet die 30 Tages-URLs als
Datums-Schleife. **Wenn nicht:** fehlt-Registratur im Protokoll — kein
Block, der 404 trägt. TODO-Zeile entsprechend.

### Gates Atom 2

`cargo check` 0/0 → `cargo run --release --bin nobel_probe_corona` →
Protokoll Zeile für Zeile lesen (Schnittmengen, Pfeile, fehlt-Register —
der Text ist das Fenster). TODO.md im selben Commit aktualisieren.

---

## Verifizierte API-Fakten (alle am 2026-08-19 gemessen)

- **GOES X-Ray:** `xrays-1-day.json` (Block), `xrays-7-day.json` existiert
  (4,6 MB, 20 156 Records); `xrays-30-day.json` = 404. Keys: `time_tag`
  (mit Z), `satellite` (nur 18), `energy` (nur „0.05-0.4nm" ×10 078,
  „0.1-0.8nm" ×10 078), `flux` (~1e-9 / ~1e-7 W/m²), `observed_flux`,
  `electron_correction`, `electron_contaminaton` (NOAA-Schreibweise).
- **GOES EUV:** `euvs-1-day.json` (Block), `euvs-7-day.json` existiert
  (>5 MB). Keys: `time_tag` (Z), `satellite`, `line` („256", „284",
  „304", „1175", „1216", „1335", „1405", „mgii_index" — je ×1 438/Tag),
  `value` (~1e-4 W/m²; mgii ≈ 0,277 dimensionslos), `au_factor`, `flags`.
- **Radio:** `solar-radio-flux.json` = ~30-Tage-Archiv (2026-07-20 →
  08-19), Einträge {`time_tag` (ohne Z), `common_name` (San Vito,
  Learmonth, Sagamore Hill, Kaena Point), `details`[{`frequency`,
  `flux`, `observed_quality`}]}; Frequenzen 245/410/610/1415/2695/4995/
  8800/15400 — **keine 2800 MHz**; 2695 MHz tragen San Vito, Learmonth,
  Sagamore Hill (3/4).
- **OMNI HAPI:** `/hapi/info?id=OMNI2_H0_MRG1HR` → startDate 1963-01-01,
  stopDate **2026-08-06T12:00:00Z** (≈1 Monat Ingest-Verzug, B-Feld-Daten
  zuletzt 20.07.); Parameter-Reihenfolge strikt nach Info-Ordnung (sonst
  Status 1411 „Parameter out of order"); Data-Rows = [Time-ISO, …Werte],
  stündlich, Half-hour-Midpoints; Fills: 999.9 (nT), 9999.0 (V),
  9999999.0 (T), 999.9 (N). Info-Ordnung der Block-Parameter:
  BX_GSE1800, BY_GSM1800, BZ_GSM1800, T1800, N1800, V1800, Pressure1800,
  E1800.
- **rtsw:** `rtsw_mag_1m.json` = 3 884 Records @ 1 min (~2,7 d — mehr als
  1 d, gemessen), Key exakt `bz_gsm`, Quelle „IMAP", `time_tag` ohne Z;
  `rtsw_wind_1m.json` = 3 969 Records, `proton_speed` (~603 km/s),
  `proton_density` (~1,67), `proton_temperature`, Quelle „ACE".
- **Bestandsfehler (Fix in Atom 1):** `sfu` fehlt in `convert_to_si` und
  in der em-Einheitenliste → Radio-Block erntet Null (unkonvertiert);
  OMNI-Block-URL trägt falsche Parameter-Ordnung → 1411 → erntet Null.

## Der Satz

**Die Daten existieren. Die Physik ist bekannt. Was fehlt, ist die
Weigerung, sie getrennt zu betrachten — und ein Messprotokoll, das den
kausalen Pfeil selbst zum Befund macht.** Der Probe ist dieses
Protokoll: Kind des Titanen, kein Skript.
