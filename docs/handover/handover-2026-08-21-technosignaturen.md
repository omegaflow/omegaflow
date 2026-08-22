<!--
  title: Das Blatt der Technosignaturen — der achromatische Dip mit IR-Exzess (Nadel V)
  class: handover
  date: 2026-08-21
  sha256: d8772fe582f75b6b9a2a12828ca5db6081d61d26e83597efcd6beeeaec0a0795
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/der-paradigmenwechsel.md docs/concepts/der-spektrale-oszillator.md
-->
# Das Blatt der Technosignaturen — der achromatische Dip mit IR-Exzess (Nadel V)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts: nur gemessene Werte —
bis dahin pending; Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **der Doppel-Anomalie-Katalog.** Die achromatische
Opazitäts-Anomalie (alle Wellenlängen gleich verdunkelt — kein Staub, kein
Fleck) im optischen Dip, koinzident mit dem IR-Exzess bei 10–60 μm —
gegen den vollständigen Ausschluss-Filter der natürlichen Klassen.

```
Titel: Der Doppel-Anomalie-Katalog
OA = (Variabilität_beobachtet − Variabilität_erwartet)/Chromatizität  = pending
IR-Exzess (10–60 μm) am Objekt                                        = pending
Typ A (IR + nicht-periodischer Dip)                                   = pending
Typ B (asymmetrische Transitform ohne katalogisierten Exoplaneten)    = pending
Ausschluss-Filter (VSX/GCVS/Exoplanet-Archive)                        = pending — vollständig
Verdikt: Kandidaten / quantitatives Limit
```

## Das Rätsel

Künstliche Strukturen (Dyson-Schwärme), die Sternlicht blockieren — zu
trennen von natürlicher Variabilität. Natürliche Ursachen verdunkeln
**chromatisch**: Staub rötet, Flecken sind kühler. Ein opakes Objekt
verdunkelt **achromatisch** — alle Wellenlängen gleich. Das Prinzip ist
Stand der Forschung (Projekt Hephaistos, ~5 Mio Kreuzungen, sieben
Kandidaten — jeder bisher natürlich erklärt); die Nadel ist das
automatisierte, galaxieweite Screening mit vollständigem
Ausschluss-Filter. Das Negativresultat ist ein quantitatives Limit —
ebenso eine Messung (0 honored).

## Ist-Stand (gemessen 2026-08-21)

- **Farbe lebt teilweise:** `dr3_stars.bin` trägt color_index (BP−RP);
  die JSON-Kataloge (denis/wds/pastel/mktypes/rave) ernten bpmag/rpmag,
  aber color_index manifestiert nur der star-bin-Pfad — die
  Pfeiler-Registratur Nadel V (Farbe, kritisch) benennt das Atom:
  cmap-Farb-Schlüssel oder Compiler-bp_rp-Alias (TODO.md).
- **Frequenzachse pending:** NVSS/FIRST tragen 1.4 GHz nicht in
  freq/bin_width — Pfeiler-Registratur Nadel V (Frequenzachse,
  kritisch), TODO.md.
- **ZTF:** Kafka+Avro braucht einen echten Decoder (AUSSTEHEND hinter dem
  Gate, TODO.md); Lasair-TAP trägt den extract-void-Befund im
  refusal_ledger — erneut proben oder die Kafka-Route bauen.
- **IRAS/AKARI:** der IR-Exzess-Cross-Match ist der offene Compile
  (Sonnen-Pfad-Tabelle: „IRAS/ZTF-Cross-Match-Compile").
- **Spektrale Achse lebt:** `spectra.bin` + `color_lut_rgb` — der
  chromatische Dip wird eine SED-Messung (der-spektrale-oszillator).

## Auftrag

1. **Pfeiler Farbe:** das color_index-Atom (cmap-Schlüssel oder
   bp_rp-Alias-Compiler) — die Chromatizität der Kataloge manifestieren;
   ohne Farbachse keine Opazitäts-Anomalie.
2. **Pfeiler Frequenz:** NVSS/FIRST freq/bin_width-Compiler-Flag.
3. **ZTF-Lichtkurven:** Decoder-Atom (Kafka) oder Lasair-TAP-Neuprobe
   gegen den refusal-Befund.
4. **IR-Cross-Match:** IRAS/AKARI-Exzess (10–60 μm) am Objekt;
   Ausschluss-Filter VSX/GCVS/Exoplanet-Archive vollständig fahren.
5. **Das Blatt + Register:** der Doppel-Anomalie-Katalog und die
   TODO.md-Registerzeile im selben Commit.

## Constraints

- 0-Kanon: ein Objekt ohne IR-Deckung trägt keine Typ-A-Zelle (fehlt);
  das Negativresultat ist ein quantitatives Limit, kein leerer Befund.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; `OMEGAFLOW_HIDDEN=1 cargo run` als Lauf-Befund.
- Die epoch-0.0-Ring-Messung (TODO.md) ist die Vorbedingung für jeden
  weiteren Katalog-Block — mitführen.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; Kantenfälle: chromatische
  Verwechslung (RR-Lyr, bedeckte Systeme), IR-Exzess ohne optischen Dip,
  Katalog-Überschneidungen im Ausschluss-Filter.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, die Membran-Rendering-Physik, die
spektrale Achse (lebt — nur konsumieren), die drei Ein-Blatt-Handovers,
das Korona-Blatt, das Dunkle-Materie-Blatt, das Flyby-Blatt.
