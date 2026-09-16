<!--
  title: Handover — Bau-Folge 51 (Stand 2026-09-16)
  session: Bau-Folge 51
  class: handover
  date: 2026-09-16
  sha256: 7670cd8df8ebaf3f86bd8c655f0b44fe3bea4ae6870b67ce5594a6cebbadfec6
  status: live
-->
# Handover — Bau-Folge 51 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## TE-Gate-Arx — die Produktion läuft auf Arx, die CI-Messung steht aus

- Der Switch steht im Baum: die Produktions-Signifikanz-Nulls laufen auf
  `TeNull::Arx` (corona ksg-n/binned-n, nobel bz/laic, pcmci-Benchmark-Default
  = arx); die Kalibrier-Gate-FP/FN-Tests messen Arx (`arx_sweep_n1000` — früher
  `residual_sweep_n1000`, `gate_ksg_finds_anchor_links_floor`, beide
  multi-driver-Zellen, `pcmci_recovers_known_dag`). Der te-gate-Dispatch nach
  dem Push dieses Commits ist die Verifikation (n=1000-Gates +
  `block_sweep_n1000` + `arx_sweep_n1000`); die n=150-Zellen laufen im ci-check.
  (Schritt: den Run lesen — `gh run list --workflow=te-gate` / `gh run view <id>`;
  bei rot benennt der Assert-Text die Zelle.)

## Corona kde-1/kde-2 — Residual-Maschinerie direkt, kein TeNull

- Die 1-/2-Konditional-Pfade (`conditional_te_stats_lagged`,
  `conditional_te_stats_lagged_2` in corona_conditional_probe.rs) tragen die
  Residual-Surrogat-Maschinerie direkt (kein TeNull-Selektor) — ein
  Arx-Gegenstück für diese Signaturen existiert nicht; die vier TeNull-Stellen
  (ksg-n/binned-n) laufen auf Arx. (Schritt: `arx_restricted_surrogate_2`
  bauen und die zwei Pfade umstellen — oder der gemessene Befund als descoped.)

## DRS-FITS — Dispatch misst einen Spalten-Bruch

- Dispatch `drs-fits-cdn.yml` → run `35116934467` (failure, 49 s): die Granule ist
  live (HTTP 200, 1 022 400 B), trägt BINTABLE `SCI_SCIENCE_1Hz` + Spalte `ESA00001`,
  aber **nicht** die Parser-Spalten `ESA00002` / `DST11077–079` / `DST11083–085`
  (`drs_rows_from_table`, `src/archivar/fits.rs:1010`). `drs_series` liefert None;
  die Diagnose (`tools/harvest/src/bin/drs_fits_compiler.rs:51`) benennt fälschlich
  „kein SCI_SCIENCE_1Hz". (Schritt: die TTYPE-Karten der Granule messen und die
  Spaltennamen neu abbilden; die Diagnose auf die fehlende Spalte umstellen.
  Index-Zeit-Drift / Anker `at earth` bleibt.)

## Star-Katalog — gaia-cdn brach am fehlenden Checkout

- Run `35113568241` (failure): `.github/workflows/gaia-cdn.yml` hatte keinen
  `actions/checkout`-Schritt → `cargo` lief im leeren Workspace (`exit 101`). Checkout
  + Rust-Toolchain ergänzt. `dr3_stars.bin` liegt auf dem CDN
  (75 001 828 B = 44-B-Stride, `rv`), Stand 2026-08-23.
  (Schritt: `gaia-cdn.yml` neu dispatchen, das 44-B-Asset verifizieren.)

## Mail-Fang — KV-Namespace blockt an der Token-Scope

- `cloudflare/wrangler.toml:14` trägt `REPLACE_WITH_WRANGLER_KV_NAMESPACE_ID`;
  `wrangler kv namespace create MAIL_QUEUE` und der REST-POST liefern
  `Authentication error [code 10000]`, obwohl `wrangler` `workers_kv (write)` listet.
  `account_id`/`FORWARD_TO` stehen leer (`.secrets.local`). Operator-gebunden.
  (Schritt: KV-Namespace im Dashboard anlegen oder den Token account-scoped mit
  „Workers KV Storage:Edit" neu erzeugen — Post an `entscheid`.)

## Katalog-Konsumenten — Feld-Semantik offen

- Offen: VLASS (kein src-Modul, phi ohne Distanz/Parallaxe), `catalog_dcom5`
  (CometRec ohne GM), `des_y6` (nur `sigma_m`), `ossos` (Kepler ohne GM), `twomrs`
  (kein Distanzfeld), `las` (Frame/Anker fehlt), `nexrad_level2` (44-B-Bin ohne
  Standort). (Schritt: je Quelle Feld-Semantik messen oder `descoped`/`pending`.)

## ODR — Serie/Granulat und Semantik

- `voyager-odr-cdn.yml` existiert (Einzeldatei); Reihen-Leser-Arm gebaut + getestet.
  Offen: Granulat für die 484-Datei-Serie (13,61 GB > 2-GB-Limit); `sample_count`-
  Semantik; Galileo 12-bit kein Record im PPI-Archiv; `year_full` 00–89 `None`;
  Voyager Decimation>1 unverifiziert. (Schritt: Granulat wählen + Workflow erweitern.)

## smail_recv — leerer Body bei verschachteltem MIME

- Rekursiver MIME-Abstieg gebaut + getestet; Root cause (leerer Sotgiu-Reply)
  unbestätigt. (Schritt: bei erneutem leerem Body die Worker-`message.raw`-Quelle
  messen — `cloudflare/email_worker.js`.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## CI-Format-Gate und external-state

- CI-`format` rot @625452e5: `src/gate/commit_gate.rs:540` + fremde
  measure/utils/harvest-Dateien. external-state `CI-Status` ist fällig (HEAD-Wechsel).
  (Schritt: rustfmt-Diff anwenden; den CI-Lauf des gepushten SHA messen.)

## Descoped — ausgemusterte TE-Nulls (0-Kanon, Messung CI-Run 35092997862)

Gemessene Befunde der ausgemusterten Nulls (n=1000, a=0,9, CI-Run
35092997862) — der Befund selbst ist der Eintrag, kein Unterhalt:

- **Residual** — FPR 19,51 %/ksg @ a=0,9: ausgemustert; der `--null residual`-Arm
  des Benchmarks trägt die Ruhestands-Zeile und läuft nur noch benannt.
- **Restricted** — FPR 10,71 %/binned: ausgemustert (kein Produktions-Verbraucher).
- **Block** — FPR-Anstieg +2,47pp binned / +2,75pp ksg über a: Befund gemessen,
  Entscheidung offen — `block_sweep_n1000` (te-gate) misst, ob eine Blocklänge
  hält (Anstieg ≤ 2pp); hält eine: Block mit dieser Länge bleibt in
  `multi_force_te_probe.rs`; hält keine: Probe auf Arx, Block ausmustern.
  (Schritt: den `block_sweep_n1000`-Abschnitt des te-gate-Runs lesen.)
- **Phase** — FPR 8,52 %/ksg: bleibt als Spektral-Placebo (registrierte
  Eigenschaft). `placebo_pair_eeg_probe` (alle 6 Stellen) und
  `tools/measure/src/rest.rs:697` sind Phasen-Placebos: Vertrag spektral
  (fam-Schwelle über phasen-randomisierten Surrogaten), kein
  FPR-Kontroll-Anspruch — kein Switch, die Eigenschaft steht hier.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
