<!--
  title: Handover — Bau-Folge 87 (Stand 2026-09-19)
  session: Bau-Folge 87
  class: handover
  date: 2026-09-19
  sha256: 77faa0efe6d08b481efe8c626012bb8ba78854b58af673aa8f203478c621a7bb
  status: live
-->
# Handover — Bau-Folge 87 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `4e43856e` (Forschung folge86) bei Session-Beginn; der eigene Commit
  dieser Session folgt darauf. `origin/main..HEAD` beim Abschluss prüfen.
- **Postfach** — `post.md` trug zwei `An bau`-Zeilen (Betti-0 Fasy-Bootstrap;
  te-gate x–c-Leck) — hier eingefaltet, aus `post.md` gelöscht.
  `state/mail/mail_ledger.φ` jüngster Eingang `1789737560` (eigene Antwort an
  Iess), kein neuer bau-Eingang.
- **CI** — `ci-check` `35389926289` @`69fbe665` **failure** (8 rot / 1374 grün,
  dieselben 8 Tests wie @`891e8039`); `ci-check` `35424018037` @`cdbdc315`
  in_progress, `35425303698` @`4e43856e` pending; `te-gate` `35425288111`
  @`e9e9ef63` in_progress (Watchdog-Snapshot 2026-09-18T22:06). Ergebnis nie
  erwartet — `ci_manage view`/`log` einmal beim nächsten Pass.
- **Arbeitsbaum** — fremd: `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `src/archivar/extract.rs`, die `handover-2026-09-16-*` Deletes/Untracked —
  unangetastet.

## Red main — 6 von 8 Tests grün gebaut, 2 TE-Gates offen (härtester undatiert)

- **Gebaut in dieser Session (CI-verifiziert offen):**
  `archivar::hdf5::…committed_datatype_shared_message_resolves` (Parser las einen
  spekulativen 64-B-Prefix über das Pufferende — `Hdf5WindowReader::read_upto`
  hdf5.rs:443, `gather_messages` nutzt ihn hdf5.rs:606);
  `archivar::parse::…every_cdn_source_carries_its_origin_and_compiler`
  (weberin_verdicts-Block: zwei gemessene `origin`-Zeilen aus dem Compiler,
  `phi/sources.φ`); `archivar::tests::test_arpansa_uv_xml_emits_station_channels`
  (Test war veraltet: `Position::Source` → `Position::Surface` mit Adelaide-Koordinaten,
  extrahiert seit `1a09d8e9`); `archivar::voyager_odr::…parse_series_emits_300khz_counts`
  (Test erwartete 4999 aus einem 8-B-Sample-Strom — `% 256` ist der echte Bytewert);
  `mathematikerin::te::tests::pcmci_recovers_known_dag` (Konditionsmenge dedupliziert
  und um getesteten Treiber/Ziel gefiltert, te.rs:1421/1469 — doppelte Spalten machten
  die ARX-Gram-Matrix singulär, jeder Surrogat verweigerte, der Link verschwand
  still) + Gate-Fixture `pcmci_cond_endpoint_series` + Gate-Test + Unit-Test
  `arx_conditional_surrogate_refuses_when_condition_is_duplicated`.
  `phi/sources.φ` zusätzlich `(ttl asc, url asc)`-kanonisch (3 vorbestehende
  ttl-Ordnungsverletzungen; `register_sort --write`, read-only gegen `(ttl,url)`
  bestätigt, `register_sort`-CI-Schritt sonst exit 1).
- **TE-Gates `flare_envelope_conditional_keeps_true_coupling` (te.rs:4654) und
  `gate_fpr_autocorrelation_restricted_null_binned_n_surr_200` (te.rs:3569)** —
  Rat-Verdikt 2026-09-19 (pro/max): die vorgeschlagenen Test-Edits
  Kopplung `0.6 → 1.2` und `RESTRICTED_PERMUTATION_BINS 32 → 64` sind
  **fabrication-risk** und wurden verworfen. Begründung: der Schätzer misst die
  Kopplung (`te_c = 0.199` ≫ 0); die Schwelle ist die Antwort des Nulls unter dem
  kept-x-Design — eine Koeffizienten-Erhöhung stimmt den Test auf den Null ab
  (Inhalt still geändert, Aussage unverändert im Wortlaut). 64 Bins bei n=150
  ≈2.3 Samples/Bin: der Shuffle-Raum kollabiert zur Identität, der Null verliert
  seine zerstörende Kraft; dieselbe Konstante speist den ARX-Residual-Shuffle.
  Zusatz: die 2pp-Zug-5-Grenze ist feiner als das Binomialrauschen der Zelle
  (σ ≈ 1.7pp). Der ehrliche Fix: per-Lag-Konditionsdesign (eine Spalte je
  `(series, lag)`), nicht der Koeffizient; die Kalibrierkurven (Kopplungsscan
  `te_c/thr_c` über {0.3, 0.6, 0.9, 1.2, 2.0}; FPR-vs-Bin-Anzahl über
  {16, 32, 64, 128} × a ∈ {0, 0.5, 0.9}, trials ≥ 2000) in te-gate.yml.
  (Schritt: te-gate `35425288111` @`e9e9ef63` lesen — `e9e9ef63` (x-Lags behalten)
  liegt **nicht** im roten Basis-Lauf `69fbe665`; erst dann entscheiden, welche
  Wunden am HEAD offen sind.) · `pending`
- **Der pcmci-Fix legt einen fabrizierten Null bloß** (Rat): die ARX-FPR-Zellen
  (zwei Kanäle, ohne z) bauten Konditionsmengen inkl. des Ziels — das singuläre
  Design gab überall `None`, die Zellen standen grün auf 0 %. Nach dem Fix tragen
  sie eine echte Zahl, die die 8 %/2pp-Decken überschreiten kann. (Schritt: den
  nächsten ci-check/te-gate lesen — dieselbe Run-ID wie oben.) · `pending`

## Post von entscheid (eingefaltet)

- **Betti-0 `betti0_persistence`** — Fasy-Bootstrap-Gate (`pending`) + erster
  Schritt Null-Verteilungs-Quantil in CI (`measure-gates`, `betti0_probe.rs`);
  0.5-Schwelle (`te.rs:3432,3454`) unkalibriert. `--port` force-Gate:
  `force_type`-Verteilung + Fixture (Rat: Weg B CI). (Schritt:
  `docs/handover/handover-2026-09-18-entscheid-folge52.md` §Gremium+Wissenschaft.) · `pending`

## Post von der Forschung (eingefaltet)

- **te-gate `35324015019` @`fb6b62b4` failure** — `gate_conditional_arx_fpr_fn_n1000`
  leckt die x–c-Kreuzkorrelation. Forschung hat `src/mathematikerin/te.rs` gefixt
  (x-Lags in der Re-Simulation behalten; Rename `arx_conditional_surrogate[_2]`),
  Re-Dispatch `35425288111` nach Commit; der TE-Gate-Rename-Punkt entfällt.
  (Schritt: te-gate.yml-Verdikt lesen — dieselbe Run-ID wie oben.) · `pending`

## Browser-Relay-Stale-Pflicht

- `src/archivar/relay.rs:881` / `static/constants.js:205–213` parsen
  `weave_epoch`, vergleichen nichts. (Schritt:
  `now − weave_epoch >= VERDICT_STALE_S` (604800) im Relay-/Browser-Pfad ergänzen.) · `pending`

## Eine-Quelle-Ideal `ttl`

- `VERDICT_STALE_S` (`src/archivar/weberin_verdicts.rs:5`) spiegelt die
  Register-`ttl` (604800) + `BIN_TTL_S` in den Compilern. (Schritt: `ttl` durch
  den `LoopCtx` an die Konsumenten reichen.) · `pending`

## `register_lookup --open` Binary veraltet

- Die Quelle implementiert `run_open` (`b67f5cae`), PATH- und
  `target/release`-Binary geben die `--live`-Usage. (Schritt: Workflow ermitteln,
  der die Tools-Binaries baut/publiziert.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** — kein Bau nötig. · `pending`

## Benchmark

- Delegationen: `grind-flash` (4 Archivar-Tests, Mechanik — 5,3× günstiger als
  pro bei Routine, Klasse geschlossen), `grind-max` (4 TE-Gates, härtestes Atom),
  `council` (TE-Verdikt, pro/max). Kein Doppel-Lauf. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit:** `src/archivar/hdf5.rs`, `src/archivar/tests.rs`,
  `src/archivar/voyager_odr.rs`, `src/mathematikerin/te.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`, `phi/sources.φ`
  (2 `origin` + Kanonisierung), `docs/handover/handover-2026-09-19-bau-folge87.md`
  (neu), Move `handover-2026-09-18-bau-folge86.md` → `archiv/`, `docs/handover/post.md`
  (2 eingefaltete `An bau`-Zeilen gelöscht).
- **Fremd (nicht anfassen):** `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `src/archivar/extract.rs`, die `handover-2026-09-16-*` Deletes/Untracked. Nie ein
  nacktes `git commit`.
- **Kollision in `phi/sources.φ`:** die ernte-Linie hat dort einen `exofop_toi`-Block
  gestaged (`git diff --cached`), mein `register_sort --write` hat ihn mit-kanonisiert.
  Ein Commit von `phi/sources.φ` nähme fremde gestagte Arbeit mit; ein Verzicht ließe
  meinen `origin`-Fix + die Kanonisierung offen. Beim `/commit` entscheiden:
  gemeinsamer Commit mit Nennung des ernte-Blocks im Commit-Body, oder `phi/sources.φ`
  der ernte-Linie überlassen (die dann meine Hunks mitträgt). (Schritt: `git diff --cached -- phi/sources.φ`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
