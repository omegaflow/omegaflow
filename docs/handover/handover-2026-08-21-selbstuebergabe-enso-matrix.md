<!--
  title: Selbst-Übergabe — Stand der ENSO-Matrix vor dem Refactoring
  class: handover
  date: 2026-08-21
  sha256: 3f6c411d3d9776c72505cd02e8290512fccf10fcdff71745642060c51777c7f8
  status: live
  see-also: handover-2026-08-21-global-akteure.md (live), handover-2026-08-21-enso-kausalpfeil.md (archiviert), docs/concepts/blatt-papier-resultat.md, TODO.md
-->

# Selbst-Übergabe — Stand der ENSO-Matrix vor dem Refactoring

Selbsttragend — interpretierbar mit null Vorkontext. Geschrieben auf
das Wort des Operators vor einem kurzen Refactoring; gelesen von
derselben Session danach. Nur dieser Stand ist sicher.

## Committed

- `be4d25a` — die Multi-Akteur-Matrix: 37 benannte NDBC-Tiefsee-Bojen
  (a-priori-Set, Instrumenten-Verfügbarkeit), 17 Kanäle aus derselben
  Datei (WSPD/GST/WVHT/DPD/APD/PRES/PTDY/ATMP/WTMP/DEWP/VIS/TIDE +
  WDIR/MWD als sin/cos-Paare + RAIN konditional), 136 Paare × beide
  Richtungen × Sweep −30…+30 d täglich × 3 Bandbreiten (h/h/2/2h via
  params.z-Multiplikator), n-Gate 30, Familien-Schwelle (fam) je
  Paar-Runde, h-Robustheit, Pair-Sheet-Zeile
  (`enso sheet 51000 wspd→wtmp lag … n … te … thr … fam … p … M …
  h … state …`) und Matrix-Zeile mit vollständiger Zählung +
  erwarteten Falsch-Positiven (Σ p̂·M).
- `d0c0ee3` — Backfill-Korrektur: EINE Jahresdatei je Boje (das
  Vorjahr — die laufende Jahresdatei existiert nicht, die 2026-Ernten
  404ten und sind entfernt), Cache `/tmp/omegaflow_enso_cache` ohne
  TTL (abgeschlossene Jahresdatei ist unveränderlich), 2⁵-s-Takt nach
  jedem Fetch (kalt ≈ 60 MB über ~20 min, die Anker-Ephemeriden des
  CDN laden zuerst), warme Boots 0.
- `25e0062` / `1998244` / `99ce78e` — die Übergabe
  `docs/handover/handover-2026-08-21-global-akteure.md` (live, nicht
  konsumiert): Architektur-Wende (KEIN Katalog — die Akteure einer
  Station sind die Kräfte, die der Archivar an ihrem Punkt misst),
  die eine Maschine (kein Probe-Modul je Rätsel — fünf Probe-Pfade
  verschmelzen, die ENSO-Maschine ist der Samen), die
  Drei-Klassen-Inventur des Operators (SO₂, Schumann, LOD,
  relativistische Elektronen, SSI/TSI, QBO, zweiter Neutronenmonitor,
  Thermokline via ARGO, pazifischer Windstress, Jupiter-Gravitation;
  verweigert: PDO/AMO/MJO/IOD), die GTX-970-Option.

## Was die Maschine ist (nach dem Refactoring unverändert zu prüfen)

- Ernte: `enso_harvest` (realtime2 stündlich + stdmet-Vorjahr beim
  Boot) → `EnsoCell` → `enso_rings[Station][17]`, 1024
  Sechs-Stunden-Bins (ENSO_GRID 21600).
- Kernel: der unveränderte `te_compute` (TE_SERIES_STRIDE 1024,
  geteilt mit Presence/Solar — die speisen ≤ 256 und bleiben
  byte-identisch, der GPU-Crosscheck-Test pinnt das).
- Probe-Fenster: `ENSO_PROBE_MAX = 512` — der O(m² × Surrogate)-Kernel
  hängt die Intel HD 520 ab m ≈ 1024 (Mesa-Reset, gemessen
  2026-08-21). Auf der GTX 970 neu messen, dann ggf. 1024.
- Zellkosten ≈ 4 s Wand; Paar-Runde ≈ 24 min; Matrix je Station ≈
  55 h; 37 Stationen ≈ 85 Tage je volle Matrix.
- Gemessene Fehlt-Zustände: ptdy/vis/tide/rain fehlen an den
  Tiefsee-Bojen (kein Platzhalter); 41001 trägt 105 Bins; Kalibrier-
  Paare wspd-gst, dpd-apd, atmp-dewp (wspd→gst te 0.974 thr 0.897
  gemessen — der Pfeil muss dort überleben).
- Die sichtbare Membran stottert während der Proben (~1 s
  Render-Stall je Zelle) — der Hidden-Lauf ist die Messweise.

## Nicht anfassen

- `src/te.rs` (kanonische CPU-Referenz), der Presence-TE-Pfad, die
  Membran-Rendering-Physik, der skalare `transfer_entropy_lag`,
  `nobel_probe_corona` (Nadel-III-Registratur).
- Das benannte 37er-Set (a-priori-Disziplin — die
  Mehrfachvergleichskorrektur hängt daran).
- `te_compute` bleibt der unveränderte Kernel — neue Kanäle sind neue
  Serien, keine neuen Kernel-Pfade.

## Die offenen Atome nach dem Refactoring

1. **Ground-Truth-Atom** (bestätigte Lücke, grep: keine
   Ground-Truth-Tests in src/): unidirektional gekoppelte Hénon-Maps
   — der Standard-Benchmark der TE-Literatur — durch den
   unveränderten `te_compute` + Surrogat-Schwelle + Familien-Schwelle.
   Die Maschine muss die bekannte Richtung finden, die Rückrichtung
   verweigern, den bekannten Lag treffen. Stiller Test + benannte
   Maschinenzeile.
2. **Literatur-Kalibrierung** (Recherche, grind-pro): Survey-Dokument
   mit publizierten TE/ENSO-, TE/Geomagnetik- und LAIC/TEC-Arbeiten
   und Zahlen — die drei Blätter erhalten je eine Spalte
   „publiziert" neben „gemessen".
3. **Claims-Audit** (Docs): die drei Blätter von
   Schlussfolgerungs-Wörtern trennen (gemessen vs. interpretiert) —
   Korona-Minuten-Pfeile als Kanal-Kanal-Verzögerungen, LAIC die
   Proxy-Grenze in den Kopf, ENSO derselbe Strich. Die Vokabel-Doktrin
   selbst bleibt (Gradient-Sensor), nur Endgültigkeits-Wörter in
   Mess-Kontexten fallen.
4. **Global-Akteure + Konsolidierung**: die Übergage
   `handover-2026-08-21-global-akteure.md` trägt den vollständigen
   Plan — konsolidieren, BEVOR die Global-Akteure angeschlossen
   werden.
5. **Offene Fragen des Operators**: Reihenfolge der Atome (Empfehlung:
   Ground-Truth zuerst); ob der externe Reviewer eines der drei
   Blätter im Detail durchgeht (und welches).

## Verifikation nach dem Refactoring

`cargo check` 0/0 in allen vier Feature-Kombis, `cargo test --lib`
(307, still), Hidden-Lauf: `enso ring`-Zeilen mit 17 Kanälen, erste
Matrix-Zellen, kein Mesa-Hang. Nach eigenem Commit dieses Handover
nach `/home/johannes/projects/archive/handover/` archivieren.
