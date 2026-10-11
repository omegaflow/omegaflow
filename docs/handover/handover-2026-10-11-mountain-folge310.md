<!--
  title: Handover — Mountain-Folge 310 (2026-10-11)
  session: Mountain-Linie in einem Pass — canon.φ entfernt (phi/ = 7 Register), particle-cern-Parser (Tail/Class-Ref/Bootstrap), harvest.φ-Census (Dispatcher), dead_sources geleert, Bayern-DOM registriert
  class: handover
  date: 2026-10-11
  sha256: 29d703e0c81ee0ef5d95d7409cc7def6c187851c5c37548fcf7c6740d8152944
  status: live
-->
# Handover — Mountain-Folge 310 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; Stand
HEAD `8739dffaa`). Diese Session konsumierte `handover-2026-10-11-mountain-folge309.md`
(→ `archiv/`). flash only, kein pro/max. Operator-Trigger dieser Session: die
Linien-Start-Direktive, „alle 5 an agenten und weitere agenten für 3", „die dead
sources leer" — 8 unabhängige Täufer parallel.

## Burn: open 0.0 · close 0.15 · cap 0.5 — Grund: 1× line + 6× grind-flash + 2× general (Täufer); `session_burn` total 31 Sessions $1.8614; deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### particle-cern — Baskets + Sample-Verifikation
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt
- **Lage:** (gemessen 2026-10-11 via `src/archivar/root.rs` + ROOT-Quelle) `parse_streamer_elements`
  parst jetzt den Element-Tail (`fArrayLength/fArrayDim/fMaxIndex[5]/fTypeName`; v3-Range
  `== 3`) + Class-Ref (kein stilles `String::new()`); `parse_tree` bootstrappt über
  `TBranchElement` + `0x80000000|idx`-Ref-Tags; `cargo check` 0/0; unabhängig gegen
  ROOT-Quelle verifiziert (`core/meta/src/TStreamerElement.cxx:579-587`; `TBufferFile.cxx:2824-2857`).
  `phi/blocked_sources.φ`-note nachgezogen (254 Z.).
- **Blockade:** Basket-Arrays (`fBasketBytes/fBasketEntry/fBasketSeek`) unimplementiert; Verifikation am echten Sample fehlt.
- **Braucht:** `TBranch` v12-Basket-Arrays in `src/archivar/root.rs` lesen; dann am Sample `AliVSD_Masterclass_6.root` (`https://opendata.cern.ch/record/1120/files/AliVSD_Masterclass_6.root`, sha256 `a5f0583c…`, lokal `/tmp/opencode/`) verifizieren.

### Beobachtungsoperator + Fit — Schritt 2 (ref_hz vertakten)
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11, unverändert) `ReferenceStage`/`reference_stage`/`ref_uplink_hz` (`src/archivar/odf.rs:2218-2242`) stehen; der Konsument `tools/measure/src/bin/pioneer10_odf_residuum.rs:502,510` behandelt `ref_hz` noch als Skalar.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die zwei `pub`-Funktionen im Konsumenten verdrahten; den TSF-vs-Uplink-Diskriminator (dt12/13) bestätigen; dann Troposphäre/Ionosphäre in die Kette.

### ci-gate — Trigger: nächster abgeschlossener Lauf
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD nach diesem Commit
- **Lage:** (gemessen 2026-10-11 via `ci_manage status`, Standing-Pass) `ci-gate`-Läufe in Flug; die 18 Lints in `77a7f7e71` getilgt.
- **Blockade:** kein abgeschlossener Lauf am neuen HEAD.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die verbliebenen Lints tilgen (kein `#[allow]`).

### PEP + Tudat — Schritt 3 (mehrere Bahnen)
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11, folge306) vier Häuser als `ephemeris_*` registriert; Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** weitere Zeugen durch die Kette, Kette CI-nah verankern.

### Mycelium-φ-Blöcke — Serien-Manifest-Format
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Entwurf des Serien-Manifest-Formats
- **Lage:** (gemessen 2026-10-11, unverändert) der ecad-Riss ist gelöst (netloc = Quellen-Identität); AODN-`ttl 1` = Datensatz-Kadenz, kein fester Refresh.
- **Blockade:** ohne Serien-Manifest trägt jeder Klima-Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format; der CI-Wait je Block steht in `state/zustand/wartend.φ`.

### register_sort — Tool-Gap `blocked`-Opener
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) `target/release/register_sort phi/blocked_sources.φ` → rc=2 `unknown block opener 'blocked'`; `--help` listet `parser-def` als Opener, nicht `blocked`.
- **Blockade:** das Tool trägt das Dispositions-Register `blocked_sources.φ` nicht.
- **Braucht:** `register_sort` um die `blocked`-Opener (Key/Access/…) erweitern, oder die register-Form auf `parser-def` als Opener prüfen.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-10, unverändert) Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated).
- **Blockade:** Overpass bot-gated (HTTP 406 direkt+Proton).
- **Braucht:** Riss 4-Abschluss oder Descope.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → `phi/sources.φ`-Zeile → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment (`state/zustand/wartend.φ`), kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11) HTTP 200, sha256 `ad345805…`; `sensor.community` bereits registriert.
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

### Asservatenkammer-Survey-Marker — Prosa-Reconciliation
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die Marker-Reconciliation des Surveys
- **Lage:** (gemessen 2026-10-11) `docs/surveys/survey-2026-10-08-research-api-mcp.md`: Status-Absatz trägt den gemessenen Stand, die `Top-Schritte` (`:55-74`) noch die stale „Braucht: Key…"-Sprache.
- **Blockade:** keine.
- **Braucht:** die `Top-Schritte` auf den gemessenen Status ziehen (Header-sha neu).

### docs/concepts/bindings-dust-maske-gebco.md — Body stale
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die Prosa-Reconciliation des Docs
- **Lage:** (gemessen 2026-10-11) see-also in diesem Atom gezogen (`tools/harvest/data/dust-maske.φ`); der Body (`:11`, `:95-99`) nennt noch `phi/bindings/*.φ` (dust-maske dorthin gezogen, bathymetrie-gebco descoped) und ein binding, das `phi/witnesses.φ:97` referenziert.
- **Blockade:** keine.
- **Braucht:** Body + `phi/witnesses.φ:97`-Referenz auf die gemessenen Heime ziehen.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte die grossen punkte parallel mit agenten abarbeiten und ABSCHLIESSEN, NICHT VERSCHLEPPEN" | 2026-10-11 | Operator (Session, Mountain 305)
„was ist mit unseren ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig zu machen" | 2026-10-11 | Operator (Session, Mountain 306)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)
„hast du auch phi weiter bereinigt?" | 2026-10-11 | Operator (Session, Mountain 310)
„alle 5 an agenten und weitere agenten für 3" | 2026-10-11 | Operator (Session, Mountain 310)
„verstehst du nicht ich will die dead sources leer haben kein mensch interessiert was da drin steht ausser tote quellen die wir haben wollen" | 2026-10-11 | Operator (Session, Mountain 310)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. **Dieses Atom (Mountain 310):**
- **phi/ weiter bereinigt** — `phi/canon.φ` entfernt (Tautologie; Gate trägt `REGISTER_CLASSES`,
  `commit_check.rs:288` schließt canon aus) → `phi/` = 7 Register; `.gitignore`/`AGENTS.md`/zwei Konzept-Doks nachgezogen.
- **particle-cern** — `src/archivar/root.rs` Tail/Class-Ref/Bootstrap/Version-Gate, `cargo check` 0/0, gegen ROOT-Quelle verifiziert; `blocked parser-def`-note aktualisiert.
- **harvest.φ** — Census: nur 1 Multi-Format-Workflow (`harvest.yml`, Input vorhanden); Dispatcher `harvest-long.yml`-Auflösung + `force`/`timeout`-Gate in `.github/workflows/harvest-dispatch.yml` geheilt.
- **dead_sources.φ geleert** (Operator-Wort) — 5 Blöcke (ITS_LIVE ×3 · MASIE · MODVOLC), `register_sort` canonical; 3 Kataloge → `phi/declined_sources.φ`.
- **Bayern-DOM** (Mycelium-304) — `phi/sources.φ`-Block, `ttl 63072000` aus dem 2-Jahres-Turnus der Bayernbefliegung gemessen, `terms CC-BY-4.0`.
- **docs/SOURCE_PORT.md** — von der descopten `phi/pipeline/`-Maschinerie auf den gemessenen Stand (7 Register + `wartend.φ` + Arme) gezogen.
- Stehender Pass zitiert, nicht kopiert. Geteilter Baum: `tools/gate/src/bin/commit_check.rs` · `tools/utils/src/bin/archive_search.rs` fremd modifiziert — nicht angefasst.
- Eigene Pfade dieses Atoms: `.gitignore` · `AGENTS.md` · `docs/SOURCE_PORT.md` · `docs/concepts/bindings-dust-maske-gebco.md` · `docs/concepts/kanal-ontologie-komplettbau.md` · `phi/sources.φ` · `phi/dead_sources.φ` · `phi/declined_sources.φ` · `phi/blocked_sources.φ` · `phi/canon.φ` (D) · `src/archivar/root.rs` · `.github/workflows/harvest-dispatch.yml` · diese Übergabe · Archiv-Move folge309.
