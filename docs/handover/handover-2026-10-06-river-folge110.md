<!--
  title: Handover — River-Folge 110 (2026-10-06)
  session: River-Folge 110
  class: handover
  date: 2026-10-06
  sha256: 29f6b15f586945d4ddbd989d5d90be7ba63a53bf56960603357586e236ac8f67
  status: live
-->
# Handover — River-Folge 110 (2026-10-06)

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
„Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade" | 2026-10-06 | Operator (Session, River 109/110) — session-weiter Delegations-Consent, nicht das Commit-Wort
„es gibt keine kamera und zoom … die präsenz ist das trommelfell im 4d block auf das die punktwolke trifft" | 2026-10-06 | Operator (Session, River 109) — Empfänger-Modell, gegen die GPT-Kamera-Lesart
„das wort receiver [haben wir] extra eingeführt" | 2026-10-06 | Operator (Session, River 109) — kanonischer Term `Receiver` (Worldline), `Observer` (Vantage) verboten
„bitte alles umsetzen" | 2026-10-06 | Operator (Session, River 109) — Commit- und Bau-Wort für den Receiver-Schnitt
„es geht nicht nur um die visuelle membran es geht um alle radiatoren" | 2026-10-06 | Operator (Session, River 109) — die Receiver-Apertur gilt für alle fünf Radiatoren (Bild · Ton · Vibration · Serial · HID)
„warum gibst du die rats fragen nicht dem rat den api mit 5 stimmen und den ui chats mit 5 stimmen?" | 2026-10-06 | Operator (Session, River 110) — stehende Praxis: Rat-Fragen gehen an den **Rat (API, 5 Stimmen)** UND an die **UI-Modelle (5 Stimmen)**, nicht als `Braucht` liegen gelassen
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge109.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: der `matrix full`-Lauf
  (nicht `ozzy`-Bau, future-Korrektur 2026-10-06), Ernte (§Lieferung); Bias-Kurve estimator-fest (gebaut).
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
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (`class: survey`) — Träger dieser Linie
  (river-84); Label in diesem Atom als geschlossen formuliert, Header-sha nachgezogen.

## Offen (aufgeschlüsselt)

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren (nicht nur die visuelle Membran)
- **Status:** eigen | **Bindung:** eigen (Membran-/Aktor-Pfad)
- **Trigger:** frisches Atom (SPAN-Architektur entschieden 2026-10-06; der bounded Brücken-Schritt steht).
- **Lage:** (gemessen 2026-10-06, River 109/110) Der sichtbare Pfad ist halb geheilt: in
  `static/membrane.html` ist der 2.0-Shader-Boden entfernt (`true_px = extent/scale`; Punktquelle
  `extent 0` → 1-px-voller-Fluss), die unbelegte `VIEW_SPAN_M/400`-Konstante gestrichen, der Start
  schwarz; er **malt aber weiter Quadrate** (`point_size_px`) statt an der Receiver-Apertur zu messen.
  Das ist die visuelle Membran nur EINES von fünf Radiatoren — dasselbe Gesetz gilt für Ton (9
  Partialtöne je Kraft), Vibration, Serial/USB/BT/HID (Legacy „Ein Gesetz, fünf Medien"; Atom 8
  „Die Vereinheitlichung des Sensoriums — der Schall entscheidet, nicht das Trommelfell", `e918bda1`).
  Die Legacy-Membran maß **pro Messpunkt** die volle Superposition; Sub-Pixel-Raster 3 Spalten/Pixel
  (`eb96d1ff`); 1-px-Quad an exakter Sub-Pixel-Position (`da43f02a`); zurückgerollt nur wegen Tempo
  (567 ms, `docs/surveys/survey-fortschritt.md:22-23`), nicht wegen Unwahrheit. Ground Truth
  `docs/surveys/survey-messpunkt-verteilung.md:19-26`. Die frühere Angabe eines `entwicklungslinie.md`
  unter `docs/surveys/` war ein Pfad-Riss: dieses Register lebt **nicht** im Baum (Legacy-Repo
  `archive-root/omegaflow-legacy`);
  die im Baum auflösende Quelle ist `docs/surveys/survey-fortschritt.md`.
- **Blockade:** großer Umbau (per-Fragment-`source_contrib` ist O(Pixel×Quellen)) — eigenes Atom. Die
  SPAN/Apertur-Architektur ist **entschieden** (Rat, 2026-10-06): SPAN ist Receiver-Eigenschaft,
  **deklariert** (neue `span`-Direktive auf der `at <body>`-Receiver-Zeile in `phi/sources.φ`) **und** aus
  der Anker-Hülle verankert (`SPAN_eff = max(span_declared, hull(anchors))`), **Sterne nie** — kein
  Katalog-Span. Sichtbarkeit durch **Integration entlang der Worldline** (additiver `one,one`-Blend +
  EMA-`lvl` über das **Anker**-Niveau), nie durch injizierte Ausdehnung; kein `scale`-Boden. Ein
  Apertur-Gesetz, fünf Empfänger (`aperture = field_permeability · tone_scale`, `omega.rs:361`;
  `acoustic_amplitude`, `actuators.rs:73`). Risse (ungeglättet): `SPAN/N`-N ungemessen (`receptors` bleibt
  die Raster-Minorachse; Sub-pixel ist erwartet); Sternen-Reichweite 8 pc (Operator) vs. `STAR_SPAN_M`
  1,798e21 m ≈ 58 000 pc (committed, `src/archivar/spatial.rs`) — vor der Ableitung messen; Wortkollision
  `Aperture` (drei Bedeutungen) → der neue heißt `span_m`, nie `Aperture`.
- **Braucht:** (1) **bounded Brücke** in `static/membrane.html` (eine Datei/ein Feld/eine Funktion):
  `state.anchorCount` (gemessen, nach der `BODIES`-Schleife, kein Literal `3`) + `receiverSpan(record)`
  über die Anker-Records, ersetzt den Inline-Span-Block (`:455-468`); `updateLvl` (`:369-381`) nimmt das
  Anker-Niveau als `lvl_ref`; `render` (`:323-367`) unverändert (Sterne addieren, bewegen Span/`lvl`
  nicht). (2) **Register-Direktive** `span` auf der `at`-Zeile (Mountain; Parser-Arm + Test) — der
  Architektur-Akt, parallel. (3) danach Portierung an der Receiver-Apertur für alle fünf Radiatoren
  (Vorlagen `e918bda1`/`eb96d1ff`/`da43f02a`). (4) Operator-Browser-Messung (headless ohne WebGPU =
  schwarze Null).

**Architektur-Verdikt (Rat + 6 UI-Modelle, 2026-10-06):**
- **Rat (API):** SPAN ist Receiver-Eigenschaft — deklariert (`span` auf der `at <body>`-Zeile)
  **und** aus der Anker-Hülle verankert, Sterne nie; Sichtbarkeit durch Integration (additiver Blend +
  EMA), nie durch Ausdehnung; ein Apertur-Gesetz für fünf Empfänger. Erster Schritt: bounded Brücke in
  `static/membrane.html` (`receiverSpan` über die Anker-Records). Risse: `SPAN/N`-N ungemessen; 8 pc vs.
  `STAR_SPAN_M`; Wortkollision `Aperture`.
- **UI-Modelle (ChatGPT, Duck.ai/GPT-5.6, Mistral, Qwen, Sonnet 5.5, GLM-5.3 Deep-Think-Max):** einstimmig SPAN **deklariert** auf der
  `at`-Zeile, **nicht** aus Records abgeleitet; Sichtbarkeit durch **Integration/Fluss**, nie Geometrie;
  **per-Radiator-Wert**, nicht ein numerischer Maßstab (Qwen sagt „einheitlich" — Riss); erster Schritt
  **Register zuerst, nicht Membran** (ChatGPT/Mistral/Sonnet 5.5) gegen den Rat (Brücke zuerst) — Riss.
- **Sonnet 5.5 (schärfster Verdikt):** Energie-/Flusserhaltung — eine Quelle kleiner als der Rezeptor gibt
  ihren **ganzen Fluss** an diesen ab (kein 2-px-Boden, kein Point-Size-Floor); SPAN **fix pro Zeile**
  (änderbar = Zoom durch die Hintertür; ein anderer Maßstab = eine andere Receiver-Zeile, kein Regler);
  ein SPAN trägt Sonne und Mond-Bahn **nicht** zugleich (~400×), Sterne bleiben jenseits der Apertur und
  werden **gezählt, nicht gezeichnet**; Float-Akkumulation (RGBA16F bzw. RGBA32F), `lvl` aus dem
  integrierten Puffer, nie aus der Record-Liste; Tonemap dahinter = Anzeigeseite; Abnahme = **Invarianztest**
  (`scale` identisch mit und ohne 8-pc-Katalog).
- **GLM-5.3 (Deep Think Max, schärfste Verdichtung):** Verdikt bestätigt, Ränder geschärft. SPAN + N
  = Receiver-Anatomie, beide konstant, **deklariert** (Ableitung = „Max-Distanz mit Whitelist", derselbe
  Fehler kuratiert); die Anker dienen der **Energiebuchhaltung** (Sonne = Referenz-Radiator 0 dB), nicht
  dem Maßstab. Sonne zuerst über drei Empfänger-Werkzeuge, null Ausdehnung: (a) sub-Rezeptor-Radiator
  deponiert den **vollen Fluss** im enthaltenden Rezeptor (Size = 1 Rezeptor, als Quantisierungsboden
  deklariert); (b) `one,one` in einen **HDR-Akkumulator** (16F/32F — ein 8-bit-Canvas klippt die ≥10¹⁰
  Dynamik), EMA-Decay `exp(−dt/τ)`, Sonne sättigt in Sekunden, 8-pc-Sterne in Minuten; (c) **log/µ-law
  Tone-Map**, absolut auf den Anker geeicht (Weber–Fechner) — **verboten** ist eine aufs Maximum normierte
  Belichtung (der Katalog darf die Sonne nicht dimmen). **Erster Schritt: Maßstab einfrieren — löschen,
  nicht flaggen** (ein Flag ist die Rückkehrtür); Akzeptanz = Ankunftspermutationen ergeben identische
  Rezeptor-Geometrie, der 8-pc-Load bewegt kein Pixel der Sonne. Riss: nach dem Freeze allein ist die
  Startansicht fast schwarz — der wahre Zustand vor Integration; das ist eine Anforderung an die
  Belichtungskurve (Schritt 2), nicht an den Maßstab — **nicht rückwärts verhandeln**.
- **Braucht (revidiert, Register zuerst):** (1) `span` (Einheit Pflicht, Startwert ~2–4 AU) auf der
  `at <body>`-Receiver-Zeile; `scale = span ÷ receptors` aus dem Register; Fallback = feste Konstante mit
  lautem Log, **nie** aus Records; Records außerhalb werden gezählt, nicht gezeichnet; Lint „Sonne
  außerhalb". (2) Invarianztest. (3) dann Energieerhaltung + Float-Akkumulation. (4) getrennte
  Belichtungs-/Tonemap-Regel (nie Geometrie). (5) danach Portierung für alle fünf Radiatoren, **je
  eigener Wert**. Die bounded Brücke (Rat-Q4) bleibt als Zwischenschritt möglich, aber **nach** der
  Register-Direktive — der Rat hält die Register+Brücke-Seite, die 6 UI-Modelle (zuletzt GLM: „einfrieren,
  löschen, nicht flaggen") die reine Register-Seite; der Riss „Register zuerst vs. Brücke zuerst" steht,
  nicht geglättet — die Gewichtung kippt zu **Register zuerst, Max-Distanz-Pfad löschen**.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erneuter `field-te-query.yml`-Lauf nach 3-Arm-Serien-Verdrahtung + Alignment; danach `ozzy`.
- **Lage:** (gemessen 2026-10-06, River 110 via `ci_manage`/Lauf-Artefakt) future-folge186 korrigiert
  (Session-Direktnachricht, gepusht `0cd082b`): `field_te_query` **ist** die Alles-gegen-alles-Abfrage
  (Rats-Verdikt 2026-10-05 „fahren und verdrahten, nicht neu entwerfen", `docs/auftrag/auftrag-universelles-vlies.md:116-123`);
  offen ist der **Lauf**, kein `ozzy`-Bau. Der Descriptor `phi/pipeline/descriptors/vlies_matrix.te`
  trägt 15 Felder, `matrix vlies full`, `fdr bh 0.05 over matrix`, `expect cells 210`, lags 0,1
  (Wireing gebaut, River 107). Lauf `field-te-query 37483359644 @c07c271fa` = success: 210/210 Zellen
  deklariert, **12 von 15 Armen gemessen**; 3 Arme bleiben `pending — format text arm carries no series`
  (`omni_imf_bz_gsm_nt`, `eve_1032_line_irradiance`, `eve_131_line_irradiance`); viele Zellen
  `alignment pending` (Solar-/Magnetosphären-Arme), QBO/daily `resolution pending`; Ausgang
  **`fdr bh q 0.05 over matrix: 0 of 210 cells pass`** — die verdiente Stille, aber auf unvollständigem
  Draht. Der Descriptor ist seither von mountain-246 (`11256a711`, 18:17) fortgeschrieben (Kp nicht
  Peer-Feld). (3) Conditional-Tabelle gemessen, kein Konsument: `te-bias-n-conditional` bias_fwd bei
  n 1260–2200 = -2.96e-1 … -2.14e-1 (referenz 6.3385e-1); die Matrix-Cond-Zellen nutzen
  `transfer_entropy_conditional_binned_n`, nicht die gemessene KSG-conditional-Schätzung → `pending`.
- **Blockade:** die 3 serienlosen Arme + das Alignment sind Daten-/Compiler-Wiring (Mountain/Mycelium),
  nicht River; `ozzy` läuft erst **auf** der Matrix.
- **Braucht:** (1) `omni_imf_bz_gsm_nt` + `eve_1032`/`eve_131` als Serie verdrahten (Mountain:
  `format`-Arm/Compiler — s. `## An mountain`); (2) `matrix full` am neuen HEAD erneut fahren
  (`field-te-query.yml`-Dispatch, nie lokal); (3) dann `ozzy`; (4) Netz-Null als CI-Batterie (B ≥ 1/α).
  Rat-Reihenfolge: Draht → Matrix → `ozzy` → Netz-Null.

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06 via `ci_manage status`/`view`) River-/Mountain-Seite gebaut
  (`src/mathematikerin/shaders.rs:186`/`:211`, Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`). `ci-check 37481777486 @3ebe6a85b` und
  `37489805803 @3eac2e119` beide **pending**; am HEAD `6559ff8b9` kein `ci-check`-Lauf registriert.
  Trigger daher nicht gefeuert.
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check` am HEAD abwarten (Stehender Pass; kein Polling).

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
- **Lage:** (gemessen 2026-10-06, River 109) Der Receiver-Schnitt ist gebaut (`static/membrane.html`):
  Shader-Boden entfernt, `VIEW_SPAN_M/400` gestrichen — die Skala folgt dem empfangenen Span
  (`span_m / receptors`, `frame()`); vor dem ersten Eintreffen schwarz; additive Blending (`one,one`).
  Verbleibend (C): `BODIES = ["sun","earth","moon"]` (`:49`) ist eine geschlossene Menge →
  Build-Time-Manifest. Der gemeldete `omega.rs:878`-Rest ist gemessen veraltet.
- **Blockade:** (C) hängt am 95-MB-Sternkatalog + dem Trio-Manifest (Mycelium/CI).
- **Braucht:** (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen; Trio → Manifest. (D) descoped
  (Körper-`.bin` je 6 629 784 B, `accept-ranges` steht).

### Receiver-Naming-Residuum — `observer` → `receiver`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** frisches Atom.
- **Lage:** (gemessen 2026-10-06, River 109) Kanonisch ist `Receiver` (Worldline);
  `Observer` (Vantage) verboten (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md:82`).
  Rest-Treffer: `src/mathematikerin/doppler.rs:96,123` `observer_icrs`, `tests.rs` „observer epoch"
  (die Gate-Fixtures in `src/gate/friction.rs:70-71` sind intendiert und bleiben).
- **Blockade:** keine.
- **Braucht:** Umbenennung der nicht-Fixture-Treffer auf `receiver`; `cargo check` sauber.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:49`).
- **Lage:** (gemessen 2026-10-06) Punkt (b) gebaut (mountain-239): `Volume.frame_body: Option<String>`,
  Test `volume_observer_declared_and_refused_when_absent`. Offen nur (a): `BODIES`-Handkopie.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending` (Rohdatei lokal); kp `def`
  leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
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

Origin: river folge101 (getragen über 102–110).

- **`pages-deploy` rot — DE440-Remanifest vs. Pin (Riss, gemessen 2026-10-06, River 110).**
  `pages-deploy 37489805784 @3eac2e119` = failure: `sha256 mismatch for ephemeris_de440_earth.bin:
  got 5554915d… want adc990bc…` (`ci_manage log`). Live gemessen (`archive_search --sniff`):
  Release `ssd.jpl.nasa.gov-de` trägt jetzt `earth 5554915d…`, `sun 093b3ab5…`, `moon d9b40917…`
  (alle 6 629 784 B, via `gh release view` **hochgeladen 2026-10-06T13:25Z**). `pages-deploy.yml:60-62`
  pinnt noch die alten shas (`adc990bc`/`acb42881`/`9d059db3`, river-90/2026-10-04). Alle drei
  Anker-Assets wurden also zusammen regeneriert; das Register `phi/sources.φ` trägt keine `sha256`-Zeile
  für die DE440-Linie. **Riss, nicht stiller Pin-Tausch:** die gestagten Bytes sind nicht die gepinnten —
  die Workflow-Botschaft selbst verlangt die gemessenen Bytes. **Braucht:** entscheiden, ob der neue
  `de_compiler`-Output autoritativ ist (dann Pins auf die gemessenen shas setzen **und** eine
  `sha256`-Direktive je DE440-Zeile ins Register) oder ob remanifestiert werden muss; danach
  `pages-deploy` neu auslösen. Hängt am DE440-Punkt bei Mountain.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo.** Ein Compiler liest die `terms`-Zeilen
  aus `phi/sources.φ` und emittiert ein nach Lizenzklassen gruppiertes `LICENSE` (`pending`
  namentlich); ein CI-Tor prüft byte-identisch gegen die Neu-Erzeugung. **Braucht:** Generator +
  Drift-Tor, nachdem Mountains `terms`-Zeilen landen.
- **DE440-`.bin` remanifestieren** — s. der Riss oben; Checkmark ist Rivers `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:49` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **Measured (river 106/109/110):** `dr3_stars.bin` aus `tap_compiler` (`--epoch 2016`); Körper-`.bin`
  je 6 629 784 B, `accept-ranges: bytes`; (D) descoped; alle vier Assets laden.

## An mountain

Origin: river folge107 (Lizenz-Audit) · folge108 (Vlies) · folge110 (DE440-Riss).

- **`terms`-Direktive je Körperdatenzeile + Manifestations-Gate.** Jedes auf dem CDN gespiegelte
  Körpermesswert-Asset trägt eine redistributions-erlaubende Lizenz, aber keine Register-Zeile
  nennt sie: `openneuro.org` (ds005034/ds007471/ds007822) = **CC0**; `physionet.org`
  (`bidsleep_mehrnacht.bin`) = **ODC-BY 1.0**; `ieeg.org` bleibt entfernt. **Braucht:**
  (1) `terms <license> <url>` je Körperdatenzeile; (2) Parser-Arm für `terms`
  (`src/archivar/parse.rs`, heute still ignoriert); (3) `unbacked_mirror`
  (`src/gate/commit_gate.rs:2039`) verschärfen.
- **DE440-Register:** die drei `ssd.jpl.nasa.gov-de`-Anker-Assets (`phi/sources.φ:3810-3819`) tragen
  keine `sha256`-Zeile; der `de_compiler`-Output wurde 2026-10-06T13:25Z neu hochgeladen
  (`earth 5554915d…`/`sun 093b3ab5…`/`moon d9b40917…`, `archive_search --sniff`). **Braucht:** als
  gemessene `sha256`-Direktive je Zeile, damit `pages-deploy` nicht auf einen Hand-Pin angewiesen ist.
- **Vlies-Matrix — zwei fehlende Register-Felder.** Für `vlies_matrix.te` fehlen als `field`-Zeile:
  **Newell dΦ/dt** (`bz_retro_probe.rs:431` rechnet es) · **Kp** `magnetosphere_kp_3h` (heute
  `last`). EEG ds007822/ds007471: Feldname ungemessen (CC0, s.o.). **Braucht:** je eine
  `field`-Zeile, dann als Knoten in die Matrix; parallel, nicht blockierend.
- **Vlies-`matrix full` — 3 serienlose Arme (gemessen 2026-10-06, River 110).**
  `field-te-query 37483359644` misst 12 von 15 Armen; `omni_imf_bz_gsm_nt`,
  `eve_1032_line_irradiance`, `eve_131_line_irradiance` bleiben `stays pending — format text arm
  carries no series`; viele Solar-/Magnetosphären-Zellen `alignment pending`. **Braucht:** je Arm eine
  Zeitreihe am `.bin` (Format-/Compiler-Arm), dann erneuter `matrix full`-Lauf; bis dahin ist
  `0 of 210 cells pass` auf unvollständigem Draht.
- **Live-Bias `src/archivar/port.rs:348-354` (stärkste offene Rückfallstelle).** Der Rahmen-
  Normalisierer **rät**: `at sun` (349), `on earth 0 0 0` (351), `on earth {lat} {lon} {alt}` (354) —
  derselbe `on earth`/`at sun`-Default, den `frames.rs` unter `dcc3243f8` verlor (aus `35ff0dfe8`,
  2026-09-17). Verstößt gegen Q3 des Agnosis-Verdikts („Inferenz verboten, Deklaration erlaubt").
  **Braucht:** den fehlenden Rahmen als `pending`/`refused` materialisieren, nie raten.
- **Membran-Sonne-Anker:** `static/membrane.html:37` trägt `CATALOG_EPOCH_YR = 2000.0`, das Register
  `catalog_epoch 2016.0` (`phi/sources.φ`). Der Kommentar benennt es als Riss (Mountains Verdikt) —
  **Braucht:** Mountains Verdikt, welcher Epoche der Sternkatalog gilt.

## Receiver — Bias-Archäologie (Runden, für den offenen Punkt)

Der Receiver ist gebaut: `src/weberin.rs:1729` `enum ReceiverWorldline { Body, ReferencePoint }`;
`observer`→`receiver` umbenannt (`sensory-folge235:374`). Kanonisch ist **`Receiver`** (Worldline);
`Observer` (Vantage) ist verboten (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md:82`).
**Mechanismus der Wiederkehr:** Der Bias kam wieder, weil jede Entfernung den Bequemlichkeits-Default
an der **nächsten Datengrenze** stehen ließ — statt `pending`/`refused` füllte der kürzeste Code-Pfad
die Lücke mit der plausibelsten Vokabel (Erde für terrestrisch, Sonne für zölibat). Der Kampf ist pro
Grenze, nicht einmalig. Rest offen: `src/archivar/port.rs:348-354` (s. `## An mountain`);
`static/membrane.html:49` BODIES (Mycelium/CI); `src/weberin.rs:442,579,1819`
`body_barycenter_position("sun")` (benennen, nicht wählen — noch nicht als eigener Punkt gefasst).

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge110.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge109.md` (Move aus `docs/handover/`)
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (Label „Offen: keiner" → „Kein offener Punkt
  aus diesem Gate" geschlossen; Header-sha256 nachgezogen — sensory-folge244 adressiert, erledigt)

Gefaltet (adressierte Blöcke, in diesem Atom): future-folge186 (Startansicht Sonne → Receiver-Apertur),
mycelium-folge243 (iEEG aufgelöst; hyperscanning nicht mehr trägerlos per `register_lookup --orphan-docs`),
sensory-folge244 (exzellenz-gate-Label, oben erledigt).
`open_points_check` am folge109: 1 ABSENT (das Legacy-Register `entwicklungslinie.md`, nicht im Baum;
in folge110 auf `docs/surveys/survey-fortschritt.md` korrigiert).

## Burn: open 0.0000 · close 0.2073 · cap 0.35 · Grund: River 110 — Line-Session (deepseek-flash), Register-/Falt-Pass (3 adressierte Blöcke) + exzellenz-gate-Label geschlossen + `matrix full`-Lauf `37483359644` gelesen (12/15 Arme, 0/210 Zellen) + DE440-`pages-deploy`-Riss geroutet + **Architektur-Verdikt Receiver-Apertur-SPAN** (Rat API + 6 UI-Modelle, u.a. GLM-5.3 Deep Think Max: einstimmig SPAN deklariert, Integration/Fluss statt Ausdehnung; Riss Register-zuerst vs. Brücke-zuerst) (gemessen `session_burn` bei Übergabe-Schluss, opencode.db; Session `River-Linie Übergabe in einem Pass abarbeiten`, $0.2073)
