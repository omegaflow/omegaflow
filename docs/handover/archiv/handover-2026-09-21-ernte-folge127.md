<!--
  title: Handover — Ernte-Folge 127 (Stand 2026-09-21)
  session: Ernte-Folge 127
  class: handover
  date: 2026-09-21
  sha256: 99db1d1fb6f2113eb0e5e978d42e130a6142f8399ae7f5e5e112399a45783d62
  status: live
-->
# Handover — Ernte-Folge 127 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile; Status-Tag (`wartend` | `operator-gebunden` |
`blockiert` | `termin`). Wartestellungen sind kein Auswahlpunkt. Das Handover
wird vor allem anderen gegen den Baum gehalten.

## Stehender Pass (gemessen 2026-09-21, Folge 127)

- **HEAD** `84dffc5d` beim Start (geteilter Baum, fremde Commits laufen
  gleichzeitig); eigener Commit folgt. Arbeitsbaum: eigener Pfad-Satz (unten);
  fremde Arbeit (forschung/entscheid) ist committet und gepusht.
- **CI** — `ci_manage list` 2026-09-21 ~09:0x: `ci-check` `35570367312` pending;
  `ps1-cdn` `35569486280` in_progress; `rpw-cdn` `35569266300` @`d6bf1df1`
  **failure** (422 Asset-Cap, Compile grün) — siehe P1; `tools-build`
  `35570036898` success.
- **Zustand** — `docs/zustand/external-state.md` gemessen und fortgeschrieben
  (P1-Zeilen, CI-Zeile, neue Rosetta-ODF-Zeile).

## P1 CDN-Familien-Tag-Rotation (gebaut, Verifikation offen)

- Der Rat entschied: die Familie = das Solar-Orbiter-RPW-E-Feld-Paar; der Tag
  ist die **gemessene Herkunft** (Haus-Konvention Tag = Herkunfts-Netloc), nicht
  `rpw.ssd.jpl.nasa.gov`. `rpw_efield.bin` → `amda.irap.omp.eu`,
  `rpw_efield_lira.bin` → `rpw-lira.obspm.fr`.
- 6 Sites gesetzt (`cargo check -p omegaflow-harvest` grün, 0 Warnungen):
  `tools/harvest/src/bin/rpw_compiler.rs:1,326`,
  `tools/harvest/src/bin/bia_efield_compiler.rs:2,625`,
  `.github/workflows/rpw-cdn.yml:26,36`, `phi/sources.φ:1130,1139`.
- **Offen:** nach Push `gh workflow run rpw-cdn.yml`; Asset per
  `archive_search --sniff <url>`; danach die zwei alten Kopien auf
  `ssd.jpl.nasa.gov` löschen (`gh release delete-asset`, Muster
  `ps1-cdn.yml:146–148`) — erst, wenn der neue Tag trägt.
- **Folge-Atom (gemessen nötig):** generischer Umbau von
  `upload_asset`/`CDN_RELEASE` (`src/archivar/cdn.rs:40`) — jeder Aufrufer
  benennt seinen Familien-Tag; ~74 Sites; der nächste 422 ist sonst terminiert.
  Dazu gehören `eve_compiler.rs:8` (Tag-Name `ssd.jpl.nasa.gov` falsch — EVE ist
  LASP/NOAA) und `ps1-cdn.yml:149` (PS1-Finale auf dem vollen Tag; die
  `ps1-dr2-*`-Slab-Konvention steht bereit). (Schritt: Rat/Architektur, dann
  grind-pro.)

## PS1

- **Order-10-Final** (`wartend`) — `ps1_dr2_coverage.fp01` auf `ssd.jpl.nasa.gov`
  **absent**; Lauf `35563001793` success, `35569486280` in_progress, beide
  „final combine not reached: not every band part stands yet"; Größe `Pending`
  (`phi/footprints.φ:19`). Schritt: `ci_manage list` nach `ps1-cdn`; bei
  Final-Combine die Größe aus dem Log `ci_manage log <id>` → `footprints.φ`.
- **Ernte-Rate** (gemessen) — aus `35563001793`: 14 `ps1_part_*`-Chunks /
  85,3 min ≈ 0,16 Chunk/min; bei 2007 Bändern à 10 Chunks weit unter der
  Final-Bedingung. Kein eigener Schritt (an Order-10-Final gebunden).

## DEMETER (`wartend` auf Re-Dispatch)

- Re-Dispatch `demeter-cdn.yml` `35567568429`; bei success 77 `url`+`sha256`-Zeilen
  registrieren. Der `rs-catalog`-500 ist ein eigener Register-Befund. Schritt:
  `ci_manage view 35567568429`.

## Quaoar sha256 (`wartend` auf Zenodo-Rückkehr)

- `zenodo.21185812` (572 467 032 B) — Zenodo 504, sha256 `pending`. Schritt:
  `archive_search --verdict https://zenodo.org/api/records/21185812`.

## Wartend (kein Auswahlpunkt)

- **TAP-Backends** dachs.fai.kz + pithia.cbk.waw.pl (`ledger.φ`) — Trigger sync-QUERY/tables 500→200.
- **SSDC Limadou** — operator-gebunden (PI-Freigabe).
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno/Cassini) `blocked_sources.φ`.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **Babamul / IA2 TAP** `blocked_sources.φ` — kein gebauter Konsument → `pending`.
- **Split-Routing-Verifikation** — operator-gebunden (`An entscheid`).

## Termin

- **EMODnet HFRADAR NADR** — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- **EPA RadNet ERM_RESULT Position** (`blocked_sources.φ:74`, Register-Tag
  `[bau] parser-def`) → Post `An bau`.
- **GHRC GLM-L2-LCFA-Konsument** → Post `An bau` (token-freie S3-Route trägt nur
  L2-LCFA, nicht L1B; `sources.φ:8023`).
- **LIRA/RPW-BIA E-Feld (CDN)** → eigener P1 (oben), kein Post.

## Benchmark

- **Ernte-Folge 127** — `council` (P1 Familien-Tag-Entscheid), `grind-pro` (P1
  6-Site-Rotation), `grind-flash` ×3 (P2/P6 GHRC+AQS, P3 Katalog-Digest, P4/P5
  PS1). Kein flash/pro-Vergleich derselben Aufgabe; Burn nicht gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/rpw_compiler.rs`,
  `tools/harvest/src/bin/bia_efield_compiler.rs`,
  `tools/register/src/bin/register_lookup.rs`, `.github/workflows/rpw-cdn.yml`,
  `phi/sources.φ` (P1-URLs + P2/P6-notes), `phi/footprints.φ`,
  `docs/handover/post.md` (`An ernte` gelöscht, `An bau` neu),
  `docs/zustand/external-state.md` (P1/CI/Rosetta), neues Handover
  `handover-2026-09-21-ernte-folge127.md`, Move
  `handover-2026-09-21-ernte-folge126.md` → `archiv/`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
