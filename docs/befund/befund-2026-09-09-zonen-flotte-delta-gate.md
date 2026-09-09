<!--
  title: Befund — der pP-Δ-Zweig: die Faltung gemessen und das Δ-Restriktions-Gate gebaut (Stationen-Streuung 108→88,6 km)
  class: befund
  date: 2026-09-09
  sha256: d3e104a632fae7cff9193aed794f631ea6a3555b2c01e2300de49f6f951debd9
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-diagonale.md
-->
# Befund: der pP-Δ-Zweig — Faltung gemessen, Δ-Restriktions-Gate gebaut

## Frage & Bindung

Registerzeile „Streuung senken — der mehrdeutige pP-Zweig bei Δ≈30°; an der tiefen Geometrie der dominante Hebel (√N trägt langsam)." Der Council entschied nach der Messung der Ursache: bauen, nicht pending tragen — die Maschinerie existiert bereits.

## Was gemessen wurde (read-only, kein Edit)

Die eigene ak135-pP-Tabelle (`src/archivar/ak135.rs`) ist in Δ-Bändern mehrdeutig: die pP-Familie faltet (mehrere Ankünfte je Δ), und der Ein-Ankunft-Zugriff `interp_delta` (ak135.rs:337) kollabiert sie auf einen Wert — 2–6 s Sprung über ~0,1° Stationsabstand. Gemessene Faltungsbänder: 410 km Δ30–33, 450 Δ30–34, 500 Δ32–35, 550 Δ35–37, 600 Δ37–39; 660 km trägt ≤46° keine Faltung. Die Tiefenrichtung bei festem Δ bleibt strikt injektiv — die Mehrdeutigkeit ist Ankunfts-Multiplizität, nicht zwei Tiefen mit einem Lag.

## Was gebaut wurde

`delta_branch` (tools/measure/src/depthphase.rs): vor der pP-Korrelation wertet `measure_station` den code-gelesenen Lag beidseitig des Stations-Δ aus — das Maximum von `|p_p_lag(Δ±δ, h_ref) − p_p_lag(Δ∓δ, h_ref)|/(2δ)` über vier Halbweiten (0,25/0,5/0,75/1,0°; ein einziger 0,75°-Abgriff taucht in der Faltung auf 0,27 s/deg — gemessen —, eine einzige Weite verfehlt also die Spitzen). Die Schwelle ist der **gemessene glatte teleseismische Gradient** (Median derselben Metrik über das Ein-Zweig-Band Δ40–90° bei derselben Tiefe: 0,13→0,66 s/deg) × 3,0 — der Faktor liegt im gemessenen Spalt (schwächstes Faltungsboden-Verhältnis ≈ 3,6). Überschreitet die Station die Schwelle, wird sie als benannter Skip geführt (`"pP Δ-branch fold (branch-unstable)"`), nie als fabrizierte Zeit, nie 0.

Kalibrier-Gate (te.rs-Disziplin): FP — Δ31/32/33@410, Δ32/33@450, Δ34@500, Δ35@550, Δ37@600 müssen `unstable` sein; FN — Δ45@410–600, Δ60@450/600 müssen stabil sein; die 660-Geometrie (Δ30–46) wird nie geflaggt; die Tiefenrichtung bleibt injektiv (Δ40–60 nie geflaggt).

## Die Messung (Fleet-Neulauf, beide Gates aktiv)

Das Δ-Gate übersprang 64 Stationen flottenweit; 13 von 16 Ereignissen trugen eine Inversion, 3 Ereignisse fielen vollständig in die Faltung → `pending` (abwesend, nicht 0). Die Stationen-Streuung (Median der In-Ereignis-σ) sank von 108,1 km auf **88,6 km**. Beide Mittelwerte als benannte Größen: mit Klemmung **−14,6 km** (σ 72,5, se 20,1), nach Exklusion **−61,8 km** (σ 61,6, se 17,1) über 13 Ereignisse.

## Die Befunde

1. **Das Δ-Gate greift den ursprünglichen Engpass an** und senkt die Streuung messbar (108→88,6 km) — es entfernt genau die gemessene Faltungs-Domäne und behält die Ein-Zweig-Geometrie (660 km, alle Δ≥40°).
2. **Die Rest-Streuung (88,6 km) ist Pick-Rauschen**, nicht Faltung: die Lag-Tiefen-Steigung ist flach (0,16–0,41 s je 5 km im Band), ein Sekunden-Pickfehler wird 12–32 km Tiefe — der nächste Hebel, √N trägt langsam.
3. **Das Gate ist orthogonal zum 660-Gate:** jenes benennt die Wand, dieses schrumpft die Streuung; beide bleiben.

## Verdikt

Die Faltung ist gemessen, das Instrument gebaut und grün (Kalibrier-Gate, `cargo check` 0/0). Die Streuung sank 108→88,6 km — gemessen, nicht versprochen; die verbleibende Streuung ist Pick-Rauschen und bleibt als nächster Hebel benannt.
