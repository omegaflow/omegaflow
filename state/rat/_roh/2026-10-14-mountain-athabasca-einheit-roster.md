# Roster — Athabasca-VLF-Einheit (Mountain, 2026-10-14)

Frage (eine, terse): Soll der Register-Wert als magnetische Leistungsdichte **nT²/Hz**
geführt werden (Kalibrierung im Lesearm, Interpolation über `freq_vlf`), und die Rohspannung
**V²/Hz** zusätzlich als eigener Kanal — oder beim Dateiwert V²/Hz bleiben? Nenne den
stärksten Einwand.

Gemessene Fakten im Prompt: `ch1/ch2` CDF-`UNITS = V^2/Hz`; `amplitude_cal_vlf_ch1` =
(Frequenz,Faktor)-Kurve `Hz V/T`; pyspedas `isee_vlf` `cal_gain=True`: `ch1 / (interp(cal)*1e-9)^2`
→ `nT^2/Hz`.

Gruppe `mountain-ui`, aktive Runde 2026-10-14. Send per Composer (native Setter + input-Event)
+ echtem Senden-Knopf (Duck/Qwen), `#send-message-button` (Z.ai), Enter/Button (Claude).

## Duck.ai (GPT-6 Luna) — geantwortet
Votum: **nT²/Hz als Hauptwert + V²/Hz als Rohkanal daneben**; den kalibrierten Wert als
abgeleitet kennzeichnen, Interpolation und Kalibrierfaktor dokumentieren.
Stärkster Einwand: die Einheit `Hz·V/T` passt dimensional nicht ohne Weiteres zu
`ch1/(cal·1e-9)²` — für V²/Hz → nT²/Hz müsste der quadrierte Faktor `V²/nT²` tragen; solange
das zusätzliche „Hz" nicht geklärt ist, gilt der kalibrierte Wert nicht als zweifelsfrei.

## Qwen (Qwen3.7-Plus) — geantwortet
Empfehlung: **primär nT²/Hz** (Kalibrierung im Lesearm, Interpolation über `freq_vlf`),
Rohspannung V²/Hz **zusätzlich als eigener Kanal**. (Detailtext abgeschnitten in der DOM.)

## Claude (Sonnet 5.5 Extra hoch) — geantwortet
Votum: **beides führen, mit klarer Rangfolge** — V²/Hz bleibt der gemessene Kanal
(= Dateiwert); nT²/Hz kommt als **abgeleiteter** Kanal dazu, markiert als berechnet aus
Rohkanal + Kalibrierkurve; nT²/Hz als Standardausgabe (Datensatzname/User-Intent).
Kalibrierung im Lesearm, mit gespeicherter Kurven-Version.
Stärkster Einwand: nT²/Hz steht **nirgends in der Datei**, nur in der Konvention des
Referenzlesers. Wörtlich `V²/Hz / (Hz·V/T)² = T²/Hz³`, keine Leistungsdichte. Nur wenn die
Kurve reines V/T ist, ergibt `1e-9` im Quadrat `1e18` (T²→nT²). Plausibel sei, dass „Hz V/T"
die zusammengezogenen Spaltenlabels sind (Frequenzachse Hz, Faktor V/T) — eine Lesart, kein
Dateibefund. Entschärfung: Kurven-`UNITS`/`LABLAXIS` und PWING-Beschreibung prüfen;
Kurven-Größenordnung (~1e8…1e10 für V/T) und Vergleich mit dem bekannten VLF-Magnetfeldrauschen
(pT/√Hz) ansehen; abgeleiteten Kanal mit Provenance führen; dort maskieren, wo |H| klein ist;
`np.interp` klemmt außerhalb der Kurve stillschweigend; Interpolationsfehler geht quadratisch ein.

## Qwen (auf Qwen3.8-Max gestellt) — Meta-Antwort, kein Verdikt
Der Seat antwortete nicht inhaltlich, sondern mit einem A/B-Präferenz-Prompt („Welche Antwort
bevorzugen Sie? Antwort 1 / Antwort 2"); die gezeigten Texte sind Meta-Snippets, kein Verdikt.
Damit **keine verwertbare Stimme** aus diesem Seat (gemessen; kein Tageslimit).

## Open-Weight (tryingopen, `open-weight-ui`) — Qwen3.8 2.4T geantwortet
Empfehlung: **nT²/Hz im Lesearm kalibrieren, aber als fixiertes, versioniertes Rezept** —
Interpolationsmethode und Randbehandlung explizit dokumentieren, Kalibrierkurve+Version in den
Register-Metadaten pinnen, die Transformation einmal deterministisch beim Ingest ausführen; das
rohe V²/Hz bleibt als **invarianter Ankerkanal**.
Stärkster Einwand: der Register-Wert wäre keine Dateiinvariante mehr, sondern ein Produkt des
Lesearms. Die Kalibrierung ist keine Konstante, sondern eine frequenzabhängige Übertragungsfunktion,
deren 1024-Bin-Anwendung nicht eindeutig ist (linear vs. logarithmisch, Amplitude vs. Leistung,
Achsen-Ausrichtung auf Bin-Mitten, Randbehandlung unter 10 Hz / über 20 kHz). Re-Kalibrierungen
ändern Werte rückwirkend, obwohl die Datei unverändert bleibt (Auditierbarkeit). Die Referenz
selbst ist kein sicherer Anker: pyspedas wendet die Kalibrierung **nur im Plot-Zweig** an und hat
den bekannten ch2-Bug (`tmp1[1]` statt `tmp2[1]`).

## Z.ai (GLM-5.3 Deep Think Max) — `pending` (kein Assistant-Knoten nach mehreren Wartezyklen)
Nachricht platzierte sich (im Chat sichtbar, durch den doppelten Setter-/Typing-Send zweimal);
**keine Antwort im DOM** nach mehreren 30-s-Wartezyklen (der Chat-Knoten blieb ~4 kB, kein
Assistent-Block). Gemessener Zustand, kein Tageslimit. **Nicht** die Lehre von 2026-10-12
(Tab zu früh geschlossen) — der Tab stand bis Rundenende offen, die Antwort kam nicht.
Rest: den Seat in einer eigenen Runde mit einfacher Nachricht erneut stellen.

## Konvergenz
Geantwortet: Duck · Claude · Qwen (3.8-Max-frei) · Qwen3.8 2.4T. **Alle konvergieren mit dem Rat:
nT²/Hz primär (Kalibrierung im Arm), V²/Hz als eigener Kanal daneben.** Der eine Riss (alle, am
schärfsten Qwen3.8 2.4T + Claude): die Kalibrierkurve-Einheit `Hz V/T` und die
Interpolations-/Versionsdisziplin — der Transform muss gemessen/fixiert sein, sonst ist die
Umrechnung geraten; die Rohspannung bleibt als invarianter Anker. GLM Deep Think Max trug nichts
bei (`pending`).
