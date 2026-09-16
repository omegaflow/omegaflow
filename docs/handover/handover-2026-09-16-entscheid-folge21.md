<!--
  title: Handover — Entscheid-Folge 21 (Stand 2026-09-16)
  session: Entscheid-Folge 21
  class: handover
  date: 2026-09-16
  sha256: adc2fd59f59ad4e8b4f80820a638a7b3c2fa79599ed82eac223c3a976ccd4ce0
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

**A Register-offene Registrierungen (browser-gemessen 2026-09-16):**
- **limadou/CSES** — PI-Freigabe (`phi/pipeline/ledger.φ:26–28`): SSDC-Konto lebt,
  CAS-Login 200; nur die Freigabe fehlte. Nachfassen **gesendet** 2026-09-16
  (Resend id `0ebfe257-d059-45d3-9468-448c6dffd32e`) → „Warten auf Rückmeldung".
  Damit ist **keine** register-offene Registrierung mehr offen.

**Aufgelöst (browser-gemessen, kein Operator):**
- **CDDIS IONEX**: der vorhandene `EARTHDATA_EDL_TOKEN` öffnet das Verzeichnis
  (`https://cddis.nasa.gov/archive/gnss/products/ionex/2026/` → HTTP 200, 96 KB) —
  der `blocked key-needed`-Eintrag ist stale.
- **GES-DISC**: kein selbst angelegter App-Key — der OAuth-Flow nutzt GES-DISCs eigene
  `client_id` (`e2WVk8Pw6weeLUKZYOxvTQ`); die EULA-Autorisierung ist erledigt, die
  Credentials kamen zurück (1 h TTL). Bau implementiert den Authorization-Code-Flow
  (`S3CredentialRoute::OAuth`, `range.rs:283`); das EDL-Konto ist `Application Creator: False`.
- **INTERMAGNET/GIC**: registriert via HAPI (`sources.φ:1600`, `:4348–4365`) — kein `client_id`.

**Nicht Operator (Register-Verdikt, gemessen 2026-09-16):**
- `declined`: CTBTO (`declined_sources.φ:2979`, Redistribution verboten), CTA (`:177`),
  Fink-Schema (`:3449`), **WWLLN** (`blocked_sources.φ:40–43` — UW-copyright, „nominal
  cost": kostenpflichtig; **keine kostenpflichtigen Dienste** — Operator-Wort 2026-09-16).
- `pending`/`parser-def` (Bau/Ernte): ANTARES (`declined_sources.φ:236` → lebt als
  pending-Registrierung), Babamul, Voyager RSS, GHRC DAAC, drs-fits, ARPANSA, fugin, IA2,
  IGRA-2.
- **EPA AQS**: Bulk-Zip **keylos** (`phi/pipeline/ledger.φ:38–40`, parser-gap
  „Arithmetic Mean") — kein API-Key nötig.
- **Registriert** (kein Operator): TNS (`sources.φ:1269`), ONC (`sources.φ:792`), Rubin,
  IGETS, Lasair, SSDC-Konto.

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
  Antwort offen. NSE/Haug (Keimer): Antwort offen. CSES-Limadou (Sotgiu, ASI SSDC):
  Nachfassen gesendet 2026-09-16, Antwort offen — Postfach-Zeile in
  `docs/zustand/external-state.md`. (Schritt: Postfach bei Fälligkeit.)

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
