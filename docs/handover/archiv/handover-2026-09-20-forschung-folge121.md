<!--
  title: Handover — Forschung-Folge 121 (Stand 2026-09-20)
  session: Forschung-Folge 121
  class: handover
  date: 2026-09-20
  sha256: 3cc5aab17ffd4ca35d1623a8b215592cbeb364b71591a0c038b5574ebebf833d
  status: live
-->
# Handover — Forschung-Folge 121 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein
Auswahlpunkt — sie nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 121)

- **HEAD** `e09a996b` (== `origin/main`, entscheid folge65); Arbeitsbaum == HEAD
  (`git_safety --snapshot`: nichts zu sichern).
- **Postfach** — kein neuer Ledger-Eingang seit `1789922257` (`sales@pine64.org`,
  Verweis auf `info@pine64.org`); `smail` im line-Profil nur per ask, der
  Zustand-Eintrag zitiert.
- **CI** — `ci-check` `35527911970` @`e09a996b` in_progress; `te-gate`
  `35513982359` @`7d0a1272` in_progress seit 13:36Z (kein Update); `ci-check`
  `35526010713` @`b9112e5d` failure: (a) Clippy `question_mark`
  (`src/archivar/extract.rs:3425`) an HEAD bereits gefixt (`(*dist_scale)?`),
  geschlossen; (b) `path_reference_scan`-Test rot (7 tote `see-also` + 4 absolute
  Pfade) — in diesem Atom repariert.

## Punkt 1 — `path_reference_scan`-Health-Gate (repariert, CI-Verdikt offen)

Der Test `committed_tree_has_no_broken_references_and_no_absolute_paths`
(`tools/register/src/bin/path_reference_scan.rs`) ist der Health-Gate des
getrackten Baums. Repariert (gemessen, 8 Dateien):

- tote `see-also`-Ziele auf `archiv/`-Pfade korrigiert:
  `docs/paper/tonga-lamb-crosscheck.md:7`,
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md:7`,
  `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md:7`.
- nirgends auflösbare Ziele entfernt: `auftrag-flyby-doppler-rohdaten.md` (in
  `035a9191`/`1db52c0e` gelöscht), `phi/pipeline/refusal_ledger.φ` (gitignored,
  `.gitignore:66` — im Fresh-Checkout tot).
- absolute Pfade ersetzt: `survey-2026-09-17-omegaflow-legacy-konzepte.md:18`,
  `survey-2026-09-17-verlorene-diskussionen.md:17,27`,
  `phi/pipeline/index.φ:126` → repo-relative/`archive-root`-Form.
- `granit.md`→`docs/granit.md` (`tools/gate/src/bin/auftrag_header.md:6`),
  `master.md`→`docs/specs/master.md` (`…omegaflow-legacy-konzepte.md:7`).
- Fremde Docs (auftrag/entscheid/bau/gate) im selben Atom mitrepariert — der Gate
  ist geteilt; ohne sie bliebe der Test rot.

**Offen:** Verdikt des `ci-check`-Laufs auf dem gepushten HEAD (Trigger
Lauf-Abschluss; `ci_manage view <id>`).

## Punkt 2 — Relay `static/*.js`: Auslieferung + silent Verifikation (gebaut, CI-Verdikt offen)

Befund (gemessen): `browser_relay` ist ein Marker-Feature (`Cargo.toml:24`);
`tools-build.yml:22` und `release-build.yml:42` bauten `omegaflow` OHNE Feature —
die committede Route (`src/archivar/relay.rs:417-423`) war im ausgelieferten
Binär wegkompiliert (Riss: Quelle trägt die Route, das Ding nicht). Der
`ci-check`-Test-Job lief ohne Feature (`cargo test --release`), `relay.rs` wurde
dort nicht kompiliert.

- **Gebaut (Rat 2026-09-20, Empfehlung (c) — beides):**
  `tools-build.yml:22` und `release-build.yml:42` tragen `--features browser_relay`;
  `ci-check.yml:55` → `cargo test --release --features browser_relay`; neuer
  silenter Routentest `top_level_static_js_routes_reach_the_asset` in
  `src/archivar/relay.rs` (GET der vier `.js` → 200, `/does-not-exist.js` → 404;
  kein Fenster/GPU/Audio/Serial). Consent unverändert: `main_flow.rs:620` bindet
  den Relay an `!hidden` — Kompilieren ist kein Radiieren.
- `cargo check` / `--features browser_relay` / `--tests --features browser_relay`:
  0 Fehler, 0 Warnungen.
- **Offen:** Verdikt des `ci-check`-Laufs (Routentest grün?) und `tools-build`
  (Binär trägt das Feature) auf dem gepushten HEAD.

## Punkt 4 — Browser-Anbindung (Extension verbunden; Cookie-Transfer mechanisch gebaut)

Extension „OpenCode Browser" (`cabnfapnafjlijmbpmgjkgobhdkbmpci`) liegt in Chrome
**Profile 1** (Operator-Profil), verbunden (Operator-Wort), Brücke verifiziert:
`browser_targets` meldet das Chrome-Target, `browser_open` öffnet einen Tab ohne
Fokus, `browser_get_text`/`browser_get_html` lesen Inhalt. Bridge-Server
`127.0.0.1:4517` antwortet 426 (WebSocket-Handshake). Cookie-Editor
(`hlkenndednhfkekhgcdicdfddnkalmdm`) in Profile 1 installiert (Operator-Akt).

Der Cookie-Transfer ist mechanisch gebaut (Operator-Wort „bau B"):
`archive_search --playwright` liest `OMEGAFLOW_COOKIES=<cookie-editor.json>` und
setzt die Cookies via `context.addCookies` vor `goto` — headless wie headed.
Unset = keine Cookies (0 geehrt), gesetzt-aber-kaputt = benannter Abbruch
`exit(2)`. `node --check` + `cargo check -p omegaflow-utils --bin archive_search`:
clean. Werkzeug-Karte (`docs/concepts/tools-map.md`) fortgeschrieben.

- **Offen:** Der nächste `tools-build`-Lauf (nach Push) backt das neue
  `archive_search`-Binär (`include_str!` in `playwright.rs:5`); bis dahin trägt
  das Release-Binär den alten cjs. Der Operator-Export (Cookie-Editor → JSON) ist
  der eine Operator-Akt, wenn eine Pfad-3-Consent-Wand ihn auslöst.

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `ci-check`-Verdikt @`f75e3245` (path_reference_scan + Relay-Routentest) | wartend | eigen | `ci_manage view 35530480032` (Trigger Lauf-Abschluss) |
| 2. `tools-build`-Verdikt (Binär trägt `browser_relay` + `OMEGAFLOW_COOKIES`) | wartend | eigen | `ci_manage view 35530482159` (Trigger Lauf-Abschluss) |
| 3. TE-Gate-Verdikt `35513982359` @`7d0a1272` | wartend | eigen | `ci_manage view 35513982359` (success → vier `fpr_rise_sigma_test`-Zeilen) |
| 4. `archive_search`-Rebuild mit `OMEGAFLOW_COOKIES` (cjs `include_str!`) | wartend | eigen | `ci_manage view 35530482159` (tools-build) |
| 5. Cookie-Editor-Export (JSON für `OMEGAFLOW_COOKIES`) | operator-gebunden | operator | bei Pfad-3-Consent-Wand: Cookie-Editor → Export |
| 6. Hardware-Sponsoring Framework/Tuxedo/Pine64 | wartend | dritter | Trigger Antwort (Framework `NG2HWBZM`, Tuxedo `#991311279`, Pine64) |
| 7. Flyby-Path-2-Kette | termin:2026-09-28 | termin | Zellen ab Perigäum füllen |
| 8. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 9. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |

Kein abarbeitbarer undatierter Punkt bleibt: die Punkte 1–4 sind `wartend`
(Trigger Lauf-Abschluss), 5 ist `operator-gebunden`, 6/8 `wartend` (dritter),
7/9 datierte Wiedervorlagen.

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/ci-check.yml`, `.github/workflows/tools-build.yml`,
  `.github/workflows/release-build.yml`
- `src/archivar/relay.rs` (silenter Routentest)
- `docs/auftrag/auftrag-sonden-rohdaten-anfrage.md`,
  `docs/paper/tonga-lamb-crosscheck.md`
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`,
  `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`,
  `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md`
- `tools/gate/src/bin/auftrag_header.md`, `phi/pipeline/index.φ`
- `tools/utils/src/bin/archive_search/playwright_fetch.cjs` (`OMEGAFLOW_COOKIES`)
- `docs/concepts/tools-map.md` (Drei-Pfade-Tabelle + Cookie-Zeile)
- `docs/handover/handover-2026-09-20-forschung-folge121.md` (neu)
- Move `handover-2026-09-20-forschung-folge120.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (An entscheid), `docs/zustand/external-state.md`
  (CI-Status + Postfach-Zeile)

## Benchmark

- Punkt 1 (Doc-Ref-Reparatur) an `grind-flash` delegiert: mechanisch, ein Agent,
  kein Doppellauf — die gemessene Routine-Klasse (flash, 2026-09-16) ist der
  Sieger. Punkt 2 Diagnose an `general` (read-only), Bau an `grind-flash`,
  Architektur an `council`. Punkt 4 (Cookie-Injektion B) direkt gebaut (kleiner,
  klar umrissener cjs-Schnitt). Kein pro/max-Doppellauf in diesem Atom.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
