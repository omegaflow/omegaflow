<!--
  title: Handover — Ernte-Folge 88 (Stand 2026-09-18)
  session: Ernte-Folge 88
  class: handover
  date: 2026-09-18
  sha256: 8a2ccad2a3f3348c5282a50bb59311265f1495d67986250f579c5a2bf9110fd8
  status: live
-->
# Handover — Ernte-Folge 88 (2026-09-18)

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

- **HEAD** `8dadf821` == `origin/main` (Fast-Forward, Push steht). Safety-Net
  `refs/safety/1789739087` (Session-Start, HEAD `9f75056c`).
- **Postfach** — external-state-Eintrag zitiert (`docs/zustand/external-state.md:20`,
  Entscheid-Folge 51): letzter Ledger-Eingang `1789737560` (eigene Iess-Antwort),
  Trigger (neuer Eingang) nicht gefeuert. Kein eigener Post gesetzt.
- **CI** — external-state-CI-Zeile zitiert (`external-state.md:22`, Entscheid-Folge 52,
  `ci_manage list` ~16:0xZ, HEAD `e6ec51c6`): `harvest` `35351460528` (gedi) /
  `35351464250` (atl03) / `35351458540`, `harvest-long` `35351467845` (lro)
  in_progress; `harvest` `35351465316`, `planetary-odf-cdn` `35351411938` (rosetta)
  pending. **Kein Ernte-Run-Abschluss** — die Wartestellungen bleiben wartend.
  `docs/zustand/external-state.md` + `docs/handover/post.md` sind **fremd
  modifiziert** (Entscheid-Folge 52) — nicht angefasst.

## Source-Port — offene Arme (härtester undatierter zuerst)

- **Cassini RSS Compiler-Arm** `phi/blocked_sources.φ:17-19` — CORSS_8xxx,
  Bahnverfolgung em/Range-Rate; Arm ungebaut, ein Teil request-only.
  (Schritt: Arm in `tools/harvest` + `src/archivar` bauen, dann Register.) `blockiert` (Arm).
- **ExoFOP Pipe-Arm** `phi/pipeline/ledger.φ:66-68` — Endpoint + Force-Gate
  gemessen 2026-09-18: `/tess/download_toi.php?sort=toi&output=pipe` HTTP 200,
  Wert+Frame (TIC/TOI + RA/Dec + BJD + Transit-Tiefe ppm), Kraft `em`. Arm fehlt;
  `phi/sources.φ` ist fremd-modifiziert → sources.φ-Block erst nach fremdem Commit.
  (Schritt: Pipe-Parser-Arm + `extract.rs`-Dispatch; dann `sources.φ` + `harvest.φ` + CDN.) `blockiert` (fremde sources.φ).
- **GHRC GLM L1B + TRMM LIS** `phi/blocked_sources.φ:35-37` — Compiler gebaut,
  in `sources.φ` registriert; CDN-Manifestation offen.
  (Schritt: `gh workflow run <cdn>.yml`, dann Asset sniffen + `sources.φ`-Block present.) `offen`.
- **KCDC** `phi/blocked_sources.φ:47-50` — `kcdc_kascade` in `sources.φ` fehlt
  (fremd-blockiert). (Schritt: Register-Block + CDN-Manifestation.) `blockiert` (fremde sources.φ).
- **FUGIN Bulk** `phi/blocked_sources.φ:39-41` — Pilot `fgn00000000.sky1` CDN steht;
  Bulk offen. (Schritt: `gh workflow run fugin-cdn.yml`.) `offen`.

## Wartend (kein Auswahlpunkt)

- **gedi/atl03/lro/rosetta** `phi/harvest.φ:26,44,54,92` — Trigger Run-Abschluss;
  bei success sha256/Größe + `asset present` in `phi/sources.φ`. `wartend`.
- **Lasair-LSST** `external-state.md:23` — Re-Messung nach dem at-risk-Fenster
  (2026-09-19). `termin`.
- **Voyager/Mariner/Viking/Juno-Antworten** `blocked_sources.φ:52-66` — Anfragen
  offen, Antwort wartend. `wartend`.
- **Limadou PI-Freigabe** `ledger.φ:26-28` — Follow-up per-act consent (Operator). `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:90-128` — `--port` braucht das Operator-Wort für den
  lokalen Release-Binär-Lauf. `operator-gebunden`.

## Benchmark

- **flash-first, kein Doppel-Lauf:** `grind-flash` = Endpoint-Discovery der drei
  Kandidaten; `grind-pro` = Force-Gate-Verdikt (andere Aufgabe, kein Benchmark-Paar).
  Der erste `grind-pro`-Dispatch scheiterte an DNS (`ENOTFOUND api.deepseek.com`),
  der Retry lief durch. Kein Regressions-Doppel.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-18-ernte-folge88.md` (neu),
  `docs/handover/archiv/handover-2026-09-18-ernte-folge87.md` (verschoben),
  `phi/declined_sources.φ` (NANOGrav + SRCNet `decline`-Blöcke),
  `phi/pipeline/ledger.φ` (ExoFOP-Note aktualisiert; NANOGrav/SRCNet entfernt).
- **Fremd (nicht anfassen):** `docs/handover/post.md`, `docs/zustand/external-state.md`
  (Entscheid-Folge 52), der `entscheid-folge51`-Rename, die `handover-2026-09-16-*`-Moves,
  `phi/sources.φ` (fremd modifiziert), die bau-folge86-Dateien. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
