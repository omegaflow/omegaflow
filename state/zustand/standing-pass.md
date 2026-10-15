<!--
  title: Stehender Pass (privat) — Mycelium
  class: zustand
  date: 2026-10-15
  sha256: fbf34310a3e5d66754dd079e2466f894791bc5214490a25a0c604b85804cdb4e
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge352)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `1ccd3f852` (== `origin/main`) · geschrieben 2026-10-15 · **Mycelium-352**.

## Rote Läufe am HEAD (gemessen, nicht geraten)

Am HEAD `d42e27eb8`/`378359ab3`/`1ccd3f852` neu gestartet (Push 2026-10-15), Stand bei
Pass-Schreibung **gemessen** (`ci_manage list`, einmal — kein Polling):

| run-id | workflow | Zustand | Träger |
|---|---|---|---|
| `38704081781` | wdcb-obn-wayback-cdn | queued (am `d42e27eb8`) — der **211er**-Matrix-Rettungslauf | Mycelium |
| `38704082015` | wdcb-sep-live-cdn | **success** (am `d42e27eb8`) — der **7er**-Lauf (lebender `www.wdcb.ru/sep`-Baum) | Mycelium |
| `38704081473` | sources-repo-licence | **success** | Mycelium/CI |
| `38704081810` | register-coverage | **success** (am `d42e27eb8`) | Mycelium |
| `38704150805` | register-coverage | **success** | Mycelium |
| `38704082223` | harvest-dispatch | **success** (am `d42e27eb8`) | Mycelium |
| `38704082180` | ci-gate | **failure** (am `d42e27eb8`) — der gemessene Kern-Befund (s. unten) | River |
| `38704150827` | ci-gate | **failure** (am `378359ab3`) — derselbe Kern-Befund | River |
| `38704269028` | ci-gate | in Arbeit (am `f7852a78f`) | — (Grund **unread**) |
| `38704378380` | ci-gate | queued | — (Grund **unread**) |
| `38704992362` | ci-gate | queued | — (Grund **unread**) |
| `38693040881` | zenodo-raw-cdn | am HEAD `b8fd39b49` — Matrix-Lauf (gps_cmonoc/gps_campaign/cedc_catalog) | Mycelium |

**Bekannter Kern-Befund (gemessen aus dem Log, Lauf `38704082180` am HEAD `d42e27eb8`,
`failure`):** `ci-gate` bricht im **Kern** — `could not compile omegaflow (lib) due to 7
previous errors` + `(lib test) due to 3 previous errors`: `error[E0425]: cannot find
function 'frame_earth' in this scope` (`src/mathematikerin/machines/tests.rs:461/489/498`)
plus Clippy-Lints (`too many arguments (12/7)` `src/archivar/extract.rs:4100`; manual
`Range::contains` `src/archivar/tle.rs:90`; negated comparison `src/archivar/tle.rs:147/341`;
very complex type `src/mathematikerin/machines/matrix.rs:219/1158`, `omega.rs:2099`).
**Träger: River** (Feld/Kern). Derselbe Befund stand schon am HEAD `10b521783`
(`38702218068`). Kein Mycelium-Pfad.

## Lokal gemessen am HEAD `1ccd3f852`

Drei Register-Werkzeuge gebaut (`register_sort`, `cdn_reconcile`), `cargo check` 0/0:
`register_sort phi/sources.φ` **canonical** (2 805 Blöcke) · `cdn_reconcile --fail` →
`cap+tag contract clean (334 registry hosts)` · `register_lookup --orphans --fail` = **0** ·
`register_lookup --tokens --noecho-fail` = **exit 0** (0 XFER-MISSING).

## Postfach

- `mail_watchdog`: `health=200 recv=true tunnel=true ledger_age_s=absent` (2026-10-15,
  letzte Zeile `1792051993`).
- `post/in` (privates Haus): `2026-10-14-mycelium-nssdc-datenantrag.md` (Operator-Hand,
  Route (b) entschieden).

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` (gemessen, 956 Zeilen):
  `2026-10-14T12:56:49Z` (`ses_ec618129fffeLFawr4V3pQKpOl`, `line`/`wort`) — unverändert
  gegenüber dem vorigen Pass. Der dortige Riss (der vorvorige Pass nannte
  `2026-10-14T22:10:06Z`, der nicht mehr in der Datei steht) bleibt ungeglättet.

## Orphan-Zensus

- `register_lookup --orphans` = **0** · `--orphan-docs` = **19** Dokumente mit offenen
  Markern ohne Live-Handover-Träger · `--descoped-check` = 0 widerlegt.

## Mehrfach-Träger

- Frisch dieses Rund: `xfer:mycelium-352-1`/`-2`/`-3` (an Mountain: `field` + Host-`terms`)
  und `xfer:mycelium-352-4` (an River: der `ci-gate`-Kern-Befund) — **unbeglichen**.
- `xfer:mountain-349-1` (WDC-B-Verdikt) ist **angewandt** (die `terms`-Zeilen stehen am
  Baum) und damit beglichen; `xfer:mountain-348-3`/`-6` waren Echo. Die älteren offenen
  Tokens (`xfer:mountain-348-1`/`-2`/`-4`/`-5`, `xfer:mycelium-342-3`/`-4`, `-342-10`,
  `-343-2`) stehen weiter in `mycelium-folge352` — kein Doppel-Eintrag.

## Gearbeitet (dieses Rund)

- **WDC-B-Erbe-Rettung 32 → 211.** Die 179 neuen Wayback-Payloads (CDX `collapse=urlkey`,
  `statuscode:200`) als `raw_mirror`-Blöcke in `phi/sources.φ` + Matrix
  `wdcb-obn-wayback-cdn.yml` (32 → 211 Einträge). Gruppen: GravConst 55 · geo_min.val 44 ·
  gdata 31 · sdata +18 · geomagni.ind/aa 16 · catal_pc 8 · gps 7. Jede Datei von Wayback
  `id_` gehasht (byte-identisch gegen `bul199806.dat` verifiziert). Commits `d42e27eb8` +
  `378359ab3`.
- **Der lebende Baum `www.wdcb.ru/sep` registriert:** 7 `raw_mirror`-Blöcke
  (`cat001`…`cat007`, `cat003.gz`) + Lauf `wdcb-sep-live-cdn.yml`; die 5 Schwester-Hosts
  als `reference`-Blöcke.
- **Mountains Verdikt `xfer:mountain-349-1` angewandt** (direkter Edit des Punkts des
  Eigentümers, beide Enden genannt): Wayback `terms CC-BY-NC-4.0`; lebender `sep`-Baum
  `terms wdcb-sep-policy http://www.wdcb.ru/sep/policy.html`. `field` je Datensatz + die
  Host-`terms` = Mountain.
- **Die drei Zenodo-Assets** (CMONOC 7798617, CEDC 8084659) standen schon in Register +
  `zenodo-raw-cdn.yml` (mycelium-351); nur der CI-Lauf ist unread.
- folge352 geschrieben, folge351 → `archiv/`.
- **Folge-Pass:** die **GEOFON-CI-Naht** gebaut (`.github/workflows/geofon-waveform-cdn.yml`,
  `--route geofon`, gemessene GE-Station WLF) — der offene Punkt ist geschlossen. Die
  **WDC-B-URL-Gruppen gemessen: keine Datendateien** (nur `text/html` + Karten-Bilder), der
  Punkt ist geschlossen. Der **TIMED-Block** bleibt offen — die TLE-`origin` (celestrak)
  ist in diesem Rund nicht erreichbar (timeout), kein erfundener `origin`.

## Snapshot / Tools

- `git_safety --snapshot` → `refs/safety/1792051952`; `--close` geprüft, Push steht
  (HEAD == `origin/main`).
- `.tools_ensure`-Sweep: die `bin/`-Wrapper zogen ihre Freshness bei jedem Aufruf;
  `register_sort`/`cdn_reconcile`/`register_lookup` lokal aus dem Debug-Build gelaufen.
  Ein Werkzeug-Riss gemessen: `archive_search --wayback` (CDX) → HTTP 503; der
  `web.archive.org/cdx/search/cdx`-Endpunkt antwortet per `curl` mit 200; `sfetch` meldet
  `fetch void`.

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
