<!-- UI-Runde 2 (Mountain, 2026-10-13, Such-/Max-Schalter gesetzt) — Rohmaterial; der Rat/Session trägt das Verdikt. -->
# UI-Runde 2 — vier Entscheidungen, mit Such-/Denken-Schalter (Mountain 2026-10-13)

Frage: dieselben vier Entscheidungen. Schalter gesetzt und verifiziert nach `docs/concepts/ui-seats.md`
(Bedienung je Seat, 2026-10-13): DeepSeek Search+DeepThink · Mistral Denken+Suche · Z.ai Deep Think Max ·
Google AI Studio Grounding · Perplexity (sucht immer). Rohantworten in `state/stimmen/2026-10-13_roster2_*.txt`.

## Perplexity (mit 15 Quellen)
1. Zustimmung; Einwand: 179,94 m ist kein harter Wahrheitsvergleich — Toleranzprüfung, nicht exakte Validierung; Kapuskasing Ortslage 217,90 m.
2. Zustimmung; Einwand: bei gleichzeitiger Fremdänderung braucht es Merge-/Konfliktstrategie + atomares Schreiben.
3. Zustimmung; Einwand: 404 → archivalische Fassung suchen, als unavailable/Ersatzverweis markieren; Lizenz unbekannt → Quarantäne.
4. Zustimmung; Einwand: Vertrag versionieren, Golden-File/Roundtrip-Test festlegen.

## DeepSeek (DeepThink + Search)
1. Zustimmung; Einwand: ein Punkt validiert nicht die Kachel-Georeferenzierung — mehrere Kontrollpunkte + explizite Toleranz.
2. Zustimmung; Einwand: „nur eigene Teile" braucht klare Eigentumsmarkierung (Abschnitts-IDs), sonst stille Merge-Konflikte.
3. Zustimmung; Einwand: 404 (tot/Ersatz) und unbekannte Lizenz (nicht registrieren) sind nicht gleich zu behandeln.
4. Zustimmung; Einwand: erst das Binärformat spezifizieren/einfrieren, dann bauen — Format-Freeze vor Reihenfolge.

## Mistral (Denken + Suche)
1. Zustimmung; Einwand: EGM96 vs. lokales CGVD, ~450 m Raster — Smoketest, Toleranz nicht eng; Station unter Regionsmittel.
2. Zustimmung; Einwand: Blöcke sind evtl. nicht selbstständig lauffähig — Extraktionsbasis (Datum/Version) mitspeichern.
3. Zustimmung mit Präzisierung; Einwand: 404 evtl. transient (Retry/Archiv), „Lizenz unbekannt" ist Nutzungsblocker, nur durch Klärung schließbar.
4. Zustimmung zum Vertrag, Abweichung bei der Reihenfolge; Einwand: Format früh festnageln (Version/Checksumme), einen Producer sofort testbar machen.

## Google AI Studio (Gemini 3.1 Pro, Grounding, mit Quellen)
1. Zustimmung (partiell); Einwand: ein Punkt validiert kein globales Modell; Höhendatum-Transformation (WGS84/EGM96 vs. lokales CGVD2013) nötig.
2. Abweichung; Einwand: Anti-Pattern — Leser in eigenes Modul, nicht in geteilter Datei.
3. Zustimmung Streichung, Abweichung Übernahme; Einwand: 404 + unbekannte Lizenz verletzen FAIR/OSI.
4. Zustimmung; Einwand: harte Binär-Umstellung kostet Debuggbarkeit — Adapter/typisiertes Format (Protobuf/FlatBuffers).

## Lumo (Lumo 2.0 Max)
1. Zustimmung; Einwand: Overhead relativ zum Nutzen bei nur einem Stationstest.
2. Zustimmung; Einwand: Scope-Grenzen dokumentieren.
3. Zustimmung; Einwand: Lizenzstatus vor Nutzung klären.
4. Zustimmung mit Vorbehalt; Einwand: Konverter-Schicht JSON→Binär wartbarer als Programm-Änderung.

## Sakana (Namazu)
1. Zustimmung mit Einschränkung; Einwand: ~15″/500 m Raster — dokumentierter Toleranzbereich, keine Gleichheit.
2. Zustimmung; Einwand: semantische Kopplungen (Imports/Reihenfolge) — byte-identische Erhaltung fremder Blöcke garantieren.
3. Teilweise; Einwand: 404/Lizenz unbekannt sind disqualifizierende Mängel, nicht „Lücken".
4. Zustimmung Umstellung; Abweichung „alle drei bauen" (YAGNI); Adapter weniger invasiv.

## MiniMax (M3, Search)
Teilantwort — der Composer nahm nur das Prompt-Ende; das Modell meldete „mir fehlt der Input". **Gemessen: Eingabe gekappt** (kein stiller Ausfall), Antwort unbrauchbar.

## Geteilte Open-Weight-Seats (`tryingopen`, Lock gehalten, ein Tab je Modell)
- **DeepSeek V4 Pro** (1.7T): (1) bedingte Zustimmung — Ein-Punkt-Vergleich validiert fast nichts, mehrere Stichpunkte + Toleranzband + Bezugssystem. (2) Zustimmung mit Auflage — fragil bei gemeinsamen Imports/Zeilenverschiebungen; besser eigenes Modul oder Merge-Workflow + atomares Schreiben. (3) Duplikate streichen ja; die zwei Lücken: Abweichung — 404 kein Ersatz, unbekannte Lizenz rechtlich nicht nutzbar. (4) Abweichung — Vertrag gehört nicht automatisch dem Leser; erst versionieren, Leser um JSON erweitern oder Adapter.
- **GLM 5.3** (753B, Z.ai): (1) Zustimmung — Geoid-Abweichung Nord-Ontario ~30 m überdeckt alles; Datum + Toleranz (±10–15 m) vorab, zweites DEM (CDEM) mitabgleichen. (2) Zustimmung mit Vorbehalt — ohne Sperre/CAS bleibt last-writer-wins; Datei nach Linien teilen. (3) Zustimmung — „Duplikat" inhaltlich belegen (Hash/Version), 404 → Archiv/tot markieren, unbekannte Lizenz = Use-Blocker. (4) Zustimmung — Binär nur mit versioniertem Layout (Magic/Endianness/Feldreihenfolge); jedes Programm einzeln mit Conformance-/Golden-File abnehmen.
- **Inkling** (975B, „Answered by Thinking Machines", `tryingopen`): **hat geantwortet** (vom Operator nachgereicht, 2026-10-13). (1) Abweichung — ein globales Dataset für einen einzigen Messpunkt erzeugt unverhältnismäßigen Wartungs-/Abhängigkeits-Overhead; lokale Subset-Validierung ohne Registereintrag reicht. (2) Abweichung — block-scoped verschleiert Verantwortlichkeiten, erhöht Merge-Konflikte; eigene Leser gehören in ein eigenes Modul. (3) Zustimmung Streichen; Abweichung Einfügen — die zwei sind keine „Lücken", sondern ungeklärte Blocker (404 ohne Archivnachweis, fehlende Lizenz); Eintrag ohne Klärung verbietet sich. (4) Abweichung — einseitige Konformität ignoriert weitere JSON-Konsumenten und bricht Rückwärtskompatibilität; Adapter/Vertrag neu verhandeln statt Programm zwangsweise umschreiben. **Lehre:** mein voriges „nicht erreichbar" war eine ungemessene Behauptung (Faulheit), kein Messwert. Nemotron 3 Ultra hatte in Runde 1 geantwortet.

## Z.ai / GLM-5.3 (Deep Think Max) — PFLICHT, geantwortet
1. Zustimmung; Einwand: Vertikaldatum/Toleranz ungeklärt (179,94 m orthometrisch; ellipsoidisch ~30 m mehr; Geoidundulation Nord-Ontario ≈ −30 m); SRTM15+ ist 15″-Zellenmittel (~450 m) und Oberflächenmodell (Vegetationsbias im borealen Wald) — ohne Toleranzfenster (±10–15 m) scheitert die Prüfung künstlich oder besteht bedeutungslos.
2. Zustimmung als Sofortmaß; Einwand: die gemeinsame Präambel ist nicht block-isolierbar (Import-/Konfigzeilen); Dauerlösung = eigenes Modul, Eigentumsgrenze = Dateigrenze.
3. Zustimmung; Einwand: Redundanz ist ein Feature — nur streichen bei identischer Quelle (gleicher Ursprung), nicht bei gleichem Inhalt anderer URL (das wäre ein Fallback); die zwei Lücken brauchen je eine Aktion (Archiv/Wayback bzw. Alternativhost; Lizenzabklärung/Sperrvermerk).
4. Geteilt: Binär zustimmen, wenn „Vertrag gehört dem Leser" projektweit gilt; risikoärmster Weg wäre ein dünner JSON→Binär-Adapter statt Änderung eines validierten Programms; vorab prüfen, dass das Binärformat double precision trägt; „bauen erst nach Vollständigkeit" — Abweichung: frühe Einzel-Builds fangen genau solche Vertragsdrifts auf.
