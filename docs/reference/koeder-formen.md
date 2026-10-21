<!--
  title: Köder und Datenformen — was die Herkunft trägt (2026-10-21)
  class: ref
  date: 2026-10-21
  status: live
  see-also: docs/reference/search-channels.md docs/surveys/survey-2026-10-20-datenherkunft-forensik.md
-->
# Köder und Datenformen — was die Herkunft trägt

**Herkunft dieses Dokuments.** Der Inhalt stand in der Ratsdatei
`state/rat/2026-10-20-rat-weberin-koeder.md` (Runde „Herkunft aus Fragmenten,
Anker-Merge, Köder"). Sie wurde mit `7d6eb96` gelöscht, weil sie die bereits
vergebene ID `R-2026-10-20-a` doppelt trug und nicht im Register stand — ein
Registerfehler, kein inhaltlicher. Der Text ist aus `7d6eb96^` geborgen und
hier als Referenz festgehalten; das Verdikt der Runde bleibt dort, wo es
hingehört, und wird hier nicht als neues Ratsverdikt ausgegeben.

## Die Frage der Runde

Soll die Linie den Weg **Köder → Zwilling → Merge** weiter ausbauen (A), zuerst
die Bedingungskette formalisieren (B), die keylosen Kanäle zur Pflicht machen
(C), oder die Zwilling-Suche abschließen (D)? Zusatzfrage: Für jede Datenform
im Register ist zu sagen, **welches Signal die Herkunft trägt, welcher Anker
zulässig ist und wo der Köder blind bleibt.**

**Votum der fünf bezahlten Sitze:** **3 × A, 2 × B** (DeepSeek V4 Pro, GLM-5.3,
MiMo-V2.6 → A; Step-5-Preview, Qwen3.8 → B). Kein D.

## Datenformen im Register (gemessen 2026-10-20, `phi/sources.φ`)

| Familie | Blöcke | Formate | Beispiel |
|---|---:|---:|---|
| Tabelle/Text (CSV, Festspalten, Text) | 1152 | 23 | ICRF3, SuperMAG, FIRMS |
| sonstige (Einzelformate) | 368 | 269 | GCN, Kataloge, Rohformate |
| Ephemeride (SPK, EPM, INPOP) | 141 | 9 | JPL DE, IMCCE |
| EEG/BIDS | 136 | 5 | OpenNeuro, ds004100 |
| Radio-Science (ODF/ODR/ATDF) | 67 | 23 | Voyager, Juno, LRO |
| JSON/API | 19 | 1 | Open-Meteo, ALeRCE |
| Archiv (zip/gz/tar) | 17 | 6 | WWLLN, TNS |
| Sky/HEALPix | 11 | 1 | GW-Skymaps |
| FITS | 10 | 5 | FUGIN, VizieR |
| NetCDF | 6 | 1 | GEBCO, ISD |
| Spektrum | 5 | 1 | EDGES T21 |
| Bild | 4 | 3 | Faksimile, Tafeln |
| Audio | 4 | 3 | Sonifikation |
| Geo | 1 | 1 | Grenzen |

## Signal · Anker · blinder Fleck je Form

| Form | Signal (was die Herkunft trägt) | Anker (zulässig) | Köder blind für |
|---|---|---|---|
| **Tabelle/Text** | Kopf + Einheit + Zeitspalte | Name · Spalte · Einheit · Zeit · Wert | kopierte oder semantisch umbenannte Spalten; Text trägt kein IEEE-Byte |
| **FITS** | Magic `SIMPLE`/`BITPIX` + Header-Karten | Header-Schlüssel · HDU/`EXTNAME` · WCS · `DATE-OBS` · Checksum | Header sind überschreibbar/transplantierbar; `BSCALE`/`BZERO` werden beim reinen Wertvergleich ignoriert; echte 0 vs. NULL-Maske |
| **NetCDF** | Attribute + Dimensionen + Variable | Variablenname · `units` · shape · Koordinaten | `scale_factor` (gepackte Werte), `history` umschreibbar, externe Referenz ohne `channel_ref` |
| **ODF/ODR** | Container-/Record-Aufbau (33 B, Typ 0/10/12/13) | Layout + Feld-ID + Version | Typisierung steht in der Struktur, nicht im Byte; Konvertierung kann sie verlieren |
| **Ephemeride** | Frame + Zeitskala + Koeffizienten | Körper · Zentrum · Epoche · Frame · Record | Wert ohne Frame/Zeit; TDB↔UTC und Interpolation |
| **EEG/BIDS** | Sidecar-JSON + Kanalstruktur + Record | Subjekt · Task · Run · Kanal · `channel_ref` · Zeit | Anonymisierung kappt die absolute Zeit; Re-Referenzierung ändert Werte; `0 honored` |
| **JSON/API** | Schema/Version/Typen + HTTP-Status/Content-Type | Ressource/URI · Schema-ID · Zeitstempel | kein Magic, kein fester Record; Pagination-State; Feld-Null vs. Lücke |
| **HEALPix** | `NSIDE` + Ordnung + Frame | Pixel-ID · Ordnung · Frame · Epoche | Maskenpixel mit Wert 0 tragen keine Lücke |
| **Bild** | Dimensionen + EXIF/ICC + Struktur | Hash · Aufnahme-/Pixelregion | Byte-Bedingung fällt aus; entfernte Metadaten |
| **Audio** | Codec + Abtastrate + Kanäle | Hash · Zeitfenster · Kanal | Resampling verändert Bytes und Werte |

Der gemeinsame Satz: **Was für Tabellen gilt (Kopf/Einheit/Spaltenzahl), gilt
für FITS (Header-Karten) und NetCDF (Dimensionen/Attribute) nur teilweise; für
ODF/ODR und Ephemeriden entscheidet der Record-Aufbau, für HEALPix Ordnung und
Pixelzahl.**

## Vier Punkte, in denen alle fünf Sitze einig waren

1. **`channel_ref` fehlt als Pflichtglied** — ohne Kraft (Antenne, Gain,
   Bandpass) kein Sample; „eine genealogische Karte ohne Geologie".
2. **Der Konflikt gewinnt** — gleicher Anker, verschiedene Werte: beide bleiben,
   markiert (`conflict`, `riss: null-echt`), nie überschrieben.
3. **Der strengste Lizenzterm regelt** — `no-redistribution` bleibt Verweis,
   nicht Kopie; CC-BY-Attribution wandert in die Herkunftskarte.
4. **`0 honored` gilt auch für den Köder** — eine API, die über Lücken
   schweigt, sieht aus wie eine Null.

## Die Risse, die bleiben (je Sitz einer, ungeglättet)

- **Byte-Kraft-Riss:** dasselbe Byte aus Datei und Paper trägt andere Kraft.
- **Lizenz-Zwilling:** freie Teilmenge ≠ geschlossener Arm; `lro_trk` bindet 400
  Dateien, die Serie hat ~100 000.
- **Kraftabriss:** `substorm` war ein Stopp der Maschine, keine Abwesenheit.
- **Sichtbarkeit:** „manifestiert" kollidiert mit `no-redistribution`; das
  Merge-Produkt bleibt unauditierbar.
- **Verifikationsgrad fehlt:** Vertrauen ist binär gerendert, nicht gestuft.

## Gemessenes Offen (Stand der Runde)

`ilrs_npt` (CDDIS-Wand), `babamul`/`demeter`/`tns` (kein Lizenztext),
`dhm_gauge` (kostenpflichtig), `fugin` (NAOJ-Copyright); keylose Web-Engines
dünn (Mwmbl 1 Treffer, Marginalia 504, SearXNG 1/75 JSON); bezahlte Arme
402/432/429/400 (Konto, nicht Code).

## „Je seltener, desto goldener" — und die Grenze (Schumann)

Das Prinzip steht **als Suchhebel** im Baum, nicht als Wahrheitsregel:

- `docs/concepts/fuenf-funken-anomalie-suche.md`: „Der seltene Wert ist nicht
  ein Teil, sondern die **Verdrahtung**" — mehrere Broker, TDB-Uhr,
  Transfer-Entropie als eine Kette; die Anomalie-Taxonomie macht das Seltene
  zum **Kandidaten**, nie zum Urteil (Klasse 3 bleibt „benannt und ungebaut").
- `docs/surveys/survey-2026-10-14-gewebe-faeden.md`: „Die seltenen Gesetze sind
  die Nadeln" — die Seltenheit entscheidet, wo man sucht, nicht was wahr ist.
- `bin/find` kennt **ZWILLING** und **NAH** als gemessene Abstände; einen
  Zahlenwert „Rarität" gibt es nicht. `weave` trägt keine `rarity`-Spalte.

**Beispiel Schumann.** Der Registerblock (`phi/sources.φ:1126`) ist genau so
ein seltener Fund:

```
url …/zenodo.org/resonance_schumann.csv
terms CC-BY-4.0 https://zenodo.org/records/15225820
origin …/NiNo_SR_params_2024.csv/content
harvest raw_zenodo
format csv
sha256 3183a14f…  ttl 3600  at earth
field freq1 resonance_schumann_f1_hz …
field freq2 resonance_schumann_f2_hz …
field freq3 resonance_schumann_f3_hz …
```

- **selten:** nur **ein** Jahrgang (2024) auf Zenodo, **eine** Quelle,
  7.835 Zeilen, 454.417 B — keine 2023/2025-Geschwister gefunden.
- **golden:** die einzige maschinenlesbare 3-Moden-Reihe (f1/f2/f3 +
  Amplituden), deshalb im Register und nicht nur im Paper.
- **Grenze:** der Zenodo-Record deklariert `cc-by-4.0`, die Beschreibung sagt
  dagegen „use … only with the authors' permission". Die Seltenheit macht den
  Fund wertvoll, nicht die Lizenz klar — der strengste Term regelt.

Damit ist die Regel präzise: **selten erhöht den Suchwert, nicht den
Beweiswert.** Ein seltener Wert bleibt ein Sample; er wird nicht dadurch
wahrer, dass es ihn nur einmal gibt (A = A).
