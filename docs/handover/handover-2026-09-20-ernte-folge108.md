<!--
  title: Handover — Ernte-Folge 108 (Stand 2026-09-20)
  session: Ernte-Folge 108
  class: handover
  date: 2026-09-20
  sha256: 81f3a0be512718533749bebaee18d51757c4bb60fba8f19c785a464867d17d89
  status: live
-->
# Handover — Ernte-Folge 108 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-20, Folge 108)

- **HEAD** `0a1c8ae7` (== `origin/main`). Arbeitsbaum: **fremd uncommittet**
  `src/archivar/bsp_reader/daf.rs` (nicht anfassen). Eigener Pfad-Satz:
  `phi/pipeline/ledger.φ` (dachs/pithia), `phi/blocked_sources.φ` (Lasair Token),
  `docs/zustand/external-state.md` (Lasair), `docs/handover/post.md` (An entscheid),
  neues Handover, Move `handover-2026-09-20-ernte-folge107.md` → `archiv/`.
  `git_safety --snapshot` → `refs/safety/1789914484`.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789906306`
  (CSES-Limadou WG, an entscheid); davor API-Key-/Hardware-/Security-Mails. Kein
  neuer Ernte-Dateneingang.
- **CI-Status** — `35516529023` harvest (icesat2, head `37369e95`) `in_progress`
  seit 14:27Z, `updated_at` 14:27:45 unverändert (~1,5 h; Median-Harvest
  `35515194248` ~3,5 min), Job-Log `404` → Harvest-Job gequeued; CDN-Asset
  `icesat2_atl03.bin` unverändert sha256 `fdba6385…` (sniff) → kein staged count.
  Keine ernte-eigene Aktion; Watchdog-Duty.

## Offen

- **icesat2_atl03** `phi/harvest.φ:79` — Run `35516529023` (args `--skip 3`)
  `in_progress` gemessen (`ci_manage view` + Log `404`), CDN-Asset unverändert
  `fdba6385…` → staged count nicht lesbar. (Schritt: `ci_manage view <run-id>`
  einmalig bei Run-Abschluss; dann note fortschreiben und `--skip 4` dispatchten.)
  `wartend` (Trigger Run-Abschluss).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — Re-Messung 2026-09-20 (research-max): beide DaCHS-Prozesse laufen, PostgreSQL
  `localhost:5432` tot (Restarts 2026-09-17/18), alle DB-Pfade 500 bzw. VOTable
  ERROR; VOSI `availability` meldet fälschlich „up"; IVOA-Registry (GAVO RofR)
  kennt keinen Spiegel. Weiterhin `blockiert` (extern). (Schritt: GET
  `…/tap/sync?…SELECT TOP 1 * FROM ivoa.ObsCore` 500→200 bzw.
  `pithia…/tap/tables` 500→200; Operator-Kontakt pithia `tomasik@cbk.waw.pl` wäre
  Dritt-Akt → Consent.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` / `phi/blocked_sources.φ:8-11` —
  Re-Messung 2026-09-20 (research-max): Backend 502 auf allen Pfaden, direct 000;
  Frontend 200, Banner unverändert (updated 2026-09-10); ZTF-Zwilling
  `lasair-ztf.lsst.ac.uk/api/objects` 401 (auth), kein Ersatz; kein Forum-Post.
  **`LASAIR_LSST_TOKEN` lokal nicht vorhanden** (env + `.secrets.local`) → Key-Gap,
  nicht Routen-Gap. (Schritt: Operator-Wort — Post an `entscheid`.) `blockiert` +
  `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — stündlicher Schedule. (Schritt: `ci_manage view <allwise-run>`.) `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53`
  — Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April. `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument → `pending`. `wartend`.

## Operator-gebunden

- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort (gehört zur entscheid-Queue).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 108** — zwei `research-max`-Taucher (TAP-Routen, Lasair-Route):
  Route-Recherche ist die pro/max-Klasse; kein Doppellauf. Die direkte Re-Messung
  lief flash-frei (curl/`ci_manage`).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (dachs/pithia), `phi/blocked_sources.φ`
  (Lasair Token), `docs/zustand/external-state.md` (Lasair), `docs/handover/post.md`
  (An entscheid), neues Handover `handover-2026-09-20-ernte-folge108.md`, Move
  `handover-2026-09-20-ernte-folge107.md` → `archiv/`.
- **Fremd (nicht anfassen):** `src/archivar/bsp_reader/daf.rs` (uncommittet, andere
  Linie). Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
