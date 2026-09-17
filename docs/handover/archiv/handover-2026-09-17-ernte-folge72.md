<!--
  title: Handover — Ernte-Folge 72 (Stand 2026-09-17)
  session: Ernte-Folge 72
  class: handover
  date: 2026-09-17
  sha256: 69711b1776fd105b985705297c3dc8e2beef50d3385b87e20518b545556b317c
  status: live
-->
# Handover — Ernte-Folge 72 (2026-09-17)

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

- **HEAD** `fc62c7dc`; `origin/main == HEAD` (Fast-Forward). Fremde Linien
  committeten während der Session (`dbe647d1` forschung, `fc62c7dc` entscheid) —
  damit sind `docs/SOURCE_PORT.md` + `phi/harvest.φ` wieder sauber.
- **CI** — Watchdog-Snapshot 22:26 überholt; live (`ci_manage list`):
  rosetta Post-Fix `harvest` `35270867738` **in_progress**,
  `harvest-dispatch` `35270851153` success; neue `harvest` `35270996845` failure,
  `35272163623` queued; `gaia-sso-cdn` `35272160298` queued.
- **Postfach** — `state/mail/` am Baum absent; `mail_ledger.φ` trägt nur alte
  Verifikations-/Reset-Mails, kein neuer Eingang. `docs/zustand/external-state.md`
  fällig durch HEAD-Wechsel — **fremd-modifiziert**, nicht angefasst.

## Pipeline-Karte — Index regeneriert (härtester undatiert, dieses Atom)

- **`phi/pipeline/index.φ` regeneriert** (Stand 2026-09-17; gitignored, lokal).
  Zählmethode: Blockzahl = Zeilen mit Präfix `url ` (verifiziert an
  grind_rechecks 3, grind_nasa 26, grind_dataverse 136, grind_vires_full 63).
  Neu aufgenommen: `sources_new_untested_*` (10 Dateien, 1.047+1.047+873+423+177
  +49+30+19+16+3 Blöcke); Katalogpfade `katalog/` → `pipeline/catalog/`;
  `stage/`/`weights_*`/Probe-Ausgänge als ausgelagert markiert.
- **`queue/master.φ` Re-Derivation gemessen (grind-pro): unnötig, byte-genau
  blockiert.** Die 7.430er-Fassung lag nie in Git (gitignored, nach Untrack
  lokal gewachsen, jetzt weg); Git-Blob `760a93b5` = 3.416 Blöcke. 12 Korpora am
  Datenträger (~4.570 Blöcke), der 13. unidentifiziert; kein Merge-Tool
  committed (one-off). Träger der Inventar-Funktion ist
  `archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ`
  (5.206 v6-Blöcke). Kein offener Handlungsschritt.
- **`docs/SOURCE_PORT.md` §2 korrigiert** (dieses Atom): master.φ-Zeile auf
  „am Datenträger absent / Träger `master_converted.φ` / Re-Derivation unnötig",
  `sources_potential_*`-Zeile auf „am Datenträger vorhanden (824+63, gitignored)".
  Die vorige Parenthese („nicht im Baum") warf beide in einen Topf. Ebenso
  `phi/harvest.φ`-Register-Zeile ergänzt (`args <cli>` dokumentiert).
- **`docs/specs/cdn_reconciliation.json`** (2026-09-03 überholt) —
  `.github/workflows/cdn-reconcile.yml` gebaut (Muster source-census.yml:
  `cdn_reconcile` → Commit der Tabelle); dispatcht `35273461901` (2026-09-17).
- **Register-Lücke** `sources_new_untested_*` u. a. (10 Korpora, ~4.700 Blöcke):
  im regenerierten Index gelistet, aber **in `ledger.φ` nicht als offener Posten
  registriert**. Schritt: Zustand (`ausstehend`) je Korpus in
  `phi/pipeline/ledger.φ` eintragen — `grind-flash`. · offen

## Harvest-Architektur

- **Fünf Familien-Blöcke** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  `operator-gebunden` (protected-Bucket-403); `dl3_skymap`/`juno_ocru_odf`
  `wartend` (Auslöser = Asset-Messung; `phi/harvest.φ` ist jetzt sauber).
- **rosetta_odf Dispatch-Beweis** `termin` — Lauf `35270867738` **in_progress**
  (live gemessen); bei success `phi/harvest.φ`-Block-note. Schritt:
  `ci_manage view 35270867738` einmal nach Abschluss.
- **rosetta ungelaufene Pfade** `wartend` (Auslöser = Lauf success).
- **`auto-dispatch`** `wartend` (zuletzt falten).

## Quellen-Routen

- **gedi/icesat2/swot protected-Bucket-403** `operator-gebunden` — CMR-Granule →
  direktes `GetObject` oder Operator-Datenabkommen (SWOT-EULA) — `research-max`.
- **`ephemeris_epm`** — fremde Linie (`bau`); Post-Zeile steht in `post.md`.

## Benchmark

- **`grind-flash`** (index.φ) — $0.0395; index.φ regeneriert + 6 Diskrepanzen
  gemessen (u. a. `review_kandidaten.txt` 197 → 790 Zeilen; `grind_arcgis_b01..b10`
  32 → 0 `url`-Blöcke durch Formatwechsel).
- **`grind-pro`** (master.φ) — $0.0455; Re-Derivation gemessen
  (unnötig/blockiert), Träger `master_converted.φ` identifiziert.
- Kein gedoppelter Lauf (zwei verschiedene Aufgaben, keine Benchmark-Klasse).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/cdn-reconcile.yml`,
  `docs/SOURCE_PORT.md` (§2-Korrektur + harvest.φ-Zeile),
  `docs/handover/post.md` (An-ernte-Zeile gefaltet),
  `docs/handover/handover-2026-09-17-ernte-folge72.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge71.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (modifiziert),
  die gestagten Handover-Renames (`handover-2026-09-16-*`). Nie ein nacktes
  `git commit`.
- `phi/pipeline/index.φ` ist gitignored — nicht committet, lokal regeneriert.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
