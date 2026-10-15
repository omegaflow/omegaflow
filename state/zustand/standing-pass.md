<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: dc2b15d312b36c2d83ac37c42390867adf6610d71774fc6a7d05969966e80b31
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge359)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `5923562d2` (== `origin/main`) · geschrieben 2026-10-15 17:35Z · **Mycelium-359**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds` (einmal): **283** non-success Läufe — ein **Flutstand** aus den parallel laufenden
Linien. Verteilung (gemessen, Summe 283) + ein gemessener Fehler-Job je Träger (`ci_manage jobs`):

| workflow (n) | gemessener Fehler-Job |
|---|---|
| `ci-gate` (173) | `clippy` · `register` · `dropped-gate` · `format` · `subset` (`build` success) |
| `register-coverage` (72) | `orphans` → `--tokens --noecho-fail`: **15 unbeglichen, 3 XFER-MISSING** (7 blocks) |
| `tools-build` (23) | `build` |
| `gic-al-network` (4) | — |
| `quake-feeds-cdn` (3) | — |
| `m10-mdis-raw-cdn` (2) · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `ci-check` · `zenodo-raw-cdn` · `health-check` · `venus-radar-cdn` | — |

Der tiefere Log-Grund der fremden Träger ist **unread** (kein Polling). **Eigener Befund:** die
`register-coverage`-Naht scheitert am `--tokens`-Gate (fremde + eigene unbeglichene Xfer) und am
`--orphan-docs`-Carrier-Mangel (29); die eigene folge359 fügt **keine** neue XFER-MISSING hinzu.
Push-Läufe am neuen HEAD sind **queued/unread**.

## Lokal gemessen am HEAD `5923562d2`

- **Der Tauchauftrag lief (Mycelium-359): acht `taucher`, ein Faden je Dispatch, harte Bandagen.**
  Die vollständigen Messungen stehen in `docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md`.
  Kern: HMI (JSOC steht, `hmi`-Kanal fehlt; `hmi_polar`-Tag-Riss `jsoc1` vs. `ssd.jpl`), SILSO
  (Riss: Git-Verdikt `decline unphysical` + `port.rs:2048` + `specs/sunspots.md` vs. „Lücke"),
  XUV/logR'HK (RAVE/GALAH tragen keinen Index; EVE im Code, register-/CDN-los; `phi:9898` HTTP 400),
  Shear (KiDS 16,50 GiB + Conicus 32,04 GiB; alle CDN-`url` 404; `tar_zst`/`hips3d` ohne Reader),
  Neutrino (**Code-Riss**: vier Compiler verlangen `S2Direction`, `witness.rs` gibt `PointEvent` →
  Zeile-1-Abbruch, Grund der 404), EDGES (Quellen öffentlich; Lücke in phi), PTA (kein
  Residuum im Zip; CDN 404), TGF/Wolken (FSSC PD-Katalog mit sha256; Open-Meteo keyless).
- Register-Nullstände (nachgemessen): `hmi`/`silso`/`sunspot`/`neutrino`/`lensing`/`eve_`/`format eve`
  = 0; `hips3d` = 2, `tar_zst` = 1 (nur die jeweiligen Format-Zeilen).
- `register_lookup --orphans --fail` = **0**; `--orphan-docs` = **29**; `--fired mycelium` = 0;
  `--stale mycelium --persist 3` = 0; `--addressed mycelium` = 2 (mountain-folge356, river-folge221 —
  gefaltet).
- folge358 → `archiv/`; folge359 geschrieben; Commit `5923562d2` gepusht.

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` — **bereits gewortet, nicht erneut vorgelegt**.
  `post/paid-open` (27 B) — fremde Linie; nur die Größe gelesen, kein Inhalt.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14** (`OMEGAFLOW_SESSION` unset →
  die neue Welle schreibt keine Ereignis-Zeile; der `commit_check` überspringt den Namen).

## Orphan-Zensus

- `register_lookup --orphans` = 0. `--orphan-docs` = 29 (Träger fehlt; gemessen).

## Mehrfach-Träger

- Neu (folge359): `xfer:mycelium-359-1 … -8` (acht Fäden → Mountain). Offen aus früheren Runden:
  `xfer:mycelium-352-5` (Mountain) · `xfer:mycelium-356-5` (echoed) · `xfer:mountain-354-1`
  (echoed) · `xfer:mycelium-352-1`/`-2` (echoed) · `xfer:mountain-348-1` (Mountain) ·
  `xfer:mycelium-356-3` (River) · `xfer:river197-6` (echoed).
- `river219-de2spdf` / `-sabergats` / `-juicespice` → eigene Hol-/Ablage (kein Token);
  `river219-gic2` → River selbst.
- Gefaltet/geschlossen: mountain-folge356, river-folge221 (dieses Rund); mountain-folge355,
  river-folge219 (früher).

## Gearbeitet (dieses Rund)

- Der Tauchauftrag der acht Fäden ausgeführt (je ein `taucher`, harte Bandagen) — Survey
  `survey-2026-10-15-tauchbefunde-acht-faeden.md`; folge359 mit acht Mountain-Anfragen.
- **Kern der Korrektur bestätigt:** für HMI/XUV/Shear/Neutrino/PTA steht die Quelle — der Kanal
  fehlt; für SILSO/EDGES/TGF/Wolken fehlt die Quelle.
- **Zwei harte Risse benannt:** (a) der Neutrino-Compiler-`S2Direction`-Riss (Kreuz-Riss, ein Atom
  zwei Linien); (b) der `hmi_polar`-Tag-Riss (`jsoc1` unregistriert vs. Asset `ssd.jpl`).

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund, `refs/safety/1792084075`). `.tools_ensure`-Sweep:
  das Skript verlangt einen Tool-Namen (kein Einzelnamen-Sweep); die Release-Route trägt weiter die
  Wrapper.

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
