<!--
  title: Handover — Entscheid-Folge 56 (Stand 2026-09-19)
  session: Entscheid-Folge 56
  class: handover
  date: 2026-09-19
  sha256: 0176f3d1c136d264b4537d3021b04d0df1d6cd6a8a1664f96c354da5067f68c1
  status: live
-->
# Handover — Entscheid-Folge 56 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 56)

- **HEAD** `710425c4` (== `origin/main`, bau folge92). Seit Folge 55 (`a786e208`)
  ist der Baum über `3d2e6adb` (forschung-folge93-Move) und `7358143e` (ernte
  folge96) auf `710425c4` gezogen — fremde Linien committen und pushen;
  Fast-Forward. `git_safety` Snapshot `refs/safety/1789846433`.
- **CI** — `ci_manage list` (2026-09-19 ~19:35Z @`710425c4`): **in_progress**
  `allwise-cdn` `35464786486`, `fmt-apply` `35464780631`; **pending** `ci-check`
  `35464774675`, `te-gate` `35462518676` @`3d2e6adb`; **success** `harvest-dispatch`
  `35463566653`; **failure** `fmt-apply` `35462507189` (früherer Lauf);
  **cancelled** `ci-check` `35463566592`. Wert in `external-state` bei HEAD-Wechsel
  fortgeschrieben.
- **Postfach** — `mail_ledger.φ` (mtime 07:30) jüngster Eingang `1789795811`
  (Rubin-Forum `[Science] AGN DP2`, informativ); kein neuer Eingang seit Folge 55.
- **Post** — die drei `An entscheid`-Zeilen, die **bau folge92** in `710425c4`
  committete (vC-Vollzug, A/B force-Gate, register_lookup-Symlink), **gefaltet**
  (unten). `post.md` wieder leer; dabei den seit bau folge90 (`1b0c3303`) **stalen
  Header** `2b03eabc` auf den gemessenen Leerbody-Wert `5d818b85` korrigiert
  (`omega_sh sha`).
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): `AGENTS.md`,
  `bin/ci_watchdog.sh`, `tools/measure/src/bin/hyperscanning_group_te.rs`,
  `docs/zustand/external-state.md` (CI-Zeile @`3d2e6adb`, forschung folge95),
  `handover-2026-09-19-forschung-folge95.md`, `forschung-folge94`-Rename, drei
  `handover-2026-09-16-*`-Renames.
- **Zustand-Lücke** — `external-state.md` CI-Zeile trägt fremd-uncommittete Arbeit
  (forschung folge95, @`3d2e6adb`) und ist bei HEAD `710425c4` **erneut fällig**;
  der Nachtrag gehört dem Editor, der den fremden Hunk committet (write boundary:
  nicht überschrieben). (Schritt: nach dessen Commit `ci_manage list` @`710425c4`.)

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
termin/wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem
Operator vor.

### Operator-Entscheidungen

- **Pipeline-Port force-Gate — binär A/B** (seit `entscheid-folge46`, neun
  Sessions; aus dem Postfach gefaltet). Korpus: 10 gitignorierte
  `phi/pipeline/queue/*.φ` (~3.683 Blöcke). A — lokal: `--port` (billig, kein
  Netz) + `--probe` mit tausenden Netz-Fetches lokal; Beleg bleibt
  lokal/ungemanifestiert. B — CI: Upload der 10 Korpora + Netz-Proben; geteilter
  Beleg, aber gitignore öffnen + Upload-Pfad ungebaut. (Schritt: **Operator-Wort
  A/B**; danach baut **bau** den gewählten Pfad.)
- **vC-Permeabilität — Vollzug** (aus dem Postfach gefaltet). Operator-Wort
  2026-09-19 („Messakt lokal, kein CDN") steht; gebunden ist der Vollzug:
  Release-Bin → `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>` →
  `perm_target_probe --live`. Der Akt liegt auf der Operator-Maschine (lokaler
  Funktionslauf), die Session kann ihn nicht vollziehen. (Schritt: Operator-Wort/
  Vollzug; **bau** liest `perm_target_probe --live`.)
- **register_lookup-Symlink** (aus dem Postfach gefaltet).
  `~/.local/bin/register_lookup` zeigt auf `target/release` statt
  `bin/register_lookup`; Fix ist ein PATH-Eingriff im Operator-Domain, verschränkt
  mit dem ausstehenden `release-build`-Run. (Schritt: Operator setzt Symlink bzw.
  Release-Build-Wort; danach register_lookup-Verifikation.)
- **ESP32-Modul** — on hold (Operator-Wort 2026-09-19); BOM
  `docs/specs/mantis-shrimp-bom.md`.
- `termin` — **Lasair-LSST**: API 502 (direct + Proton), Token in `.secrets.local:59`,
  Backend-Upstream absent; Re-Messung 2026-09-19 bestätigt. (Schritt:
  `archive_search --verdict https://api.lasair.lsst.ac.uk/api/query` — Trigger
  Banner-Wechsel.)
- `termin` — **BepiColombo MORE** (`bc_mpo_more`): Cruise-Daten erst zur
  Wissenschaftsphase (~April 2027); PI-Iess-Antwort `1789729151`. (Schritt:
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

- `docs/handover/handover-2026-09-19-entscheid-folge56.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-entscheid-folge55.md` (Move)
- `docs/handover/post.md` (Header-sha `2b03eabc` → `5d818b85`; drei
  `An entscheid`-Zeilen gefaltet)
- Fremd uncommittet (nicht angefasst): `AGENTS.md`, `bin/ci_watchdog.sh`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`, `docs/zustand/external-state.md`,
  `handover-2026-09-19-forschung-folge95.md`, `forschung-folge94`-Rename, drei
  `handover-2026-09-16-*`-Renames.

## Benchmark

- Kein Dispatch: reine Register-Arbeit, Routineklasse geschlossen (`grind-flash`
  $0.0008, 2026-09-16). Kein hartes Atom offen; kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
