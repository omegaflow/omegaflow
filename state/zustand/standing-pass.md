<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: 086a0a268d72124d37be743dfb00c926a7680bd4d44419c84c9fb319b69167c6
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge358)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `51bcd7fe1` (== `origin/main`) · geschrieben 2026-10-15 16:35Z · **Mycelium-358**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds` (einmal): **271** non-success Läufe — ein **Flutstand** aus den parallel laufenden
Linien. Verteilung (gemessen) + ein gemessener Fehler-Job je Träger (`ci_manage jobs`):

| run-id | workflow (n) | gemessener Fehler-Job | Beispiel-HEAD |
|---|---|---|---|
| `38768669705` | `ci-gate` (164) | `clippy` · `register` · `subset` · `format` (`build`/`dropped-gate` success) | `024b341e3` |
| `38769414504` | `register-coverage` (64) | `orphans` → `--tokens --noecho-fail` (8 unbeglichen, 4 XFER-MISSING) | `c755d4b0c` |
| — | `tools-build` (23) | `build` | — |
| — | `gic-al-network` (4) · `quake-feeds-cdn` (3) · `health-check` (3, in_progress) | — | — |
| — | `matrix-rotor` (2) · `m10-mdis-raw-cdn` (2) | — | — |
| `38767860985` | `themis_asi-fodg-cdn` (1, **eigen**) | `themis-asi-fodg` — gemessen: `gzip: …0830….pgm.gz: not in gzip format` (die 0830-URL antwortet 301/HTML) | `5b2071569` |
| — | `zenodo-raw-cdn` (1, **eigen**) · `ssd-jpl-raw-cdn` (1) · `venus-radar-cdn` (1) · `ci-check` (1) | — | — |

Der tiefere Log-Grund der fremden Träger ist **unread** (kein Polling). **Eigene Befunde:** (a) der
`themis_asi-fodg-cdn`-Fehler war die einzige gemessene eigene rote Naht — **behoben** (gzip-Magic-
Guard, `b20976b78`); (b) `register-coverage` scheitert am `--tokens`-Gate (fremde + eigene
unbeglichene Xfer) und am `--orphan-docs`-Carrier-Mangel; die eigene folge358 fügt **keine**
XFER-MISSING mehr hinzu. Meine Push-Läufe an `3188d3a88`/`51bcd7fe1` sind **queued/in_progress/
unread** — `zenodo-raw-cdn` (`38769841043`) spiegelt gerade (pioneer10_SC_23.asc, danach u. a. die
Venera-Polar).

## Lokal gemessen am HEAD `51bcd7fe1`

- **Venera-Polar gespiegelt + registriert** (`phi/sources.φ`, Block
  `zenodo.org/Venus_Venera_Backscatter_Global_Polar_840m_v1.tif`; 193 701 536 B, sha256
  `98d80740…`; `register_sort` canonical über **2808** Blöcke; `cdn_reconcile --fail` clean,
  334 registry hosts; `license_census` unverändert — `CC-BY-4.0` im geschlossenen Satz).
- `register_lookup --orphans --fail` = 0; `--orphan-docs` = **30** (Träger fehlt, fremd);
  `--fired mycelium` = 0; `--stale mycelium --persist 3` = 0.
- `open_points_check` folge357 = 0 stale-citations (1 ABSENT: Future-Archiv-Pfad, mechanisch;
  20 format-gaps unverändert).
- **`themis_asi-fodg-cdn`-Naht repariert** (gzip-Magic-Guard; gemessener Grund s. o.).

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` — **bereits gewortet, nicht erneut vorgelegt**.
  `post/paid-open` (jetzt 27 B, zuvor 17 B) — fremde Linie; nur die Größe gelesen, kein Inhalt.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14** (`OMEGAFLOW_SESSION` unset →
  die neue Welle schreibt keine Ereignis-Zeile; der `commit_check` überspringt den Namen).

## Orphan-Zensus

- `register_lookup --orphans` = 0. `--orphan-docs` = 30 (Träger fehlt; gemessen).

## Mehrfach-Träger

- Offen (in `mycelium-folge358`): `xfer:mycelium-352-5` (Mountain) · `xfer:mycelium-356-5` (echoed) ·
  `xfer:mountain-354-1` (echoed) · `xfer:mycelium-352-1`/`-2` (echoed) · `xfer:mountain-348-1`
  (Mountain) · `xfer:mycelium-356-3` (River) · `xfer:river197-6` (echoed).
- Gefaltet/geschlossen: mountain-folge355, river-folge219.

## Gearbeitet (dieses Rund)

- Venera-Polar gespiegelt + Registerzeile gesetzt (s. o.); `themis_asi-fodg-cdn`-Naht repariert.
- folge357 → `archiv/`; folge358 geschrieben; Commits `cf6c8b4d9` / `3188d3a88` / `b20976b78` /
  `51bcd7fe1` gepusht (HEAD == `origin/main`).

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund). `.tools_ensure`-Sweep: das Skript verlangt einen
  Tool-Namen (kein Einzelnamen-Sweep); die Release-Route trägt weiter die Wrapper.

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
