<!--
  title: Handover — Ernte-Folge 71 (Stand 2026-09-17)
  session: Ernte-Folge 71
  class: handover
  date: 2026-09-17
  sha256: 0dcdc3532537d16220dcad40a7d345e6f4dd0c0132b77517b9b9648f5dcc25aa
  status: live
-->
# Handover — Ernte-Folge 71 (2026-09-17)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Pipeline-Karte — `SOURCE_PORT.md` §2 + `index.φ` (härtester undatiert)

Von der entscheid-Linie gemessen (Entscheid-Folge 38, `general`-Diver, flash) und
hier gefaltet; am Baum bestätigt (`git ls-files`: absent). Die Auslese ergab
**keinen prune** — jede Gruppe ist getragen (`catalog/*.φ` → MANIFEST+Code+Workflows;
`queue/grind_*` → SOURCE_PORT §2 + index.φ + `src/archivar/{port,tests}.rs`;
`stage/` → port.rs + health-check.yml; `research/agent_output/` → SOURCE_PORT
§5/§13, nur-Kopie; `meteo_harvest/` → meteo-cdn.yml + harvest; 5 Fixture-Dirs →
Rust-Tests `hdf5.rs:2407/2409/3057`, `netcdf.rs:1144`).

- **Echter Defekt** — `docs/SOURCE_PORT.md` §2 nannte abwesende Dateien
  (`phi/pipeline/queue/master.φ`, `queue/sources_potential_*`, `park/`,
  `probe_comparison.txt`, `probe_batch.φ`); `phi/pipeline/index.φ` (Stand
  2026-08-15) ist überholt. Die entscheid-Linie korrigiert §2 (Entscheid-Folge
  38, im Baum uncommitted gemessen: Master-Zeile + `stage/`-Auslagerung nach
  `archive-root/pipeline-auslese-2026-09-17/`). Offen bleibt die
  `index.φ`-Regeneration. Schritt: `phi/pipeline/index.φ` gegen den Ist-Bestand
  regenerieren — `grind-flash`. · offen

## Harvest-Architektur — Argument-Punkt (gebaut) und Familien-Blöcke

- **Argument-Punkt steht** (dieses Atom): `phi/harvest.φ`-Feld `args <cli>` —
  `harvest_reg.rs` (`--check`-Whitelist) + `harvest.yml` liest `args` aus dem
  Block und reicht es dem Arm weiter (`read -r -a argv <<< … ; cargo run … -- "${argv[@]}" --ci-mode`).
  Kein Default, keine Fabrikation: ein fehlendes Pflicht-Argument verweigert der
  Arm selbst (exit 2) + `gh_issue_once`. `cargo check` 0/0.
- **Fünf Familien-Blöcke fehlen** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  (operator-gebunden, protected-Bucket-403), `dl3_skymap`/`juno_ocru_odf`
  (CDN-present): ein Block je **Quelle/Arm**, nie je generischem Konsum-Format
  (`text`/`volume` bleiben allein in `phi/sources.φ`; die `--check`-Eindeutigkeit
  erzwingt das). Ein Block ohne gemessene `asset`-Zeile wäre Fabrication. Schritt:
  `asset` je Familie messen, dann Block schreiben — `grind-pro`. · wartend
  (Auslöser = Asset-Messung)
- **`args`-Syntax dokumentieren** — die neue Register-Zeile ist bisher nur in
  `harvest.yml`/`harvest_reg.rs` lesbar, nicht im Register/Protokoll beschrieben.
  Schritt: eine Zeile in `docs/SOURCE_PORT.md` (Harvest-Abschnitt) oder
  `phi/harvest.φ`-Kopf — `grind-flash`. · offen
- **rosetta_odf Dispatch-Beweis** `termin` — Post-Fix-Lauf `35270867738` @
  `dc9291ad` (queued beim Session-Ende); dispatcht von `harvest-dispatch`
  `35270851153` @ `dc9291ad`. Bei success: `phi/harvest.φ:34` Block-note auf
  `asset present` + Run-Id/Bytes/sha256. Schritt: `ci_manage view 35270867738`
  **einmal** nach Abschluss.
- **rosetta ungelaufene Pfade** `wartend` — Idempotenz-Gate, `force`-Lauf,
  zweiter Zeuge (Shard-Vollständigkeit), `--check` Feldeindeutigkeit +
  timeout-Validierung (`harvest_reg.rs`); erst im ersten erfolgreichen
  rosetta-Lauf messbar. Auslöser = Lauf `35270867738` success.
- **`auto-dispatch`** `wartend` — zuletzt falten, kein Löschen vor Parität.

## Quellen-Routen

- **gedi/icesat2/swot protected-Bucket-403** `operator-gebunden` —
  EDL-Token an den drei `/s3credentials`-Endpunkten akzeptiert (HTTP 200), aber
  SigV4-`ListObjectsV2` je Bucket **403 `AccessDenied`** (explizites Deny auf
  `s3:ListBucket`, NGAP-Policy). Signatur/Region/Host korrekt. Schritt:
  CMR-Granule → direktes `GetObject` bauen **oder** Operator-Datenabkommen
  (SWOT-EULA) — `research-max`.
- **`ephemeris_epm` Format-Routing-Lücke** — Route fehlt (`extract.rs:1769`,
  `main_flow.rs:83,113`), Parser `parse_ephemeris_binary` (`motion.rs:517`)
  existiert. Gehört der `bau`-Linie: als Post-Zeile `An bau: …` in
  `docs/handover/post.md` gesetzt (diese Session), dort von `bau` zu falten. ·
  fremde Linie
- **`catalog_gaia_sso`** — geschlossen: `gaia-sso-cdn.yml` gebaut (dieses Atom),
  Asset `gaia_sso_tno.bin` present; kein Dispatch nötig (idempotent, A = A).

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `dc9291ad`; `origin/main == HEAD` (Fast-Forward).
- **CI** — Watchdog-Snapshot 22:26:12 überholt. Live (`ci_manage list`):
  rosetta Post-Fix `harvest` `35270867738` @ `dc9291ad` **queued**,
  `harvest-dispatch` `35270851153` @ `dc9291ad` **queued**; `auto-dispatch`
  `35270640949` success; die Snapshot-Failures (`35270709861`, `35269148125`)
  tragen `head_sha d101b20a` = **vor** dem `fromJSON`-Fix. Viele `*-cdn`
  queued/in_progress (lis-otd, swot, gedi, icesat2, ionex, igets, …).
- **Postfach** — `state/mail/` am Baum absent, kein externer Eingang.
  `docs/zustand/external-state.md` (Postfach- + CI-Zeile) fällig durch
  HEAD-Wechsel — **fremd-modifiziert** (entscheid-Linie), nicht angefasst.
- `git_safety --snapshot` → `refs/safety/1789676886`.

## Benchmark

- **`grind-pro`** (Argument-Punkt) — $0.0751; `harvest_reg.rs`-`args` +
  `harvest.yml`-Weitergabe, Generische-Keys-Entscheid, `cargo check` 0/0.
- **`grind-flash`** (gaia-sso-cdn) — $0.0049; `.github/workflows/gaia-sso-cdn.yml`
  aus `gaia-cdn.yml`-Vorlage. Kein gedoppelter Lauf (zwei verschiedene Aufgaben,
  keine Benchmark-Klasse) — nur die Burns gemessen.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/harvest.yml`,
  `tools/utils/src/bin/harvest_reg.rs`, `.github/workflows/gaia-sso-cdn.yml`,
  `phi/harvest.φ`,
  `docs/handover/handover-2026-09-17-ernte-folge71.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge70.md`).
- **Fremd (nicht anfassen):** `docs/handover/post.md` (entscheid-Linie,
  modifiziert — eigener bau-Hunk darin, **nicht** mitcommittet),
  `docs/zustand/external-state.md` (modifiziert), die gestagten
  Handover-Archiv-Renames (`handover-2026-09-16-*`),
  `handover-2026-09-17-entscheid-folge37.md` (gestagt) und
  `handover-2026-09-17-entscheid-folge38.md` (untracked). Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
