<!--
  title: Das Blatt der Korona-Heizung — die kausale DAG der solaren Kanäle (Nadel III)
  class: handover
  date: 2026-08-21
  sha256: 874f5d82a5a06b4f13eae3fa02b307ba29fe39d6c819f519072d0ff2ced2df0b
  status: live
  see-also: docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md docs/reference/broken-null-control.md docs/concepts/kybernetische-astrophysik.md
-->
# Das Blatt der Korona-Heizung — die kausale DAG der solaren Kanäle (Nadel III)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts: nur gemessene Werte —
bis dahin pending; Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **die kausale DAG der Korona-Heizung.** Die TE-Matrix über alle
solaren Kanalpaare — beide Richtungen, mit Lag, Schwelle und n — als ein
Blatt. Keine Theorie über Alfvén-Wellen oder Nanoflares; die gemessene
Informations-Richtung zwischen den Kanälen.

```
Titel: Der kausale DAG der Korona-Heizung
Paare (F10.7, XRS 1–8 Å, XRS 0.5–4 Å, EUV-304, EUV-284, Bz, Dichte):
  TE(A → B) = pending   TE(B → A) = pending   Lag = pending
  Schwelle = pending    n = pending
Verdikt je Paar: Pfeil / still / keine Aussage
```

## Das Rätsel

Die Photosphäre trägt ~6000 K, die Korona 1–2 MK — Energietransport gegen
den Temperaturgradienten. Alfvén-Wellen oder Nanoflares: unentschieden.
Die Alfvén-Laufzeit durch die Korona (~100 s) müsste als kohärenter
TE-Peak erscheinen; die Reihenfolge der Peaks (EUV vor Röntgen oder
umgekehrt) trägt die Unterscheidung. Korrelation trennt nicht —
Transferentropie trennt.

## Ist-Stand (gemessen 2026-08-21)

- **Die TE-Maschine lebt:** `topological_te_phase` (Takens dim 3, order 3),
  `te_compute` (WGSL), zehn phasenrandomisierte Surrogate (mean + 2σ),
  PE-Gate; `src/te.rs` bleibt der kanonische CPU-Referenzpfad.
- **`nobel_probe_corona`** (`src/bin`) hat gemessen — unter der
  korrigierten phasenrandomisierten Schwelle: **Bz → 304 still,
  304 → 284 still; EUV-304 → X-Ray und Bz → X-Ray tragen Pfeile
  (lag 0/1).** Vorläufig: der Befund steht unter dem Vorbehalt der
  offenen Pflichten (unten). 0 honored: Stille war die Antwort, kein Fehler.
- **`long_window_probe`** (`src/bin`): F10.7 × GOES-XRS, lag 0–7 d, beide
  Schwellen nach dem broken-null-control-Record — läuft auf lokalen Bins;
  die CDN-Assets `goes_xrs.bin` + `f107_penticton.bin` waren am 2026-08-21
  beide 404 (fehlt, nicht null — benannte Verweigerung).
- **Offene Pflichten** (TODO.md, Nadel Ⅲ): Mehrfachvergleichskorrektur
  über alle Paare und Richtungen; Lag-Sweep (lag 0 ist Default, kein
  Sweep); KDE-Bandbreiten-Sensitivität (h, h/2, 2h); Fenster-Kongruenz
  (OMNI↔GOES-Schnittmenge war leer, stopDate 06.08.); Multi-Force-TE
  pending; 90-Tage-Archive (GOES-30d: NGDC 404 → fehlt-Registratur;
  GONG steht mit 31 Jahren).
- **Laufzeit:** ~80–90 min gemessen — die O(n²) × Surrogate-Kosten vor
  jedem Lauf gegenrechnen (Desktop-Fork mit GTX 970 als Option).

## Auftrag

1. **Lang-Fenster-Lauf:** F10.7 × XRS auf den echten Serien, sobald die
   CDN-Assets leben (oder über lokale Bins) — beide Richtungen, lag 0–7 d,
   phasenrandomisierte + naive Schwelle; n < 30 → keine Aussage.
2. **Mehrfachvergleichskorrektur** über die Matrizen und Kanalpaare —
   Pflicht vor jedem Blatt; zwei Pfeile bei 20 getesteten Paaren ohne
   Korrektur verlassen den erwarteten Falsch-positiv-Bereich nicht.
3. **Lag-Sweep + KDE-Sensitivität** je Paar — schließt die registrierten
   offenen Punkte.
4. **90-Tage-Archiv-Ernte:** GOES-30d-Route erneut prüfen (NGDC 404),
   GONG-Fenster für den langen Lauf benennen.
5. **Das Blatt + Register:** Befund und TODO.md-Registerzeile im selben
   Commit; die vorläufigen Probe-Befunde wandern mit ihrem Vorbehalt auf
   das Blatt — erst der korrigierte Lauf füllt die Zellen.

## Constraints

- 0-Kanon: Ausfall = fehlt, nie 0.0 fabriziert; sfu-Konversion
  1e-22 W m⁻² Hz⁻¹; Plausibilität positiv testen.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; Lauf-Verifikation über die `φ window:`-Zeile
  (`OMEGAFLOW_HIDDEN=1 cargo run`).
- Die skalaren TE-Pfade (`src/te.rs`, `transfer_entropy_lag`) bleiben
  unberührt — die Blatt-Messung läuft auf `te_compute`.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; die drei Schichten
  (Rust → JS → WGSL) nur dann Zeile für Zeile, wenn Feldbedeutungen
  berührt sind.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, die Membran-Rendering-Physik, die
SSI-Spektral-Achse (eigene Atome), die drei Ein-Blatt-Handovers
(ENSO/Bz/LAIC — eigene Sessions), die Nadel-I/II/V-Blätter (eigene
Handovers).
