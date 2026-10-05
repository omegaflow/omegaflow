<!--
  title: Handover — Mountain-Folge 235 (2026-10-05)
  session: Mountain-Folge 235
  class: handover
  date: 2026-10-05
  sha256: 90bb35f03ebf3d62c408f52e33d047eb756a1daa6da84746b723142cf7b255c1
  status: live
-->
# Handover — Mountain-Folge 235 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-05-mountain-folge234.md` (→ `archiv/`) und
faltete die fünf offenen `## An mountain`-Blöcke (future-179, mycelium-231,
sensory-231, river-92 ×2). Gemessen (2026-10-05): der 1-min-`dB/dt`-Arm ist gebaut
(`--grain minute` in `intermagnet_dbdt_compiler.rs:238-250`); ExoMars
(`sources.φ:27024-27031`, `format pds4_acs_nir`) und Gaia-RRL
(`sources.φ:26986-26994`, `format gaia_rrl`) sind von Mycelium 231q registriert;
LEOS `descoped` (`blocked_sources.φ:122`); der superdarn-Tag ist von
`blocked account` auf `pending` korrigiert (`blocked_sources.φ:78`). Der
Operator-Punkt „braucht es pro?" ist gemessen beantwortet: kein pro — flash-first
(der Enclosure-Riss und die Kanal-Quellen sind mit `grind-flash` messbar). **Atom 2**
(Operator-Wort „messe nochmal den aktuellen zustand dann commit", 2026-10-05): die
drei neuen `## An mountain`-Blöcke (river-folge93 §Feld-Gesetz/§Zeugen, sensory-232)
gefaltet; die **FMI-GIC-fein-grain-Zeile** gebaut (`sources.φ:17202-17208`, `format
fmi_gic_1min`, Arm `--grain minute`, `cargo check` 0/0) — Asset-Dispatch bei
Mycelium; die **em-Apertur-Quellen-Identität** gemessen (sieben benannte z-Felder:
sechs halten, TNS `τ` unverifiziert) und der **Riss** benannt (die Apertur erreicht
≥25 em-Felder, nicht sieben — `sources.φ:8814-8820` bestätigt).

## Burn: open 0.003 · close 0.062 · cap 0.25 Grund: Runde flash-first — Line + 4 grind-flash (axis-Serien, Enclosure, FMI-GIC, em-Apertur) + 1 general (Listen-Verifikation), kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„kannst du bitte deine punkte erledigen?" | 2026-10-04 | Operator (future-177)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 234)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)

## Offen (aufgeschlüsselt)

### Enclosure-Hülle — Konservativität (Riss, messbar)
- **Status:** eigen
- **Trigger:** keiner — Probe jetzt baubar
- **Lage:** (gemessen 2026-10-05 via grind-flash) `law_bounds`
  (`src/archivar/spatial.rs:243-274`) sampelt nur am Epoche (1 s/2 s
  Finite-Difference `:266-272`; Sterne instantan `:257`), `Φ`-infliert
  (`types.rs:7`); das einzige Max ist über Samples (`:286-293`). Der Query-Horizont
  ist bis `64·ttl` (`spatial.rs:863`); Asteroiden tragen `resid_ema = 0.0`
  (`:403`). Die Hülle bricht für `e > (Φ−1)/(Φ+1) = 0.236`. Riss: folge234 nannte
  den Test `src/mathematikerin/tests.rs:3321`; der Baum trägt ihn in
  `src/archivar/tests.rs:3298` (`e: 0.0` `:3282` — nur der Kreis).
- **Blockade:** keine
- **Braucht:** Eine Probe je Bahnkörper über die Ephemeriden-Spanne gegen
  `Φ·(v_epoch+resid_ema)` (`dastcom::state_at` ist exakt Kepler); bricht einer,
  wandert das Bound auf ein Per-Body-Spannenmaximum.

### Nicht-point-event-Zeugen — `axis value`-Textserien
- **Status:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) `gbco`/`gmrt`/`rixs` sind
  `witness`-Blöcke (`phi/witnesses.φ:186-199`); die Form ist ein
  `#`-kommentiertes `axis value`-Paar je Zeile (`field_te_query.rs:1768-1784`).
  `gmrt` trägt nur **eine** Epoche (`gmr1`, kein zweiter Block möglich,
  `witnesses.φ:196-199`) — Riss. `gbco` und `rixs` je zwei Epochen, kein
  `axis value`-Emitter; `gbco`-Substrat `src/archivar/opendap.rs`
  (`parse_dds`/`parse_das`/`decode`), aber kein DAP2-ASCII-Parser.
- **Blockade:** kein Emitter (`gbco`/`rixs`), zweite `gmrt`-Epoche fehlt
- **Braucht:** einen neuen DAP2-ASCII-Serien-Compiler für `gbco` (Substrat
  `src/archivar/opendap.rs`, Vorbild `tools/harvest/src/bin/rixs_series_compiler.rs`)
  plus einen Serien-Compiler für `rixs`, dann die URL-Zeilen in
  `phi/witnesses.φ` auf die CDN-Serie zeigen; `gmrt` als Ein-Epochen-Riss
  benennen (descopen oder zweiten Epoch-Block finden).

### iEEG 4D-Anker — BIDS-Sidecar (Elektroden)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `https://www.ieeg.org/services` wieder 200
- **Lage:** (gemessen 2026-10-05 via grind-flash) `ieeg` **nicht** in
  `phi/sources.φ`; `blocked_sources.φ:154-156` `pending`; Arm ist
  `tools/harvest/src/bin/ieeg_compiler.rs` (REST `/services`, Kanallabels +
  Spannung, **keine** Koordinaten); `src/archivar/electrodes.rs` (BIDS
  `*_electrodes.tsv`) ist nur aus eigenen Tests gerufen; `Position::Electrode` →
  `return None` (`channels.rs:1298`); `IEEG_USER`/`IEEG_PASS` vorhanden;
  ds003844 x=y=z=0.
- **Blockade:** Server 503; kein `SourceConfig`-Sidecar-URL-Feld; kein Datensatz
  mit echten MNI-Koordinaten
- **Braucht:** `SourceConfig`-Sidecar-Feld + Arm, der `*_electrodes.tsv` +
  `*_coordsystem.json` lädt → `edf_emit_channels_with_electrodes`;
  `Position::Electrode`→ICRS in `channels.rs:1298`; Re-Dispatch bei Server 200.

### absorption-/advection-Slots (Entscheidung)
- **Status:** operator-gebunden (Vorbereitung eigen)
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-10-05, folge234) alle 9059 field-erzeugenden
  Direktiven tragen `absorption = 0.0` (Beer-Lambert `force_absorption` inaktiv,
  patch-levy-Tail kernel_id 5 unerreichbar); `advection` lebendig
  (`phi/sources.φ:181` = 400000.0); `ADVECTIVE_BASE_SPEED = 1.0`
  (`src/archivar/membrane.rs:424`).
- **Blockade:** Entscheidung
- **Braucht:** Wort — bleibt `absorption` überall 0.0 (dann ist der Kernel-Tail
  tot), und ist `ADVECTIVE_BASE_SPEED = 1.0` für advektive `field`-Zeilen ohne
  deklarierte Advektion gewollt?

### em-Apertur `(1+z)⁻²` — Quellen-Identität (Riss)
- **Status:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Die sieben genannten z-tragenden
  em-Felder: `:7088`/`:9681`/`:9692` NED, `:9579` PSZ2, `:1183` TNS, `:17694`
  sncat, `:17705` swiftgrb — sechs halten (τ = Block-`ttl`, Identität = Quelle).
  **Riss 1:** `:1183` TNS trägt `τ 3600` bei Block-`ttl 3600` (Regel gäbe
  `ttl/10 = 360`) und der `tns_compiler` zielt auf die API, nicht auf die
  statische CSV — unverifiziert. **Riss 2:** die Apertur erreicht **≥25**
  em-Felder mit `z`, nicht sieben (z. B. `sources.φ:8814-8820` rcsed, bestätigt);
  river-folge93 §Zeugen und der Baum konvergieren nicht — beide Linien benannt,
  nicht geglättet. **R2:** das 6. Feld-Token ist `tau` (`parse.rs:952`), nicht `ttl`.
- **Blockade:** keine (Messung)
- **Braucht:** für `:1183` die keyed-POST-Annahme der statischen CSV messen oder
  `τ` aus Prozesswissen setzen; den Riss (≥25 Felder) an River/Rat tragen — die
  Apertur-Anwendung auf `field z` selbst (`spatial.rs:718`) ist Rats-Entscheid.

### Field_te_query-Kanalquellen — Ernte-Arme fehlen
- **Status:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05, Operator-Liste) Quellen erreichbar, aber ohne
  Ernte-Arm: IERS EOP C04 (LOD/AAM), NOAA GODAS, Oulu-Neutron-Monitor, BOM RMM
  (MJO), HadISST/COBE (PDO/AMO), GISTEMP Vulkan-AOD, NINO3.4 (aus ERSST
  berechenbar), SOI als eigener Kanal. `field_te_query`-Wiring ist Rivers Natur;
  GOES XRS/AIA/EVE sind registriert (`sources.φ:2001-2423`), nur verdrahten.
- **Blockade:** kein Arm
- **Braucht:** je Quelle ein `tools/harvest/src/bin/*_compiler.rs` + Register-Zeile;
  NINO3.4/SOI zuerst (Substrat ERSST liegt).

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…` = Baum) — Pool disponiert (2 admit / 3 hold / ~97 declined).
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`) — `range`-Zeile trägt den Rat-Befund (kein Wire-Slot).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## An mycelium

Origin: mountain folge235.

- **EMM (Rat-Verdikt: Bildquelle ohne `field`):** `emm_sdc_compiler.rs` liegt
  uncommittet modifiziert im Baum (fremde Hand — ich habe sie nicht berührt).
  **Block für dich:** `emm-sdc-cdn.yml` angleichen (l2a, Datums-Range, Keyname,
  Asset), Frame-Bundle-Arm bauen (Format-Token braucht einen Loader-Arm wie
  `hips_png` in `main_flow.rs`), dann `format emm_exi_l2a`-Block (`at mars`, ttl) +
  Manifestation.
- **WQP-Re-Harvest:** `wqp_result.csv.zip` mit der alten Query re-harvesten (das
  alte Asset trägt die 4 langen Komponenten nicht) + manifestieren.
- **`goes_euvs`-Alt-Asset** auf `ssd.jpl.nasa.gov` ist nach dem `ncei.noaa.gov`-
  Alignment ein Orphan — `cdn_reconcile`-Disposition.
- **BGI AGrav** (`phi/blocked_sources.φ:168`) und **C9/CEEIN-Infraschall**
  (`:172`) warten auf Arm + Manifestation; C9 HTTP-only, Archiv endet ~2023-10-30.
- **Tianwen-1 MoRIC** (CDS HiPS, `blocked_sources.φ:107`) — Tree-Ernte + CDN offen.
- **ExoMars TGO ACS** — registriert (`sources.φ:27024-27031`, 231q); offen nur
  `acs-nir-cdn`-Lauf/Manifestation.
- **FMI-GIC 1-min** — Arm + Register-Zeile gebaut (`sources.φ:17202-17208`,
  `format fmi_gic_1min`, `--grain minute`). Offen: `.github/workflows/fmi-gic-cdn.yml`
  um einen zweiten Idempotenz-Check + Step `cargo run -p omegaflow-harvest
  --release --bin fmi_gic_compiler -- --out-bin fmi_gic_1min.bin --lsk
  kernels/naif0012.tls --grain minute --ci-mode` erweitern, dann
  `gh workflow run fmi-gic-cdn.yml` → `fmi_gic_1min.bin` auf Release
  `space.fmi.fi`.
- **superdarn-Tag** — auf `pending` korrigiert (Aufnehmer mycelium,
  `wartend.φ:8`); RST-Ernte über Globus offen.

## An future

Origin: mountain folge235.

- **Medizin-Pool:** ~97 `declined`; „rest auf on hold" ist als
  Oszillator-Gate-Verdikt umgesetzt. Kein erneutes Vorlegen (gewortet).
- **TUH EEG / NSRR** → `descoped`; iEEG bleibt `wartend` (Server 503), Keys da.

## An river

Origin: mountain folge235.

- **Enclosure-Riss:** Mountain baut die Probe je Bahnkörper über die
  Ephemeriden-Spanne (Offen). Rivers Query-Horizont `64·ttl` (`spatial.rs:863`)
  macht die Hülle relevant; die Probe entscheidet, ob das Bound auf ein
  Per-Body-Spannenmaximum wandert.
- **absorption/advection:** als Entscheidung vorgelegt (Offen) — trägt die
  Membran.
- **em-Apertur-Riss:** die Apertur erreicht ≥25 em-Felder mit `z`, nicht die
  sieben in river-folge93 §Zeugen genannten (`sources.φ:8814-8820` bestätigt);
  beide Linien benannt, nicht geglättet. Die Anwendung auf `field z` selbst
  (`spatial.rs:718`) ist Rats-Entscheid.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02,
  river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut,
  `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer
  Kovariat-Varianz. Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
