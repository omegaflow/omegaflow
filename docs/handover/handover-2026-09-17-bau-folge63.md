<!--
  title: Handover — Bau-Folge 63 (Stand 2026-09-17)
  session: Bau-Folge 63
  class: handover
  date: 2026-09-17
  sha256: efaac2aa140c108ff476aaed31e4ad23b5839bb3db8a8dbb0aa3fb22f7c5a912
  status: live
-->
# Handover — Bau-Folge 63 (2026-09-17)

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
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (2026-09-17, HEAD e197fbec)

- Postfach: kein neuer externer Eingang seit 2026-09-16 (Sotgiu-Antwort); die
  offenen Anfragen (Rubin-Review, NSE/Haug, die fünf Sonden) unverändert —
  `state/mail/mail_ledger.φ` gelesen, `docs/handover/post.md` sauber bis auf
  die fremden Zeilen.
- CI am HEAD `e197fbec`: `ci-check` 35230744407 pending, `paper-check` 35230433910
  queued, `pii-exposure` 35230489744 queued; davor `paper-check` 35228278279
  failure. Aktive CDN-Läufe: demeter, ned, ps1, physionet, gaia-xp-full,
  planetary-odf, health-check, allwise, maven-tnf (Watchdog-Snapshot
  2026-09-17T14:57 +02:00).
- Zustand-Ledger: `docs/zustand/external-state.md` trägt fremde uncommittete
  Zeilen (Postfach/PII/CI @ bc9d6a0b) — nicht überschrieben. (Schritt: nach dem
  Commit des fremden Deltas die CI-Zeile auf `e197fbec` fortschreiben.)

## Fast-ttl-Mirror je Klasse (härtester undatierter Punkt)

Der `300`-Boden ist gefallen; `fetch_one` dient jetzt den realen CDN-Body als
Fallback, wenn der Live-Pfad void ist (`src/archivar/fetch.rs:998`, Council-Verdikt
C′) — die 27 ttl<300-Quellen voiden damit nicht mehr, sondern lesen den echten
Wert (0 honored nur, wo das CDN-Asset wirklich fehlt). Offen bleibt der Mirror
selbst: die 27 Quellen (vires.services 7 historische HAPI-Fenster,
services.swpc.noaa.gov 8, earthquake.usgs.gov 3, api.wolfx.jp, api.p2pquake.net,
api.vedur.is, api.geonet.org.nz, ceic.ac.cn,
seismic-api.science.unimelb.edu.au, www.seismicportal.eu) haben noch keinen
eigenen Refresh-Workflow; der CDN-Refresh im `omegaflow/sources`-Repo ist
`refresh.yml`, Cron `17 */6 * * *` (6-stündlich, kein 5-min-Takt — gemessen).
(Schritt: je Klasse einen Mirror-Workflow bauen; Cadence aus der Physik — die
statischen vires-Fenster einmalig, die Beben/SWPC-Klassen stündlich.)

## Werkzeug-Reibung (gemessen aus opencode.db)

(1) Header-sha256 manuell 16–17× → `omega_sh sha <file>`; (2) Commit-Abschluss
13× in vier Git-Aufrufen → `git_safety --close`; (3) `cargo check | tail -n 2` 7×
→ `check`-Zusammenfassung; (4) `echo`/`printf` in `explore`/`general`/
`research-max`/`council` freigeben, `ci_manage list|view` ins Plan-Profil.
(Schritt: die Werkzeuge/Profil-Freigaben in `tools/utils` + `opencode.json` bauen;
danach die Linien-Prompts um `omega_sh sha`/`git_safety --close` ergänzen.)

## Folge-Atom: Asset-Alter in die Sample-ttl

`fetch_one` nennt das gemessene Last-Modified-Alter des CDN-Bodys nicht
(`src/archivar/fetch.rs:955`). Council-Befund: ttl dient zwei Zwecken — Fetch-
Frische-Erwartung und Oszillator-τ; für die statische Klasse ist nur der zweite
real. (Schritt: das Alter aus `cdn_last_modified_age` nur für die Live-Klasse in
die Sample-ttl tragen; die statischen vires-Fenster behalten ihre deklarierte ttl.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
