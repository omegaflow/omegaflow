<!--
  title: Handover — Forschung-Folge 120 (Stand 2026-09-20)
  session: Forschung-Folge 120
  class: handover
  date: 2026-09-20
  sha256: 9040ebede0f64b39f5622a969d57362947b82e325c36f18ca201129fd426462d
  status: live
-->
# Handover — Forschung-Folge 120 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 120)

- **HEAD** bei Session-Beginn `cf6b9799` (== `origin/main`, folge119-Commit);
  während der Session zog die ernte-Linie auf `e7e91da5` nach (fremder Commit,
  unangetastet). Baum trägt fremde uncommittete Arbeit — **nicht angefasst**:
  `post.md` (M), `external-state.md` (M, entscheid/bau-Hunks), `bau-folge111.md`
  (??), `entscheid-folge64.md` (??), `tools/measure/free_models.tsv` (??),
  `tools/measure/src/bin/free_model_bench.rs` (??), gestagte Moves
  bau110/entscheid63 → `archiv/`.
- **Postfach** — kein neuer Ledger-Eingang seit `1789922257` (`sales@pine64.org`:
  Verweis auf `info@pine64.org`). Diese Session: Hardware-Anfrage an
  `info@pine64.org` gesendet (`sent_ledger`, id `01a0bff3-85c2-7588-9a11-5be596119107`).
- **CI** — `ci_manage view`: `ci-check` `35526753935` @`cf6b9799` **pending**
  (kein Verdikt); `te-gate` `35513982359` @`7d0a1272` **in_progress** seit 13:36Z
  (kein Update). Der frühere Rot `ci-check` `35520538766` @`fa44f315`
  (`cargo test --lib` exit 101) ist durch `84aa5147` geschlossen; `35517957987`
  failure liegt auf Branch `v2026-09-20`, nicht `main`. `ci-check`-Cancels =
  Concurrency-Gruppe `ci-check-<ref>` (`cancel-in-progress:false` cancelt den
  älteren *pending* Lauf beim nächsten Push) — erwartet, kein Defekt.
- **Chrome-DevTools MCP** (`@1.9.0`, Telemetrie-Flags) — am lebenden
  `browser_relay`-Membran-Lauf verifiziert: Konsole/Netz via CDP gelesen.

## Punkt 3 — Chrome-DevTools MCP Membran-Lauf (verifiziert, Fix gebaut)

- Membran lief (`./target/debug/omegaflow`, browser_relay, Debug-Binär vom
  2026-09-18 — **stale**, Release-Binär hat die Feature nicht). Server
  `http://127.0.0.1:1618` → HTTP 200; Page geladen, CDP gelesen:
  `warn: No available adapters` (headless Chrome ohne WebGPU-Adapter —
  Umgebung, kein Membran-Defekt), `error 404` ×4.
- **Befund (der eigentliche Defekt):** `static/index.html` importiert
  `./palette.js`, `./webserial.js`, `./sensorium.js`, `./radiator.js` (Zeilen
  561–570, je in `try/catch`); die Relay-Route-Tabelle in
  `src/archivar/relay.rs` bediente nur `/`, `/time`, `/station`, `/jump/`,
  `/field`, `/constants.js`, `/crash`, `/consent`, `/sources` → die vier
  Module 404 → Sensorium/Radiator/WebSerial bleiben im Browserpfad inert
  (still verschluckt, nicht sichtbar). Seit `14ccc152` (sensorium + radiator
  split) fehlte die Route.
- **Fix:** neue Route in `relay.rs` — serve jede top-level `.js` aus `static/`
  (`path.ends_with(".js") && !path.trim_start_matches('/').contains('/')` →
  `resolve_asset("static<path>")`, `application/javascript`, sonst 404).
  `cargo check` und `cargo check --features browser_relay`: 0 Fehler, 0 Warnungen.
- **Offen:** funktionale Verifikation braucht einen frischen
  browser_relay-Build (CI oder lokal) — das Release-Binär hat die Feature nicht,
  das Debug-Binär ist stale. (Schritt: `gh workflow run ci-check.yml` nach dem
  Push; ein Membran-Lauf gegen `static/*.js` 200.)

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `ci-check` HEAD-Verdikt `35526753935` @`cf6b9799` | wartend | eigen | `ci_manage view 35526753935` (Trigger: Lauf-Abschluss) |
| 2. TE-Gate-Verdikt `35513982359` @`7d0a1272` | wartend | eigen | `ci_manage view 35513982359` (in_progress seit 13:36Z; success → vier `fpr_rise_sigma_test`-Zeilen via `ci_manage log`) |
| 3. relay `static/*.js`-Fix funktional verifizieren | wartend | eigen | frischer browser_relay-Build + Lauf gegen die vier Module (CI/local); `gh workflow run ci-check.yml` |
| 4. Browser-Anbindung Rest | operator-gebunden | operator | Extension „OpenCode Browser" (`cabnfapnafjlijmbpmgjkgobhdkbmpci`) im Operator-Profil öffnen, „connected" bestätigen; dann Cookie-Editor-Transfer Operator-Profil ↔ Playwright |
| 6. Flyby-Path-2-Kette | termin:2026-09-28 | termin | Zellen ab Perigäum füllen (`ernte`/`research-max`) |
| 7. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 8. BepiColombo MORE | termin:2027-04 | termin | Freigabe Wissenschaftsphase |

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/relay.rs` (neue top-level `.js`-Route)
- `docs/handover/handover-2026-09-20-forschung-folge120.md` (neu)
- Move `handover-2026-09-20-forschung-folge119.md` → `archiv/` (eigene Linie, atomar)
- `state/mail/sent_ledger.φ` (gitignored, nicht getrackt)

## Benchmark

- Kein Doppellauf in diesem Atom — die CI-/Browser-Messung war mechanisch
  (`ci_manage`/CDP), kein Agentenvergleich. Sieger-Klasse bleibt die gemessene
  Routine-Klasse (flash).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
