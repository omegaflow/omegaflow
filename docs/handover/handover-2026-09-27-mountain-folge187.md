<!--
  title: Handover — Mountain-Folge 187 (Stand 2026-09-27)
  session: Mountain-Folge 187
  class: handover
  date: 2026-09-27
  sha256: 587a49863737160665d05f9e17a84df014bb9b91d7d310c9c0dcf9584b94a194
  status: live
-->
# Handover — Mountain-Folge 187 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Delegations-Consent)

## Offen (aufgeschlüsselt)

### CI-check — archivar-Testrunde verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf am Commit dieser Session.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36333267310`) der rote Lauf trug
  25 archivar- (Mountain) und 5 mathematikerin-Fehler (River). Die 25 archivar-Fälle
  sind behoben (`cargo check` grün, in diesem Commit); die Verifikation durch einen
  frischen `ci-check` steht aus.
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

- (gemessen 2026-09-27 via `ci_manage log 36333267310`) 5 mathematikerin-Fehler
  bleiben offen; Panik-Sites: `src/mathematikerin/actuators.rs:576` („the tone
  oscillates"), `src/mathematikerin/tests.rs:857` (`12 != 10`), `:645`
  (`calm 0.8255919 got 0.5259096`), `:1557` (`volume parity: gpu 0 cpu 2.5`).
  Owner River.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
