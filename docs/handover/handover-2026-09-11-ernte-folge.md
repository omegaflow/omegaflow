<!--
  title: Handover — Ernte & Register (Folge, Stand 2026-09-11)
  session: Ernte-Folge
  class: handover
  date: 2026-09-11
  sha256: 849c6ae24b8347077d7d41d81357fb87d66619badbcfc0e20fb40dfc7db43e5c
  status: live
-->
# Handover — Ernte & Register (Folge, 2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Ernte (Harvest)

- ESO-TAP — Ernte + Duty (kein Register-Eintrag).
- Discovery-Download-Workflow (Korpora).
- Pioneer-10-ATDF-Restbestand — 238 Dateien / 77 Tage ernten (SPDF
  `ATDF_Data-Files_CMarkwardt_Readable`).
- IGETS/GGP — `igets.bin` absent am CDN (gemessen 2026-09-11: Release
  `igetsftp.gfz.de` nicht angelegt; run 34571077129). CI-Status prüfen,
  redispatch.
- Hi-net/NIED WIN32-Ernte — Zugang entsperrt (Login `omegaflow`, Ablauf
  2027-03-31, gemessen 2026-09-11): `POST /auth/` → Session, Stationen
  registrieren über `select_check.cgi` → `select_confirm.php`, Download über
  `cont_request.php?org1=&org2=&year=&month=&day=&hour=&min=&span=&arc=&size=&LANG=en&volc=&rn=`
  → WIN32 `.cnt`; Status `cont_status.php`. Compiler + WIN32-Reader offen.
  Vor-2004-Bestand ist ein Antrag (`form past` → `POST request_check.php`).

## CDN-Manifestation (Duty)

- jup365 — Ursache gemessen 2026-09-11: Memory (der Flatten hält alle SPK-Kernel
  via `std::fs::read` im RAM, ~6,4 GB Subtotal > 7 GB Runner → OOM; Flatten
  „canceled" bei ~9m39s). Der Per-System-Loop ist gemessen unsicher
  (`flatten_targets` kennt kein System-Filter; der Mond verliert die Libration —
  `moon_pa_de440`/`moon_de440` hängen nur an `select_system("planets")`).
  Einziger Pfad: der DafFile-mmap-Fix (`daf.rs:92` pread statt `fs::read`;
  `from_data` bleibt Byte-Paritäts-Referenz) als nächstes Atom mit Toren. Offen:
  der Dispatch mit Wächter (`gh workflow run kernel-flatten.yml`; Erfolg:
  bodies-Job grün + jup365-Release am CDN).
- Dispatch-Verifikation (gemessen 2026-09-11): gelandet — supermag (20
  Stationen), noaa-ghcn (≥1000), noaa-gsod (381), noaa-isd (167),
  copernicus-cuon. Absent am CDN: `igets.bin`, noaa-dcdb, noaa-keo-papa,
  copernicus-icoads (CI-Status ungemessen — in-flight oder fehlgeschlagen).

## TAP-Klassifikation

- 39 Endpoints disponiert (2026-09-11): 13 accept → sources.φ, 18 parser-def
  votable → blocked_sources.φ, 1 decline → dead_sources.φ, 7 ausstehend
  (Backend down am Probe-Zeitpunkt: dachs.fai.kz, vo.lmd.jussieu.fr,
  tap.roe.ac.uk/{wsa,vsa,osa,ssa}, pithia.cbk.waw.pl). 0 dead — down ist nicht
  tot; die Vollwelle wird ein Wartungs-Inventar, kein Friedhof.
- Die drei Arme wohnen in vo-tap (das öffentliche Crate, eigenes Repo, gepusht):
  (1) DaCHS-Zweig — zweiter Leseweg für `columns`/`data` (6 der 13 Accept, live
  gemessen an gavo.aip.de); (2) VOTable-Zweig — nachfrage-getrieben,
  blocked_sources.φ ist Queue, nicht Schuld, kein Parser ohne Frage; (3)
  tap_schema-Selbstabfrage ersetzt das Best-Effort-Raten der Spaltennamen. Die
  Haus-Seite (unser Votap-Parser, tap_to_json extract.rs:895) bleibt schlank und
  ißt nur — vo-tap und der Haus-Parser teilen Mechanik, sind aber zwei Dinge.
- Distance-Keys (9/13 ohne plx/dist/z): Quellen-Kuratierung (welche Tabelle
  trägt das Feld), kein Client-Problem; das cmap-`continue` bleibt ehrlich.
- Die 7 Pending: fremde Zustände — warten, nicht bauen; ein Re-Probe-Takt
  (Zensus-Welle) reicht.
- Sequenz: die Zensus-Welle (HTTP-Code + Zeit, format-blind) läuft jetzt; die
  Ernte-Welle (Endpunkte → Compiler → φ) wartet auf die Arme in vo-tap.

## Parser-Pending

- noaa-ccor — Render-Pfad (MAGIC_CCOR) gebaut; offen: der sources.φ-Block —
  Quell-URL liegt in `noaa-ccor-cdn.yml` (CCOR-1 dm_science FITS-Granule im
  `noaa-nesdis-swfo-ccor-1-pds.s3.amazonaws.com`-Bucket).
- noaa-jpss — OMPS-SDR-Reader gebaut (`noaa_jpss_compiler.rs`, Magic `OMP1`);
  offen: Datenzugang (anonymes S3-GET liefert CLASS-HTML, kein HDF5) → Render-
  Verdrahtung + sources.φ-Block nach Zugang.

## Register

- Step-5-Folge — destruktiver Schnitt (verifiziert).
- docs-reference-verteilung — Bewegung + Referenz-Rewiring, Seeds → Survey-Heimat.

## Speisekammer / Nachlese

- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- dataone — terms-Seite 401 (gemessen 2026-09-11: www.dataone.org/terms → 302
  old.dataone.org/terms → HTTP 401); Lizenzstatement unlesbar, Redistribution offen.
- cadc.argus — NICHT tot (TCP offen, TLS-Reset ~19,5 s Peer-/Exit-seitig;
  r.jina.ai erreicht `/argus/sync` mit HTTP 400 = lebt). Ernte über anderen Exit
  oder Jina-Roh-Fetch offen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
