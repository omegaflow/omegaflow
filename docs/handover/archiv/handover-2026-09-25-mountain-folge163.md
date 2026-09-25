<!--
  title: Handover — Mountain-Folge 163 (2026-09-25)
  session: Mountain-Folge 163
  class: handover
  date: 2026-09-25
  sha256: 49da3e679e3d180bf409974137924e319f01fd7e83e5365fc71070765f082467
  status: live
-->
# Handover — Mountain-Folge 163 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×166`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c "gap unit-auto-detect" phi/blocked_sources.φ`)
  Arm gebaut: `unit_from_name_suffix` erweitert (units.rs:169 — Suffixe der 166
  Feldnamen: `_arcsec`/`_arcmin`/`_jy`/`_mjy`/`_deg`/`_au`/`_days`/`_sec`/`_solar`/
  `_msun`/`tecu`/`_du`/`dobson`/`_psu`/`_hpa`/`_mb`/`_dbar`/`_wm2`/`_w_m2`/`_wm2nm`/
  `_c`/`_magnitude`) und in den Block-Force-Pfad verdrahtet (port.rs
  `field_or_review`: Suffix-Einheit + τ=ttl statt pauschal „unit absent");
  2 Einträge (CMA now 54511, JMA quake list) descoped (URL integriert →
  phi/sources.φ:372/:423 — Stale-Zwillinge). 166 Einträge tragen das Token.
- **Blockade:** keine
- **Braucht:** Re-Port der 166 Einträge über den stehenden Arm (grind); das Token
  fällt pro Eintrag beim Port. Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×166`
  (N = live count, Scanner meldet Drift).

#### gzip-Body ohne `.gz`-Suffix — kein Erkennungs-/Entpack-Pfad
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch
- **Lage:** (gemessen 2026-09-25 via `read`) `fetch_raw_with` (`src/archivar/fetch.rs:77`)
  wandelt die rohen curl-Bytes via `String::from_utf8_lossy` um, bevor ein Aufrufer
  sie sieht; ein gzip-`Content-Encoding`-Body an einer URL ohne `.gz` wird nicht
  entpackt und fällt als `data-present (non-JSON body …)` durch. Die toten
  `[0x1f,0x8b]`-Byte-Checks (`fetch.rs:590`, `:1061`) sind in diesem Atom entfernt.
- **Blockade:** keine
- **Braucht:** in `fetch_raw_with` vor der lossy-Umwandlung
  `output.stdout.starts_with(&[0x1f, 0x8b])` prüfen und `gunzip(&output.stdout)`
  (`src/archivar/inflate.rs:179`) anwenden; danach den separaten
  `fetch_raw_bytes`-Zweig bei `fetch.rs:1058` prüfen/entfernen.

#### `arxiv` HTTP 406 — serverseitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** arXiv schließt die API-Migration ab
- **Lage:** (gemessen 2026-09-25) 406 mit leerem Body, UA-unabhängig, für jede
  ungecachte Query; gecachte Queries 200. Retry für 406 entfernt.
- **Blockade:** arXiv-Edge
- **Braucht:** Wiedervorlage bei Trigger; kein Code.

#### Legacy 40-Byte-Stern-Bins rekompilieren
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sread docs/concepts/archivar-mathematikerin.md --offset 20`) der
  Loader akzeptiert nur 44-Byte-Records (`dr3_stars.bin` = 40 B + f32 rv); 40-Byte-Legacy-Bins
  bleiben dunkel (`pending recompilation`), rv wird nicht als 0.0 erfunden.
- **Blockade:** keine.
- **Braucht:** die Legacy-40-Byte-Stern-Bins über `tap_compiler`/`tycho2_compiler` auf 44 Byte
  rekompilieren und als CDN-Asset manifestieren; danach am Loader messen.
- **Quelle:** docs/concepts/archivar-mathematikerin.md

#### Glossar — `pending`-Definitionen belegen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read docs/concepts/glossar.md`) 11 Terme tragen
  `Definition pending`: Tier ii/iii `Zustandsintensität`, `Kollabierer`, `Ontologie-Motor`,
  `Kohärenz-Gradient`, `mycorrhizal_internet`; Tier i `presenceWeight`, `EpistemicState`,
  `ProvenanceMetadata`, `coherenceidx`, `permutationentropyshader`, `feed_beat_to_hrv`.
- **Blockade:** keine.
- **Braucht:** je Term die Definition aus `state/funding/profil-operator.md` destillieren und in
  `docs/concepts/glossar.md` eintragen; steht keine Definition im Korpus, bleibt der Term
  `pending`.
- **Quelle:** docs/concepts/glossar.md

#### tools-latest-Release nachziehen (`session_burn`, neue lokale Modi)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `glob ~/.local/bin/session_burn` — kein Treffer; `read
  docs/concepts/tools-map.md`) `session_burn` fehlt auf PATH; `--count/--case/--path` und der
  `post_args`-Fix liegen „im Binär nach dem nächsten Build".
- **Blockade:** keine.
- **Braucht:** den Release-Bau von `omegaflow-register`/`omegaflow-utils` in CI anstoßen
  (`gh workflow run <workflow>`); nach dem Bau `session_burn` als PATH-Symlink legen und den
  `## Offen`-Eintrag in `docs/concepts/tools-map.md` streichen.
- **Quelle:** docs/concepts/tools-map.md

#### `tools/*`-Crates vermessen (`cargo check -p` + Testlauf je Crate)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read docs/surveys/survey-2026-09-06-codestruktur.md` §3)
  nur die Core-Crate ist gemessen; die 253 `tools/*`-Dateien haben je Crate keinen
  `cargo check -p`/Testlauf.
- **Blockade:** keine.
- **Braucht:** je Funktions-Crate `cargo check -p omegaflow-<fkt>` + Testlauf als CI-Job messen
  und das Ergebnis in §3 eintragen.
- **Quelle:** docs/surveys/survey-2026-09-06-codestruktur.md

#### Konsument je pub-Fn nachweisen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read docs/surveys/survey-2026-09-06-codestruktur.md` §3)
  „Konsument je pub-Fn: lebendig oder tot (Call-Site-Beweis fehlt)" ist ungemessen.
- **Blockade:** keine.
- **Braucht:** die pub-Fn aus `src/archivar` + `src/mathematikerin` gegen `sgrep`-Call-Sites
  prüfen; tote Pfade mit Verdikt benennen.
- **Quelle:** docs/surveys/survey-2026-09-06-codestruktur.md

#### `live`/`offline`-Verdrahtung aufspalten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read docs/surveys/survey-2026-09-06-codestruktur.md` §3)
  die §8-Tabelle in `docs/concepts/die-weberin.md` führt ω-Pfad und Offline-Bins flach
  nebeneinander; die Aufspaltung ist `pending`.
- **Blockade:** keine.
- **Braucht:** die §8-Tabelle in `docs/concepts/die-weberin.md` in Membran-Teile und
  Offline-Bins trennen.
- **Quelle:** docs/surveys/survey-2026-09-06-codestruktur.md

#### Datenvertrag je Format-Modul prüfen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read docs/surveys/survey-2026-09-06-codestruktur.md` §3)
  der 26×f64-Wire und die GPU-Pack-Offsets sind je Format-Modul nicht gegen die WGSL-Zugriffe
  geprüft.
- **Blockade:** keine.
- **Braucht:** das manuelle Verifikationsprotokoll aus `docs/concepts/archivar-mathematikerin.md`
  je Format-Modul anwenden; Abweichungen fixen.
- **Quelle:** docs/surveys/survey-2026-09-06-codestruktur.md

#### Ehrlich-benannt — Klasse-5-Werkzeuglücken re-messen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`) vier Werkzeuglücken
  (S3-Scheme, ODF-Doppler-Extrakt, FITS/TDAT-Reader AMS-02, Parquet/GRIB-2/OPeNDAP) sind
  ungemessen, ob ein offener Weg existiert.
- **Blockade:** keine.
- **Braucht:** je Stelle mit dem gebauten Werkzeug messen (`archive_search`/Reader-Probe) und das
  Verdikt in den Survey eintragen.
- **Quelle:** docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md

#### Silence-Map-Probe bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`) der Survey nennt die
  Silence-Map-Probe „Erster Atom (ins Handover)"; erwartete vs. leere Zellen unter
  Modellparametern.
- **Blockade:** keine.
- **Braucht:** `tools/measure/src/bin/silence_map_probe.rs` bauen (Null-Kalibrierung als Spiegel
  der FP/FN/Symmetrie-Gates von `te.rs`).
- **Quelle:** docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md

#### Certainty — vC-Definition (L:53) messen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`) die vC-Semantik von L:53 ist
  ungemessen; `exp` und `tanh` haben inverse Asymptotik (Konfud a).
- **Blockade:** keine.
- **Braucht:** den Legacy-Klon `github.com/omegaflow/omegaflow-legacy` shallow nach
  `/tmp/opencode/of-legacy` holen und L:53 lesen; erst danach `quantum`/`decay` als Faktor in
  `field_permeability` (`omega.rs`) prüfen.
- **Quelle:** docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md

#### TDA/Betti-0-Probe bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`) single-linkage über die
  Takens-Embeddings von `te.rs` fehlt (~100 Zeilen; Schwelle = Silverman-Bandbreite).
- **Blockade:** keine.
- **Braucht:** eine measure-Probe für Betti-0 über die Takens-Embeddings von
  `src/mathematikerin/te.rs` bauen.
- **Quelle:** docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md

#### Minkowski-`ds²`-Delta-Probe
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`) `ds²` als ω-Loop-Faktor ist
  ungemessen; die Schicht-Frage (Archivar-Cone-Gate `membrane.rs` vs. Mathematikerin/WGSL) ist
  Konfud (b).
- **Blockade:** keine.
- **Braucht:** die Schicht festlegen und eine Delta-Probe mit/ohne `ds²` messen.
- **Quelle:** docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md

#### Synthetic Flight — `presence_tx`-Annahme messen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `read
  docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`) die Weltlinien-Infrastruktur +
  freies `t_presence` stehen; ob `presence_tx` (`relay.rs`) fremdgetriebene Positionen annimmt,
  ist ungemessen; die Präsenz ruht, kein Selbstantrieb.
- **Blockade:** keine.
- **Braucht:** `presence_tx` in `relay.rs` gegen fremdgetriebene Positionen messen und das
  Ergebnis in den Survey eintragen.
- **Quelle:** docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md

### Operator handelt

#### Membran-Debug — Chrome DevTools MCP anbinden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am laufenden Chrome)
- **Lage:** (gemessen 2026-09-25 via `read docs/concepts/tools-map.md`) Pfad (i) ist „noch nicht
  angebunden"; Telemetrie-Flags `--no-usage-statistics` `--no-performance-crux` sind Bedingung.
- **Blockade:** das Operator-Wort.
- **Braucht:** Operator-Wort; dann Chrome DevTools MCP mit den beiden Telemetrie-Flags anbinden
  und als Pfad in `docs/concepts/tools-map.md` registrieren.
- **Quelle:** docs/concepts/tools-map.md

#### Messpunkt-Verteilung — Verdikt je Kandidat + Architektur-Empfehlung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) das Survey fordert ein Verdikt je Verteilungs-Kandidat 1–8, hält Kandidat 9 (fehlende aktuelle Konzepte) offen und stellt §6 fünf Detailfragen (Voronoi-Anzeige vs. bilineare Mischung; Struktur-Radius-Kriterium; Budget-Skalierung; Fovea-Fallback; Silizium-Implementierbarkeit) plus eine Architektur-Empfehlung.
- **Blockade:** keine.
- **Braucht:** je Kandidat 1–8 ein Verdikt, Kandidat 9 offen halten, die 5 Detailfragen beantworten und die Architektur empfehlen (wer rechnet was, welche Daten fließen, wie die Dichte skaliert).
- **Quelle:** docs/surveys/survey-messpunkt-verteilung.md

### Gelesen — keine offene Arbeit (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung + `register_lookup --orphan-docs`) die offenen Marker dieser Dokumente sind Prosa, kein handlungsfähiger Punkt.
- **Blockade:** keine.
- **Braucht:** — (descoped mit Befund).
- **Quelle:** `docs/concepts/kybernaut-native-methodology.md`, `docs/concepts/pfeiler-der-architektur.md`, `docs/concepts/the-counter-slope.md`, `docs/surveys/survey-2026-09-02-tools-funktionsstruktur.md`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
