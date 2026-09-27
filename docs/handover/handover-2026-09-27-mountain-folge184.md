<!--
  title: Handover — Mountain-Folge 184 (Stand 2026-09-27)
  session: Mountain-Folge 184
  class: handover
  date: 2026-09-27
  sha256: d4a3a71a5953340bc7db511012e5c5bf4a65abb471f68287626d3196b8b009ad
  status: live
-->
# Handover — Mountain-Folge 184 (2026-09-27)

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
Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | 2026-09-27 | Operator (Future-Session)

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

### rx100_luminance — CDN-Manifestation (`asset fehlt`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein reales RX100-JPEG + `rx100_compiler --jpeg`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict` + GitHub-API + `sread`)
  der Asset-URL `…/sony-camera-remote/rx100_luminance.bin` liefert live HTTP 404 (Stufen 1–3),
  das Release-Tag `sony-camera-remote` existiert nicht (API 404) — `phi/harvest.φ:233`
  `asset fehlt` ist echt und erwartet; `harvest.yml` klassifiziert die CI-Abwesenheit
  (`absent=true`, run `36323640789` success). Band-Deklaration steht (`phi/sources.φ:122` →
  `… 60 0.0 0.0 5.45e14 3.2e14`), EXIF-Compiler-Pfad gebaut (River `c04e00871`).
  K=12.5 bleibt ISO-2720-Vorgabe, ungemessen.
- **Blockade:** kein reales Capture (Operator-/Sensor-Hand); kein Referenz-Luminanzmeter.
- **Braucht:** bei Capture den Compiler-Lauf + CDN-Manifestation (die `url`-Zeile `phi/sources.φ:117` steht).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
