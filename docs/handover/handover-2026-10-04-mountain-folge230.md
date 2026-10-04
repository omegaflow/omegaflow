<!--
  title: Handover — Mountain-Folge 230 (Stand 2026-10-04)
  session: Mountain-Folge 230
  class: handover
  date: 2026-10-04
  sha256: 140d22b16d3689f9427cb1a95922c37076ea1672eb4fabc12c9f8c662849b9a8
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
- **Lage:** (gemessen 2026-10-04) Sequential-Arm gebaut (`src/archivar/odf.rs`): `seq_ranging_component_frequency_hz`, `seq_ranging_ambiguity_resolution_m` (`f_first = F_EXC·2^-(first+2)`, `c/(2·f_first)`), `tnf_seq_ranging_f_exc_hz`; Test gegen Modul 203 Table 1 (Kanal 18, Komponente 4 = 1 032 556.981 Hz). **Gemessen:** DT2/DT3 tragen `exc_scalar_num/den` und `ul_cal_freq`, aber **kein `ul_freq`/FRQ_UP**; das steht nur in Data Type 6/7. Modul 203 liegt als `docs/reference/810-005-203C-sequential-ranging.txt` (sha `e218be61…`). `cargo check` 0/0.
- **Blockade:** der Sequential-Record (Codes 2/3) trägt die Uplink-Frequenz nicht — `F_EXC` ist aus DT2/DT3 nicht bildbar; Codes 2/3 liefern daher ein benanntes `None`.
- **Braucht:** `ul_freq` aus dem DT6/DT7-Record derselben SFDU-Kette beziehen (oder im ODF-Frame suchen) und `tnf_ranging_resolution` damit speisen; bis dahin Codes 2/3 `None`.

### RoPeR `gras_2c` — Arm steht; 42 Quellen ohne `field`-Zeilen (Parser exact-match)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) Der `series_named("gras_2c")`-Arm ist **bereits gebaut + committet** (`extract.rs:317-324` HEAD, Test `gras_2c_named_series_carries_gate_channels_without_a_band`, Commit `9e4d688f8`) — die folge229-Lage war stale, der Baum gewinnt. Der echte 0-Kanal-Grund: `phi/sources.φ` trägt 42 `format gras_2c`-Blöcke, aber **0 `field`-Zeilen** → `main_flow.rs:3024` bricht „field undeclared" ab.
- **Blockade:** `field`-Parser ist exact-match (`parse.rs:925`, Lookup `main_flow.rs:3050`); `gras_2c_gate_*` ist so nicht ausdrückbar (2048 Gate-Felder × 42 Quellen = 86 016 Literal-Zeilen oder ein Wildcard-Feld-Arm).
- **Braucht:** Wildcard-Feld-Arm (`gras_2c_gate_*`) **oder** die Quellen-weite `range <start_m> <step_m>`-Direktive (kein Wire-Slot; Chirp 0,45–2,15 GHz wäre sonst `freq`-Fabrikation). `comp` ist der Range-Gate-Index (Zhou 2020, DOI `10.26464/epp2020054`).

### HDF4 — MODIS CMG: Granulen lesen; SZIP-Codec fehlt
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) CI `36386043292`/`36320557426` zeigen gelesene MOD11C2/C3-Granulen über den Chunked-Pfad (NONE/RLE/DEFLATE, `src/archivar/hdf4.rs`); NBIT + SKPHUFF-Decoder gebaut + Tests. **SZIP (CODER 5) verweigert**: `cszip.c` bindet `szlib.h`/libaec — kein std-only-Dekoder.
- **Blockade:** SZIP-Dekoder (libaec) fehlt; ein `format`-Dispatch braucht die SDS/NDG-Auswahl (macht der Compiler).
- **Braucht:** Entscheid libaec-Dependency (oder SZIP-Granulen meiden); sonst kein MODIS-Rest.

### Weberin zweite Linie — astrometry-reader/curation-Arme (9 parser-def)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04) 6 VizieR `asu-tsv` `J/A+A/582/A8/{ariel,miran,obero,titan,umbri,uranu}_j` (Uranus-Monde) + `occultations.ct.utfpr.edu.br` = zweite unabhängige Positions-Linie der Weberin. AST1-Modul + Compiler stehen (13 313 Samples, sha `bc21177b…`). Der **Reader-Arm ist jetzt gebaut**: `extract.rs:206` (`verify_records`), `astrometry_series_counts` (`extract.rs:216`), `main_flow.rs:3263` Fetch-Branch, `fetch.rs:1106` Bypass; `cargo check` 0/0.
- **Blockade:** kein `AstroSample`/Richtungs-Slot — weder das 26×f64-Wire noch der `SeriesRow`-Strom trägt eine bewegte (ra,dec)-Serie; AST1 wird geparst + für das Register gehalten, nie zu einem Skalar flachgeklopft. Die 2 Gaia-ADQL: `vari_classifier_result` korrigiert HTTP 200, `cluster_ka` HTTP 400 (Cluster-Tabelle außerhalb Gaia-TAP).
- **Braucht:** einen Richtungs-/`AstroSample`-Wire-Slot definieren; bis dahin `format astrometry_series` (ohne `cmap`/`field`) registrierbar (`url`/`origin`/`compiler` = Transport). Träger: `phi/blocked_sources.φ::gap:astrometry-reader ×7`, `::gap:curation ×2`.

### pradan_ch2-Reader-Arm — roher ISRO-Zip ohne `format`
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** CI-Lauf `pradan-cdn.yml` mit konkretem `ch2_*.zip` (liefert ein Sample).
- **Lage:** (gemessen 2026-10-04) Inneres Format **gemessen**: der CDN-Asset-Zip `ch2_cla_l1_2025_10.zip` (254 320 207 B, sha `f0fd23d6…`) trägt **108 948 Member = 54 474 Paare** `cla/data/calibrated/<YYYY>/<MM>/<DD>/ch2_cla_l1_<START>_<END>.{fits,xml}` — FITS + XML-Sidecar je Beobachtung (kein PDS4/GeoTIFF/CSV). Basis `src/archivar/fits.rs` steht.
- **Blockade:** kein `format pradan_ch2`-Reader in `extract.rs`/`main_flow.rs`.
- **Braucht:** ein Member entpacken, die HDU-Art (Image vs. BINTABLE) und das XML-Schema fixieren, dann den Arm über `fits.rs` + XML bauen.

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
