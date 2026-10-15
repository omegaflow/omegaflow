<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: d5b6b071e50b54a02adf9de7b9ad3d69cef2cdc05d52e95db2e0081b5307ecd4
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge356)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `db83de03b` (== `origin/main`) · geschrieben 2026-10-15 · **Mycelium-356**.

## Rote Läufe (gemessen, nicht geraten)

`ci_manage reds` (einmal, kein Polling): **56** non-success Läufe — die Tafel ist ein
**Flutstand** aus den parallel laufenden Linien (River/Mountain committen im selben Baum).
Träger und gemessene Fehler-Jobs (`ci_manage jobs`):

| Träger | Workflow | gemessener Fehler-Job | Beispiel-run-id |
|---|---|---|---|
| fremd (Register-Gate) | `register-coverage` | `orphans` | `38733981918` @ `fd9621fa4` |
| fremd (Gate) | `ci-gate` | `dropped-gate` · `register` · `clippy` · `format` · `subset` | `38730439902` @ `e19e5cd20` |
| fremd | `tools-build` | `build` | `38718639886` @ `244be15ff` |
| fremd | `matrix-rotor` | — | `38732211774` @ `99812000c` |
| fremd | `quake-feeds-cdn` | — | `38729743881` @ `fcc4646aa` |

Der Log-Grund je Lauf tiefer ist **unread** (kein Polling); er wird via `ci_manage log <id>`
gelesen. Meine eigenen Push-Läufe am neuen HEAD `8a7b48e0e` sind **queued/unread**
(`ci-gate` `38734329244`/`38734315042` · `register-coverage` `38734329217`).

## Lokal gemessen am HEAD `8a7b48e0e`

- **F4′-Quellenmessung (weite Quellen-Leiter über Zenodo/OpenAlex):** der **globale (sub-)stündliche**
  Kanal ist **frei** — **WGLC** (`10.5281/zenodo.4774528`, CC-BY-4.0, stündlich (0.5°/5′) +
  30-Min-Stoßleistung, 2010–2025, detection-efficiency-korrigiert) und **FLASHMAP**
  (`10.5281/zenodo.17376946`, CC-BY, GLD360 global). Zwei frühere Messungen („Monatsmittel bzw.
  regional") waren zu eng.
- `register_lookup --orphans` = 0; `--orphan-docs` = **28** Dokumente (offene Marker, kein
  Live-Handover-Träger); `--tokens` = 12 unbeglichen, 0 XFER-MISSING.
- `open_points_check` folge356 = 0 stale-citations.

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` (Operator-Hand, Route (b)).
  `register_lookup --fired mycelium` = 1 (dieser Punkt; bereits gewortet — **nicht** vorgelegt).

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist 2026-10-14 (unverändert gegenüber folge355).

## Orphan-Zensus

- `register_lookup --orphans` = 0 (fremd-unabhängig). `--orphan-docs` = 28 (Träger fehlt; gemessen).

## Mehrfach-Träger

- Offen (in `mycelium-folge356`): `xfer:mycelium-356-1` (an Mountain) · `xfer:mycelium-356-2`/`-3`/`-4`
  (an River) · `xfer:mycelium-352-1`/`-2`/`-3`/`-5` (an Mountain).
- Empfangen-unbeglichen aus `mountain-folge353`: `xfer:mycelium-352-1`/`-3`, `xfer:mountain-348-1`,
  `-351-1`, `-352-1`/`-2`/`-3`/`-4`.
- Gefaltet/geschlossen: mountain-folge351, river-folge215.

## Gearbeitet (dieses Rund)

- **F4′-Quellenmessung** (korrigiert, s. oben): der **globale** Kanal ist frei (**WGLC**, CC-BY) —
  der Blitz-Kanal ist **nicht** blockiert; Mycelium registriert WGLC, das an River benannt
  (`xfer:mycelium-356-3`).
- folge356 geschrieben; folge355 → `archiv/`; die An-River/An-Mountain-`xfer`-Tokens gesetzt.
- Operator-Worte dieses Runds in `state/operator-gespraeche/2026-10-15-mycelium.md` geschnitten.

## Snapshot / Tools

- `git_safety --snapshot` gesetzt: `refs/safety/1792067061` (dieses Rund).
- `.tools_ensure`: Release-Route `tools-latest` HTTP 302 (erreichbar, gemessen via
  `session_burn`/`ci_manage`); `tools-build` ist rot (Träger fremd) — die Artefakt-Builds
  gehören CI.
- Push steht (HEAD == `origin/main` == `8a7b48e0e`).

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
