<!--
  title: Handover — Forschung-Folge 93 (Stand 2026-09-19)
  session: Forschung-Folge 93
  class: handover
  date: 2026-09-19
  sha256: 3e2e9084d5b3f6a44a821a72a4ddffe90139a4b0bcbd8f9167e2f90991794f82
  status: live
-->
# Handover — Forschung-Folge 93 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Forschung-Folge 93)

- **HEAD** `88fb27ad` == `origin/main`; `git_safety` Snapshot `refs/safety/1789837073`
  (Session-Beginn). Zwischen Pass und Abschluss hat die forschung-folge92-Nachbarsession
  den Baum vorangezogen (`778c68db` → `49801083` → `88fb27ad`); deren Fold der
  Hyperscanning-Übergabe ist in dieses Register übernommen. Fremde uncommittete Arbeit
  im geteilten Baum (s. u.) — nicht angefasst.
- **Postfach** — kein `An forschung`-Eingang; `post.md` trägt nur `An bau` (3 Zeilen,
  fremd). `state/mail/mail_ledger.φ` ohne neuen Eingang.
- **CI** — `ci_manage list`/`view` (2026-09-19): **in_progress** `measure-gates`
  `35456056999` @`f2a30682`; **success** `hyperscanning-te` `35456288477` @`778c68db`
  (Lauf grün, Messung 0-honored — s. u.); **pending** `ci-check` `35456216825`;
  **failure** `measure-gates` `35454958335` @`5cba9f70` (der alte silence_map FN-Lauf).
  Die CI-Zeile in `docs/zustand/external-state.md` (Entscheid-Folge 54) wird zitiert,
  nicht kopiert.

## Hyperscanning-TE screen — Loader-Fix gebaut, CI-Verifikation ausstehend (härtester undatiert)

- **Messung** (`ci_manage log 35456288477 --all`): Lauf @`778c68db` grün, aber jeder
  Triad `absent — no readable [Fz] series (0 honored)`; 99 `.set` geladen.
- **Ursache** (in-session am S3-Objekt `sub-G01S01_..._eeg.set` per `od` gemessen):
  das ds007822-`.set` ist MAT-v5 mit Top-Level-Variablen `data` (single [19,1500,40]),
  `setname`, `nbchan`, `pnts`, `trials`, `srate`, `xmin`, `xmax` — **kein `chanlocs`**;
  die Kanalnamen liegen nur im BIDS-`_channels.tsv`-Sidecar (Header `name`), das der
  Workflow nicht lud. `eeg_from_source` verlangte eingebettete `chanlocs` → absent.
- **Fix** (in-session): `eeg_from_source` lässt leere Labels zu; neu
  `parse_channels_tsv` + `labels_from_channels_tsv` (Sibling `_channels.tsv`); beide
  Bins injizieren die Labels, wenn das Set keine trägt; der Workflow lädt die
  `_channels.tsv`-Sidecars mit. `cargo check -p omegaflow-measure --all-targets`
  0 Fehler/0 Warnungen. (Schritt: nach Commit/Push `gh workflow run
  hyperscanning-te.yml`, dann `ci_manage view <id>` + Artefakt
  `hyperscanning-te-report`; ein lesbares [Fz]-Series + familienweiser Schwellenwert
  = die Messung.)
- **Bestätigungsstufe** — der Screening-Lauf fährt p95/200; die Überlebenden brauchen
  p99/1000. (Schritt: `hyperscanning_group_te --percentile 99 --surrogates 1000` auf den
  Survivor-Zellen; eigener Workflow-Dispatch.)
- **Kohärente Phasen-Null** — gemeinsame Rotation beider Serien (der eigentliche
  Paar-Null; heute trägt die Einzel-Phasen-Null nur als konservative Obernull).
  (Schritt: neues `TeNull`-Modell in `src/mathematikerin/te.rs` + Kalibrier-Gate-Test
  im selben Atom.)
- **Eigen-Historie als Konditionierer** — die eigene Vergangenheit des Ziels ist der
  nächstliegende Konfundierer; der Apparat existiert (`conditional_te_*_n` mit `conds`).
  (Schritt: `hyperscanning_group_te` ruft `&[]` — `LaggedCond` auf die Zielserie setzen.)
- **Mehrere Frontalkanäle** — Fz/F3/F4 als getrennte Matrizen. (Schritt: `--channel F3`/`F4`.)
- **Topologische TE** (Takens, `te_compute`) als Upgrade der binned 4-Bin-Schätzung.
  (Schritt: `topological_te_phase` statt `transfer_entropy_binned` im Gruppen-Werkzeug.)

## silence_map_probe FN-Gate — Verifikation (aus Folge 92)

- Lauf `35456056999` @`f2a30682` **in_progress**; der Fix `1056bc88` liegt im
  Lauf-Baum (per `git merge-base` verifiziert). (Schritt: `ci_manage view 35456056999`
  + Artefakt `measure-gates.txt`; grün ⇒ FN-Gate frei.)

## vC-Permeabilitäts-Karte — `operator-gebunden` (an entscheid gepostet)

- **Messakt**: `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>
  ./target/release/omegaflow` lokal nach Core-Release-Build; danach `perm_target_probe
  --live <pfad>`. (Schritt: Operator-Wort + Release-Build; siehe `post.md`.)
- **Richtungssemantik**: Legacy `exp(-vC/(g+1/C))` vs. heute `tanh(v_c/(g+PERM_GROUND))`.
  (Schritt: Operator-Wort.)

## TE-Gate n=1000 Conditional-Null — `wartend` (bau-eigen)

- `35427414837` @`fe6bdb2b`; neuer `te-gate` `35451283398` pending. (Schritt:
  `ci_manage view 35451283398`; grün ⇒ `211A→193A`-Conditional-Check frei,
  `docs/paper/solar-seconds-matrix.md:37,46`.)

## MAG-Asset — `blockiert` (ernte)

- `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (853331 B, sha256
  `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB). (Schritt:
  `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach `voyager_odr_compiler.rs`,
  dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `termin`

- Cruise-Daten erst zur Wissenschaftsphase (~April 2027) freigegeben. (Schritt:
  kein TAP-Abruf vorher.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.): „in einigen Tagen"; er sendet die TRISP-NSE-Daten
  selbst. Trigger = Dateieingang. (Schritt: bei Eingang `nse_haug_trisp`-Quelle +
  Compiler + `sources.φ`.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Der Loader-Gap wurde in-session (line, flash) gemessen und gefixt; kein
  pro/max-Dispatch — der geplante research-max-Auslöser „roter Lauf" trat nicht ein
  (der Lauf war grün, die Messung 0-honored). Burn (gemessen, `session_burn`):
  Gesamtburn $1.0029 / 46 Sessions; line-Anteil (Flash) $0.6322 / 6 Sessions.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/eeglab.rs` (Labels optional + channels.tsv-Leser + Tests)
- `tools/measure/src/bin/hyperscanning_group_te.rs` (Label-Injektion)
- `tools/measure/src/bin/hyperscanning_te_matrix.rs` (Label-Injektion)
- `.github/workflows/hyperscanning-te.yml` (channels.tsv-Download)
- `docs/handover/handover-2026-09-19-forschung-folge93.md` (neu)
- `docs/handover/archiv/handover-2026-09-19-forschung-folge92.md` (Move)
- **Fremd (nicht angefasst):** `docs/zustand/external-state.md` (Entscheid-Folge 54),
  `docs/handover/post.md`, `phi/blocked_sources.φ`, `phi/pipeline/ledger.φ`,
  `src/archivar/hdf5.rs`, `src/mathematikerin/te.rs`,
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`, die `handover-2026-09-1*`-Moves,
  `?? bin/register_lookup`, `?? handover-2026-09-19-{bau-folge90,entscheid-folge54,ernte-folge94}.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
