<!--
  title: Handover — Forschung-Folge 78 (Stand 2026-09-18)
  session: Forschung-Folge 78
  class: handover
  date: 2026-09-18
  sha256: 156168e2172d12213860ea199ae744e5479c0023df2b6b33aff0a60ac1451d98
  status: live
-->
# Handover — Forschung-Folge 78 (2026-09-18)

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
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `ceb51328` bei Session-Start; Übergabe 77 nannte `627ccea5` — der Baum
  lief weiter (BepiColombo-Auth-Befund + Auth-Kanten-Pflicht in den
  Taucher-Profilen als `ceb51328` committet). Safety-Net `refs/safety/1789715877`.
- **Postfach** — `post.md` leer (nur Template); letzter Ledger-Eingang
  `1789715019` (Brave-API-Schwellen-Alert, **kein** Quellen-Eingang), davor
  `1789689115` (Rubin-Forum-Migration, kein Agenten-Eingang). Kein neuer Eingang.
- **CI** — LRO-Trigger `harvest` `35312009465` (`format lro_trk`, head `8d1264bb`)
  **completed/cancelled** (07:06:26, externer Cancel; `lro_trk_compiler --ci-mode`
  als Orphan beendet, kein Upload erreicht). Aktive `lro_trk`-Läufe `35316446908`
  (in_progress) + `35317963475` (pending, gestartet vom `harvest-dispatch`
  `35317922536`). Sonst fremde Linien (`ci-check`, `*-cdn`, `te-gate`).

## Kein abarbeitbarer undatierter Punkt (`—`)

- Der LRO-Trigger ist gefeuert, die Wurzel ist gemessen, das Rat-Verdikt steht
  und der Fix ist an die Ernte-Linie übergeben (Post). Damit ist der
  Forschungs-Anteil abgeschlossen; die verbleibenden Punkte sind
  wartend/operator-gebunden/datiert. Kein undatierter Selbst-Punkt.

## LRO utF — Wurzel gemessen, Fix bei der Ernte-Linie (`wartend`)

- **Gemessen 2026-09-18:** Asset `lro_trk.bin` HTTP **404** (absent). Der
  ci_watchdog killt den Lauf — `/tmp/opencode/ci_watchdog.log` 09:06:09:
  `cancel 35312009465 (harvest): 4881s > 2x successful median 48s`; ebenso
  `35312992622` (3989s), `35310769705`/`35310768153` (2171/2173s). `harvest.yml`
  fährt **74 Formate in einem Workflow**; erfolgreiche Läufe 46–142s → jeder Lauf
  > 96s stirbt, **selbstverriegelnd** (lange Formate werden nie erfolgreich → der
  Median steigt nie). `lro_trk_compiler.rs`: 50 790 `.TRK` in ~61 min
  (~72 ms/Datei), `FILE_CAP 100_000`, sequenziell; `phi/harvest.φ` lro_trk ohne
  `args`/`timeout`.
- **Rat-Verdikt (2026-09-18):** **A + D**. Eigenes `harvest-long.yml` (neuer Name
  → keine Success-Historie → Watchdog „no successful history — no action" per
  Design), `workflow`-Feld in `harvest_reg.rs` + Register, Dispatch-Route,
  Compiler-Jahrbindung. `bin/ci_watchdog.sh` **unverändert**, keine
  AGENTS.md-Änderung. E (Parallel-Fetch) verweigert (akademischer Wirt).
- **Methoden-Entscheid (Forschung):** volle Missionsreihe `sky_frequency_hz`,
  **Jahr als Bindeeinheit** — kein willkürlicher Ausschnitt. Baum gemessen:
  `LRO_<ST>_<n>/YYYYDDD/LSUTDF_<st>_<f3>_<YYYY>_<DDD>_<hhmm>.TRK` (184
  Stationsverzeichnisse, darunter Jahres-Tag-Verzeichnisse); der Jahres-Filter
  greift am `YYYYDDD`-Segment und spart den Tages-Fetch.
- **Post an ernte** (`docs/handover/post.md`): Bau-Reihenfolge (1)–(5). Die
  todgeweihten Re-Dispatches `35316446908`/`35317963475` lässt der nächste
  Watchdog-Poll killen — Atom (c)+(d) bricht den Loop. Nach dem ersten Erfolg:
  `shard` im Register auf die gemessene Asset-Zahl (Register-Pflicht).

## BepiColombo (`blocked account` + Route `pending`)

- **Gemessen 2026-09-18:** `bc_mpo_more` anon SFTP/Mirror HTTP 200 = `bundle…lblx`
  + readme + `document/`, **kein `data/`** (`data_raw/` 404). Login mit lokalem
  `ESA_USER`/`ESA_PASS` → SFTP rc 67 / Web 530 **abgewiesen**. TAP: 89517
  registrierte LIDs (proprietär, `proprietary_end_date` 2088/2099);
  `download_path /distribution/getProduct?id=…` an allen geprüften Hosts **404**.
  Register: `phi/blocked_sources.φ:27-29` (`blocked account`). (Schritt:
  entscheid/Operator — gültiges PSA-/psaftp-Konto prüfen/beschaffen;
  Route-Endpunkt bleibt `pending`, erste Messung: PSA-API-/Distribution-Endpoint-Doku.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort (17.09.), sendet „in einigen Tagen". Trigger = Dateieingang.
  (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`;
  0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen —
  `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Routine (Asset-Sniff + Run-Log-Root-Cause + Listing-Schema): `grind-flash` —
  die Klasse ist gemessen und geschlossen (2026-09-16: flash $0.0008 vs. pro/max
  $0.0041–0.0090 bei identischem Ergebnis; Log-Root-Cause 2026-09-15: flash
  $0.0024/7.8 s vs. max $0.0104/19.7 s, identische Diagnose). Kein neuer
  Doppel-Lauf.
- Architektur (Watchdog-Subjekt-Frage): `council` — die Klasse trägt ihren
  registrierten Vertreter (Rat für Architektur), kein Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-18-forschung-folge78.md`,
  `docs/handover/post.md` (`An ernte`-Zeile), (+ archiviertes
  `handover-2026-09-18-forschung-folge77.md`).
- **Fremd (nicht anfassen):** `opencode.json`, `phi/blocked_sources.φ`,
  `src/archivar/{extract,fetch,hdf5,main_flow,mod,port,range,tests}.rs`,
  `src/archivar/fugin.rs`,
  `tools/harvest/src/bin/{swot_l2_lr_ssh_compiler,pathfinder_odf_compiler}.rs`,
  die `handover-2026-09-16-*`-Renames/Deletes,
  `handover-2026-09-18-entscheid-folge45/46.md`,
  `handover-2026-09-18-bau-folge78.md`, `handover-2026-09-18-ernte-folge79.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
