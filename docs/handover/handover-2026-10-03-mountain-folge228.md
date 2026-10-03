<!--
  title: Handover — Mountain-Folge 228 (Stand 2026-10-03)
  session: Mountain-Folge 228
  class: handover
  date: 2026-10-03
  sha256: fc8f3d9e6fcd19faa641d4c016f3edef7288e3e279689ce526f2ee55ad3dd278
  status: live
-->
# Handover — Mountain-Folge 228 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-03-mountain-folge227.md` (→ `archiv/`).

## Burn: open 0.0011 · close 0.1255 · cap 0.5 Grund: blocked-sources hygiene, Swarm-DISS-Verdikt + TEC-Compiler, adressierte Blöcke gefaltet

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
  `state/zustand/wartend.φ` (`petrel19-license`); Verdikt-Zeile `phi/blocked_sources.φ`.
- **Blockade:** Lizenz ungeklärt.
- **Braucht:** Antwort abwarten (Wiedervorlage); bei Lizenz Register-Zeile in `phi/sources.φ`
  + `ephemeris_house_gate`/`flyby_anderson_probe` auf das vierte Haus erweitern
  (`ephemeris_house_gate.rs:297-299`, heute fest `de`/`inpop`/`epm`).

### Probe-Artefakt vs Haus-Gate — Perigäum-Zeiten gesetzt, Haus-Quellen-Riss benannt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) **Perigäum-Zeiten besorgt:** JPL Horizons (CENTER `500@399`,
  Range-Minimum, 1-min) für alle sechs Vorbeiflüge — Galileo I `1990-12-08 20:35`, II
  `1992-12-08 15:09`, NEAR `1998-01-23 07:23`, Cassini `1999-08-18 03:28`, Rosetta
  `2005-03-04 22:09`, MESSENGER `2005-08-02 19:13` UTC (Höhen 303–2338 km). Als 4. Spalte
  `t_perigee_utc` in `tools/measure/anderson_residuals.tsv`; der Probe mappt UT→TDB via NAIF LSK.
  Lauf 6/6 Override: Galileo I `tdot_max` 1,4349, MESSENGER 0,02634 `rift-generator`. **Riss bleibt:**
  am selben Perigäum trägt der Probe MESSENGER `de_inpop` 22,49 km / `inpop_epm` 33,14 km gegen das
  Haus-Gate 0,16/18,06 km — beide ziehen nicht dieselben Ephemeriden-Bins/Quellen.
- **Blockade:** keine (die Haus-Quellen-Differenz ist der nächste Messpunkt).
- **Braucht:** die vom Haus-Gate geladenen Ephemeriden-Bins gegen die Probe-Defaults
  (`data/ssd.jpl.nasa.gov-de/…` vs `data/ssd.jpl.nasa.gov/…`) abgleichen und den Riss auflösen.

### Ranging-Decode — §2.2 gebaut und verdrahtet, Live-Sample fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) DSN 810-005 Modul 214 Rev B liegt in
  `docs/reference/810-005-214B-ranging.txt` (+ `.pdf`). **Gebaut:** `ranging_component_code`/
  `_length`/`ranging_composite_period`/`ranging_ambiguity_resolution_m` (Tabelle 2; L = 1 009 470
  nach Gl. 9; Auflösung `c·L/(4·f_RR)` nach Gl. 11, ≈75 660 km bei 1 MHz), `ranging_resolution_from_cycle_time_m`
  und **`tnf_ranging_resolution(frame, bytes)`** (verdrahtet `TnfDt2`/`TnfDt3` `first_comp_num`/
  `last_comp_num`/`rng_cycle_time`, Codes 2/3) + vier Tests.
- **Blockade:** kein Ranging-Format-2–5-Sample im Baum (die Tests decken die Mathematik).
- **Braucht:** ein echtes SEQ/PN-Ranging-Sample (Format 2–5) ziehen und `tnf_ranging_resolution`
  dagegen messen; die PN-Codes 4/5 tragen keine `first_comp_num` im DT4/DT5 — dort bleibt die
  Auflösung bis zu einer Feldmessung offen.

### RoPeR `gras_2c` — Kraft-/Feld-Zuordnung fehlt (Mountain-Seite)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via `sread phi/sources.φ:11085-11091`) der RoPeR-Bin
  `gras_2c_roper_hx1-ro_…_00046_a.bin` ist registriert (sha `d4b7397a…`, format `gras_2c`,
  `at mars`, `ttl 604800`), trägt aber **keine `field`-Zeilen**; der `gras_2c`-Arm steht
  (`main_flow.rs:2932`), der Record hat 42 `.2C`-Paare (mycelium-folge224: „der Zuordnungsteil
  bleibt bei Mountain").
- **Blockade:** kein 2C-Feld-/Spalten-Schema gemessen.
- **Braucht:** das 2C-Bin-Schema des RoPeR-Produkts (Zenodo-Record 15812343 / PDS4-Label)
  gegen `roper_pds4_compiler.rs` und das `gras_2c`-Modul messen, dann `field`-Zeilen in
  `phi/sources.φ:11085` setzen.

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

Origin: mountain folge228.

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

## An mycelium

Origin: mountain folge228.

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
