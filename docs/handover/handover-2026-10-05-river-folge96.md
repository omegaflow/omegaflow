<!--
  title: Handover — River-Folge 96 (2026-10-05)
  session: River-Folge 96
  class: handover
  date: 2026-10-05
  sha256: 57b4d797f53ca8cc87be574069afa4d136de992bb28672dd21309fa1420645c6
  status: live
-->
# Handover — River-Folge 96 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … Erst nach der Arbeit: die Übergabe fortschreiben" + „River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-05 | Operator (Session, River 96) — session-weiter Delegations-Consent, nicht das Commit-Wort
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-05-river-folge95.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 trägt die kalibrierte
  Westfall–Young-max-T-Null (α = 0.05/0.01). Offen: Verdrahtung der Bias-Korrektur
  (`TE_NEFF_THRESHOLD`), BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau,
  Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Archäologie + Dreifach-Verdikt (Kimi K3 / z.ai GLM 5.3 / Claude); Läufer für die Code-Punkte an Mountain/River/Sensory.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; Riss; zwei Hände)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain)
- **Trigger:** Mountains `aperture:<class>`-Landung (`parse.rs` + `phi/sources.φ` + `PRESENCE_FLAG_FLUX` = Bit 3).
- **Lage:** (gemessen 2026-10-05) 25 z-tragende `em`-Feldzeilen in 18 Blöcken; 3
  Koordinaten-Träger (`ned_redshift_z`, `ned_byparams_redshift_z`, `ned_gitter_redshift_z`)
  am HEAD `inverse-linear` (Mountain 237 gelandet). Die `(1+z)⁻²`-Apertur gated heute auf
  `kernel_id ∈ {0,1}` (`spatial.rs:760-768`, WGSL `shaders.rs:186-189`/`:211-216`) — ein
  Proxy: `magnetar_period_s`, `planck_psz2_snr`, `frb_dm_em`, `frb_dispersion`,
  `frb_scatter_ms`, `sn_max_mag`, `sdss_photoobj_psfmag_{r,g,i}`, `bat_fluence_erg_cm2`
  sind nicht fluss-wertig, bekämen aber `(1+z)⁻²` (gemessener Riss). **Rat-Verdikt (b):**
  die `field`-Direktive (9+-Token, `parse.rs:928`) bekommt ein positions-unabhängiges
  Token `aperture:<class>`, `class ∈ {none, flux}` (`prefix scan`, kollidiert nicht mit
  freq/bin_width); `flux` → `(1+z)⁻²`, `none` → Identität; **pflichtig**, wenn der Block
  `z <key>` trägt (fehlend → `report_anomaly`, nie Default — absent + mandatory → Record
  skipped). Ein Deklarations-Träger, drei Leser: `FieldConfig.aperture`/`Sample.z_flux`
  (Rust), Wire-Bit `PRESENCE_FLAG_FLUX = 8.0` (`types.rs:306-308`, `presence_flags`),
  WGSL `(u32(mt3.z) & 8u) != 0u`; Wire-Größe bleibt 26 × f64 (`static/constants.js:148`
  forwardet unmaskiert). Die volle Antwort-Tabelle (magnitude/period/…) ist benannt,
  nicht gebaut — kein gemessener Bedarf. `redshift_response(flux, z)` in `spectral.rs`
  als einzige CPU-Transform-Quelle.
- **Blockade:** das `aperture:`-Token, das `z_flux`-Feld, Bit 3 und der CPU-Gate fehlen am
  Baum (`sgrep 'aperture:' src/archivar/parse.rs` = leer) — Mountains Register-/Parser-Akt.
- **Braucht:** Mountain landet Deklaration + Parser + Bit + CPU-Gate (`spatial.rs:760-768`
  auf `sample.z_flux`) und die 25 `phi/sources.φ`-Zeilen deklariert; dann flippt River die
  WGSL-Gates `shaders.rs:186-189`/`:211-216` auf `(u32(mt3.z) & 8u) != 0u` und baut den
  GPU-Paritätstest (`mathematikerin/tests.rs`, Adapter `:1033`: flux/Bit gesetzt, z=1 →
  0.25; Bit klar → unskaliert; `kernel_id=1` ohne Bit, z=1 → unskaliert = Riss-Regression).
  CI, nie lokal; CPU/GPU-Parität ist das Tor.

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23,
  beste Qualität 1999–April 2005, nicht-uniform; NUR nur als SuperMAG-Station.
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN;
  (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Universelles Vlies — `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-05 via `fd -i ozzy` = leer, `sgrep -i ozzy` nur Spec)
  `ozzy` ist Spec, keine Quelldatei; `field_te_query` fährt Einzel-Paare. Der
  universelle Rahmen steht (ICRS baryzentrisch, DE/INPOP/EPM, Gaia DR3, 2MRS).
- **Blockade:** keine (Rat-Verdikt 2026-10-05 liegt; Matrix-Form steht am Baum).
- **Braucht:** (Reihenfolge) (1) Bias-Kurve über n messen (`te_bias_n_probe`
  erweitern) + `TE_BIAS_MK_EMBEDDED` → (2) die 8 probe-gelesenen Kanäle an den Draht +
  15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`) → (3) `ozzy`-Bibliothek;
  (4) Netz-Null-CI (B ≥ 1/α). Ernte parallel (Mountain/Mycelium/Future) — nicht Rivers
  Hand. **Ozzy läuft auf der Matrix, nicht davor.**

### Agnosis — Membran-Trio & Presence-Volume (River-Hand)
- **Status:** eigen | **Bindung:** eigen (cross-line: Mountain, Sensory)
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-06) `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` — der serverlose Pfad rendert nur diese drei; `omega.rs:878` bindet den Presence-Volume-Bin hart an „earth"-geodätisch; `/jump/<body>` kennt zur Laufzeit nur `earth`. Dreifach-Verdikt: `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md`.
- **Blockade:** keine.
- **Braucht:** Trio → Build-Time-Manifest aus der Hüllen-Pipeline (kein Body-Name im File); `omega.rs:878` → SSB-Rahmen oder deklarierter Rahmen.

## An mountain (Feld-Gesetz + Register)

Origin: river folge96.

- **EMM `emm_exi_l2a` — Feldzeile fehlt.** Der Loader-Arm ist gebaut und committed
  (river 96): `src/archivar/emm_exi.rs` (SDC-Archiv → gunzip → FITS-Primary `DATE-OBS`
  + `SCI`-Image-Mittel → `(t, mean, COMP_RADIANCE)`), `extract.rs`
  `series_parse_bin`/`series_component_name`-Arme, `main_flow.rs:2905` Dispatch-Token.
  Die Quelle `phi/sources.φ:17252-17256` trägt `format emm_exi_l2a` **ohne `field`-Zeile**
  → der Zweig bricht mit „field undeclared" ab (0 Oszillatoren). **Braucht:**
  `field emm_exi_radiance emm_exi_radiance inverse-square em <unit> <ttl> <freq> <bin_width>`
  (Component-Name `emm_exi_radiance`, `COMP_RADIANCE = 0`), plus die Klassen-Zeile
  `aperture:none` (kein z in dem Block).
- **em-Apertur (Rat 2026-10-05) — Mountains Hand.** Siehe eigener Punkt oben:
  `aperture:<class>`-Grammatik in `parse.rs:928`, `FieldConfig.aperture`/`Sample.z_flux`,
  `PRESENCE_FLAG_FLUX = 8.0` in `types.rs`, `presence_flags`-Arm, CPU-Gate
  `spatial.rs:760-768` auf die Wertklasse, und die 25 `phi/sources.φ`-Zeilen deklariert
  (`flux` für `_mjy`-Flussdichten + `bat_fluence_erg_cm2`, `none` für die übrigen).
  Rivers Hand (WGSL-Gate-Flip + GPU-Paritätstest) folgt in demselben Atom.

- **Agnosis — Anchor-Bypass + `frames.rs`-Defaults.** (gemessen 2026-10-06)
  `spawn_ephemeris_bootstrap` lässt Anker-Körper `body_in_enclosure` umgehen
  (`main_flow.rs:473-495`) und ordnet per `anchor_uses` (`:403-427`, Earth zuerst
  → Erdbias-Reihenfolge); `frames.rs:6,86,114,117` defaulten terrestrisch →
  „on earth", zöliakal → „at sun". Dreifach-Verdikt
  (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` §3): Bypass entfernen
  (Zulassung nur Hülle); Inferenz verweigern, nur Deklaration erlauben.
  **Braucht:** Mountain (Loader/Register) baut den Bypass aus und materialisiert
  die Defaults als Per-Source-Deklaration.

## An sensory (Weberin)

Origin: river folge96.

- **Gaia-Reduktions-Observer hart „earth".** (gemessen 2026-10-06) `weberin.rs:1801`
  `body_barycenter_position("earth", t.tdb, eph)` als astrometrischer Observer;
  Gaia ist baryzentrisch → Observer = SSB-Presence, nicht „earth". **Braucht:**
  Observer als Pflicht-Parameter + `observer=<…>`-Deklaration, in der Provenienz
  echoed (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` §3 Q2).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/emm_exi.rs`
- `src/archivar/extract.rs`
- `src/archivar/main_flow.rs`
- `src/archivar/mod.rs`
- `src/archivar/membrane.rs`
- `src/mathematikerin/s2.rs`
- `src/mathematikerin/shaders.rs`
- `docs/handover/handover-2026-10-05-river-folge96.md`
- `docs/handover/archiv/handover-2026-10-05-river-folge95.md` (Move aus `docs/handover/`)

## Burn: open 0.0000 · close 0.0660 — `session_burn` River-Linie
(Session „River-Linie in einem Pass starten"; 2 `grind-flash`-Taucher: EMM-Arm + Presence-Bit-Leser; 1 Rat).
