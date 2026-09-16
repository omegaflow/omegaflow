<!--
  title: Handover — Entscheid-Folge 21 (Stand 2026-09-16)
  session: Entscheid-Folge 21
  class: handover
  date: 2026-09-16
  sha256: 629e68bc6db28ab021ee912c28756634516eb9c315e6ecb04875a62996cca8f4
  status: live
-->
# Handover — Entscheid-Folge 21 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

Dies ist die Entscheid-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind als Nachricht an ihre Linie
überführt (nie in ein fremdes Handover geschrieben).

## GitHub-Purge der PII-Alt-Commits (härtester undatierter Punkt)

- Geteilter Zustand: `docs/zustand/external-state.md`. Neu gemessen 2026-09-16:
  `pii-exposure` run 35081360127 @ `2187c30c` → 45 (Datei, Ref)-Kombinationen aus
  zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar (Vortag 44) —
  die GitHub-GC hat die Objekte nicht entfernt. Postfach (2026-09-16 10:53Z):
  GitHub-Support-Bestätigung #4761801 empfangen 2026-09-15 20:12Z; keine
  GC-Bestätigung, keine `privacy@github.com`-Antwort. (Schritt: Postfach; bleibt
  die GC-Bestätigung aus, GitHub auf #4761801 nachfassen — Operator.)

## Consent-Akte (Operator — per-Akt-Consent)

Forschung-Folge 31 (post, 2026-09-16) bündelt die Operator-gebundenen Quellen; hierher gefaltet.

**A Credentials/Token (gegen `.secrets.local` gemessen 2026-09-16):**
- **EDL `client_id` — fehlt.** Ein App bei `urs.earthdata.nasa.gov` (`/home` →
  Profil → Applications → Create New Application; Redirect `https://localhost`)
  bedient CDDIS IONEX, GIC/INTERMAGNET und GES-DISC. Danach `EARTHDATA_CLIENT_ID` /
  `EARTHDATA_CLIENT_SECRET` in `.secrets.local`; Bau-Punkt: der leere Zweig
  `S3CredentialRoute::OAuth => None` (`src/archivar/range.rs:283`, `:392`, `:510`;
  Buckets `gesdisc*`, `goldsmr5`, `goldsmr2`).
- **Fink** `https://doc.lsst.fink-broker.org/` (200) — `fink_client_register`, Zugangsdaten
  vom Team; **ANTARES** `https://antares.noirlab.edu/` (200) — Registrierung + Stream-Request
  → Operator.
- **EPA AQS** API-Key `https://aqs.epa.gov/aqsweb/documents/data_api.html` (200) — Mail an
  `aqs@epa.gov` (anon-Fallback AirData) → Operator.
- **Schon vorhanden** (kein Operator mehr): `TNS_API_KEY`/`TNS_UA`, `OCEANNETWORKS_TOKEN`
  (ONC), `RUBIN_USER`/`RUBIN_PASS` → nur Verdrahtung (Bau).

**B Konto/Agreement (URLs verifiziert 2026-09-16):**
- WWLLN `https://wwlln.net/` (200) — Zugang auf der Seite/Kontakt anfragen (akademisch,
  nicht kommerziell).
- CTBTO vDEC `https://www.ctbto.org/resources/for-researchers-experts/vdec/request-for-data`
  (403, Cloudflare) — Webform + Research Proposal (zero-cost contract).
- CTAO Science Portal `https://www.ctao.org/emission-to-discovery/data-and-computing/`
  (200) — proposal-getrieben; User-Registry direkt blockiert.
  (Schritt: je `/consent`, dann Registrierung.)

**C Anfragen an Dritte (per-Akt-Consent):**
- limadou-PI-Freigabe (Sotgiu, ASI SSDC): Account gültig, CAS-Login 200,
  „Permission Denied" — PI-Freigabe offen. Entwurf bereit
  `state/mail/limadou-pi-nachfassen.md` (gitignored). (Schritt: `/consent`, dann
  `smail --to alessandro.sotgiu@roma2.infn.it --subject "Re: CSES-Limadou L2
  data access request" --body state/mail/limadou-pi-nachfassen.md --send`.)
- Voyager/Mariner/Viking (NSSDC-SDDPT) + Cassini/Juno (JPL-NAV): Vorlagen liegen in
  `state/mail/auftrag-sonden-rohdaten-anfragen.md` + `auftrag-flyby-doppler-rohdaten.md`
  (gitignored). Das getrackte Gegenstück `docs/auftrag/auftrag-sonden-rohdaten-anfrage.md`
  fehlt im Baum (`docs/auftrag/` trägt nur `archiv/`); redigierter Auftrag neu anlegen.
  (Schritt: Auftrag schreiben, dann `/consent` + Versand je Adresse.)
- BiSON-Team (`bison@contacts.bham.ac.uk`): Entwurf `state/mail/bison-team-anfrage.md`
  (gitignored). (Schritt: `/consent`, dann `smail … --send`.)
- DEMETER/CDPP: `https://cdpp.irap.omp.eu/` (200) — Order über REGARDS (SIPAD abgelöst),
  kein Mail-Kanal. Zhangheng-1/CSES: SSDC-Kontoformular
  `https://tools.ssdc.asi.it/UserManager/requestUser.jsp` (200), Portal
  `https://limadou.ssdc.asi.it/` (200). (Schritt: `/consent`, dann Formular/Order.)

**D Route-/Exit-Wort (gemessen 2026-09-16, `archive_search --verdict`):**
- **Kein freier Proton-Exit** für `.com`/`.edu`/`.org` (`bin/proton-wg.sh suggest` →
  kein Treffer): die Proton-Route ist ohne paid Exit gegenstandslos; die Host-Routen
  unten fallen damit auf Browser-Bridge (Operator-Profil) oder absent.
- **Hinson 1997** (`10.1029/97GL01608`): 403 = Cloudflare-Bot-Shield, keine Paywall;
  headless `--playwright` 403 (Interstitial cleart nicht). Die Zahlen sind aus
  offenen PDS-Daten re-derivierbar (`GO-J-RSS-1-ODF-V1.0`, `galileo_odf_compiler`)
  → Ernte/Bau, kein Operator nötig; nur der Volltext bräuchte Bridge/Proton.
  (Schritt: PDS-Re-Derivation als Ernte-Atom.)
- **TNF** (`pdssbn.astro.umd.edu/…/lunocc2012.tnf`): direkter Host HTTP 0, Wayback
  503 → Proton für den Byte-Nachweis. (Schritt: Operatorwort, dann `bin/proton-wg.sh`.)
- **Haw 1997** (`10.2514/2.3240`): Paywall → ILL/Proton. (Schritt: Operatorwort.)
- **HAWC**: keine Proton-Route — CA-Bundle-Route (`OMEGAFLOW_CA_BUNDLE`, `curl
  --cacert` → 200, 2026-09-13); heute direkt 000, Wayback 503. Register/Bau-Punkt,
  nicht Operator.
- **LIS/OTD**: Route 200 (`lightning.nsstc.nasa.gov/data/`); Blocker ist das
  GHRC-Konto → E.

**E Konsumenten (Tor 1) — Bau-Reihenfolge, kein Operator:**
- ERI/VLASS/CORS-Konsument, LASzip-Decoder (Konsument fehlt), JVO skynode-TAP,
  Babamul, GHRC-DAAC, WFAU VSA/WSA. (Schritt: als Bau-Punkt an die Bau-Linie.)
- Parser-Magic Gaps 1 (`Frame::Data` in `types.rs`), 8 (`flush!()`-Gate),
  12 (Category/Group-Vererbung). (Schritt: an die Bau-Linie.)

## Ernte-Folge 42/43 — Register-/Ernte-Rest (gefaltet 2026-09-16)

- Ernte-Nachlauf: MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings) —
  Register/Ernte-Pflicht. (Schritt: `sources.φ` + CDN.)
- Register-Digest-Rest: Fink/ALeRCE (Proxy-Pfad; s. A), TDAT/FITS-Konsument, Akteure,
  Daten-Holdings (erste Messung), Orphan-Verdicts. (Schritt: je Punkt messen.)

## adoption — Drei-Mail-Block: Adressen gemessen, Send beim Operator

- Entwürfe in `state/mail/adoption-mails.md` (gitignored), gepinnt auf Sha
  `50db1ed`; der gepinnte Papier-Link verifiziert 2026-09-16 (HTTP 200). Die drei
  Adressen sind gemessen und im Entwurf hinterlegt, je mit Quell-URL — keine
  geraten. Reg 4 (Amplitude) bleibt `pending` mit gemessenem ~5-Hz-Anker
  (Station 14, 1988). Operator-Entscheid 2026-09-16: noch nicht senden. (Schritt:
  Adressen bestätigen + senden — Operator; Consent `/consent`.)

## Warten auf Rückmeldung (extern gebunden)

- Rubin RSP (Shaughnessy, SLAC): 2026-09-15 21:11 in die Einzelprüfung genommen,
  Antwort offen. NSE/Haug (Keimer), CSES-Limadou (Sotgiu, ASI SSDC): Antwort
  offen — Postfach-Zeile in `docs/zustand/external-state.md`. (Schritt: Postfach
  bei Fälligkeit.)

## Termine (Wiedervorlage)

- 2026-09-22 — AllWISE-Coverage-Verifikation (CDN-Asset `allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen
  (`papers/flyby-path-2-preregistration.md`).
- 2026-09-18 — Lasair (Wiedervorlage).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster); Feld-Zustand füllen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
