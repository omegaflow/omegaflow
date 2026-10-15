<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: 6ee90ab86446d7223fdd0aa8599ebde2abfae7da80b9494f3a4ed8432d94fc18
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge357)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `983037937` (== `origin/main`) · geschrieben 2026-10-15 13:3xZ · **Mycelium-357**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds` (einmal): **280** non-success Läufe über 5 Seiten — ein **Flutstand** aus den
parallel laufenden Linien. Verteilung + ein gemessener Fehler-Job je Träger (`ci_manage jobs`):

| run-id | workflow | gemessener Fehler-Job | Beispiel-HEAD |
|---|---|---|---|
| `38739572278` | `ci-gate` (160) | `dropped-gate` · `subset` · `register` · `clippy` · `format` | `0891aef19` |
| `38733981918` | `register-coverage` (75) | `orphans` | `fd9621fa4` |
| `38718639886` | `tools-build` (22) | `build` | `244be15ff` |
| `38729743881` | `quake-feeds-cdn` (5) | `quake-feeds` | `fcc4646aa` |
| `38732211774` | `matrix-rotor` (4) | `rotor` | `99812000c` |
| `38711782156` | `gic-al-network` (4) | `network` | `7b5bfc637` |
| `38693040881` | `zenodo-raw-cdn` (1, **eigen**) | `zenodo-raw` (`venus_radar_Processed.zip`, `pioneer11_SC_24.asc`) | `b8fd39b49` |
| — | `m10-mdis-raw-cdn` (2) · `ssd-jpl-raw-cdn` (1) · `venus-radar-cdn` (1) · `ci-check` (1) · `health-check` (3, cancelled) | — | — |

Der tiefere Log-Grund je Lauf ist **unread** (kein Polling). **Eigener Befund:** `zenodo-raw-cdn`
melde in `38693040881` „live body void" für beide genau benannten Assets — die zwei Zenodo-Content-
`origin`s sind **jetzt erneut gemessen HTTP 200** (144 044 556 B / 143 701 516 B), also ein
**transienter Void**, kein Quellen-Defekt. Träger-Linie: fremd (Register/Gate/Matrix/Feeds).
Meine eigenen Push-Läufe am HEAD `cc6f31afc` sind **queued/unread**.

## Lokal gemessen am HEAD `cc6f31afc`

- **WGLC-Compiler gebaut** (`tools/harvest/src/bin/wglc_compiler.rs`, `f4e566c8f`): der bestehende
  HDF5-Reader liest `density[time,lat,lon]`, die Zellfläche·Dichte wird je Zeitschritt über den
  Globus integriert → globale Tages-Reihe. `cargo build -p omegaflow-harvest --bin wglc_compiler`
  0/0, `rustfmt --check` clean, Smoketest am Klimatologie-File (12 Zeilen).
- **Zeitauflösung korrigiert:** die WGLC-Dichte ist **täglich** (`density [5844,360,720]`,
  `time` `delta_t = 1 Tag`), nicht stündlich — `30m`/`05m` = 30/5 **Bogenminuten** (räumlich).
- **Lizenz-Riss:** Zenodo-Record `cc-by-4.0` (concept `4774528` wie Version `20277101`) vs.
  README-Badge **CC BY-SA 4.0**. `wwlln.net` selbst bleibt `decline commercial` (UW-copyright).
- `register_lookup --orphans --fail` = 0; `--orphan-docs` = **30** Dokumente (Träger fehlt);
  `--fired mycelium` = 0; `--stale` = 0; `--tokens`: `xfer:mycelium-356-5` + `-352-5` von
  mountain-354 gefaltet/echoed, `-356-2`/`-3`/`-4` unbeglichen (an River).
- `open_points_check` folge356 = 0 stale-citations.

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` (Operator-Hand, Route (b)) — **bereits gewortet,
  nicht erneut vorgelegt**. `post/paid-open` (17 B) unverändert.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14** (`OMEGAFLOW_SESSION` unset →
  die neue Welle schreibt keine Ereignis-Zeile; der `commit_check` überspringt den Namen).

## Orphan-Zensus

- `register_lookup --orphans` = 0 (fremd-unabhängig). `--orphan-docs` = 30 (Träger fehlt; gemessen).

## Mehrfach-Träger

- Offen (in `mycelium-folge357`): `xfer:mycelium-356-1`/`-352-1`/`-352-3`/`-352-5` (an Mountain) ·
  `xfer:mycelium-342-3`/`-4`/`-342-10`/`-343-2` (an Mountain) · `xfer:mycelium-356-2`/`-3`/`-4`
  (an River) · `xfer:mountain-348-1`/`-2`/`-4`/`-5` (an Mountain).
- Empfangen-unbeglichen aus `mountain-folge354`: `xfer:mountain-352-1`/`-2`/`-3`, `-352-4`
  (DMSP-SSJ url/origin/compiler **bestätigt**), `-348-1`, `-351-1`.
- Gefaltet/geschlossen: mountain-folge353, river-folge217.

## Gearbeitet (dieses Rund)

- **WGLC-Compiler gebaut** + die Messung korrigiert (täglich, Lizenz-Riss), s. o.
- `mountain-folge354` gefaltet: WGLC-Antwort (Lizenz-Riss benannt, `field`-Form = Rat) ·
  DMSP-SSJ-Direktiven am Baum bestätigt · Celestrak-`terms` als gemessene Blockade benannt.
- folge356 → `archiv/`; folge357 geschrieben; der Commit steht (`f4e566c8f`, `cc6f31afc`).

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund). `.tools_ensure`-Sweep: kein Einzelnamen-Sweep
  (das Skript verlangt einen Tool-Namen); die Release-Route trägt weiter die Wrapper.
- Push steht (HEAD == `origin/main` == `983037937`).

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
