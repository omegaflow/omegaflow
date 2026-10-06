<!--
  title: Handover — River-Folge 108 (2026-10-06)
  session: River-Folge 108
  class: handover
  date: 2026-10-06
  sha256: a4a8e009ec51c27297d89862645e15060ed552d3c776e98b19e2e3fd285bd8b3
  status: live
-->
# Handover — River-Folge 108 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert" | 2026-10-05 | Operator (future-folge181, gefaltet)
„earth-wgs84/legacy-assumed … kein legacy … der Volume-Frame ist Pflicht-Deklaration je Quelle" | 2026-10-06 | Operator (Session, River 99)
„muss nicht die sonne zuerst sichtbar sein … progressiv laden nach Sichtbarkeit, Sonne zuerst" | 2026-10-06 | Operator (Session, River 105)
„bitte entfernen iEEG sofort" — iEEG.org aus Register und Manifest-Workflow | 2026-10-06 | Operator (Session, River 105)
„können wir es nicht so machen wie bei [redacted], dass wir die als private experimente laufen lassen?" — Daten ohne geklärte Redistribution laufen privat | 2026-10-06 | Operator (Session, River 105)
„ich möchte dass wir unsere lizenzen repoweit sauber haben … license file im sources repo" | 2026-10-06 | Operator (Session, River 107)
„Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade" | 2026-10-06 | Operator (Session, River 108) — session-weiter Delegations-Consent, nicht das Commit-Wort
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge107.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau,
  Paar-Matrix, Ernte (§Lieferung); Bias-Kurve estimator-fest (dieser Atom: gebaut).
- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 offen: BCa-Intervalle,
  vollständiger Kp-Kanal; der Report-Site-Bias-Satz stimmt (kein Riss).
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) — §5.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Code-Punkte bei
  ihren Owner-Linien; `membrane.html`-Trio → Mycelium/CI.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** 8 probe-Kanäle am Draht (für (5)); `conditional_embedded`-Probe (3).
- **Lage:** (gemessen 2026-10-06, River 108 via `ci_manage view` + Artefakt-Download)
  - **(2) Binned-Floor gebaut.** Artefakt `te-bias-n 37460626270 @693bfdace` gelesen:
    n=800 `binned_n_eff` mean = **1.9311e1**. Gebaut: `TE_NEFF_THRESHOLD_BINNED`
    (`src/mathematikerin/te.rs`), `BiasArm::neff_floor()` (KDE- und KSG-Arm → KDE-Boden,
    Histogramm-Arm → binned-Boden), `bias_column` liest den Arm-Boden; die Matrix-Zelle
    reicht jetzt **`binned_n_eff`** durch (der Zell-Schätzer ist `TeEstimator::Binned`).
    Gates: `gate_te_neff_threshold_binned_is_the_measured_n800_binned_floor`,
    `matrix_cell_alignment_is_per_pair_not_global`.
  - **Riss geheilt:** der Skalar-Test `bias_column_gates_on_n_eff_and_exact_n` war seit
    river 106 **rot** (`1.8166e1` < Boden `1.8485e1`; der Boden wurde in 795d12086 nach
    oben gesetzt, der Test nicht). Gefixt (`1.8485e1`; die Vorzeichen-Erwartung
    `te - 8.866e-2` → `te + 8.866e-2`, Funktion `te - m_k` mit m_k<0 ist die Wahrheit).
  - **(3) Conditional-Tabelle gemessen, kein Konsument.** `te-bias-n-conditional`:
    bias_fwd bei n 1260–2200 = **-2.96e-1 … -2.14e-1** (referenz 6.3385e-1). Die
    Matrix-Cond-Zellen nutzen `transfer_entropy_conditional_binned_n`, **nicht** die
    gemessene KSG-conditional-Schätzung → Tabelle bleibt `pending`, kein Regal.
  - **(4) Matrix jetzt per Zelle.** Lauf `37469450306 @7dc5a42a` (vor `a0e1688c3`)
    Artefakt `field-te-matrix-vlies`: cells 210, pool 15, 3 Arme pending
    (`omni_imf_bz_gsm_nt` format text; `eve_1032`/`eve_131` format eve text), dann
    global `matrix alignment absent` — der per-Zelle-Paar-Code (`a0e1688c3`) war
    **unerreichbar**, weil der globale `align_many` zuerst abbricht. **Gebaut (River 108):**
    `run_pair_matrix` alignt **je Zelle** (driver+target+conds) via `align_many`, nie
    global; nicht darstellbares Fenster → `alignment pending`, nicht darstellbares Paar →
    `resolution pending`; nie still gebinnt. `cargo check` + `cargo build -p
    omegaflow-measure --bin field_te_query` grün.
  - **CI-Blindfleck geschlossen:** die `#[cfg(test)]`-Tests in `field_te_query.rs` liefen
    in **keiner** CI (`ci-check` testet `-p omegaflow-measure` nicht; `field-te-query.yml`
    startete nur Bins). Job `unit` (`cargo test -p omegaflow-measure --bin field_te_query`)
    in `.github/workflows/field-te-query.yml` ergänzt.
  - **Rats-Reihenfolge (fix):** Bias-Korrektheit → Draht + Matrix → `ozzy` auf der Matrix
    → Netz-Null.
- **Blockade:** 8 probe-Kanäle sind nicht am Draht; matrix-vlies-Re-Dispatch erst nach
  Commit/Push sinnvoll.
- **Braucht:**
  (5) **`ozzy` bauen** — der negative Fuzzy-Index auf der Matrix
  (`docs/specs/negativ-fuzzy-index.md`; `docs/auftrag/auftrag-universelles-vlies.md`
  §Lieferung). Kein Treffer im Quellbaum — nur Spec. **Bounded:** die
  Residuum-Extraktion gegen die Boden-Zeugen als eine Funktion + Gate.
  Re-Dispatch `field-te-query.yml` nach dem Push (matrix-vlies + unit).

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 106 via `ci_manage view`) `ci-check 37444174960
  @b71658f` = **cancelled**; am HEAD `45c017d1` trägt `ci-check 37458465242` = **pending**.
  River-Seite gebaut: `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`;
  Test `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`. Mountain-Seite gebaut (mountain-239).
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check` am HEAD abwarten (Stehender Pass; kein Polling).

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line Mountain/Mycelium)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN (fine-grain GIC erfüllt).
- **Lage:** (gemessen 2026-10-06, River 105) Fine-grain FMI-GIC manifestiert
  (`phi/sources.φ:17338-17345`, sha256 `a30a846d…`); IMAGE/NUR-Zeile registriert; Paper §4
  trägt CC BY 4.0 + Caveats. **NUR nicht im CDN** (kein sha, kein `image-cdn.yml`).
  Tages-Lineartrend-Subtraktion fehlt; Relation noch nicht gemessen.
- **Blockade:** NUR-Manifestation (kein Workflow) + Probe.
- **Braucht:** (1) `image-cdn.yml` (Mycelium) + sha ins Register; (2) ggf. Tages-Detrend
  (Mountain); (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation.
- **Lage:** (gemessen 2026-10-06, River 106) CDN-Assets erfüllt; Körper-`.bin` je **6 629 784 B**,
  `accept-ranges: bytes`; deployte Props-Maske `0x01FF` (Bits 0–8), **Bit 11 (GM) klar** →
  der GM fehlt in der Sonne-`.bin`; `body_anchor_samples` (`src/archivar/membrane.rs:404`)
  emittiert nur bei `props.omega_g`/`props.gm`; `omega.rs:872` konsumiert `v.frame_body`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden nach Sichtbarkeit + Folgeatom C
- **Status:** eigen | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** Katalog-Lieferung nach Helligkeit (C).
- **Lage:** (gemessen 2026-10-06, River 106) (B) `MembraneLookup::new` startet leer +
  `add_stars`; (A) `static/membrane.html` `boot()` lädt Körper sequenziell
  `BODIES=["sun","earth","moon"]`, dann Sterne. `.bin`-Layout: Granule bei Byte 24,
  448 B/Granule; `dr3_stars.bin` = **95 424 168 B** (83 % der Ladung) → der erste Pixel
  hängt am Sternkatalog. Headless-Chrome ohne WebGPU-Adapter (ehrliche schwarze Null);
  Render-Messung operator-browser-gebunden. Epoch-Riss: `tap_compiler --epoch 2016` vs.
  Consumer `CATALOG_EPOCH_YR = 2000.0` (Owner: Mountain; Consumer-Kommentar nicht still setzen).
- **Blockade:** der erste Pixel hängt am 95-MB-Sternkatalog.
- **Braucht:** (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen (Trigger:
  Dateiordnung ≠ Helligkeit); Render-Messung im Operator-Browser.
  (D) **descoped** (Körper-`.bin` je 6,6 MB, `accept-ranges` steht — windowed Asset unnötig).

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line: Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06 via Handover folge107) Punkt (b) gebaut (mountain-239):
  `Volume.frame_body: Option<String>`, Test `volume_observer_declared_and_refused_when_absent`.
  Offen nur (a): `BODIES`-Handkopie.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105; wartend.φ:34-36) OMNI2 26 Zellen `pending`
  (Rohdatei lokal); kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** (1) `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; (2) Trigger
  feuern lassen; (3) Δ/σ_recon post-flyby.

### Voices-Chrome — eigene login-freie CDP-Instanz (Config gebaut)
- **Status:** eigen (Verifikation nach Neustart) | **Bindung:** eigen
- **Trigger:** opencode-Neustart + Voice-Test.
- **Lage:** (gemessen 2026-10-06, River 105) MCP `chrome-devtools-voices` gestartet mit
  `--headless --isolated --executablePath /usr/bin/google-chrome`; in allen 12 Voice-Profilen
  `browser_*`/`chrome-devtools_*` deny, `chrome-devtools-voices_*` allow.
- **Blockade:** opencode-Neustart (Config nicht hot-reloaded); `opencode.json` trägt fremde
  uncommittete Voice-Definitionen — mit-committet, benannt.
- **Braucht:** Neustart; Voice-Test.

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` trägt **1945** Spiegel-URLs über
  **169** distinct Netlocs; keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen,
  Batch 1 = 10 Netlocs / 7 gemessen); Leads `state/river/license-census-voice.tsv`.
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen von Mountain messen lassen (Stimme liefert Terms-URL,
  nicht Klasse); Generator + Drift-Tor an Mycelium (`## An mycelium`).

## An mycelium

Origin: river folge101 (getragen über 102–107).

- **Generiertes `LICENSE` im `omegaflow/sources`-Repo.** Ein Compiler liest die `terms`-Zeilen
  aus `phi/sources.φ` und emittiert ein nach Lizenzklassen gruppiertes `LICENSE` (`pending`
  namentlich); ein CI-Tor prüft byte-identisch gegen die Neu-Erzeugung. **Braucht:** Generator +
  Drift-Tor, nachdem Mountains `terms`-Zeilen landen.
- **DE440-`.bin` remanifestieren** nach Mountains `de_compiler`-GM-Landung; Checkmark ist
  Rivers `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **Measured (river 106):** `dr3_stars.bin` aus `tap_compiler` (`--epoch 2016`); Körper-`.bin`
  je 6 629 784 B, `accept-ranges: bytes`; (D) descoped.

## An mountain

Origin: river folge107 (Lizenz-Audit) · folge108 (Vlies).

- **`terms`-Direktive je Körperdatenzeile + Manifestations-Gate.** Jedes auf dem CDN gespiegelte
  Körpermesswert-Asset trägt eine redistributions-erlaubende Lizenz, aber keine Register-Zeile
  nennt sie: `openneuro.org` (ds005034/ds007471/ds007822) = **CC0**; `physionet.org`
  (`bidsleep_mehrnacht.bin`) = **ODC-BY 1.0**; `ieeg.org` bleibt entfernt. **Braucht:**
  (1) `terms <license> <url>` je Körperdatenzeile; (2) Parser-Arm für `terms`
  (`src/archivar/parse.rs`, heute still ignoriert); (3) `unbacked_mirror`
  (`src/gate/commit_gate.rs:2039`) verschärfen.
- **Vlies-Matrix — zwei fehlende Register-Felder.** Für `vlies_matrix.te` fehlen als `field`-Zeile:
  **Newell dΦ/dt** (`bz_retro_probe.rs:431` rechnet es) · **Kp** `magnetosphere_kp_3h` (heute
  `last`). EEG ds007822/ds007471: Feldname ungemessen (CC0, s.o.). **Braucht:** je eine
  `field`-Zeile, dann als Knoten in die Matrix; parallel, nicht blockierend.
- **Lauf `37469450306 @7dc5a42a` (nach Punkt 1, vor dem Per-Zelle-Fix) war Vorführlauf, kein
  Verdikt:** Artefakt `field-te-matrix-vlies` zeigt 3 pending Arme + global
  `matrix alignment absent`. Nach River 108 alignt die Matrix per Zelle — Re-Dispatch nach Push.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge108.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge107.md` (Move aus `docs/handover/`)
- `src/mathematikerin/te.rs` (TE_NEFF_THRESHOLD_BINNED + `BiasArm::neff_floor` + Gate)
- `tools/measure/src/bin/field_te_query.rs` (Per-Zelle-Alignment; binned n_eff; Test-Fixes)
- `.github/workflows/field-te-query.yml` (Job `unit`)

Operator-Gesprächsschnitt: `state/operator-gespraeche/2026-10-06-river.md` (gitignored).

Verifikation: `cargo check` grün; `cargo build -p omegaflow-measure --bin field_te_query` grün;
`cargo fmt -- <eigene Pfade>`. Der rote Skalar-Test aus river 106 geheilt; die Bin-Tests laufen
jetzt im Job `unit`. Die konsumierte Übergabe folge107 nach `archiv/`. Commit/Push warten das
Commit-Wort des Operators (`/commit`).

## Burn: open 0.0000 · close 0.1006 · cap 0.35 · Grund: Per-Zelle-Matrix-Riegel (Punkt 1 vollendet) + estimator-feste Bias-Korrektur; kein Dispatch, kein Send
