# Roster-Runde — Katalogachse (2026-10-10)

Frage an alle Seats: Sollen die Katalog-/Suchseiten aus `phi/` (72 IVOA-TAP-Indexe, b2find, re3data, pangaea, dryad, seanoe; ~180k Z.) entfernt und als live-Sucharme in `archive_search` geführt werden? Ein Compiler (`tap_index_merge`) liest die Dateien heute. Verdikt (a)/(b)/(c)?

## Frontier-Seats

### Qwen (Qwen3.7-Plus)
**(a)** — Dateien aus phi entfernen und `tap_index_merge` abschaffen. Begründung: Discovery ist Metadaten-Suche, keine Materialisierung von Messbytes; statisch kompilierte Indexe (~180k Z.) erzeugen technische Schuld/Build-Overhead/Daten-Staleness; Live-Arme in `archive_search` erlauben föderierte, aktuelle Abfragen.

### Z.ai (GLM-5.3, Deep Think/Max)
**(c)** — Achse präzisieren: nicht „Dateien vs. live", sondern „wo wohnt der lokale Index, womit gespeist". Gegen (a): Live-Fan-out über 72 TAP-Endpunkte = Anti-Pattern (Latenz-Schwanz, Teilfehler, Rate-Limits, kein Ranking; IVOA harvet selbst in lokale Stores). Gegen (b): 180k Z. gescrapte Suchseiten im Repo = Daten im Codemodul (Diff-Noise, Verrottung, kein Refresh, HTML-Kopplung). (c): **Harvest statt Scrape** (VOResource/RegTAP, OAI-PMH, re3data-API, Dryad-API), **Compiler migrieren, nicht abschaffen** (als Ingest-Stufe), `phi/` schrumpft auf Konfig + Fixtures; eine Arm-Schnittstelle, zwei Implementierungen (Index-Arm = Fast Path; Live-Arm pro Quelle). Kriterium: „global suchen/ranken" → lokaler Index via Harvest; „gezielt stöbern" → Live-Arm.

### Claude (Sonnet 5.5, Extra hoch)
**(c)** — „gepinnter Snapshot vs. Live-Abfrage": die Katalogseiten gehören aus phi raus (sie sind Zeiger auf Messbytes), bleiben aber als **versionierter Snapshot** (`fetched_at` + Quell-Hash) samt `tap_index_merge` als **Build-Schritt in `archive_search`**; Live-Arme nur für Refresh/Verifikation/frische Quellen. Gegen (a): reine Live-Suche über 72+ Upstreams hängt von Latenz/Rate-Limits/Ausfällen ab, nicht-deterministisch, quellenübergreifendes Merge/Dedup nicht pro Anfrage tragbar. Gegen (b): phi vermischt Discovery und Materialisierung, Metadaten-Drift rauscht Provenienz/Diffs. **Wichtig:** „Verdikt kippt Richtung (b), falls etwas in phi diese Dateien als Eingabe konsumiert" — **genau das ist gemessen der Fall** (`tap_index_merge` liest sie).

### DeepSeek (DeepThink + Search)
**(c)** — die Dateien sind **abgeleitete Suchindizes**, kein Katalogdatum; der Compiler als Datenaufbereitung durch einen echten **RegTAP-Index** ersetzen, der die 72 TAP-Endpunkte live abfragt. „Ein Compiler, der statische Dateien liest, ist kein Sucharm." Discovery (Indizes abfragen) ≠ Materialisierung (Daten laden).

### Mistral (Vibe)
**(c)** — Achse „generierte Artefakte im Source-Tree vs. Runtime-Föderation mit auffrischbarem Cache": 180k Z. fremder volatiler Metadaten sind ein Snapshot-Cache → raus aus phi; Live-Arme als Quelle der Wahrheit korrekt; **Compiler umwidmen** (nicht abschaffen): vom Build-Schritt über Statikdateien zur **Runtime-Föderations-/Normalisierungsschicht + Cache-Refresher**; 72 heterogene Endpunkte brauchen Schema-Mapping + Stale-Fallback.

### Lumo (Proton, 4/4-Seat)
**Richtung (c)** (Streaming beim Lesen) — Achse „Source-of-Truth vs. ableitbarer Cache": geerntete Bulk-Metadaten im Repo verrotten → raus aus phi; Compiler/Harvest als **Artefakt-produzierenden Job** behalten (Diff/Reproduzierbarkeit), Live-Arme für Frische.

### Gemini (Google AI Studio, Gemini 3.1 Pro Preview)
**pending** — Frage gesendet (142 Tokens, Grounding mit Google Search aktiv), Antwort beim Lesen nicht im sichtbaren Text (Temporary Chat). Beim nächsten Pass erneut lesen.

### Duck.ai (GPT-Seat)
**Tageslimit erreicht** („Wird in 1h zurückgesetzt") — gemessener Nichtantwort-Grund.

## Geteilte Open-Weight-Seats (`open-weight-ui`)

### Tryingopen → GLM 5.3 Flash (answered by Z.ai)
**(b)** — Dateien bleiben: Index-Snapshots sind **Eingangsartefakte des Compilers**, nicht phi-UI-Ballast; `tap_index_merge` braucht die deterministische Quelle für reproduzierbare Merges (Pin, Diff, Rebuild). Live-Arme kommen als **zusätzliche Achse** dazu, nicht als Ersatz (Suche = Read-Path, Materialisierung = Compile-Path, unterschiedliche Konsistenzgarantien). Erst wenn der Compiler auf Live-TAP-Föderation umstellt, dürfen die Dateien weg — heute: Gewinn an Frische gegen Verlust an Reproduzierbarkeit.

## Konvergenz / Riss

- **Frontier konvergiert auf (c):** Dateien raus aus `phi/`, aber **Compiler/Index migrieren, nicht abschaffen** (Snapshot-/Cache-Artefakt, außerhalb der Versionierung), Live-Arme als Quelle der Wahrheit für Frische; Harvest statt Scrape (RegTAP/VOResource).
- **Dissens:** Qwen = (a) (Compiler abschaffen), GLM-5.3-Flash = (b) (Dateien bleiben, Reproduzierbarkeit). Beide Extreme benannt, nicht geglättet.
- **Wichtigstes Signal:** Claude nennt die Bedingung, unter der (b) kippt — „falls etwas in phi die Dateien als Eingabe konsumiert"; der Baum bestätigt genau das (`tap_index_merge` + `port.rs`).
