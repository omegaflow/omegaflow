<!--
  title: Handover — Galileo-Tiefe: Rotor-Spin epochal verankert, Paper über dem Gate, Receiver-Ursachen-Pfad CI/CDN-bereit
  class: handover
  date: 2026-09-05
  sha256: 3d558152680f3eec499011f794d085426ec9ba9bc15164a31389c5d12a6b30db
  status: live
  see-also: docs/handover/handover-2026-09-05-galileo-epsilon-revision.md docs/paper/galileo-rotor-spin-era-floor.md docs/TODO.md AGENTS.md
-->

# Handover: Galileo-Tiefe — Rotor-Spin epochal verankert, Paper über dem Gate, Receiver-Ursachen-Pfad CI/CDN-bereit

Eine frische Session liest genau dieses Dokument und führt den Stand weiter. Geschrieben
von der Session, die den ε-Achsen-Handover (`handover-2026-09-05-galileo-epsilon-revision`)
vollständig ausgeführt und weit über den ursprünglichen Auftrag hinausgetrieben hat.

## Kurzstand

Der Galileo-Faden ist in seiner Tiefe bearbeitet: **27 Galileo-Befunde** (`docs/befund/
befund-galileo-*.md`), **1 Paper über dem Rust-Paper-Gate** (`docs/paper/
galileo-rotor-spin-era-floor.md`), **~24 neue Mess-/Harvest-Proben** (`tools/measure/src/bin/
galileo_*.rs`, `tools/harvest/src/bin/galileo_*.rs`, `ck_daf_probe.rs`, `galileo_floor_4d.rs`,
`galileo_atdf_receiver_compiler.rs`), plus Dunkle-Materie/Front-C-Folgen. Alle Commits dieser
Session waren **pfad-beschränkt** (`git commit -- <datei>`) — die parallele Session arbeitet
aktiv am selben Repo (commit_gate-Refactor, Bio/Korona); ihre Staging-Inhalte nie überschrieben.

## Was geschlossen wurde (Häkchen)

1. **ε-Achsen-Revision** (`befund-galileo-rausch-kurve` v2 + `-epsilon`): der gemessene Winkel
   war α (Sonnenort), nicht ε (Elongation). Plasma-Deutung **getötet**; die Fall-Magnitude ist
   **Ära/Sonnenzyklus-konfundiert** (kollabiert unter Ära-Kontrolle auf ~2×;
   `befund-galileo-alpha-zeit-sonnenzyklus`).
2. **Der „Station-42-Ton" (52,39 mHz) ist epochal als Galileo-Rotor-Spin bestätigt**: die
   −77000-Rotor-CKs für 1990-12-07..10 messen in Rust **52,39006 mHz (19,0876 s)** — Verhältnis
   **1,000001** zum Ton (`befund-galileo-rotor-spin-epoch-anchor`; `ck_daf_probe.rs` — Rust-DAF-
   Leser, um Big-Endian-DAF + type-1-Quaternion-Dekodierung erweitert). Dez-1990 **Dual-Spin**
   bestätigt (`befund-galileo-1990-ck-dualspin`). (Das externe „0,0525-Hz-Dual-Spin"-Zitat war
   nur 3,15 rpm in Hz — korrigiert.)
3. **Das Paper** `galileo-rotor-spin-era-floor.md` steht über dem Gate: `export_latex --check`
   Titel 68/75, Abstract 148/200, Zahlen ok, sha ok; `reference_verify` exit 0. Ein gemessenes
   Verdikt: die eine epochal verankerte Linie (Rotor) gegen eine entkoppelte/era-konfundierte
   Rausch-Decke (null konditionale TE gegen ref/mode/Kadenz; Same-Day null).
4. **TE-Maschinerie auf den Galileo-Daten** (das omegaflow-Kerninstrument, das vorher fehlte):
   Record-Feld-TE (`befund-galileo-te-staerke-floor`: −2560-Boden nicht gerichtet treibend auf
   Tages-Achse), Spec-TE (`befund-galileo-te-spec`: entkoppelt), Same-Day-Spec (null), In-Pass-
   Stärke-Rampe (`befund-galileo-inpass-staerke-rampe`: an 43/63 echte Boden↔Rauschen-Kovarianz,
   an 14 Epochen-Kollokation), In-Pass-Richtung (simultan), 4D-Form+Farbe.
5. **Receiver-Ursachen-Pfad (R-RX) ist CI/CDN-bereit**: `galileo_atdf_receiver_compiler.rs`
   (extrahiert Item 71 Doppler-Receiver-Referenz 4-bit, Item 92/94/95; Korrektur: Receiver-
   Referenz = Item 71, nicht 73) + `.github/workflows/galileo-receiver-cdn.yml`. **`galileo_
   receiver.bin` liegt auf dem CDN-Release `pds-ppi.igpp.ucla.edu`** (CI-Lauf 33975873380, ✓).
6. **Externe Recherche** (Multi-LLM, Ergebnis in `~/Schreibtisch/rechercheauftrag-extern-galileo-
   dsn_findings.md`): AGC = dBm×10 bestätigt (Simpson-Erratum + TRK-2-34-Kalibrierformel);
   Receiver-Ära (BVR-Rollout Sept 1995, 1995er-Wartungszyklus, DGT ab 1996, 5.12.1995-
   Modulationswechsel); 60-s = dokumentierte Count-Time-Option.

## Was offen ist (echte Arbeit, nicht nur registriert)

1. **DER nächste Schritt — der Floor-Ursachen-Test**: `galileo_receiver.bin` liegt auf dem CDN;
   jetzt die **Receiver-Identität (ref 3/4/5, Amp-Typ) gegen die station-gebundene Floor-Lautheit**
   über die ganze Floor-Ära testen. Die 4D-Analyse zeigte: identische (x,y,z,t), verschiedene
   Station → 0,02–500 Hz (st43 0,031 vs st14 25,85 Hz, 1995-11-24) = **Ground-Receiver-Signatur**.
   Das Asset ist da; der Test fehlt. Der Compiler liest `data/galileo_receiver.bin` (GARX,
   96 B/sample); eine Measure-Probe + Befund ist zu bauen. Sample-TDF-Facts in
   `/tmp/opencode/` (evtl. nicht mehr da — neu ziehen).
2. **All-Spin-Bus-CK (−77000) epochal** — bereits durch die Rotor-CK-Dekodierung epochal verankert;
   kein weiterer Bedarf, außer man will die Dec-10 ~0,29-mHz-Halb-Tages-Schwingung.
3. **Der ~2×-Rest nach Ära-Kontrolle** (echte Geometrie oder Rauschen?) — daten-dünn, ein
   1996-Fenster (Opposition 1991–95 n=0, Konjunktion 1990–93 n=0, gemessene Leere).
4. **omni2-/f107-Ergebnis**: der nahe-Erde-Sonnenwind erklärt die laute Pioneer-1978–82-Ära
   NICHT (HSS-ärmste Epoche; refuted) → DSN-Empfangs-/Betriebs-Ära-Baseline als Rest-Erklärung.
   Gleiche Logik auf den Galileo-Ära-Boden übertragbar.

## Daten & Assets

- Resid: `data/galileo_resid.bin` (900 MB, 14 077 825 Samples). Receiver: CDN
  `galileo_receiver.bin` (auf `pds-ppi.igpp.ucla.edu`-Release). Ephemeride: `data/ephemeris_
  galileo_daily.bin`. Rotor-CKs: `/tmp/opencode/ck90341..344_rtr.bc` (evtl. nicht mehr da).
- CDN (`omegaflow/sources`): 203 Releases (je Quell-Host); `pds-ppi` hält `galileo_resid.bin` +
  `galileo_receiver.bin`; `naif` noch leer (gll-ck lädt erst bei eigenem Dispatch).

## Parallel-Session-Hinweis (wichtig)

Die andere Session arbeitet aktiv und committet/refactored (commit_gate, Bio/Korona, Nadel-XIII).
**Immer pfad-beschränkt committen** (`git add <mein> && git commit -- <mein>`), nie blind — sonst
wandern ihre gestagten Änderungen in eigene Commits. `phi/sources.φ` wird von beiden berührt —
bei Konflikten nicht überschreiben, abstimmen.

## Abschluss

Der Galileo-Faden ist im Kern geschlossen (ε-Revision, Plasma getötet, Ton epochal verankert,
Paper über dem Gate). **Der eine offene, heute machbare Schritt ist der Receiver-Ursachen-Test**
(galileo_receiver.bin → Probe → Receiver-Identität vs. Floor-Lautheit → Befund), plus das ~2×-Ära-
Rest-Fenster. Erst wenn der Floor eine benannte Ursache hat (Receiver-Zustand je Station), ist die
Galileo-Rausch-Frage vollständig — nicht nur als begrenztes Negativ charakterisiert.
