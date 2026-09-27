<!--
  title: Handover — Mountain-Folge 185 (Stand 2026-09-27)
  session: Mountain-Folge 185
  class: handover
  date: 2026-09-27
  sha256: 23870b4fb23f1f1eff912e2c9bb223889d21f6af645885c5d964ff407bf77998
  status: live
-->
# Handover — Mountain-Folge 185 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)
„Du kannst" Mountain-Folge 182 | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort
„die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session)
RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | 2026-09-27 | Operator (Future-Session)
Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | 2026-09-27 | Operator (Future-Session)
kein Foto des Operators im CDN — RX100-Capture nicht manifestieren | 2026-09-27 | Operator (Session)
„Du kannst" — Ausführung des Phase-1-Plans (line-Agent) | 2026-09-27 | Operator (Session, Delegations-Consent)

## Offen (aufgeschlüsselt)

### NED ByParams — Token-Kanal (Code-Kanal gemessen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) `X-NED-Timeout-Token` wird
  nur am Form-POST gesetzt (`ned_byparams_compiler.rs:122-123`); die
  Ticket-Poll-GET (`poll_ticket` → `http_get`) und der Ergebnis-Fetch
  (`fetch_body`) tragen ihn nicht. `state/mail/mail_ledger.φ` (170 Z.) ohne
  NED-Eintrag, `.secrets.local` ohne Token → Trigger nicht eingetroffen.
- **Blockade:** Token fehlt; die entscheidende Messung ist der echte Job.
- **Braucht:** mit dem Token den echten ByParams-Job fahren und messen, ob
  Poll/Fetch ohne Header scheitern; falls ja, Token an `http_get`-Signatur und
  `fetch_body` (Header + Cookie-Jar) ergänzen.

### gap:iaga-text — IAGA-2002-Zeilenleser
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Port-/Ernte-Lauf der Zenodo-Alberta-GIC-Messung.
- **Lage:** (gemessen 2026-09-27) `phi/blocked_sources.φ::gap:iaga-text`
  (Zenodo record 10594301, `Mag_Data.zip` nT, IAGA-2002-Text) — der Reader-Arm
  fehlt; `pack_iaga` (`src/archivar/geo.rs:302`) packt nur Stations-Codes, kein
  Datenformat. Der Modell-Teil ist `decline model` (`declined_sources.φ:5143`).
- **Blockade:** kein IAGA-2002-Datenformat-Leser; die Datei-Erreichbarkeit des
  Records ist ungemessen (Datenblatt 200, Record 504).
- **Braucht:** zuerst `archive_search --verdict https://zenodo.org/records/10594301`
  (Route messen); dann `pack_iaga` um den IAGA-2002-Zeilenleser erweitern — oder
  die Route als `declined` disponieren, falls die Datei nicht erreichbar ist.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
