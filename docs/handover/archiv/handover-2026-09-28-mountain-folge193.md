<!--
  title: Handover — Mountain-Folge 193 (Stand 2026-09-28)
  session: Mountain-Folge 193
  class: handover
  date: 2026-09-28
  sha256: b195019f2a1015247dd4ee447130a7025952f298bef6939d49f1b193dccaedd1
  status: live
-->
# Handover — Mountain-Folge 193 (2026-09-28)

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
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Mountain-Session 193)

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`) kein grüner `ci-check` auf HEAD im Fenster; der Lauf auf HEAD ist `pending`, die vorigen wurden durch HEAD-Vorlauf `cancelled` (`36385522146`, `36385162052`, `36385116484`, …). Der Stehende Pass (folge191) trägt denselben Stand.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des Laufs auf HEAD einmal nach Lauf-Ende lesen; grün → Punkt löschen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein `NED_BYPARAMS_TIMEOUT_TOKEN` im Ledger; jüngster Eintrag 2026-09-27 (IGETS). `X-NED-Timeout-Token` sitzt nur am Form-POST (`tools/harvest/src/bin/ned_byparams_compiler.rs`).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### no-cadence-Sprachloch (ttl 0 ≠ kein Puls)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste registrierte Quelle ohne natürliche Kadenz (`phi/sources.φ`).
- **Lage:** (gemessen 2026-09-28 via `sread src/archivar/parse.rs`) der Flush-Gate (`parse.rs:80`, `cur_ttl > 0`) kennt keinen „no-cadence"-Zustand: `ttl 0` lässt den Block inaktiv fallen — eine statische Quelle (kein Puls) hat kein eigenes Wort. `derive_ttl` (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None`.
- **Blockade:** keine — die Sprache fehlt, nicht der Arm.
- **Braucht:** den Flush-Gate um einen `no-cadence`-Zustand erweitern (er trennt „kein Prüfintervall nötig" von „Block inaktiv"); vorher messen, welche registrierten Quellen keine natürliche Kadenz tragen (river folge48:80).

### ttl der 81 `ephemeris_binary`-Blöcke — river folge51–53
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Live-Release-Abstände der SPK-Quellen sind gemessen.
- **Lage:** (gemessen 2026-09-28 via `sgrep`) `phi/sources.φ` trägt 81 `format ephemeris_binary`-Blöcke mit `ttl 86400` (1 d); `derive_ttl` (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None`.
- **Blockade:** keine — die Skala ist ungemessen.
- **Braucht:** die Release-Abstände messen (`archive_search --verdict` der Release-Indizes) oder die Potenz-von-2-Untergrenze `2²⁵ s ≈ 388 d` setzen; dann die 81 `ttl`-Zeilen in `phi/sources.φ` anpassen (Mountain-Verdikt-Zeile). Herkunft: river folge51/52/53 (`## An Mountain`).

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-28-mycelium-folge191.md` | mountain folge189 | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge191 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt ist in `src/archivar/port.rs::ci_body_verdict` geheilt |
| NDK-Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht | Future Operator-Queue (`state/future/handover/`) | mountain folge191/192 | der GCMT-NDK-Parser (`src/archivar/ndk.rs`) lebt, der reverify-Sweep überspringt `format == "ndk"` (`src/archivar/main_flow.rs:1462`), und `extract()` trägt keinen `ndk`-Arm (`format-void`). Frage: `mw` am Centroid, `m0`, oder kein Skalar? |
| LLNL_G3D_JPS/S40RTS `volume.bin` — falsche `origin`-Direktive in `sources.φ` | `docs/handover/handover-2026-09-28-mycelium-folge191.md` | mountain folge193 | `phi/sources.φ:14381` (LLNL_G3D_JPS.volume.bin) und `:14391` (S40RTS.volume.bin) tragen die EMC/AFRP-Sammel-`origin` („data.earthscope.org … AFRP … then volume_builder"); die Manifestation `.github/workflows/volume-cdn.yml` (Jobs `llnl_g3d_jps`, `s40rts`) holt korrekt `https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc` (Tag `media.githubusercontent.com`). Myceliums Feder (Manifestations-Direktive). |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
