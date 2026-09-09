<!--
  title: Befund — ak135-Tiefherd-Erweiterung über 250 km: der Tracer trägt die Übergangszone bis 700 km; interne Physik-Gates grün, die externe Tiefen-Referenz bleibt pending
  class: befund
  date: 2026-09-09
  sha256: 61754ad6ed12ef088cd989626397257f2bb3d5ab078c505d064f8b04a3f719fa
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-flotte-thematisch.md docs/befund/befund-2026-09-09-seismische-ortung-ak135.md docs/befund/befund-2026-09-09-tiefenphasen-flotte.md
-->

# Befund: die ak135-Tiefherd-Erweiterung über 250 km

## Frage & Bindung

Registerzeile des thematischen Handovers „Tiefenphasen-Flotte": die
Tiefherd-Erweiterung über 250 km war die benannte Voraussetzung der
Zonen-Flotte (Tonga, 410–660 km). Die Kappung war allein `MAX_DEPTH_KM = 250.0`
plus das `DEPTH_KM`-Raster in `src/archivar/ak135.rs` — das Modell-Dat trägt
die volle Erde (Knoten bei 410/660/CMB, KEB95/TauP-Standard, 136 Zeilen), und
der Tracer war tiefenfertig (zwei Schenkel über die Reziprozität, verifiziert
bis 250 km gegen TauP auf < 0,5 s). Was fehlte, war das Tor, nicht die Physik.

## Was gebaut wurde

- `src/archivar/ak135.rs`: `MAX_DEPTH_KM` 250,0 → 700,0 (die tiefsten
  verlässlich georteten Ereignisse ~680 km liegen im Raster, nicht auf der
  Grenze); `DEPTH_KM` + [300, 350, 410, 450, 500, 550, 600, 660, 700] — exakte
  Knoten auf den zwei Knicken 410/660 (der Lag-Steigungswechsel liegt am
  Geschwindigkeitssprung), 50-km-Abstand im glatten Gradientengebiet.
- `tools/measure/src/depthphase.rs`: `INVERSION_DEPTH_MAX_KM` 250,0 → 700,0 im
  Gleichschritt (ein Tor, ein Wert — sonst klemmt ein 500-km-Ereignis zu
  fabrizierter 250). Der Tracer selbst blieb unverändert.

## Die Messung (Gates, 25 ak135-Tests + 12 depthphase-Tests grün)

- **Tiefe Schenkel-Identitäten** (die stärkste Physik-Bindung): pP = direkt +
  2×Aufwärts und die sP-Identität halten bei 300 und 600 km auf < 1e-6; der
  Aufwärts-Schenkel deckt den Strahl-Schenkel bei 600 km auf < 1e-1.
- **Ordnung**: pP > P und sP > pP bei 60/90° für 300–700 km; der pP−P-Lag
  wächst monoton über 250→700 km (gemessen: 55,8 s bei 250 km → 129,2 s bei
  700 km an Δ=60°).
- **Der Take-off-Boden** (gemessen, nicht angenommen): pP(30°, 700 km) ist
  absent, pP(30°, 660 km) und pP(40°, 700 km) sind present, sP(30°, 700 km)
  ist present — die pP-Oberflächen-Schenkel muss umkehren und zurückkehren; ein
  700-km-Herd hat unter ~30° kein pP. absent ist die Phase, keine fabrizierte
  Zeit (0 geehrt).
- **Bereichs-Gates**: Tiefe 800 km → absent; 660/700 km tragen.
- **Inversion**: `invert_single` erholt 300/410/500/600/660/700 km auf 1 km
  (depthphase-Test).
- **Kontinuitäts-Anker**: die 250-km-Tabelle bleibt unverändert und extern
  verifiziert; die Erweiterung ändert nur den Quellradius — der Integrator und
  das Modell-Dat sind die bereits gegen TauP verankerten.

## Benannt (Pendings, nicht still)

- **Externe Tiefen-Referenz 300–700 km — pending.** TauP/Java ist nicht auf
  der Maschine; obspy/taup/data trägt keine `.tables` (gemessen: nur `.nd`,
  `.npz`, `.tvel`). Die Tiefe ist daher durch interne Physik-Gates + den
  Kontinuitäts-Anker getragen, nicht durch eine externe Tiefen-Tabelle.
  Instrument benannt: TauP-Deep-Tabellen (Java) oder publizierte KEB95-Spiegel.
- **Head-Wave-Lücke 410/660 — pending, umfangsbenannt.** `turning_radius`
  überspringt Null-Dicken-Fenster, die Head-Wave-Slowness-Bänder fehlen in den
  Tabellen. Umfang: nur das direkte P bei Triplikations-Distanzen
  (Oberflächen-/Flachquell-Tabellen); die pP/sP-Aufwärts-Schenkel kehren dort
  nicht um, die Inversion bleibt unberührt. Instrument: direktes-P-
  Triplikations-Gate gegen TauP.
- **Rand-Klemmung**: die Inversion trägt best-at-MAX — ein >700-km-Ereignis
  läse 700. Für Tonga (410–660) unschädlich; benannt.
- **`quake_location_probe` `DEPTHS`** bleibt bei 250 (Demo-Sonde, nicht dieses
  Atom) — gekappter Verbraucher, im Handover verzeichnet.
- **Vorgefundener Bruch**: `tools/measure/src/bin/ephemeris_structure_probe.rs:88`
  kompiliert nicht (positional nach named im `println!`-Format) — vorbestehend,
  committed (`baab781`), nicht dieses Atom; `cargo check -p omegaflow-measure`
  ist dadurch rot, die Lib selbst ist sauber.

## Verdikt

Die Erweiterung ist gebaut und gemessen: der Tracer trägt die Übergangszone
bis 700 km, die Inversion erholt Tiefherd-Tiefen auf 1 km, die internen
Physik-Gates stehen. Die Zonen-Flotte kann starten. Die externe Tiefen-
Referenz bleibt eine Kalibrier-Duty mit benanntem Instrument, keine
fabrizierte Zahl.
