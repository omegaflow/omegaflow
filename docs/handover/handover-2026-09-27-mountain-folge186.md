<!--
  title: Handover — Mountain-Folge 186 (Stand 2026-09-27)
  session: Mountain-Folge 186
  class: handover
  date: 2026-09-27
  sha256: 8594e899c483573898c43567569d9ae975f7a351fe2c48e125b86224b40d9fec
  status: live
-->
# Handover — Mountain-Folge 186 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Delegations-Consent)

## Offen (aufgeschlüsselt)

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen folge185/2026-09-27) `X-NED-Timeout-Token` nur am Form-POST
  (`ned_byparams_compiler.rs:122-123`); Poll-GET und Ergebnis-Fetch ohne Header;
  `.secrets.local` und `state/mail/mail_ledger.φ` ohne Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen.

### Fink-Per-Objekt-Lichtkurven — Reader-Erweiterung
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster Bau-Augenblick (Schema gemessen).
- **Lage:** (gemessen 2026-09-27 via `archive_search`/`sfetch`) Konus lebt als Zeuge
  (`phi/witnesses.φ:10`); echter `diaObjectId` 314002968168367863 via conesearch
  (ra 55.0, dec −30.0, r 3600); `GET /api/v1/sources?diaObjectId=…` HTTP 200, 3602 B,
  135 Keys (`r:`/`f:`/`xm:`); `GET /api/v1/fp?diaObjectId=…` HTTP 200, `[]`.
- **Blockade:** keine.
- **Braucht:** `tools/measure/src/weberin/fink_alerce.rs` um die Per-Objekt-Route
  erweitern (Schema liegt vor); Alt-Host `dead` (`phi/dead_sources.φ:143`).

## An Mycelium (Register-pending, aus Mountain-Feder)

Drei gemessene Live-Routen liegen als `pending` in `phi/blocked_sources.φ`; die
`sources.φ`-Zeile, der Compiler und die CDN-Manifestation sind Myceliums Feder
(`pending` → mycelium).

- **SuperDARN Radar-Positionen** — `phi/blocked_sources.φ` pending
  `github.com/vtsuperdarn/hdw.dat` (41 `hdw.dat.<code>`, lat Feld 4 / lon Feld 5,
  raw 200). Ersetzt den HTML-Arm.
- **DAS2 Iowa** — `phi/blocked_sources.φ` pending
  `planet.physics.uiowa.edu/das/das2Server/hapi` (HAPI 1.1, CSV); native das2.2
  `jupiter.physics.uiowa.edu/das/server` (551 Datasets, u. a. Galileo/Juno/Cassini
  MAG nT) → Coverage-Frage offen.
- **Occultation-DB UTFPR** — `phi/blocked_sources.φ` pending
  `occultations.ct.utfpr.edu.br` (JSON-API `/api/objects`+`/api/events`;
  radius/albedo/density/oblateness/atmosphere/Δ/JD+RA-DEC; keine Stations-lat/lon).

## Verweise (Prosa mit offenen Markern)

- `docs/surveys/survey-2026-09-13-weberin-quellen-treffer.md` und `-rerun.md` sind
  trägerlos; der Orphan-Zensus gehört in den Stehenden Pass (Mycelium). Die Docs
  bleiben per Operator-Wort C unverändert.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
