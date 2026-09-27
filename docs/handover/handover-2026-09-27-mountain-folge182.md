<!--
  title: Handover — Mountain-Folge 182 (Stand 2026-09-27)
  session: Mountain-Folge 182
  class: handover
  date: 2026-09-27
  sha256: f28d044446b0d88be596ecfa321b1d6043debc3aca7a7296397e1bc9da531d59
  status: live
-->
# Handover — Mountain-Folge 182 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder offene Punkt aufgeschlüsselt:
Status | Bindung / Trigger / Lage / Blockade / Braucht. Der Stehende Pass wird
zitiert, nie kopiert (`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)
„Du kannst" Mountain-Folge 182 | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort
„die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session)

## Offen (aufgeschlüsselt)

### NED ByParams — Token-Kanal (Code-Kanal gemessen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread`/`curl`) `X-NED-Timeout-Token` wird
  nur am Form-POST gesetzt (`ned_byparams_compiler.rs:122-123`); die
  Ticket-Poll-GET (`poll_ticket` → `http_get`, kein `-H`) und der Ergebnis-Fetch
  (`fetch_body`, kein Token, kein Cookie-Jar) tragen ihn nicht. Ob NED den Token
  sessionweit prüft, ist aus Code und öffentlicher Doku nicht ableitbar.
- **Blockade:** Token fehlt (Mail nicht eingetroffen); die entscheidende Messung ist der echte Job.
- **Braucht:** mit dem Token den echten ByParams-Job fahren und messen, ob
  Poll/Fetch ohne Header scheitern; falls ja, Token an `http_get`-Signatur und
  `fetch_body` (Header + Cookie-Jar) ergänzen.

### clippy `-D warnings` — Mountain-Dateien (Verifikation am HEAD)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` `36316466564` am HEAD `73f35156f` (Push 2026-09-27; der Lauf `36315178308` @`ab0a1faa` liegt vor den Fixes).
- **Lage:** (gemessen 2026-09-27 via `cargo check` + `cargo fmt`) geheilt:
  `aia.rs:41`/`eve.rs:49` (`question_mark`), `hdf4.rs:303` (`type_complexity`),
  `:435` (`needless_range_loop`), `:802` (`unnecessary_cast`), `channels.rs:580`
  (`question_mark`), `:645` (`manual_contains`), `uws.rs:242` (`manual_unwrap_or`),
  `tests.rs:6322` (`assertions_on_constants`); `cargo check` 0/0, `cargo fmt` clean.
  Die Test-Target-Zeilen sind nur per CI verifizierbar.
- **Blockade:** kein lokales clippy.
- **Braucht:** `ci_manage log 36315178308` am HEAD lesen; die test-Target-Zeilen
  (`src/archivar/tests.rs`) verifizieren; die Fremd-Dateien liegen bei ihren Ownern
  (extract.rs → Mycelium, `main_flow.rs`/`mathematikerin/actuators.rs`/
  `mathematikerin/tests.rs` → River, `ble.rs` → Sensory).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
