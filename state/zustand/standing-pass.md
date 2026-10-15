<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: 3b75445e163928a0a5182e9e355ef1d180497cbb44287e8ea67caa3c9b7b6bbd
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge358)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `b07e37f82` (== `origin/main`) · geschrieben 2026-10-15 17:00Z · **Mycelium-358**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds` (einmal): **279** non-success Läufe — ein **Flutstand** aus den parallel laufenden
Linien. Verteilung (gemessen) + ein gemessener Fehler-Job je Träger (`ci_manage jobs`):

| run-id | workflow (n) | gemessener Fehler-Job | Beispiel-HEAD |
|---|---|---|---|
| `38768669705` | `ci-gate` (170) | `clippy` · `register` · `subset` · `format` (`build`/`dropped-gate` success) | `024b341e3` |
| `38769414504` | `register-coverage` (68) | `orphans` → `--tokens --noecho-fail` (8 unbeglichen, 4 XFER-MISSING) | `c755d4b0c` |
| — | `tools-build` (23) | `build` | — |
| — | `gic-al-network` (4) · `quake-feeds-cdn` (2) · `matrix-rotor` (2) · `m10-mdis-raw-cdn` (2) · `health-check` (2) | — | — |
| `38767860985` | `themis_asi-fodg-cdn` (1, **eigen**) | `themis-asi-fodg` — gemessen: `gzip: …0830….pgm.gz: not in gzip format` (die 0830-URL antwortet 301/HTML) — **behoben** (`b20976b78`) | `5b2071569` |
| — | `zenodo-raw-cdn` (1, **eigen**) · `ssd-jpl-raw-cdn` (1) · `venus-radar-cdn` (1) · `ci-check` (1) | — | — |

Der tiefere Log-Grund der fremden Träger ist **unread** (kein Polling). **Eigene Befunde:** (a) die
rote `themis_asi-fodg-cdn`-Naht war die einzige gemessene eigene — **behoben** (gzip-Magic-Guard);
(b) `register-coverage` scheitert am `--tokens`-Gate (fremde + eigene unbeglichene Xfer) und am
`--orphan-docs`-Carrier-Mangel; die eigene folge358 fügt **keine** XFER-MISSING mehr hinzu. Meine
Push-Läufe sind **queued/in_progress/unread** — `zenodo-raw-cdn` spiegelt die erweiterte Matrix
(Venera-Polar, Provenienz, Arecibo 1988).

## Lokal gemessen am HEAD `b17570464`

- **Drei Venus-Bildträger gespiegelt + registriert** (`phi/sources.φ`, `zenodo-raw-cdn.yml`):
  **Venera-Polar** (193 701 536 B, sha256 `98d80740…`), **Provenienz-Raster** (89 312 099 B, sha256
  `6aa27235…`), **Arecibo 1988** (965 390 866 B, sha256 `fe5ed9f8…`). `register_sort` canonical
  über **2810** Blöcke; `cdn_reconcile --fail` clean; `license_census` unverändert.
- **Provenienz-Kopf gemessen** (HTTP-Range): klassisches TIFF, 18 775 × 9 388, **4-bit**,
  unkomprimiert, gekachelt 128×128 (10 878 Kacheln), GeoTIFF-GeoKeys; die **12 Klassen (0–11)** aus
  dem Sidecar-XML (0 NoData · 1 Pioneer · 2/3 Arecibo 2001/1988 · 4 Venera 15/16 · 5/6 Arecibo
  2015/2017 · 7–11 Magellan). **Arecibo 1988:** 30 032 × 31 973, 8-bit, unkomprimiert, 128er Kacheln.
- **80,9-GB-Basemap:** Rat-Entscheidung steht (byte-exakter `.parts`-Split), Ausführung braucht einen
  **streamenden Splitter** — `raw_mirror` gattert auf die vorab-bekannte Ganz-Sha256 (77-GB-Grenze).
- `register_lookup --orphans --fail` = 0; `--orphan-docs` = **30**; `--fired mycelium` = 0;
  `--stale mycelium --persist 3` = 0. `open_points_check` folge357 = 0 stale-citations.

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` — **bereits gewortet, nicht erneut vorgelegt**.
  `post/paid-open` (27 B, zuvor 17 B) — fremde Linie; nur die Größe gelesen, kein Inhalt.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14** (`OMEGAFLOW_SESSION` unset →
  die neue Welle schreibt keine Ereignis-Zeile; der `commit_check` überspringt den Namen).

## Orphan-Zensus

- `register_lookup --orphans` = 0. `--orphan-docs` = 30 (Träger fehlt; gemessen).

## Mehrfach-Träger

- Offen (in `mycelium-folge358`): `xfer:mycelium-352-5` (Mountain) · `xfer:mycelium-356-5` (echoed) ·
  `xfer:mountain-354-1` (echoed) · `xfer:mycelium-352-1`/`-2` (echoed) · `xfer:mountain-348-1`
  (Mountain) · `xfer:mycelium-356-3` (River) · `xfer:river197-6` (echoed).
- `river219-de2spdf` / `-sabergats` / `-juicespice` → eigene Hol-/Ablage (kein Token);
  `river219-gic2` → River selbst.
- Gefaltet/geschlossen: mountain-folge355, river-folge219.

## Gearbeitet (dieses Rund)

- Venera-Polar, Provenienz und Arecibo 1988 gespiegelt + registriert; Provenienz-Kopf + Klassen
  gemessen; `themis_asi-fodg-cdn`-Naht repariert.
- **Basemap-Formatentscheidung in der Linie gefällt** (kein Rückfrage-Punkt): byte-exakter
  `.parts`-Split (Rat-Form), md5-Ganzes + sha256/Teil; bis der streamende Splitter steht,
  provisorische HTTP-Range-Lesung vom Origin-COG.
- **River-Übergabe gefaltet:** DE-2/SPDF, SABER/GATS, JUICE/SPICE sind eigene Hol-/Ablage-Aufgaben
  (kein Token); GIC-2 trägt River selbst.
- folge357 → `archiv/`; folge358 geschrieben; Commits `cf6c8b4d9` … `b17570464` / `b07e37f82` gepusht.

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund). `.tools_ensure`-Sweep: das Skript verlangt einen
  Tool-Namen (kein Einzelnamen-Sweep); die Release-Route trägt weiter die Wrapper.

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
