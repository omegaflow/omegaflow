# UI-Stimmen — Auftrag „Eigene Ephemeriden + Kulturerbe" (Ratsfrage 2026-10-12, Mountain)

**Frage (eins je Runde):** Architektur-Verdikt zum Auftrag `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md` — konvergiert oder reißt ihr? Größter blinder Fleck? Reihenfolge: ITRF2020/EOP + ein ODF zuerst, oder LLR zuerst?

Gruppe `mountain-ui` (eigene Frontier-Seats). Rohmaterial; das Verdikt trägt die Session/Rat.

## Qwen (Qwen3.7-Plus) — **konvergiert**
- **Verdikt:** „KONVERGIERT (aber nur unter eiserner Disziplin bei der Datenhomogenisierung). Die Trennung von *Heritage* (Validierung/Rekonstruktion) und *Fuel* (primäre Fit-Daten) ist der entscheidende Hebel."
- **Blinder Fleck:** „epochenspezifische, nicht-stochastische Systematik im historischen Korpus" — (a) Zeitskalen-Brüche TAI/TT/TDB in alten Archivdaten; (b) thermischer Rückstoß (Pioneer-Anomalie) muß als **eigener zu fittender Parameter je Sonde** modelliert werden, sonst absorbiert der Solver ihn in die Gravitationsparameter; (c) historische DSN-Antennen-Koordinaten mit **Meter-Offsets** zum modernen ITRF.
- **Reihenfolge:** **ITRF2020/EOP + ein modernes ODF zuerst.** LLR ist extrem sensitiv gegen Erdorientierung/Geozentrum → wenn LLR zuerst, Degeneriertheit zwischen EOP-, Rahmen- und lunarer Dynamik. Dann ODF zur Stabilisierung des baryzentrischen Rahmens, erst dann LLR.
- **Schlußsatz:** „Investieren Sie 80 % der Engineering-Zeit in die Metadaten-Pipeline und die systematische Fehlerbudgetierung des *Heritage*-Registers, nicht in den Solver selbst."

## Claude (Sonnet 5.5 Extra hoch) — **konvergiert**
- **Verdikt:** „Verdikt: konvergiert. Alle vier Punkte halten, drei brauchen eine Schärfung."
- **Reihenfolge:** **LLR zuerst.** „Die Frage ist teilweise schief gestellt, denn ITRF2020/EOP ist keine Alternative, sondern das **erste Modul der LLR-Kette**. LLR bringt eine Observable, saubere Normal Points, Jahrzehnte Abdeckung, mm–cm-Niveau, ist am ersten Tag falsifizierbar." Ein ODF zuerst wäre schwerer zu deuten (Binärformat, Medienkalibrierung, Sondentrajektorie mit eigenen freien Parametern: Manöver, Strahlungsdruck). Den **ODF-Parser parallel als Unit-Test** (Format/Zeit/Frequenzrampen), nicht als astronomischen Test.
- **Brennstoff:** „Dort steckt der echte Brennstoff in **Doppler und Range** (Voyager 2 an Uranus/Neptun, Pioneer, Viking-Lander-Ranging), **nicht in Bildern**. Die Metadaten der Ära (Stationssets, Referenzrahmen, Uhr) sind wichtiger als die Bits selbst."

## Z.ai (GLM-5.3, Deep Think **Max**) — `pending`
- Nach >90 s noch „Thinking… Deep Think Max"; keine Antwort innerhalb des Rundenschlusses. **Gemessener Nichtantwort-Grund:** Deep Think rechnet noch. Nicht wiederholt anfragen.

## Duck.ai (Luna) — `pending`
- Kein Antworttext sichtbar (Seat zeigt nur Composer/Sidebar). **Gemessener Nichtantwort-Grund:** kein Turn erzeugt. Nicht wiederholt anfragen.

## DeepSeek Chat (chat.deepseek.com, Suchmodus) — **konvergiert**
- **Verdikt:** „Konvergiert — mit einer Einschränkung." „Ein Archivar, zwei Register ist genau der richtige Schnitt … Ein Gate, das nur prüft, ob das Modell zu sich selbst passt, ist kein Gate. Das ehrliche Kalibrier-Gate ist ein Hold-out: Testdaten müssen vor dem Fit festgeschrieben werden." „Unmodelliertes gibt pending, nie 0.0 — ein 0.0 ist eine Behauptung, ein pending ist eine epistemische Lücke."

## DeepSeek V4 Pro (tryingopen, 1.7T) — **konvergiert, bestätigen mit Auflagen, kein Riss**
- **Register:** rohe `obs`/`heritage` strikt von abgeleiteten `observable`/`asset` trennen; **content-addressed/immutable + Schema-Version + Provenance**; eine Observable nur mit vollständigem Reduktionsrecord (Frames, Zeitskalen, Clocks, Medien, Bias).
- **Hold-out:** vor dem Fit eingefroren, einmalig, kein Re-Editing; **mehrere Hold-outs pro Regime/Epoche** statt einem.
- **Operator:** `pending`, nie 0.0 — nicht verhandelbar; aber **jeder Operator braucht ein explizites Noise-/Bias-Budget**, sonst wird `pending` zur Müllkippe.
- **Blinder Fleck:** „nicht der Ephemeriden-Fit, sondern die **Reduktion der Rohdaten auf eine saubere Observable**" — Zeitskalen (TDB/TT/UTC), Frames, Antennendelays, Troposphäre/Ionosphäre, Stationskoordinaten, bei Sonden die nicht-gravitativen Kräfte + die Heterogenität/Dokumentationslücken des historischen Korpus. „Der Fit ist der kleinste Teil; die Kalibrierkette ist der Engpass."
- **Reihenfolge:** **ITRF2020/EOP zuerst (einfrieren, nicht mitfitten)**, dann **LLR vor dem ersten ODF** — LLR ist der billigste vollständige End-to-End-Beweis (eine Observable, ein Körperpaar, keine Sonden-Nongrav). Der erste ODF ist der **zweite** Operator.

## Qwen3.8 2.4T (tryingopen, 2.4T) — **konvergiert; These 4 ist kein Einzelfall, sondern eine Regel**
- **Die Verschärfung — `raw_level` 0/1/2:** **Level 0 (Brennstoff)** = Tracking-Counts (TRK/ODG-Roh), VLBI-Korrelatoroutput, LLR-Normalpunkte, Radar-Echos. **Level 1 (vorbelastet)** = ODF, SPICE SPK/CK (bereits Fits gegen jemandes DE + Frame + Uhr). **Level 2 (Erbe)** = Bildpfade mit Pose aus DE/SPICE-Fit. → **Level ≥ 1 ist Kalibrier-Kreuzcheck und Heritage, nie Brennstoff**; sonst reparametrisiert omegaflow nur JPLs Ephemeriden — „das gilt für Pioneer/Viking/Cassini genauso wie für Mariner 10".
- **Blinder Fleck:** der **ITRF↔ICRF-Tie** (EOP, Stationskoordinaten, Zeitskalen TT/TDB, SCLK-Korrelation) — Frame-Fehler werden vom Fit als „Physik" absorbiert. Fit ≠ Operator: DE440 schätzt hunderte Asteroidenmassen/Stationsbiases/Plasmakalibration mit → ein schlanker Dynamik-Anteil erzeugt systematische Residuen, die der Hold-out nicht sieht. **Hold-out nach Bögen und Stationen splitten, nicht zufällig nach Samples** (sonst testet man Interpolation, keine Systematik).
- **Reihenfolge:** **Frame zuerst, dann LLR** — „ITRF/EOP + ein ODF zuerst" ist eine **Falle** (erbt die Kontamination am ersten Tag); „LLR zuerst ohne Frame-Schicht geht auch nicht". Also: **Frame als Fundament, LLR als erster Stein**, danach der erste Level-0-Sondenpass (Counts, nicht ODF).

## Z.ai (GLM-5.3, Deep Think **Max**) — **konvergiert, mit der schärfsten Verschärfung**
- **Korrektur:** in Runde 1/2 irrtümlich als `pending` verbucht (Tab zu früh geschlossen); die Antwort lag im Chat `chat.z.ai/c/94e59dba…` und ist vollständig da.
- **Grundregel:** „**Diagnostik darf alles sehen** (auch DE/INPOP/EPM, post hoc); **Brennstoff darf nichts Fremdes sehen.**"
- **(3) zwei Geschwister fehlen:** (a) **`pending` muß ins Beobachtungsdatum schreiben** — die Observable weiß, daß sie degradiert ist → Gewicht herabsetzen oder quarantänisieren, **nie als sauber mitteln**; (b) **Bias-Parameter ≠ physikalische Parameter** — Bias schluckt Systematik, nur Physik fließt ins Produkt; **Zero-Partials heißen „unbestimmt" (Kovarianz → ∞ ins Manifest), nie „0 ± 0"**.
- **(4) bestätigt:** Bilder würden erst Brennstoff, wenn die Pose **aus den Bilddaten selbst** rekonstruierbar wäre (Stern-/Limb-Fit) — Forschungsprojekt, kein Pipeline-Feature. „Nicht versprechen."
- **Größter blinder Fleck: die Unabhängigkeit selbst.** Die Regel aus (4) gilt auch **auf der Eingangsseite** — LLR braucht Störkörper (Sonne, Jupiter, Saturn) mit **publizierten Fit-Anfangszuständen**; historische UT1 vor der VLBI-Ära teils über LLR **mit DE** bestimmt; Stationsties der Erbe-Epochen sind selbst Rückextrapolate. „Euer System ist **an den Rändern an DE verankert** — und die Residuen werden gut sein, gerade weil die Anker gut sind. **Gute Residuen bestätigen euch eine Unabhängigkeit, die ihr nicht habt.**" Abhilfe: **Priors first-class (Quelle + Kovarianz), Produkt-Manifest („unabhängig bzgl. X, gegeben Priors auf Y")**, plus ein Strang, der Anker durch eigene Daten ersetzt.
- **Reihenfolge:** das Entweder-Oder ist falsch gestellt. Erst die **Rahmenschicht (ITRF2020 + EOP + Zeitskalen)** — Vorbedingung für beide Wege. Dann **Rollentrennung: LLR zuerst als Brennstoff** (kleinstes wohlgestelltes Problem; die Observable ist **relativ** — in der Differenz Station–Reflektor hebt sich der Fehler der heliozentrischen Erdephemeride in führender Ordnung auf → **geringste Zirkularität aller Datenarten**; Librationen anfangs deklarierter Prior). **Ein ODF parallel — als Prüfling, nicht Brennstoff:** ein Bogen, alles Externe auf publizierte Werte fixiert, nur Raumfahrzeugzustand + Arc-Biases fitten, publizierte Residuen reproduzieren (validiert Parser/Zeitmarken/Medien; später der Ingestionspfad). **Brennstoff danach: Radar (innere Planeten) → ΔDOR/VLBI (ICRF3) → erst dann Tiefraum-Tracking. Genau dann wechselt der Korpus die Klasse: Erbe → Brennstoff.** So versöhnen sich beide Ziele.
- **Projektrisiko:** „LLR-Normalpunkte-Beschaffung ist **Politik, nicht Architektur** — früh klären."

## Open-Weight-Notiz — GLM 5.3 (753B) doppelt gefahren
- **GLM 5.3 (Z.ai, 753B) über tryingopen:** nach dem Z.ai-Deep-Think-Max-Erfolg zusätzlich im Open-Weight gestellt; zur Rundenschließung noch im **Reasoning** (langer CoT), keine fertige Antwort. Das GLM-Verdikt steht **vollständig** über den Z.ai-Seat (Deep Think Max) — der open-weight-Zwilling ist derselbe Modellstamm und redundant, `pending` mit gemessenem Grund (kein neuer Inhalt zu erwarten).

## Duck.ai (Luna) / Nemotron 3 Ultra — `pending`
- Duck.ai: kein Turn erzeugt. Nemotron 3 Ultra (open-weight, Runde 1): „That model didn't respond." Beide gemessen `pending`.

## Der Riss (Stimmen gegen den Rat)
- **Reihenfolge:** Qwen (ITRF/EOP+ODF zuerst) gegen Claude (LLR zuerst, ITRF/EOP als deren erstes Modul). Beide konvergieren in der Architektur; nur die Sortierung reißt. **Nicht glätten.**
- **M10-Bild als Brennstoff:** beide UI-Stimmen bestätigen den Rat — der Brennstoff ist **Tracking (Doppler/Range)**, nicht das Bild. Das Bild ist **Erbe**.
- **Geteilter blinder Fleck** (Rat + Qwen + Claude): **Metadaten/Zeitskalen/Rahmen** der Ära (TAI/TT/TDB, DSN-Stations-Offsets, Medien-/Transponder-Kalibrierung, thermischer Rückstoß) — die arbeitende Ebene (Parser/Solver) sieht ihn nicht.
