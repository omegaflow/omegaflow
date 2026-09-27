<!--
  title: Handover — Mountain-Folge 188 (Stand 2026-09-27)
  session: Mountain-Folge 188
  class: handover
  date: 2026-09-27
  sha256: a94e367d6379ec016e81280b2ac38c9ee1782939edb8ce15d96f1fcde7a68180
  status: live
-->
# Handover — Mountain-Folge 188 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session, Mountain 187)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-27 | Operator (Mountain-Session 188)

## Offen (aufgeschlüsselt)

### CI-check — archivar-Testrunde verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf am Commit dieser Session.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view/log 36341839537`) der Lauf
  @`5780d939f` (Mountain 187) trug noch **4 rote archivar-Fälle** — die 25 waren
  nicht vollständig begrünt: `charm2::tests::parse_tsv_keeps_positive_parallax_matches_and_drops_absent`
  (`by_recno.into_values()` = nichtdeterministische HashMap-Ordnung), `iaga::tests::parse_text_reads_the_measured_fcc_second_family`
  + `to_geo_rows_emits_one_record_per_present_component` (f64-Exact `-94.088`),
  `tests::test_port_block_with_force_and_name_unit_synthesizes` (nicht-physikalischer
  Fixture-Name `geomagnetic_index` → declined statt pending). Alle vier in diesem
  Atom geheilt (`charm2` Reihenfolge-Einfügereihenfolge, `iaga` ε-Vergleich, `tests`
  physikalischer Fixture-Name `omni_bx_gsm`); `cargo check` 0/0. Die Verifikation
  durch einen frischen `ci-check` steht aus.
- **Blockade:** keine.
- **Braucht:** nach dem Push `gh workflow run ci-check`; Ergebnis **einmal** lesen —
  `ci_manage view <id>` / `ci_manage log <id>` (kein Polling).

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread tools/harvest/src/bin/ned_byparams_compiler.rs`)
  `X-NED-Timeout-Token` nur am Form-POST (`http_post_form`, Z. 116-123);
  `poll_ticket` (`http_get`, Z. 247-280) und `fetch_body` (Z. 325) tragen bewusst
  keinen Header; `.secrets.local` und `state/mail/mail_ledger.φ` ohne Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

## An River (gemessen, fremde Feder)

- (gemessen 2026-09-27 via `ci_manage log 36341839537`) 4 mathematikerin-Fälle
  offen: `mathematikerin::actuators::tests::a_negative_sum_routes_the_tone_left`
  (`src/mathematikerin/actuators.rs:576` „the tone oscillates"),
  `mathematikerin::tests::the_no_te_branch_logs_one_line_per_fresh_field_sample`
  (`tests.rs:857`, `12 != 10`),
  `mathematikerin::tests::the_tone_code_relaxes_the_aperture_between_floor_and_unity`
  (`tests.rs:645`, `calm 0.8255919 got 0.5259096`),
  `mathematikerin::tests::volume_probe_parity_masked_corner_and_plain`
  (`tests.rs:1557`, `volume parity: gpu 0 cpu 2.5`). Owner River.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
