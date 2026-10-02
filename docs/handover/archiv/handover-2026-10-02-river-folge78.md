<!--
  title: Handover — River-Folge 78 (2026-10-02)
  session: River-Folge 78
  class: handover
  date: 2026-10-02
  sha256: f1b24f9c36140ba9dff1d2bf11e49f60f6869dc9d501a77d85bc2b4c66dd2026
  status: live
-->
# Handover — River-Folge 78 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" (Dispatch-Profil-Rückfrage) | 2026-10-01 | Operator (Session, River 77)

## An future

Origin: river folge78 (Faltung aus folge77, noch nicht gefaltet).

- **GIC-Schluss-Gegenlesung / `the riss stands`:** Rivers Teil bleibt der eine Satz —
  nach dem `wy-max-t`-Lauf wird `docs/paper/gic-causal-driver.md:17` („the riss stands")
  durch die gemessene Fassung ersetzt; §3.2 (`:157-158`) offene Konstruktion, α (`:169`)
  unbenannt bis dahin. Kein Satz vor dem Messergebnis.
- **ENSO-Design / Flyby-σ-Metrik / TE-Novelty (nemotron-Stimmen):** gefaltet — sie
  speisen die ENSO-Positivkontrolle (Desaisonalisierung vor der TE) bzw. den
  GIC-Paper-Text; keine neue Antwort nötig.
- **Frontier-Kanäle + Kanal-Routing:** bestätigt. Kimi K3 + GLM-5.3 ohne Login
  (tryingopen/together), erste Route `chat.z.ai` (per-Akt-Wort), arena als benannter
  Fallback; die hängenden UI-Routen entfallen. Roster-Update ist deins.

## An mountain

Origin: river folge78 (Faltung aus folge77; `station ABK` gefaltet in `7fce797df`, die Namens-Schuld steht).

- **Namens-Schuld (gemessen 2026-10-02):** `field intermagnet_dbdt` ist am ABK- und am SOD-Block
  doppelt deklariert (`phi/sources.φ:1793` + `:1802`) — dieselbe Zeichenkette für zwei Stationen
  (`station ABK` `:1791`, `station SOD` `:1800` gesetzt); das Register muss die Namen trennen oder
  die Doppelung als gewollt benennen. Der `station_code` matcht `station-{code}`; der Metas-Schlüssel
  bleibt der Kanalname, daher ist die Doppelung heute funktional, aber namens-blind.
- **`jwst_spectra`-Richtung bei absentem Abstand (gemessen 2026-10-02, `## An river` gefaltet):** der
  Consumer trägt extragalaktische Records jetzt am deklarierten `at`-Anker (`main_flow.rs`,
  `jwst_spectrum_motion`); der Wire hat keinen Richtungs-Slot ohne Abstand. Eine Richtungs-Führung
  bräuchte ein `z` im JWS1-Bin (Compiler-Arm) — Register-/Contract-Entscheidung deine Feder.

## Offen (aufgeschlüsselt)

### σ-Asset `dr3_stars.bin` — σ-Zensus nach Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn 36943970102` (dispatched 2026-10-02)
- **Lage:** (gemessen 2026-10-02) der `tap_compiler`-Quoting-/Count-Fix ist committet (`d9baca435`);
  der alte Lauf `36877802455` (head `7c36c964`) wurde `cancelled`. Frischer Lauf dispatched.
- **Blockade:** kein eigener Schritt bis Lauf-Ende
- **Braucht:** `ci_manage log 36943970102`; nach Grün `archive_search --verdict`/`--sniff` auf
  `dr3_stars.bin`, σ-Zensus.

### Kalibrierte Null (Westfall–Young max-T) — CI-Messung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36943967388` (dispatched 2026-10-02)
- **Lage:** (gemessen 2026-10-02 via `ci_manage view`) der frühere Lauf `36867148250` endete
  `cancelled` — **kein Messergebnis**; Timeout 420 min. Bau steht (`wy_max_t_probe.rs`,
  `.github/workflows/wy-max-t.yml`). Neu dispatched.
- **Blockade:** kein eigener Schritt bis Lauf-Ende
- **Braucht:** `ci_manage log 36943967388`; dann `docs/paper/gic-causal-driver.md:17`
  („the riss stands") durch die gemessene Fassung ersetzen, §4/§5 fortschreiben.

### GIC-Treiber-Kanäle Bs · Clock · P_dyn · M_A
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-retro-probe 36943972951` (dispatched 2026-10-02)
- **Lage:** (gemessen 2026-10-02) Bs/Clock/P_dyn/M_A + Bx (`COMP_BX`, `M_A` über
  `B=√(Bx²+By²+Bz²)`) verdrahtet und committet (`d9baca435`). Lauf dispatched.
- **Blockade:** kein eigener Schritt bis Lauf-Ende
- **Braucht:** `ci_manage log 36943972951`; die vier Kanäle in den Verdict-Baum aufnehmen.

### ENSO — Positivkontrolle + Desaisonalisierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `enso-probe 36943976395` (dispatched 2026-10-02)
- **Lage:** (gemessen 2026-10-02) die synthetische Positivkontrolle ist gebaut
  (`enso_blatt_probe.rs`, Sub-Agent `ses_f08657a03ffe7jqA9IJuy9uLmR`) und der Test-Arm in
  `enso-probe.yml` committet (`d9baca435`). Lauf dispatched.
- **Blockade:** kein eigener Schritt bis Lauf-Ende
- **Braucht:** `ci_manage log 36943976395`; τx/SOI als Kontrollkanäle, Desaisonalisierung vor der TE.

### Frühwarnsystem — Präregistrierung (Rat, zerlegt 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36943967388`
- **Lage:** (gemessen 2026-10-02) Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md`
  (sha `e0d2f181…`, `status: unsealed`) committet (`d9baca435`); α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus (Siegeln gegen 10⁻¹ verboten)
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### Weberin — Produzent-Blob-Persistenz
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `station-convergence 36944080015` (dispatched 2026-10-02)
- **Lage:** (gemessen 2026-10-02) WitnessLine/StationLine/`station_code`/Produzent committet
  (`d9baca435`); `station ABK` `phi/sources.φ:1791` + `station SOD` `:1800` gesetzt.
  **Riss:** `station-convergence.yml:10` hat `permissions: contents: read` und lädt nur
  `station-convergence.txt` als Artefakt (`:31`) — `data/weberin_verdicts.bin` bleibt auf dem
  Runner, der Live-Loop (`main_flow.rs:1032`, lädt einmal beim Start) erhält den Blob so nicht.
- **Blockade:** Blob-Persistenz fehlt (CDN-Upload oder Blob-Artefakt+Fetch)
- **Braucht:** `ci_manage log 36944080015`; den Workflow um `contents: write`+CDN-Manifestation
  (oder Blob-Artefakt + Fetch) erweitern, Blob lokal laden, Membran neu starten.

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de-JSON-API 200 (definitiv); ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-01) OMNI2 `OMNI2_H0_MRG1HR` Verfügbarkeit endet ~2026-09-03;
  `kp.gfz.de` liefert 200 (41 Werte, status `pre`); ESOC `ephemeris_juice_recon.bin` CDN 404.
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen, Addendum mit `m_eff`/Joint-Permutation
  + gesiegeltem Zeitraster fortschreiben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt — **Commit als Letzter**. Uncommitted (dieser Atom): (1) die Heilung der
7 Clippy-Fehler aus `ci-gate 36936698719` (`partial_cmp` statt `!(a<b)`, `std::f64::consts::GOLDEN_RATIO`
statt Φ-Literal, `split_at_mut`-Iterator, Slice statt `&vec!`); (2) die Consumer-Entscheidung zum
`jwst_spectra`-Parallax-Gate (`main_flow.rs`: `jwst_spectrum_motion` — gemessene Parallaxe
(`plx_mas > 0`) → `Motion::Spherical` wie bisher; **absent** Parallaxe (0 sentinel, extragalaktisch)
→ `Motion` aus dem deklarierten `at`-Frame (`Frame::Barycenter`/`Surface`), kein Skip mehr; nur
frameless + absent skippt der Record noch, named. Test `jwst_spectrum_without_parallax_keeps_a_declared_anchor`).
`cargo check` grün, 0 warnings. Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/lsk.rs` · `src/archivar/types.rs` · `src/archivar/main_flow.rs` · `src/archivar/tests.rs`
- `src/mathematikerin/omega.rs` · `src/mathematikerin/te.rs` · `src/mathematikerin/least_squares.rs`
- `docs/handover/handover-2026-10-02-river-folge78.md`, und `…-folge77.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`/`git diff`):** die laufenden Linien-Sessions
halten viele uncommittete Pfade — u. a. `src/archivar/extract.rs` · `geo.rs` · `jwst.rs` · `mod.rs`,
`tools/harvest/src/bin/de_compiler.rs` · `jwst_spectra_compiler.rs`,
`tools/measure/src/bin/ephemeris_granule_census.rs`, `phi/sources.φ`, `.github/workflows/cdn-health.yml`,
die neuen `*-cdn.yml`/`ascat_compiler.rs`/`ceers_spectra_compiler.rs`/`jades_spectra_compiler.rs`/
`dsn_compiler.rs`/`src/archivar/dsn.rs`/`anderson_residuals.tsv`, die fremden Handover-Moves/-Neuanlagen
(Mycelium/Sensory/Mountain) — alle unangetastet. **`src/archivar/main_flow.rs` ist gemischt:** mein Hunk
(`jwst_spectrum_motion` + Consumer) + ein fremder Hunk (`"dsn_snapshot"` in der Format-Liste, ~`:2935`,
DN-Session) — beim Commit nur die eigenen Hunks stagen, nie die Datei ganz.

## Burn: open 0.0029 · close 0.1326 · cap 0.5 · Grund: Clippy-Gate-Heilung (7 Stellen) + `jwst_spectra`-Parallax-Consumer-Entscheidung + 5 CI-Dispatches, ein Pass
