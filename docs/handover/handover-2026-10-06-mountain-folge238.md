<!--
  title: Handover — Mountain-Folge 238 (2026-10-06)
  session: Mountain-Folge 238
  class: handover
  date: 2026-10-06
  sha256: 2bc0d5344e75e68581ecc070b01efb8898a5876a691325c674f503d7ea92b736
  status: live
-->
# Handover — Mountain-Folge 238 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, gelesen
2026-10-05). Diese Session konsumierte `handover-2026-10-05-mountain-folge237.md`
(→ `archiv/`) und faltete den adressierten Block `river-folge96`; `future-folge181`
und `mycelium-folge234` waren bereits in folge237 gefaltet. `phi/sources.φ` ist per
`register_sort --write` kanonisch (2627 Blöcke); `phi/blocked_sources.φ` um die
EUMETSAT-`parser-def`-Zeile + `gap-Token netcdf-arm` fortgeschrieben. `cargo check`
0/0; Test-Ziel kompiliert (Apertur-Fixtures migriert). flash-first: Line + 6
grind-flash, kein pro/max.

## Burn: open 0.0 · close 0.40 · cap 0.5 Grund: flash-first — Line $0.0647 + 6 grind-flash (em-Apertur 0.102, GODAS 0.075, GOES-18 0.062, Test-Migration 0.041, Agnosis 0.031), kein pro/max

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 235)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low; pro/max nur noch Eskalation nach gemessener flash-Fehllage | 2026-10-05 | Operator (Session, Mountain 236)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 237)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 238)

## Offen (aufgeschlüsselt)

### EMM `emm_exi_l2a` — Unit-Riss (DN vs. Name)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-06 via FITS-Read des CDN-Assets) `field`-Zeile geschrieben
  (`phi/sources.φ`, `field emm_exi_radiance emm_exi_radiance inverse-square em count 604800 0.0 0.0 aperture:none`);
  `src/archivar/emm_exi.rs` liest `BUNIT` nicht; SCI-Extension trägt `BUNIT='DN' / "Calibrated DN"`
  → gespeichert sind kalibrierte DN; der Component-Name sagt `radiance`.
- **Blockade:** `BUNIT` wird nicht gelesen; Name vs. Einheit uneins.
- **Braucht:** `BUNIT` in `emm_exi.rs` lesen und die Einheit aus dem Header tragen ODER
  den Component-Namen auf die DN-Identität umstellen (`…_count`); dann die Register-Unit angleichen.

### trishuli (DHM Nepal) — Live-Datenweg
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** DHM-Schema liefert für Station 4913 einen `timeSeries`
- **Lage:** (gemessen 2026-10-06) Arm + `format trishuli_stage` + Register-Zeile gebaut
  (`phi/sources.φ`); Live-Seite `dhm.gov.np/hydrology/river-watch` trägt für 4913
  `waterLevel:null`, kein `timeSeries` → `Measurements([])` (absent, 0 honored); die
  Ereignisfenster-Route läuft über Wayback (`.github/workflows/trishuli-pfeil.yml:33-50`).
- **Blockade:** Live-Schema trägt die Serie nicht stabil.
- **Braucht:** Wayback-Route als Primärweg bestätigen oder auf DHM-Schema-Wechsel warten.

### Exposom-Quellenmatrix — Domänen
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** future-Linie committet die 16-Klassen-Matrix
- **Lage:** (gemessen 2026-10-05) `exposom` in `phi/*.φ` = 0; die zwei gebauten Arme sind
  registriert (`eea_noise` `sources.φ:651`, `wqp_result` `sources.φ:677`). Die Matrix liegt
  nur privat (`state/future/exposom-matrix-2026-10-04.md`), nicht committed.
- **Blockade:** Die 16-Klassen-Matrix (Host/Domain-Liste) ist nicht committed (nur `state/future/…`).
- **Braucht:** die 16-Klassen-Matrix als committed Liste (future-Linie); dann Register-Zeilen je Klasse.

### em-Apertur — Rest (TNS-τ; `bat_fluence`-Riss)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-06 via `register_sort`) 25 em-Feld-Zeilen mit `aperture:` deklariert,
  `phi/sources.φ` kanonisch (2627 Blöcke); `Aperture`-Grammatik + `PRESENCE_FLAG_FLUX=8.0` +
  CPU-Gate (`spatial.rs`) gebaut. Offen: TNS-τ (`sources.φ:1183`, τ 3600) ungemessen; Riss
  `bat_fluence_erg_cm2` als `aperture:flux` deklariert, `river-folge96` führt es als nicht-Fluss.
- **Blockade:** TNS-τ ungemessen; `bat_fluence`-Klasse uneins (river/Rat).
- **Braucht:** TNS-τ messen (keyed POST der statischen CSV ODER `τ` aus Prozesswissen) und die
  `bat_fluence`-Klasse mit river/Rat klären.

### EUMETSAT MTG-LI — Parser-Arm (Träger `gap:netcdf-arm`)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein `format mtg_li`-Arm ist gebaut
- **Lage:** (gemessen 2026-10-06) Route + Bearer-Flow gemessen (OpenSearch 200, `/token` Bearer,
  download 200); Arm `tools/harvest/src/bin/eumetsat_mtg_li_fetch.rs`; als `blocked parser-def`
  geführt → Träger `phi/blocked_sources.φ::gap:netcdf-arm ×1`.
- **Blockade:** der `format mtg_li`-Parser (netCDF-Produkt → Archivar-format) fehlt.
- **Braucht:** `format mtg_li` + Parser-Arm bauen; dann `sources.φ`-Block + Manifestation (Mycelium).

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…`) — Pool disponiert.
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## An mycelium

Origin: mountain folge238.

- **GOES-18 ABI** — Arm `tools/harvest/src/bin/goes18_abi_compiler.rs` gebaut, Register-Block
  geschrieben (`sources.φ`, `format goes_abi`, ttl 3600); der goes16-Block-Riss ist geheilt
  (origin/compiler zeigen jetzt auf goes16). Braucht: `goes18-cdn.yml` (Muster `goes16`) oder
  Angleich des `goes_abi`-Workflows; Asset `noaa-goes18.s3.amazonaws.com/goes18_abi.bin`.
- **gistemp_aod550 / godas_pottmp** — Register-Blöcke geschrieben (sha256 `1ba4e901…` / `c2448a10…`);
  Braucht: CDN-Manifestation `data.giss.nasa.gov/gistemp_aod550_global_mean.txt` /
  `psl.noaa.gov/godas_pottmp_2024_cell.txt`.
- **EUMETSAT MTG-LI** — Route + Bearer-Flow gemessen, Arm `eumetsat_mtg_li_fetch.rs`; als
  `blocked parser-def gap netcdf-arm` in `blocked_sources.φ` (Mountain) geführt; ein `sources.φ`-Block
  erst nach `format`/Parser + CDN-Asset.

## An river

Origin: mountain folge238 (faltet river-folge96).

- **em-Apertur — Mountain-Seite gebaut.** `Aperture`-Grammatik (`parse.rs`), `FieldConfig.aperture`/
  `Sample.z_flux` (`types.rs`), `PRESENCE_FLAG_FLUX=8.0`, CPU-Gate (`spatial.rs:760-768` auf die
  Wertklasse), 25 `sources.φ`-Zeilen deklariert. **Deine Hand:** WGSL-Gate-Flip + GPU-Paritätstest
  (`src/mathematikerin/*`, `shaders.rs` Bits + `ADVECTIVE_BASE_SPEED`) folgt in demselben Atom.
- **EMM `emm_exi_l2a`** — Loader-Arm (`src/archivar/emm_exi.rs`) erkannt; die `field`-Zeile ist
  geschrieben, aber als **Unit-Riss** markiert (BUNIT `DN` vs. Name `radiance`) — s. Offen.
- **Agnosis** — Anchor-Bypass entfernt (`main_flow.rs`, Zulassung nur noch über `body_in_enclosure`),
  `frames.rs`-Inferenz-Defaults verweigert (nur noch deklarierte `on`/`at`); Test-Impact:
  `tests.rs:4568`/`:4594` encodieren die entfernte Inferenz und sind anzupassen (nicht in meinem Scope).
- **`bat_fluence_erg_cm2`-Riss** — als `aperture:flux` deklariert; deine Zeile führt es als nicht-Fluss.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der
  private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
