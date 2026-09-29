<!--
  title: Handover — River-Folge 64 (2026-09-29)
  session: River-Folge 64
  class: handover
  date: 2026-09-29
  sha256: a418ffbf69ac233806f9ced013bfdbb279e573e96da19d474eb6eb738682f6ec
  status: live
-->
# Handover — River-Folge 64 (2026-09-29)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`. Nur eigene Arbeit:
pfad-begrenzter Commit; fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" / „folge dem rat" | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„hast du alle eigenen punkte bis zur kante geplant?" | 2026-09-29 | Operator (Session, River 63)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent …" | 2026-09-29 | Operator (Session, River 64) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### TE-Merge — CI-Verifikation (clippy)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`/`ci-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 64 via `ci_manage log 36503272430`) `ci-gate`
  auf `69eef305a` = failure; gemessene Ursache: 4 clippy `-D warnings` —
  `src/mathematikerin/te.rs:4413` `manual_memcpy` (river, **in diesem Atom geheilt**:
  `xf[3..].copy_from_slice(&yf[..n-3])`) + `src/archivar/hips.rs:299:9`
  (`chunks_exact_to_as_chunks`), `:338:46` (`manual_is_multiple_of`), `:408:14`
  (`needless_range_loop`) — Mountain-Datei (`b595a8f1c`). `ci-check 36503272357` pending.
- **Blockade:** die 3 `hips.rs`-clippy-Fehler (Mountain) halten das gemeinsame `ci-gate` rot.
- **Braucht:** Mountain heilt `hips.rs`; dann `ci_manage log <id>` am neuen HEAD.

### WGSL-Naht — `te_compute` Horizont
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` auf dem neuen HEAD grün (dann ohne äußeren Anlass dispatchbar).
- **Lage:** (gemessen 2026-09-28, Rat Q3) die CPU-Membran misst am Kreuz-Horizont τc,
  die WGSL `te_compute` (`shaders.rs:751/766`) weiter an (tx,ty); Produktion trägt
  `TE_KSG_K_PROD=0` (GPU-KSG-Slots absent), die Roh-Paritäts-Gates halten.
- **Blockade:** keine.
- **Braucht:** `find_cross_mi_lag` in `te_compute` spiegeln + Horizont-Parity-Gate.

### ENSO TE-Probe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gefeuert (gemessen 2026-09-29) — `ersstv5_nino34` ist registriert
  (`phi/sources.φ:11111-11117`: `url` + `origin` ERDDAP `nceiErsstv5` +
  `compiler tools/harvest/src/bin/ersstv5_compiler.rs` + `field ersstv5_nino34_ssta`).
- **Lage:** (gemessen 2026-09-29 River 63 via `register_lookup ersstv5`) die Quellenzeile
  steht; der Compiler existiert; der Blatt-/Paar-Zuschnitt ist operator-gebunden
  (`state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`, Aufnehmer river).
- **Blockade:** CDN-Manifestation (Mycelium) + Operator-Zuschnitt des Paares.
- **Braucht:** Mycelium fährt `ersstv5_compiler --ci-mode` + Release-Asset; dann die
  Probe (OMNI `omni_hro_imf_bz_gsm_nt` × `ersstv5_nino34_ssta`) nach
  `nobel_probe_corona.rs` binden — der Zuschnitt kommt vor den Operator.

### Rätsel Ⅰ — Jeans-Engine, Zensus gesetzt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keine (nächster Schritt ist der Bau).
- **Lage:** (gemessen 2026-09-29 River 64, `jeans_residuum_probe`-Sweep über
  `data/ssd.jpl.nasa.gov/dr3_stars.bin`, 1 704 587 Sterne, 0 refused) über den
  **vollen Katalog** bleibt der Median N_total je belegter Zelle 1–2 für 50→500 pc;
  Zellen ≥ 32 fallen von 8 246 (50 pc) auf 1 060 (500 pc); die Zell-Median-Parallaxe
  sinkt 0,53→0,21 mas. Die plx > 5 mas-Subprobe (11,84 % = 201 904 Sterne, within
  ~200 pc) belegt nur 408 Zellen (50 pc) / 160 (75 pc) / 8 (≥ 200 pc) → jene Zellen
  tragen je ~10²–10³ Sterne, also ≥ 32. **Der Median 1 ist ein Distanz-Gewichtungs-
  Artefakt des vollen Katalogs, nicht die Schätzer-Wahrheit.** Der Ratsbeschluss
  (vertikale 1D-Jean-Form über K_z, symmetrisierte |z|-Bins, Fehlerkorrektur Pflicht)
  steht unverändert.
- **Blockade:** σ_ϖ/σ_pm fehlen im `dr3_stars`-Record (Mountain Compiler-Fehler-Atom) —
  ohne Fehlerkorrektur ist jede σ_z-Schätzung eine Rauschmessung.
- **Braucht:** Schätzer-Modus auf der **plx > 5 mas-Subprobe** mit |z|-Bins; das
  σ-Atom (Mountain + Mycelium Manifestation) zuerst.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESOC publiziert einen SKD `v474+` mit `juice_cog_000115_…` oder einer
  dedizierten `*recon*`-SPK in `spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/`.
- **Lage:** (gemessen 2026-09-29 River 62, general) **ABSENT**: Enumeration endet bei
  `juice_cog_000114_230416_261003_v01.bsp`; Gate `data/flyby2/gate-juice-2026-09-28.json`
  recon/sigma/verdict `pending`, `riss: false`.
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon
  data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin --sigma-recon <km>`.

### Weberin-Lücke — Prosa-Träger `membran-ladearchitektur`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain liefert die `format vlde`-Quelle (dann derived-field-Wiring).
- **Lage:** (gemessen 2026-09-29 River 64) übernommen von mountain-folge202;
  `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` trägt den real offenen
  Marker `:234-235` (Vlies-Dichte/TE/Verdict als Derived-Field-Schicht; Reader
  `src/archivar/vlies.rs` steht, `format vlde` fehlt in `phi/sources.φ`); die
  Enclosure-/Jump-/ω-Loop-Punkte der Survey sind gebaut (Nachtrag `:203-233`). River
  ist Prosa-Träger.
- **Blockade:** die `format vlde`-Quelle (Mountain) fehlt.
- **Braucht:** Mountain `format vlde`; dann das derived-field-Wiring im ω()-Loop.

### Trägerlose Prosa — Orphan-Zensus
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keine.
- **Lage:** (gemessen 2026-09-29 River 64 via `register_lookup --orphan-docs`) = 2:
  `docs/concepts/kybernetische-astrophysik.md` (9 Marker: Subjekt des Aufsatzes + zwei
  `wartet ⊂ erwartet`-Artefakte; **kein Work-Item**) · `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`
  (2 Marker: beide `wartet ⊂ erwartet`-Artefakte). `docs/surveys/survey-fortschritt.md`
  in diesem Atom geheilt (Heading `:65` „Geschlossene Verbesserungen", sha erneuert).
- **Blockade:** Scanner-Substring-Bug (`tools/register/src/bin/register_lookup.rs:112`,
  `wartet` matcht in `erwartet`) — Owner Mountain.
- **Braucht:** Mountain heilt `open_marker_matches` (Wortgrenzenlogik wie `blocked`);
  bis dahin sind beide Dokumente mit gemessenem Falsch-Positiv-Verdikt getragen.

## An mountain
Origin: river folge64.

- **`hips.rs` clippy:** `cargo clippy --all-targets -- -D warnings` auf `69eef305a`
  nennt `src/archivar/hips.rs:299:9` (`chunks_exact_to_as_chunks`), `:338:46`
  (`manual_is_multiple_of`), `:408:14` (`needless_range_loop`). Hält
  `ci-gate 36503272430` rot. Braucht: die drei Lint-Fixes.
- **Rätsel Ⅰ Compiler-Fehler-Atom (getragen aus folge63):** der `dr3_stars`-Record (44 B)
  trägt keine σ_ϖ/σ_pm-Spalten; ohne sie ist jede σ_z-Schätzung eine Rauschmessung.
  Braucht: erweiterter Record + neue Asset-Version (CDN).
- **Scanner-Bug `wartet ⊂ erwartet`:** `register_lookup.rs:112` (`open_marker_matches`,
  Substring) erzeugt Falsch-Orphans in `kybernetische-astrophysik.md`/`legacy-konzepte.md`.
  Braucht: Wortgrenzenlogik wie bei `blocked`.

## An mycelium
Origin: river folge64.

- **ENSO-Manifestation (getragen aus folge63):** die `ersstv5_nino34`-Quellenzeile steht;
  sobald committet, `ersstv5_compiler --ci-mode` fahren und das Release-Asset
  `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` manifestieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte
Commit-Pfade dieser Session:

`src/mathematikerin/te.rs` ·
`docs/surveys/survey-fortschritt.md` ·
`docs/handover/handover-2026-09-29-river-folge64.md` ·
`docs/handover/archiv/handover-2026-09-29-river-folge63.md`.
