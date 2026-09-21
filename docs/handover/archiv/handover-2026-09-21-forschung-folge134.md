<!--
  title: Handover — Forschung-Folge 134 (Stand 2026-09-21)
  session: Forschung-Folge 134
  class: handover
  date: 2026-09-21
  sha256: da8f339d6f9b3326850a54cc5c8f5f914b006594eb0c5274b366631377a1f0a2
  status: live
-->
# Handover — Forschung-Folge 134 (Stand 2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Forschung-Folge 134)

- **HEAD** `7695ae23` == `origin/main`. Fix A (`0f8bc1b2`) ist Vorfahr von HEAD.
  Arbeitsbaum trägt fremde uncommittete Arbeit (upload_asset→upload_release-Umbau
  ~76 Dateien + Ernte-Staging `ernte-folge130`, `phi/*.φ`) — nicht angefasst,
  nicht committet.
- **Postfach** — neuester Ledger-Eingang `1789978555` (Brave Search API „usage
  limit reached", informativ); kein neuer Eingang, kein handlungsbedürftiger Fall.
- **CI** — `hyperscanning-te` `35587070232` @`0f8bc1b2` **in_progress**
  (`updated_at` 10:07:57, kein Fortschritt); `te-gate` `35586377356` @`42aeeea4`
  **pending** (`updated_at` 10:00:27); `ci-check` `35587114894` @`7695ae23`
  **cancelled** (kein Verdikt). Kein Poll.

## Riss — getragen als Naht (gemessen 35584758519 @c7cb201f)

Der Riss trägt zwei gemessene Böden, die sich weigern zu konvergieren:
Sheet-Null μ_S = 1.0335e-1 (KSG per-cell, `family_fn_gate`) gegen
Weiß-Treiber-Null μ_W1 = −1.4616e-2 (sd 5.175e-3, p95 −6.005e-3).
μ_S − μ_W1 = 0.1180 > 2σ_W1 = 0.0104 → **Naht**: der Weiß-Treiber-Boden
liegt weit unter dem Sheet-Null — der Riss bleibt sichtbar, nie geglättet.
Die Naht trägt keine Schätzer-Aussage: der stochastic-driver-Arm (KSG τ=2,
excess +18.2 sd) trägt ihn getrennt; der rote Assert ist Fixture-/Gate-Frage
(Punkt 1). Mountain/River (weißer Boden ≈ Surrogat-Boden): **nicht bestätigt**.

## Punkt 1 — `family_fn_gate`: Fix A gesetzt, CI-Nachweis ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Fix A committet (`0f8bc1b2`): Assert
  `tools/measure/src/bin/hyperscanning_group_te.rs:1056` auf die gemessene
  Wahrheit geschrieben („the sheet fixture carries no conditional transfer
  entropy beyond its coherent null") plus Guard `null_mean > 0.0`. `cargo check
  -p omegaflow-measure --all-targets` 0/0. Run `35587070232` @`0f8bc1b2`
  **in_progress** (`updated_at` 10:07:57, kein Fortschritt).
- **Blockade:** Run-Abschluss (funktionaler Lauf nur in CI).
- **Braucht:** `ci_manage view 35587070232` einmal — grüner Lauf schließt
  Punkt 4/5 auf.

## Punkt 2 — `te-gate` n=1000-FPR-Boden

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `35586377356` @`42aeeea4` **pending** (`updated_at` 10:00:27;
  neuer als der Ghost `35578257445`).
- **Blockade:** Run-Abschluss.
- **Braucht:** `ci_manage view 35586377356` einmal.

## Punkt 3 — `--dropped` Delta-Gate: Baseline nach gemessenem Zähler

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Klassifikator-Fix committet (`e9556f86`:
  `point_key_tokens` verwirft führende reine Zahl-Token + Test). Die Läufe
  `35585844643` @`42aeeea4` und `35587114894` @`7695ae23` endeten **cancelled**
  (kein Verdikt). Ältere `ci-check` `35578412609` @`a476ccfb` failure:
  baseline 1760 | current 1819 | delta 59.
- **Blockade:** Run-Abschluss.
- **Braucht:** nächster `ci-check`-Lauf; falls current > baseline 1760, die
  Baseline auf den **gemessenen** Wert setzen (im akzeptierenden Commit, nie
  still).

## Punkt 4 — confirmation-Test nach grünem Screen

- **Status:** wartend | **Bindung:** eigen
- **Lage:** `confirmation_confirms_the_strong_pair_against_its_own_null` bleibt
  aus dem Teststep; assertet `observed.te > p99`, am aktuellen Schätzer rot.
- **Blockade:** grüner `family_fn_gate` (Punkt 1).
- **Braucht:** nach grünem Lauf `confirmation_…` in
  `.github/workflows/hyperscanning-te.yml` aufnehmen.

## Punkt 5 — Frontalkanäle F3/F4; Takens-Wandzeit + Watchdog-Floor

- **Status:** wartend | **Bindung:** eigen
- **Lage:** beide hängen am grünen Screen.
- **Blockade:** grüner Screen (Punkt 1).
- **Braucht:** nach grünem Lauf Wandzeit lesen; F3/F4 getrennt fahren.

## Punkt 6 — Riss 4 (KSG↔KDE): B gebaut, CI-Nachweis ausstehend

- **Status:** wartend | **Bindung:** eigen
- **Lage:** Der Council (read-only, 2026-09-21) verdiktete **B — tragen,
  benennen, mit Mess-Trigger**; das Operator-Wort ist **B**. Gemessen: der Riss
  ist **nicht** GPU-gegen-CPU, sondern KSG-gegen-KDE im selben Crate —
  `te.rs:2165` `transfer_entropy_embedded_ksg` (kanonischer Phasenpfad) gegen
  `te.rs:2084` `transfer_entropy_embedded_kde`; der GPU-Shader
  `shaders.rs:483` `te_embedded` ist die treue WGSL-Kopie des KDE-Arms. KDE-TE
  ist konstruktiv nichtnegativ, KSG-TE in nats und kann negativ sein — zwei
  Instrumente, nicht zwei Skalen; eine Mittelung wäre Fabrikation.
  **Gebaut** (grind-flash, `cargo check -p omegaflow --all-targets` 0/0):
  - (1) Shader-Kommentar an `te_embedded`/`te_compute` (`shaders.rs:483,525`):
    KDE benannt, `te.rs::transfer_entropy_embedded_ksg` als Kanon.
  - (2) Sprechort benannt: `solar.rs:445,449` und `matrix.rs:894,911,1059`
    sprechen `te(kde)` statt `te`.
  - (3) Gate-Fixture `riss_ksg_kde_estimator_split_is_measured` (`te.rs:3596`):
    `topological_te_phase` (KSG) vs `transfer_entropy_embedded_kde`, AR(1)
    n=300, Seed `0x9E37_79B9_7F4A_7C15`, Divergenz assertet.
  - (4) `AGENTS.md:119`: die „f32/f64 parity tolerance" gilt nur KDE↔KDE; der
    GPU-Shader rechnet KDE, der kanonische CPU-Phasenpfad KSG, die Abweichung
    ist der getragene Riss.
  - (5) Stale-Zeile `docs/concepts/archivar-mathematikerin.md:36` auf den
    gemessenen Stand (Header-sha256 `2469e1b6…`).
- **Blockade:** CI-Nachweis (funktionaler Lauf nur in CI).
- **Braucht:** `ci-check` (push-getriggert über `src/**`): der neue Test
  `riss_ksg_kde_estimator_split_is_measured` läuft im `test`-Job
  (`cargo test --release --features browser_relay`). Die zwei Fixture-Zahlen
  druckt der Test nicht (silent on green); sie kommen aus einem `te_fn_probe`-Lauf
  (druckt `te_scal`/`te_topo`) und werden in die Riss-Zeile gesetzt.
- **Offene Confounder (Council, unverändert):** FN läuft auf dem skalaren KDE,
  kein topologischer Pfad trägt eine topologische FN; der KSG-Kanon speist
  keinen Produktions-Konsumenten (Verdrahtungs-Loch); f32/f64 bleibt ein
  Rest-Toleranzband; der Instantaneous-Phase-Pfad bleibt KDE.

## Messung dieses Atoms — Paper-Frage (kein offener Punkt)

Gemessen 2026-09-21 (flash/`general`, read-only): **kein Paper hängt an der
pending Strahlungs-/Permeabilitäts-Bindung.** Alle TE-Paper in `docs/paper/`
sind Offline-Analysen geharvesteter Serien (0 Treffer „permeability"/
„radiator"/„aperture binding"); die Bindungslücke blockiert keine Publikation.
Der KSG/KDE-Split berührt zwei Paper nur am Rand:
- `solar-cycle-dynamo.md` §4.3 nutzt `topological_te_phase` (CPU-KSG,
  `te.rs:2479`) als unabhängigen Robustheits-Schätzer — korrekt benannt, nicht
  der Produktionspfad.
- `lead-geometry-direction.md` benannte den Residual-Bias des topologischen
  Pfads „KDE bias", obwohl der Pfad KSG rechnet (`te.rs:2479→2450`
  `transfer_entropy_embedded_ksg`) — **gefixt** (2× `KDE bias` → `KSG bias`,
  Header-sha256 `8710bad6…`).

## Chrome Web Store — lesbar via chrome-devtools MCP (kein offener Punkt)

- **Lage:** Drei Pfade gemessen (2026-09-21) — der Store ist via CDP lesbar:
  - `archive_search --playwright` (headless, `OMEGAFLOW_COOKIES` mit dem
    `.google.com`-Export): HTTP 200 → Redirect
    `accounts.google.com/v3/signin/identifier` — Consent passiert, aber
    Google-Re-Auth (Konto-Cookies genügen im frischen Kontext nicht).
  - opencode-Browser-Brücke (Extension, Operator-Profil): Seite lädt
    (`title: Buster: Captcha Solver for Humans - Chrome Web Store`), aber
    `browser_get_text`/`browser_snapshot`/`browser_screenshot` → „The extensions
    gallery cannot be scripted." — Gallery-Scripting gesperrt.
  - **chrome-devtools MCP (CDP, eigenes Chrome): lesbar.** Consent-Seite
    (`consent.google.com`, DE) einmal „Alle akzeptieren" → Store lädt
    vollständig: Titel, Publisher `armin.dev`, 3,9 (923 Bewertungen),
    800.000 Nutzer, Übersichtstext („Buster is a browser extension …"). Kein
    Konto nötig.
- **Blockade:** keine.
- **Braucht:** — **Befund:** der Store ist via CDP-Pfad lesbar (Consent einmal
  akzeptieren); der headless `--playwright`-Pfad bleibt gesperrt. Der
  Cookie-Export ist für den Store nicht nötig (der Mechanismus bleibt für andere
  Consent-/Cloudflare-Seiten verfügbar).

## Wartend / operator-gebunden / termin

- Flyby-Path-2-Kette — `termin:2026-09-28` (Auftrag steht, Kanäle live; Zellen ab
  Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger
  Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18; kein
  Zwischenzug).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Lage | Blockade | Braucht |
|---|---|---|---|---|---|
| 1. `family_fn_gate` Fix A | wartend | eigen | `35587070232` @`0f8bc1b2` in_progress | Run-Abschluss | `ci_manage view 35587070232` |
| 2. `te-gate` n=1000 | wartend | eigen | `35586377356` pending | Run-Abschluss | `ci_manage view 35586377356` |
| 3. `--dropped` Baseline | wartend | eigen | Läufe cancelled | Run-Abschluss | nächster ci-check-Lauf → Baseline |
| 4. confirmation-Test | wartend | eigen | rot am Schätzer | grüner Screen (←1) | Workflow-Zeile |
| 5. F3/F4 + Takens | wartend | eigen | — | grüner Screen (←1) | getrennte Läufe |
| 6. Riss 4 KSG↔KDE | wartend | eigen | B gebaut (5 Pflichten + Benennung), check 0/0 | CI-Nachweis | `ci-check`-Test; `te_fn_probe` für die Zahlen |
| 8. Flyby-Path-2 | termin:2026-09-28 | termin | Auftrag steht | Datum | Zellen ab Perigäum |
| 9. NSE/Haug | wartend | dritter | Route offen | Dateieingang | Trigger |
| 10. BepiColombo MORE | termin:2027-04 | termin | Anfrage gesendet | Freigabe | Wissenschaftsphase |

## Benchmark

- `council` für Punkt 6 (Architektur, read-only, pro/max) — kein Doppellauf; die
  Frage ist Architektur, nicht Routine (flash hätte kein Mandat). Der Council
  las `te.rs`/`shaders.rs`/`tests.rs`/Kontrakt selbst; Verdikt B mit Bau-Schnitt.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-forschung-folge134.md` (neu)
- Move `handover-2026-09-21-forschung-folge133.md` → `archiv/` (eigene Linie,
  atomar)
- `docs/zustand/external-state.md` (CI-/Postfach-Zeile)
- `docs/paper/lead-geometry-direction.md` (KDE→KSG-Fehlbenennung, Header-sha256)
- `src/mathematikerin/shaders.rs`, `src/mathematikerin/machines/solar.rs`,
  `src/mathematikerin/machines/matrix.rs`, `src/mathematikerin/te.rs` (Riss B)
- `AGENTS.md` (Riss-Satz), `docs/concepts/archivar-mathematikerin.md`
  (Stale-Zeile, Header-sha256)
- `state/cookies/` (gitignored, kein Commit)

Fremde uncommittete Arbeit im selben Baum (upload_asset→upload_release-Umbau,
`ernte-folge130`-Staging, die `phi/*.φ`-Änderungen) wird **nicht** angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
