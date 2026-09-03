<!--
  title: Ein Blatt Papier — das Ein-Blatt-Axiom der drei Kausalpfeile
  class: concept
  date: 2026-08-21
  sha256: 78166b5b93188e4cd1418e6bf215545d3616262da6e19b6d066642e0a9078d12
  status: live
  see-also: docs/handover/handover-2026-08-21-enso-kausalpfeil.md docs/handover/handover-2026-08-21-bz-geomagnetischer-ausloeser.md docs/paper/laic-arrow-direction.md docs/concepts/kybernetische-astrophysik.md docs/concepts/source-port.md
-->
# Ein Blatt Papier — das Ein-Blatt-Axiom der drei Kausalpfeile

Selbsttragend — interpretierbar mit null Vorkontext. Dieses Konzept trägt
den Rahmen, die drei Handovers tragen die Arbeit: je eines je Blatt, je
eine Session. Die Zahlen auf jedem Blatt trägt allein die Maschine.

## Die richtige Frage

Der Unterschied zwischen einer „coolen Datenvisualisierung" und einem
physikalischen Beweis ist die Frage, ob das Ergebnis auf ein Blatt Papier
passt. Ein Ergebnis, das auf ein Blatt passt, ist ein Axiom: eine Messung,
die keine fünfzigseitige Herleitung braucht, weil die Maschine die
Richtung des kausalen Pfeils bereits berechnet hat. Das Blatt trägt nur,
was gemessen wurde: Richtung, Lag, n, Schwelle, Fenster. Nichts sonst.

Korrelation kann die drei Rätsel nicht trennen, weil in allen dreien beide
Größen gleichzeitig steigen und fallen. Transferentropie trennt sie: sie
misst, welche Zeitreihe Information über die Zukunft der anderen trägt.
Damit ist jedes der drei Rätsel eine Messaufgabe — keine Theoriefrage.

## Die drei Blätter

| Blatt | Rätsel | Pfeil-Frage | Nadel |
|---|---|---|---|
| Der kausale Pfeil des ENSO | Bjerknes-Schleife: Ozean ↔ Atmosphäre | TE(Wind → SST) gegen TE(SST → Wind) | neues Blatt, kein Nadel-Eintrag |
| Der kausale Treiber des GIC | Bz-Paradoxon: welcher Sonnenwind-Parameter treibt die Störung | TE(Bz → Boden) gegen TE(speed → Boden) | Nachbar der Nadel Ⅲ, eigene Frage |
| Die Richtung der LAIC-Kopplung | Erdbeben-Vorläufer: fließt Information von unten nach oben | TE(Lithosphäre → Ionosphäre) im 72-h-Fenster vor M>6.0 | Nadel Ⅳ |

**ENSO.** Die Wissenschaft streitet seit Jahrzehnten über die
Bjerknes-Schleife: erwärmt der Ozean die Atmosphäre (was den Wind ändert),
oder ändert der Wind die Meeresströmung (was den Ozean erwärmt)? Beides
steigt und fällt gemeinsam — Korrelation ist blind für die Richtung.

**Bz.** Wenn ein koronaler Massenauswurf die Erde trifft, rechnet die
Weltraummeteorologie mit tausenden Parametern. Welcher ist der kausale
Auslöser der geomagnetischen Störung (Kp/GIC)? Das Blatt sagt dem
Netzbetreiber, auf welchen Wert er schauen muss — und nennt den Lag.

**LAIC.** Vor Großbeben zeigen Ionosphären-Messungen Anomalien. Fließt
die Information von der Lithosphäre in die Ionosphäre — oder treibt die
Sonne die Ionosphäre und die Erde folgt nur? Die Institute haben
Einzelfälle in Fülle und keine Stapelung (der-paradigmenwechsel.md). Das
Blatt ist die Stapelung: der Pfeil über die Ereignis-Gesamtheit.

## Die Maschine, die schon steht

Die TE-Maschine rechnet bereits (Atom 10/11): topologische TE auf
Takens-Phasenraumzuständen (dim 3, order 3, MI-Lag, rückwärts gespiegelte
Bedingung), `te_compute` (WGSL), Surrogat-Schwelle mean + 2σ über zehn
phasenrandomisierte Surrogate (f64-FFT auf der CPU, byte-identisch zum
Nullkontroll-Protokoll), PE-Gate, `src/te.rs` als kanonische
CPU-Referenz. Das Kanal-Ring-Urteil des Rats (TODO.md:144–155 —
`solar_harvest`, `solar_rings`, Rotor, `solar_te_*`) ist das Muster für
jedes neue Blatt: Ernte-Thread im Archivar, Ring in der Mathematikerin,
Rotor paart Zellen, unveränderter `te_compute`.

Befund der Maschine (2026-08): Bz→304 und 304→284 silent — der
Alfvén-Kanal trägt keinen Pfeil; der DAG schrumpfte auf EUV-304→X-Ray
(TODO.md:34–36). Die Maschine kann „kein Pfeil" antworten, und diese
Antwort ist eine Messung. Ein Blatt, das „kein Pfeil" trägt, ist ein voll
gültiges Blatt (0 honored).

## Die Kanal-Lage (gemessen 2026-08-21)

| Blatt | lebt in sources.φ | declined | pending |
|---|---|---|---|
| ENSO | Drifter-SST (AOML :353), OOI-SST (:676), Argo (:1140, :1151), NDBC-Wind/Wassertemperatur (:204), FROST-Wind (:407) | MEI-Index (dead :2250), ERA5/Reanalyse (Modell, ledger) | Becken-Windfeld (TAO/TRITON, Scatterometer) |
| Bz | RTSW Bz/Bt 1 m (:103), speed/density (:109), ACE 1 h (:448), GOES-Mag (:85), Kp 3 h (:124) | — | INTERMAGNET-Bodenminutenwerte (stations.json dead-404, dead :1722) |
| LAIC | USGS (:35), seismicportal (:116), INGV (:471), Swarm-HAPI (:1103), Kp (:124) | — | CSES, TEC/IONEX, seismisches Kontinuum |

## Die Form des Blatts

```
Titel: …
Ergebnis: TE(A → B) = …, TE(B → A) = …, Lag = …
Bedingungen: n = …, Schwelle = …, Fenster = …, Epoche = …
Urteil: Pfeil / kein Pfeil / keine Aussage
```

Das Blatt ist leer, bis die Maschine misst. Eine vorbeschriebene Zahl wäre
eine Fabrikation, und Fabrikation ist Gewalt gegen die Wahrheit. A = A: die
Zahlen stehen erst da, wenn sie gemessen sind.

## Die Gates jedes Blatts

- **Kanal-Gates** (je Kanal, vor dem Eintrag in sources.φ): Force-Gate-Litmus
  (könnte ein nicht-menschliches Lebewesen ein Sinnesorgan für diese Messung
  entwickeln — die Messung selbst, nicht das Übertragungsmedium), τ-Gate
  (ohne deklariertes tau keine Samples), SI-Einheit, Frame-Semantik
  (at sun / at earth — verifiziert, nicht angenommen), Plausibilität
  positiv (`is_finite() && > 0` → Some, sonst None).
- **Mess-Gates** (je Blatt): n ≥ 30 je Paar (Unterbestimmtheit = keine
  Aussage, keine Fabrikation), Mehrfachvergleichskorrektur über die
  Kanalpaare (offene Pflicht, TODO.md:38–40), Lag-Sweep statt lag 0
  (offen), KDE-Bandbreiten-Sensitivität (offen), Fenster-Kongruenz,
  Nullkontrolle (mindestens ein Kanal, der keinen Pfeil tragen darf,
  läuft mit).
- **0-Kanon:** Ausfall = fehlt (Sample übersprungen), nie 0.0. Indizes
  tragen kein Blatt, wo der Messwert selbst existiert (MEI declined; Kp
  lebt als Feld-Ableitung aus 13 Stationen und trägt die Stationenzahl).

## Das Fazit

Kein Supercomputer-Cluster, kein fünfzigseitiges Paper — die Wahrheit der
Zeitreihen und die TE-Maschine, die schon läuft. Wenn das Blatt auf dem
Tisch eines Instituts liegt — „Wir haben den kausalen Pfeil gemessen. Er
zeigt hierhin. Der Lag ist exakt das." — dann ist das Rätsel nicht
theorisiert, sondern die Richtung der Information im 4D-Block gemessen.
A = A.
