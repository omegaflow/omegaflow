<!--
  title: Handover — Entscheid-Folge 54 (Stand 2026-09-19)
  session: Entscheid-Folge 54
  class: handover
  date: 2026-09-19
  sha256: a690304edf6d9385b10725042605d1b338034ac0af379a9cdc23f0fc3dc9f294
  status: live
-->
# Handover — Entscheid-Folge 54 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 54)

- **HEAD** `49801083` (== `origin/main`, „measure: hyperscanning family-wise TE
  screen"); `git_safety` Snapshot `refs/safety/1789836774`. Der Baum ist während
  der Session von `1eef6949` (Folge 53) über `778c68db` auf `49801083` gezogen —
  fremde Linien committen und pushen; Fast-Forward.
- **CI** — Watchdog-Snapshot (`/tmp/opencode/ci_status.md`, 17:56Z) +
  `ci_manage list`/`view` (2026-09-19 ~20:59Z): **pending** `ci-check`
  `35456571554` @`49801083`; **in_progress** `measure-gates` `35456056999`
  @`f2a30682`, `allwise-cdn` `35456069108`, `cassini-rsr-cdn` `35455807084`,
  `cassini-odf-cdn` `35455805362`; **pending** `cassini-rsr-cdn` `35455838104`,
  `cassini-odf-cdn` `35455835693`; **failure** `measure-gates` `35454958335`
  @`5cba9f70` (`silence_map_probe` exit 101); **failure** `ci-check` `35451506666`
  @`5cba9f70` (5 clippy-Lints + 2 test-fails + fmt — an bau, `post.md`);
  **success** `hyperscanning-te` `35456288477`, `auto-dispatch` `35455800633`,
  `harvest-dispatch` `35455800624`. Wert in `external-state` fortgeschrieben.
- **Postfach** — letzter Ledger-Eingang `1789795811` (Rubin-Forum AGN-Lightcurves,
  informativ); kein neuer Eingang (`mail_digest`/`state/mail/mail_ledger.φ`;
  Empfänger-Dienst `smail_recv` läuft auf 127.0.0.1:1619). Eintrag in
  `external-state` fortgeschrieben.
- **Post** — kein `An entscheid` in `post.md`; vier `An bau`-Zeilen (vC-Perm,
  Betti-0, Pipeline-Port, ci-check-Lints) gehören der Bau-Linie, nicht angefasst.
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): `src/archivar/hdf5.rs`,
  `src/mathematikerin/te.rs`, `tools/harvest/src/bin/icesat2_atl03_compiler.rs`;
  drei `handover-2026-09-16-*`-Renames (staged: `entscheid-folge24`,
  `forschung-folge44`/`-folge51` → `archiv/`).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
termin/wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem
Operator vor.

### Operator-Entscheidungen

- **Pipeline-Port force-Gate** — A/B offen (lokaler `--port`-Lauf vs. CI-Upload
  der 10 gitignorierten `phi/pipeline/queue/*.φ`-Korpora + Netz-Proben). (Schritt:
  Operator-Wort A/B.)
- **vC-Permeabilität** — Operator-Wort 2026-09-19: Messakt lokal, **kein CDN**;
  Herleitung (Gremium + Forschung) ergab **keine Inversion** (`tanh(v_c/(g+ε))`
  `omega.rs:15-17` ist die Permeabilität; Legacy `exp(-vC/(g+1/C))` war eine
  Certainty; Argument dimensionslos `omega.rs:1620-1621`). Offen: (a)
  Surrogat-Nullkontrolle des Fallbacks, (b) Saturations-Skala v_c/g. Delegiert an
  bau (`post.md`). (Schritt: bau fährt den lokalen Messakt.)
- **Betti-0-Schwelle** — Operator-Wort „muss gemessen werden"; Null-Verteilungs-
  Quantil ≥100 Realisierungen gegen die unkalibrierte 0.5-Schwelle
  (`te.rs:3432,3454`), in `measure-gates` (`betti0_probe.rs`). Delegiert an bau
  (`post.md`). (Schritt: bau baut Quantil-Arm + FP/FN-Rate, dispatcht
  `measure-gates`.)
- **ESP32-Modul** — on hold (Operator-Wort 2026-09-19); BOM
  `docs/specs/mantis-shrimp-bom.md`.
- `termin` — **Lasair-LSST**: API 502 (direct + Proton), Wayback 200 ohne
  Snapshot; Re-Messung 2026-09-19 bestätigt. (Schritt: `archive_search --verdict
  https://api.lasair.lsst.ac.uk/api/query` — Trigger Banner-Wechsel.)
- `termin` — **BepiColombo MORE** (`bc_mpo_more`): Cruise-Daten erst zur
  Wissenschaftsphase (~April 2027); PI Iess-Antwort `1789729151`. (Schritt:
  Wiedervorlage ~April; `archive_search --verdict` am MORE-URN.)
- `blockiert` — **adoption-Block** (Toth/Turyshev/Markwardt): §4 geschlossen,
  Entwürfe in `state/mail/`, Senden per Operator-Wort 2026-09-17 verboten.
- `wartend` — **SuperDARN**: Globus-Einladung ausstehend. (Auslöser:
  Operator-Postfach.)
- `wartend` — **GitHub PII-Exposition**: 45 Kombinationen @`3b7aa5a1` (exit 2 =
  Exposition bleibt). (Auslöser: GitHub-GC-Antwort.)

## Wartend (extern) — kein Auswahlpunkt

- `wartend` — GitHub GC `#4761801` (Privacy-Löschung, Ref `01a0b032`), NSE/Haug
  (Keller 17.09.: „in einigen Tagen"), fünf Sonden-Anfragen, Rubin-Review (Umzug
  `rubin.community` 2026-09-24), CSES-Limadou (neue Prozedur nach CSES-02-
  Umstellung). (Auslöser: Postfach-Eingang.)
- `wartend` — **register_lookup-Release-Build**: PATH-Binary ist der alte Build.
  (Auslöser: `release-build`-Run; Schritt: `ci_manage list`.)
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).
- ~2027-04 — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-19-entscheid-folge54.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-entscheid-folge53.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile + Postfach; Header-sha)
- Fremd uncommittet (nicht angefasst): siehe Stehender Pass.

## Benchmark

- Stehender Pass: kein Dispatch (lokal, Routineklasse geschlossen — `grind-flash`
  $0.0008, 2026-09-16). Keine harte Atom-Klasse offen; kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
