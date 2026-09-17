<!--
  title: Handover — Ernte-Folge 67 (Stand 2026-09-17)
  session: Ernte-Folge 67
  class: handover
  date: 2026-09-17
  sha256: b2c338720b472cb205fa5da44a09266f661f7b89a769d770b1122e8c47002943
  status: live
-->
# Handover — Ernte-Folge 67 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Harvest-Architektur — Council-Verdikt steht, Bau offen (härtester undatiert)

Council (2026-09-17) gegen die gemessenen Zahlen (248 Workflows, 2-GiB-422, `phi/sources.φ`-Trigger-Lücke):

- **`phi/harvest.φ`** — Master-Register, ein Block je Format, sortiert; erste Zeile `asset fehlt|present` (ein Block **ohne** `asset`-Zeile ist ungemessen, nicht dispatchbar — nie Default `fehlt`). Felder: `format`, `tag`, `arm`, `pattern` (Idempotenz-Regex, verbatim), `shard` (fehlt bei Einzel-Asset, nicht 0), `idempotent true|false`, `note` (Run-Id/Bytes/sha256).
- **`tools/utils/src/bin/harvest_reg.rs`** — Reader (std-only, Archivar-Template): `--lookup <format>`, `--arm <stem>` (Rückwärtssuche), `--check` (Gate-Test: ungemessener Block → kein Dispatch).
- **`.github/workflows/harvest-dispatch.yml`** — der EINE Dispatch-Punkt: push auf `main`, Pfade `phi/sources.φ`, `phi/harvest.φ`, `tools/harvest/src/bin/**`; Diff → Formate → Lookup → Dispatch nur bei `fehlt`; ungemessen → „register duty", kein Dispatch. Ersetzt `auto-dispatch.yml` nach Parität.
- **`.github/workflows/harvest.yml`** — der EINE generische Harvest: `workflow_dispatch` `format`,`force`; Idempotenz-Gate (`gh release view`+`pattern`) als zweiter Zeuge; `--ci-mode`; Void → gewickeltes `gh_issue_once` + `exit 1`; `concurrency: group: workflow-format`.
- **Shard-Politik:** Wand 2³¹ B, Budget 2³⁰ B (`PODF_SHARD_BUDGET`, `odf.rs`); Serien record-aligned sharden, opake Archive + Manifest (tnbfits-Muster).
- **Migration:** Atom 1 Loop auf `maven_tnf` (Parität gemessen); dann die absent-Assets; dann die 54/47 idempotenten Familien batchen; `demeter`/`ps1` (`idempotent false`) zuletzt; Aggregatoren als benannte Ausnahmen; `auto-dispatch` zuletzt falten. Kein Löschen vor Parität.
- **Risiken:** stale `present` (zweiter Zeuge + `--verify phi`); statische Job-Keys (`timeout`/`cancel-in-progress`) je Format beim Bau messen; Drift → Commit-Gate-Fixture gegen Workflow-Idempotenz.
- **Erster Atom:** `phi/harvest.φ` (ein Block maven_tnf) + `harvest_reg.rs` + beide Workflows; Nachweis ohne Last: maven `fehlt` → Dispatch → Idempotenz-Gate sieht Shards present → skip → grün; dann ein `force`-Lauf auf einem absent-Asset.
  (Schritt: diesen Atom bauen — `grind-max` mit `council`-Begleitung.)

## CDN-Register-Schuld — Stand nach diesem Atom

- **Erledigt (git trägt):** false-green Live-Abnahme — `swot-cdn` `35246189331` @ `d94ea86d` endete `failure` mit `exit code 1` + dedupliziertem Issue („issue already open"); `juno_ocru_odf` registriert + am CDN present (329770304 B, sha256 `a37e76a4…`; `phi/sources.φ`-Block + `extract.rs`/`main_flow.rs`/`tests.rs`-Dispatch).
- **Fixes angewandt, Dispatch offen:**
  - `vlass_tap_component/source` — `create_dir_all` fehlte vor dem Write (`vlass_tap_compiler.rs:292`). (Schritt: `gh workflow run vlass-tap-cdn.yml`.)
  - `hess_dl3`/`magic_dl3` — FITS-Trailing-Space im EVENTS-Header (`dl3.rs` `is_events` trimmt + `eq_ignore_ascii_case`). hess: `gh workflow run dl3-skymap-cdn.yml -f telescope=hess`. magic: URL noch offen (`opendata.magic.pic.es` / Zenodo `11108474`), dann `-f telescope=magic -f url=<fits-url>`.
  - `maxi_J0006+202` — Scheme `https→http` (`maxi-cdn.yml:32`, `sources.φ:4435`; https-TLS-Kette gebrochen). (Schritt: `gh workflow run maxi-cdn.yml`.)
  - `swot_l2_lr_ssh` — Bucket-Reihenfolge protected-first (`swot_l2_lr_ssh_compiler.rs:866`). **Aber:** der HEAD-Lauf zeigt credentialisierte (SigV4) Listings **beider** Buckets → 403 → Void. Der Reorder allein reicht nicht; die Bucket-/Konto-Autorisierung ist die offene Ursache. (Schritt: EDL-Token-Autorisierung für `podaac-swot-ops-cumulus-protected` messen — SWOT-Daten-Agreement/Access — `grind-pro`/`research-max`.)
  - `gedi_l2a` — Granule-Ordner-Layout `GEDI02_A.002/GEDI02_A_YYYYDDD` + Tag ≤ 2025-07-09 (`gedi_l2a_compiler.rs`, `gedi-cdn.yml`; Publikation stoppte 2025-07-09). (Schritt: `gh workflow run gedi-cdn.yml`; nach Erfolg registrieren.)
  - `icesat2_atl03` — protected bucket + `ATLAS/ATL03/007/YYYY/MM/DD/` + Tag ≤ 2026-05-31 (`icesat2_atl03_compiler.rs`, `icesat2-cdn.yml`; Publikation stoppte 2026-05-31). Version-Ordner „007" ist die eine Annahme (bei Void zuerst nachmessen). (Schritt: `gh workflow run icesat2-cdn.yml`; nach Erfolg registrieren.)
- **`ned.json`** — Resumability-Bug: `break` mitten im Slice **vor** `gh release upload`, partielle `raw/cone_*.json` gehen im frischen Checkout verloren → 1/40 Slices, jeder Lauf beginnt bei Cone 48 neu. (Schritt: `ned-cdn.yml` Shard-Loop resumierbar machen — `grind-pro`.)
- **`rosetta_odf`** — Lauf `35231817955` war pending; `planetary-odf-cdn` (mro_odf Runner-Shutdown). (Schritt: `ci_manage view 35231817955` beim nächsten Pass.)

## DEMETER — Lauf hängt (in_progress seit 13:41Z)

- `demeter-cdn` `35228716483` @ `671f2bd9` in_progress seit 13:41Z, updated 13:47Z (über 3 h auf `demeter-residential`). Kein Poll. (Schritt: einmal `ci_manage view 35228716483`; bei success 77 `demeter_isl`-`url`+`sha256`-Zeilen ans Ende von `phi/sources.φ`, Felder `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`, `format demeter_isl`, `at earth`, `ttl 604800`.)

## CI-Queue / Idempotenz — Nachweis

- Der ernte-Push (`d94ea86d`, 130 Workflow-Dateien) feuerte **keine** Harvest-Welle: nach 16:22Z nur `swot-cdn` + `ci-check` (gemessen `ci_manage list`) — die `auto-dispatch`-Entkopplung wirkt. (Schritt: über die nächsten regulären Läufe beobachten.)

## Stehender Pass (gemessen 2026-09-17)

- Postfach: letzter externer Eingang 2026-09-16 Sotgiu-Antwort; GitHub-Einladung `ivoa/uvor` (nicht Ernte); die fünf Sonden-Anfragen 13:47–13:55Z ohne Antwort; NSE/Haug 18:50Z. `docs/zustand/external-state.md` ist fremd-modifiziert — CI-Status/Postfach-Eintrag dort nicht angefasst; besitzende Linie faltet.
- CI-Status am HEAD `d94ea86d`: `swot-cdn` failure (gewollt), `ci-check` pending/cancelled, `demeter-cdn` in_progress.

## Benchmark

- `council` (max) für die Architektur — das Register-Urteil trug; Architektur bleibt die max-Klasse.
- `research-max` für die S3-Routen (swot/gedi/icesat2) — live-gemessen (CMR + Bucket-Layout); `grind-pro` setzte daraus drei Compiler-Fixes um; `grind-flash` löste `juno_ocru_odf` (present + registriert). Routine-Klassen bleiben flash-geschlossen.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/{gedi,icesat2,maxi}-cdn.yml`,
  `phi/sources.φ`, `src/archivar/{dl3,extract,main_flow,tests}.rs`,
  `tools/harvest/src/bin/{vlass_tap,swot_l2_lr_ssh,gedi_l2a,icesat2_atl03}_compiler.rs`,
  `docs/handover/handover-2026-09-17-ernte-folge67.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge66.md`).
  Fremd (nicht anfassen): gestagte Renames in `docs/handover/archiv/`,
  `docs/zustand/external-state.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
