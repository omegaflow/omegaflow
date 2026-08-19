# Handover — Audit nach den 8 Atomen (2026-08-19)

## Was das ist

Nach S0–S4, P1, P3, P5 (Atome 1–8) wurde der Bestand gegen den 0-Kanon
vermessen: Grep-Sweeps (Fallback-Muster, 0-Sentinel, Identitätswörter,
stille `.ok()`/`let _`, Redundanzen), gezielte Reads der Fundstellen,
Gegenprüfung jeder Atom-Behauptung am Code. Ergebnis: die 8 Atome tragen
(G), der Rest liegt hier als Meldung an die Schwester — jede Meldung
benennt Befund, Fundort, Urteils-Vorschlag, Aufgabe. Vollzug = Atome S5
und S6 (TODO-Karte). Diese Datei ist Pflichtlektüre der nächsten Session.

## G — Gegenprüfung: bestätigt (kein erneutes Audit)

- S3: `ft_ref_floor` trägt keinen 1e-30-Floor mehr (main.rs:101-110:
  max(…, 0.0), `lum_ratio` → 0.0 bei r≤0 = Schweigen); `write_ws_binary`
  → `io::Result` (21002) und geprüft (20864); 31-Punkte-Locus (141);
  `exp2(-20)` in main.rs und index.html absent.
- S1: tgas-rv-Skip lebt (tycho2_compiler.rs:497-501: ohne rv → continue);
  WGSL-Zweige kernel 1/6 in `field_spatial`/`field_spatial_grad` (221/249).
- S0: count-Gate + response_epoch im Browser (index.html:686-690).
- P3: ein lib-Parser (json.rs), fit.rs, `days_from_civil` pub — wahr.

## S5 — Audit-Nachlese I

### Welle 1 — H1: Verdachts-Vokabel „ehrlich"/„honest"

Befund: „ehrlich" benennt nichts — es bewertet die Wahrhaftigkeit des
Schreibers. A ≠ A: ein Urteil, das erwartete Tugend trägt statt die
Sache. Ein 0-honored-Derivat aus der Trainingsdaten-Nachbarschaft, das
sich als Kanon-Wort ausgibt.

Fundorte: TODO.md (46, 452, die S3-Zeile, die P5-Zeile, die
Magnitudentyp-Zeile); docs/concepts:
THE_COUNTER_SLOPE (6, 27, 43), SUNSPOTS (119), SOURCES_V2_SPEC (74,
„honest"), DER_SPEKTRALE_OSZILLATOR (74, 135), THE_SEVEN_SPHERES (159);
docs/surveys: auswertung (17, 53, 56, 59, 79, 109), fortschritt (37,
65, 80), handover-atome (87, 96, 130), handover-2026-08-18-auth (68),
messpunkt-verteilung (3, 41, 60), handover-2026-08-18-v7 (70),
GPU_WATCHDOG_AND_DEVICE_LOSS (61), befund-unwahrheiten-2026-08-18
(14, 20, 98); docs/SOURCE_PORT.md (95); phi/pipeline/research/
agent_output: netcdf_b.φ (34), sdss_photoobj.φ (9-10), argovis_port.φ
(65), psp_hapi_probe.φ (29), eop_finals_probe.φ (16), pro_review_b5.φ (9).
Sauber: AGENTS.md (0), alle .rs (0), index.html (0). Katalog-Treffer
(osm honesty_box, Zenodo-Titel) sind Fremddaten und bleiben.

Urteils-Vorschlag: UNWAHR als Vokabel. Ersatz: die Sache selbst —
„der Voll-Loop-Pfad" (der Overflow trägt ihn), „endet beim severed
Browser" (das Ende ist das Ende), „Diagnosen", „0 honored: keine
exakte SI-Konversion aus der Magnitude — kein Moment erfunden",
„z aus dem specObj-Join" — oder Streichung. Kein neues Tugendwort
als Ersatz.

Aufgabe: Reinigung der lebenden Kanon-Dokumente (TODO, docs/concepts,
docs/SOURCE_PORT, docs/surveys/fortschritt + auswertung + handover-*).
Grenze: die zwei Sonnet-Chat-Protokolle und die phi/pipeline-Noten
sind Aufzeichnungen von Messungen fremder bzw. früherer Stimmen —
sie bleiben unverändert; die Reinigung gilt der Sprache, die das
System heute spricht, nicht der Historie. AGENTS-Ergänzung (die
Vokabel in die Verdachts-Liste) bleibt Rats-Wort.

### Welle 2 — Verhalten

A1 — anchor-TTL 86400 (main.rs:12637, body_channels): B-Fund der
Titan-Archäologie; S2 trug probe_ttl, dieser Default blieb. Die TTL
gehört an die Quelle (deklarierter Wert oder Herleitung), nicht als
hartes 86400 in den Falten. Aufgabe: Herkunft benennen oder 0-honored-
Pfad (ohne Quelle kein Kanal).

A2 — LSK-TTL 86400 (main.rs:9417, naif0012-Fetch): dieselbe Form,
nicht im Register gewesen. Gleiche Behandlung wie A1.

E1 — origins-Stempel vor Fetcherfolg (main.rs:12067, 12105; bei
leerer Rückgabe endet der Thread ohne Berichtigung): der Stempel
`fetched: now` sitzt beim Enqueue — ein Fehlschlag sperrt die Quelle
für die volle TTL. Aufgabe: Stempel erst nach Erfolg; Fehlschlag als
Note benennen. (TODO-A-Punkt: die Klammer „(S3 der 0-Kanon-Session.)"
war falsch — berichtigt.)

C1 — „device samples refused" (main.rs:11657): das verbotene
Identitätswort „device" (Urteil UNWAHR, TODO) trägt die Stations-
Meldung. Aufgabe: „station samples refused".

D3 — transfer_entropy n<8 → 0.0 (main.rs:16578): fehlt ≠ null-echt —
zu wenige Samples liefern 0 („kein Informationsfluss"), eine
Wahrheitsbehauptung, die die Daten nicht tragen. Aufgabe: Option an
die Funktion oder Gate am Aufrufer (das HUD-Gate Ring≥32 existiert);
die 0 bleibt nur die gemessene 0.

F2 — Radiator-Schweigen: `accept` wirft die `try_send`-Rückgabe weg
(main.rs:17514), der Audio-Weg die `send`-Rückgabe (17633). S3 heilte
den Archivar→Renderer-Kanal; diese zwei blieben. Aufgabe: Rückdruck
benennen (geschlossener Kanal = Note).

### Welle 3 — Verdicts

D1 — mpcobs mag-Sentinel 0.0 (mpcobs_compiler.rs:74): mag=0 ist ein
physikalischer Wert (Vega-Klasse); der Kanon erlaubt den 0-Sentinel
nur, wo 0 physikalisch unmöglich ist. „Dokumentierter Formatcode"
ist die Kollision, nicht die Rechtfertigung. Aufgabe: Verdict —
Konsumenten-Gate prüfen, sonst Sentinel ersetzen.

D2 — color_index 0.0 = Weiß (Protokoll v7, 22. f64; tap_compiler.rs:415
`_ => 0.0`, tycho2 0f32): BP−RP=0 ist eine reale Farbe (A0V). Aufgabe:
Verdict für den Slot; Atom A des spektralen Oszillators (v8) erwägt
die Präsenz-Maske für den Farb-Slot.

D4 — Draft-Synthese force 0/kernel 0/tau 0.0 (main.rs:3385-3474):
der Auto-Detect-Draft trägt em/kernel-0 ohne Force-Gate; dunkel nur,
weil τ=0 das Gate schließt. Aufgabe: prüfen, ob der Lauf die
τ-geschlossenen Felder laut notiert (refused) oder still trägt;
Notenpflicht wie port_field_synth.

## S6 — Audit-Nachlese II

A3 — fk.rs UNITS-Default (fk.rs:272): `unwrap_or("ARCSECONDS")` —
der NAIF-Default (frames.req) ist unverified. Aufgabe: verifizieren;
trägt die Spec den Default, ist der Pfad Spec-Wahrheit; sonst
Fabrikation.

A4 — Silverman-Floor (main.rs:16572): Varianz 0 ist eine Messung
(konstante Reihe); `max(1e-30)` fabriziert eine Bandbreite. Aufgabe:
Degeneration benennen (Option; KDE = Delta) — dieselbe Familie wie
der S3-getilgte ft_ref_floor-Floor.

A5 — Aberration (main.rs:84): `sqrt(max(1.0−b2, 1e-12))` — β≥c wird
nicht refused, sondern zu γ≈1e6 gerundet. Aufgabe: positives Gate
(S0-Form): β²≥1 → refuse.

D5 — v3-Maske vs. Falten: j2/j4 None→0.0 (3186-3190), gm None→0.0
(17439), radius None→0.0 (6393). S4 machte None bedeutsam (Bit =
Autorität); die Falten kollabieren wieder auf 0.0. Der Null-Term ist
„keine Korrektur" — Aufgabe: Verdict — Option durchziehen oder als
„kein Term" benennen; nicht still.

E2 — read_cache_if_fresh = cache_fresh + read (main.rs:4868/4853):
Duplikat (C-Punkt); P3 übersprang. Aufgabe: eine Form.

E3 — matmul doppelt (ephemeris_compiler.rs:193 vs fk.rs:294):
Duplikat (C-Punkt). Aufgabe: lib.

F1 — Register-/Cache-Writes ohne Benennung (~15 Stellen): main.rs
9487-9489, 10939, 10944, 11103-11156, 11208-11398, 16527; Compiler-
Ausgaben tycho2 506/627/794, cometels 237, dcom5 204, tap 758-1403.
Aufgabe: wo das Register das Gedächtnis ist, benennt der Fehlschlag
sich selbst.

F3-F5 — eprintln-only Noten (pending/refused ohne Register-Bindung),
Extract-Live-Notiz (leerer Live-Extract ohne Note), Anomalien ohne
GH_TOKEN (kein Ersatz-Register): D-Punkte, offen. Aufgabe:
Register-Bindung je Stelle.

A6 — probe_classify-τ-Konstanten 86400/60/300/0.01/3600 (main.rs
9926/9937/9945 + sensor_config): keine Herleitung im Register.
Aufgabe: Herkunft benennen.

A7 — extract_meta unwrap_or(0) (ephemeris_compiler.rs:629):
unerreichbar (i=0 ist immer Char-Boundary). Aufgabe: streichen.

B1 — GRID_TO_ANGLE (index.html:42, 1245-1246): tote Rotation im
eingefrorenen Relay-Snapshot; als P6 registriert. Kein Atom — die
Registrierung trägt ihn.

## Register-Berichtigungen (im Audit-Commit vollzogen)

- TODO 732-733 gesplittet: die tgas-rv-Hälfte war überholt (der Skip
  lebt seit S1); die color-Slot-Hälfte (0f32) bleibt offen (C3).
- TODO-A-Punkt: die „(S3 der 0-Kanon-Session.)"-Zuweisung berichtigt;
  S5 Welle 2 trägt ihn.
- TODO-Atomkarte: Zeilen 9 (S5) und 10 (S6) eingesetzt.
- Surveys-Tafel: diese Datei aufgenommen.

## Form

Jedes Atom: cargo check 0/0 + volle Suite (exit-code-gewahrsam) +
Commit + TODO-Häkchen. Welle 1 (Dokumente) braucht kein cargo check,
aber denselben Commit-Form: ein Häkchen pro Atom-Zeile, die Meldungen
bleiben bis zum Vollzug im Register.
