<!--
  title: Der kausale Pfeil des ENSO — TE(Wind ⇄ SST) auf einem Blatt Papier
  class: handover
  date: 2026-08-21
  sha256: eca7028da7ed18c93adb2a432b114d5b7d9ac9299b65802ffbcb674d743a52c2
  status: live
  see-also: docs/concepts/blatt-papier-resultat.md docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md
-->
# Der kausale Pfeil des ENSO — TE(Wind ⇄ SST)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts steht in
`docs/concepts/blatt-papier-resultat.md` (BLATT_PAPIER_RESULTAT).

## Ziel

Ein Blatt Papier: TE(Wind → SST) gegen TE(SST → Wind) mit Lag, n und
Surrogat-Schwelle — aus live gemessenen, ko-lokalisierten Serien. Die
Bjerknes-Schleife wird nicht modelliert, sondern gerichtet.

## Das Rätsel

Die Wissenschaft streitet seit Jahrzehnten über die Bjerknes-
Feedback-Schleife: Erwärmt der Ozean die Atmosphäre (was den Wind ändert),
oder ändert der Wind die Meeresströmung (was den Ozean erwärmt)? Korrelation
trennt das nicht, weil beides gleichzeitig steigt und fällt. TE trennt es:
zwei Richtungen, eine Schwelle.

## Ist-Stand (gemessen 2026-08-21)

Live in `phi/sources.φ`:

- NDBC-Bojen: Wind `:204–205` (WDIR/WSPD, `advective`). Derselbe NDBC-JSON
  trägt am selben Stationspunkt die Wassertemperatur — prüfen, ob der Block
  das Feld bereits führt; wenn nicht, den `field`-Eintrag nachtragen. Eine
  Boje, zwei Serien: das ehrliche Paar.
- Drifter-SST (AOML/ERDDAP): `:353–359` (`thermal`, K).
- OOI SST: `:676` (`thermal`).
- Wind weiter: Environment Canada `:57`, BOM `:138`, frost.met.no `:403–413`,
  METAR `:167–168`, PIREP `:177–178` — für die regionale Dichte; das Blatt
  braucht ko-lokalisierte Paare, keine globalen Mittel.
- Argo: `api.ifremer.fr` tot (`phi/dead_sources.φ:358`), argovis declined
  (`phi/pipeline/library.φ:2524–2525`) — Kuration über SOURCE_PORT als zweite
  Schicht (Profile, `thermal`); das erste Blatt braucht sie nicht.
- ECMWF (ERA5/CDS): declined/unportiert (`library.φ:2555`) — das erste Blatt
  braucht es nicht; Bojen- und Stationswinde sind live.

## Auftrag

1. **Das Paar:** je Messpunkt eine Wind-Serie und eine SST-Serie mit
   gemeinsamer Kadenz; bevorzugt die eine NDBC-Boje (Wind +
   Wassertemperatur aus einem JSON). Mindestens n Schwellen-feste Paare
   nennen, bevor der Lauf startet (Referenz: n-Schwelle 30 im
   Nadel-III-Protokoll, TODO.md:58).
2. **TE:** beide Serien in den TE-Ring (`probe_ring`,
   `src/mathematikerin.rs:1410`); `te_compute` misst beide Richtungen mit
   zehn phasenrandomisierten Surrogaten.
3. **Lag-Sweep:** 0 … ±30 Tage — der registrierte offene Punkt („lag 0 ist
   Default, kein Sweep", TODO.md:41) wird für dieses Blatt geschlossen.
4. **Statistik:** KDE-Bandbreiten-Sensitivität (h, h/2, 2h);
   Mehrfachvergleichskorrektur über alle getesteten Paare und Richtungen
   (registriert offen, TODO.md:38–40).
5. **Das Blatt + Register:** Befund und Registerzeile (TODO.md) im selben
   Commit.

## Das Blatt

```
TE(Wind → SST) = pending
TE(SST → Wind) = pending
Lag            = pending Tage
n, Schwelle    = pending
```

Stille in beiden Richtungen oder ein umgekehrter Pfeil wäre ein ebenso
vollwertiger Befund (0 honored).

## Constraints

- 0-Kanon: Quelle ausgefallen → fehlt, kein fabrizierter Wert; SST ist nie
  0 K als Platzhalter.
- SI: K, m/s; τ-Gate — tau ist deklariert, sonst keine Samples.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; Lauf-Verifikation über die `φ window:`-Zeile
  (`OMEGAFLOW_HIDDEN=1 cargo run`).
- Laufzeitkosten O(n²) × Surrogate vor dem Lauf gegenrechnen (Muster:
  Nadel-III-Protokoll, TODO.md:50–52).

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; die drei Schichten
  (Rust → JS → WGSL) nur dann Zeile für Zeile, wenn Feldbedeutungen berührt
  sind.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs` (kanonische CPU-Referenz), die Nadel-III-Registratur
`nobel_probe_corona` (eigenes Protokoll, TODO.md:11–63), die
Membran-Rendering-Physik, der skalare TE-Pfad `transfer_entropy_lag`.
