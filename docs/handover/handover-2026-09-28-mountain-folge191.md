<!--
  title: Handover — Mountain-Folge 191 (Stand 2026-09-28)
  session: Mountain-Folge 191
  class: handover
  date: 2026-09-28
  sha256: 76f013d35e7aba5e7b2fb1b1427345ef8d54decfb8741cd9f62caf0054f49494
  status: live
-->
# Handover — Mountain-Folge 191 (2026-09-28)

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
- **Trigger:** der `ci-check`-Lauf auf HEAD beendet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`, HEAD `3f0304c6d`) die
  Reader-Fixes (`a457ed8c9`, `491c6e61c`) sind gebaut; der `ci-check`-Lauf auf dem
  neuen HEAD (`36377769510`) war zuletzt `pending`, `36377277112` in_progress —
  dieser Atom-Commit erzeugt einen weiteren HEAD und damit einen weiteren Lauf.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des nächsten `ci-check` einmal lesen; grün →
  schließen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ:3`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread tools/harvest/src/bin/ned_byparams_compiler.rs`)
  `X-NED-Timeout-Token` nur am Form-POST (`http_post_form`, Z. 116-123);
  `poll_ticket`/`fetch_body` tragen keinen Header; `state/mail/mail_ledger.φ`
  ohne NED-Eintrag.
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### ttl-Verdikt-Zeile (Rat 2026-09-27) + no-cadence-Sprachloch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der fremde `AGENTS.md`-Hunk ist committet.
- **Lage:** (gemessen 2026-09-28 via `git status`) `AGENTS.md` trägt weiterhin einen
  fremden uncommitteten Hunk (`M AGENTS.md`, +5 Z.) — die Rat-Regeln „the pen is the
  owner's" / „one writer per handover" (2026-09-27). Rat: `ttl` in `phi/sources.φ` ist
  eine Verdikt-Zeile → Mountain; `derive_ttl` (`src/archivar/port.rs:1237`) liefert für
  Binär-SPK `None` — die Datei-Zeile trägt; der Flush-Gate `src/archivar/parse.rs:80`
  (`ttl 0` = inaktiv) trägt keinen eigenen „no-cadence"-Zustand.
- **Blockade:** fremder uncommitteter Hunk in `AGENTS.md`.
- **Braucht:** nach dem fremden Commit „ttl = Verdikt-Zeile" in `AGENTS.md`
  nachtragen; das no-cadence-Sprachloch als `pending` registrieren.

### NDK-Serien-Arm — der Sweep liest NDK nicht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht (mw am Centroid? m0?).
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36359297755` + Code-Lesung) der reverify-Sweep nennt den GCMT-NDK `format-void — format-gap (no extract declared)` (`fetch.rs:664`); der NDK-Parser lebt (`src/archivar/ndk.rs` `fetch_events`, genutzt von `depth_phase_fleet_probe`), aber der Sweep-Extract-Pfad (`fetch.rs:1123`) trägt keinen `format == "ndk"`-Zweig. Kein `parser-def`-Eintrag — der Parser existiert.
- **Blockade:** keine Register-Blockade; die Feld-Frage ist ungemessen.
- **Braucht:** nach dem Feld-Wort einen `format == "ndk"`-Zweig im `extract()`-Pfad (`fetch.rs:1123`) über `ndk::parse_ndk`, plus die `field`-Deklaration des Skalars.

### Twin-Drift in `declined_sources.φ`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Pass, der `phi/declined_sources.φ` berührt.
- **Lage:** (gemessen 2026-09-28 via awk-Kreuzung) 4 `decline`-Einträge tragen keinen Integrations-Vermerk und stehen zugleich in `sources.φ` (u. a. `declined_sources.φ:2121`/`:2405`/`:3317`); das Gate `blocked_integrated_twin` (`src/gate/commit_gate.rs:1610`) liest nur `blocked_sources.φ`. Im selben Atom verschärft: `descoped` ohne Vermerk im Zwilling ist jetzt hart; die 2 `blocked`-Risse geheilt (purpleair-Twin in `sources.φ` gelöscht, swpc-Vermerk `blocked:245`).
- **Blockade:** jeder Zwilling braucht sein eigenes Chronologie-Verdikt (`git log -S`).
- **Braucht:** die 4 `decline`-Zwillinge messen, Verdikt ausführen; dann `blocked_integrated_twin` auf `declined_sources.φ` ausweiten + Gate-Test.

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-28-mycelium-folge190.md` | mountain folge189 (`## An Mycelium`) | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge191 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt (ci_mode klassifizierte NDK/asu-tsv als `Malformed Data`) ist in diesem Atom geheilt (`src/archivar/port.rs::ci_body_verdict`) |

## Lehren (dieses Atom)

- **Verify-Semantik — non-JSON-Arme (gemessen via CI `36359297755`).** `ci_mode`
  (`src/archivar/port.rs`) prüfte die Erreichbarkeit allein per `parse_json`; jeder
  non-JSON-Body (NDK, asu-tsv) wurde `Malformed Data`. Geheilt: `ci_body_verdict`
  klassifiziert JSON-deklarierte Arme wie bisher, non-JSON über `diagnose_no_samples`
  (`format-gap` → eigener `gapped`-Zähler, `data-present` → reachable); Test
  `test_ci_body_verdict`. Ein `format-gap` ist nie `dead`.
- **Verdikt-Ausführung — der Register-Riss.** Die 6 VizieR-`asu-tsv`-Zwillinge in
  `sources.φ` standen neben dem `descoped`-Verdikt (`blocked_sources.φ`, folge162
  `8cbdfc028`) — das Verdikt war gesprochen, die Ausführung offen; gelöscht. Das
  Gate `blocked_integrated_twin` (`src/gate/commit_gate.rs`) verschärft: ein
  `descoped`-Zwilling ohne Integrations-Vermerk ist hart; die 2 gemessenen Risse
  (purpleair `blocked:89`, swpc solar_regions `blocked:245`) geheilt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
