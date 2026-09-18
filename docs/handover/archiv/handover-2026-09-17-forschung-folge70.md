<!--
  title: Handover — Forschung-Folge 70 (Stand 2026-09-17)
  session: Forschung-Folge 70
  class: handover
  date: 2026-09-17
  sha256: b098fe433ca0b7a0da442517926171fde002223174beddc5676fbb91b31e7a34
  status: live
-->
# Handover — Forschung-Folge 70 (2026-09-17)

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

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — keine neue Zeile an die Forschung-Linie; `post.md` trägt nur noch
  die zwei `An alle Linien`-Broadcasts. Die `An forschung`-WWLLN-Zeile war
  überholt (folge68 archiviert, kein WWLLN-Punkt in folge69,
  `phi/declined_sources.φ:3658` gesetzt) und wurde in diesem Atom gelöscht.
  Letzter Mail-Eingang `state/mail/mail_ledger.φ` 2026-09-15/16 — keine neue Mail.
- **CI-Status @Session-Start `1cdab309`** (`ci_manage list`): S-Band-Ursache
  gemessen (s.u.). Auto-Dispatcher `dbe647d1` hat nach dem band_field-Fix neue
  `harvest`-Läufe gelegt (**35277931000/33828/37121**, queued) + X-Lauf
  **35277855134** (queued); `ned-cdn`/`ps1-cdn` in_progress.
- **Zustand-Ledger** — `docs/zustand/external-state.md` clean (die frühere
  Fremd-Modifikation ist committed).

## Kein abarbeitbarer undatierter Punkt

Alle offenen Punkte warten auf einen CI-Lauf (`wartend`), sind `operator-gebunden`
oder `termin`. Das Atom dieser Session: die S-Band-Blockade gemessen (der Lauf trug
den band_field-Fix nicht — stale code, kein echter Nullwert), den
`Second-witness`-Diagnose-Bug behoben, das Postfach aufgeräumt. Der nächste
abarbeitbare Schritt hängt am Abschluss der queued Läufe.

## Ulysses X-Band-Recovery — Manifestation + Validierung (`wartend`)

- X-Lauf **35277855134** (HEAD `7bf17ada`, enthält die X-Kette) queued.
  Trigger = Lauf-Abschluss. (Schritt: `ci_manage view 35277855134`; bei Erfolg
  sha256/Größe messen, `phi/harvest.φ` + `phi/sources.φ`-X-Block auf present;
  Log-Census `n_xband_out`/`med_rejected`/`ref_rejected` gegen das abgeleitete
  X-Ref-Fenster `[2197e4, 2200e4]` + `X_FSKY_MED_HALF_WIDTH` deuten.)

## Ulysses S-Band-Manifestation (`wartend`)

- Ursache gemessen (2026-09-17, dieses Atom): Lauf `35270658104` (HEAD `d101b20a`)
  trug den `TKFORM`-10→11-Fix (DOWNLINK_BAND) noch nicht; Job-Log
  `bands S 0 / X 0`, `0 S-band fsky samples` → `no fsky samples — the series
  stays unwritten`. Der Fix landete erst in `7bf17ada` (21:38 UTC). Der
  `shard is incomplete`-Fehler war ein Diagnose-Bug (assets=0 gegen shard=1),
  behoben in `.github/workflows/harvest.yml`. Trigger = Abschluss der
  Auto-Dispatch-Läufe `35277931000/33828/37121`. (Schritt: `ci_manage view <id>`;
  bei Asset sha256/Größe messen + Blöcke auf present; Health-Issue 36 schließen.)

## LRO utF (`wartend`)

- Trigger = Abschluss des Auto-Dispatch-Laufs (einer der queued `harvest`-Läufe).
  (Schritt: sha256/Größe messen, `phi/harvest.φ` + `phi/sources.φ`-Block auf present.)

## GOES-16 ABI (`wartend`)

- Wie LRO. Trigger = Abschluss des Auto-Dispatch-Laufs. (Schritt: sha256/Größe
  messen, Blöcke auf present.)

## §4 fsky-Census (`wartend`)

- Trigger = neuer Census-Lauf-Abschluss. (Schritt: `ci_manage view <id>` +
  `gh run download <id>` → `pioneer-cell-census.txt`; Kreuz-Rang + r²-Peak gegen
  die Zwei-Arm-Frage deuten.)

## BepiColombo (`wartend`)

- Radio-Science-Bundle `bc_mpo_more/` lebt, `data/` 404 (Cruise). Trigger =
  `data/`-Manifestation. (Schritt: bei Manifestation PDS4-TNF/ODF-Parser nach dem
  tatsächlichen Datentyp.)

## CDN-Concurrency Follow-up (`wartend`)

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht;
  `cancel-in-progress: false` bleibt. Trigger = erfolgreicher `mro_odf`-Lauf.
  (Schritt: auf die erwarteten Shard-Namen härten, messbar nach dem Lauf.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort gesendet (17.09.). Trigger = Dateieingang. (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- **S-Band-Diagnose** (`ci_manage view` + `gh run view --log` + `gh release view
  --json assets`) — Routine-Klasse, flash-first, kein Doppellauf (der gemessene
  Routine-Klassen-Sieger `grind-flash` ist registriert). Kein pro/max nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Hunks dieser Session**: `.github/workflows/harvest.yml`
  (Second-witness-Zweigordnung: 0-Asset zuerst), `docs/handover/post.md`
  (überholte WWLLN-Zeile), `phi/pipeline/ledger.φ` (Ulysses-Note), dieses Handover
  (+ archiviertes `handover-2026-09-17-forschung-folge69.md`).
- **Baseline:** HEAD `1cdab309` == `origin/main` (alles gepusht).
- Fremde uncommittete Arbeit (nicht anfassen): `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search*.rs` (+ untracked `arxiv_src.rs`), die
  Worktree-Deletionen/`archiv/`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
