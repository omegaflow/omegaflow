<!--
  title: Survey — omegaflow-legacy: verlorene, entblockbare Konzepte (Stand 2026-09-17)
  class: survey
  date: 2026-09-17
  sha256: 2e68780c8bf2f28d82f9f511d3aee11498a56baae310a4b161bcde4825c3b9a9
  status: live
  see-also: docs/concepts/master.md docs/handover/handover-2026-09-17-forschung-folge56.md
-->
# Survey — omegaflow-legacy: verlorene, entblockbare Konzepte (Stand 2026-09-17)

Anlass: der GitHub-Klon von `omegaflow/omegaflow-legacy` (privat, archiviert 2026-09-03,
2114 Commits) wurde gegen den heutigen Baum gemessen — `docs/concepts/lost-concepts.md`,
`future-concepts.md`, `propability.md`, `master.md` und die ~55 Konzeptdateien. Frage:
welche Ideen sind verloren gegangen, erhaltenswert, und heute entblockbar?

Provenienz der Messung: `git clone` von `github.com/omegaflow/omegaflow-legacy` (shallow)
nach `/tmp/opencode/of-legacy`; Gegenstücke im heutigen Baum über
`sgrep`/`archive_search --root /home/johannes/projects/omegaflow`.

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
| L:76 | **Channel Apertures** (kanal-selektive TE-Apertur) | teilweise; Radiation-Bindung `pending` (Atom 9) | — | ja, als Fortsetzung |
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
- **Certainty quantum/decay**: Takens-Spread + PE sind aus der TE-Maschine ableitbar;
  als Faktor in den Permeability-Atem (`omega.rs` `field_permeability`). Schritt 1 ist
  die **vC-Definition** von L:53 im Legacy-Klon zu messen — exp und tanh haben inverse
  Asymptotik; ein Port ohne Lektüre riskiert eine invertierte Formel.
- **TDA/Betti-0**: ~100 Zeilen single-linkage über die Takens-Embeddings von `te.rs`
  (Schwelle = Silverman-Bandbreite der Embedding-Streuung).
- **Synthetic Flight**: die Weltlinien-Infrastruktur + freies `t_presence` stehen; zu
  messen ist, ob `presence_tx` (`relay.rs`) fremdgetriebene Positionen schon annimmt.
  Ethik: die Präsenz **ruht** auf einer vom Operator gewählten Weltlinie — kein
  Selbstantrieb, kein „fliegen".
- **Delay Spectrum**: die Lichtlaufzeit-Faltung existiert (survey-Messpunkt-Verteilung);
  die lag-Matrix als Instrument wäre eine neue measure-Probe.

## Klein, entblockt

- **⌘K-Block-Suche** (`search-command-palette.md`): `static/palette.js` halb gebaut;
  der einzige Blocker war die offene Client-Frage (Handover 2026-09-12: M07 descoped).
- **SI-Wahrheits-Konsole** (`wetterstation.md`): `force_type`→Einheit statt kryptischer
  Debug-Werte (Name = Implementation im Anzeigepfad); klein, unblockiert.
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
