<!--
  title: Handover — Ernte-Folge 75 (Stand 2026-09-17)
  session: Ernte-Folge 75
  class: handover
  date: 2026-09-17
  sha256: c4ea27fe5698ac58cf807f47080e61de1e23acf95d71b46032423d4d17cfd583
  status: live
-->
# Handover — Ernte-Folge 75 (2026-09-17)

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

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `1cdab309` beim Start == `origin/main` (`ernte: register the dispatched
  ulysses/lro/goes harvest run ids in handover 74`). Fremd im Baum (nicht
  angefasst): `docs/concepts/tools-map.md` (M),
  `tools/utils/src/bin/archive_search.rs` + `archive_search/pdf.rs` (M),
  `tools/utils/src/bin/archive_search/arxiv_src.rs` (??), die drei
  `handover-2026-09-16-*`-Renames (D + `archiv/`??).
- **CI** — Watchdog-Snapshot 23:30 + `ci_manage view`: `harvest-dispatch`
  `35277699933` success, `auto-dispatch` `35277836478` success; die drei
  Ulysses/LRO/GOES-`harvest`-Läufe: `35277931000` (ulysses_atdf) **failure**
  (issue #38), `35277937121` (goes16_abi) **failure** (issue #37), `35277933828`
  (lro_trk) in_progress; `rosetta_odf` `35270867738` **cancelled**.
  `ned-cdn` `35277092340` + `ps1-cdn` `35276288978` in_progress.
- **Postfach** — keine ernte-Zeile in `post.md`; kein neuer Ledger-Eingang
  (`state/mail/mail_ledger.φ` letzter Eintrag `1789670592`, 2026-09-17; nur
  Cloudflare-Codes + Rubin-Forum-Migration, **kein Agenten-Eingang**).

## Pipeline-Port (härtester undatiert, operator-gebunden)

- **force-Gate am kleinsten Korpus** — `phi/pipeline/queue/sources_new_untested_183l.φ`
  (19 Blöcke, gitignored): fünf `force`-Direktiven gesetzt (seismic-body/thermal/
  thermal/em/gravity an Erdbeben/Drifter/METAR/Sonnenregionen/NEO; `grind-pro`).
  Die übrigen 13 Blöcke tragen keinen physikalischen Kanal (Registry/Count/Text →
  Review, nicht Port) oder brauchen kein `force` (`last_line`-Arm). Zwei
  Port-Grenzen gemessen: Unit hartkodiert `"1"` (`port.rs:24`, nicht in der
  Unit-Registry) und `force` ist block-level (METAR unterrepräsentiert 3 von 4
  Feldern). (Schritt: der Lauf ist der offene Punkt → unten.)
- **Der Lauf hat keinen sanktionierten Ort** `operator-gebunden` — `grind-pro`
  gemessen: kein CI-Workflow fährt `--port` (`sgrep` über `.github/workflows`:
  0 Treffer), der einzige `--probe`-in-CI (`health-check.yml:103`) iteriert
  `phi/pipeline/stage/*_converted.φ`, das gitignored ist (`.gitignore:66`), und
  lokale Funktionsläufe sind verweigert (`opencode.json:129/131`). Korrektur des
  Vor-Handovers: `release-build.yml:42-84` **baut/verteilt** das Binär
  (workflow_dispatch → Release) — der Stale-Binär-Blocker ist über CI lösbar,
  nur der Lauf fehlt. (Schritt: Operator-Wort für den lokalen Lauf des
  Release-Binärs auf den gitignorierten Korpora, ODER ein CI-Workflow, der den
  Korpus trägt und `--port`+`--probe` fährt — Operator-Entscheid, nicht
  Session-Wahl.)
- **Ledger-Noten der 10 Korpora** — auf den gemessenen Stand gezogen
  (`phi/pipeline/ledger.φ`: `--port blockiert` + force-Gap benannt; 183l mit den
  fünf gesetzten Direktiven). Erledigt.

## Harvest-Architektur

- **ulysses_atdf S-Band leer** `blockiert` — `harvest` `35277931000` kompilierte,
  manifestierte aber nur `ulysses_atdf_x.bin`; die S-Band-Familie ist leer (0
  S-Band-Samples über alle Dateien; z. B. `2037038A.TDF` trägt bands S 58583 / X
  58583 mit 115608 band-boundary-Rejects an `atdf.rs:783`, jedes Folgepaar kreuzt
  die Bandgrenze → die Paarung `atdf.rs:755-786` verwirft alle S-Records), Pattern
  `^ulysses_atdf(_t…)?\.bin$` matcht 0 Assets (issue #38). Die `forschung`-Linie
  committete `ff212c28` (forschung-folge70) mit dem Befund, der band_field-Fix
  `7bf17ada` löse das; die Messung an `a4f7fca1` widerlegt das (Post-Zeile an
  forschung). (Schritt: in
  `reduce_uly_skyfreq` jedes Record mit dem nächsten gleichen-Band-Record paaren
  (Bandzustand mitführen) ODER das `DOWNLINK_BAND`-Label gegen das rohe ATDF-Feld
  verifizieren — braucht Rohdaten-Messung; `grind-max`; danach Re-Dispatch.)
- **goes16_abi Fix** `blockiert` — `harvest` `35277937121` returned void (issue
  #37): 112 Keys in der neuesten Stunde, 0 M6-Granules gewählt, weil
  `channel_index` `_M6C` suchte, die S3-Keys aber `-M6C` tragen
  (`goes16_abi_compiler.rs:27`). Fix 2026-09-17 gesetzt (`find("-M6C")`, Test
  `channel_index_reads_band_from_m6_name` deckt es; `cargo check -p
  omegaflow-harvest` 0/0). (Schritt: nach dem Push `gh workflow run harvest.yml
  -f format=goes16_abi`, dann `ci_manage view` einmal.)
- **rosetta_odf Re-Dispatch** `wartend` — `35270867738` cancelled (2026-09-17T21:30Z;
  Laufzeit ~63 min). Re-dispatched mit Timeout-Eingabe: `35279473017` queued.
  Befund: `harvest.yml` liest nur `github.event.inputs.timeout` (Default 240),
  **nicht** das `timeout`-Feld des `phi/harvest.φ`-Blocks (`rosetta_odf` trägt
  `timeout 350`). (Schritt: `ci_manage view 35279473017` einmal; bei success
  sha256/Größe messen, `asset present` in `phi/harvest.φ` nachtragen.)
- **ulysses_atdf_x** — asset present gemessen 2026-09-17: `ulysses_atdf_x.bin`
  5546584 B sha256 `a57af1c1…` (von `harvest` `35277931000` manifestiert, derselbe
  Compiler schreibt die X-Familie); registriert in `phi/harvest.φ` +
  `phi/sources.φ:6984`. Die X-Ref-Fenster-/Halbwertsbreiten-Validierung (derived)
  bleibt `pending`. (Schritt: beim nächsten `atdf`-Atom die abgeleitete
  Validierung nachziehen.) · wartend
- **Fünf Familien-Blöcke** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  `operator-gebunden` (protected-Bucket-403); `dl3_skymap`/`juno_ocru_odf`
  `wartend` (Auslöser = Asset-Messung). · nicht auswählbar
- **rosetta ungelaufene Pfade** `wartend` (Auslöser = Lauf success);
  **`auto-dispatch`** `wartend`. · nicht auswählbar

## Quellen-Routen

- **gedi/icesat2/swot protected-Bucket-403** `operator-gebunden` — CMR-Granule →
  direktes `GetObject` oder Operator-Datenabkommen (SWOT-EULA) — `research-max`.
- **`ephemeris_epm`** — fremde Linie (`bau`).

## Parser-Gap / Register

- **EPA AQS** — geschlossen: der `disponiert`-Eintrag in `phi/pipeline/ledger.φ`
  ist gestrichen (2026-09-17). Erledigt.

## Benchmark

- **CDN-Failure-Diagnose flash gegen pro** (Klasse ohne registrierten Sieger,
  `grind-flash` + `grind-pro` parallel auf denselben zwei roten Läufen): **beide
  identische Grundursachen** — goes16_abi `_M6C`→`-M6C`
  (`goes16_abi_compiler.rs:27`), ulysses_atdf S-Band leer (`atdf.rs:783`
  Bandgrenze). **Sieger `grind-flash`** (Tier günstiger; `session_burn`-Aggregat
  grind-flash $0.0322/2 Sessions vs grind-pro $0.1509/2 — nicht session-isoliert,
  Aggregat über 36 Sessions).
- Der `--port`-Fix aus folge74 ist per `cargo check` (0/0) verifiziert; die
  Korpora-Portierung selbst ist weiterhin nicht am Baum (Lauf blockiert).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/harvest/src/bin/goes16_abi_compiler.rs`,
  `phi/harvest.φ`, `phi/sources.φ`, `phi/pipeline/ledger.φ`,
  `docs/handover/handover-2026-09-17-ernte-folge75.md` (+ archiviertes
  `handover-2026-09-17-ernte-folge74.md`).
- **Fremd (nicht anfassen):** `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search.rs`,
  `tools/utils/src/bin/archive_search/pdf.rs`,
  `tools/utils/src/bin/archive_search/arxiv_src.rs`, die drei gestagten
  `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
