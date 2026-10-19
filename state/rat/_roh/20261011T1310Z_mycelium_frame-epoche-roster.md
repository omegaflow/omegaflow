# Architektur-Verdikte — Mycelium Frame/Epoche (voller Roster, 2026-10-11T13:1xZ)

Frage (voll gestellt, Begründungspflicht): Coverage-Manifest trägt je Shard eine bbox (6 f64);
der Query-Loader liest sie roh als ICRS-Meter → gesetzte Boxen schließen falsch aus.
**A** = CRS(+Epoche) als Token der `shard`-Grammatik (Wire-Kontrakt-Änderung, Operator-Wort).
**B** = kein Token; der Loader leitet die CRS aus dem Format-Token ab, transformiert zur Query-Zeit;
unbekannt → `Pending`.

Vorbereitung: Recherche `state/stimmen/20261011T1250Z_mycelium_frame-epoche-recherche.txt`
(`archive_search --all`, 249 Z.). Roster: Frontier (`mycelium-ui`) + starke Open-Weight
(`open-weight-ui`, Lock gesetzt/gelöscht) + Rat.

## Stimmen

| Stimme | Klasse | Verdikt |
|---|---|---|
| **Rat** (5 Verfassungsstimmen) | Rat | **B** |
| **Claude** (Sonnet 5.5 Extra hoch) | Frontier | **A** — „B bleibt als Writer-seitige Ableitung nützlich: beim Schreiben ableiten und das Token stempeln, am Leser aber nie raten." Risse B: Prune-Kontrakt (LAS-VLR nur durch Öffnen), Epoche nicht ableitbar, Determinismus (Bibliotheksversion). |
| **Mistral** (Large 4 / Vibe) | Frontier | **A** — „zwei Loader-Versionen schließen bei identischem Manifest unterschiedlich … `Pending` ist unter B keine Lösung, sondern eine Verlagerung: ‚kein Ausschluss' ist bei einer Coverage-Abfrage eine falsch-positive Zusage." |
| **DeepSeek Chat** (DeepThink) | Frontier | **A** — „der Kontrakt liegt dann im Wire, nicht in einer Loader-Konvention … B ist ein Latentzünder, den Pending nur verdeckt." |
| **Gemini 3.1 Pro Preview** (AI Studio) | Frontier | **A** — „CRS als explizite, selbsttragende Wahrheit … der Riss von B ist die verdeckte Kopplung (Abstractions Leak) zwischen Index-Metadaten und Dateiformat." |
| **Lumo 2.0 Max** (Proton) | Frontier | **A** — „für den Anspruch ‚Bytes selbsttragend, A=A über Loader/Versionen' ist A das fundierte Verdikt." |
| **MiniMax M3** | Frontier | **A** — „dieselbe Box, dieselbe Geometrie, quer durch Loader- und Registry-Versionen … STAC→WGS84 und LAS→VLR sind Heuristiken, nicht Kontrakt." |
| **Qwen3.8 2.4T** | **Open-Weight (2,4T)** | **A** — „sechs f64 ohne CRS/Epoche … der Leser leiht ihnen den Bezugspunkt; zwei Loader leihen verschieden. … Abfederung: B's Pending als Fallback." |
| **Inkling** | **Open-Weight (975B)** | **A** — „nur ein explizites CRS+Epoche-Token macht das Manifest selbsttragend … Preis B: dauerhafte Fragilität und faktische Rückkehr zu `--bbox-off`." |
| **Qwen3.7-Plus** | Frontier (schwach) | **B** — „Option B bewahrt den bestehenden Wire-Datenkontrakt." |
| **GLM-5.3** (Z.ai) | Frontier | **A** — „die Bedeutung der Bytes folgt aus den Bytes … der Loader wird zum reinen Dekoder … der Riss von B: das CRS wird zu einer Funktion des Readers statt der Daten (STAC `proj:bbox`, LAS-VLR erst nach Öffnen/teils widersprüchlich, Loader-Version); ‚unbekannt→Pending' macht Vertragsverletzungen zu unsichtbaren Laufzeitzuständen." |
| **DeepSeek V4 Pro** | Open-Weight (1,7T) | Nichtantwort — „site has run out of API credit" / „No response" |
| **GLM 5.3** | Open-Weight (753B) | Nichtantwort — „site has run out of API credit" |
| **Perplexity** | Frontier | `unread` — Frage abgesetzt, keine Antwort im Text erfasst |
| **Duck.ai** (Luna) | Frontier | Nichtantwort — nicht abgeschickt (leerer Composer) |

## Befund

- **9× A** (Claude, Mistral, DeepSeek Chat, Gemini 3.1 Pro, Lumo, MiniMax M3, GLM-5.3 (Z.ai), Qwen3.8 2.4T, Inkling)
  gegen **2× B** (Rat, Qwen3.7-Plus).
- **Beide** starken Open-Weight, die antworteten (Qwen3.8 2.4T, Inkling), sagen **A**.
- Recherche stützt **A** (luma.gl verlangt/verwirft CRS vor dem Upload; GeoParquet „reproject before you compare").
- `tryingopen` trägt nicht durchgängig: DeepSeek V4 Pro + GLM 5.3 = „site out of API credit";
  Qwen3.8 2.4T + Inkling antworteten (ngrok/Alibaba).

**Verdikt:** **A** — CRS + Epoche gehören als Token in den `shard`-Vertrag; `Pending` bleibt der
verlustfreie Fallback für tokenlose Alt-Manifeste (Claude: B als Writer-seitige Ableitung beim
Stempeln). Der Rat hält formal B; die gesamte UI-Frontier **und** die starken Open-Weight reißen ihn.
Die Entscheidung ist ein **Operator-Wort** (Wire-Kontrakt).
