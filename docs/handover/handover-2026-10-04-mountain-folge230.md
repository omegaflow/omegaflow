<!--
  title: Handover — Mountain-Folge 230 (Stand 2026-10-04)
  session: Mountain-Folge 230
  class: handover
  date: 2026-10-04
  sha256: 05919f8ad02f31d8e1995d5b56d6770ff03865e24ed65eb1ea3b3ee7bba45da2
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

### Ranging-Decode — Modul 203 liegt, Sequential-Port offen
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-04 via `archive_search --pdf-text`) DT2/DT3-Offsets verifiziert; der reale MRO-MAGr-Frame ist legales **Sequential-Ranging**, `ranging_composite_period` implementiert die PN-Tabelle. **Modul-Blockade gelöst:** `docs/reference/810-005-203C-sequential-ranging.txt` liegt (sha `e218be615f36…`; 203E-PDF direkt 429, 203C per Proxy textrein). Doku-Formeln (203C §2.2.1): `f_0 = f_S/2^7` (S-Band) bzw. `(221/749)·f_X/2^7` (X-Band), `f_n = f_0·2^-n`; äquivalent zur notierten Form mit **`F_EXC = FRQ_UP/32`** (S-Band; X-Band zusätzlich ×221/749). Komponenten 4–24, jede halb so tief wie die Vorgängerin; Ambiguität der tiefsten Komponente.
- **Blockade:** der Sequential-Zweig in `tnf_ranging_resolution` (`src/archivar/odf.rs:351`) fehlt — Codes 2/3 rufen die PN-`ranging_composite_period`.
- **Braucht:** Sequential-Arm in `odf.rs`: `f_first = F_EXC·2^-(first+2)`, Auflösung `c/(2·f_first)`; `FRQ_UP`/`exc_scalar` aus dem TNF-Record (DT2/DT3-Felder prüfen); Test gegen Modul 203 Table 1 (Kanal 18, Komponente 4 = 1 032 556.981 Hz). Bis dahin für Codes 2/3 ein benanntes `None`.

### RoPeR `gras_2c` — Mapper-Arm fehlt, field-Zuordnung blockiert
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) gap-Token `gras-2c` (`phi/blocked_sources.φ`): `.2C+.2CL`, `Table_Binary`, Record 16453 × 52, `Group_Field_Binary` 2048 × (R,I) f32 LE; Compiler `roper_pds4_compiler.rs` → G2CB `(t, amp, comp)`. `series_component_name` hat keinen `gras_2c`-Arm (`extract.rs:342+`) → `series_named` → `None` → 0 Kanäle. 42 `.2C`-Paare meint 42 **Datei**-Paare.
- **Blockade:** band-tragender Reader-Arm ausstehend (`gras_2c::parse_series` setzt hart `SPECTRAL_NO_BAND`, `extract.rs:244`); die Range-Achse trägt ohne Wire-Slot.
- **Braucht:** `series_named("gras_2c")`-Arm bauen; offen (a) quellen-weite `range <start_m> <step_m>`-Direktive (Chirp 0,45–2,15 GHz ist Quellen-Eigenschaft; `freq` wäre Fabrikation), (b) `field`-Zeilen `gras_2c_gate_*` in `phi/sources.φ`. `comp` ist der Range-Gate-Index, kein Polarisationspaar (Zhou 2020, DOI `10.26464/epp2020054`).

### HDF4 — MODIS CMG: Granulen lesen; SZIP-Codec fehlt
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) CI `36386043292`/`36320557426` zeigen gelesene MOD11C2/C3-Granulen über den Chunked-Pfad (NONE/RLE/DEFLATE, `src/archivar/hdf4.rs`); NBIT + SKPHUFF-Decoder gebaut + Tests. **SZIP (CODER 5) verweigert**: `cszip.c` bindet `szlib.h`/libaec — kein std-only-Dekoder.
- **Blockade:** SZIP-Dekoder (libaec) fehlt; ein `format`-Dispatch braucht die SDS/NDG-Auswahl (macht der Compiler).
- **Braucht:** Entscheid libaec-Dependency (oder SZIP-Granulen meiden); sonst kein MODIS-Rest.

### Weberin zweite Linie — astrometry-reader/curation-Arme (9 parser-def)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) 6 VizieR `asu-tsv` `J/A+A/582/A8/{ariel,miran,obero,titan,umbri,uranu}_j` (Uranus-Monde) + `occultations.ct.utfpr.edu.br` = zweite unabhängige Positions-Linie der Weberin; 2 Gaia-ADQL (`vari_classifier_result` HTTP 200 korrigiert, `cluster_ka` HTTP 400 — keine Cluster-Tabelle in Gaia-TAP). Alle 9 `blocked parser-def` (`astrometry-reader` ×7, `curation` ×2). **AST1-Modul gebaut** (`src/archivar/astrometry_series.rs`, `MAGIC b"AST1"`) + **Compiler gebaut** (`tools/harvest/src/bin/vizier_astrometry_compiler.rs`, 13 313 Samples über 6 Tabellen, sha `bc21177b…`).
- **Blockade:** der `format astrometry_series`-Reader-Arm ausstehend (`extract.rs`/`main_flow.rs` ohne Zweig; `sgrep astrometry phi/sources.φ` = 0); `Sat` trägt ohne `AstroSample`-Slot. Serien-Arm teils uncommittet in einer anderen Linie.
- **Braucht:** Reader-Arm + Registrierung; `Sat`-`AstroSample`-Slot. Träger: `phi/blocked_sources.φ::gap:astrometry-reader ×7`, `::gap:curation ×2`.

### pradan_ch2-Reader-Arm — roher ISRO-Zip ohne `format`
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** CI-Lauf `pradan-cdn.yml` mit konkretem `ch2_*.zip` (liefert ein Sample).
- **Lage:** (gemessen 2026-10-03) `pradan_ch2_compiler.rs` lädt den rohen Zip (Magic `PK`/`SIMPLE`), aber kein `format pradan_ch2`-Reader in `extract.rs`/`main_flow.rs` und keine Registrierung; ein Sample fehlt.
- **Blockade:** das innere Produkt-Format (PDS4/GeoTIFF/CSV je Payload) ist ungemessen.
- **Braucht:** ein `ch2_*.zip`-Sample über `pradan-cdn.yml` ziehen, innere Dateien sniffen, dann den `format pradan_ch2`-Arm bauen.

### Medizinische/Life-Science-Datenquellen — Disposition offen
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03, sensory-folge227) `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (sha `4ead2c97…`) trägt ~90 Quellen mit `--verdict`-Stand (Somatik/Neuro/Psyche/Biologie/Chemie/Genomik/Proteomik/Metabolomik/Strukturbiologie/Bildgebung/Register); nur OpenNeuro + PhysioNet registriert, NeuroVault `declined`. DUA-/Kosten-Zugänge an Future geroutet.
- **Blockade:** Force-Gate/Zulassung je Quelle steht aus (Menge).
- **Braucht:** Survey gegen `phi/sources.φ` halten, je Quelle Zulassung/`decline`-Grund setzen; Kandidaten vor dem Verdikt messen (`archive_search --verdict`).

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
