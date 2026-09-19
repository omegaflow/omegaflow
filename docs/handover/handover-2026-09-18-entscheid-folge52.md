<!--
  title: Handover — Entscheid-Folge 52 (Stand 2026-09-18)
  session: Entscheid-Folge 52
  class: handover
  date: 2026-09-18
  sha256: 7a39c5a97aab0ea8d1fc3a290fc84df0f028a2c32629465baed7b8b77b55ee19
  status: live
-->
# Handover — Entscheid-Folge 52 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, Entscheid-Folge 52)

- **HEAD** `e6ec51c6`; `git_safety` Snapshot `refs/safety/1789739929`.
- **CI** — `ci_manage list` (~16:0xZ): **in_progress**: `measure-gates`
  `35353722981`, `harvest-long` `35351467845`, `harvest`
  `35351464250`/`35351460528`/`35351458540`; **pending**: `ci-check`
  `35353719021`, `harvest` `35351465316`, `planetary-odf-cdn` `35351411938`;
  **failure**: `measure-gates` `35351695849`; **success**: `weberin-verdicts-cdn`
  `35353017204`, `harvest-dispatch` `35352946735`/`35351394369`; **cancelled**:
  `ci-check` `35353693845`/`35352946727`/`35351962459`/`35351733595`/
  `35351695613`/`35351515660`/`35351394160`. Wert in `external-state` fortgeschrieben.
- **Post** — die `An entscheid`-Zeile (Betti-0-Rat-Verdikt + Fabrication/Legacy)
  gefaltet, Zeile aus `post.md` gelöscht.
- **Postfach** — kein neuer Eingang seit `1789737560`; Eintrag in `external-state`
  zitiert (nicht fällig).
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): `measure-gates.yml` (M),
  `forschung-folge83`→`archiv/` (staged), drei `handover-2026-09-16-*`-Renames
  (staged), `bau-folge85.md` (M), `post.md` (M), `silence_map_probe.rs` (M),
  `forschung-folge84` (neu).

## Gremium + Wissenschaft — vorgelegt 2026-09-18

Die drei operator-gebundenen Punkte (Betti-0, Legacy/vC, Pipeline-Port) wurden Rat
(`council`) und Wissenschaft (`research-max`) vorgelegt. Ergebnis:

- **P1 Betti-0** — Rat bestätigt die Leiter (`te.rs:2062–2071`, Anzeige-Gitter);
  Wissenschaft korrigiert einen Rat-Konfund: in 0-Dim *ist* die letzte Vereinigung
  der Tod des längsten Balkens (Werte koinzidieren), das Problem ist die
  Skalar-Reduktion eines Multisets + die **unkalibrierte 0.5-Schwelle**
  (`te.rs:3432,3454`, keine FP/FN-Rate). Erster messbarer Schritt: Null-Verteilung
  über ≥100 Realisierungen, 95 %-Quantil gegen 0.5 (CI `measure-gates`,
  `betti0_probe.rs` konsumiert die Funktion bereits). Literatur: Fasy et al.
  arXiv:1303.7117; Chazal et al. arXiv:1406.1901; Bubenik arXiv:1207.6437.
- **P2 Legacy/vC** — Rangfolge bestätigt (Silence Map → Certainty → TDA/Betti-0,
  Minkowski 4.). „Fabrication-Rest" ist kein Baum-Begriff; Träger ist diese
  Übergabe (die frühere Z. 67), Inhalt = „Der Scherenschnitt der Abwesenheit"
  (`kybernetische-astrophysik.md:471–479`) = Silence-Map-Probe. vC-Inversion jetzt
  belegt: Legacy `exp(-vC/(g+1/C))` (`archive-root/…/minkowski-field-permeability.md:162`)
  vs. heute `tanh(v_c/(g+PERM_GROUND))` (`omega.rs:1606`) — Designfrage. Erster
  Schritt: v_c-Verteilung (AR(1)-Null, Two-Cluster, Phasen-Surrogate) in CI.
- **P3 Pipeline-Port** — Rat: **Weg B (CI)** mit Korpus-Upload; Weg A nur als
  gemessener leichter Lauf + `OMEGAFLOW_HIDDEN=1` + Operator-Wort. Erster Schritt
  ohne Wort: `force_type`-Verteilung + Fixture. Konfunde: Korpus-Ort/Größe unbenannt;
  `force_gate`-Modulname am Baum absent.

Die Folgeschritte reisen als Post an bau/forschung; die Verdikte selbst liegen dem
Operator vor.

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem Operator vor.

- `operator-gebunden` — **Betti-0-Schwellenleiter**: Rat + Wissenschaft bestätigen
  das Verdikt (Leiter behalten; `persistence` leiter-unabhängig); Präzisierung:
  Skalar-Reduktion + unkalibrierte 0.5-Schwelle (`te.rs:3432,3454`). (Schritt:
  Operator-Wort zum Verdikt; der erste Messschritt — Null-Verteilungs-Quantil in CI
  — läuft ohne Wort an bau.)
- `operator-gebunden` — **vC-Semantik** (Legacy-Certainty vs. heutige Permeabilität):
  `exp(-vC/(g+1/C))` gegen `tanh(v_c/(g+PERM_GROUND))` (`omega.rs:1606`) — invertiert,
  Designfrage. (Schritt: Operator-Wort, welche Richtung gilt.)
- `operator-gebunden` — **Pipeline-Port force-Gate**: Rat empfiehlt **Weg B (CI)**
  mit Korpus-Upload; Weg A nur leichter Lauf + `OMEGAFLOW_HIDDEN=1` + Wort. Erster
  Schritt ohne Wort: `force_type`-Verteilung + Fixture (an bau). (Schritt:
  Operator-Wort A/B nach der Korpus-Inventur.)
- `operator-gebunden` — **ESP32-Modul**, physischer Träger für Puls/HRV; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)
- `termin` — **Lasair-LSST**: API 502, Wiedervorlage nach dem at-risk-Fenster.
  (Schritt: `archive_search --verdict https://api.lasair.lsst.ac.uk/api/query` —
  Trigger 2026-09-19 in `external-state`.)
- `termin` — **BepiColombo MORE** (`bc_mpo_more`): PI Luciano Iess
  (`1789729151`) — Cruise-Daten erst zur Wissenschaftsphase (~April), kein
  Zwischenzugang; eigene Antwort `1789737560`. (Schritt: Wiedervorlage ~April;
  `archive_search --verdict` am MORE-URN.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten.
- `wartend` — **SuperDARN**: Zugangsanfrage 2026-09-18 gesendet (Ledger
  `1789718159`); Globus-Einladung ausstehend. (Auslöser: Operator-Postfach.)
- `wartend` — **GitHub PII-Exposition**: 45 @`3b7aa5a1` (exit 2 = Exposition
  bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801`, Privacy-Löschung (Ref `01a0b032`), NSE/Haug
  (Keller 17.09.: „in einigen Tagen"), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou (neue Prozedur nach
  CSES-02-Umstellung). (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: der Dispositions-Digest lebt im
  Quellcode, aber das PATH-Binary ist der alte Build. (Auslöser: Release-Build;
  Schritt: `ci_manage list` auf `release-build`.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-19 — Lasair-LSST Re-Messung (nach dem at-risk-Fenster).
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).
- ~2027-04 — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-18-entscheid-folge52.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge51.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile; Header-sha)
- `docs/handover/post.md` (`An entscheid`-Zeile gelöscht; Header-sha)
- Fremd uncommittet (nicht angefasst): `measure-gates.yml`, `forschung-folge83`→`archiv/`
  (staged), drei `handover-2026-09-16-*`-Renames (staged), `bau-folge85.md`,
  `silence_map_probe.rs`, `forschung-folge84`.

## Benchmark

- Stehender Pass: kein Dispatch (lokal, Routineklasse geschlossen — `grind-flash`
  $0.0008, 2026-09-16).
- Gremium + Wissenschaft: `council` (pro/max) und `research-max` (pro/max) für die
  drei Entscheid-Punkte dispatcht — hartes Urteils-/Recherche-Atom, kein
  flash-Äquivalent (die Gremien sind die benannten Körper). Erster `research-max`-
  Lauf: Netzabbruch; Retry lieferte. Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
