<!--
  title: Handover — Forschung-Folge 82 (Stand 2026-09-18)
  session: Forschung-Folge 82
  class: handover
  date: 2026-09-18
  sha256: 31a7235160a985248278eddacba0e430fe5b429d6c6c6b9cd046aab452771cf8
  status: live
-->
# Handover — Forschung-Folge 82 (2026-09-18)

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

- **HEAD** `faa6c173` (während der Session vorgerückt; `origin/main` beim Push prüfen).
- **Postfach** — kein neuer Agenten-Eingang. CSES-Limadou (`212`): Sotgiu bittet,
  einige Wochen bis zur neuen Zugangsprozedur zu warten; Rubin-RSP (`200`): Review
  läuft. Kein psahelp-/NSE-Eingang. `post.md` unverändert (bc_mpo_mag-Zeile bei ernte).
- **CI** — `ci-check` `35329963293` failure; `harvest`-Serie failure; `signal-cone-audit-cdn`
  `35336801783`/`health-check` `35335138589` in_progress. Fremder Ledger, nicht angefasst.

## Measure-Gates — CI-Verdikt ausstehend (härtester undatiert)

- Gebaut und rat-/wissenschaftskorrigiert (2026-09-18, „die 2"): **Betti-0**
  `src/mathematikerin/te.rs` (`pub fn betti0_persistence(series, dim)` — `order`
  gestrichen, single-linkage/Union-Find über `embed_series`, power-of-2-Leiter,
  vier Gates) + `tools/measure/src/bin/betti0_probe.rs`; **Silence-Map-Probe**
  `tools/measure/src/bin/silence_map_probe.rs` (Katalog-KDE vs. beobachtet,
  analytisches Poisson-σ `√λ̂` → Schwelle `λ̂ − 2√λ̂`, `blind`-Klasse statt
  „consistent", Ensemble-Gate 2⁷=128, Wortlaut „Katalog-Defizit"/Silverman-`h`,
  sechs Gates). `cargo check`/`-p omegaflow-measure` 0 Fehler/0 Warnungen.
  (Schritt: `gh workflow run` des Gates-Workflows, Verdikt einmalig
  `ci_manage view <id>`; die Gates laufen nur in CI.)
- **Betti-0 Schwellenleiter — offen:** Die Zweierpotenz-Leiter hat keinen
  Literatur-Rückhalt (`uneindeutig`); Standard sind exakte kritische Werte
  (Edelsbrunner et al., DOI 10.1090/mbk/069) bzw. datengetriebene ε-Wahl
  (DBSCAN k-distance, DOI 10.1145/3068335), Persistenz-Signifikanz per
  Bootstrap-Band (Fasy et al., arXiv:1303.7117). (Schritt: Rat/Operator
  entscheidet Leiter vs. exakte kritische Werte.)
- **0-Kanon je stiller Zelle gegen `sources.φ`/`ledger.φ` — `pending`:** Der
  Umfang aus folge81 kennt nur die `absent`-Klasse; die gebaute Probe prüft die
  stille Zelle nicht gegen die Register. (Schritt: eigenes Atom bauen oder mit
  Messung descopen.)

## Fabrication-Rest — `operator-gebunden`

- `docs/concepts/kybernetische-astrophysik.md:471–479` trägt die
  Erwartungsmodell-Sprache weiter („Erwartungsmodell"), obwohl der Rat-Umfang das
  Fermi-/Technosignatur-Modell gestrichen hat (`folge81:51`). (Schritt: Konzept auf
  die Katalog-Form korrigieren oder als historische Stufe kennzeichnen.)

## Legacy-Rest — `operator-gebunden` (zurückgestellt)

- Rat-Rangfolge (2026-09-18), nur als Fortsetzung je eigener Messung:
  **Minkowski** (Delta-Probe: wie viele eigene Quellen spacelike, Archivar-Abfrage);
  **Certainty** (vC-Asymmetrik messen, exp/tanh inverse; `omega.rs:1558–1560`);
  **Delay Spectrum** (Instrument-Definition zuerst, nur TE-basiert);
  **Synthetic Flight** (nur Präregistrierung/Falsifikationsmetrik, kein Flug);
  **Channel Apertures** (Atom-9-Permeabilitäts-Binding, eine Leiter). Der
  §Ⅴ-Modell-Render der Silence Map bleibt gestrichen.

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). Post bei ernte. (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `wartend`

- Freigabe-Anfrage an `psahelp@cosmos.esa.int` gesendet (Resend `01a0b3c2-…`).
  Trigger = Antwort. (Schritt: bei Eingang authentifizierter TAP-`data`-Abruf am
  MORE-URN.)

## TE-Gate n=1000 Conditional-Null — `wartend` (fremder Ledger)

- `35324015019` @`fb6b62b4` pending. Trigger: Run-Abschluss. Nach grünem Verdikt
  wird der `211A→193A`-Conditional-Check frei (`docs/paper/solar-seconds-matrix.md:37,46`).
  (Schritt: `ci_manage view 35324015019`; TE-/Null `src/mathematikerin/te.rs`.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.), „in einigen Tagen"; TRISP/MLZ-Anfrage (`214`) läuft.
  Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler
  + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem Datum.
  (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Bau der zwei Messkanäle: `grind-pro` (2 Delegationen), Urteil `council` +
  Historien-Recherche `research-max`. Die Routine-Klasse bleibt geschlossen;
  kein Doppel-Lauf. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/mathematikerin/te.rs` (nur Betti-0-Hunks),
  `tools/measure/src/bin/betti0_probe.rs`,
  `tools/measure/src/bin/silence_map_probe.rs`,
  `docs/handover/handover-2026-09-18-forschung-folge82.md`,
  archiviertes `docs/handover/archiv/handover-2026-09-18-forschung-folge81.md`.
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, `opencode.json`,
  `src/archivar/{relay,main_flow,mod}.rs`, `src/archivar/weberin_verdicts.rs`,
  `static/{index.html,constants.js,radiator*,sensorium*,verdicts.test.mjs}`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs`, `.github/workflows/ci-check.yml`,
  die `handover-2026-09-18-{bau,ernte,entscheid}-folge*.md` und die
  `handover-2026-09-16-*`-Renames/Deletes.
- **Untracked:** `state/mail/psa-helpdesk-more-freigabe.md`,
  `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (gitignored).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
