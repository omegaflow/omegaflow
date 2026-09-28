<!--
  title: Handover — Mountain-Folge 192 (Stand 2026-09-28)
  session: Mountain-Folge 192
  class: handover
  date: 2026-09-28
  sha256: eb5d7826cf81231f1c6fbd91c9423a35a35302c094de20766a8da00c760cf586
  status: live
-->
# Handover — Mountain-Folge 192 (2026-09-28)

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
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first — den billigsten Vertreter, dessen Profil den Job trägt; ein `max`-Agent nur für die harten Atome, nie für Routine. Benenne die lokalen Tools (`archive_search`, `sgrep`, `sfetch`) in der Delegation — nicht curl oder webfetch. Benchmarks nur mit Operator-Wort oder gemessen falschem/unvollständigem flash-Ergebnis — Klassen mit gemessenem Sieger werden zitiert, nie verdoppelt (Modell-Politik, AGENTS.md). Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Mountain-Session 192)

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf `36381494347` auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`, HEAD `72db8ef`) `36381494347` ist `pending`; die vorigen ci-check-Läufe (`36380327273`, `36380220789`, `36380125552`, …) wurden durch HEAD-Vorlauf `cancelled`; kein grüner ci-check im Fenster.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36381494347` einmal nach Lauf-Ende lesen; grün → schließen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein `NED_BYPARAMS_TIMEOUT_TOKEN` im Ledger, jüngster Eintrag 2026-09-27 (IGETS). `X-NED-Timeout-Token` sitzt nur am Form-POST (`tools/harvest/src/bin/ned_byparams_compiler.rs`).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### ttl-Verdikt-Zeile (Rat 2026-09-27) + no-cadence-Sprachloch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der fremde `AGENTS.md`-Hunk ist committet.
- **Lage:** (gemessen 2026-09-28 via `git status`) `AGENTS.md` trägt weiterhin einen fremden uncommitteten Hunk (`M AGENTS.md`); `derive_ttl` (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None`; der Flush-Gate (`src/archivar/parse.rs:80`) trägt keinen „no-cadence"-Zustand.
- **Blockade:** fremder uncommitteter Hunk in `AGENTS.md`.
- **Braucht:** nach dem fremden Commit „ttl = Verdikt-Zeile" in `AGENTS.md` nachtragen; das no-cadence-Sprachloch als `pending` registrieren.

### positive-maske.md — Träger (Orphan-Doc)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Pass, der `docs/concepts/positive-maske.md` berührt (die zwei offenen Marker).
- **Lage:** (gemessen 2026-09-28 via `register_lookup --orphan-docs`) das Dokument trägt 2 offene Marker — Stationsterm und die M9.1-Picker-Verdrahtung in die Flotte; Owner Mountain (mycelium-folge188-Fold). `recherche-galileo-kadenz-reconciliation.md` ist geschlossen (Header nachgetragen, stale Marker aufgelöst).
- **Blockade:** keine.
- **Braucht:** die Treiber `Slab2`/`Tomografie` als Ernte-Kandidaten registrieren und den M9.1-Zentroid-Nachfolger in die Flotte verdrahten; Marker schließen.

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-28-mycelium-folge191.md` | mountain folge189 | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge191 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt ist in `src/archivar/port.rs::ci_body_verdict` geheilt |
| NDK-Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht | Future Operator-Queue (`state/future/handover/`) | mountain folge191/192 | der GCMT-NDK-Parser (`src/archivar/ndk.rs`) lebt, der reverify-Sweep ruft ihn nicht (`extract()` ohne `format == \"ndk\"` → `format-void`). Frage: `mw` am Centroid, `m0`, oder kein Skalar? |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
