<!--
  title: Survey — Ehrlich benannt: wo das Werkzeug fehlte (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: 9a610596fafa3394c2c7b79107a634b9faee6c0623a4839c9cbcb5156cfcc977
  status: live
  see-also: docs/specs/ref-auth-apis.md docs/concepts/docs-naming.md
-->
# Ehrlich benannt — wo das Werkzeug fehlte (Stand 2026-09-14)

Dieser Survey folgt dem Operator-Hinweis: das Wort **„ehrlich" (honest)** markiert
im Register die Stellen, wo ein Bau-Vorhaben mit dem Verdikt „Daten nicht
veröffentlicht / nicht öffentlich / request-only / gated / Paywall" abgebrochen
wurde. Die Messung zeigt: in vielen Fällen war nicht die Quelle verschlossen,
sondern das **Werkzeug** fehlte (Parser, Download, Reader, Token-Hook) — oder ein
Query-Parameter war falsch. Der Survey listet die Stellen und trennt
Werkzeug-Lücke von echter Absenz.

## Der Goldschatz (das Muster in Reinform)

`docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:78-84` — „Suchlauf-Befund
(2026-09-03) — reine Web-Recherche. Vorbemerkung, **ehrlich benannt**: Dieser
Suchlauf verfügte nur über Web-Recherche — **kein Datei-Download, kein
ODF-Parser, kein N-Körper-Solver**. Der in der Abnahme geforderte Schritt
„geladener Tracking-Pass → Residuum → mm/s-Wert" ist daher **nicht** vollzogen.
0 honored: keine einzige mm/s-Zahl wird hier genannt oder re-deriviert."

Die Flyby-Doppler-Rohdaten (Galileo, NEAR, Cassini, Rosetta, Messenger, Juno)
wurden als `open`/`pending`/„nur intern JPL-NAV" registriert — die Tabelle selbst
zeigt je Zeile „RS/ODF nein" (nicht geladen). Die Absenz ist der fehlende
**ODF-Parser/Download**, nicht die Quelle.

Der kanonische Präzedenzfall steht im Haus selbst:
`docs/specs/ref-auth-apis.md` §A — **SuperMAG**: „funktioniert (gemessen
10.09.2026 …; der frühere ‚geht nicht'-Befund war der **falsche Parameter**
`user`/`username` statt `logon`)."

## Die Klassen

### 1. `request-only` / internes Archiv — Sonden-Rohdaten (DSN/JPL/NAV/ODF)

| file:line | Quelle | Claim (wörtlich) | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:78-84` | JPL/DSN ODF (Flybys) | „kein Datei-Download, kein ODF-Parser" | Flyby-Doppler-Residuum (mm/s) | **JA** (ODF-Parser/Download) |
| `docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md:125,151-155,168-170,180` | Juno Earth-EDR / JPL-NAV | „pre-EFB-Merged-ODF, nicht öffentlich archiviert"; „Roh-Doppler ausschließlich über DSN-Datenanfrage" | Juno-Erd-Flyby ΔV∞ | teils (TRK-2-34-Parser vorhanden; Zugang = Anfrage) |
| `docs/auftrag/archiv/auftrag-voyager-roh-doppler-zugang.md:35,61-63,78-81` | JPL/DSN ODF/TDF (TRK-2-34) | „kein offener Endpunkt, `request-only`" | Voyager-Cruise-Doppler 1998–2002 | NEIN (DSN-Anfrage) |
| `docs/auftrag/archiv/auftrag-quiet-zone-uebertragung.md:34,178-190` | NH/JPL/DSN-ODF | „Harvest nicht offen — `request-only`" | NH-Quiet-Zone-Harvest | NEIN (Anfrage) |
| `docs/auftrag/archiv/auftrag-quiet-zone-vorfilter.md:71,74,79-80` | NH Nav-Doppler / Mariner | „SPDF 404; Nav-Doppler `request-only`" | NH-Harvest | NEIN |
| `docs/handover/archiv/handover-2026-09-12-forschung-folge7.md:80-81` | Galileo ODF | „ODF-Format (1 vs 2) bleibt ungemessen, bis der ODF-Doppler-Extrakt etwas hält" | Galileo-Doppler | **JA** (ODF-Doppler-Extrakt) |
| `docs/auftrag/archiv/auftrag-lisa-pathfinder-psd.md:35,59-61` + `tools/measure/src/bin/lpf_psd_probe.rs:40` | ESA LPF-Legacy-Archiv | „behind CAS auth with the TAP interface disabled"; „`not-published`" | LISA-PF-Kreuzspektrum | NEIN (CAS-Auth) |
| `docs/auftrag/archiv/auftrag-extern-weberin-zweitlinien.md:73-77` | PRIDE ΔDOR / EVN | „Kein offenes VLBI/ΔDOR/Range gemessen" | Sonden-Positions-Zweitlinie | NEIN (Login-Gate) |

### 2. Paywall (Paper/Volltext)

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/paper/woo-armstrong-1979-jgr-abstract.md:6,16` | Woo & Armstrong 1979 JGR | „abstract-only (full text paywalled)" | S-Band/Allan-Werte | NEIN (Verlag) |
| `docs/paper/armstrong-1998-phase-scintillation-abstract.md:6,16` | Armstrong 1998 Radio Science | „abstract-only; JPL preprint handle down" | Phasen-Szintillation | NEIN |
| `docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md:34,36,120,171-175` | Wohlmuth 1997 / Haw 1997 / Hinson 1997 | „Volltext Paywall (nicht gelesen)" | Galileo-Borduhr-Sprung A/B | **JA** (Schließer: „RSS-Datenköpfe der PDS-Sätze GO-J-RSS-* → Reader") |

### 3. Gated API / Login / SSO / SMS

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `docs/specs/ref-auth-apis.md:136,137,138,140` | GRACE-FO/SWOT, SMAP, CDDIS IONEX, AppEEARS | „Earthdata vorhanden; S3-Scheme ungetragen"; „Live 200" | Gravity/EM/Thermal-Ports | **JA** (S3-Reader / EarthData-Token-Hook) |
| `docs/specs/ref-auth-apis.md:133,134,139` | NASA ADS, Space-Track, GES DISC | „Subendpoints dead 404"; „Query 401-auth"; „griddap dead 404" | EM-Katalog-Ports | teils |
| `docs/paper/laic-arrow-direction.md:224` | CSES (leos.ac.cn) | „login-gated SPA … requires a Chinese mobile number" | CSES-Ionosphärenkanal | teils (DEMETER-`.DAT`-Parser offen) |
| `docs/handover/archiv/handover-2026-09-13-ernte-folge11.md:45-46` | WWLLN | „Realtime-Roh ist `not-published` (Mitgliedschaft)" | Blitzortung | NEIN |

### 4. Teilchen-/Quantendaten

| file:line | Quelle | Claim | blockierter Bau | Werkzeug-Lücke |
|---|---|---|---|---|
| `phi/dead_sources.φ:415` | AMS-02 | „nur Papier-Abbildung, kein maschinenlesbarer Flux-Endpoint" | Teilchenfluss | **JA** (HEASARC AMS02SPEC FITS/TDAT) |
| `phi/dead_sources.φ:4803` | Super-Kamiokande | „ohne Roh-Messkanal; Teilchen-Kanal pending" | atmosph. Neutrino-Richtung | NEIN |
| `phi/dead_sources.φ:4807` | Telescope Array | „ein einzelnes UHE-CR-Ereignis, kein τ" | UHE-CR-Feld | NEIN |
| `docs/concepts/kybernetische-astrophysik.md:393-395` | Quantenschaum | „trägt keinen Tatort — Detektor-Klicks/Qubits gehören dem Labor" | Quantenschaum-Nadel | NEIN (Domänengrenze) |

### 5. Reader-/Werkzeug-Lücke (das „Werkzeug fehlte" wörtlich)

- S3-Scheme ungetragen (`ref-auth-apis.md:136`) — GRACE-FO/SWOT PODAAC.
- ODF-Doppler-Extrakt (Galileo, `handover-2026-09-12-forschung-folge7.md:80-81`).
- FITS/TDAT-Reader für AMS-02 (`dead_sources.φ:415`).
- Parquet/GRIB-2/OPeNDAP offen (`handover-2026-09-10-autonom.md:91-92`).

### 6. Query-/Parameter-Bug (SuperMAG-Muster)

- `docs/specs/ref-auth-apis.md:135` — SuperMAG: der frühere „geht nicht"-Befund
  war der falsche Parameter (`user`/`username` statt `logon`). Der Beleg, dass ein
  „nicht erreichbar" ein Werkzeug-/Query-Fehler sein kann, kein Wall.

## Re-Messung

Jede Werkzeug-Lücke (Klasse 5) ist der Auftrag der folgenden Party: für jede
Stelle messen, ob mit dem gebauten Werkzeug (EarthData-Token-Hook, S3-/Reader,
ODF-Parser) ein offener Weg existiert. Die `request-only`-Fälle (Klasse 1) sind
keine Mauern, sondern ungesendete DSN-Anfragen — eine Register-Pflicht, kein
Werkzeug. Die echten Absenzen (Klassen 2–4) bleiben benannt, bis eine Messung
sie öffnet.

Der Survey trägt die Stellen; die Party trägt die Messung.
