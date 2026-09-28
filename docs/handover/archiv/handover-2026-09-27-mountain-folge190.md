<!--
  title: Handover — Mountain-Folge 190 (Stand 2026-09-27)
  session: Mountain-Folge 190
  class: handover
  date: 2026-09-27
  sha256: 0f3af24b2a58236681cebae2f13ec5eebe4af6358bdbe8c096baded813a574b0
  status: live
-->
# Handover — Mountain-Folge 190 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger
für fällig erklärt.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session, Mountain 187)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-27 | Operator (Mountain-Session 190)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht als Handover-Punkt getragen | 2026-09-28 | Operator (Session, Mountain 190)

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD nach diesem Fix-Commit ist beendet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage jobs`/`log 36347555576`, HEAD
  `a457ed8c9`) dort `build`/`format`/`clippy` grün, `test` rot mit zwei Namen —
  `archivar::tests::test_cache_fresh_cdn_stamp_equality_and_release_branch`
  (geheilt: der Test kodierte die alte Stamp-Semantik; `river 47` `e1f1baa65` hat
  die mtime-ttl als Re-Check-Intervall gedreht) und
  `mathematikerin::tests::volume_probe_parity_masked_corner_and_plain` (geheilt:
  der Probe lag exakt auf der Polachse → NaN-Geodät; x-Offset gesetzt); `dropped-gate`
  rot (`baseline 989 | current 1052 | delta 63` — **mycelium**; `dropped-baseline.md`
  fremd gestaged). Die Reader-Features (`where source`, ODF-Serien-Arm) sind in
  diesem Atom gebaut; `cargo check --tests` 0/0.
- **Blockade:** keine.
- **Braucht:** `ci_manage log` des nächsten `ci-check` einmal lesen; grün →
  schließen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ:3`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via
  `sread tools/harvest/src/bin/ned_byparams_compiler.rs`)
  `X-NED-Timeout-Token` nur am Form-POST (`http_post_form`, Z. 116-123);
  `poll_ticket`/`fetch_body` tragen keinen Header; `state/mail/mail_ledger.φ`
  ohne NED-Eintrag (jüngster Eintrag `1790533094` = IGETS, Z. 172).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### ttl-Verdikt-Zeile (Rat 2026-09-27) + no-cadence-Sprachloch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der fremde `AGENTS.md`-Hunk ist committet.
- **Lage:** (gemessen 2026-09-27 via `git status`/`git diff`) `AGENTS.md` trägt
  weiterhin einen fremden uncommitteten Hunk (`M AGENTS.md`) — die Rat-Regeln
  „the pen is the owner's" / „one writer per handover" (2026-09-27). Rat: `ttl` in
  `phi/sources.φ` ist eine Verdikt-Zeile → Mountain; `derive_ttl`
  (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None` — die Datei-Zeile
  trägt; alle 81 `format ephemeris_binary`-Blöcke tragen `ttl 86400` (HEAD). Der
  Flush-Gate `src/archivar/parse.rs:80` (`ttl 0` = inaktiv) trägt keinen eigenen
  „no-cadence"-Zustand.
- **Blockade:** fremder uncommitteter Hunk in `AGENTS.md` (ein pfad-eigener Commit
  würde fremde Arbeit sweepen).
- **Braucht:** nach dem fremden Commit „ttl = Verdikt-Zeile" in `AGENTS.md`
  nachtragen; das no-cadence-Sprachloch als `pending` registrieren.

### NDK-Serien-Arm — der Sweep liest NDK nicht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht (mw am Centroid? m0?).
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36359297755` + Code-Lesung) der reverify-Sweep nennt den GCMT-NDK `format-void — format-gap (no extract declared)` (`fetch.rs:664`); der NDK-Parser lebt (`src/archivar/ndk.rs` `fetch_events`, genutzt von `depth_phase_fleet_probe`), aber der Sweep-Extract-Pfad (`fetch.rs:1123`) trägt keinen `format == "ndk"`-Zweig. Kein `parser-def`-Eintrag — der Parser existiert.
- **Blockade:** keine Register-Blockade; die Feld-Frage ist ungemessen.
- **Braucht:** nach dem Feld-Wort einen `format == "ndk"`-Zweig im `extract()`-Pfad (`fetch.rs:1123`) über `ndk::parse_ndk`, plus die `field`-Deklaration des Skalars.

### Twin-Drift in `declined_sources.φ`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Pass, der `phi/declined_sources.φ` berührt.
- **Lage:** (gemessen 2026-09-28 via awk-Kreuzung) 4 `decline`-Einträge tragen keinen Integrations-Vermerk und stehen zugleich in `sources.φ` (u. a. `declined_sources.φ:2121`/`:2405`/`:3317`); das Gate `blocked_integrated_twin` (`src/gate/commit_gate.rs:1610`) liest nur `blocked_sources.φ`. Im selben Atom verschärft: `descoped` ohne Vermerk im Zwilling ist jetzt hart; die 2 `blocked`-Risse geheilt (purpleair-Twin in `sources.φ` gelöscht, swpc-Vermerk `blocked:245`).
- **Blockade:** jeder Zwilling braucht sein eigenes Chronologie-Verdikt (`git log -S`).
- **Braucht:** die 4 `decline`-Zwillinge messen, Verdikt ausführen; dann `blocked_integrated_twin` auf `declined_sources.φ` ausweiten + Gate-Test.

## Träger (Orphan-Faltung, Operator-Wort „falte alle")

- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` — aufgelöst (Verdikt 2026-09-25, Nachtrag 2026-09-27): kein offener Punkt.
- `docs/concepts/positive-maske.md` — M9.1-Picker verdrahtet (`depth_phase_fleet_probe.rs` `--mww`/`anchor_register`); Treiber-Quellen registriert (GLO-30; Slab2 `phi/sources.φ:14451`; 3D-Tomografie `phi/sources.φ:7529`/`:7534`) — geschlossen.
- `docs/surveys/survey-2026-09-17-sonden-request-only.md` — **gebaut**: nativer ODF-Serien-Arm (`odf.rs` `MAGIC_ODF_SERIES` `ODFS`, `write_odf_series`/`parse_odf_series`/`odf_series`, 4 Tests; Dispatch `odf_serie` in `extract.rs`/`main_flow.rs`).
- `docs/auftrag/auftrag-flyby2-kette.md` — **gebaut**: `where source <name>` isoliert case-insensitiv (`src/archivar/extract.rs::row_matches`, 4 Tests).

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| ESACCI-Reader-Arm | `docs/handover/archiv/handover-2026-09-27-mycelium-folge188.md:131` | mountain folge190 | gebaut (`b4106e69a`, `60dfedd`) und funktional verifiziert: `tools/measure/src/bin/esacci_sst_read.rs` liest den CDN-Bin (1008128 B, sha256 `8b1502c113bc51c83f085ac8d4202233f596f4511d289364317b4e653fe0c386`, 16802 Records, `val 271.19–304.61 K`, mean 286.64, 0 NaN) — die Anforderung ist erfüllt |
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-27-mycelium-folge189.md` | mountain folge189 (`## An Mycelium`) | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge190 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt (ci_mode klassifizierte NDK/asu-tsv als `Malformed Data`) ist in diesem Atom geheilt (`src/archivar/port.rs::ci_body_verdict`) |

## Lehren (dieses Atom)

- **Fixe statt verschleppen (Operator-Wort 2026-09-28).** Ein arbeitbarer Schritt
  wird im Atom gebaut, das ihn nennt; ihn als Handover-Punkt mit `Status: autonom`
  / „dispatchbar" zu tragen ist Verschieben. Angewandt: `where source` gebaut,
  ODF-Serien-Arm gebaut, mathematikerin-Parität geheilt, Polachsen-Riss geheilt.
  Gate verankert: `deferral_markers` + Fixture `deferral_dispatchbar_not_built`
  (`src/gate/commit_gate_vocab.json`, Test in `src/gate/commit_gate.rs`).
- **Polachsen-Geodät (gemessen via CI `volume_probe_parity`).** `motion.rs:402`
  `icrs_to_body_geodetic` degenerierte exakt auf der Rotationsachse (`p = 0` →
  `0/0`); der CPU-Bracket `src/archivar/volume.rs:133` (`f64::max(NaN, 0.0) = 0.0`)
  maskierte das zu einem plausiblen 2.5, die GPU endete auf 0.0. Geheilt: die
  Funktion liefert jetzt `None` statt NaN (Endlichkeits-Schranke).
- **CDN-Frische-Semantik.** `river 47` `e1f1baa65` hat die release-Stamp-Frische
  gedreht (mtime-ttl = Re-Check-Intervall, Stamp entscheidet jenseits der ttl);
  der Archivar-Test war stale und wurde nachgezogen — nicht der Code.
- **Verify-Semantik — non-JSON-Arme (gemessen via CI `36359297755`).** `ci_mode`
  (`src/archivar/port.rs`) prüfte die Erreichbarkeit allein per `parse_json`; jeder
  non-JSON-Body (NDK, asu-tsv) wurde `Malformed Data`. Geheilt: `ci_body_verdict`
  klassifiziert JSON-deklarierte Arme wie bisher, non-JSON über `diagnose_no_samples`
  (`format-gap` → eigener `gapped`-Zähler, `data-present` → reachable); Test
  `test_ci_body_verdict`. Die Rat-Stimme: ein `format-gap` ist nie `dead`.
- **Verdikt-Ausführung — der Register-Riss.** Die 6 VizieR-`asu-tsv`-Zwillinge in
  `sources.φ` standen neben dem `descoped`-Verdikt (`blocked_sources.φ`, folge162
  `8cbdfc028`) — das Verdikt war gesprochen, die Ausführung offen; gelöscht. Das
  Gate `blocked_integrated_twin` (`src/gate/commit_gate.rs`) verschärft: ein
  `descoped`-Zwilling ohne Integrations-Vermerk ist hart; die 2 gemessenen Risse
  (purpleair `blocked:89`, swpc solar_regions `blocked:245`) geheilt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
