<!--
  title: Handover — Bau-Folge 112 (Stand 2026-09-20)
  session: Bau-Folge 112
  class: handover
  date: 2026-09-20
  sha256: 93d71d48523ca8953eb9ea3c4d835367d602e1ea3fbbfd2b9b67841087555869
  status: live
-->
# Handover — Bau-Folge 112 (2026-09-20)

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
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `84c1cd76` == `origin/main` → gepusht, Fast-Forward. Arbeitsbaum zu
  Beginn sauber; während der Session erschien fremde uncommittete Arbeit
  (`handover-…-entscheid-folge66.md`, der `folge65`-Move,
  `docs/surveys/survey-funding-pflichtfrei.md`) — **nicht angefasst**.
- **Postfach** — aus `external-state.md` zitiert (letzter Eingang `1789930255`,
  Pine64 `info@` Ox64-Zusage, bittet um Versanddaten/Telefon — von Forschung-Folge
  123 bereits beantwortet, `sent_ledger 1789931195`; kein bau-relevanter Eingang).
- **CI** — `ci_manage list` 2026-09-20: `tools-build` `35531153732` **success**;
  `ci-check` `35531418862` @`84c1cd76` **pending** (kein Verdikt);
  `te-gate` `35531196101` in_progress. CI-Zeile in `external-state.md` auf
  HEAD `84c1cd76` fortgeschrieben.
- **Binär** — `bin/.tools_ensure omegaflow`: sha256 `02ee415e…`, frisch (Träger
  `tools-build` `35531153732`). Binär-Zeile in `external-state.md` fortgeschrieben.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| `ci-check`-Verdikt am HEAD `84c1cd76` | `wartend` | `termin` (Run-Abschluss) | Run `35531418862` einmalig aus `/tmp/opencode/ci_status.md` / `ci_manage view` lesen; bei Rot `ci_manage log <id>`. |
| 3 `ausstehend` Queue-Korpora (`ledger.φ:58–68`; 30-astro, earth-stac-sentinel, exotic-neutrino-ligo) | `blockiert` | `linie:ernte` | Blocker: `default_kernel_for("") = None` (kein `force`-Direktiv) → jeder `field_in` fällt; der lokale `--port`-Lauf ist seit 2026-09-20 sanktioniert. Ernte-Linie. |

Kein session-abarbeitbarer undatierter Bau-Punkt — der CI-Punkt ist Wartestellung;
die Disposition der Survivor ist per Register-Owner-Mapping (`verifiziert` → ernte)
an die Ernte-Linie gegangen (`post.md`).

**Messung dieses Atoms (kein Punkt):** die 7 `parser-gap`-Queue-Korpora auf dem
frischen Binär `02ee415e` (port.rs-Fix ra/dec/plx/z+dist/pmra/pmdec/radvel):

| Korpus | konv. | parsen (vor Fix) | verifiziert | declined |
|---|---|---|---|---|
| 13k_230-domains | 1048 | 1045 (8) | 148 | 897 |
| 14k_new-unchecked | 874 | 873 (8) | 25 | 848 |
| 15k_230-domains | 1048 | 969 (8) | 156 | 813 |
| 183l | 19 | 14 (1) | 4 | 10 |
| 2k | 177 | 14 (0) | 3 | 11 |
| 7k | 425 | 422 (6) | 21 | 401 |
| candidate-staging | 50 | 49 (8) | 11 | 38 |

Der ra/dec-Fix greift: parsen stieg von 39 (Summe vor Fix) auf 3386; 368 Survivor
(vorher 0). Alle 7 Ledger-Einträge `parser-gap` → `verifiziert` gesetzt.

## Benchmark

- **Bau-Folge 112**: mechanischer Re-Lauf (port+probe) über `grind-flash`
  (flash-Tier). Die Routine-Klasse (Register/mechanische Messung) ist geschlossen
  (flash $0.0008–0.0017, `docs/concepts/tools-map.md`) — kein Doppellauf, kein
  pro/max-Bedarf. Die Session (build/flash) schreibt Ledger + Übergabe.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (7 Einträge `parser-gap` →
  `verifiziert` + Notes), `docs/zustand/external-state.md` (CI-Zeile + Binär-Zeile),
  `docs/handover/post.md` (Post an ernte), neues
  `docs/handover/handover-2026-09-20-bau-folge112.md`, Move
  `handover-2026-09-20-bau-folge111.md` → `archiv/`.
- **Fremd (nicht anfassen):** `handover-…-entscheid-folge66.md`, der
  `folge65`-Move, `docs/surveys/survey-funding-pflichtfrei.md`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
