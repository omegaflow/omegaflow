<!--
  title: Handover — Entscheid-Folge 41 (Stand 2026-09-18)
  session: Entscheid-Folge 41
  class: handover
  date: 2026-09-18
  sha256: fd2a007407dda98e39eeaa29bdf66830a075c2eb8244567e60afe420dd22dd69
  status: live
-->
# Handover — Entscheid-Folge 41 (2026-09-18)

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

- **HEAD** `da2cf8f9` == `origin/main` — die in Folge 40 genannte Push-Divergenz
  ist aufgelöst (Merge `72c96da0` + Folge-Commits; Fast-Forward möglich). Der
  frühere `blockiert`-Punkt „Push-Divergenz" ist damit am Baum erledigt und
  gestrichen.
- **Postfach** — neuester `state/mail/mail_ledger.φ`-Eingang `1789670592`
  (2026-09-17, Rubin-Forum-„News", **kein Agenten-Eingang**); unverändert seit
  Folge 40. `docs/zustand/external-state.md` trägt den Wert.
- **CI @`da2cf8f9`** — Watchdog-Snapshot 00:34 + `ci_manage list`: `ci-check`
  `35284285968` pending @`da2cf8f9`, `ci-check` `35278522861` in_progress
  @`ff212c28`; `harvest` `35278345279`, `allwise-cdn` `35281913809`,
  `health-check` `35286550387` aktiv; success `quake-feeds-cdn` `35284468943`,
  `swpc-mirror-cdn` `35283920499`, `harvest` `35278476391`; failure (fremde
  Linien) `harvest` `35278536469`, `pii-exposure` `35273689697` exit 2;
  `harvest` `35279473017` cancelled.
- **Zustand-Ledger** — `docs/zustand/external-state.md`: PII-Zeile um die
  Neumessung @`da2cf8f9` ergänzt (`pii-exposure` `35287195140` dispatched),
  CI-Zeile auf `da2cf8f9`, Postfach-Zeile auf Folge 41.

## Handlungsfähig — Auswahlpunkte

- `wartend` — **GitHub PII-Exposition**: Wert 45 @`ab45b3f3` (exit 2 = Exposition
  bleibt); Neumessung @`da2cf8f9` dispatched (`pii-exposure` `35287195140`,
  pending). (Auslöser: Run-Abschluss oder GitHub-GC-Antwort.) (Schritt:
  `ci_manage view 35287195140` einmal beim nächsten Pass; Wert in
  `docs/zustand/external-state.md` fortschreiben.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt, 20-s-Bande): §4 ist
  geschlossen (forschung `b4e70b1d`, Paper v10). Die Entwürfe
  `state/mail/adoption-{toth,turyshev,markwardt}.body.txt` liegen vor; **Senden
  ist verboten** (Operator-Wort 2026-09-17, bekräftigt). Kein Sendeschritt, keine
  Sende-Anfrage, keine Register-Zeile.
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Aus anderen Linien gefaltet

- `forschungs-gebunden` — **`survey-2026-09-17-verlorene-diskussionen.md` (b), zwei
  Bau-Zeilen geschlossen** (Meldung der Bau-Linie, aus `post.md` gefaltet):
  (1) „Voyager Decimation>1" im Baum verifiziert (`src/archivar/voyager_odr.rs:256`
  `decimation_ratio`, Tests `:517`/`:582`; `galileo_odr.rs:171` `year_full`),
  Beleg `bau-folge57:77`; (2) „Earthdata-Token-Hook live" gebaut und gefahren
  (`tools/utils/src/bin/archive_search/token.rs:24` 401/403/307-Auslöser, `:40`
  `EARTHDATA_EDL_TOKEN`-Direktpfad; live 2026-09-18: `archive_search --sniff` der
  geschützten PODAAC-GRACE-URL → 401 ohne Token, 200 mit Token, 84 701 036 B,
  sha256 `8bd14764105360e06b35c0ab35312def4869d402fca8a834249fa5b92f1cfec0`).
  (Schritt: die zwei Zeilen auf „geschlossen mit Beleg" ziehen; `--verdict` läuft
  bewusst ohne Hook — geschützte Earthdata-Assets nur via `--sniff`/Content-Fetch.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, GitHub Privacy-Löschung (Resend `01a0b032`),
  NSE/Haug I(q,t) (TRISP/MLZ Keller; Daten zugesagt), fünf Sonden-Anfragen
  (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno), Rubin-Review (Umzug
  `rubin.community` 2026-09-24). (Auslöser: Postfach-Eingang.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-entscheid-folge41.md`,
  `docs/handover/archiv/handover-2026-09-17-entscheid-folge40.md` (Move),
  `docs/handover/post.md` (Post-Zeile an `ernte`).
- Fremd uncommittet (nicht angefasst): die `ernte-folge75`-Arbeit
  (`tools/harvest/src/bin/goes16_abi_compiler.rs`, `phi/harvest.φ`,
  `phi/sources.φ`, `phi/pipeline/ledger.φ`,
  `docs/handover/handover-2026-09-17-ernte-folge75.md`), `src/archivar/atdf.rs`,
  die drei gestagten `handover-2026-09-16-*`-Renames.
- Nicht getrackt (`state/`, gitignored): `state/mail/adoption-*.body.txt` und
  `state/mail/adoption-mails.md`.

## Benchmark

- Kein Doppellauf: der Stehende Pass + der PII-Dispatch sind Routine
  (flash-Klasse), keine gemessene Benchmark-Klasse; kein pro/max-Dispatch nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
