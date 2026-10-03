<!--
  title: Handover — Mountain-Folge 227 (Stand 2026-10-03)
  session: Mountain-Folge 227
  class: handover
  date: 2026-10-03
  sha256: 0b1e126d57d4b23179c1c42b4173340cc230b715ea59d16d4a044cedce1a07c4
  status: live
-->
# Handover — Mountain-Folge 227 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-02-mountain-folge226.md` (→ `archiv/`).

## Burn: open 0.0002 · close 0.0540 · cap 0.45 Grund: operator-directed one-pass atom (ci-gate lints, M3 verdict, Pioneer-Floor Blatt)

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
„1 ja bitte" — den privaten TE-Pfad entlocken (Detrend-along-p + CMI/pTE-mit-p-Kovariate), Lauf lokal/silent, nie CI (LOCK privat) | 2026-10-02 | Operator (river-folge82, 2026-10-02)
„ja voranmelde und dann lauf in ci" — Pioneer-Floor-Voranmelde-Blatt bauen, dann Lauf in CI | 2026-10-02 | Operator (river-folge82, 2026-10-02)
„braucht es pro und max?" — Reaffirmation flash-first; pro/max nur mit gemessener flash-Fehllage | 2026-10-03 | Operator (Session, Mountain 227)

## Offen (aufgeschlüsselt)

### JWS2-Kontrakt — CI grün, Manifestation offen (Trigger gefeuert)
- **Status:** wartend | **Bindung:** eigen (Manifestation: Mycelium)
- **Trigger:** `jwst-cdn-watch 37000087113` Lauf-Ende — **gefeuert**: `completed success`
  (measured 2026-10-03 via `ci_manage view 37000087113`).
- **Lage:** (gemessen 2026-10-03) der Watch trackt `jwst_spectra.bin`; **keine `jws2`-Zeile**
  in `phi/sources.φ` (`sgrep -i jws2 phi/sources.φ` = 0). Riss: der folge226-Trigger nennt
  den Spectra-Watch, nicht einen JWS2-Bin-Namen.
- **Blockade:** JWS2-Bin-Name + Manifestation (Mycelium).
- **Braucht:** Mycelium benennt den JWS2-Bin, setzt die `sources.φ`-Zeile + `sha256`; dann
  fällt der Punkt. Watch-Trigger gegen den JWS2-Arm prüfen.

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

### Probe-Artefakt vs Haus-Gate — Semantik benannt, Epochen-Neu-Lauf offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `sread tools/measure/src/bin/flyby_anderson_probe.rs`)
  die Spalten-Semantik ist benannt (`register_json` schreibt `first_row_ref_s`, `epoch_basis`,
  `tdot_max_scope`); die Absolut-Spalten (00:00-UTC vs Perigäum-TDB) bleiben unversöhnt
  (`docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md:71-82,101-104`). Die Perigäum-Zeiten
  selbst fehlen im Baum:   `tools/measure/anderson_residuals.tsv` trägt `t_utc = perigee date at
  00:00 UTC` (Zeile 11), exakte closest-approach-Zeiten `pending a clean Table I`.
  **Override gebaut (2026-10-03):** `parse_residuals` nimmt eine optionale 4. Spalte
  `t_epoch_tdb`; `register_json` schreibt `t_epoch_tdb` je Zeile und schaltet `epoch_basis`
  um; Tests `parse_residuals_reads_optional_epoch_override`, `_rejects_two_columns`.
- **Blockade:** kein Perigäum-Zeit-Feld im Residuen-Artefakt (Override-Mechanismus steht).
- **Braucht:** Perigäum-TDB je Zeile aus einer sauberen Anderson-Table-I eintragen, dann
  Neu-Lauf und Spalten-Vergleich mit dem Haus-Gate.

### RoPeR `.2C` — PDS4 `Table_Binary` mit `Group_Field_Binary`, Group-Flattening offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via Zenodo-Label `…_00046_A.2CL`) `Table_Binary`,
  `Record_Binary` `record_length` 16453 B × 52 records; `Group_Field_Binary Scientific_Data`
  2048 Wiederholungen × (R,I) `IEEE754LSBSingle`; danach V/P/Attitude `SignedMSB2`. Der
  `pds4_binary`-Arm (`src/archivar/pds4_binary.rs`) parst flache `Field_Binary`-Listen, keine
  Gruppen; `pds4_binary_compiler.rs` nimmt `--label`/`--dat`. Registersatz `.2CL`+`.2C` voll
  in Zenodo `15812343`/`15812357`.
- **Blockade:** kein Group-Field-Parser im `pds4_binary`-Arm.
- **Braucht:** `pds4_binary` um `Group_Field_Binary` erweitern (+ Test) oder einen dedizierten
  `roper_pds4_compiler` bauen; dann `.2C`+`.2CL` gegen den Zenodo-Satz laufen.

### Ranging-Decode — 810-005-214 öffnet die Range-Ambiguität
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via `pdftotext`) DSN 810-005 Modul 214 Rev B liegt in
  `docs/reference/810-005-214B-ranging.txt` (+ `.pdf`). Der TNF-Arm `odf.rs` trägt die
  Ranging-Formate (SEQ/PN-Phase, Codes 2–5). **§2.2-Kern gebaut (2026-10-03):**
  `ranging_component_code`/`_length`/`ranging_composite_period`/`ranging_ambiguity_resolution_m`
  (Tabelle 2; L = 1 009 470 nach Gl. 9; Auflösung `c·L/(4·f_RR)` nach Gl. 11, ≈75 660 km bei
  1 MHz) + drei Tests — die Ambiguitätsauflösung ist die Gl. 11, nicht `c·f_R/2`.
- **Blockade:** keine.
- **Braucht:** die Auflösung an ein Ranging-Format-2–5-Sample verdrahten
  (`first_comp_num`/`last_comp_num`/`chop_comp_num` aus `TnfDt2`/`TnfDt3` + `f_RR`) und den
  Spalten-Vergleich ziehen.

### Pioneer-Floor-Lauf — Blatt steht, Workflow + Run offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03) das Voranmelde-Blatt steht:
  `docs/blatt/blatt-pioneer-floor-falsifikation.md` (Header-sha `7a2ec29e…`); Methode, Null,
  dreiteiliges Kriterium fixiert; „GIC §3.2" als unentfaltete Referenz benannt.
  **`pioneer-floor.yml` gebaut (2026-10-03):** Corpus von CDN (`pioneer{10,11}_navio.bin`
  + `ephemeris_{earth,pioneer10_daily,pioneer11_daily}.bin`) → `pioneer_navio_residuum` →
  `pioneer_navio_negative_fuzzy --zone` → `pioneer_navio_zone_drift`, Artefakt-Upload.
- **Blockade:** keiner (die `pioneer{10,11}_navio.bin` sind auf dem CDN, 206 gemessen
  2026-10-03).
- **Braucht:** Lauf `pioneer-floor 37111508656` (dispatcht 2026-10-03) endet → Artefakt
  `pioneer-floor.txt` einmalig lesen (`ci_manage log`/Artifact); kein Statistik-Wert vor dem
  Blatt. Nicht pollen.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  den `complex_te_probe` um den Detrend-along-p-Arm und den CMI/pTE-mit-p-Kovariate-Arm
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf **lokal/silent**,
  **nie in CI** (NSE-Daten bleiben im Haus). Träger `state/mountain/kuprat-complex-te/`.
  Step: Probe bauen, `--selftest` grün, dann der Sweep; `no statement`/`pending` bleiben
  erlaubte Ergebnisse (0 honored). Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.

## An mycelium

Origin: mountain folge227.

- **JWS2-Bin:** den JWS2-Bin-Namen benennen, `sources.φ`-Zeile `format`/`url`/`origin`/
  `compiler` + `sha256` setzen; der `jwst-cdn-watch 37000087113` ist grün, aber kein
  JWS2-Bin registriert. Dann fällt der Punkt.
- **M3 (Chandrayaan-1 M3, ENVI):** Verdikt ist gesetzt (`phi/blocked_sources.φ:545-547`
  auf die lebende Route `pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0004/`, direct 200 gemessen
  2026-10-03); der Arm `pds3_img_compiler` steht, das M3-Asset `pds3_img_m3g20081118t222604_v03_loc.bin`
  (8758832 B, sha `5771de98…`, roundtrip, folge218) ist unregistriert. Bitte die
  `sources.φ`-Zeile (`format pds3_img`, `origin` = JPL-Route, `compiler`
  `tools/harvest/src/bin/pds3_img_compiler.rs`) setzen und manifestieren. CI-Runner-403
  bleibt (lokaler Bau, CI-IP) — Transport nennt das gemessene Hindernis.

## An future

Origin: mountain folge226 (2026-10-02/03, Browser + curl; noch nicht gefaltet).

- **ESA/ESOC als zweite Zeugenlinie — Rohdaten nicht öffentlich.** Hoffmann & Budnik 2026
  (`arXiv:2609.23482`) rechnen auf ESOC-eigenem Orbit-Determination; die originären
  radiometrischen Daten (Rosetta EAR1/2005, BepiColombo MORE `release 2099`, Solar Orbiter)
  sind nicht öffentlich. Eigenständige Nachrechnung braucht eine ESOC-Datenanfrage (per-Akt-Wort)
  — operator-gebundener Punkt für die Queue.
- **CLPDS Chang'e (NAOC/GRAS) — Registrierung nötig.** `https://clpds.bao.ac.cn/` REST-API
  anonym lesbar, Download login-gated; Registrierung `…/register` (Konto, Name, Telefon,
  E-Mail, Ausweisnummer). Operator-gebundener `blocked account`-Punkt.
- **JAXA G-Portal — Download nur nach Registrierung.** `https://gportal.jaxa.jp/gpr/user/regist1`.
  Operator-gebundener Account-Punkt.
- **Shandong PDS-Spiegel (SDU Weihai) — von hier nicht erreichbar.** `pds.wh.sdu.edu.cn`
  listet Chang'e-1/2, Host antwortet nicht (Timeout, cn-only) → `blocked ip-blocked`.
- **PDS-PPI EPN-TAP — Maschinenendpunkt.** `vo-pds-ppi.igpp.ucla.edu/tap/{capabilities,sync}`
  je 200 per curl; `tap_compiler`-Arm passt; kein Account.
- **KASI-DALO — kein Self-Signup.** `pda.kasi.re.kr/login.php`; Konto nur per Anfrage an KASI.
  Operator-gebundener Account-Anfrage-Punkt.
