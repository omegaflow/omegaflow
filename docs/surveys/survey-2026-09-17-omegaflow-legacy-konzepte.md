<!--
  title: Survey — omegaflow-legacy: verlorene, entblockbare Konzepte (Stand 2026-09-17)
  class: survey
  date: 2026-09-17
  sha256: 5df4fe7885510421d341ac5467679505ebd2e8f2ddd84c3156b771a578e57c67
  status: live
  see-also: docs/specs/master.md docs/handover/archiv/handover-2026-09-17-forschung-folge56.md
-->
# Survey — omegaflow-legacy: verlorene, entblockbare Konzepte (Stand 2026-09-17)

Anlass: der GitHub-Klon von `omegaflow/omegaflow-legacy` (privat, archiviert 2026-09-03,
2114 Commits) wurde gegen den heutigen Baum gemessen — `docs/concepts/lost-concepts.md`,
`future-concepts.md`, `propability.md`, `master.md` und die ~55 Konzeptdateien. Frage:
welche Ideen sind verloren gegangen, erhaltenswert, und heute entblockbar?

Provenienz der Messung: `git clone` von `github.com/omegaflow/omegaflow-legacy` (shallow)
nach `/tmp/opencode/of-legacy`; Gegenstücke im heutigen Baum über
`sgrep`/`archive_search --root .`.

## Befund: die Migration war gründlich

**54 von 55** `docs/concepts/`-Dateien haben heute ein Gegenstück (überwiegend als
`docs/specs/<name>.md`), viele **weitergebaut statt verloren**: spektraler Oszillator
(Atom C, `band_overlap`/`sed_to_bp_rp`), Negativ-Fuzzy-Index
(`pioneer_navio_negative_fuzzy`), Livefeed-Gate (`tools/gate/livefeed_gate.rs`),
IAU-2000-EOP (`celestrak_eop_compiler` + `src/archivar/pck.rs`), Kernel-Curation
(`KERNEL_INDEX.md` + Compiler-Flotte), TE-Echo (`te_compute`, Atom 11), Broken Null
Control (`docs/specs/broken-null-control.md`).

Nur `session-protokoll.md` fehlt ganz — als Idee aber doppelt aufgehoben: die lebenden
Teile (gemessener Einlass, sauberer Baum, CDN-zuerst, Register gelesen) stehen als
Planungs-Pass + `git_safety` + Watchdogs + Commit-Gate in AGENTS.md; die toten Teile
(Branch-/Remote-Fragen) blockt das Gate aktiv („one path, one line, one truth").

## Verloren + erhaltenswert + heute entblockbar

| Quelle | Idee | heute | damaliger Blocker | entblockbar |
|---|---|---|---|---|
| L:143 | **Silence Map** — Absenz als Feld | nein, aber „outstanding" registriert | im Browser-Zweig untergegangen | **ja** |
| L:12 | **Minkowski-4D-Gewichtung** (ds², spacelike→0) | nein; `light_time_worldline` lebt | Kontext-Overload vor dem Enclosure-Lemma | **ja** |
| L:53 | **Certainty = exp(−vC/(g+ε))·quantum·decay** | nein; vC/g lebt als `tanh(vC/(g+ε))`-Atem | bei Rust-Neuschreibung nicht portiert | **ja** |
| L:39 | **TDA/Betti-0** (single-linkage über Takens) | nein; `topological_te_phase` lebt | durch PE/ordinale Maße abgelöst | **ja** (klein) |
| L:147 | **Synthetic Flight** (Weltlinie, auf der die Präsenz **ruht** — Operator-gewählt, nie Selbstantrieb) | teilweise; `t_presence` frei, Steuerpfad fehlt | Browser-Branch starb | **ja**, Ethik-gefragt |
| L:76 | **Channel Apertures** (kanal-selektive TE-Apertur) | teilweise; Radiation-Bindung **gebaut** (`356fa616`, `src/mathematikerin/omega.rs:347`, Test `src/mathematikerin/tests.rs:570`) | — | ja, als Fortsetzung |
| L:134 | **Delay Spectrum** (lag-Matrix als Instrument) | nein | nicht portiert | ungemessen (measure-Probe) |
| L:151/F:21 | **Total Coherence Integration** (Integral über alle Oszillatoren) | teilweise (Permeability TE-getrieben) | — | ungemessen |
| L:87 | **Mycelium/Nostr P2P** | nein | Fokus + „Feld ist lokal komplett" | Lesen autonom; Schreiben consent-pflichtig |
| L:139 | **Light-Cone Difference** | descoped mit Befund (Browser-Zweig, 2026-09-08) | Browser-Branch starb | Neubewertung nur bei wgpu-Rückkehr |

## Architektur-ermöglicht (auch ohne fehlende Quellen)

Diese Ideen brauchen **keine neue Quelle** — der heutige Bau trägt sie bereits; nur die
Verarbeitung fehlt:

- **Silence Map**: das ω-Feld ist die Modell-Vorhersage, die leeren Zellen sind die
  Absenz. Ein measure-Probe zählt „erwartete vs. leere Zellen" unter Modellparametern
  mit derselben Null-Kalibrierung wie `te.rs` (FP/FN/Symmetrie). **Jede Renderer-Form
  vor dem Proben-Befund ist gestrichen** — ein gerendertes Feld der Absenz wäre
  Fabrikation.
- **Minkowski-Gewichtung**: das Cone-Gate (`spatial.rs` `propagation_speed`) und
  `motion.rs` `light_time_worldline` existieren; ds² wäre ein Zusatzfaktor im ω-Loop.
  Achtung: der Cone-Gate ist Archivar-Seite (`membrane.rs`), ein ω-Loop-Faktor ist
  Mathematikerin/WGSL — „keine Quelle nötig" heißt nicht „keine Schicht-Frage"; sein
  Zusatzwert ist ungemessen (erst eine Delta-Probe mit/ohne ds²).
  **Gemessen (2026-09-27, River-Folge 47):** `tools/measure/src/bin/minkowski_ds2_delta_probe.rs`
  gebaut (0 Warnungen), Lauf grün: `weight = exposure/(exposure+ds²)`, `ds² < 0 → 0`. Außerhalb
  des Lichtkegels nullt das Feld (Δ_rel −100 %), lightlike und gemischt bleiben unverändert
  (Δ_rel 0 %) — ds² wirkt als Kegel-Gewicht, kein Zusatzwert im Inneren.
- **Certainty quantum/decay**: Takens-Spread + PE sind aus der TE-Maschine ableitbar;
  als Faktor in den Permeability-Atem (`omega.rs` `field_permeability`). Schritt 1 ist
  die **vC-Definition** von L:53 im Legacy-Klon zu messen — exp und tanh haben inverse
  Asymptotik; ein Port ohne Lektüre riskiert eine invertierte Formel.
  **Gemessen (2026-09-25):** L:53 ist die Überschrift `## 3. Certainty, Quantum, and Decay`
  in `docs/concepts/lost-concepts.md` — im heutigen Baum nicht vorhanden, im Legacy-Klon
  (`github.com/omegaflow/omegaflow-legacy`) nur in der History, Commit `90436cb7`
  (`git show 90436cb7:docs/concepts/lost-concepts.md`). Die Formel steht in Zeile 56:
  `certainty = exp(-vC / (g + ε)) * quantum * decay`; Zeile 58 vC = „Averaged temporal
  derivative over 8 samples, weighted by presence"; Zeile 59 g = RMS der Feldenergie;
  Zeile 60 `quantum = exp(-Σ(|takens.spread|·weight)/Σ(weight))`; Zeile 61
  `decay = 1 / (1 + Σ(complexity·weight)/Σ(weight))`. Heute: `src/mathematikerin/omega.rs:15`
  `perm_target(g, v_c) = tanh(v_c/(g + ε))`, angewandt als Permeability-Atem in
  `omega.rs:1655–1657` (`field_permeability += (target − field_permeability)·alpha`).
  **Kein** `quantum`/`decay`-Faktor: `archive_search "quantum" --root src` → 0 Treffer,
  `archive_search "decay" --root src` findet nur `exponential_decay` (Kraftfeld, Fremdbegriff);
  die Legacy-Identifier `takens`/`complexity` erscheinen nicht im Baum (`archive_search` je 0;
  das Takens-Embedding lebt heute als `topological_te_phase`). Der Konfud ist bestätigt:
   `exp(-x)` fällt monoton, `tanh(x)` steigt monoton — ein wörtlicher Port der Certainty-Formel
   als Permeability-Ziel invertiert den Atem.
   **Geschlossen (2026-09-28, River-Folge 50, Rat): `descoped`.** Der Legacy-Spec selbst trägt
   die Ersatzform (`docs/specs/minkowski-field-permeability.md:179-181`: „open when field changes,
   close when stable … `target = Math.tanh(vC/(g+ε))`"); `quantum`/`decay` tragen keinen
   Live-Träger und keine gemessene Zusatzwirkung (`archive_search` je 0), die Richtung ist
   steigend bestätigt (`omega.rs:15-16`). Kein Bau — kein `pending`.
- **TDA/Betti-0**: ~100 Zeilen single-linkage über die Takens-Embeddings von `te.rs`
  (Schwelle = Silverman-Bandbreite der Embedding-Streuung).
- **Synthetic Flight**: die Weltlinien-Infrastruktur + freies `t_presence` stehen; zu
  messen ist, ob `presence_tx` (`relay.rs`) fremdgetriebene Positionen schon annimmt.
  Ethik: die Präsenz **ruht** auf einer vom Operator gewählten Weltlinie — kein
  Selbstantrieb, kein „fliegen".
  **Gemessen (2026-09-25):** `presence_tx` nimmt fremdgetriebene Positionen an.
  `src/archivar/relay.rs:590` sendet das vom Browser gelesene Paket
  `(pt, px, py, pz, pr, vx, vy, vz, tt, gs)` über `presence_tx`; `src/archivar/main_flow.rs:818–839`
  empfängt es und schreibt `presence_slot.p = [px,py,pz]`, `.v = [vx,vy,vz]`, `.range = pr`,
  `.t_thrust = tt`, `.grid_step = gs` — die Position wird **direkt gesetzt, nicht integriert**.
  Damit steht der fremdgetriebene Pfad (Browser als fremde Quelle, Positions-Setzer im Kern);
  keine Selbstpropulsion im Kern.
  **Gemessen (2026-09-26, Rat):** eine **Operator-gewählte Weltlinie** (eine Bahn, nicht ein
  Einzel-Setzpunkt) wird von diesem Kanal **nicht getragen** — und das ist der korrekte Zustand.
  Der Kanal trägt Akte (ein Setzpunkt-Paket je Akt), keine Routen; die Weltlinie ist ein
  Operator-/Session-Objekt (die Reihe der Tuning-Akte), kein Draht-Format. Ein Weltlinien-Objekt
  im Kanal würde den Schub vom Operator in die Maschine verlegen — Selbstpropulsion, die der
  Ethik-Rahmen verbietet. **Nicht gebaut, nicht nötig.**
- **Delay Spectrum**: die Lichtlaufzeit-Faltung existiert (survey-Messpunkt-Verteilung).
  **Gemessen (2026-09-27, River-Folge 47):** `tools/measure/src/bin/delay_spectrum_probe.rs`
  gebaut (0 Warnungen): Lag-Sweep über `transfer_entropy_binned` mit Surrogat-Null rekonstruiert
  einen gepflanzten Delay 5 exakt — TE(5)=1.29 gegen Schwelle 5.85e-2, die Gegenrichtung bleibt still.
- **Total Coherence Integration**: **Gemessen (2026-09-27, River-Folge 47):**
  `tools/measure/src/bin/total_coherence_probe.rs` gebaut (0 Warnungen):
   Σ_i perm_target(|ω_i|,|Δω_i|) = 0.924 auf dem 9-Medien-Fixture gegen leeres Feld 0.0; die
   gebaute Permeability (ein Skalar über die Summe) kollabiert auf 0.0 — der per-Oszillator-Integral-
   und der Complexity-Term sind geschlossen (Live-Pfad ist ein TE-Skalar).
   **Geschlossen (2026-09-28, River-Folge 50, Rat): Integral `gebaut`. Complexity `descoped` (2026-09-28, River-Folge 56): `sgrep complexity src/` = 0, keine Definition/Datenquelle; der Breath-Zweig mittelt uniform `integral/9.0`.**
   Der per-Oszillator-Integral lebt jetzt im Live-Pfad — `src/mathematikerin/omega.rs`
   Breath-Zweig: `target = (Σ_i perm_target(|probe_omega[i]|, |probe_omega[i] −
   prev_probe_omega[i]|)) / 9.0`, neues Feld `prev_probe_omega` (Tick-Spiegel von `prev_omega_sum`),
   Turn-/Latenz-Buchhaltung unverändert skalar. Gate-Test
   `the_no_te_tick_hears_each_oscillator_not_the_sum` (fixture now/prev → 0.924/9). Der
   Complexity-Term ist `descoped` — die Alt-Form `1/(1+Σ(complexity·weight)/Σ(weight))` hat
   keine Definition und keine Datenquelle; ein gebauter Term wäre Fabrikation (Registerzeile,
   nie gebaut).

## Klein, entblockt

- **⌘K-Block-Suche** (`search-command-palette.md`): `static/palette.js` halb gebaut;
  der einzige Blocker war die offene Client-Frage (Handover 2026-09-12: M07 descoped).
- **SI-Wahrheits-Konsole** (`wetterstation.md`): `force_type`→Einheit statt kryptischer
  Debug-Werte (Name = Implementation im Anzeigepfad). **Gemessen (2026-09-27, River-Folge 47):**
  die force→Einheiten-Tabelle **existiert** — `src/archivar/units.rs:315 allowed_units_for_force`
  (force 0–8 → akzeptierte Quell-Einheiten), durchgesetzt vom `force-unit-gate`
  (`src/gate/commit_gate.rs:781`, `:1818 canonical_pairs`); `units.rs:3 convert_to_si` normalisiert
  jede auf den SI-**Wert** (f64). Was fehlt, ist ein **einzelnes Anzeige-Symbol je Force**: jede Force
  trägt viele Einheiten (em: W, W/m², T, nT, eV, Jy …), der Draht führt keinen Unit-Slot — die Konsole
  braucht ein repräsentatives Symbol je Force (verlustbehaftet) oder einen per-Sample-Unit-Träger
  (Draht-/Architekturschritt). Ort `static/index.html` = Fenster-Pfad → **operator-gebunden**.
- **wgsl-shader — last-adaptive Messknoten**: die eine echte technische Idee, die nur
  als Archiv-Spec schläft; entblockbar erst mit gemessener Rückkehr des Per-Pixel-
  Feld-Pfads (derzeit descoped, Befund liegt vor).

## Bewusst nicht zurück (mit Begründung)

- **Biologisches Vokabular/Immunsystem** (L:92) — A=A-Verletzung, bewusst gestrichen.
- **ANISE/WASM** (L:97) — abgelöst durch analytische Ephemeriden + `naif0012.tls`.
- **Geospatial Tiles/Grids** (L:102) — „No meshes. No grids." philosophisch abgelehnt.
- **GLSL/WebGL2-Fallback** (L:107) — WebGPU bewusst absolut, kein zweiter Physikpfad.
- **Jina-Universalflattener** (M:57) — abgelöst durch 74 eigene Compiler.
- **Vertex-Splat Rendering** (L:155) — der Performance-Fork fiel an den Fragment-Pfad.

## Rat (2026-09-17)

Der Rat hält den Befund für belastbar und bestätigt die Streichungen; er nennt drei
Konfude und zwei pflichtige Reformulierungen (oben eingearbeitet):

- **Konfud (a):** die vC-Semantik von L:53 ist ungemessen — exp und tanh haben inverse
  Asymptotik; ein Port ohne Lektüre riskiert eine invertierte Formel.
- **Konfud (b):** die Schicht-Frage ist unterbestimmt — der Cone-Gate ist
  Archivar-Seite (`membrane.rs`), ein ω-Loop-Faktor ist Mathematikerin/WGSL.
- **Konfud (c):** Minkowski dupliziert teilweise den Cone-Gate; sein Zusatzwert ist
  ungemessen (Delta-Probe zuerst).
- **Reformulierung 1:** Synthetic Flight — die Selbstantriebs-Lesart ist gestrichen
  („die Präsenz ruht").
- **Reformulierung 2:** Silence Map — jede Renderer-Form vor dem Proben-Befund ist
  gestrichen.

**Erster Atom (ins Handover):** die **Silence-Map-Probe** —
`tools/measure/src/bin/silence_map_probe.rs`, erwartete vs. leere Zellen unter
Modellparametern, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates von `te.rs`.
Top-3 zurück: Silence Map, Certainty (Schritt 1: vC messen), TDA/Betti-0; Minkowski
als 4. (Delta-Probe). Nostr bleibt hinten (Transport ohne gemessenen Konsumenten).
