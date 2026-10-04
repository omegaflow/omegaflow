<!--
  title: Handover — Mountain-Folge 230 (Stand 2026-10-04)
  session: Mountain-Folge 230
  class: handover
  date: 2026-10-04
  sha256: 59a38e9198b865345a9a156c20b7d23e6ff7d4d89777f56af6319cc36e469b4f
  status: live
-->
# Handover — Mountain-Folge 230 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, Stand
2026-10-03T20:38Z). Diese Session konsumierte `handover-2026-10-03-mountain-folge229.md`
(→ `archiv/`).

## Burn: open 0.0000 · close 0.0469 · cap 0.5 Grund: blocked_sources-Verdikte (future-174), Witness-URLs korrigiert, Modul 203 gezogen, Kp-Feld-Fix, twomass/swarm-Register

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert … das Commit-Wort" | 2026-10-02 | Operator (Session, Mountain 221)
„braucht es max?" — pro/max nur mit benanntem Hart-Atom oder gemessener flash-Fehllage; flash-first | 2026-10-02 | Operator (Session, Mountain 222)
„ich schicke immer an omegaflow im cc damit ihr sie im ledger habt" — jeder Operator-Send trägt `code@omegaflow.space` im Cc | 2026-10-02 | Operator (Session, Mountain 225)
„Recherche-Trio (chat.z.ai GLM-5.3 Deep Search · claude.ai · Kimi K3 über tryingopen.com; Sonnet-Fallback arena.ai) = erster Kanal für scharfe Recherche" | 2026-10-02 | Operator (Session)
„1 ja bitte" — den privaten TE-Pfad entlocken (Detrend-along-p + CMI/pTE-mit-p-Kovariate), Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja voranmelde und dann lauf in ci" — Pioneer-Floor-Voranmelde-Blatt bauen, dann Lauf in CI | 2026-10-02 | Operator (river-folge82)
„braucht es pro und max?" — Reaffirmation flash-first | 2026-10-03 | Operator (Session, Mountain 227)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen, in `declined`/`dead` migrieren; blocked hält nur Gewolltes | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen, nicht declinen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dir bitte nochmal die aktuelle blocked sources ansehen?" + „… wir haben alle quellen die ich gar nicht nutzen kann doch gestrichen" | 2026-10-03 | Operator (future-folge174)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes; die Übergabe IST der Stand" | 2026-10-04 | Operator (Session, Mountain 230)

## Offen (aufgeschlüsselt)

### PETREL19 — viertes Ephemeriden-Haus (Aufnahme wartet auf Lizenz)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine Antwort von `tian-we@163.com` auf die Lizenz-Anfrage (der eigene Send ist kein Trigger) — Beleg `state/mail/mail_ledger.φ`.
- **Lage:** (gemessen 2026-10-04 via `archive_search petrel19 --root state/mail`) nur der gesendete Record `mail_ledger.φ:222`, keine Antwort. GitHub `license: null`; Arm `ephemeris_compiler.rs` steht; Dritt-Wait `state/zustand/wartend.φ` (`petrel19-license`); `phi/blocked_sources.φ` `pending` (PETREL19).
- **Blockade:** Lizenz ungeklärt.
- **Braucht:** Antwort abwarten; bei Lizenz Register-Zeile in `phi/sources.φ` + `ephemeris_house_gate`/`flyby_anderson_probe` auf das vierte Haus erweitern (`ephemeris_house_gate.rs:297-299`, heute fest `de`/`inpop`/`epm`).

### Ranging-Decode — Sequential-Arm gebaut; FRQ_UP fehlt in DT2/DT3
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) Sequential-Arm gebaut; `FRQ_UP` jetzt auflösbar: `ul_freq` liegt in DT6 (`odf.rs:1158`, Bytes 174..182) und DT7 (Bytes 282..290); neu `tnf_ranging_resolution_with_frq_up(frame,bytes,frq_up)` (F_EXC = FRQ_UP·exc_scalar_num/exc_scalar_den, TRK-2-34 note 17), verdrahtet in `tnf_format_probe --ranging`. Der Single-SFDU-`tnf_ranging_resolution` bleibt ein benanntes `None` (DT2/3 tragen kein FRQ_UP). Modul 203 in `docs/reference/810-005-203C-sequential-ranging.txt`. `cargo check`/build 0/0.
- **Blockade:** kein robuster Paarungs-Schlüssel DT6/7 ↔ DT2/3 (Zeit-Tag/rec_seq/DSS) — der Probe nimmt die erste `ul_freq` der Datei.
- **Braucht:** Paarung über Zeit-Tag/DSS (falls im ODF vorhanden) oder die Größe bleibt ein benanntes `None` ohne Paar.

### RoPeR `gras_2c` — Arm steht; 42 Quellen ohne `field`-Zeilen (Parser exact-match)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) Der `series_named("gras_2c")`-Arm ist **bereits gebaut + committet** (`extract.rs:317-324` HEAD, Test `gras_2c_named_series_carries_gate_channels_without_a_band`, Commit `9e4d688f8`) — die folge229-Lage war stale, der Baum gewinnt. Der echte 0-Kanal-Grund: `phi/sources.φ` trägt 42 `format gras_2c`-Blöcke, aber **0 `field`-Zeilen** → `main_flow.rs:3024` bricht „field undeclared" ab.
- **Blockade:** `field`-Parser ist exact-match (`parse.rs:925`, Lookup `main_flow.rs:3050`); `gras_2c_gate_*` ist so nicht ausdrückbar (2048 Gate-Felder × 42 Quellen = 86 016 Literal-Zeilen oder ein Wildcard-Feld-Arm).
- **Braucht:** Wildcard-Feld-Arm (`gras_2c_gate_*`) **oder** die Quellen-weite `range <start_m> <step_m>`-Direktive (kein Wire-Slot; Chirp 0,45–2,15 GHz wäre sonst `freq`-Fabrikation). `comp` ist der Range-Gate-Index (Zhou 2020, DOI `10.26464/epp2020054`).

### HDF4 — SZIP descoped; NBIT/SKPHUFF lesen
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) Verdikt: SZIP (CODER 5) wird **nicht** implementiert (kein std-only Dekoder, keine libaec-Dependency — der std-Stack bleibt); NBIT+SKPHUFF stehen (`src/archivar/hdf4.rs`), MOD11C2/C3 lesen über NONE/RLE/DEFLATE; gap-Token-Note in `phi/blocked_sources.φ` fortgeschrieben.
- **Blockade:** keine.
- **Braucht:** nichts — MODIS-Rest über SZIP entfällt.

### Weberin zweite Linie — astrometry-reader/curation-Arme (9 parser-def)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) 6 VizieR `asu-tsv` `J/A+A/582/A8/{ariel,miran,obero,titan,umbri,uranu}_j` (Uranus-Monde) + `occultations.ct.utfpr.edu.br` = zweite unabhängige Positions-Linie der Weberin. AST1-Modul + Compiler stehen (13 313 Samples, sha `bc21177b…`). Der **Reader-Arm ist jetzt gebaut**: `extract.rs:206` (`verify_records`), `astrometry_series_counts` (`extract.rs:216`), `main_flow.rs:3263` Fetch-Branch, `fetch.rs:1106` Bypass; `cargo check` 0/0.
- **Blockade:** kein `AstroSample`/Richtungs-Slot — weder das 26×f64-Wire noch der `SeriesRow`-Strom trägt eine bewegte (ra,dec)-Serie; AST1 wird geparst + für das Register gehalten, nie zu einem Skalar flachgeklopft. Die 2 Gaia-ADQL: `vari_classifier_result` korrigiert HTTP 200, `cluster_ka` HTTP 400 (Cluster-Tabelle außerhalb Gaia-TAP).
- **Braucht:** einen Richtungs-/`AstroSample`-Wire-Slot definieren; bis dahin `format astrometry_series` (ohne `cmap`/`field`) registrierbar (`url`/`origin`/`compiler` = Transport). **Contract-Entwurf liegt** (2026-10-04, read-only): der ankerlose Kanal (EEG) + die Richtungs-Serie (AST1) brauchen denselben fehlenden Slot — `station_code`/`name` tragen das Label, der ICRS-Rahmen bleibt absent (0 honored). Entscheid offen. Träger: `phi/blocked_sources.φ::gap:astrometry-reader ×7`, `::gap:curation ×2`.

### pradan_ch2-Reader — gebaut (ZIP+FITS), `field`-Zeile offen
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) Reader gebaut: `src/archivar/pradan_ch2.rs` + std-only `zip_members` (`src/archivar/inflate.rs`), Format in `series_parse_bin`/`series_component_name`/`main_flow` verdrahtet. Inner shape: FITS `SPECTRUM`-BINTABLE `CHANNEL`(I2)/`COUNTS`(E, count) + leere Primary; Wert = Σ`COUNTS`, t = Beobachtungsstart TDB. **Member-Korrektur:** 27 237 `.fits` + 27 237 `.xml` (nicht 54 474 Paare — die frühere Zahl war doppelt). `cargo check`/build 0/0.
- **Blockade:** die `field`-Zeile fehlt; `phi/sources.φ` ist geteilt (fremder bidsleep-Hunk), die Zeile liegt uncommittet im Baum.
- **Braucht:** `field pradan_ch2_cla_l1_counts pradan_ch2_cla_l1_counts inverse-square em count 604800 0.0 0.0` in den pradan-Block (`phi/sources.φ`) → beim Freigeben der Datei committen.

### Medizinische/Life-Science-Datenquellen — disponiert (3 gewollte als pending)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) ~90 Quellen disponiert: `phi/declined_sources.φ` trägt die Decline-Zeilen (`registry/katalog`, `health-stats`, `no-physical-force`, `imagery`, `molecular`, `aggregate-index`, `reference`, `model`, `commercial`, `registry`, `literature`) mit `--verdict`-Evidenz; `phi/blocked_sources.φ` trägt 3 gewollte Rohdaten-Quellen als `pending` — `ieeg.org`, `TUH EEG`, `NSRR`. **Weberin-Eignung (`SOURCE_PORT.md §8`, vier Punkte) nachgetragen:** die Declines scheitern an Punkt 1 (kein Kraft-/Zeugen-Träger → Oszillator-Gate, Punkte 2–4 damit gegenstandslos); die 3 gewollten tragen (1) `electric` ✓ (2) Zeitreihe ✓ (3) 4D-Anker **fehlt** (kein ICRS/Körper-Ort je Elektrode) (4) zweite Linie ✓ (OpenNeuro/PhysioNet-EEG). **Korrigiert:** kein bidsleep-Riss — `sources.φ:3415` `advective m/s²` ist Accelerometrie (`G_STANDARD`, `bidsleep_compiler.rs:14`) und korrekt; echte Lücke: OpenNeuro (`openneuro_pd_eeg`, 99 Blöcke) parst, emittiert aber **0 Kanäle** (`main_flow.rs:3175` „electrode positions carry no body frame").
- **Blockade:** für die 3 gewollten fehlt Arm+Asset; der Zugang ist Registrierung/DUA (operator-gebunden).
- **Braucht:** iEEG-/TUH-/NSRR-Zugang + Arm/Compiler/Asset; EDF-Parser liegt bereit, sobald ein Asset da ist.
- **Gebaut (2026-10-04):** OpenNeuro-EEG-Kanal-Slot — der Arm emittiert jetzt gedecimierte `electric`-V-Kanäle je Elektrode am Anker (`station_code`, ohne ICRS-/Kopf-Rahmen, kein Fabrikat; `main_flow.rs`/`openneuro_eeg.rs`, Test); die 99 `openneuro_pd_eeg`-Blöcke strahlen damit. Plus std-only EDF/EDF+-Parser (`src/archivar/edf.rs`, Header+Signale+physikalische Konversion, 4 Tests).

## Träger (Prosa, eigene)

- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`) — stehende Rätsel-Messreihe.
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`) — Voranmelde-Blatt + Lauf-Ergebnis (CI `37111508656` success).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`) — Rätsel-Register; Träger seines offenen Markers.
- `docs/concepts/tools-map.md` (`class: concept`) — Werkzeug-Karte; Träger ihrer zwei offenen Marker.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`) — Register-Inventur.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`) — `dead_sources.φ`-Relevanz-Erstpass.

## An mycelium

Origin: mountain folge230.

- **`twomass_psc` + `swarm_tec` — Reader-Arme stehen (river 86), Register-Block gehört manifestiert.** `cargo check` 0/0; `main_flow.rs:4622`/`:4680`, `extract.rs:3204`/`:3262`, `fetch.rs:1105-1106`, `src/archivar/twomass.rs`. Bitte den jeweiligen `url`/`origin`/`compiler`/`sha256`-Rahmen setzen; die Verdikt-Zeilen (Mountain) sind:
  - `twomass_psc` (Zulassung: `em`, `at sun`, `ttl 31536000`, Muster `catalog_allwise_psd`):
    ```
    format twomass_psc
    ttl 31536000
    at sun
    cmap .
    ra ra
    dec dec
    field jmag twomass_j_mag inverse-square em mag 31536000 0.0 0.0
    field e_jmag twomass_e_j_mag inverse-square em mag 31536000 0.0 0.0
    field hmag twomass_h_mag inverse-square em mag 31536000 0.0 0.0
    field e_hmag twomass_e_h_mag inverse-square em mag 31536000 0.0 0.0
    field kmag twomass_k_mag inverse-square em mag 31536000 0.0 0.0
    field e_kmag twomass_e_k_mag inverse-square em mag 31536000 0.0 0.0
    ```
  - `swarm_tec` (Compiler `swarm_tec_compiler.rs:258-262` druckt den Block; `at earth`):
    ```
    format swarm_tec
    ttl 86400
    at earth
    cmap .
    lat Latitude
    lon Longitude
    field absolute_vtec_tecu absolute_vtec_tecu inverse-square em TECU 86400 0.0 0.0
    ```
- **`quake_ptevent`-Assets (chile/tohoku/jma) unregistriert.** (gemessen 2026-10-03, river-folge87) Die kompilierten Assets liegen auf dem CDN (chile 45 B, tohoku 45 B, jma 7181 B), aber `phi/sources.φ` trägt sie nicht. Die zwei Witness-URLs sind 2026-10-04 korrigiert (Chile → FeatureServer-Layer 1, Tohoku → Layer 5; `phi/witnesses.φ`); nach dem Re-Harvest die drei Assets mit `format quake_ptevent` registrieren.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI (NSE-Daten bleiben im Haus). Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz; der Detrend-Arm trägt den steilen, der Conditional-Arm den milden Fall. `no statement`/`pending` bleiben erlaubte Ergebnisse. Der private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
