<!--
  title: Handover — Mountain-Folge 185 (Stand 2026-09-27)
  session: Mountain-Folge 185
  class: handover
  date: 2026-09-27
  sha256: f02ded8cef13ac575787e967eae2ddacf1e5a93fd6ae23b272c76e0e7661faf3
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
Orphan-Doc-Reconciliation (D5): C — die reconciled Docs bleiben unverändert; kein Träger, keine Freigabe, kein Nachzug-Commit (Diff verwahrt: `state/mountain-185-orphan-doc-nachzug.patch`) | 2026-09-27 | Operator (Session)

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

### SuperDARN-Radar-Positionen — HTML-Parser-Arm
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Port-/Ernte-Lauf der Radar-Stationskoordinaten.
- **Lage:** (gemessen 2026-09-27) `phi/blocked_sources.φ::gap:html-parser-arm`
  (`https://superdarn.ca/radar-info` HTTP 200, 247 907 B, HTML-Tabelle); kein
  Positions-Reader-Arm. FITACF/RAWACF-Compiler stehen (`sources.φ:11487/:9814`).
- **Blockade:** kein HTML-Extraktor-Arm für die Radar-Tabelle.
- **Braucht:** `parser-def html`-Arm bauen (Tabelle → Radar-ID/lat/lon), oder die
  Route als `declined` disponieren, falls die Positionen anders beschaffbar sind.

### Fink-Per-Objekt-Lichtkurven — Reader-Arm
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Erweiterung des Fink-Zeugen auf Per-Objekt-Serien.
- **Lage:** (gemessen 2026-09-27) `https://api.lsst.fink-portal.org/api/v1/sources`
  HTTP 200 (45 B); der Konus lebt als Zeuge (`witnesses.φ:10`,
  `/api/v1/conesearch`), die Per-Objekt-Endpunkte (`/api/v1/sources`,
  `/api/v1/fp`) tragen keinen Reader. Alt-Host `dead` (`dead_sources.φ:143`).
- **Blockade:** kein Per-Objekt-Lichtkurven-Leser; Antwort-Inhalt ungemessen
  (200 mit 45 B).
- **Braucht:** Antwort messen (`curl`/`archive_search --sniff`), dann den
  `skydirection`/Fink-Reader um die Per-Objekt-Route erweitern — oder als
  Reader-Lücke (parser-def) registrieren, falls die Route bestehen bleibt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
