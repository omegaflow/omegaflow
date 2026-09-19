<!--
  title: Handover — Entscheid-Folge 53 (Stand 2026-09-19)
  session: Entscheid-Folge 53
  class: handover
  date: 2026-09-19
  sha256: d9bdd3376cef5225d1b4df92c15e6d89854c3eadc6224461ccbefabed854c9bb
  status: live
-->
# Handover — Entscheid-Folge 53 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Entscheid-Folge 53)

- **HEAD** `1eef6949` (== `origin/main`, Forschung-Folge 91); `git_safety`
  Snapshot `refs/safety/1789835428`.
- **CI** — Watchdog-Snapshot (`/tmp/opencode/ci_status.md`, 17:56Z) +
  `ci_manage list`/`view` (2026-09-19 ~18:33Z): **in_progress** `ci-check`
  `35455108694` @`1eef6949` (HEAD-Gate läuft), `health-check` `35451454869`
  (pending), `te-gate` `35451283398` (pending); **failure** `measure-gates`
  `35454958335` @`5cba9f70` — `cargo test --release -p omegaflow-measure --bin
  silence_map_probe` exit 101 (Forschung-Folge 91 Re-Dispatch, `ci_manage log
  35454958335 --all`); **failure** `harvest-dispatch` `35451187565` (cassini
  dispatch HTTP 422 unexpected inputs `format`/`timeout`); **success**
  `cassini-rsr-cdn` `35451218520`/`35451194865`, `cassini-odf-cdn`
  `35451207386`/`35451193936`, `auto-dispatch` `35451187530`, `quake-feeds-cdn`
  `35449063079`, `ned-cdn` `35448969689`, `ps1-cdn` `35448643344`,
  `swpc-mirror-cdn` `35448402406`. Wert in `external-state` fortgeschrieben.
- **Postfach** — letzter Ledger-Eingang `1789795811` (Rubin-Forum AGN-Lightcurves,
  informativ); kein neuer Eingang. Eintrag in `external-state` zitiert (nicht fällig).
- **Post** — die `An entscheid`-Zeile (vC-Permeabilitäts-Karte, aus Forschung-Folge
  91) gefaltet, Zeile aus `post.md` gelöscht.
- **Arbeitsbaum** — fremd uncommittet (nicht angefasst): `.github/workflows/harvest-dispatch.yml`,
  `src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`,
  `tools/harvest/src/bin/{cassini_odf,cassini_rsr,icesat2_atl03}_compiler.rs`,
  `tools/measure/src/eeglab.rs`, `tools/measure/src/bin/hyperscanning_te_matrix.rs`
  (neu); drei `handover-2026-09-16-*`-Renames (staged: `entscheid-folge24`,
  `forschung-folge44`/`-folge51` → `archiv/`).

## Handlungsfähig — Auswahlpunkte

**Kein session-abarbeitbarer undatierter Punkt.** operator-gebunden/blockiert/
termin/wartend sind keine Auswahlpunkte; die operator-gebundenen liegen dem
Operator vor.

### Operator-Entscheidungen 2026-09-19

- **vC-Permeabilität** — über **Gremium + Wissenschaft hergeleitet** (2026-09-19).
  Verdikt: **keine Inversion** — Legacy `exp(-vC/(g+1/C))` war eine *Certainty*
  (Gewissheit, komplementär), heute `tanh(v_c/(g+PERM_GROUND))` (`omega.rs:15-17`)
  ist die *Permeabilität* (beschränkt, aufwärts sättigend, Verbraucher `aperture`
  `omega.rs:345`). Das Argument ist **dimensionslos** — `v_c = |Δω_sum|`,
  `g = |ω_sum|` (`omega.rs:1620-1621`), also Δω/ω; der Gremium-Konfund „Rate 1/T"
  ist am Code widerlegt. Physikalisch wahrer Kern ist der TE-Pfad
  `inTE/(inTE+threshold+ε)` (null-kontrolliert); der tanh-Fallback trägt dieselbe
  Monotonie, aber **ohne Surrogat-Nullkontrolle** (vC misst Rauschen + Signal).
  Offen: (a) Nullkontrolle des Fallbacks, (b) Saturations-Skala der Live-Verteilung
  v_c/g. Die Permeabilität wird **nicht auf CDN abgelegt**; der Messakt bleibt
  lokal: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow`
  nach Core-Release-Build, danach `perm_target_probe --live <pfad>`.
  (Schritt: (a)+(b) an bau/forschung; kein CDN.)
- **Betti-0-Schwelle** — **muss gemessen werden** (Operator-Wort; Verdikt
  bestätigt: Leiter behalten, `persistence` leiter-unabhängig). Erster Messschritt
  (Null-Verteilungs-Quantil über ≥100 Realisierungen, `measure-gates`,
  `betti0_probe.rs`) geht an bau; Skalar-Reduktion + 0.5-Schwelle
  (`te.rs:3432,3454`) sind der Gegenstand.
- **Pipeline-Port force-Gate** — A/B offen; Korpus-Inventur beantwortet (Korpus =
  10 gitignorierte `queue/`-Dateien, ~3.683 Blöcke / ~1,8 MB; CI-Problem = Upload
  der gitignorierten Korpora + Netz-Proben; lokaler Lauf = `--port` lokal (billig)
  + `--probe` mit tausenden Netz-Fetches). (Schritt: Operator-Wort A/B.)
- **ESP32-Modul** — **on hold** (Operator-Wort 2026-09-19); BOM
  `docs/specs/mantis-shrimp-bom.md`.
- `termin` — **Lasair-LSST**: API 502 (Root + `/api/query` + Token-Query) über
  Proton-Exit; Re-Messung am 2026-09-19 bestätigt (direct + Proton 502, Wayback 200
  ohne Snapshot). (Schritt: `archive_search --verdict
  https://api.lasair.lsst.ac.uk/api/query` — Trigger Banner-Wechsel / nächste
  Wiedervorlage.)
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

- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-24 — Rubin-Forum-Umzug auf `rubin.community`.
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).
- ~2027-04 — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-19-entscheid-folge53.md` (neu)
- `docs/handover/archiv/handover-2026-09-18-entscheid-folge52.md` (Move)
- `docs/zustand/external-state.md` (CI-Zeile; Header-sha)
- `docs/handover/post.md` (`An entscheid`-Zeile gelöscht; Header-sha)
- Fremd uncommittet (nicht angefasst): siehe Stehender Pass.

## Benchmark

- Stehender Pass: kein Dispatch (lokal, Routineklasse geschlossen — `grind-flash`
  $0.0008, 2026-09-16).
- vC-Herleitung: `council` (pro/max) + `research-max` (pro/max) parallel dispatcht
  — hartes Urteils-/Recherche-Atom, kein flash-Äquivalent (die Gremien sind die
  benannten Körper). Beide lieferten; der benannte Dimensions-Riss des Gremiums
  ist am Code widerlegt (`omega.rs:1620-1621`). Kein Doppellauf.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
