<!--
  title: Standpass — Mycelium (privat)
  class: zustand
  date: 2026-10-15
  sha256: 56fa3b331ea10b70687d51e722060528f5bcde4308694d91912355bdd142f6f2
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge360)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `ec2f6efde` (== `origin/main`) · geschrieben 2026-10-15 19:58Z · **Mycelium-360**.

## Rote Läufe (gemessen via `ci_manage`, kein Polling)

`ci_manage reds --pages 12` (einmal): **653** non-success Läufe über 12 Seiten — ein
**Flutstand** aus den parallel laufenden Linien. Verteilung (gemessen, Summe 653) + ein
gemessener Grund je Träger (`ci_manage jobs` / `ci_manage log <id>`):

| workflow (n) | gemessener Grund |
|---|---|
| `ci-gate` (345) | Job `clippy`: `RUSTFLAGS=-D warnings` → `src/archivar/gfc.rs` (`unnecessary_cast`, `needless_range_loop`, `excessive_precision`) + `src/mathematikerin/channel.rs` (`collapsible_if`), alles fremd. Job `format`: rustfmt-Drift in `tools/harvest/src/bin/venus_radar_compiler.rs`, `tools/service/src/bin/llm_gateway.rs`, `tools/utils/src/bin/archive_search/net.rs`. Job `subset`: `cargo test --lib` = **2839 passed · 49 failed · 51 ignored** (fremde Tests; `build`/`dropped-gate` success). Job `register`: **unread**. |
| `register-coverage` (184) | Job `orphans`. Lokal gemessen: `register_lookup --orphans --fail` = **0**; `--tokens --noecho-fail` = **0 NOECHO** (18 unbeglichen, 5 XFER-MISSING, nicht failend). Tieferer Grund der roten Läufe **unread**. |
| `tools-build` (41) | `build` — **unread**. |
| `quake-feeds-cdn` (11) · `health-check` (9) · `matrix-rotor` (8) · `de2-cdn` (8) · `gic-al-network` (5) | **unread** (fremde Träger). |
| `sources-repo-licence` (4) · `m10-mdis-raw-cdn` (4) · `entsoe-cdn` (3) · `dsn-cdn` (3) · `ci-check` (3) | **unread**. |
| je 2: `ssd-jpl-raw-cdn` · `modvolc-cdn` · `kasi-cdn` · `intermagnet-xyz-cdn` · `hdf5-real-granule` · `clpds-cdn` | **unread**. |
| je 1: `zenodo-raw-cdn` · `vnp46a3-cdn` · `venus-radar-cdn` · `themis_asi-fodg-cdn` · `swpc-mirror-cdn` · `swpc-efield-cdn` · `sources-refresh` · `ps1-cdn` · `nohrsc_snowfall-cdn` · `nasa-power-t2m-cdn` · `knmi-cdn` · `hips-png-cdn` · `cdaweb-tec-cdn` | **unread**. |

Push-Läufe am neuen HEAD `ec2f6efde` sind **queued/unread**. Der eigene Beitrag dieses Atoms
(neues `md5.rs`, geänderte `sha256.rs`/`raw_mirror.rs`) ist clippy-sicher gebaut (der
`unnecessary_cast`/`needless_range_loop`-Kern ist fremd).

## Lokal gemessen am HEAD `ec2f6efde`

- **Der streamende `.parts`-Splitter + das Quell-md5-Tor stehen** (Mycelium-360):
  `src/archivar/md5.rs` neu (inkrementeller std-only MD5, RFC 1321, Vektoren + Streaming
  gemessen); `src/archivar/sha256.rs` trägt jetzt einen inkrementellen `Sha256`;
  `raw_mirror` liest den Temp-Body stromweise in `CDN_PART_MAX_BYTES`-Teile und nimmt
  `--md5` als Gate; `raw_mirror_blocks.sh` liest die `md5`-Registerzeile.
- **Gemessene Grenze:** `raw_mirror` holt den Body zuerst in den Temp und splittet dann →
  Temp + Teile ≈ 162 GB; der Standard-Runner (14 GB SSD) trägt den 80,9-GB-Lauf nicht. Der
  direkte curl→parts-Strom mit Upload-as-you-go ist der nächste begrenzte Bau (Retry auf
  stdout ungemessen). **Kein** `phi`-Block, **kein** Matrix-Eintrag gesetzt (sonst
  `cdn_reconcile --fail`-Riss bzw. ein garantiert-roter 80,9-GB-Lauf).
- **Riss (fremd, gemessen):** `cargo check --tests -p omegaflow-harvest` bricht an
  `glofas_ewds_compiler.rs:686` und `nist_ncnr_nse_compiler.rs:212` (`unwrap_err()` auf
  Nicht-`Debug`-Typen) — der Grund der `ci-gate`-`subset`-Rots.
- Register-Nullstände (unverändert): `register_lookup --orphans --fail` = 0; `--fired
  mycelium` = 0; `--stale mycelium --persist 3` = 0; `--addressed mycelium` = 2 (schon in
  folge359 gefaltet).
- folge359 → `archiv/`; folge360 geschrieben; Commits `290754559` + `ec2f6efde` gepusht.

## Postfach

- `post/in` (privates Haus, `~/projects/omegaflow-work/post/in`):
  `2026-10-14-mycelium-nssdc-datenantrag.md` — **bereits gewortet, nicht erneut vorgelegt**.
  `post/paid-open` (27 B) — fremde Linie; nur die Größe gelesen, kein Inhalt.

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` ist **2026-10-14T12:56:49Z**
  (`OMEGAFLOW_SESSION` unset → die neue Welle schreibt keine Ereignis-Zeile; der
  `commit_check` überspringt den Namen).

## Orphan-Zensus

- `register_lookup --orphans` = 0. `--orphan-docs` = **30** (Träger fehlt; gemessen, größte:
  `survey-2026-10-10-worldwide-lidar-quellen.md` 52, `flyby-path-2-addendum` 27).

## Mehrfach-Träger

- Offen aus dem Tauchauftrag: `xfer:mycelium-359-1 … -8` (acht Fäden → Mountain, unbeglichen).
  Dazu `xfer:mycelium-352-5` · `xfer:mycelium-352-1`/`-2` · `xfer:mountain-348-1` (Mountain) ·
  `xfer:mycelium-356-3` (River) · `xfer:river197-6` (echoed).
- `river219-de2spdf` / `-sabergats` / `-juicespice` → eigene Hol-/Ablage (kein Token);
  `river219-gic2` → River selbst.
- Gefaltet/geschlossen: mountain-folge356, river-folge221 (früher gefaltet); mountain-folge355,
  river-folge219.

## Gearbeitet (dieses Rund)

- Der streamende `.parts`-Splitter + das Quell-md5-Tor gebaut (Details oben).
- Die acht Fäden aus folge359 unverändert; nur der Venus-Transport-Grundriss ist neu.

## Snapshot / Tools

- `git_safety --snapshot` gesetzt (dieses Rund, `refs/safety/1792094282`). `.tools_ensure`-Sweep:
  das Skript verlangt einen Tool-Namen (kein Einzelnamen-Sweep); die Release-Route trägt weiter
  die Wrapper.

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
