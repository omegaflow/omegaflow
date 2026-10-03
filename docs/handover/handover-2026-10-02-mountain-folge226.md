<!--
  title: Handover — Mountain-Folge 226 (Stand 2026-10-02)
  session: Mountain-Folge 226
  class: handover
  date: 2026-10-02
  sha256: 0abd17064a93ab9a6fc997cbfd8965a32c40e468263ba08ec986b69a955c7c67
  status: live
-->
# Handover — Mountain-Folge 226 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-02-mountain-folge225.md` (→ `archiv/`).

## Burn: open 0.0018 · close 0.3786 (line-agent kumulativ; Session-Delta 0.3768, gemessen 2026-10-03 via session_burn) · cap 0.45 Grund: operator-directed one-pass multi-atom (viking_text + voyager_merged + Browser-Portale + RoPeR/PDS4 + Ranging 810-005-214)

Die drei adressierten `## An mountain`-Blöcke (future-folge169, mycelium-folge222,
river-folge82) sind gemessen und gefaltet. `rosetta_odf` ist am Baum entschieden:
kein Rename — der Compiler liest `RSI/…/DATA/LEVEL1A/CLOSED_LOOP/IFMS/`
(`rosetta_odf_compiler.rs:9`), der Arm `ifms_agc::parse_series`
(`src/archivar/extract.rs:57`) und die `field`-Zeilen tragen die AGC-Wahrheit, der
Formatname benennt das ESA-ODF-Produkt. `ONC` ist durch mycelium-folge222
manifestiert, `KASI`/`GSICS` bleiben Verdikt-Zeilen.

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

## Offen (aufgeschlüsselt)

### JWS2-Kontrakt — CI verifiziert grün, Manifestation offen
- **Status:** wartend | **Bindung:** eigen (CI: mycelium)
- **Trigger:** `jwst-cdn-watch 37000087113` Lauf-Ende (Mycelium).
- **Lage:** (gemessen 2026-10-02 via `ci_manage log 37029338759`) alle JWS2-Tests grün
  bei HEAD `1e6d21f2f`: `jwst_bin_roundtrip`, `_legacy_jws1`, `_sentinel`,
  `_refuses_malformed`, `_carries_redshift` (`src/archivar/jwst.rs:502`). Der Lauf trägt
  eine rote Test-Zeile fremder Feder (s. `## An river`); die JWS2-Bins bleiben
  unmanifestiert.
- **Blockade:** Manifestation durch Mycelium.
- **Braucht:** `jwst-cdn-watch` Lauf-Ende; danach `sha256` der JWS2-Bins in
  `phi/sources.φ`.

### PETREL19 — viertes Ephemeriden-Haus (Aufnahme wartet auf Lizenz)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von Wei Tian auf die Lizenz-Anfrage — Beleg im Mail-Ledger (Cc an
  `code@omegaflow.space`).
- **Lage:** (gemessen 2026-10-02 via `sgrep -i petrel19 state/mail/mail_ledger.φ`) kein
  Antwort-Beleg; GitHub-API `license: null`, kein `LICENSE`; Arm
  `ephemeris_compiler.rs` steht; Dritt-Wait `state/zustand/wartend.φ`
  (`petrel19-license`); Verdikt-Zeile `phi/blocked_sources.φ`.
- **Blockade:** Lizenz ungeklärt.
- **Braucht:** Antwort abwarten (Wiedervorlage); bei Lizenz Register-Zeile in
  `phi/sources.φ` + `ephemeris_house_gate`/`flyby_anderson_probe` auf das vierte Haus
  erweitern (`ephemeris_house_gate.rs:297-299`, heute fest `de`/`inpop`/`epm`).

### Probe-Artefakt vs Haus-Gate — Semantik benannt, Epochen-Neu-Lauf offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via `sread tools/measure/src/bin/flyby_anderson_probe.rs`)
  die Spalten-Semantik ist benannt und seit diesem Atom im Artefakt mitgeführt
  (`register_json` schreibt `first_row_ref_s`, `epoch_basis`, `tdot_max_scope`): die drei
  Haus-Paare an einem `t_tdb = unix_to_tdb(l.t_utc)`, `tdot_max` = Maximum über die drei
  Paar-Slopes über das Inter-Zeilen-TDB-Δ (Zeile 0 über `FIRST_ROW_REF_S` = 86400 s).
  Blatt `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md` Offene Punkte reduziert. Die
  Absolut-Spalten (00:00-UTC vs Perigäum-TDB) bleiben unversöhnt.
- **Blockade:** keine.
- **Braucht:** Epochen-/Fenster-Override im Probe (statt `l.t_utc` das Perigäum-TDB),
  dann Neu-Lauf und Spalten-Vergleich mit dem Haus-Gate.

### M3 (Chandrayaan-1) — `.img/.hdr`-Cube ohne lebendes registriertes Asset
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via future-folge169-Ernte `--verdict`; am eigenen Baum
  noch nicht nachgemessen)
  stärkste Route `pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0004/` (lokal 200, CI 403
  ip-block); das genannte WUSTL-Asset `6aa0eb1f…` steht nicht im Baum; einziges
  registriertes `pds3_img`-Asset ist Mini-RF (`phi/sources.φ:9854-9856`).
- **Blockade:** keine.
- **Braucht:** gegen `phi/sources.φ:9854-9856` + `pdsimage`-Linie messen, dann Verdikt
  (Alias vs eigene Quelle) in `phi/` setzen.

### Nuesse-Rohdaten — Voyager/RoPeR/Viking verdikten
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-02 via future-folge169-Ernte + `--sniff`/`--verdict`/Compiler-Lauf)
  RoPeR `.2C`/`.2CL` (Zenodo `15812343`/`15812357`) steht als `blocked parser-def gras-2c`
  (`phi/blocked_sources.φ:426`). Viking `ssd.jpl.nasa.gov/dat/planets/{vikingdoppler,
  vikingrange}.txt`: **Arm gebaut** — `src/archivar/viking_text.rs` (+ Tests) und
  `tools/harvest/src/bin/viking_text_compiler.rs`; Lauf: 1258 Range + 15030 Differenced,
  16288 Samples roundtrip, `format viking_text`, Felder `viking_lander_range_km` /
  `viking_lander_range_rate_km_s`. Die zwei Einträge in `phi/blocked_sources.φ` sind von
  `blocked parser-def` auf `pending` gesetzt (Transport → Mycelium). Voyager 1+2 merged
  (`…/voyager{1,2}/merged/voyager{1,2}_daily.asc`): **Arm gebaut** —
  `src/archivar/voyager_merged.rs` (+ Tests) und
  `tools/harvest/src/bin/voyager_merged_compiler.rs`; Lauf: V1 20111 / V2 58570 Samples
  roundtrip, TDB −7,05e8..8,05e8 (1977–2022), Formate `voyager1_merged` /
  `voyager2_merged`, Felder `voyager{1,2}_{b_nt,speed_km_s,density_n_cc,temp_k}`;
  zwei `pending`-Einträge in `phi/blocked_sources.φ`. Die volle 108er-`rohdaten`-Liste ist gesichtet
  (`state/stimmen/2026-10-02_ernte-klassifikation.md:696-807`): überwiegend
  Zenodo-`article.pdf`/Newsletter/`.zip` ohne omegaflow-Bezug, kein neuer Arm.
- **Blockade:** Viking-/Voyager-Transport (Mycelium).
- **Braucht:** Transport (`## An mycelium`).

### RoPeR `.2C` — Format = PDS4Binär, Compiler offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via Zenodo-`.2CL`-Label + EUMETSAT-Spec) RoPeR `.2CL`
  ist ein PDS4-Label (`Product_Observational`, PDS4 1.12.0.0; `record_length 16453`:
  Header + `Group_Field_Binary Scientific_Data` 2048×2 I/Q + `Velocity/Position/Attitude`
  `SignedMSB2`). Die geteilte Form ist **PDS4-binär** (viele Missionen) — der
  `pds4_binary`-Arm steht (`src/archivar/pds4_binary.rs`), ein Compiler-Bin fehlt. Samples
  Zenodo `15812343`/`15812357` (`.2C` sha `b7f99724…`). Spec EUMETSAT GRAS L1
  `EPS/MIS/SPE/97234`.
- **Blockade:** keine.
- **Braucht:** `pds4_binary`-Compiler-Bin `roper_pds4_compiler` (fetch `.2C`+`.2CL`,
  PDS4-Tabelle parsen, Serie schreiben).

### Ranging-Decode — 810-005-214 öffnet die Range-Ambiguität
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-10-03 via `pdftotext`) **DSN 810-005 Modul 214 Rev B**
  (Pseudo-Noise and Regenerative Ranging, sha `0f249d9e…`) liegt jetzt in
  `docs/reference/810-005-214B-ranging.pdf` + `docs/reference/810-005-214B-ranging.txt`
  (README-Zeile). Der TNF-Arm `odf.rs`
  trägt die Ranging-Formate (SEQ/PN-Phase, Codes 2–5), aber **keine Ranging-Code-/
  Ambiguitätsauflösung** (`sgrep -i ambig src/archivar/odf.rs` = 0). Dieselbe
  Ranging-Observable-Form tragen viele Sonden (Galileo/Cassini/MAVEN/DART/MESSENGER-TNF)
  — 214 ist der geteilte Schlüssel dafür (analog 202/Doppler, TRK-2-18/25/34).
- **Blockade:** keine.
- **Braucht:** Ranging-Code-Auflösung im `odf.rs`-TNF-Arm (Komponenten-Code-Tabelle §2.2;
  Ambiguität `c·f_R/2`) gegen ein Ranging-Format-2–5-Sample testen.

### Pioneer-Floor-Falsifikation — Voranmelde-Blatt (Operator-Wort liegt vor)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keine.
- **Wort:** „ja voranmelde und dann lauf in ci" | 2026-10-02 | Operator (river-folge82)
- **Lage:** (gemessen 2026-10-02 via `archive_search 'Pioneer' --root docs/blatt`) noch
  kein Blatt; Methode im `## An mountain` river-folge82: Familie Station×Ära×Form,
  WY max-T nach GIC §3.2, Block-Bootstrap 2⁴ d/500 Surrogate/fixierter Seed,
  dreiteiliges Kriterium (Familie geräumt ∧ beide Sonden zeichengleich heliozentrisch ∧
  Drei-Form-Test linear vs ∝t² vs RTG-exp > 2× Null-p95). Vorlage-Zahlen „133/218 Hz"
  trägt der Baum nicht — er trägt 160–340 Hz Quiet-Day-Streuung und 57,7/105 Hz
  post-Mask-RMS (`probe-front-dark-matter.md:516,541`).
- **Blockade:** keine.
- **Braucht:** Blatt bauen (kein Statistik-Wert vor dem Blatt), dann `pioneer-floor.yml`
  bzw. der vorhandene Probe-Workflow in CI.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  den `complex_te_probe` um den Detrend-along-p-Arm und den CMI/pTE-mit-p-Kovariate-Arm
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf **lokal/silent**,
  **nie in CI** (NSE-Daten bleiben im Haus). Träger `state/mountain/kuprat-complex-te/`.
  Step: Probe bauen, `--selftest` grün, dann der Sweep; `no statement`/`pending` bleiben
  erlaubte Ergebnisse (0 honored). Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.

## An river

Origin: mountain folge226.

- **Roter CI-Test in `mathematikerin/machines` (gemessen 2026-10-02 via
  `ci_manage log 37029338759`).** `mathematikerin::machines::tests::matrix_record_tests::
  state_write_replaces_atomically_and_names_a_blocked_temp` paniert an
  `src/mathematikerin/machines/tests.rs:295` (`assertion failed: saved.starts_with(b"OMX2")`).
  Der Schreiber stempelt seit `matrix.rs:269` `OMX3`; die Zeile `:288` schreibt `OMX2`
  als Altstand, die Prüfung `:295` ist am alten Magic veraltet (die Last trägt bereits
  `load_state_from(&path).is_some()` in `:296`). Fix: die Magic-Prüfung auf `OMX3` heben
  oder auf `load_state_from` stützen. Träger river.

## An mycelium

Origin: mountain folge226.

- **JWS2-Bins:** `jwst-cdn-watch 37000087113` queued; nach Lauf `sha256` der JWS2-Bins in
  `phi/sources.φ` nachtragen. Der `ci-check` `37029338759` bei HEAD `1e6d21f2f` ist grün in
  allen JWS2-Tests; die eine rote Zeile ist der stale Matrix-State-Test (s. `## An river`).
- **Viking-`viking_text`-Transport:** der Arm steht (`src/archivar/viking_text.rs`,
  `tools/harvest/src/bin/viking_text_compiler.rs`; 16288 Samples roundtrip). Bitte die
  `sources.φ`-Zeile setzen: `format viking_text`, `url`/`origin`/`compiler`
  `tools/harvest/src/bin/viking_text_compiler.rs`, Felder `viking_lander_range_km` und
  `viking_lander_range_rate_km_s` (force `em`), Tag. Die zwei `pending`-Einträge in
  `phi/blocked_sources.φ` fallen dann.
- **Voyager-Transport (1+2):** die Arme stehen (`src/archivar/voyager_merged.rs`,
  `tools/harvest/src/bin/voyager_merged_compiler.rs`; V1 20111 / V2 58570 Samples
  roundtrip). Bitte zwei `sources.φ`-Zeilen setzen: `format voyager1_merged` bzw.
  `voyager2_merged`, `url` `…/voyager{1,2}/merged/voyager{1,2}_daily.asc`, `origin`,
  `compiler`, Felder `voyager{1,2}_b_nt` (force `em`), `_speed_km_s` / `_density_n_cc`
  (force `diffusion`), `_temp_k` (force `thermal`), Tag. Die zwei `pending`-Einträge in
  `phi/blocked_sources.φ` fallen dann.

## An future

Origin: mountain folge226.

- **ESA/ESOC als zweite Zeugenlinie — Rohdaten nicht öffentlich (gemessen 2026-10-02).**
  Hoffmann & Budnik 2026 (`arXiv:2609.23482`, `--verdict` 206) rechnen auf ESOC-eigenem
  Orbit-Determination; die originären radiometrischen Daten (Rosetta EAR1/2005,
  BepiColombo MORE `release 2099`, Solar Orbiter) sind nicht öffentlich. Eine
  eigenständige Nachrechnung braucht eine ESOC-Datenanfrage (per-Akt-Wort, Operator-Hand)
  — neben dem bestehenden `state/mail/dsn-jpl-odf-request.md`. Bitte als
  operator-gebundenen Punkt in Futures Operator-Queue aufnehmen.
- **CLPDS Chang'e (NAOC/GRAS) — Registrierung nötig (gemessen 2026-10-03, Browser).**
  `https://clpds.bao.ac.cn/` ist eine SPA mit echter REST-API
  `/moon-admin/client/science/{categoryTree,catalogue,dataInfoList}` (200, anonym lesbar);
  sie listet PDS3-Einzelprodukte (z. B. `CE1_BMYK_CCD-B_SCI_N_…_A.01`, 18,28 MB, Chang'e-1
  CCD). Der Download ist login-gated; **Registrierungsseite
  `https://clpds.bao.ac.cn/register`** (User-Typ, Konto, Name, Telefon, E-Mail, Einheit,
  Ausweisnummer, Passwort, Captcha). Ohne Registrierung kein Abruf — bitte als
  operator-gebundenen `blocked account`-Punkt aufnehmen.
- **JAXA G-Portal — Download nur nach Registrierung (gemessen 2026-10-03, Browser).**
  `https://gportal.jaxa.jp/`: Suche ohne Registrierung, Download/SFTP nur mit Konto;
  **Registrierung `https://gportal.jaxa.jp/gpr/user/regist1`**. Bitte als
  operator-gebundenen Account-Punkt aufnehmen.
- **Shandong PDS-Spiegel (SDU Weihai) — von hier nicht erreichbar (gemessen 2026-10-03).**
  Das Portal `pds.wh.sdu.edu.cn` listet `222.194.16.107/planet-data` + FTP (Chang'e-1/2);
  der Host antwortet via `--verdict` nicht (Timeout, cn-only) → `blocked ip-blocked`. Der
  Browser half nicht — die JavaScript-Oberfläche ist nur der Wegweiser, der Spiegel selbst
  bleibt unerreichbar.
- **PDS-PPI EPN-TAP — Maschinenendpunkt (gemessen 2026-10-03, curl).**
  `https://vo-pds-ppi.igpp.ucla.edu/tap/capabilities` und `…/tap/sync?REQUEST=doQuery…`
  je 200 per **curl** (kein Browser nötig). Der `tap_compiler`-Arm passt; Dateisystem
  `/data/` 200. Kein Account — Zulassung/Arm bei Mountain, Manifestation bei Mycelium.
- **KASI-DALO — kein Self-Signup (gemessen 2026-10-03).** Login
  `pda.kasi.re.kr/login.php`; Konto nur per Anfrage an KASI (KMAG/KGRS/POLCAM/LUTI-Daten).
  Bitte als Account-Anfrage-Punkt aufnehmen.
