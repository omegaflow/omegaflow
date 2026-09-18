<!--
  title: Handover — Forschung-Folge 79 (Stand 2026-09-18)
  session: Forschung-Folge 79
  class: handover
  date: 2026-09-18
  sha256: 4c863303720b0d54f77f68617e48deeef964b19a2a0cb7c2d60e01658d14626f
  status: live
-->
# Handover — Forschung-Folge 79 (2026-09-18)

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

- **HEAD** `0aaa34e1` bei Session-Ende; Session-Start `3471b051` — der Baum lief
  weiter (ernte-Folge 80 `91741691`, entscheid-Folge 48/49). Safety-Net
  `refs/safety/1789717602`.
- **Postfach** — letzter Ledger-Eingang `1789715019` (Brave-API-Schwellen-Alert,
  **kein** Agenten-Eingang); `post.md` leer — die `An ernte`-Zeile (LRO-Fix) hat
  die Ernte-Linie abgeholt. Kein neuer Eingang.
- **CI** — `te-gate` `35313041295` in_progress @`b8bb7b01` + `35314110831`
  pending @`7b67b10d` (Verdikt ausstehend); `harvest`-Serie 08:03 failure
  (Ernte); `ci-check` in_progress; LRO-Re-Dispatch `35316446908` in_progress /
  `35317963475` pending (Ernte). Shared state (`docs/zustand/external-state.md`)
  ist von der Entscheid-/Ernte-Linie fortgeschrieben — nicht angefasst.

## Kein abarbeitbarer undatierter Punkt (`—`)

- Der BepiColombo-Route-Punkt ist abgearbeitet: Route gemessen, Register-`note`
  aktualisiert (`phi/blocked_sources.φ`). Verbleibend sind wartend/operator-gebunden/
  datiert. Kein undatierter Selbst-Punkt.

## BepiColombo (`operator-gebunden`, Route gemessen 2026-09-18)

- **Route gemessen:** PSA migriert → `https://psa.esa.int/` (v7.10.1); TAP+
  11.1.0 unter `https://psa.esa.int/psa-tap/tap/` (sync/async/data/tables);
  anonymes ADQL 200 (39 000 444 `psa.epn_core`-Zeilen). Altes `psa.esac.esa.int`
  tot, `archives.esac.esa.int/psa/epn-tap/tap` 404. Datei-Route:
  `https://psa.esa.int/psa-tap/data?retrieval_access=DIRECT&retrieval_type=PRODUCT&tapclient=EPN-TAP&id=<urn>`.
- **Gegatet:** 89517 `bc_mpo_more`-Zeilen, `access_url` NULL; MORE-URN →
  HTTP **403** (`DataRetrieval` forbidden), Kontrolle `bc_mpo_mag` → 200/zip
  anonym; FTP-`data_raw/calibrated/derived/browse` alle 404. Register:
  `phi/blocked_sources.φ:26-29` (`blocked account`, note aktualisiert).
- (Schritt: entscheid/Operator — gültiges PSA-Konto; danach ein authentifizierter
  TAP-`data`-Abruf am MORE-URN. Erste Messung: Konto.)

## TE-Gate n=1000 Conditional-Null (`wartend`, fremder Ledger)

- Verdikt ausstehend; die Läufe und die rote Zelle stehen im TE-Gate-Eintrag von
  `docs/zustand/external-state.md` (Entscheid-Folge 49). Trigger: Run-Abschluss.
  Nach grünem Verdikt wird der `211A→193A`-Conditional-Check frei
  (`docs/paper/solar-seconds-matrix.md:37,46`) — dann die konditionale Sonde auf
  dem Solar-Korpus.

## NSE/Haug (`wartend`)

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

- BepiColombo-TAP-Route: `grind-flash` (Routine-Recherche) — vollständige,
  entscheidende Messung in einem Lauf (TAP-Zeilenzahl, `access_url` NULL,
  403 für MORE-URN gegen Kontrolle 200). Kein pro/max-Doppel-Lauf: die
  Routine-Recherche-Klasse ist gemessen und geschlossen (2026-09-16: flash
  $0.0008 vs. pro/max $0.0041–0.0090 bei identischem Ergebnis).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/blocked_sources.φ` (BepiColombo-note),
  `docs/handover/handover-2026-09-18-forschung-folge79.md`,
  (+ archiviertes `handover-2026-09-18-forschung-folge78.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, `opencode.json`,
  `src/archivar/{hdf5,extract,fetch,mod,port,range,tests,main_flow}.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`, die `handover-2026-09-16-*`-Renames,
  `handover-2026-09-18-{bau,ernte,entscheid}-folge*.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
