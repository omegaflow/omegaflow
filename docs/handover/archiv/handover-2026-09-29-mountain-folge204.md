<!--
  title: Handover — Mountain-Folge 204 (Stand 2026-09-29)
  session: Mountain-Folge 204
  class: handover
  date: 2026-09-29
  sha256: 5bdeb0c38f3fae0056ef9a04de626536bdb91a636ad191057beeac6db64c21e4
  status: live
-->
# Handover — Mountain-Folge 204 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger für
fällig erklärt.

Jeder offene Punkt trägt fünf Felder, damit der nächste Leser ohne Vorwissen
entscheiden kann: **Trigger** (was ihn kippt) · **Lage** (gemessener Zustand,
Stempel) · **Blockade** (warum es hängt oder „keine") · **Braucht** (der wörtliche
Schritt) · **Empfehlung** (Mountains Votum, was zu tun ist).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Session, Mountain 190)
„kannst du das bitte fixen" — der `--verdict`-Werkzeugdefekt wird im Atom geheilt | 2026-09-28 | Operator (Session, Mountain 194)
„bitte fixen statt verschleppen" — no-cadence-Sprachloch und ttl der 81 SPK-Blöcke im Atom gebaut | 2026-09-28 | Operator (Session, Mountain 194)
„was sagt der rat?" — Rat zur Design-Frage | 2026-09-28 | Operator (Session, Mountain 194)
Hier ausführen, keine Rangfolge, flash-first delegieren — session-weiter Consent (Delegation) | 2026-09-28 | Operator (Session, Mountain 195)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor Plan neu messen, nicht zitieren | 2026-09-28 | Operator (Session, Mountain 199)
Führe den bestätigten Plan aus — `line`-Agent, flash-first delegieren, eine Session ist ein abgeschlossenes Atom; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 202)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor dem Plan neu messen, nicht zitieren | 2026-09-29 | Operator (Session, Mountain 204)
Du kannst. Führe den Plan aus — als `line`-Agent (auto-bestätigt); dispatch flash-first; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 204)
„kannst du bitte nachrichten an die linien schreiben, auf die du wartest, dass sie die trigger bevorzugt abarbeiten sollen; hast du die übergabe so geschrieben, dass jeder einzelne zu klärende punkt mit erklärung und empfehlung vorgetragen wird?" — jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte | 2026-09-29 | Operator (Session, Mountain 204)

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen + mycelium (`dropped-baseline`)
- **Trigger:** `ci-gate`/`register-coverage`/`ci-check` am dann aktuellen HEAD enden.
- **Lage:** (gemessen 2026-09-29 via `ci_manage jobs` + GH-API-Job-Log) @`5d6c9c685`: `register-coverage 36506672142` success; `ci-gate 36506672052` rot = **nur `dropped-gate`** (clippy/build/format success), `dropped-gate: baseline 1113 | current 1127 | delta 14` — getragen von mycelium (`docs/zustand/dropped-baseline.md`); `ci-check 36506672136` rot = **11 Test-Fehler** in den neuen Lese-Armen. Die 11 sind in diesem Atom geheilt (`gras_2c.rs` HEADER_BYTES 8; `pds3_binary.rs`/`pds3_table.rs`/`pds4.rs`/`pds4_binary.rs` COLUMN_TYPE_BYTES 16→32 + abgeleitete Offsets; `pds3_img.rs` line_stride; `pds4_binary.rs` NaN/Inf→absent; `hips.rs` Test-Slice band-major; `register_lookup.rs` Scanner-Wortgrenze). `cargo check` 0/0.
- **Blockade:** `dropped-gate` gehört mycelium.
- **Braucht:** grüner Lauf am neuen HEAD (nach Commit); `## An mycelium` (`dropped-baseline` 1113→1127).
- **Empfehlung:** nach diesem Commit den neuen HEAD-Lauf abwarten; parallel mycelium bitten, den `dropped-baseline`-Bump im nächsten Commit zu setzen — dann ist ci-gate grün; kein Mountain-Schritt bleibt.

### Fixe-Tabellen — Körper `halley`/`itokawa`
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Ephemeris-Build (CI) + `url`/`origin`/`compiler` (mycelium).
- **Lage:** (gemessen 2026-09-29) `src/archivar/kernels/naif_body_ids.tsv` trägt jetzt `2025143 itokawa 10` und `1000036 halley 10` (NAIF-Konvention, aus SBDB + `aa_summaries.txt` gemessen); `body_table()` (`ephemeris.rs:25`) speist Compiler + Runtime. Itokawa: Hayabusa-SPK in `sources_index.φ:313778ff`; Build = CI (`ephemeris_compiler.rs`, Netz+Compute). Halley: **kein SPK-Kernel in NAIF** (gemessen), Horizons nur Erscheinungs-Records → neuer Compiler-Pfad nötig. `at halley`/`at itokawa` = 0 in `sources.φ`.
- **Blockade:** Halley-Route fehlt; Itokawa-Build = CI.
- **Braucht:** Itokawa-CI-Lauf; Halley-Route-Entscheid (Horizons-Apparitions-Fetch).
- **Empfehlung:** Itokawa jetzt bauen lassen (die tsv-Registrierung ist fertig); Halley zurückstellen, bis die Route entschieden ist — kein Blindbau. `sources.φ`-Block an mycelium delegieren.

### pds3/pds4-CDN-Verdikt
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** `sources.φ`-Zeilen nach Körper-Registrierung.
- **Lage:** (gemessen 2026-09-29) `pds3_fixed_width`/`pds4_fixed_width` = 0 in `sources.φ`; `spectral` registriert (`:2416`).
- **Blockade:** `url`/`origin`/`compiler` = mycelium.
- **Braucht:** Fixe-Tabellen; `## An mycelium`.
- **Empfehlung:** mit der itokowa-Registrierung die pds3/pds4-Zeilen mit mycelium zusammen setzen; ohne Körper kein Body-Anker — also nach Punkt 2.

### Rosetta — Typ-18 vs Horizons
- **Status:** wartend | **Bindung:** mycelium (Workflow) + eigen (Messung)
- **Trigger:** Mycelium richtet `rosetta-ephemeris-cdn.yml` auf das Horizons-Asset.
- **Lage:** (gemessen 2026-09-29, Rat-Wort) Rat: **kein** Typ-18/19-Arm — `ORER_…00031.BSP` führt 16 Segmente `target -226`, alle Typ 18, 2-Tage-Erd-Fenster; der Reader-Filter (`ephemeris.rs:348`) liefert 0 Granule, `rosetta-ephemeris-cdn.yml:63-69` ist by construction rot. Horizons deckt dasselbe Fenster ±20 d, ICRS (`sources.φ:15947-15952`). Register-Riss: zweiter `at rosetta`-Anker `sources.φ:15786-15791` (unbaubar).
- **Blockade:** Workflow/Anker = mycelium.
- **Braucht:** `## An mycelium`; Zähler-Messung Typ-18/19 über die geernteten Kernel als eigener Schritt (baut den Arm erst bei zweitem Konsumenten).
- **Empfehlung:** Workflow auf Horizons richten, toten BSP-Anker löschen; den Typ-18-Arm **nicht** bauen — erst die Zähler-Messung. Der Rat sprach klar gegen den Einzelgranulat-Bau.

### commit_check `ereignis-ohne-folge` — Session-Bindung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort da (2026-09-29).
- **Lage:** (gemessen 2026-09-29, Rat) Rat: Bindung **per Session** (Feld 1), nie Wall-Clock, nie unbounded; die unbeschränkte Surfacing-Ebene (`register_lookup --open`-Scan) bleibt. Target: `commit_gate.rs:1777` (Signatur + Feldfilter), `commit_check.rs:171-179` (Session durchreichen via `.githooks/pre-commit:24`), Fixtures; fehlender Session-Name überspringt per Namen, nie still.
- **Blockade:** keine.
- **Braucht:** Bau (Gate-Atom); Pass-Ebene als eigener Punkt (erledigt-Journalklasse).
- **Empfehlung:** im nächsten Mountain-Atom bauen (der Rat ist da, das Gate-Fixture steht schon); die Surfacing-Ebene bewusst unangetastet lassen und als eigenen Punkt führen.

### LAB_A/MLZ NSE — Substance-Witness
- **Status:** wartend | **Bindung:** eigen + operator
- **Trigger:** Asset-/CDN-Wort des Operators.
- **Lage:** (gemessen 2026-09-29, Rat) Rat: Substance witness, kein Wire-Arm. `witness.rs:26` trägt jetzt `LABR`; `lab_reader_compiler.rs:35-37` druckt die Substance-Klasse statt `pending`. 13 [RETRACTED-SAMPLE]-Läufe privat (`data/lab_a.data/…`, gitignored).
- **Blockade:** kein CDN-Asset (Operator-Einverständnis).
- **Braucht:** `phi/witnesses.φ`-Zeile (asset-url); Operator: NSE/SAMPLE_AUTHOR-Datenrechte.
- **Empfehlung:** als privates Holding führen und die `witnesses.φ`-Zeile erst setzen, wenn das Operator-Wort zum CDN fällt; die Rechte-Frage in Futures Queue.

### twomass_psc — Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Verdikt-Schritt — `phi/dead_sources.φ` (oder Manifestation).
- **Lage:** (gemessen 2026-09-29 via `archive_search --verdict`) Kandidat-Blöcke `stage/pre_cdn_lost_blocks_unpooled.φ:2024/2770/2823`; alle drei URLs HTTP 404 (direkt + proton), kein Wayback-Snapshot; `TWOMASS.twomass_psc` in `tap_index_wsa.φ:798`; `sources.φ` 0. Blöcke sind Prä-CDN und gespliced (URL/`source`-Mismatch).
- **Blockade:** kein lebendes Asset.
- **Braucht:** Manifestations-Duty (Compiler + `origin`) oder `dead_sources.φ`-Verdikt.
- **Empfehlung:** als `dead_sources.φ`-Verdikt schließen (drei tote URLs, kein Snapshot) — eine neue Manifestation wäre ein eigener Port-Auftrag, nicht dieses Atom.

### Trägerlose Docs (Mountain-Natur)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Träger-Schritt — `docs/concepts/arxiv-api.md` u. a.
- **Lage:** (gemessen 2026-09-29) vier Docs tragen Mountain-Natur-Marker: `docs/concepts/arxiv-api.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`, `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md`, `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md:87`.
- **Blockade:** keine.
- **Braucht:** Träger- oder `descoped`-Zeile je Doc.
- **Empfehlung:** die drei substanziellen Docs als Träger in die eigene Prosa-Träger-Liste heben (unten bereits geschehen); `fremde-parser-sammlungen.md:87` bleibt echter Träger — keine `descoped` nötig.

### Sonden-Flotte Asien/Russland — Compiler-Bin/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits`/`gras_2c` gebaut; `blocked_sources.φ` pds4-fits/gras-2c auf `pending`; Compiler-Bin + Live-Sample fehlen.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt.
- **Braucht:** CDN-Trigger (mycelium); Sample/Verifikation.
- **Empfehlung:** auf den ersten Live-Sample-Träger warten; der Arm steht, ein Blindbau ohne Sample wäre Fabrikation.

### HiPS-PNG (MoRIC) — Arm steht; Ernte/CDN/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium-Ernte/CDN (`hips_png_compiler --ci-mode`); Sample-Fixture.
- **Lage:** (gemessen 2026-09-29) `src/archivar/hips.rs` + `hips_png_compiler.rs` gebaut; `blocked_sources.φ` `pending`; Norder7/Dir0/Npix0.png 200 sha256 `aa6318fe…`.
- **Blockade:** Tree 12·4⁷ Kacheln, kein Einzel-Asset; kein CDN-Eintrag.
- **Braucht:** `## An mycelium` (Ernte-/CDN-Direktive + Träger); Sample-Fixture.
- **Empfehlung:** eine Einzelkachel als Fixture sichern (der Arm ist gebaut, nur ungetestet am Einzelasset); Ernte/CDN an mycelium.

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin` + Workflow.
- **Lage:** (gemessen 2026-09-29) ERDDAP griddap `nceiErsstv5`, `--verdict` 206; Verdikt-Zeile + `ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5` (`geo.rs`) gebaut; Asset nicht auf CDN.
- **Blockade:** Manifestation (mycelium).
- **Braucht:** `## An mycelium`.
- **Empfehlung:** Manifestation an mycelium; keine Mountain-Arbeit offen.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.
- **Empfehlung:** reiner Token-Wait; die Quelle ist erschlossen, nur der Zugang fehlt.

### auftrag-flyby2-kette — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** JUICE in-situ + Δ publiziert (`docs/paper/flyby-path-2-addendum-2026-09-29.md`).
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger.
- **Blockade:** JUICE in-situ + Δ nicht publiziert.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.
- **Empfehlung:** wartend, keine Vorarbeit möglich; der Trigger ist extern und datiert.

### Rätsel Ⅰ — `dr3_stars`-Record ohne σ-Spalten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Record-Erweiterung + neue Asset-Version.
- **Lage:** (gemessen 2026-09-29) `dr3_stars`-Record (44 B) trägt keine σ_ϖ/σ_pm-Spalten; ohne sie ist jede σ_z-Schätzung eine Rauschmessung (Median-Parallaxe 0,529 mas).
- **Blockade:** Record-Struktur + CDN-Version.
- **Braucht:** erweiterten Record (σ_ϖ/σ_pm) im Compiler; neue Asset-Version.
- **Empfehlung:** als eigenes Compiler-Atom führen (Record-Erweiterung + neue Asset-Version); das ist ein hartes Atom, kein Nebenprodukt.

### ODF-Flyby-Verdikt — Fenster + Shard-Riss
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-29) Keine der vier ODF-`url`-Zeilen (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; `window` fehlt; Galileo PPI-Annex-TDF deckt Earth-1, MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss: `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76`.
- **Blockade:** ODFs fehlen serverseitig; Shard-Riss-Trägerschaft.
- **Braucht:** `## An future` (ODF-Anfrage); Riss-Träger.
- **Empfehlung:** den Shard-Riss als Riss in der eigenen Übergabe tragen (nicht glätten); die DSN-Anfrage als Operator-Akt in Futures Queue.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle; Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (echter Marker `:87` astroquery-Modul-Gegenprobe).
- `docs/concepts/arxiv-api.md` | Träger Mountain (Quellen-Zugangsweg `:59`/`:65-67`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Träger Mountain (Register-Inventur `:28-141`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | Träger Mountain (`dead_sources.φ`-Erstpass `:52-70`).

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge204.

**Bitte: diese Trigger bevorzugt abarbeiten** — Mountain wartet auf vier von dir, zwei davon blockieren CI bzw. einen Build:

1. **`dropped-baseline` 1113→1127** (gemessen 2026-09-29, `ci-gate 36506672052`, delta 14) — **blockiert `ci-gate` am HEAD**; der Bump im annehmenden Commit schließt die Serie. Der Hunk liegt bereits in deinem Baum.
2. **`ephemeris_itokawa` `sources.φ`-Block** — die tsv-Registrierung ist gebaut, ohne deinen Block läuft der Build nicht:
   `url …/ssd.jpl.nasa.gov-ephemeris/ephemeris_itokawa.bin` / `format ephemeris_binary` / `origin procedure: …` / `compiler tools/harvest/src/bin/ephemeris_compiler.rs` / `at itokawa` / `no-cadence`.
3. **`rosetta-ephemeris-cdn.yml`** auf `ssd.jpl.nasa.gov-horizons/ephemeris_rosetta.bin` richten (Rat-Wort 2026-09-29); toten BSP-Anker `sources.φ:15786-15791` löschen, kanonisch `:15947-15952` — der Lauf ist sonst by construction rot.
4. **`ersstv5` / HiPS / pds3-pds4** — Manifestationen, sobald Punkt 2 steht.

Weiteres zur Kenntnis: **`format vlde`** — Zeile steht in `phi/witnesses.φ`, kein `sources.φ`-Akt. **Legacy-CDN-Assets** — kein Delete; alle vier URLs live (HTTP 206, 2026-09-29). **twomass_psc** — Kandidaten-URLs 404, Mountain neigt zum `dead_sources.φ`-Verdikt; dein Einspruch willkommen, falls du eine Manifestation beabsichtigst.

## An future (Operator-Queue, private)
Origin: mountain folge204.

**Bitte bevorzugt vorlegen**, sobald der Operator spricht:
- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets trägt das Erd-Encounter-Fenster; die ODFs fehlen serverseitig. *Frage:* DSN/JPL-Rohdaten-Anfrage stellen? (Operator-Hand)
- **NSE/SAMPLE_AUTHOR-Datenrechte** (gemessen 2026-09-29, Rat): 13 [RETRACTED-SAMPLE]-Läufe als Substance-Witness `LABR`; CDN-manifestiert oder privates Holding? (Operator-Hand)

Zur Kenntnis: Sonden-Flotte CSF/Konto-gated (CNSA, ISRO PRADAN, MBRSC EMM); CSES-/Swarm-Zugang (SSDC, Trigger 2026-10-02); GIC-Einreichung; KARI/ISRO-Konten (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).

## An river (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge204.

- **Trägerlose Docs** (gemessen 2026-09-29): `docs/concepts/kybernetische-astrophysik.md` (per-Voxel Jeans-Engine `:51-55/:355`), `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`, `docs/surveys/survey-fortschritt.md` — bitte je `descoped`/Träger-Zeile; der Scanner-Substring-Bug (`wartet ⊂ erwartet`) ist in diesem Atom geheilt (`register_lookup.rs`), die Falsch-Orphans verschwinden mit dem nächsten Bin.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
