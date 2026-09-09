<!--
  title: Befund — pP/sP-Tiefenphasen (ak135-S-Modell + Oberflächen-Reflexion): der Tracer trägt jetzt Vs, S und die Reflexions-Phasen — die scharfe Tiefe wird aus den Laufzeiten selbst gelesen, nicht aus Nah-Stationen
  class: befund
  date: 2026-09-09
  sha256: 888e2b4806e227c19301dbbca8fd78e8e805b9c741635c82affab22fc3812893
  status: done
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-seismische-ortung-tiefe.md docs/befund/befund-2026-09-09-feldstandard-seismik.md
-->

# Befund: die pP/sP-Tiefenphasen (ak135-S-Modell + Oberflächen-Reflexion)

## Frage & Bindung

TODO-Registerzeile „Scharfe Tiefe — pending": die Flachtiefe ist aus dem globalen
Netz schwach bestimmt (0–50 km ununterscheidbar); scharfe Tiefe braucht
Tiefenphasen (pP/sP) oder Nah-Stationen. Dieses Atom baut die Tiefenphasen als
Laufzeitkurven in `src/archivar/ak135.rs` — dieselbe kuratierte Klasse, die der
Ortung zugrunde liegt.

## Was gebaut wurde

- Das ak135-Kernel trägt jetzt Vs: `parse_nodes` liest die dritte Spalte
  (depth, vp, vs, rho) und baut ein S-Geschwindigkeitsmodell bis zur
  Kern-Mantel-Grenze (2891,5 km — der äußere Kern trägt vs = 0 und endet die
  S-Liste; S durchläuft den flüssigen Kern nicht).
- Der Tracer ist parametrisiert: `turning_radius`/`integrate`/`ray` laufen für P
  (`vp`/`grid`) und S (`vs`/`sgrid`) identisch — die S-Maschinerie erbt die
  TauP-Verifikation der P-Maschinerie.
- `s_travel(delta)` — Oberflächen-S-Laufzeit.
- `p_p_travel(delta, depth)` — P aufwärts → Oberfläche → P abwärts.
- `s_p_travel(delta, depth)` — S aufwärts → Oberfläche → P abwärts.
- Der aufsteigende Schenkel ist eine direkte Integration Quelle→Oberfläche
  (kein Umkehrpunkt): die frühere Ray-Differenz (ud−dd) trug die Auslöschung
  zweier ~600-s-Integrale; die direkte Form liest den ~3,5-s-Schenkel aus 20 km
  direkt.

## Verifikation (14 Tests, alle grün)

- P/Oberfläche und P/Tiefe regressiv gegen die TauP-Tabellen (unverändert
  bestanden).
- `p_p_travel(d, 0) == s_p_travel(d, 0) == p_travel(d)` — die Reflexion trägt
  bei Quelltiefe 0 keine Zeit.
- Leg-Identitäten exakt (1e-6): T(pP) = T(P direkt) + 2·T(aufwärts);
  T(sP) = T(P direkt) + T(P aufwärts) + T(S aufwärts).
- Physikalische Ordnung: S > P, pP > P direkt, sP > pP, Monotonie in Δ.
- Aufwärts-Schenkel deckungsgleich mit der Ray-Differenz (1e-1 s — die Differenz
  ist die Auslöschungs-Unsicherheit der Ray-Methode, gemessen).

## Benannt

- Der sP-Gültigkeitsbereich endet an der P-Kern-Grenze des Abwärts-Schenkels
  (~98°), nicht an der S-Kern-Grenze — der aufsteigende Schenkel ist ein direkter
  Weg ohne Umkehrpunkt und trägt jeden Strahlparameter bis zur Quellen-Grenze.
- Das Picken von pP/sP in `quake_location_probe.rs` ist der Nachfolger: die
  Laufzeitkurven stehen, das Lesen der Phase aus der Wellenform fehlt.

## Folge (Register)

- „Scharfe Tiefe" rückt von „Nah-Stationen ODER Tiefenphasen" auf „Tiefenphasen
  stehen, das Picken fehlt" — der Nah-Stationen-Weg (Hi-net) bleibt die
  Alternative.
