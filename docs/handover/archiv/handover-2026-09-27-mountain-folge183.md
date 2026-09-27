<!--
  title: Handover — Mountain-Folge 183 (Stand 2026-09-27)
  session: Mountain-Folge 183
  class: handover
  date: 2026-09-27
  sha256: e63d62ae9aa890e3271b87e9d773335c9aa09a28b95da77389ca1b50237de981
  status: live
-->
# Handover — Mountain-Folge 183 (2026-09-27)

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
RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | 2026-09-27 | Operator (Future-Session)

## Offen (aufgeschlüsselt)

### NED ByParams — Token-Kanal (Code-Kanal gemessen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread`) `X-NED-Timeout-Token` wird nur am
  Form-POST gesetzt (`ned_byparams_compiler.rs:122-123`); die Ticket-Poll-GET
  (`poll_ticket` → `http_get`, kein `-H`) und der Ergebnis-Fetch (`fetch_body`,
  kein Token, kein Cookie-Jar) tragen ihn nicht. Ob NED den Token sessionweit
  prüft, ist aus Code und öffentlicher Doku nicht ableitbar.
- **Blockade:** Token fehlt (Mail nicht eingetroffen); die entscheidende Messung ist der echte Job.
- **Braucht:** mit dem Token den echten ByParams-Job fahren und messen, ob
  Poll/Fetch ohne Header scheitern; falls ja, Token an `http_get`-Signatur und
  `fetch_body` (Header + Cookie-Jar) ergänzen.

### clippy `-D warnings` — Mountain-Dateien (Verifikation am HEAD)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` `36320549136` am HEAD `edfcedad0` fertig (gemessen
  2026-09-27 via `ci_manage view` — pending).
- **Lage:** (gemessen 2026-09-27 via `cargo check` + `cargo fmt`) geheilt:
  `aia.rs:41`/`eve.rs:49` (`question_mark`), `hdf4.rs:303` (`type_complexity`),
  `:435` (`needless_range_loop`), `:802` (`unnecessary_cast`), `channels.rs:580`
  (`question_mark`), `:645` (`manual_contains`), `uws.rs:242` (`manual_unwrap_or`),
  `tests.rs:6322` (`assertions_on_constants`); `cargo check` 0/0, `cargo fmt` clean.
  Die Test-Target-Zeilen sind nur per CI verifizierbar.
- **Blockade:** kein lokales clippy; Lauf pending.
- **Braucht:** `ci_manage log 36320549136` am HEAD lesen; die test-Target-Zeilen
  (`src/archivar/tests.rs`) verifizieren.

### RX100 Luminanz — Kamera-EXIF-Weg (Kalibrierer descoped; Bau an River)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via `git diff`/`sgrep`/`sread`) der EXIF-Weg ist
  von River gebaut (River-Atom 2026-09-27: `src/archivar/rx100.rs` `exif_exposure`,
  `rx100_compiler --jpeg`, Band in `extract.rs` `series_rows`) — der Compiler-Teil
  dieser Zeile ist damit erledigt, nicht doppelt bauen. Offen in Mountain-Recht:
  `phi/sources.φ:117-122` (`field rx100_luminance_cdm2 … inverse-square em cd/m2
  60 0.0 0.0`) hat die Band-Slots `freq`/`bin_width` **absent** (0.0), während der
  Record-Schreiber jetzt `5.45e14`/`3.2e14` trägt — Deklaration und Draht
  widersprechen sich. `harvest.φ:233` `asset fehlt` bleibt (kein reales Capture).
  K=12.5 bleibt ISO-2720-Vorgabe, ungemessen (kein Referenz-Luminanzmeter).
- **Blockade:** kein reales Capture für die CDN-Manifestation.
- **Braucht:** `phi/sources.φ:122` Feld-Deklaration auf `… 60 0.0 0.0 5.45e14 3.2e14`
  setzen (bzw. messen, ob `series_rows` die einzige Band-Quelle bleibt; dann die
  Feld-Slots als bewusst latent benennen), `register_sort` canonical; CDN-Manifest,
  sobald ein reales JPEG + Compiler-Lauf steht.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
