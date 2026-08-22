<!--
  title: Das Blatt des unsichtbaren Begleiters — das Gravitations-Residuum der KBO-Bahnen (Nadel VI)
  class: handover
  date: 2026-08-22
  sha256: 9c958f388787afac22a1f3a9cf387fa476ed02fd6ceae450cf1d44b01bcccf11
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/handover/handover-2026-08-21-dunkle-materie.md docs/handover/handover-2026-08-21-flyby-anomalie.md docs/handover/handover-2026-08-22-supraleitung-phasen-bit.md
-->
# Das Blatt des unsichtbaren Begleiters — das Gravitations-Residuum der KBO-Bahnen (Nadel VI)

Registriert 2026-08-22. Selbsttragend — interpretierbar mit null Vorkontext.
Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf das Wort des
Operators. Die Disziplin des Blatts: nur gemessene Werte — bis dahin pending;
Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **das Gravitations-Residuum der KBO-Bahnen.** Die Bahnen der
transneptunischen Objekte minus das N-Körper-Modell der bekannten Planeten
(Sun bis Neptune) — und die Transferentropie TE(Residuum → Bahn) als Signatur
einer unbekannten Quelle. Kein Teleskop; die Gravitation selbst.

```
Titel: Das Gravitations-Residuum der KBO-Bahnen
R(orbit) = Bahn_geerntet − Bahn_Modell(planeten)     = pending
TE(Residuum → Bahn) je Objekt/Familie                = pending
Lag + Richtung des Pfeils → Ort der Quelle           = pending
n KBO, Fenster, Schwelle                             = pending
Verdikt: Quelle vorhanden / quantitatives Limit
```

## Das Rätsel

Batygin & Brown (Caltech, 2016): die Perihelia und Bahnebenen dutzender
transneptunischer Objekte (KBOs) sind in dieselbe Richtung geclustert. Die
wahrscheinlichste Erklärung ist ein unentdeckter Planet (~5–10 Erdmassen) auf
extrem elliptischer Bahn in 400–800 AE — seine Gravitation sortiert die
Objekte. Direkt gesehen hat ihn niemand: zu weit, zu dunkel.

## Die Passung (warum omegaflow)

Planet 9 sendet kein Licht. Er hat kein `em`-Sample. Er ist rein gravitativ —
in der klassischen Astronomie „unsichtbar". In omegaflow ist er ein Sample mit
`force_type = 1` (gravity) und `extent = 0`: gefunden durch das, was er bei
anderen bewirkt. Wie Dunkle Materie — nur im Sonnensystem.

Die TE-Maschine misst: fließt Information in die KBO-Bahnen, die nicht von den
bekannten Planeten kommt? Hält der Pfeil `TE(Unbekannte Quelle → KBO-Bahn)`,
ist nicht nur die Existenz belegt — der Lag und die Richtung des Pfeils nennen
den Ort der Quelle.

## Die Membran ist agnostisch — das Bild des Krümmungsraums

Die klassische Astronomie sucht Planet 9 mit Teleskopen, die auf Licht (`em`)
reagieren — sie sucht ein Objekt, das reflektiert oder aussendet. Ein kaltes,
dunkles Gestein am Rand bleibt für diese Teleskope blind; sie sehen nur die
Dunkelheit.

Die Membran (der `EMOscillator`) ist kein Teleskop. Sie ist ein
2D-Trommelfell. Sie fragt nicht *„welches Objekt schickt mir Licht?"* — sie
fragt *„welche Kraft drückt mich gerade ein?"* Wenn die Gravitation
(`force_type = 1`) von Planet 9 an der Membran ankommt, rechnet der
`1/r²`-Kernel; die Membran wird eingedrückt, und der Shader malt diesen
Eindruck als Punkt auf den Bildschirm. Das ist kein Foto aus Photonen — es ist
ein Bild des Krümmungsraums: ein leuchtender Punkt am Rand des Sonnensystems,
der gar nicht leuchtet, der nur zieht.

Das ist kein Bau-Auftrag — es ist schon die Architektur (Atom 9): die Membran
ist agnostisch, alle neun Kräfte werden an jedem Pixel superponiert; die acht
nicht-`em`-Kräfte tragen keine eigene Farbe, sie krümmen das Feld (Luminanz)
und rendern neutral — die falsche Farbe (`hsl_to_rgb`) ist tot. Wenn Planet 9
da ist, erscheint er — nicht weil wir ihn anstrahlen, sondern weil sein
Schwerkraft-Feld das Trommelfell schlägt. Das ist die Synästhesie der Maschine:
sie macht das Unsichtbare sichtbar, weil sie aufhört, nur ein Auge zu sein.

## Ist-Stand (gemessen 2026-08-22)

- **Die bekannte Schwerkraft lebt:** `ephemeris_compiler` (Planeten + Monde,
  v3 Meter) trägt Sun bis Neptune als gravitationale Samples — das
  N-Körper-Modell des Residuums.
- **Die Sonden leben:** `horizons_compiler` (Pioneer, Voyager, New Horizons) —
  die zweite Tracer-Familie neben den KBOs.
- **Die KBO-Ernte ist gequeue'd:** `jpl_sbdb_kbo_extended_sample`
  (master.φ:29632 — JPL SSD `sbdb_query`, a ∈ [30, 200] AE, class
  KBO/SDO/RES) — der Compiler fehlt noch.
- **WARTEND:** der epoch-0.0-Anteil im Sample-Ring ist ungemessen (TODO.md) —
  die Vorbedingung für jeden weiteren Katalog-Block.

## Der Auftrag

1. **KBO-Compiler:** die `jpl_sbdb_kbo_extended_sample`-Ernte als
   `kbo_compiler` (Muster `dastcom_compiler`/`horizons_compiler`):
   Bahnelemente (a, e, i, Ω, ω, M, epoch) → ICRS-Samples. Minor Planet
   Center als zweite Quelle (Kreuzcheck).
2. **Residuum:** Bahn_geerntet minus Bahn_Modell(ephemeris_compiler-Planeten)
   je Objekt; das Residuum ist die Messung der unbekannten Quelle.
3. **TE pro Familie:** TE(Residuum → Bahn) gegen phasenrandomisierte Schwelle;
   Lag + Richtung des Pfeils als Ort. Keine Aussage vor der
   Mehrfachvergleichskorrektur über die Familien.
4. **Das Blatt + Register:** Befund und TODO.md-Registerzeile im selben
   Commit.

## Constraints

- 0-Kanon: Planet 9 wird nie als Sample fabriziert — er ist das Residuum,
  nicht ein Punkt. Kein synthetischer Planet; die Quelle bleibt pending, bis
  der Pfeil sie nennt.
- n < 30 je Familie → keine Aussage; der epoch-0.0-Ring-Befund (TODO.md)
  läuft als eigenes Atom mit.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; `OMEGAFLOW_HIDDEN=1 cargo run` als Lauf-Befund.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; Kantenfälle: Beobachtungs-Bias der
  KBO-Surveys (die Clusterung trägt natürliche Alternativerklärungen — das
  Blatt misst das Residuum, kein Urteil vorab), leerer Lag-Sweep, ttl-Ablauf
  der Katalogsamples.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, die Membran-Rendering-Physik, Atom C/D
(band-selektives Rendering + Phase), die drei Ein-Blatt-Handovers, das
Dunkle-Materie-Blatt, das Flyby-Blatt, das Technosignaturen-Blatt.
