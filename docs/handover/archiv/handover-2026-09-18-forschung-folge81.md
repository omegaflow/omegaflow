<!--
  title: Handover — Forschung-Folge 81 (Stand 2026-09-18)
  session: Forschung-Folge 81
  class: handover
  date: 2026-09-18
  sha256: 876f8c43617b8867a2c723bca8c91cc3d585af7dc65f03402d82929f5686739b
  status: live
-->
# Handover — Forschung-Folge 81 (2026-09-18)

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

- **HEAD** `427f029a` == `origin/main`; Safety-Net `refs/safety/1789719897`.
- **Postfach** — kein Agenten-Eingang; `post.md` trug nur die eigene ernte-Zeile.
- **CI** — `te-gate` `35324015019` @`fb6b62b4` **pending** (Queue seit 08:22Z);
  `harvest` 08:20-Serie failure; `ci-check` in_progress; `allwise-cdn`/
  `planetary-odf-cdn` in_progress. Shared state nicht angefasst.

## Silence-Map-Probe — `operator-gebunden` (Bau-Wort ausstehend)

- Rat-Verdikt steht (read-only): Korpus `dr3_stars.bin` (registriert
  `phi/sources.φ:7750`, CDN-manifestiert); kubische ICRS-Zellen `2^n m`
  (Default Median-Nächster-Nachbar); Silverman-KDE des vollen Katalogs vs.
  beobachtete Dichte; 10 Poisson-Thinning-Surrogate, `mean+2σ`, voller-Kreis-RNG;
  0-Kanon je stiller Zelle gegen `sources.φ`/`ledger.φ`; vier `te.rs`-Gates im
  selben Atom. Kein Fermi-/Technosignatur-Modell, keine Zeit-/Spektralachse,
  kein PE-Gate, kein Rendering/WGSL/ω-Loop, kein Register-Schreiben.
  Reihe unverändert: Silence Map → Certainty/vC → TDA/Betti-0 → Minkowski.
  (Schritt: Bau-Wort → `tools/measure/src/bin/silence_map_probe.rs` nach dem
  präzisierten Umfang; Gates über CI.)

## MAG-Asset — `blockiert` (ernte)

- `bc_mpo_mag` anonym abgerufen: `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip`
  (853331 B, sha256 `6f3724f7…`, valides ZIP, PDS4-`.tab` 4.83 MB + Label).
  CDN-Manifestation braucht einen PDS4-`.tab`-Parser; Post an ernte gesetzt.
  (Schritt: `tools/harvest/src/bin/bc_mpo_mag_compiler.rs` nach Muster
  `voyager_odr_compiler.rs`, dann `sources.φ`-Eintrag + CI-Manifestation.)

## BepiColombo MORE — `wartend`

- Register korrigiert (`phi/blocked_sources.φ:26-29`): `bc_mpo_more` (Radio
  Science) ist proprietär (`release_date` 2099-01-01, 89434/89517, keine offene
  Collection, `data?PRODUCT` → 403), **kein** Konto-Gate. Freigabe-Anfrage an
  `psahelp@cosmos.esa.int` gesendet 2026-09-18 (Resend `01a0b3c2-…`); Entwurf
  `state/mail/psa-helpdesk-more-freigabe.md`. Trigger = Antwort. (Schritt: bei
  Eingang authentifizierter TAP-`data`-Abruf am MORE-URN.)

## TE-Gate n=1000 Conditional-Null — `wartend` (fremder Ledger)

- Lauf `35324015019` @`fb6b62b4` pending. Trigger: Run-Abschluss. Nach grünem
  Verdikt wird der `211A→193A`-Conditional-Check frei
  (`docs/paper/solar-seconds-matrix.md:37,46`). (Schritt: `ci_manage view
  35324015019`; TE-/Null-Konstruktion `src/mathematikerin/te.rs`, Kalibrier-Gate
  FP/FN/Symmetrie/n-Floor.)

## NSE/Haug — `wartend`

- Keller-Antwort (17.09.), „in einigen Tagen". Trigger = Dateieingang.
  (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`.)

## Legacy-Konzepte — `operator-gebunden` (nach Silence Map)

- vC-Definition L:53 **getragen** (`src/mathematikerin/omega.rs:1558–1560`);
  Certainty, TDA/Betti-0, Minkowski als 4. weiter `pending`; Nostr hinten.
  (Schritt: nach der Silence Map die vC-Asymptotik-Messung und TDA/Betti-0 als
  je ein Atom.)

## Paper / Präregistrierung — `termin`

- Flyby Path 2 datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- BepiColombo-Recherche + Silence-Map-Rat: `research-max`/`council` für ein
  Urteils-Atom (Route-/Architektur-Entscheidung), kein Routine-Doppel-Lauf.
  Die Routine-Recherche-Klasse bleibt geschlossen (2026-09-16).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-18-forschung-folge81.md`,
  `docs/handover/post.md` (eigene Zeile),
  `phi/blocked_sources.φ` (eigener Hunk 26-29),
  archiviertes `docs/handover/archiv/handover-2026-09-18-forschung-folge80.md`.
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, `opencode.json`,
  `src/archivar/*`, `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`, die `handover-2026-09-18-{bau,ernte,entscheid}-folge*.md`.
- **Untracked:** `state/mail/psa-helpdesk-more-freigabe.md`,
  `data/psa.esa.int/mag_der_sc_ib_a001_e2k_00000_20181024.zip` (gitignored).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
