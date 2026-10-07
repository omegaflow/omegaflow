<!--
  title: FMHY für die Forschungsschicht — Messung 2026-10-07
  class: survey
  date: 2026-10-07
  sha256: 793e8f81139ed69e9b22f4ea338ea32016826f0e95ed67e631e4e4db278dc873
  status: live
  see-also: docs/concepts/tools-map.md state/future/research-chatbots-2026-10-07.md
-->
# FMHY für die Forschungsschicht — Messung 2026-10-07

Ziel: FMHY (`fmhy.net`, FreeMediaHeckYeah) als **Quellen-Discovery** für die
Recherche-Schicht prüfen — was davon ist für omegaflow (Astrophysik, Space Weather,
GIC, Transfer-Entropie, freie Stimmen) brauchbar. Ein erster Schnitt; der Rest der
FMHY-Wiki ist noch zu vermessen (Nächste Messung unten).

## Gemessen (2026-10-07, Rohquellen via `curl`)

- `fmhy.net/ai#specialized-chatbots` — Rohquelle `docs/ai.md` (46 KB) und Live-Anker
  (176 KB; `h3 id="specialized-chatbots"` Z. 1969–2172).
- `fmhy.net/reading#academic-papers` — Rohquelle `docs/reading.md` (83 KB).
- `fmhy.net/educational` — Rohquelle `docs/educational.md` (183 KB).

## Befund — brauchbar für uns

**Recherche-Chatbots:**
- **alphaXiv** (`alphaxiv.org`) — Paper-QA über arXiv. **Gebaut:** `archive_search
  --alphaxiv <query>` (MCP `https://api.alphaxiv.org/mcp/v1`, Tool `discover_papers`;
  public `2c12d4707`). Web-UI ohne Login.
- **Elicit** (`elicit.com`) — **API nur ab Pro** ($49/Monat; `pricing` gemessen);
  Basic gratis: 138 M Paper, Paper-Chat mit Volltext. Deshalb **nicht** verdrahtet.
- **Sakana Chat** — Browser, Modell „Namazu", Sign-in nötig.
- **SciSpace** — CloudFront-403, nicht erreichbar (auch US/JP-Exit + Browser).
- **Sci-Bot** — Sci-Hub-Frontend; **nicht nutzen** (saubere Wege: Open Access,
  arXiv/Repositorien, Autor:innen-Anfrage, Fernleihe).

**Literatur-Werkzeuge** (über die `archive_search`-Arme `--arxiv/--crossref/--openalex/
--semanticscholar/--pubmed/--europepmc/--core/--zenodo/--unpaywall/--doaj` hinaus):
Connected Papers, LitMaps, Open Knowledge Maps, Citrus Search, Ai2 Asta, Consensus,
Retraction Watch, PubPeer, Internet Archive Scholar, OpenAire, OA.mg, Lens/Dimensions,
re3data, DataONE, arxivxplorer, searchthearxiv, soarxiv.

**educational-Ergänzungen:** Manim (erklärte Mathe-Videos), LabPlot (wissenschaftlicher
Plotter), OEIS, WolframAlpha, ProofWiki, nLab; Astronomy: `spaceweather.gov`, SolarHam,
CelesTrack, Aladin, ESAsky, ExoplanetArchive, EarthData, Copernicus Browser, GOES-16;
Spacecraft: Gunter's Space Page, Encyclopedia Astronautica, Sven's Grahn, JPL
Photojournal; Study/Research: ResearchRabbit, Inciteful, Publish or Perish, GROBID
(PDF-Metadaten-Extraktion), OpenRefine, Co-STORM.

## Rest-Messung (2026-10-07, ausgeführt)

Die genannten Anker sind vermessen (`docs/misc.md` · `educational.md` · `ai.md` ·
`text-tools.md`, 2026-10-07 via `curl`; `archive_search --verdict` je Kandidat, 26 gemessen):

- `misc#satellite-earth-data` (Z.608) · `edu#aerospace-engineering` (Z.507) ·
  `text-tools#latex-tools` (Z.493) · `text-tools#note-taking` (Z.197) — vorhanden.
- **`ai#machine-learning` existiert nicht** — `docs/ai.md` trägt nur Chatbots/Image/
  Audio/Agents, kein ML-/Dataset-Kapitel.

**Stärkste neue Kandidaten** (registerleer, an Mountain/Mycelium geroutet):
- **KCG2** `https://prop.kc2g.com/` (200) — Ionosphären-/HF-Ausbreitungskarte, GIC/TEC-nah.
- **SDO Dashboard** `https://sdo.gsfc.nasa.gov/data/dashboard/` (200) — Sonnen-EUV-Livefeed
  (vor Kandidatur gegen `spdf.gsfc.nasa.gov` gegensuchen).

Werkzeugseitig (LaTeX/PDF-Kette, registerleer): **Typst** (`typst.app`, Rust), **SimpleTex**
(`simpletex.cn`), **LaTeX-OCR** (`lukas-blecher.github.io/LaTeX-OCR`). Astronomie-Viewer:
WorldWide Telescope (`worldwidetelescope.org`), In-The-Sky (`in-the-sky.org`). CelesTrack
`pending` (keine Stufe antwortet). Nicht-neu: Earthdata/FIRMS/SWPC/Exoplanet Archive/ESAsky/
NTRS gehalten; Sentinel Hub/CDSE/`api.nasa.gov`/Macrostrat declined/dead.

## Quellen

- `raw.githubusercontent.com/fmhy/edit/main/docs/ai.md` · `docs/reading.md` ·
  `docs/educational.md` (measured 2026-10-07 via `curl`)
- `fmhy.net/ai#specialized-chatbots` Live-Anker (measured 2026-10-07 via `archive_search --playwright`)
- `elicit.com/pricing` (measured 2026-10-07 via `archive_search --playwright`)
- `api.alphaxiv.org/mcp/v1` — 401 ohne Auth (measured 2026-10-07); `--alphaxiv` public `2c12d4707`
- Adoptions- und Riss-Details: `state/future/research-chatbots-2026-10-07.md` (privat)
