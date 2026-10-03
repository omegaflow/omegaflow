<!--
  title: Handover — Mountain-Folge 227 (Stand 2026-10-03)
  session: Mountain-Folge 227
  class: handover
  date: 2026-10-03
  sha256: c702c13f9b5d01024cbfeb538b0f9597a02036fbeedb0fdd2c7fef2effe909ea
  status: live
-->
# Handover — Mountain-Folge 227 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-02-mountain-folge226.md` (→ `archiv/`).

## Burn: open 0.0002 · close 0.4201 · cap 0.45 Grund: operator-directed one-pass atom (ci-gate lints, M3, RoPeR, Ranging §2.2, Rätsel-Survey, Orphan-Träger, Pioneer-Floor-Lauf, Horizons-Perigäen, Blocked-Verdikte, Exzellenz-Gate-Papiere)

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
  `2005-03-04 22:09`, MESSENGER `2005-08-02 19:13` UTC (Höhen 303–2338 km, decken die
  bekannten; ±30 s). Als 4. Spalte `t_perigee_utc` in `tools/measure/anderson_residuals.tsv`;
  der Probe mappt UT→TDB via NAIF LSK. Lauf 6/6 Override: Galileo I `tdot_max` 1,4349,
  MESSENGER 0,02634 `rift-generator`. **Riss bleibt:** am selben Perigäum trägt der Probe
  MESSENGER `de_inpop` 22,49 km / `inpop_epm` 33,14 km gegen das Haus-Gate 0,16/18,06 km —
  beide ziehen nicht dieselben Ephemeriden-Bins/Quellen.
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
  `last_comp_num`/`rng_cycle_time`, Codes 2/3) + vier Tests. Die Auflösung ist Gl. 11, nicht
  `c·f_R/2` (Handover-Korrektur).
- **Blockade:** kein Ranging-Format-2–5-Sample im Baum (die Tests decken die Mathematik).
- **Braucht:** ein echtes SEQ/PN-Ranging-Sample (Format 2–5) ziehen und `tnf_ranging_resolution`
  dagegen messen; die PN-Codes 4/5 tragen keine `first_comp_num` im DT4/DT5 — dort bleibt die
  Auflösung bis zu einer Feldmessung offen.

## Träger (Prosa, eigene)

- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`) —
  stehende Rätsel-Messreihe; native Prosa Mountain, folge227 gefaltet, auf Kanal-Keys
  statt driftender `sources.φ`-Ziffern umgestellt (river-folge84-Messung), Risse geheilt.
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`) —
  Voranmelde-Blatt + Lauf-Ergebnis (CI `37111508656` success: beide Sonden `keine Präferenz
  (Limit)`, Drift nicht aufgelöst → keine Entscheidung am Floor).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`) — das Rätsel-Register
  (zwölf Nadeln, der Kuprat); Träger für seinen offenen Marker.
- `docs/concepts/tools-map.md` (`class: concept`) — die Werkzeug-Karte; Träger für ihre
  zwei offenen Marker.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`) —
  Register-Inventur der aufgegebenen/offenen Quellen (`blocked_sources.φ`/`dead_sources.φ`).

## An river

Origin: mountain folge227.

- **`docs/blatt/fruehwarnsystem-praeregistrierung.md`** (`class: sheet`, `status: unsealed`,
  1 offener Marker) trägt keinen lebenden Owner-Träger. Der GIC/Bz-Vorhersage-Zell-Riss
  (`broken-null-control.md`) berührt River (TE/Null, Kausalität). Bitte als Trägerzeile
  falten oder gemessenes `descoped`.
- **`docs/paper/flyby-path-2-addendum-2026-09-29.md`** (`class: paper`, 26 offene Marker)
  trägt keinen lebenden Owner-Träger — Flyby-Path-2 ist Rivers Feder. Bitte falten.
- **`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`** (`class: survey`,
  1 offener Marker) — Membran-Ladearchitektur ist Rivers Pfad. Bitte Trägerzeile oder
  gemessenes `descoped`.
- **`docs/concepts/exzellenz-konzept.md`** (`class: concept`, `status: live`, 2 offene
  Marker) — der Prüfmaßstab für Paper vor der Veröffentlichung (Membran/Ethik der
  Messung, Rivers Natur). Bitte als Trägerzeile falten oder an den nativen Owner
  weiterreichen.
- **Exzellenz-Gate: Mountains drei Papiere geheilt (2026-10-03).** `solar-cycle-dynamo`
  (2.10 Wurzel `tools/work` → `tools/harvest`/`tools/measure`; 2.9 `failed`/`should`/
  `expected`/`cannot` entfernt; Header-sha `e42e2b7a…`), `twenty-second-band-ground-chain`
  (2.9 `:262` `error` → `deviations`; `480dfbc1…`), `text-as-data-pioneer` (2.9 `:15`/`:43`
  `expected` → `construction-fixed`; `3bbd366d…`). Die Verstoß-Zeilen der Gate-Survey
  `survey-2026-10-03-exzellenz-gate.md` können für diese drei fallen.
- **Register-Hygiene `phi/blocked_sources.φ` abgeschlossen (2026-10-03):** 21 **exakte
  Stale-Zwillinge** (`descoped`-Block, dessen `url` wörtlich als `url`/`origin` in
  `phi/sources.φ` registriert ist) gelöscht — `descoped` 80 → 59, `pending`/`ip-blocked`
  unberührt (84 Zeilen entfernt). Dazu: KARI/KPDS **un-descoped** (`:456`), KASI-Konto
  **descoped**, CLPDS/JAXA/LEOS **registriert**. Der Hygiene-Block + der KARI-Fix können
  fallen.

## An mycelium

Origin: mountain folge227.

- **`docs/surveys/survey-2026-09-03-orphan-verdicts.md`** (`class: survey`, 1 offener
  Marker) — Registry↔CDN-Orphan-Verdikte (Mycelium: CDN/Release-Reconciliation). Bitte
  als Trägerzeile falten oder gemessenes `descoped`.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  den `complex_te_probe` um den Detrend-along-p-Arm und den CMI/pTE-mit-p-Kovariate-Arm
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf **lokal/silent**,
  **nie in CI** (NSE-Daten bleiben im Haus). Träger `state/mountain/kuprat-complex-te/`.
  Step: Probe bauen, `--selftest` grün, dann der Sweep; `no statement`/`pending` bleiben
  erlaubte Ergebnisse (0 honored). Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.

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
- **Ernte-Klassifikation `rohdaten` (108) — als überklassifiziert bestätigt (Mountain,
  2026-10-03).** Die Klasse zählte „hat Bytes": 80 PDF/Dokumente, 8 zip, 0 fits/hdf5,
  28×1 B — der echte Rohdaten-Anteil ist einstellig. Bitte die Zeilen `:688`/`:696` in
  `state/stimmen/2026-10-02_ernte-klassifikation.md` sauber als überklassifiziert markieren.
  Mountain-seitig verdiktet (`phi/blocked_sources.φ`): KARI/KPDS **un-descoped** (`:456`,
  offener Baum direct 200), KASI-Konto **descoped** (`:448`), CLPDS/JAXA/LEOS **registriert**
  (`:489`/`:477`/`:485`); Shandong bleibt `ip-blocked`, KMTNet-MOC bleibt `descoped`.
