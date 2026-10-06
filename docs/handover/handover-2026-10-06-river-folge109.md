<!--
  title: Handover — River-Folge 109 (2026-10-06)
  session: River-Folge 109
  class: handover
  date: 2026-10-06
  sha256: e866203fb087988390ea999158b2d7dac84862d318cad2b55fb9be98efa2392b
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
„es gibt keine kamera und zoom … die präsenz ist das trommelfell im 4d block auf das die punktwolke trifft" | 2026-10-06 | Operator (Session, River 109) — Empfänger-Modell, gegen die GPT-Kamera-Lesart
„das wort receiver [haben wir] extra eingeführt" | 2026-10-06 | Operator (Session, River 109) — kanonischer Term `Receiver` (Worldline), `Observer` (Vantage) verboten
„bitte alles umsetzen" | 2026-10-06 | Operator (Session, River 109) — Commit- und Bau-Wort für den Receiver-Schnitt
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

### Membran — Receiver-Schnitt gebaut; progressives Laden nach Helligkeit (C)
- **Status:** eigen (C wartend) | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** Katalog-Lieferung nach Helligkeit (C); Trio-Manifest (Mycelium/CI).
- **Lage:** (gemessen 2026-10-06, River 109) Der **Receiver-Schnitt ist gebaut** (`static/membrane.html`):
  der Shader-Boden `2.0` ist entfernt (`true_px = extent/scale`; Punktquelle `extent 0` → 1-px-voller-Fluss,
  ausgedehnter Körper → `coverage = min(1, true_px²)`); die unbelegte `VIEW_SPAN_M`/`400`-Konstante ist
  gestrichen — die Skala folgt dem **empfangenen** Span (`span_m / receptors`, `frame()`); vor dem ersten
  Eintreffen ist der Span 0 → schwarz. Die additive Blending (`one,one`) trug den integrierenden Empfang
  bereits. Verbleibend (C): `BODIES = ["sun","earth","moon"]` (`:48`) ist eine geschlossene Menge →
  Build-Time-Manifest (Mycelium/CI). Der gemeldete `omega.rs:878`-Rest ist gemessen veraltet (s. 108).
- **Blockade:** (C) hängt am 95-MB-Sternkatalog + dem Trio-Manifest (Mycelium/CI).
- **Braucht:** (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen; Trio → Manifest. **Riss notiert:**
  die Zoom-Tasten/Wheel (`:390-407`) sind eine Kamera-Reminiszenz gegen die Trommelfell-Doktrin —
  Entscheidung offen. Render-Messung im Operator-Browser (headless ohne WebGPU-Adapter = ehrliche schwarze Null).
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
- **Live-Bias `src/archivar/port.rs:348-354` (stärkste offene Rückfallstelle).** Der Rahmen-
  Normalisierer **rät**: `at sun` (349), `on earth 0 0 0` (351), `on earth {lat} {lon} {alt}` (354) —
  derselbe `on earth`/`at sun`-Default, den `frames.rs` unter `dcc3243f8` verlor (aus `35ff0dfe8`,
  2026-09-17). Verstößt gegen Q3 des Agnosis-Verdikts („Inferenz verboten, Deklaration erlaubt").
  **Braucht:** den fehlenden Rahmen als `pending`/`refused` materialisieren, nie raten.

## Receiver — Bias-Archäologie (River 109)

Der Receiver ist gebaut: `src/weberin.rs:1729` `enum ReceiverWorldline { Body, ReferencePoint }`;
`observer`→`receiver` umbenannt (`sensory-folge235:374`). Kanonisch ist **`Receiver`** (Worldline);
`Observer` (Vantage) ist verboten (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md:82`).
`AGENTS.md:183-188` ist in diesem Atom auf `Receiver` aligniert.

**Runden (Versuch → Rückfall):** reines Feld (Legacy) → agnostische Oszillatoren (07-22) → Observer
gebaut, dann gelöscht (08-11) → Trommelfell-Doktrin (08-12) → De-Zentrierung (08-19) → `port.rs`-Inferenz
(`35ff0dfe8`, 09-17) → `observer_icrs` (`9936018bb`, 09-26) → Bootstrap-Revert (`e1f1baa65`, 09-27) →
serverloses `BODIES`-Trio (`45e1c2b7f`/`638a11bb9`, 10-04) → Aberration/Doppler als Receiver-Relationen
(`630bcd3f3`, 10-05) → Survey (`742a6dbbb`, 10-06) → Anchor-Bypass + `frames.rs`-Defaults + `omega.rs`-Literal
+ `weberin.rs`-Receiver (`dcc3243f8`/`17d626ef7`, 10-06).

**Mechanismus der Wiederkehr:** Der Bias kam wieder, weil jede Entfernung den Bequemlichkeits-Default an
der **nächsten Datengrenze** stehen ließ, an der ein Record auch ohne deklarierte Worldline vollständig
sein musste — und statt `pending`/`refused` füllte der kürzeste Code-Pfad die Lücke mit der plausibelsten
Vokabel (Erde für terrestrisch, Sonne für zölibat, das sichtbare Trio für den serverlosen Pfad): ein
0-Kanon-Verstoß, den der Gradient mit „earth"/„sun"/„observer" als Default-Wort stützt. **Konsequenz:**
Der Kampf ist pro Grenze, nicht einmalig; die Grenze muss `pending`/`refused` zulassen.

**Rest offen:** `src/archivar/port.rs:348-354` (s. `## An mountain`); `static/membrane.html:48` BODIES
(Mycelium/CI); `src/weberin.rs:442,579,1819` `body_barycenter_position("sun")` (benennen, nicht wählen).
**Naming-Residuum:** `doppler.rs:96,123 observer_icrs`, `tests.rs` „observer epoch" (Gate-Fixtures in
`friction.rs:70-71` sind intendiert). **subject/object:** in `src/` kein Treffer → `pending`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge109.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge108.md` (Move aus `docs/handover/`)
- `src/gate/commit_gate.rs` (Kern-Borrow-Heilung: `dimensionless-force` vor `force-unit-gate`)
- `tools/measure/src/bin/field_te_query.rs` (3 rote Tests geheilt)
- `static/membrane.html` (Receiver-Schnitt: Shader-Boden raus, `VIEW_SPAN_M`/`400` gestrichen,
  Skala aus dem empfangenen Span, schwarzer Start)
- `AGENTS.md` (`observer` → `Receiver`-Worldline, `## Block Universe Physics`)

Operator-Gesprächsschnitt: `state/operator-gespraeche/2026-10-06-river.md` (gitignored).

Verifikation: `cargo check` grün; `cargo build -p omegaflow-measure --bin field_te_query` grün;
`cargo fmt -- <eigene Pfade>`. Der rote `field-te-query`-Lauf `37480365699 @290e14601` gemessen
(3 Tests: `bias_column_gates_on_n_eff_and_exact_n` — n 8546 ist gemessen, nicht off-table;
`count_quantile_without_the_form_is_refused` — der `driver`-Arm brach vor dem `count`-Arm ab;
`spectral_epoch_comparison_never_averages` — `sign_agreement` liest jetzt die zentrierten
Vorzeichen, nicht die Rohvorzeichen) und geheilt; der Kern-Borrow zusätzlich. Die `unit`-Tests
liefen vor River 108 in keiner CI (der `unit`-Job ist neu). Re-Dispatch `field-te-query.yml`
nach dem Push: `field-te-query 37483359644 @c07c271fa` (queued), Ausgang unread — nie gepollt.
Der Push löste zusätzlich `ci-gate 37483354207`, `ci-check 37483354197`, `tools-build 37483354248`
und `register-coverage 37483354340` am eigenen HEAD aus (Ausgang unread).

**Receiver-Schnitt (River 109, 2. Atom):** `static/membrane.html` — der 2-px-Shader-Boden entfernt
(`true_px = extent/scale`; Punktquelle → 1-px-voller-Fluss, ausgedehnter Körper → `coverage = min(1,true_px²)`);
`VIEW_SPAN_M/400` durch den empfangenen Span (`span_m/receptors`) ersetzt; vor dem ersten Eintreffen 0 →
schwarz. Kein `cargo`-Gate für JS/WGSL — die Render-Messung ist operator-browser-gebunden (headless ohne
WebGPU-Adapter = ehrliche schwarze Null); die vier Zuschnitte wurden am Text geprüft. Externe Chat-Verdikte
(einstimmig Fabrikation): ChatGPT, HF-Kimi-K3, HF-GLM-5.3, HF-Qwen3.8-27B, HF-Llama-3.3-70B, DeepSeek-R1,
Qwen, Mistral, Duck.ai — gegen den in-house Schwarm (verteidigte den Boden). Der `Receiver`-Term ist
`survey-2026-10-06-agnostik-llm-verdikt.md:82`.

## Burn: open 0.0000 · close 0.3024 · cap 0.35 · Grund: Kern-Borrow-Heilung + field-te-query-Re-Dispatch + Receiver-Schnitt + Rat/Schwarm/Chat-Verdikt; 1 Dispatch, kein Send
