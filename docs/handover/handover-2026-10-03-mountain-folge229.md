<!--
  title: Handover — Mountain-Folge 229 (Stand 2026-10-03)
  session: Mountain-Folge 229
  class: handover
  date: 2026-10-03
  sha256: 66199aa94b331b74a1f9245d29465e48dcfdcc7e295f33419099486d1dcf4bc7
  status: live
-->
# Handover — Mountain-Folge 229 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-03-mountain-folge228.md` (→ `archiv/`).

## Burn: open 0.0011 · close 0.0440 · cap 0.5 Grund: adressierte Blöcke gefaltet, Ranging-/Epochen-Riss gemessen, §1-Host-Verdikte

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
„ich schicke immer an omegaflow im cc damit ihr sie im ledger habt" — jeder Operator-Send trägt `code@omegaflow.space` im Cc (Ledger-Aufnahme) | 2026-10-02 | Operator (Session, Mountain 225)
„Recherche-Trio (chat.z.ai GLM-5.3 Deep Search · claude.ai · Kimi K3 über tryingopen.com; Sonnet-Fallback arena.ai) = erster Kanal für scharfe Recherche" | 2026-10-02 | Operator (Session)
„1 ja bitte" — den privaten TE-Pfad entlocken (Detrend-along-p + CMI/pTE-mit-p-Kovariate), Lauf lokal/silent, nie CI (LOCK privat) | 2026-10-02 | Operator (river-folge82)
„ja voranmelde und dann lauf in ci" — Pioneer-Floor-Voranmelde-Blatt bauen, dann Lauf in CI | 2026-10-02 | Operator (river-folge82)
„braucht es pro und max?" — Reaffirmation flash-first; pro/max nur mit gemessener flash-Fehllage | 2026-10-03 | Operator (Session, Mountain 227)

## Offen (aufgeschlüsselt)

### PETREL19 — viertes Ephemeriden-Haus (Aufnahme wartet auf Lizenz)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von Wei Tian auf die Lizenz-Anfrage — Beleg im Mail-Ledger (Cc an
  `code@omegaflow.space`).
- **Lage:** (gemessen 2026-10-03 via `sgrep -i petrel19 state/mail/mail_ledger.φ` = 0) kein
  Antwort-Beleg; GitHub-API `license: null`; Arm `ephemeris_compiler.rs` steht; Dritt-Wait
  `state/zustand/wartend.φ` (`petrel19-license`); Verdikt-Zeile `phi/blocked_sources.φ:462`.
- **Blockade:** Lizenz ungeklärt.
- **Braucht:** Antwort abwarten (Wiedervorlage); bei Lizenz Register-Zeile in `phi/sources.φ`
  + `ephemeris_house_gate`/`flyby_anderson_probe` auf das vierte Haus erweitern
  (`ephemeris_house_gate.rs:297-299`, heute fest `de`/`inpop`/`epm`).

### MESSENGER-Probe vs Haus-Gate — Riss ist die Epochen-Naht, nicht das Verzeichnis
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `ephemeris-house-gate 37134474346` @`2005-08-02 19:13:08` (MESSENGER-Perigäum, dispatched 2026-10-03).
- **Lage:** (gemessen 2026-10-03 via grind-flash) Die 228er-Hypothese „`ssd.jpl.nasa.gov-de/…`
  vs `ssd.jpl.nasa.gov/…`" ist **widerlegt**: `flyby_anderson_probe.rs:10-12` ==
  `ephemeris_house_gate.rs:11-13` (wortgleich `data/ssd.jpl.nasa.gov-de/ephemeris_de441_earth.bin`,
  `data/ftp.imcce.fr/…inpop…`, `data/ftp.iaaras.ru/…epm…`); ein `ssd.jpl.nasa.gov`-Ephemeriden-Bin
  existiert im Baum nicht (`de_compiler.rs:15` schreibt nach `ssd.jpl.nasa.gov-de`). Der Riss ist
  der **Epochen-Zeitpunkt**: die Probe liest 2005-08-02 **00:00 UTC** (JD 2453584.5, die
  DE/EPM-Granulat-Naht), das Gate-Ergebnis 0,1588/18,064 km entstand in `8ddd02f48` (2026-09-30)
  noch mit der JUICE-Konstante `PERIGEE_UNIX_HMS_S = 11:45:12` für alle sechs Epochen
  (`ephemeris_house_gate.rs:9`). Naht-Signatur: `de_epm` stimmt (17,914 vs 17,916), nur die
  INPOP-Paare springen.
- **Blockade:** Post-Override-Artefakt `data/flyby2/anderson-probe-2026-09-28.json` lokal absent;
  die Ephemeriden-Bins sind CI-only (`glob data/**/ephemeris_*earth*` leer).
- **Braucht:** Ergebnis des dispatched `ephemeris-house-gate` (MESSENGER, `19:13:08`) lesen;
  konvergiert `de_inpop` gegen ~22,49 km, ist `body_barycenter_position` an der
  INPOP-Granulat-Naht (`src/archivar/motion.rs:117`) der nächste Messpunkt — nie das Verzeichnis.

### Ranging-Decode — echtes Format-2/3-Sample gemessen, Decoder liefert None
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via `tnf_format_probe --ranging` gegen MRO MAGr TNF
  `https://pds-geosciences.wustl.edu/mro/mro-m-rss-1-magr-v1/mrors_0xxx/tnf/mromagr2011_359_0115xmmmv1.tnf`,
  48 940 916 B, sha256 `4caf60ec…`): 202 339 Frames, darunter **4284 Format-2** (UL-SEQ) und
  **20 Format-3** (DL-SEQ); `tnf_dt2`/`tnf_dt3` dekodieren sie, aber `tnf_ranging_resolution`
  liefert für **alle** `None`. Ursache gemessen: die dekodierten `first_comp_num=4`,
  `last_comp_num=14`; `ranging_composite_period` weist `last > 6` ab (Tabelle 2
  `docs/reference/810-005-214B-ranging.txt:712` trägt nur `b1..b6`, L(1..6)=1 009 470). Riss:
  realer Frame vs. Spec-Komponentenbereich 1..6.
- **Blockade:** ungemessen, ob die DT2/DT3-Offsets (`odf.rs:711/:712` → `bytes[164]/[165]`) für
  die MRO-MAGr-TNF-Version passen oder die Komponentenzahl real >6 ist.
- **Braucht:** die DT2-Feld-Offsets gegen das MRO-MAGr-TNF-Schema/TRK-2-34 messen (oder belegen,
  dass `first/last` nur 1..6 tragen); `tnf_format_probe --ranging` (gebaut, `tools/measure`) auf
  weitere TNF-Homes anwenden.

### RoPeR `gras_2c` — Mapper-Arm fehlt, field-Zuordnung blockiert
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) Das Schema steht als gap-Token `gras-2c`
  (`phi/blocked_sources.φ:18`): `.2C+.2CL`, `Table_Binary`, Record 16453 × 52,
  `Group_Field_Binary` 2048 × (R,I) f32 LE; Compiler `roper_pds4_compiler.rs` → G2CB
  `(t, amp, comp)`. `series_component_name` hat **keinen** `gras_2c`-Arm (`extract.rs:342+`),
  `series_named` → `None` → 0 Kanäle, unabhängig von jeder `field`-Zeile. Die 228er-Angabe
  „42 `.2C`-Paare" meint 42 **Datei**-Paare (42 `.2C` + 42 `.2CL`), nicht 42 Paare je Record.
- **Blockade:** band-tragender Reader-Arm fehlt (die range-Dimension ist nicht darstellbar;
  `gras_2c::parse_series` setzt hart `SPECTRAL_NO_BAND`, `extract.rs:244`).
- **Braucht:** `gras_2c`-Reader-Arm mit `(t, v, comp, freq, bin_width)` bauen (River, siehe
  `## An river`); danach die `field`-Zeile setzen.

### Dispositions-Register-Hygiene — 110 `sources.φ:<n>`-Zitate driften
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03, Mycelium 225) 1685 `note`-Zeilen über
  `dead_/declined_/blocked_sources.φ`, max_len 256 (kein Eintrag >256). **110** Notizen
  zitieren `sources.φ:<n>` (dead 5 · declined 89 · blocked 16); `sources.φ` ist grow-only
  (16233 Z.), die Zitate driften.
- **Blockade:** die mechanische Umstellung braucht das Ziel der zitierten Zeile zum
  Schreibzeitpunkt (git) — sonst droht ein falscher Kanal-Key.
- **Braucht:** je Zitat den Ziel-Netloc über den Git-Stand des Schreib-Commits messen, dann
  `sources.φ:<n>` durch den Netloc ersetzen; **nie** die Nummer nachführen.

### descoped-Stale-Zwillinge im blocked-Register löschen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03, Mycelium 225) Stichprobe der 80 `descoped`-Notizen:
  ~18 **Stale-Zwillinge** (URL bereits in `phi/sources.φ`), ~15 Feld-Drops (Gaia
  `parallax_mas`/`bp_rp`), ~29 „kein Arm", 1 kommerziell korrekt.
- **Blockade:** keiner (mechanisch, aber Einzelfall-Review).
- **Braucht:** je Stale-Zwilling die URL in `phi/sources.φ` suchen; existiert sie, den
  Eintrag löschen (`released`), sonst behalten.

## Träger (Prosa, eigene)

- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`) —
  stehende Rätsel-Messreihe; die river-folge84-Risse sind 2026-10-03 geheilt (Kanal-Keys
  statt driftender `sources.φ`-Ziffern).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`) —
  Voranmelde-Blatt + Lauf-Ergebnis (CI `37111508656` success: beide Sonden `keine Präferenz
  (Limit)`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`) — das Rätsel-Register;
  Träger für seinen offenen Marker.
- `docs/concepts/tools-map.md` (`class: concept`) — die Werkzeug-Karte; Träger für ihre zwei
  offenen Marker.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`) —
  Register-Inventur der aufgegebenen/offenen Quellen.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha
  `ac672e3e…`) — `dead_sources.φ`-Relevanz-Erstpass + Force-Gate-Verdikt; Trägerzeile
  (gefaltet aus mycelium-folge224).

## An river

Origin: mountain folge229.

- **`twomass_psc.bin` — Wire-Arm fehlt (Zulassung steht).** Gemessen 2026-10-03:
  `irsa.ipac.caltech.edu/twomass_psc.bin` **200** (`.github/workflows/cdn-health.yml:53`),
  Compiler `tools/harvest/src/bin/twomass_compiler.rs` + Modul `src/archivar/twomass.rs`
  (`MAGIC "2MPS"`, 64-B-Record, 8×f64: `ra_deg, dec_deg, jmag, e_jmag, hmag, e_hmag, kmag,
  e_kmag`; Selection `Jmag {limit}`, Erwartung J<11). **Zulassung (Mountain):** 2MASS-PSC-
  J/H/Ks-Photometrie → `em`, `ttl 31536000`, `at sun` (Muster `catalog_allwise_psd`,
  `phi/sources.φ:11596`). **Kein `format` ohne deckenden Arm** (Operator-Wort 2026-09-30):
  `main_flow.rs` hat keinen `twomass_psc`-Zweig — bitte den Reader-Arm nach
  `catalog_allwise_psd`-Muster (`main_flow.rs:4557`) bauen (`cmap .`, `ra ra`, `dec dec`,
  sechs `field`-Zeilen `twomass_{j,e_j,h,e_h,k,e_k}_mag` → `em mag`). Danach schreibt
  Mycelium `url`/`origin`/`compiler`/`sha256`.

- **`swarm_tec` — Reader-Arm fehlt (Compiler steht).** Gemessen 2026-10-03:
  `https://swarm-diss.eo.esa.int/?do=download&file=Level2daily/Entire_mission_data/TEC/TMS/Sat_A/<SW_OPER_TECATMS_2F_*.ZIP>`
  200 zip (sha `8a94f1fd…`), anonym. Compiler `tools/harvest/src/bin/swarm_tec_compiler.rs`
  gebaut+verifiziert (CDF v3 via `omegaflow::cdf::CdfFile`; 26827 Records, 0 skipped,
  bin-sha `42550ef8…`, Roundtrip identisch). **Kein `format` ohne deckenden Arm** (Operator-Wort
  2026-09-30): `main_flow.rs` hat keinen `swarm_tec`-Zweig. Bitte den Reader-Arm bauen —
  Parser-form `map .` + `lat Latitude` + `lon Longitude` (`cmap .` ist inert: `lat`/`lon` binden
  nur an Map/ProfileMap/Rows/Volume, `parse.rs:1028/:1037`), `field absolute_vtec_tecu
  absolute_vtec_tecu inverse-square em TECU 86400 0.0 0.0`. Danach Mycelium:
  `url`/`origin`/`compiler`/`sha256`.

- **`gras_2c` — band-tragender Reader-Arm fehlt (Schema + Compiler stehen).** Gemessen
  2026-10-03: das 2C-Schema ist vermessen (52 Records × 2048 (R,I)-f32-Bins; Compiler →
  G2CB `(t, amp, comp)`), aber `series_named`/`series_component_name` haben keinen
  `gras_2c`-Arm → 0 Kanäle. Ein Einzelname würde 2048 Bins je Trace auf einen Kanal kollabieren
  (range verloren); `gras_2c::parse_series` setzt hart `SPECTRAL_NO_BAND` (`extract.rs:244`).
  Bitte den Arm band-tragend bauen: `(t, v, comp, freq, bin_width)` emittieren, damit die
  range-Achse im `field`/`freq`/`bin_width`-Vertrag darstellbar ist. Danach schreibt Mountain
  die `field`-Zeile. Details: gap-Token `gras-2c` (`phi/blocked_sources.φ:18`).

## An mycelium

Origin: mountain folge229.

- **§1-Compiler-Hosts — Mountain-Verdikte (fünf Risse aus mycelium-folge225).** Keine der fünf
  Assets hat eine `url`/`format`/`compiler`-Zeile in `phi/sources.φ`; „Manifestor" ist der
  Workflow. Je Host:
  - `vizier.cfa.harvard.edu` (`camargo_uranus` → `uranus_*_probe`): **keep** — der rohe
    TSV/SPK-Spiegel ist Astrometrie-Input der measure-Probes, verschieden von der descopten
    asu-tsv-**Feld**-Registrierung (`phi/blocked_sources.φ:232-254`); der descope gilt nur für
    den Feld-Anspruch.
  - `noaa-eri-pds.s3.amazonaws.com` (`eri_imagery.bin`): **declined bleibt**
    (`phi/declined_sources.φ:5185`); `eri-cdn.yml` + `noaa_eri_compiler` (GeoTIFF → per-Pixel-Feld)
    ist der stale Manifestor — bitte Workflow + Compiler entfernen.
  - `dachs.fai.kz` (`fai_kz_obscore.bin`): **declined** (`phi/declined_sources.φ:2126` nennt genau
    dieses Asset); `fai-kz-cdn.yml` + `fai_kz_compiler`/`fai_kz.rs` stale — bitte entfernen.
  - `gsaweb.ast.cam.ac.uk` (`gaia_alerts.bin`): **keep** — Witness `s2-direction`
    (`phi/witnesses.φ:58`) ist jünger und misst den konkreten `/alerts/alertsindex`-Feed; die
    `/alerts`-Portal-decline (`phi/declined_sources.φ:2217`) gilt nur für das HTML-Portal.
  - `ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca` (`vlass_sources_se.bin`): **keep** — die registrierten
    VLASS-Assets (`vlass_tap_source/component.bin`, `phi/sources.φ:11049/:11061`) liegen unter
    `ws-uv.canfar.net`; die `/argus`-decline (`phi/declined_sources.φ:4369`) ist ein anderer Pfad,
    kein Kollisions-Riss.

- **`released`-Harvest-Duties tragen (das Register löst sie nicht als Punkt).** Die
  `released`-Quellen in `phi/blocked_sources.φ` (Zugang/Arm geklärt) tragen den offenen
  Download nur im `note`, nicht im offenen Satz. Bitte als Ernte-Punkte falten:
  `moon.bao.ac.cn` (Chang'e 1–6 GRAS), `nssdc.ac.cn` (Tianwen-1/Zhurong),
  `sdc.emiratesmarsmission.ae` (Hope/Al-Amal EMM, MBRSC — Compiler fehlt),
  `superdarn.ca/data-download` (MAP-Grid RST, Globus).
- **ISRO/ISSDC (PRADAN) — kein Operator-Akt.** `released`; die Credentials stehen gemessen
  in `.secrets.local` (`PRADAN_USER`/`PRADAN_PASS`, via `bin/secrets_keys`, 2026-10-03),
  Compiler steht → Ernte ist autonomes Lesen. Der frühere „Operator-Hand"-Verdacht war stale.
- **KPLO/KARI KPDS** (`kari.re.kr/kpds/.../PublicRelease/`) harvestierbar; Mycelium.
- **Regel (Future folge172, privat):** vor jedem `Operator-Hand`/`operator-gebunden`-Label
  `.secrets.local` per `bin/secrets_keys` messen; ein Label wird im selben Pass vorgelegt
  oder als gewortet vermerkt.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  den `complex_te_probe` um den Detrend-along-p-Arm und den CMI/pTE-mit-p-Kovariate-Arm
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf **lokal/silent**,
  **nie in CI** (NSE-Daten bleiben im Haus). Träger `state/mountain/kuprat-complex-te/`.
  Step: Probe bauen, `--selftest` grün, dann der Sweep; `no statement`/`pending` bleiben
  erlaubte Ergebnisse (0 honored). Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.
