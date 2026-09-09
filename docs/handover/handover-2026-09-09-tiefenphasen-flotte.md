<!--
  title: Handover — Tiefenphasen-Flotte: drei Atome schließen die Pilot-Linie — 1-km-Inversionsklasse + gemeinsame Lib, die Freiflächen-Polarität (R_pp/R_sp abgeleitet), und die Flotte (16 Ereignisse, Mittelwert +1,7 km unverzerrt, se 4,7 km; Streuung 19/36 km dominiert das ±10-km-Gate)
  class: handover
  date: 2026-09-09
  sha256: eb36854e5a57a4a5d98433fae42e3a9634a9b45ffef7b96bc58b2d1d6294a0bc
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-feldpilot.md docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-inversionsklasse.md docs/befund/befund-2026-09-09-tiefenphasen-polaritaet.md docs/befund/befund-2026-09-09-tiefenphasen-flotte.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->

# Handover — Tiefenphasen-Flotte

Übergabe für die nächste Sitzung. Drei Atome, drei Befunde, drei Commits —
`1232e90` (Inversionsklasse), `d5fa10b` (Polarität), `3b9c70d` (Flotte). Das
konsumierte Handover (feldpilot) liegt im Archiv.

## 1. Was gebaut und gemessen wurde

- **Atom A — 1-km-Inversionsklasse** (`1232e90`): der Picking-Kern zog in
  `tools/measure/src/depthphase.rs` (gemeinsame Lib, verhaltensneutral, Tests
  umgezogen); die Inversion rastet 0–250 km in 1-km-Schritten statt der
  50-km-`DEPTHS_FINE`-Raste. 231 km invertiert auf 231 km. Befund
  `befund-2026-09-09-tiefenphasen-inversionsklasse.md`.
- **Atom B — pP/sP-Polarität** (`d5fa10b`): die Freiflächen-Reflexions-
  koeffizienten R_pp/R_sp aus der ak135-Oberflächenschicht (α 5,80, β 3,46),
  Energie-Erhaltung als Vorzeichen-Test. R_pp negativ im ganzen Pilotband
  (Nullstelle 53,9°, Pilot ~27,5° → −0,65), R_sp ≈ −1. Das gemessene
  „sP durchgehend negativ" trägt die Freifläche allein; die pP-Mischung
  (4×+, 2×−) trägt der Quell-Strahlungsterm (`pending` ohne CMT). Befund
  `befund-2026-09-09-tiefenphasen-polaritaet.md`.
- **Atom C — die Flotte** (`3b9c70d`): `depth_phase_fleet_probe.rs` läuft die
  Ereignis-Schleife über die gemeinsame Lib und akkumuliert mean/sd/√N
  (`mean`/`sample_sd` in `stats.rs`). Nebenbefund: der MiniSEED-Decoder liest
  Fehlkörper jetzt `absent` statt zu panicken (Grenzprüfung, 2 Tests). Befund
  `befund-2026-09-09-tiefenphasen-flotte.md`.

## 2. Die Messung (die Flotte)

26 registrierte Tiefereignisse in der Hindu-Kush-Box; die 16 größten gemessen
(M6.2–7.5, Katalog-Tiefe 107,7–231,0 km). Per-Ereignis-Offsets (Median −
Katalog): `[+11, −3, +19, −18, +33, −13, +5, +14, −27, +25, −14, +25, −7, −11,
−25, +12]` km. **Mittelwert +1,7 km (unverzerrt, se 4,7 km);** σ 19 km über
Ereignisse, 36 km über Stationen dominiert das ±10-km-Gate — der Pilot-Offset
+19 km war Streuung, kein Bias. Der Engpass ist die Korrelation, nicht das
Modell.

## 3. Offene Nachfolger (benannt, nicht gedeutet)

1. **Zonen-Flotte** — global tiefe Zonen (Tonga 410–660 km, Bonin, Banda,
   Ägäis), neue Auswahlregel vor dem ersten Fetch registrieren; braucht zuerst
   die Tiefherd-Erweiterung über 250 km.
2. **Tiefherd-Erweiterung über 250 km** — `MAX_DEPTH_KM = 250` in `ak135.rs`
   kappt die Tiefenphasen; Voraussetzung der Zonen-Flotte.
3. **Streuung senken** — der mehrdeutige pP-Zweig bei Δ≈30° (Triplikation) und
   das Coda-Verhaken der Korrelation sind der gemessene Engpass.
4. **Quell-Term/CMT** — das Vorzeichen-Verhältnis der auf-/absteigenden
   P-Strahlung; `pending` ohne CMT-Lösung.

## 4. Zustand des Arbeitsbaums

- Arbeitsbaum sauber; alle drei Commits auf `origin/main` (nachgemessen: `git
  merge-base --is-ancestor` für 1232e90/d5fa10b/3b9c70d gegen origin — alle
  gepusht). Kein Reset, kein Stash-Drop von mir.
- `cargo check` 0/0; Tests grün (18 ak135 + 35 measure-lib, still).
- Das TODO-Register wurde von einer parallelen Sitzung aufgelöst (`bd4d123`:
  „the handover is the register, git is the history") — der Register lebt jetzt
  in den thematischen Handovers; `handover-thematisch-tiefenphasen-flotte.md`
  ist auf den Stand dieser Sitzung nachgezogen.

## 5. Register

Geschlossen: feinere Inversionsklasse, pP/sP-Polarität, die Flotte. Offen
(nachgezogen ins thematische Handover): Zonen-Flotte + Tiefherd-Erweiterung,
Streuung, Quell-Term/CMT; unverändert mitgeführt: Stationsterm, Tonga,
W-Phase-M9, Stromboli, CDN ETOPO1, MiniSEED-Dopplung, Hi-net.
