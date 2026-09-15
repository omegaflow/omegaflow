<!--
  title: Handover — Ernte-Folge 29 (Stand 2026-09-15)
  session: Ernte-Folge 29
  class: handover
  date: 2026-09-15
  sha256: a2752d36cf6d3ca3879710165d6d0a86abc10b1914921736d03b89a4dee57b97
  status: live
-->
# Handover — Ernte-Folge 29 (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage.

## Tor 1 — Compiler stehen, Konsumenten fehlen

- ERI/VLASS/CORS-Compiler gebaut; ERI trägt die Index-Route. `ERI1` fehlt noch
  in `zeuge.rs::magic_identity` + Feld-Leser. (Schritt: magic_identity +
  Feld-Leser, wenn ein Konsument benannt ist — Operator.)
- GK2A `GKA1`, GOES `GAB1`, GDP `GDPT`: kein Leser in `src/archivar/`.
  (Schritt: Leser je Magic, wenn ein Konsument benannt ist.)
- LASzip-Decoder steht; Konsument fehlt. (Schritt: Konsument benennen — Operator.)
- FITS `P`-Format dekodiert; Rice fehlt. (Schritt: Rice nur bei Pixel-Konsument.)
- JVO skynode-TAP (akari/irsf/nobeyama/saga): anonym nur unter
  `/skynode/do/tap/<node>/sync` (Tor 1). (Schritt: Konsument/Proxy benennen.)
- WFAU VSA/WSA `superseded-by-integrated`; SuperCOSMOS-pm-Kandidat
  (Wiedervorlage 2026-12-02 schweigt).
- Babamul: Zugang steht — `Authorization: Bearer <BABAMUL_API_TOKEN>` an
  `/api/babamul/objects?object_id=…` bzw.
  `/api/babamul/surveys/{survey}/alerts?start_jd=&end_jd=` (≤ 1 JD) HTTP 200
  (gemessen 2026-09-15). (Schritt: Konsument benennen — Operator.)

## Parser-Magic — 4 Gaps benannt (Konsument fehlt)

- Gap 1 + Gap 8 (Auto-frame aus `lat_key`/`lon_key`; `map` als Frame-Indikator):
  brauchen `Frame::Data` in `types.rs` + das `flush!()`-Gate in `parse.rs`; ein
  neues Enum-Arm bricht 9 erschöpfende `match frame`-Arme (`membrane.rs`,
  `main_flow.rs`, `matrix.rs`, `port.rs`), und `Data` hat keinen Körper-Anker
  (`frame_body_name`, `anchor()` verwerfen die Kanäle). (Schritt: Konsument für
  per-Zeile-Datenkoordinaten benennen — Operator.)
- Gap 6 (`window`/`from`/`until`): Schema-Feld ohne Leser; der Extract-Pfad hat
  keinen Zeilen-Epoch-Filter. (Schritt: epoch-Filter in der Extract-Schleife
  benennen, dann das Feld.)
- Gap 12 (Category/Group-Vererbung): kein `group`-Direktiv und kein
  Gruppen-Schema in `sources.φ`. (Schritt: Gruppen-Schema entwerfen oder als
  Curation streichen.)

## Register-Hygiene — giveup_scan (fremd-blockiert)

- 154 klassenlose `decline `-Zeilen (trailing space) in `phi/declined_sources.φ`;
  die Datei ist fremd-modifiziert. (Schritt: nach ruhigem Baum jeder Zeile die
  Klasse aus note/grind zuweisen, dann `giveup_scan --summary` gegenprüfen.)

## CDN-Manifestation (gated — nach Push + Consent)

- Dispatch je Quelle: celestrak-eop, goes, himawari, gk2a, uscrn, cosmic, maxi,
  isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass, eri;
  + `source-census.yml`. (Schritt: `gh workflow run <wf>` nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst. (Schritt: fremde Staging
  auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`; GAVO-Konto
  nicht nötig (banded sync umgeht die ~20000-Kappung). (Schritt:
  `gh workflow run gaia-xp-full-cdn.yml` nach Push + Consent.)

## Ernte-Nachlauf

- GES-DISC OAuth: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`, keinen
  `client_id`. (Schritt: client_id beim Operator, dann `S3CredentialRoute::OAuth`
  in `range.rs` verdrahten.)
- MPC-Shard UnnObs-Dispatch + shard-url (Operator); Fink-Konus dead.
- GHRC-DAAC + ARPANSA-UV + NOAA CDO/GEDI/NSIDC/PODAAC-SWOT entblockt →
  Ernte/Compiler + Konsument offen.
- VLASS `cirada.VLASS_Source`: gemessen 2026-09-15 — GET wie POST auf
  `ws-uv.canfar.net/youcat/sync` liefern 400 (nicht 200); POST redirectet auf
  `/youcat/sync/<id>/run`. (Schritt: Auth-/Sync-Schema klären — Operator.)
- HAWC: `data.hawc-observatory.org` sendet nur das Leaf-Zert (Let's Encrypt
  `YR1`, kein Intermediate) → `curl` verify 60; `-k` → 200. (Schritt: TLS-Kette
  fixen oder Ausnahme benennen — Operator.)
- LHAASO: `www.lhaaso.ac.cn` DNS tot; `english.ihep.cas.cn/lhaaso/` HTTP 200
  (DigiCert RapidSSL). (Schritt: IHEP-Pfad als Registrierungsroute benennen.)
- Fink: `api.lsst.fink-portal.org` GET/POST 200; `api.fink-portal.org` Timeout.
  (Schritt: LSST-Host als Route festhalten.)
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

## Zugangsanfragen — Ernte-Verdikt (an die Entscheid-Linie)

Gemessen 2026-09-15 (Registerzeilen + Hosts). Verdikt je versandter Anfrage:
- entbehrlich/redundant/geschlossen (streichen): NOIRLab (`ls_dr10.tractor`
  anonym, `sources.φ:8498`; nur die Speisekammer 2026-12-02 bleibt), JSOC (AIA
  anonym via CDN, `sources.φ:8064+`), LPF (anonyme DRS-FITS-Route,
  `blocked_sources.φ:36–38`, 2026-09-14 gemessen), GAVO (kein Konto nötig; banded
  sync umgeht die Kappung), BiSON (kein phi-Eintrag nötig, anonyme Routen 200;
  offen bleibt nur die BiSON-Tabelle), IGETS (`igets.bin` in `sources.φ:8039–8043`;
  offen bleibt der Reader), TOAR (Verdikt 2026-08-19, `dead_sources.φ`: WOUDC
  liefert dieselben WMO-Daten, anonym nutzbar).
- hält (Antwort/Kontakt offen): CSES-Limadou (kein Widerspruch: CSES-SPA
  `leos.ac.cn` ≠ CSES-Limadou L2 `ASI SSDC`; L2-Zugang lokal in `.secrets.local`),
  NSE/Haug (Rohdaten descoped, kein Deposit; Keimer-Anfrage hält), Rubin RSP (RSP
  via Fink-LSST anonym 200; Antwort an Shaughnessy offen).
(Schritt: Entscheid-Linie streicht die 7 entbehrlichen Wartepunkte; Ernte-Linie
trägt BiSON-Tabelle + IGETS-Reader weiter.)

## Register-Digest-Überführung (Rest; gegen eigenen Stand geprüft)

- LAIC-Bausteine gemessen: MiniSEED (`tools/measure/src/miniseed.rs`), DEMETER
  order-flow (`demeter_harvest.rs`) + Parser (`demeter.rs`, `demeter_compiler.rs`),
  COSMIC netCDF (`cosmic_ro_compiler.rs`), TEC-GIM LZW (`lzw.rs`, `ionex.rs`),
  Ereignisraten (`laic_probe.rs`) vorhanden; **CSES-SPA-Login fehlt** (kein
  `*cses*` im Baum). (Schritt: CSES-SPA-Zugang benennen — Operator.)
- QuakeML-1.2-Parser, BGC `/bgcargoplus`, vDEC-Roh, ONC-Token, TNS-anonym,
  JUNO-Release, TA-Vollkatalog. (Schritt: `survey-2026-09-13-weberin-quellen.md` —
  je Quelle Route + Reader.)
- Fink/ALeRCE-Persistenz, Lasair (Wiedervorlage 2026-09-18 schweigt), Hinson 1997.
  (Schritt: erste Messung je Host — curl.)
- Offen aus den Werkzeug-Lücken: S3-OAuth (`client_id`), TDAT/FITS-Konsument.
- INTERMAGNET / IONEX-GIM / direkter GIC ausstehend. (Schritt:
  `concepts/der-kausalpfeil.md` — Kanal je Quelle festlegen.)
- Akteure: Flotten-Scatter+CMT, Stationsterm +5,69 s, Tonga-Luftgang, W-Phase M9,
  ETOPO1-CDN (395 MB), MiniSEED-Konsolidierung, Erdmoden, DART, Grundwasser,
  Gravimeter-SFTP, Radon. (Schritt: je Akteur Quelle + Route.)
- Daten-Holdings: `abk_dbdt_1h_*`, kegel-log, GIC/corona; new_horizons/voyager1/2
  976-B-Platzhalter; ~50-G-Backup-Ziel-Layout. (Schritt: erste Messung —
  Holdings lesen, Herkunft je Stück benennen.)
- Orphan-Verdicts: 55 undokumentierte `stale_pending`; Step-4 CI-Dedupe neu
  scopen; 14 undokumentierte `repo_tag`. (Schritt:
  `survey-2026-09-03-orphan-verdicts.md` — je Verdict dokumentieren/stretchen.)
- Korpus-Rest: thread-matrix-Lücken (Gravimeter, Infraschall, Hydroakustik,
  seismische Worldlines, HF-Radar, GIC, Blitz, BGC-Argo, VHE-Teleskope);
  WWLLN-netcdf/BPA-GIC; BiSON-Tabelle; tmp-opencode-Scratch/SuperDARN-FITACF/
  NOAA-NRS; mirror-research ~2300-Quellen-Migration; GLO-30 DEM. (Schritt: je
  Treffer Quelle + Route, sonst streichen.)

## Baum

- Fremde Session weiter live (breite ` M`/`MM`/`D `-Menge; committet `7c66bff`).
  Fremde Staging nicht angefasst.
- Fremder Compile-Fehler transient: `tools/measure/src/bin/pcmci_class_benchmark.rs`
  (`TeNull::XShift` nicht abgedeckt) blockiert `cargo check --workspace` —
  fremde TE-Arbeit.
- Ernte-Linie: `folge25`–`folge28` liegen in `docs/handover/archiv/`; live ist
  `folge29`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
