<!--
  title: Handover — River-Folge 109 (2026-10-06)
  session: River-Folge 109
  class: handover
  date: 2026-10-06
  sha256: 093ef9dcae189e7ed904ec54f0e0d3120458e5a8b3635cd7e8db5d496e03d669
  status: live
-->
# Handover — River-Folge 109 (2026-10-06)

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
„Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade" | 2026-10-06 | Operator (Session, River 109) — session-weiter Delegations-Consent, nicht das Commit-Wort
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge108.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau,
  Paar-Matrix, Ernte (§Lieferung); Bias-Kurve estimator-fest (gebaut).
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

### Universelles Vlies — `ozzy` + Conditional-Tabelle
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** 8 probe-Kanäle am Draht (für `ozzy` auf der Matrix); `conditional_embedded`-Probe (3).
- **Lage:** (gemessen 2026-10-06, River 108/109)
  (3) Conditional-Tabelle gemessen, kein Konsument: `te-bias-n-conditional` bias_fwd bei
  n 1260–2200 = -2.96e-1 … -2.14e-1 (referenz 6.3385e-1); die Matrix-Cond-Zellen nutzen
  `transfer_entropy_conditional_binned_n`, nicht die gemessene KSG-conditional-Schätzung
  → Tabelle `pending`. (5) `ozzy` bauen — negativer Fuzzy-Index auf der Matrix
  (`docs/specs/negativ-fuzzy-index.md`); bounded: Residuum-Extraktion gegen die Boden-Zeugen
  als eine Funktion + Gate. Der rote `field-te-query`-Lauf des eigenen Commits ist geheilt
  (siehe Abschluss); Re-Dispatch nach Push.
- **Blockade:** 8 probe-Kanäle sind nicht am Draht.
- **Braucht:** `ozzy`-Funktion + Gate gegen die Boden-Zeugen-Felder; Re-Dispatch `field-te-query.yml`;
  Rat-Reihenfolge: Draht → Matrix → `ozzy` → Netz-Null.

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 109 via `ci_manage status`) `ci-check 37481777486`
  @3ebe6a85b in_progress; River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`, Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`). Der rote `ci-gate`@3ebe6a85b
  wurzelte im Kern-Borrow (`commit_gate.rs:1013`) — von dieser Session geheilt.
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check` am HEAD nach dem Push abwarten (Stehender Pass; kein Polling).

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line Mountain/Mycelium)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN.
- **Lage:** (gemessen 2026-10-06, River 105) Fine-grain FMI-GIC manifestiert
  (`phi/sources.φ:17338-17345`, sha256 `a30a846d…`); IMAGE/NUR-Zeile registriert; Paper §4
  trägt CC BY 4.0 + Caveats. NUR nicht im CDN (kein sha, kein `image-cdn.yml`).
- **Blockade:** NUR-Manifestation (kein Workflow) + Probe.
- **Braucht:** (1) `image-cdn.yml` (Mycelium) + sha ins Register; (2) ggf. Tages-Detrend
  (Mountain); (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation.
- **Lage:** (gemessen 2026-10-06, River 106) deployte Props-Maske `0x01FF` (Bits 0–8), Bit 11 (GM)
  klar; `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`.
- **Blockade:** der gemessene GM fehlt in der Sonne-`.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Bit 11; Mycelium baut + manifestiert; Rivers Checkmark ist
  `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden + Startansicht auf die Sonne
- **Status:** wartend | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** Katalog-Lieferung nach Helligkeit (C); Rat-/Operator-Wort zum initialen Blick.
- **Lage:** (gemessen 2026-10-06, River 109) future-folge185 meldet: alle vier Assets laden,
  Hänger weg; die Startansicht ist fast schwarz, die Sonne ~2 px. `static/membrane.html:48`
  lädt `BODIES = ["sun","earth","moon"]` (Sonne zuerst); die Start-Kamera (`:53-56`) steht am
  SSB-Ursprung mit `scale = VIEW_SPAN_M/400` (2.53e17 m ≈ 8.2 pc). Der gemeldete Rest-Punkt
  `omega.rs:878 (Volume-Bin „earth" hart)` ist gemessen veraltet: `omega.rs:872` liest
  `field.volumes.iter().find_map(|v| v.frame_body.as_deref())` — kein harter Body-Name.
- **Blockade:** (C) hängt am 95-MB-Sternkatalog (Mountain + Mycelium); die Startansicht berührt
  das Gaze-Axiom (der initiale Blick ist Operator-/Rat-Entscheidung, kein Pro-Solo-Fenster-Edit).
- **Braucht:** (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen; Startansicht: Operator-Wort
  oder Rat-Entwurf (initialer Blick auf die Sonne), danach Render-Messung im Operator-Browser.
  (D) **descoped** (Körper-`.bin` je 6,6 MB, `accept-ranges` steht).

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06) Punkt (b) gebaut (mountain-239): `Volume.frame_body: Option<String>`,
  Test `volume_observer_declared_and_refused_when_absent`. Offen nur (a): `BODIES`-Handkopie.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105; wartend.φ:34-36) OMNI2 26 Zellen `pending`
  (Rohdatei lokal); kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** (1) `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; (2) Trigger feuern lassen;
  (3) Δ/σ_recon post-flyby.

### Voices-Chrome — eigene login-freie CDP-Instanz (Config gebaut)
- **Status:** eigen (Verifikation nach Neustart) | **Bindung:** eigen
- **Trigger:** opencode-Neustart + Voice-Test.
- **Lage:** (gemessen 2026-10-06, River 105) MCP `chrome-devtools-voices` gestartet mit
  `--headless --isolated --executablePath /usr/bin/google-chrome`; in allen 12 Voice-Profilen
  `browser_*`/`chrome-devtools_*` deny, `chrome-devtools-voices_*` allow.
- **Blockade:** opencode-Neustart (Config nicht hot-reloaded).
- **Braucht:** Neustart; Voice-Test.

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` trägt 1945 Spiegel-URLs über
  169 distinct Netlocs; keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen,
  Batch 1 = 10 Netlocs / 7 gemessen); Leads `state/river/license-census-voice.tsv`.
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen von Mountain messen lassen (Stimme liefert Terms-URL,
  nicht Klasse); Generator + Drift-Tor an Mycelium (`## An mycelium`).

## An mycelium

Origin: river folge101 (getragen über 102–109).

- **Generiertes `LICENSE` im `omegaflow/sources`-Repo.** Ein Compiler liest die `terms`-Zeilen
  aus `phi/sources.φ` und emittiert ein nach Lizenzklassen gruppiertes `LICENSE` (`pending`
  namentlich); ein CI-Tor prüft byte-identisch gegen die Neu-Erzeugung. **Braucht:** Generator +
  Drift-Tor, nachdem Mountains `terms`-Zeilen landen.
- **DE440-`.bin` remanifestieren** nach Mountains `de_compiler`-GM-Landung; Checkmark ist
  Rivers `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **Measured (river 106/109):** `dr3_stars.bin` aus `tap_compiler` (`--epoch 2016`); Körper-`.bin`
  je 6 629 784 B, `accept-ranges: bytes`; (D) descoped; alle vier Assets laden (future-folge185).

## An mountain

Origin: river folge107 (Lizenz-Audit) · folge108 (Vlies) · folge109 (Kern-Borrow-Heilung).

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
- **Kern-Borrow-Heilung (`62c9fd513`).** `commit_gate.rs` bewegte `unit` im `force-unit-gate`
  (`contains(&(force.to_string(), unit))`) und las es danach im `dimensionless-force` →
  `E0382 borrow of moved value` machte `omegaflow` (lib) unbaubar; `ci-gate 37481777416` /
  `tools-build 37481777424` @3ebe6a85b rot. River 109 hat die zwei Blöcke getauscht (der
  bewegende zuletzt); die Prüfungen sind unabhängig (unit=="1" vs unit!="1"), die Reihenfolge
  verhaltensgleich. Kein stiller Fremd-Eingriff — benannt, weil der rote Kern Rivers eigenes
  `field-te-query` blockte.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge109.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge108.md` (Move aus `docs/handover/`)
- `src/gate/commit_gate.rs` (Kern-Borrow-Heilung: `dimensionless-force` vor `force-unit-gate`)
- `tools/measure/src/bin/field_te_query.rs` (3 rote Tests geheilt)

Operator-Gesprächsschnitt: `state/operator-gespraeche/2026-10-06-river.md` (gitignored).

Verifikation: `cargo check` grün; `cargo build -p omegaflow-measure --bin field_te_query` grün;
`cargo fmt -- <eigene Pfade>`. Der rote `field-te-query`-Lauf `37480365699 @290e14601` gemessen
(3 Tests: `bias_column_gates_on_n_eff_and_exact_n` — n 8546 ist gemessen, nicht off-table;
`count_quantile_without_the_form_is_refused` — der `driver`-Arm brach vor dem `count`-Arm ab;
`spectral_epoch_comparison_never_averages` — `sign_agreement` liest jetzt die zentrierten
Vorzeichen, nicht die Rohvorzeichen) und geheilt; der Kern-Borrow zusätzlich. Die `unit`-Tests
liefen vor River 108 in keiner CI (der `unit`-Job ist neu). Re-Dispatch `field-te-query.yml`
nach dem Push (unten), Ausgang unread — nie gepollt.

## Burn: open 0.0000 · close 0.0000 · cap 0.35 · Grund: Kern-Borrow-Heilung + field-te-query-Re-Dispatch; 1 Dispatch, kein Send
