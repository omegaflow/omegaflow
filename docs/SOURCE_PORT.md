<!--
  title: Source-Port: Der eine Pfad
  class: concept
  date: 2026-10-11
  sha256: 0fb3105af7ffbcc2949083782d470bfc5cdc9e0bc7a9df08cedbfd61e9d6e57d
  status: live
  see-also: docs/README.md AGENTS.md phi/sources.φ
-->
# Source-Port: Der eine Pfad

Selbsttragend. Interpretierbar von einer Session mit null Vorkontext. Dieses
Dokument ist der einzige Einstiegspunkt für alle Source-Arbeit (Grind, Port,
Kuration). Eine neue Session liest genau dieses eine Dokument.

## 1. Was die Arbeit ist

Die Source-Arbeit ist die Kuration der Quellen: eine Kandidaten-Adresse wird
gemessen (Erreichbarkeit, Dateityp, Struktur), durch das Oszillator-Gate und
die Weberin-Eignung geführt und in genau ein Register disponiert. Die
Maschinerie, die das mechanisch aus Korpora konvertierte — die alte
gold/pre-cdn-Grammatik, der `--port`-Konverter, der Probe-Sweep und das
`ledger.φ` — ist **descoped** (2026-10-11; Befund
`docs/handover/archiv/handover-2026-10-11-mountain-folge306.md`): `phi/pipeline/`
ist am Datenträger absent (gemessen 2026-10-11: `git ls-files 'phi/pipeline*'`
leer, kein Verzeichnis auf der Platte). Lebende Arme (`archive_search`) und die
Register ersetzen sie.

Der Zustand offener Arbeit liegt **nicht** mehr in einem Pipeline-Ledger: er
liegt in `state/zustand/wartend.φ` (12 pending Compiler + Zugangs-Waits;
gitignored, lokal) und wird owner-getaggt mit `register_lookup --open` gelesen.
`phi/` trägt die **4 lebenden Register** (`sources`,
`witnesses`, `footprints`, `harvest`),
kein weiteres.

## 2. Pfadkarte (gemessen 2026-10-11)

| Pfad | Was es IST |
|------|------------|
| `phi/sources.φ` | Das kanonische Register (Annahme-Ziel). 2730 `url`-Blöcke. |
| `phi/witnesses.φ` | Das Zeugen-Register (S²-Richtung, Gestalt, Presence …). 34 Einträge. |
| `phi/footprints.φ` | Die Coverage-Records `.fp01`. 5 Einträge. |
| `phi/harvest.φ` | Das Harvest-Master-Register: ein Block je Arm/Format, Felder `asset` (present/fehlt), `format`, `tag`, `arm`, `workflow`, `pattern`, `idempotent`, `timeout`; 77 Formate. `harvest.yml` liest den Block und reicht die Felder dem Arm weiter; `harvest_reg --check` whitelistet sie, ein fehlendes Pflicht-Argument verweigert der Arm selbst (exit 2). |
| `state/zustand/wartend.φ` | Das Wartend-Register (gitignored, lokal): `key \| Gegenüber \| Trigger \| Aufnehmer \| Lage`, eine Zeile je Wait; 12 pending Compiler + Zugangs-Waits. |
| `src/mathematikerin/descriptors/*.te` | Kanal-Deskriptoren (20) — Konsument `tools/measure/src/bin/field_te_query.rs`. |
| `src/archivar/testdata/` | Test-Fixtures (8 Verzeichnisse: twomass, ncei_ssi, ncei_goes_xrs, noaa_goes16, noaa_gk2a, noaa_wod, cosmic_wetprf, celestrak_eop). |
| `tools/harvest/data/` | Werkzeug-Eingaben: `asteroid/` (gm + diameters), `nrs_stations.φ`, `dust-maske.φ`. |
| `tools/register/data/bands.φ` · `tools/measure/data/scan_coverage.φ` · `tools/utils/data/library.φ` | Werkzeug-Eingaben je Konsument. |
| `docs/specs/sources-v2-spec.md` | Die Kontroll-Spec (Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln). |
| `archive-root/` | Externes Archiv (Legacy): `handover/`, `bundles/`, `concept-history/`, `omegaflow-legacy/`, `omegaflow-legacy-backup-2026-09-02/`, `vanilla-dateidocs/`, `commit_rewrite-2026-09-06/`. |

**Descoped (2026-10-11, am Datenträger absent):** `phi/pipeline/queue/`,
`phi/pipeline/stage/`, `phi/pipeline/research/`, `phi/pipeline/catalog/`
(`tap_index_*.φ`, die Katalog-Dumps), `phi/pipeline/ledger.φ`, `index.φ`,
`decline_lens.φ`, `library.φ`, `frame_registry.φ`, `master_urls.txt`,
`master.φ`, `prompt.φ`, `MANIFEST.φ`, `leads.φ`, die `probe_*`-Entwürfe und die
statischen Katalog-Dispositionen. Diese Pfade stehen nicht zur Verfügung; wo
eine Messreihe gebraucht wird, tragen git und die datierten Snapshots in
§12–§16 sie. Der eine lebende Katalog-Arm ist `--regtap`/`--tap` in
`archive_search`.

## 3. Die Register

`phi/sources.φ`: kanonisches Format nach SOURCES_V2_SPEC §1.0 — Blöcke
sortiert nach `ttl` (aufsteigend), dann nach `url`; keine Kommentare; eine
Leerzeile zwischen Blöcken; Direktivenreihenfolge: `url`, Blockzustand
(`ttl`/`no-cadence`, `format`, `header`, `post_body`, `at`/`on`), Fanout-Familie, dann
Extracts in Abhängigkeitsreihenfolge. `no-cadence` trägt eine Quelle ohne
natürliche Kadenz (ein statisches Archiv): der Parser setzt den Guard 2²⁵ s ≈ 388 d
(reservierter Wert), das Wort überlebt den Round-trip; eine explizite `ttl` dieses
Werts liest sich als `no-cadence`.

Die drei früheren Dispositions-Register (`dead_sources.φ`/`declined_sources.φ`/
`blocked_sources.φ`) sind mit dem Architektur-Akt (Operator-Wort 2026-10-11) aus
dem Kanon entfernt: `phi/` trägt nur noch `sources`/`witnesses`/`footprints`/
`harvest`. Die Verdikt-, `dead`- und Zugangs-Zustände leben nicht mehr als
eigene Register; die Zugangs-/Verdikt-Grenze (Operator-Wort 2026-09-15) bleibt
als Prinzip in `AGENTS.md` (Source-Curation) beschrieben.

**Der Konsument ist kein Kriterium der Grenze.** Die Presence bewegt sich frei
durch den 4D-Block: eine physikalische Messung ist intrinsisch wertvoll, auch
wenn heute kein ω()-Konsument sie liest. „kein gebauter Konsument" ist eine
**Bau-Reihenfolge**, keine Quellen-Identität.

## 4. Der Zustand — wartend.φ + register_lookup --open

Es gibt keine Pipeline-Zustandsmaschine mehr (`ausstehend`/`verifiziert`/
`kompiliert`/`void`/`parser-gap`/`disponiert` sind mit `phi/pipeline/ledger.φ`
descoped, 2026-10-11). Der Zustand lebt an zwei Orten:

- **Die Register selbst** sind das Ledger: eine entdeckte Quelle ist bis zur
  Disposition `pending` (Ernte/Prüfung ungebaut), danach steht ihr Verdikt in
  `phi/sources.φ`. Die drei früheren Dispositions-Register sind entfernt.
- **`state/zustand/wartend.φ`** trägt das Warten: eine Zeile je Wait,
  `key | Gegenüber | Trigger | Aufnehmer | Lage`. Der Aufnehmer arbeitet die
  Zeile im selben Atom, in dem der Trigger feuert. Die 12 pending Compiler
  (copernicus-insitu ×8, noaa ×4) und die Zugangs-Waits stehen dort.

`register_lookup --open` ist die Queue: es liest die Zustands-Register
owner-getaggt (`parser-gap`/`asset fehlt` → mountain,
`pending` → mycelium, released → released) und legt die Zeilen dem Owner vor.

## 5. Der eine Pfad (Prozedur)

Pro Quelle, in einem Atom:

1. **Messen.** Bekannte URL → `archive_search --verdict <url>` (Leiter direkt
   → Proton-Exit → Wayback) und `archive_search --sniff <url>` (Magic-Bytes +
   sha256). JS-gerenderte Seite → `--playwright <url>`. PDF → `--pdf-text`,
   sonst `--pdf-image` + vision. Unbekannte Quelle → die Suchmaschinen
   (`--tavily`/`--exa`/`--linkup` keyed, `--mwmbl`/`--marginalia` keyless);
   `--all` ist die letzte Bewegung (84 Arme, sequenziell — die letzte, nie die
   erste). Die Arm-Landschaft liest man aus `archive_search --help` (der Kanon),
   nicht aus diesem Dokument.
2. **Struktur prüfen.** 200er-HTML ist kein Daten-JSON; Binär-/Zip-Inhalt
   wird mit `curl` + `unzip`/Reader geprüft, nie als JSON-Fetch.
3. **Gate.** Oszillator-Gate (§8), τ-Bestimmung (§8), Weberin-Eignung (§8).
4. **Disponieren.** `accepted` → `phi/sources.φ` (Format §1.0, plus
   `url`/`origin`/`compiler`/`sha256`; Manifestation via `*-cdn.yml`).
   Eine abgewiesene Quelle (`decline`/`dead`/Zugangssperre) wird als gemessener
   Befund getragen (Handover-Register), nicht in ein eigenes Register
   geschrieben — die drei Dispositions-Register sind 2026-10-11 entfernt.
   Endpunkt tot → erst die tote-Endpoint-Recherche
   (§9), dann `dead` mit `note`, die den Recherche-Stand nennt.
5. **Manifestieren.** Eine neue/geänderte Quelle wird in `phi/sources.φ`
   registriert, damit der CI-Manifestator (`*-cdn.yml --ci-mode`) das Asset auf
   das CDN bringt (Session-Duty; ein lokal geerntetes, unmanifestiertes
   Dataset ist eine Register-Schuld). Der CDN-Upload läuft nur mit `--ci-mode`
   — ein lokaler Lauf allein speist das gemeinsame Gedächtnis nicht.
6. **Schließen.** `cargo check` 0/0; der Commit ist die letzte Handlung, das
   Handover-Register im selben Schritt; Erledigtes wird entfernt.

Es gibt **keinen mechanischen Korpus-Konverter** mehr (`cargo run -- --port …`,
Probe-Sweep, Tag-Library-Linse `source_scanner`) — diese Bins und ihr
`phi/pipeline/`-Schreibpfad sind descoped (2026-10-11). Die Discovery ist
live über `archive_search`: `--leads` scannt unregistrierte Kandidaten-Heime
minus der registrierten Hosts; `--root`/`--index`/`--git` sind lokale Wege. Die
Kuration ist per Block, nicht per Korpus (die Per-Block-Kuration aus §8 und der
Proton-Exit aus §9 bleiben der Kern). Der eine Katalog-Arm ist `--regtap`/`--tap`;
`--regtap` erntet die RegTAP-Liste der TAP-Dienste, `--tap` fragt einen
TAP-Endpoint direkt. Die kanonischen Register werden nie gemirrort — nur
`--verify phi` spiegelt (CI).

## 6. Grammatik-Kurzfassung

Bindend: SOURCES_V2_SPEC §1. Die lebenden `field`-Formen: 5 Token
(`field <key> <force> <unit> <tau>`), 6 Token (mit Kernel), 9 Token
(`field <key> <name> <kernel> <force> <unit> <tau> <absorption> <advection>`).
Der 3-Token-`field` und die `force`-Direktive werden laut abgelehnt (P01).
τ = 0 schließt das Gate (kein Oszillator, 0 honored). `on <body> <lat> <lon>
[alt]` (Meter), `at <body>`, `body <body>` + `lat`/`lon`-Keys (datengetragen).
Extracts: `map/cmap/rows/flatten`, `first/last/lastrow/lastline/lastobj/
objlast/path/deep/regex/count`, `geojson/cmrpolygon/celestialpolygon/
keplermap/hapi/ephemeris/vectors`; cmap: `ra/dec/plx/pmra/pmdec/radvel/
dist/dist_scale/z`.

## 7. Referenz-Karte

| Dokument | Rolle | Status |
|----------|-------|--------|
| `docs/specs/sources-v2-spec.md` | Kanonische Grammatik, τ-Gate, Force-Unit-Registry, File-Regeln | bindend |
| `docs/specs/force-system.md` | 9 Kraftkanäle, IDs, Ausbreitungsgeschwindigkeiten | bindend |
| `docs/specs/url-templates.md` | Template-Variablen (Spatial/Temporal) | bindend |
| `docs/specs/constants.md` | Φ, c, Konstanten | bindend |
| `docs/specs/time.md` | Zeit-/Epoch-Handling | nachschlagen |
| `docs/specs/si-units.md` | Force-Unit-Matrix (SUPERSEDED als Kontrolle, physische Referenz) | nachschlagen |
| `docs/reference/NIST_SP330_tables.md`, `NIST_SP811_units.md`, `ucum-essence.xml` | Unit-Identität (BIPM/UCUM) | nachschlagen |
| `docs/concepts/parser-magic.md` | Parser-Intelligenz + offene Lücken (P05/P07/P08) | nachschlagen |
| `docs/reference/naif_body_ids.tsv` | Bodynamen für Frames | nachschlagen |
| `docs/specs/domain-coverage.md` | Host-Manifest der Grind-Wellen | nachschlagen |
| `docs/specs/parser-evaluation-matrix.md` | 4-Token-Behauptung (widerspricht P01) | SUPERSEDED |

## 8. Oszillator-Gate + τ + Klassifikation

Oszillator-Gate: `force` deklariert die physikalische Ausbreitung DER MESSUNG
selbst, nicht das Transportmedium der API. Litmus: könnte ein
nicht-menschlicher Organismus ein Sinnesorgan für diese Messung evolvieren?
Automatisch decline: nackte Zählwerte ohne Ereignis-Records, Stationslisten/
Register/Kataloge ohne Messwert, Modell-Forecasts/Reanalysen, Referenz-
Konstanten, aggregierte Indizes (skalar, positionslos), Text-Warnungen,
abgeleitete Satellitenprodukte, geographische Infrastruktur, position-only.

Der **Aktuator, nicht die Herkunft** (Rat 2026-09-15): „hergestellt" ist eine
Ontologie, keine Kraft-Kategorie — das Gate trägt den Treiber. Sensor (misst
das Feld am Ort) → accept; **passiver Treiber** (Kinematik OHNE Aktuator = das
treibende Feld: Drifter/Strömung, Asteroid/Gravitation, Ballon, Boje) →
accept; **aktives Objekt** (ein Aktuator trägt die Kinematik: Flugzeug, Schiff,
Zug, Vogel) → `decline no-physical-force` (Telemetrie eines Akteurs, kein
propagierendes Feld). Was der Akteur *ausstrahlt* (Wirbelschleppe, Lärm, Wärme,
Transponder), von einem Sensor am Ort gemessen, bleibt accept.

τ: temporal decay des Prozesses in Sekunden; schnell wechselnd → ttl/10,
stabil (Kataloge, Geologie) → ttl; explizites Prozesswissen schlägt beides.
Kernel-Default pro Force für Zeilen **ohne** Kernel-Token (Konverter-Fallback,
`sources-v2-spec.md:70-71`): em/gravity → inverse-square, acoustic/seismic-body/
diffusion → gaussian-inverse-square, seismic-surface → erfc, thermal →
exponential-decay, advective → patch-levy. Eine deklarierte Zeile bindet: der
Parser liest den Token zuerst (`parse.rs:759/1167`), der Konverter materialisiert
den Default nur im Synthese-/Entwurfspfad (`port.rs:18/1214/1237`).

Klassifikation: `accepted` → `sources.φ`; jede abgewiesene Quelle wird als
gemessener Befund getragen (Handover-Register), nicht in ein eigenes Register
geschrieben — die drei Dispositions-Register sind 2026-10-11 entfernt.

**Weberin-Eignung — die vierte Prüfung (Operator-Wort 2026-10-02).** Nach
`force`/τ/Klassifikation wird jede Quelle zusätzlich auf die *komplette*
Weberin-Eignung geprüft (`docs/concepts/die-weberin.md` §3/§4/§8). Vier Punkte,
je benannt, vor jedem `url`/`compiler`-Eintrag:

1. **Linien-/Zeugen-Art** — trägt die Quelle eine physikalische Kraft oder einen
   der drei Zeugen (S²-Richtung, räumliche Gestalt/Binding, Presence)? Ein
   Katalog/Abgeleitetes trägt keine.
2. **Zeitreihe** — trägt sie eine Reihe (τ/TE, Takens-Einbettung), oder nur einen
   Einzelpunkt? Ohne Reihe gibt es kein Gewebe (Vlies/TE), nur einen Zettel.
3. **4D-Anker** — ICRS/TDB + Körper/Station? Ohne Rahmen keine Weltlinie.
4. **Zweite unabhängige Linie** — gibt es sie auf derselben Größe, oder läuft nur
   *eine*? Eine einzelne Linie ergibt `absent`/DirectionOnly, nie Placed. Ein
   Kanal ist **nicht** „gedeckt", solange nur eine Linie läuft; die fehlende
   zweite Linie ist ein Register-Punkt (`pending`/`blocked`), nie stillschweigend
   „gedeckt".    Die Auswertung ist gebaut:
   `tools/measure/src/bin/weberin_verdicts_compiler.rs` +
   `src/archivar/weberin_verdicts.rs`.

**Toter Endpoint ist kein Endzustand.** Funktioniert der Endpoint nicht, wird
erst recherchiert: alternative Endpoints, URL-Änderungen (API-Versionen,
Redirects, Pfad-Renames) und Misspellings. `dead 404/400/5xx/dns/timeout`
ist ein Recherche-Auftrag, keine Disposition. Wirklich declined sind nur
drei Fälle: (a) der Anbieter ist komplett abgeschaltet (kein öffentlicher
Nachfolger), (b) die Quelle ist nicht-physikalisch (Oszillator-Gate), (c) der
Anbieter ist kommerziell (Bezahl/Proprietär-Zugang). Erst wenn die
Recherche leer bleibt, wird `dead` mit `note` festgeschrieben, die den
Recherche-Stand nennt (Alternativen geprüft, Fund: keine).

## 9. Fix-Rezepte + API-Fakten (hart erworben)

- `map` braucht datengetragene Position: `lat`/`lon`-Keys oder GeoJSON
  `geometry.coordinates.N`. Positionslose Feeds → skalar (`last <dot.path>`,
  `lastrow`, `lastobj`) an einem Frame.
- `map`-Extract mit nicht-surface Frame (`at sun`) ist eine Fehlkennzeichnung
  — Migration hatte `wgs84`-Blöcke falsch gelabelt; surface-Frame heißt
  `on <body>`.
- ArcGIS: HTTP 200 mit `{"error":{"code":400}}` bei unbekannten outFields →
  `outFields=*` zuerst; Schema-Epochen wechseln (`_1_` vs `_0_`-Felder).
- Open-Meteo: Split-Hosts (`archive-api.`/`flood-api.`/`marine-api.`/
  `air-quality-api.`/`ensemble-api.`), nie `api.open-meteo.com/v1/<x>/v1/<x>`.
- TAP: `FORMAT=csv/json` nicht überall; VOTable-only-Kataloge = parser-def;
  `SELECT TOP 1 *` vor jedem Query; TAPVizieR kennt kein OFFSET/LIMIT.
- HAPI: BGS braucht `format=json` + `Z`-Suffix an Timestamps; VirES kennt
  `id=/start=/stop=` UND `dataset=/time.min=`; Fenster innerhalb der
  Abdeckung wählen (`{yesterday}`-Fenster + ttl 86400 bei Lag).
- ERDDAP: Constraint-Syntax des jeweiligen Servers prüfen (`[(last)]`-Formen
  400en heute oft).
- Header-Werte sind whitespace-gesplittet (keine Spaces); Secrets als
  Einzel-Marker (`header user-agent {TNS_UA}`) — Braces nicht mischen.
- Refused-Befunde des Ladens sind Source-Befunde, nie Parser-Fixes:
  `pos without body directive` → Block trägt `body <body>`; `no reference
  frame` → Block braucht `on`/`at`.
- **Auth ist kein Hindernis** solange die Registrierung frei, public und
  nicht-kommerziell ist (NASA Earthdata, NOAA, Copernicus, GBIF, …). Der
  Parser greift bereits auf `.secrets.local` zu — Key dort eintragen, als
  `{MARKER}` in `url`/`header` referenzieren;   `resolve_secret`/`render_headers`
  lösen ihn auf. Kein Code nötig. Kommerzielle/private Keys bleiben declined.
  (Die frühere Re-review-Liste `phi/pipeline/review_kandidaten.txt` ist mit
  `phi/pipeline/` descoped, 2026-10-11.)
- **Der Proton-Exit ist die Netz-Ebene der Eskalation** (seit 2026-09-13 auch
  userspace ohne root, siehe unten): re-routet die eigene Exit-IP gegen
  Geo-/IP-Block. Reihenfolge: direkte Route → Proton-Exit → WebArchive.
  Gemessen 2026-09-05
  (Tunnel oben): Lasair `lasair.lsst.ac.uk` 200, API 401 = Token-gate
  (`LASAIR_TOKEN` in `.secrets.local`), `data.lsst.cloud` 200 —
  erreichbar; `rubinobservatory.org` scheitert am TLS-Handshake
  (SSL_ERROR_SYSCALL) auch mit Tunnel oben — VPN öffnet nicht jede Tür,
  der Eintrag bleibt bis ein Verdikt steht. Gemessen 2026-09-13 (Exit
  `proton-us`, US-FREE#81): `rubinobservatory.org` **200**, `data.lsst.cloud`
  **200**, CADC (`ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca`, TAP `/argus/sync`)
  **200**, NED **200** — der US-Exit öffnet Rubin, das am alten Tunnel
  scheiterte. Hostnamen korrigiert (gemessen 2026-09-13): `cadc-ccda.…` (ohne
  `ws.`) ist NXDOMAIN, `www.cadc-ccda.…` trägt die Site (200), der TAP läuft
  auf `ws.cadc-ccda.…` (400 ohne Query = lebt); `www.rubinobservatory.org`
  resettet TLS, `rubinobservatory.org` (ohne www) ist 200; `arvo.byu.edu` ist
  NXDOMAIN (tot) — der ARVO-TAP ist `arvo-registry.sci.am` (000, reset by peer).
- **Länder-Exits (free plan, gemessen 2026-09-13)**: Der `protonvpn`-CLI
  wählt im Free-Plan **kein Land** („Location selection is not available on
  the free plan") — `protonvpn connect` nimmt den schnellsten freien Server.
  Länder-Exits laufen über **WireGuard-Configs**: auf
  `account.protonvpn.com/downloads` je Land (JP CA MX NL NO PL RO CH SG US)
  eine kostenlose Server-Config herunterladen → `~/.config/wireguard/proton-<cc>.conf`
  (z. B. `proton-us.conf`; `$PROTON_WG_DIR` überschreibt) → `bin/proton-exit.sh <cc>`
  (bringt `wg-quick up`, meldet die Exit-IP; `off`/`list`/`status`). Alle Proton-
  Configs teilen `Address = 10.2.0.2/32` → es läuft **ein Exit zugleich**, der
  Helper nimmt die anderen vorher runter. Damit ist die Leiter nicht auf
  einen Tunnel festgelegt — pro Geo-Block ein anderes Land (US, NL, CH …).
  `wg`/`wg-quick` und der `protonvpn`-CLI sind installiert (Login des
  Operators, lokal); `dnsforge.de` ändert die Exit-IP nicht und trägt
  gegen Geo-/IP-Blocks nichts bei. **Gemessen 2026-09-13** (je Exit ein Test):
  `proton-us` (146.70.230.100), `proton-nl` (185.184.195.146) und `proton-ch`
  (149.88.27.156) öffnen alle Rubin/LSST/CADC/NED — **NL und CH sind schneller
  als US** (Rubin ~0,5–0,6 s vs ~2 s; CH: GAVO 0,45 s, CERN 0,42 s, ESA 0,20 s).
  Die `--verdict`-Leiter liest die aktiven `proton*`-Interfaces aus
  `/sys/class/net` und probiert sie in Stage 2 (nicht mehr fest `proton0`).
  `proton-jp` (149.88.103.44): JAXA/JMA 200, Kyoshin 302, Hi-net
  `www.hinet.bosai.go.jp` 200 (`hinetwww11.bosai.go.jp` nur 403, der
  `www.`-Vorsatz löst nicht auf). `proton-ca` (195.242.214.197): CADC 200,
  CANFAR 200. Fünf freie Länder-Exits (US NL CH JP CA), alle gemessen.
- **Userspace-Exit ohne root (gemessen 2026-09-13)**: Die 110 Free-Server
  liegen als WireGuard-Configs in `~/.config/wireguard/proton-free/*.conf`
  (aus der `logicals`-API; ein registriertes Client-Zertifikat gilt für alle
  Server). `bin/proton-wg.sh <server|cc>` startet `wireproxy` (userspace
  WireGuard → SOCKS5, kein sudo) und druckt `socks5h://127.0.0.1:25344`. Da
  `fetch.rs` curl ohne `--proxy`/`.env_clear()` spawnt, routet
  `ALL_PROXY=socks5h://127.0.0.1:25344` jeden Archivar-Fetch über den Exit
  (zero code). Gemessen je ein Exit pro Land (CA/CH/JP/MX/NL/NO/PL/RO/SG/US):
  alle öffnen `data.lsst.cloud`, `rubinobservatory.org` und
  `www.hinet.bosai.go.jp` mit 200 — auch dort, wo der lokale Exit TLS resettet
  oder Tor-Exits geblockt sind.
- **Toter Endpoint → Recherche-Rezept**: (1) Status-/Docs-Seite des Anbieters
  prüfen (Umzug, API-Version), (2) Sibling-Endpoints desselben Netloc,
  (3) URL-Pfad auf Versions-Bumps/Renames, (4) Misspelling gegen den
  Provider-Namen, (5) Proton-Exit für Netzwerk-Blocks. Nur wenn all das leer
  bleibt → `dead` mit `note`, die den Recherche-Stand nennt.
- Der `--port`-Konverter (descoped 2026-10-11 mit `phi/pipeline/`) übernahm
  `url/format/header/target/catalog` direkt, `ttl`/`no-cadence`, `on/at`,
  numerisches `lat/lon/alt` → `on earth`, `map/cmap/rows`, `lat_key/…` →
  `lat/lon/alt/epoch`, `field/field_in/…` → 9-Token-`field`, `last_row`→`lastrow`
  usw. **Er fiel**: `source`, `method`, `body` (POST), `pos`, unbekannte
  Direktiven. Die Grammatik-Fakten der 9-Token-`field`-Form bleiben lebend und
  stehen in §6 + `docs/specs/sources-v2-spec.md`; der Konverter-Pfad selbst ist
  absent.
- **Session 2026-08-19 — Teleskop-Endpoints (Wayback + curl verifiziert,
  volle Log §13):** `nhsa.esac.esa.int` tot → Herschel-Archiv lebt unter
  `archives.esac.esa.int/hsa` (HAIO: `/hsa/aio`); `svom.nscs.ac.cn` tot →
  Datenzugang `svom.ac.cn` (HTML, Zertifikat abgelaufen), GRB-Notices via GCN;
  `www.lhaaso.ac.cn` tot → Site `english.ihep.cas.cn/lhaaso`, Daten-Seite
  `/lhaaso/pdl` = statische News-Seite von 2021 (kein Datenzugang — Decline als
  Datenquelle); CHIME-Portal lebt (Umbau), Kataloge via CANFAR-DOIs
  (CISTI.CANFAR/21.0007, 23.0004, 10.11570/23.0029, 25.0066 — Archiv-Records,
  keine API); NRAO-Archiv = Angular-SPA unter `data.nrao.edu/portal/`,
  archive-service-REST-Pfade 404 (alter TAP bleibt parser-def votable).
  Erreichbar: MAGIC-Portal `magic.mpp.mpg.de/public/public-data/` (HTML + FITS,
  TLS-Kette unvollständig), HAWC `data.hawc-observatory.org/datasets/`
  (HTML + FITS, TLS-Kette unvollständig), uGMRT `naps.ncra.tifr.res.in`
  (NAPS, CAPTCHA), AGILE `agile.ssdc.asi.it` (MMIA, AGILE-LV3, Kataloge
  2AGL/MCAL-GRB/TGF).
- **GCN-API v0.1 ist tot** (2026-08-19 verifiziert): `api/v0.1/circulars`
  → 404 Remix-SPA. Circulars jetzt nur SPA-Suche + `/circulars/{id}.json`
  (JSON mit body-Freitext, KEINE ra/dec). Kein Listen-Feed gefunden — die
  `gcn_einstein_probe_*`-Blöcke im (descopten) `master.φ:27360-27385` sind
  stale (parser-gap). TAP-Formen (verifiziert): ESO `archive.eso.org/tap_obs/sync`
  (nicht `/tap_obs/tap/sync` — 404) → csv; ESA-TAP-Server nur unter
  `eas.esac.esa.int/tap-server/tap` (AMA `archives.esac.esa.int/tap-server/tap`
  → 404); SARAO `/tap/sync` → Portal-HTML (kein TAP); ALMA TAP lebt
  (`almascience.org/tap/sync`, 302→eso.org), nur VOTable — parser-def
  bestätigt; MAST Pan-STARRS: `catalogs.mast.stsci.edu/api/v0.1/panstarrs/
  dr1/<table>.json?ra=&dec=&radius=` (nicht `/panstarrs/detections` — 404);
  LAMOST: `dr11.lamost.org` dns-tot, `www.lamost.org/dr11/` HTML-Portal;
  IRSA-TAP trägt `spherex.obscore` (s_ra/s_dec); Rubin-TAP 401 (OAuth,
  blocked bestätigt); ATLAS-Forced-Photometrie = Formularseite
  `fallingstar-data.com/forcedphot/` (bootstrap, Token nötig); eROSITA DR2 =
  HTML-Landing `erosita.mpe.mpg.de/dr2/`. Die Befund-Dateien lagen unter dem
  (descopten) `phi/pipeline/research/agent_output/verify_astro{,_b}_2026-08-19.φ`;
  die Messreihe trägt git.

## 10. Secrets

`.secrets.local` (Repo-Root, gitignored, gemessen 2026-10-11: 165 Keys via
`bin/secrets_keys`) — geladen via `resolve_asset` (CWD-relativ). Probe-Läufe
außerhalb des Repo-Roots brauchen einen Symlink. `{MARKER}` in URLs wird
case-insensitiv gegen die UPPERCASE-Env-Vars aufgelöst; absent → void + stderr.
Nur die **Schlüsselnamen** werden je gelesen, nie die Werte (Secret-Hygiene).

## 11. Regeln

- Name = Implementation. Keine Kommentare in φ-Dateien; Wissen lebt im
  Register und im Handover.
- Ein Commit = ein Häkchen: das Handover-Register im selben Commit; Erledigtes
  wird entfernt.
- `cargo check` 0 Errors, 0 Warnings; `cargo check` prüft nur Syntax — die
  Verifikation ist am Baum (die Session ist der Verifikator).
- Die Session ist das Atom: eine Klasse von Quellen pro Session durch die
  Prozedur (§5) — kein halber Port.
- 0 honored: eine leere/absente Diagnose ist eine vollständige Disposition,
  kein Fehler. Absent ist erst nach Recherche absent (`pending` bis dahin).
- Zugangs-/Format-Sperren (2026-08-19, verbindlich): Auth-Formen —
  `key` (API-Key/Ticket), `account` (Konto/Login/
  Registrierung), `ip-blocked` (Bot-/IP-Sperre), `parser-def`
  (Format unkonsumierbar, trägt immer den Dateityp html/json/xml/csv, nie
  nackt). Sie sind gemessene Befunde, kein eigenes Register: Key-/Account-Waits
  stehen als Zeile in `state/zustand/wartend.φ` (Aufnehmer-Pflicht), der Rest
  als Handover-Punkt.
  Mirror-Gate: nur Quellen bleiben, deren Daten gespiegelt werden dürfen (CDN)
  — Open-Data, Regierungs-Open-Data, Forschungsdaten → bleibt, Mirror-
  Einordnung in der `note`; kommerzielle ToS/Redistribution →
  `decline redistribution`. Kaskaden-200 (Proton-Exit/
  Wayback) = entblockt → der Befund wird ein `phi/sources.φ`-Eintrag über die Prozedur (§5). `register_lookup --open` ist die
  Queue — es liest die Zustands-Register owner-getaggt (`parser-def`/
  `asset fehlt` → mountain, `pending` → mycelium, released → released) und
  `state/zustand/wartend.φ`; es gibt keinen `phi/pipeline/`-Draft-Pfad
  (descoped 2026-10-11).
- Grenze Zugang/Verdikt (Operator-Wort 2026-09-15): **kommerziell →
  `decline redistribution`** (nicht Zugangs-Sperre — der Zugang ist nicht gesperrt,
  das Verdikt ist kommerziell); **der Konsument ist kein Kriterium** — die
  Presence bewegt sich frei durch den 4D-Block, eine physikalische,
  zugängliche Messung gehört ins Register (der Konsument ist eine
  Bau-Reihenfolge, kein Quellen-Verdikt).
- **Authentifizierung ist kein Ausschlusskriterium** (Operator-Wort 2026-10-08):
  ein `key`/`account` ist eine `pending`-Operator-Aufgabe, kein
  Quellen-Verdikt — wo der Operator zugangsberechtigt ist oder ein Key besteht,
  führt die **Auth-Route** zur Quelle, nie ein `decline`.

## Anhang — datierte Recherche-Snapshots

> **Hinweis (2026-10-11).** §12–§16 sind datierte Snapshots aus den Sessions
> 2026-08-19 und 2026-09-10, konserviert als Messreihe (die Messung gehört der
> Zukunft). Die darin genannten `phi/pipeline/...`-Pfade (`queue/`·`master.φ`,
> `research/agent_output/`, `ledger.φ`, `catalog/`) sind **descoped und am
> Datenträger absent** (gemessen 2026-10-11); die genannten Kandidaten-URLs und
> Endpoint-Fakten bleiben als Messung gültig und sind über `archive_search`
> nachprüfbar. Der laufende Pfad ist §1–§11.

## 12. Teleskop-Inventar (Session 2026-08-19)

Recherche-Auftrag: alle aktiven terrestrischen + extraterrestrischen
Teleskope mit ihren APIs. Status-Legende: ✓ aktiv in `sources.φ` · ▣
Kandidat in `phi/pipeline/queue/master.φ` · ⛔ in `blocked_sources.φ` ·
● offen/unkuratiert · `pending` = Detail unverifiziert — der nächste
Schritt ist Probe/Klassifikation am vorliegenden Befund, keine neue
Adress-Recherche (0 honored). Oszillator-Gate-Schnellurteil: alle
elektromagnetischen Teleskope messen `em`; GW-Detektoren `gravity`;
CR/ν-Detektoren (Auger, TA, IceCube, KM3NeT, Super-K/JUNO) — Teilchen
ist Abstammung, nicht Kraft (die-weberin.md:104-109): gemessen wird das
em-Schauer-/Cherenkov-Licht (`em`); die Teilchen-Art (Proton, Neutrino,
Gamma) ist die Wurzel im Abstammungs-Feld, keine zehnte Kraft — ein
„particle"-Bit wäre Fabrication. Die 9er-Force-Registry trägt kein
Teilchen-Medium.

### 12.1 Weltraum — Röntgen/Gamma

| Mission | Endpoint | Status |
|---------|----------|--------|
| Chandra | Chaser/Browse (`cxc.cfa.harvard.edu`) | ● |
| XMM-Newton | XSA-TAP (`nxsa.esac.esa.int`) | ● |
| NuSTAR | HEASARC-Browse | ● |
| Swift | ✓ Swift-GRB aktiv; weiteres via HEASARC | ✓/● |
| NICER | HEASARC | ● |
| MAXI | `maxi.riken.jp` / DARTS | ● |
| IXPE | HEASARC | ● |
| XRISM | DARTS | ⛔ |
| Einstein Probe | ESA-AMA-TAP + GCN | ● Lücke |
| SVOM | `svom.ac.cn` + GCN (§13) | ● |
| Fermi | FSSC-LAT | ⛔ |
| INTEGRAL | ISDC | ● |
| AGILE | `agile.ssdc.asi.it` (§13) | ● |
| AstroSat | ISSDC AstroBrowse | ● |
| HXMT/Insight | `hsuc.ihep.ac.cn` / DARTS | ● |
| eROSITA | `erosita.mpe.mpg.de`, DR2 (31.07.2026) | ● Lücke |
| Athena | Start unverifiziert — kein Bestand | ● |

### 12.2 Weltraum — Optik/IR/Astrometrie

| Mission | Endpoint | Status |
|---------|----------|--------|
| Gaia | ESA-TAP + ARI; `dr3_stars.bin` | ✓ |
| JWST | MAST-CAOM-TAP (7 `mast_jwst_*` im Queue, §14) | ▣ |
| HST | MAST-CAOM-TAP | ● |
| TESS | MAST | ✓ |
| Euclid | Cosmos-TAP | ● Lücke |
| SPHEREx | IRSA-TAP | ● Lücke |
| CHEOPS | ESA-AMA-TAP | ● Lücke |
| Planck/Herschel-Legacy | PLA/IRSA; HSA-Umzug §13 | ● |
| Roman | Start 30.08.2026 — noch kein Bestand | ● |
| PLATO | Start 2027 — kein Bestand | ● |

### 12.3 Weltraum — Sonne/Heliosphäre

SOHO (VSO) ● · SDO (VSO/JSOC) ● · Parker ✓ (SPDF-HAPI aktiv) ·
Solar Orbiter ✓ Ephemeride aktiv, SOAR ● · STEREO (VSO) ● · Hinode
(VSO/DARTS) ● · IRIS (LMSAL/VSO) ● · Aditya-L1 (ISSDC) ● · GOLD ● ·
GOES/SWPC ✓ aktiv.

### 12.4 Boden — Optik/IR

Rubin/LSST ⛔ (TAP `data.lsst.cloud`, OAuth) · DESI ● · ESO
VLT/VISTA/VST ● (`archive.eso.org/tap_obs` + `tap_cat` — Lücke) · Subaru
SMOKA ● · Keck KOA-TAP ● (Lücke) · Gemini ▣ (`archive.gemini.edu`,
master.φ 40567) · NOIRLab AstroArchive ● · Pan-STARRS ● (Katalog-API —
Lücke) · ZTF ✓ (ALeRCE/Lasair aktiv; IRSA ●) · ASAS-SN ● (Skynet) ·
ATLAS ● (Forced-Photometrie `fallingstar.com` — Lücke) · SDSS ✓,
SDSS-V ● · LAMOST DR11 ● (Lücke) · 2MASS/WISE ⛔ (Gator/allwise) · ELT ● (kein Bestand, Start unverifiziert).

### 12.5 Radio

ALMA ⛔ (TAP) · NRAO VLA/GBT/VLBA: alter TAP tot, neuer REST
`data.nrao.edu/archive-service/restapi_*` ● · MeerKAT/SARAO ●
(`archive.sarao.ac.za/tap/` — Lücke) · CASDA ⛔ (ATCA/ASKAP/Parkes) ·
LOFAR ● (LTA) · FAST ● · CHIME: FRB-API tot → CANFAR-DOIs ● · HERA ● ·
MWA ● (ASVO) · uGMRT ● (NAPS, §13) · EVN/JIVE ● · e-MERLIN ● · MOJAVE ✓ · SKAO ● (im Bau — kein Bestand, Start unverifiziert).

### 12.6 Hochenergie/CR/ν/GW + Sonne (Boden)

H.E.S.S. ● (Endpoint offen — nicht im Befund) · MAGIC ● (Portal §13) ·
VERITAS ● (Endpoint offen — nicht im Befund) · HAWC ● (Datasets §13) ·
LHAASO ● (Site §13; Bulk-Download unverifiziert — Klassifikation am
Befund) · CTAO ● · Pierre Auger ● (`opendata.auger.org`) · Telescope
Array ● · IceCube ● (Daten-Releases) · KM3NeT ● · Super-K/JUNO ● ·
LIGO/Virgo/KAGRA ✓ (GraceDB aktiv) · LISA ● (Bauphase — kein Bestand, Start unverifiziert) · GONG ⛔ · DKIST ●
(`api.dkistdc.nso.edu`, Umzug von `api.nso.edu`).

### 12.7 Größte Lücken (Befund)

SPHEREx IRSA-TAP (offen), eROSITA DR2, ESA-AMA-TAP (Einstein Probe +
CHEOPS), Euclid-TAP, ESO-TAP, ALMA-TAP, MeerKAT-TAP, Rubin-TAP,
Pan-STARRS-Katalog-API, ATLAS-Forced-Photometrie, Keck-TAP, LAMOST DR11.
Stale Register-Einträge: NRAO-TAP (retired), CHIME-FRB-API (retired).

## 13. Web-Archive-Verifikation (Session 2026-08-19)

Auftrag: nicht erreichbare Adressen mit Web-Archive-Recherche prüfen.
Wayback-Fallbacks: `web.archive.org/web/2026/<url>` (404 = kein Snapshot →
Websearch löste die Domänen-Frage).

### Agenten-Rezept (ab Welle 2026-08-19, verbindlich)

Jeder Recherche-Agent erhält: (1) den Pfad zur Secret-Datei
`.secrets.local` (Key dort lesen, nie ausgeben), (2) die Anweisung,
JEDE unklare Route VIERstufig zu prüfen — direkt curl → Proton-Exit
(`bin/proton-wg.sh <cc>`; userspace wireproxy, kein root; `ALL_PROXY`
setzt die Route; die Kaskade umgeht Geo-Blockierung, Datacenter-IP-Sperren
und Rate-Limits) → WebArchive
(`http://archive.org/wayback/available?url=<domain>`, CDX-Fallback;
curl mit `-L` — http→https-Redirect) →
Websuche (das Websearch-Tool der Session) — erst nach leerer Kaskade `dead`,
(3) die Taxonomie **tot** (existiert nicht mehr; DNS-tot,
kein Snapshot) / **declined** (lebt, aber keine physikalische Messung
am Punkt — Modell, Vorhersage, Katalog, Archiv ohne Live-Feed) /
**blocked** (lebt, Zugang gesperrt — Auth/CAPTCHA/Login/IP-Sperre;
Sperr-Art benennen, „lokal vermutlich 200" bei Datacenter-Sperren;
Auth-Form nach Blocked-Kanon §11: key/account/ip-blocked + `reg`-Zeile;
Mirror-Frage stellen: Open-Data bleibt, kommerzielle ToS →
`decline redistribution`) /
**live** (200, offen) / **angekündigt** (Web-Beleg, kein Endpoint),
(4) KEINE Begrenzung des Abschlussberichts — die Recherchearbeit wird
in voller Länge wiedergegeben. Befunde: fester `kandidat`/`url`/`note`-
Block in `phi/pipeline/research/agent_output/`. Der Agent entscheidet
kein Oszillator-Gate — er klassifiziert nur, was die Messung IST.

| Adresse | Befund | Korrektur/Neufund |
|---------|--------|-------------------|
| `magic.mpp.mpg.de` | erreichbar | Portal existiert: `/public/public-data/` (FITS, Low-level-Open-Data/VO) — „kein öffentliches Portal“ war falsch |
| `data.hawc-observatory.org` | erreichbar | „Datasets“-Seite; öffentliche Datensätze + Arbeitsgruppen-Kontakt |
| `naps.ncra.tifr.res.in` | erreichbar | NAPS (NCRA Archive and Proposal System), JS-App, CAPTCHA |
| `agile.ssdc.asi.it` | erreichbar | SSDC-Datenzentrum live: MMIA-Archiv, AGILE-LV3, 2AGL/MCAL-GRB/TGF |
| `mast.stsci.edu/docs` | erreichbar | reines JS-SPA — für den Reader inhaltsleer; MAST-API-Doku liegt anderswo |
| `nhsa.esac.esa.int` | tot (422) | HSA lebt unter `archives.esac.esa.int/hsa` (HAIO: `/hsa/aio`) — JS-App |
| `www.lhaaso.ac.cn` | tot (422, Wayback 404) | offizielle Site `english.ihep.cas.cn/lhaaso`; Daten-/Code-Seite `/lhaaso/pdl` |
| `svom.nscs.ac.cn` | tot (422, Wayback 404) | Datenzugang laut NSSDC: `svom.ac.cn`; GRB-Notices via GCN `gcn.nasa.gov/missions/svom` (FSC: `fsc.svom.org/alerts`) |

## 14. JWST-MCT-Recherche (Session 2026-08-19)

Anlass: STScI-Artikel — Aufruf für JWST-Multi-Cycle-Treasury (MCT)
White Papers (>300 h, Frist 4.11.2026, jedes Thema). Auftrag: was misst
das JWST, das es als APIs gibt, die das Register noch nicht hat.

- Instrumente: NIRCam (Imaging 0,6–5 µm), NIRSpec (Multi-Objekt-Spektroskopie, MSAs), MIRI (5–28 µm), NIRISS (WFSS/SOSS), FGS.
- API-Landschaft: MAST CAOM-TAP (EDP-API, JWST-Metadata-API, HLSP, z.MAST), NExScI-TAP (`ps`/`pscomppars`, Atmospheric Spectroscopy Table AST, NExoList, Transit-Service), ESA JWST TAP, VizieR-TAP.
- Lücken: AST >1500 Spektren (hunderte JWST) = größte; JWST-Deep-Field-Photometrie/Redshifts (JADES DR5, COSMOS2025 `J/A+A/704/A339`); NExoList; EDP-Telemetrie; Metadaten geplanter Beobachtungen.
- Priorisierung: P0 AST-Spektren; P1 Deep-Field-Photometrie/Redshifts; P2 EDP/Metadaten; P3 NExoList.
- Disposition: „Nur Recherche, nichts umsetzen“ — keine Register-Schreibarbeit; die 7 `mast_jwst_*`-CAOM-Blöcke im Queue (master.φ ab 9059) sind älterer Bestand. Port-Entscheid P0 ausstehend.

## 15. Offene Posten aus der Session (hier registriert — kein Datei-Artefakt entstand)

Die Adressen sind recherchiert (Bestand §12–§13). Was aussteht, ist
Port-Arbeit an den vorliegenden Befunden — Block-Drafts, `--probe`,
Disposition — keine erneute Adress-Recherche. Stand 2026-08-19 nach
Agenten-Durchlauf (Drafts: `phi/pipeline/queue/grind_astro_tap_2026-08-19.φ`,
Befunde: `phi/pipeline/research/agent_output/verify_astro{,_b}_2026-08-19.φ`,
Ledger: 4 Einträge parser-gap):

1. SPHEREx IRSA-TAP — Draft gebaut + Probe: void — IRSA liefert VOTable-JSON (s_ra/s_dec nur FIELD-Metadaten) + 3,18-MB-Antwort → parser-gap (Parser-gap: VOTable-JSON).
2. eROSITA DR2 — HTML-Landing `erosita.mpe.mpg.de/dr2/`, kein API-Befund → pending.
3. ESA-AMA-TAP — Basis 404; nur EAS/Euclid-TAP lebt → offen (richtiger AMA-Pfad ungefunden).
4. Euclid-TAP — Draft gebaut + Probe: void — Antwortform `{metadata:[{name:…}], data:[[…]]}` (Rows als Skalar-Arrays) → parser-gap (Parser-gap: Spaltennamen-aus-metadata).
5. ESO-TAP — Draft gebaut + Probe: void — echte CSV (s_ra/s_dec verifiziert), aber Probe klassifiziert Header-CSV nicht → parser-gap (Parser-gap: CSV-Header).
6. ALMA-TAP parser-def bestätigt (VOTable-only, bleibt blocked); MeerKAT-TAP tot (Portal-Catch-all); Rubin-TAP 401 bestätigt (OAuth); Pan-STARRS — Draft gebaut (dr1 mean, 5 mag-Felder, `{ra}/{dec}/{radius}`) + Probe: void nur wegen Probe-Env-Marker (Endpoint lebt) → parser-gap, Nachweis im Register-Lauf offen; ATLAS-Forced = Formularseite mit Token (key-needed-Kandidat); Keck/KOA — keine URL im Bestand → offen; LAMOST — `dr11.lamost.org` dns-tot, `www.lamost.org/dr11/` HTML-Portal.
7. NRAO — Befund: Angular-SPA `data.nrao.edu/portal/`, REST-Pfade 404 (Ledger: parser-gap).
8. CHIME — Befund: Portal lebt, Count-Extract leer, Kataloge via CANFAR-DOIs (Ledger: parser-gap).
9. SVOM — GCN-v0.1-API tot, kein Listen-Feed → Block-Draft nicht möglich (Ledger: parser-gap).
10. LHAASO — Klassifikation vollzogen: `/lhaaso/pdl` = News-Seite 2021, kein Datenzugang → Decline als Datenquelle.
11. MAGIC/HAWC — HTML+FITS-Portale (TLS-Ketten unvollständig, `-k` nötig) — FITS nicht konsumierbar → pending/parser-def.
12. Herschel — HSA-Umzug in §13 nachgezogen (erledigt).
13. JWST P0–P3 — Port-Entscheid (§14) weiter offen.
14. Zukünftige Missionen (SKAO, LISA, ELT, Athena) — im Inventar §12 registriert, Startdaten unverifiziert, kein Bestand → kein Port, bis Daten existieren.
15. Sensor-Kategorien-Welle (2026-08-19): 10 Agenten (Satelliten, Flugzeuge, Drohnen, Raumstationen, Radiosonden, Bojen, Wetterstationen, Labore, Unterwasser, Sonstiges) + 1 Nachprüf-Agent (Wayback) — Befunde: `phi/pipeline/research/agent_output/{satellites,aircraft,drones,space_stations,radiosondes,buoys,weather_stations,laboratories,underwater,misc}_2026-08-19.φ` + `terrestrial_{atmo,geo}_2026-08-19.φ` + `classify_2026-08-19.φ`. Ergebnis nach Taxonomie tot/declined/blocked/live/angekündigt: 18 live-Kandidaten parser-gap (ledger.φ: AMeDAS, ECCC GeoMet, BfS-ODL, GTMBA, EMODnet, EMSO, IOOS-Glider, SmartBay, USGS-GW, NRCS-AWDB, IGRA, Wyoming, Iowa-RAOB, SondeHub, AWC-PIREP, COSMIC-2, IMO, GeoNet, meteo.lt); 14 blocked (blocked_sources.φ: EUMETSAT, GOSAT-GW, Airplanes.live, WeatherXM, AirQo, Sofar, IMD, KMA, SaveEcoBot, Meteomatics, CelesTrak, MeteoSwiss-Pollen, Météo-France, CTBTO — davon 3 ip-blocked, lokal nachprüfen); 5 dead/declined (dead_sources.φ: Saildrone, SatNOGS-API, TreeTalker, OSDR, WindBorne, IGRAC, AOML); 13 angekündigt (MTG-I2 27.08.2026, MetOp-SG B1, Sentinel-3C, C-130J, NASA-777, Axiom, Orbital Reef, Starlab, SOFF, ITER, SPARC, DUNE, EMSO-SMART-Cable).   Port-Arbeit der live-Kandidaten ausstehend.

## 16. Die fünf CDN-Tore (Speisekammer-Filter)

Die Frage ist nicht „gibt es die Daten?" (dann zeigt Aladin sie), sondern:
„frißt die Maschine sie — und muß sie eingefroren werden?" Fünf Tore, alle aus
dem Haus selbst gelernt:

| Tor | Frage |
|---|---|
| Frage vorher | Steht eine registrierte Frage/Frage-Sonde bereit, die diese Daten ißt? Erst die Frage, dann die Ernte. |
| Befund-Pflicht | Landet die Zahl in einem Befund? Dann sha256-eingefroren — sonst ist der Befund nicht wiederholbar, wenn upstream sich ändert. |
| Höflichkeit / Maßstab | Greift die Maschine stunden- oder tagelang zu? Einmal ernten, nicht poll — die NED-Lektion. |
| Transformiert | Existiert das Kompilat nur bei uns? Dann ist es unser Produkt, keine Kopie. |
| CI/Myzel muß fressen | Runner brauchen nackte GET-Adressen ohne Anmeldetanz. |

Alles andere: durch Aladin gucken. Gucken füllt keine Speisekammer — das
Register trägt die offenen Speisekammer-Fragen (aia2014, planck, eve, omni2,
goes15, der gebco-Stub); dieser Filter ist die Antwortregel dafür.

Tor 1 ordnet die Ernte, nie das Register (Entscheid-Folge XX, 2026-09-16): ein
gebauter Harvest wird registriert (`sources.φ` + CDN), der fehlende Konsument
ist eine Bau-Reihenfolge, kein Quellen-Verdikt; `blocked` = Zugang, `declined`
= Verdikt (unphysikalisch/Modell/Registry/kommerziell/superseded), `pending` =
Ernte ungebaut — nie „kein Konsument".

### 16.1 Endpoint-Zensus (der Wieger)

Der Endpoint-Zensus mißt je Endpoint: HTTP-Code (nackter GET), Antwortzeit,
Redirect-Ziel, Datum, und die Inhaltsprobe (`tap` = eine minimale DoQuery
antwortet TAP-JSON, `http` = HTTP antwortet, aber kein TAP — z. B. POST-only
oder CSV-only, `kein-http` = keine HTTP-Antwort). Ein 400 auf einen TAP-Root
ist „lebt, braucht eine Query" — die Inhaltsprobe ist der Puls, nicht der
nackte Code. `regtap_census import --regtap <root> [--ledger <pfad>]` erntet die
RegTAP-Liste der TAP-Dienste (Roundtrip über COUNT(*), Bestand-Dedupe je Host
+ URL) und trägt Entdecktes als `ausstehend kandidat <url>` in den Ledger.

### 16.2 RegTAP — gemessen (2026-09-10)

- `http://reg.g-vo.org/tap/sync` spricht TAP (GET sync, FORMAT=json, 149
  TAP-Dienste, COUNT(*)-Roundtrip 149=149). `https://reg.g-vo.org` schlägt fehl
  (TLS-Zertifikat deckt den Host nicht — curl 60) → http, nie https.
- Spalten heißen `rr.capability.cap_index` / `rr.interface.cap_index`
  (nicht `capability_index`); `rr.interface` trägt `access_url` + `intf_type`.

### 16.3 Vollwelle — die 120 Kandidaten gewogen (2026-09-10)

`regtap_census wave --ledger phi/pipeline/ledger.φ` (Host-Fruchtfolge, GET-Cap 30 s,
Probe-Timeout 60 s, 2 s Pause) wiegt alle `ausstehend kandidat`-Blöcke und
schreibt den Befund in die Ledger-Note zurück (`gewogen <datum>: http <code>
probe <probe>`). Verteilung des Eigenen: **49 tap · 67 http · 4 kein-http**
(120 Blöcke). Zwei RegTAP-Artefakte, getragen fürs Dispositions-Atom: zwei
Blöcke mit relativer `access_url` (`/tap`, ivoid `ivo://ovgso/tap` — kein
Host, wiegt `kein-http`) und zwei Doppel-Blöcke `https://dachs.oca.eu/tap`
(derselbe ivoid `ivo://purx/dachs/tap` zweimal — RegTAP liefert zwei
Interface-Zeilen, der Import dedupliziert nicht gegen die eigene Charge).
`tap` = lebt + TAP-JSON; `http` = HTTP antwortet ohne TAP (POST-only, CSV/
VOTable-only, oder Redirect-Landing); `kein-http` = keine HTTP-Antwort.

### 16.4 Disposition-Hygiene — Artefakte, Austräge, kein-http (2026-09-10)

- **Import-Fix (`regtap_census import --regtap`):** (a) relative `access_url` (kein
  Host) wird als Artefakt übersprungen statt als Kandidat geschrieben;
  (b) Dedupe gegen die eigene Charge (derselbe URL aus zwei Interface-Zeilen
  → ein Block). Verifikation gegen die live RegTAP: `0 candidates after
  Bestand-Dedupe (2 relative access_url, 0 batch duplicates skipped)` — die
  zwei relativen `/tap` werden übersprungen, kein Kandidat überlebt die
  Bestand-Dedupe (alle 120 sind schon im Ledger). Die 0 batch-duplicates
  sind korrekt: der dachs-Doppel liegt jetzt im Bestand, das Charge-Dedupe
  schützt die nächste Ernte.
- **Artefakte aufgelöst:** die zwei relativen `/tap`-Blöcke (ivoid
  `ivo://ovgso/tap`, kein Host) sind aus dem Ledger getragen — der RegTAP-
  Record trägt keine absolute `access_url`; das Geschwister `ivo://ov-gso/
  climso` lebt unter `https://epntap.climso.ovgso.fr/tap` (gewogen tap).
  Die zwei `https://dachs.oca.eu/tap`-Doppel sind auf einen Block verschmolzen.
- **dead_sources-Austrag (die zwei Rechecks):** `mast.stsci.edu/vo-tap/api/
  v0.1/caom/sync` ausgetragen und als gewogener Kandidat (HTTP 400 + TAP-JSON,
  lebt, braucht Query) in den Ledger zurück für die tap-Klassifikation.
  `tapvizier.u-strasbg.fr/TAPVizieR/tap/sync` → `decline superseded-by-
  integrated` (der kanonische Host `tapvizier.cds.unistra.fr` trägt die
  TAPVizieR-Quellen in sources.φ); der 501-Befund und der 400-Befund sind
  Zeitpunkte desselben Hosts, ein Eintrag trägt beide.
- **kein-http aufgelöst (4 → 0):** die zwei relativen `/tap` sind Artefakte
  (oben). `arvo-registry.sci.am/tap` → `dead unreachable` (DNS löst auf,
  http+https werden vom Peer zurückgesetzt; Proton-VPN-Route offen). Der
  CADC-ARGUS (`ivo://cadc.nrc.ca/argus`) war kein Tod: die RegTAP-`access_url`
  ist `/argus` ohne `/sync`, der TAP liegt unter `/argus/sync` und ist schon
  in `tap_index_cadc.φ` (21 Tabellen, --votable-Ernte 2026-08-20) — als
  Kandidat mit korrigierter URL zurück im Ledger (TLS-Reset 2026-09-10,
   Proton-VPN offen). Ledger-Bestand: **48 tap + 1 mast-Austrag + 1 cadc ·
   67 http**, kein kein-http offen.

### 16.5 tap-Klassifikations-Pass — die 50 Endpoints disponiert (2026-09-10)

Die 48 tap + mast-/cadc-Austrag je Endpoint durch Bestand-Dedupe, Force-Gate
und Schema-Discovery geführt. Zwei Werkzeug-Funde getragen:

- **das Tabellen-Inventar war auf DaCHS-TAP kaputt** — queried den HTML-Root statt
  `/sync` und las Rows nur als Objekte, während Standard-TAP-JSON
  Array-of-Arrays liefert. Fix: läuft
  jetzt über `sync_candidates` (wie `tap_speaks`) und parst über
  `parse_json_rows` (beide Formen, Spalten-Case-insensitive). Verifikation:
  alle 46 clean-tap-Endpoints liefern ein parsebares Inventar (0 void);
  vorher `tables returned void` auf jedem DaCHS-Host.

- **Bestand-Dedupe:** mast (`tap_index_mast.φ`, 15) und cadc
  (`tap_index_cadc.φ`, 21) sind schon geerntet → bestand, kein neuer Draft.
  Keiner der 48 RegTAP-Hosts trägt Bestand in sources.φ/tap_index.

Disposition (50):

| Zustand | Anzahl | Befund |
|---|---|---|
| bestand | 2 | mast, cadc — tap_index schon getragen |
| decline registry | 2 | registry.euro-vo.org (RegTAP-Service-Verzeichnis), voparis-rr.obspm.fr (rr.*+glots, kein Messwert-Feld) → dead_sources.φ |
| §8-Recherche | 2 | neocc.esa (502), koa.ipac (404) — toter Endpoint, http-Klassifikations-Linie |
| tap-index + Draft pending | 44 | Schema-Discovery getragen, `tap_index_<label>.φ` in `phi/pipeline/catalog/` |

Die 44 Überlebenden tragen ihr Inventar als `tap_index_<label>.φ`; der
Source-Draft je Endpoint (field/force/τ nach §8) ist die nächste Bau-Linie —
jede Ledger-Note trägt ihren tap-index-Verweis und „Draft pending".
