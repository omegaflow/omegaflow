<!--
  title: Handover — Forschung-Folge 64 (Stand 2026-09-17)
  session: Forschung-Folge 64
  class: handover
  date: 2026-09-17
  sha256: c8347f6ff36dfcb2bd5a17f96b9bfdcc8f2b332c2732768a7236c8f61fd8cfa2
  status: live
-->
# Handover — Forschung-Folge 64 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich; so viele offene Punkte wie möglich pro Session (die
alte „ein schwerer und fünf leichte"-Klausel ist gestrichen, `post.md` 2026-09-16).
Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur
der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird,
sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist
(Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Ein `wartend`-Punkt ist kein Auswahlpunkt — er trägt seinen Trigger und wird
nicht als Aktion verkleidet.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — neue Eingänge gemessen 2026-09-17 (Session-Read
  `state/mail/mail_ledger.φ`): **Thomas Keller (TRISP/MPI-FKF) antwortet 09:29 —
  er sendet die NSE-I(q,t)-Daten „in a few days"** (der NSE/Haug-Punkt unten ist
  damit in der Wartephase, Trigger = Dateieingang); GitHub-Support-Ticket
  **#4761801** aktualisiert (17.09. 16:27Z, GC weiter offen); daneben
  beiläufig: DAHITI-Passwort-Reset, dpa-ID-Registrierung, fünf
  Cloudflare-Login-Codes 07:06–07:11Z, GitHub-PAT „omegaflow-ci-write" angelegt,
  Rubin-Forum-Zusammenfassung. Der Zustand-Eintrag `docs/zustand/external-state.md:20`
  (Postfach) ist damit fällig (neuer Eingang) — die Datei trägt **fremde
  uncommittete Änderungen**, der Eintrag bleibt der besitzenden Linie überlassen
  (nicht angefasst).
- **CI-Status am HEAD `e0d6d0b9`** — Watchdog-Snapshot 18:09:50Z (vor dieser
  Session): §4-Läufe `35244044555` + `35244087174` `pioneer-cell-census`
  **success**, `mariner-occlt-cdn` `35244811694` success, `swot-cdn`
  `35246189331` queued, `ci-check` `35246844655` pending, `health-check`
  `35245084696` pending; `pii-exposure` `35230489744` failure (1×),
  `paper-check` `35228278279`/`35224760511` failure (1×), CDN-Welle aktiv
  (demeter/physionet/gaia-xp/allwise). Der Zustand-Eintrag
  `docs/zustand/external-state.md:22` (CI-Status @`bc9d6a0b`) ist bei
  HEAD-Wechsel fällig — ebenfalls fremd-modifiziert, nicht angefasst.
- **HEAD `e0d6d0b9`** — `origin/main` == HEAD (keine unpushed Commits,
  Fast-Forward frei). `git_safety --snapshot` = `refs/safety/1789662586`.

## §4 — fsky-Census: **geschlossen** (Paper v10)

- Beide Läufe `35244044555`/`35244087174` **success**; das Artefakt
  `pioneer-cell-census.txt` gelesen (1 071 540 Records, 3 Empfänger). Das
  Ergebnis steht in `docs/paper/twenty-second-band-ground-chain.md:188` ff.
  (Header v10, sha256 aktualisiert):
  - **Verdikt: nicht upstream — §8 (light-time) ist der erste benannte
    Kandidat.** Der resid-Census (r[8]) trägt die Papier-Signatur (1988
    mode3-lt10 46,581 / 44,119 / 50,920), der fsky-Census (r[1], rohe
    Sky-Frequenz, identische Zellen) trägt einen **anderen** stations-fixen
    Komplex (1988 46,585 / 52,816 / 45,100). Zwei Empfänger (rx43, rx63) bewegen
    ihren Dominanten zwischen den Feldern (+8,7 / −5,8 mHz auf der 1988-Zelle;
    +10,1 / −10,9 auf s1.000), rx14 nicht (Δ 0,002–0,004 mHz).
  - **50,000-mHz-Beobachtung:** 218 Census-Zeilen peaken exakt bei 50,000 mHz,
    214 davon in den groben Klassen 10–30/60 (beide Felder), 0 in den resid-feinen
    Klassen, 4 in fsky-feinen Zellen (alle st63-lt10-1992).
- **Offene Restfragen** (nächster Schritt: Probe `tools/measure/src/bin/
  pioneer10_cell_census_probe.rs` um ein Floor-Verhältnis je Zelle ergänzen,
  CI-Lauf):
  - Ist die 50,000-mHz-Linie der groben Klassen ein 20-s-Signal oder ein
    Raster-/Alias-Artefakt? Die Probe druckt keine Signifikanz.
  - Warum trägt st14 seinen mode3-Peak bereits in fsky, st43/st63 nicht?
  - Der fsky-eigene stations-fixe Komplex (46,579/57,842/45,094) — eigene
    Herkunftsfrage, von der Handover-Regel nicht adressiert.

## Docs-Pendings — gemessen 2026-09-17 (Taucher, alle `direct`)

- **Ulysses / BepiColombo / LRO — Zugang `open` (anonym), Registrierung fehlt.**
  - Ulysses: `https://pds-ppi.igpp.ucla.edu/data/ULY-J-SCE-1-TDF-V1.0/` (HTTP 200,
    `.TDF` ~30 MB, PDS3) + SPICE `naif.jpl.nasa.gov/pub/naif/ULYSSES/kernels/`.
  - LRO: `http://pds-geosciences.wustl.edu/lro/lro-l-rss-1-tracking-v1/lrors_0001/`
    (HTTP 200, `.trk` TRK-2-34 ~4,6 KB/Record; Sniff sha256
    `3787b24bc11c313b7634effb68bc4ca4b07bca79255bb3dd80463a9f42dae5d3`) + SPICE.
  - BepiColombo: `https://archives.esac.esa.int/psa/ftp/BepiColombo/` (HTTP 200,
    PDS4) + SPICE; Radio-Science-Bundle `bc_mpo_more/` trägt **noch kein `data/`**
    (Cruise) → `pending` mit Ort, kein Verdikt.
  - **Nächster Schritt:** Grind-Draft + `sources.φ`-Block je Sonde (ODF/TRK-Parser
    existiert, `odf.rs` dekodiert seit 2026-09-17 alle 18 TRK-2-34-Codes). Der
    `blocked parser-def odf`-Eintrag (`blocked_sources.φ:81–83`) ist damit **stale**
    — im Dispositions-Atom prüfen.
- **GOES-16 ABI — `open` (anonymes S3).** `noaa-goes16.s3.amazonaws.com` (HTTP 200,
  netCDF `.nc` ~28 MB) ist das Geschwister-Bucket des schon registrierten GOES-19
  (`goes_abi_rad.bin`, `sources.φ:1070`). NOAA CLASS (`blocked account`) und GSICS
  (Kalibrierung, kein Datenpfad) sind nicht nötig. **Nächster Schritt:** Compiler
  gegen das GOES-16-Bucket richten + `sources.φ`-Block (kein neuer Compiler).
- **WWLLN — Thunder-Hour `open`, Lizenz-Gate.** `www.wwlln.net/climate/th_clim/data/`
  und `.../th_yr/data/` (HTTP 200, netCDF `.nc.zip`, 2005–2025, 0,05°); Spiegel
  Zenodo `records/10725446` (CC BY-SA 4.0, NetCDF-4). Quell-Lizenz „research
  (non-commercial) use" — die **CDN-Manifestation braucht die Operator-Entscheidung**
  (Zenodo-Spiegel manifestierbar). Roh-/Stations-Feed `blocked account` /
  `decline redistribution`. **Nächster Schritt:** netCDF-Compiler-Gerüst
  (`src/archivar/netcdf.rs`) + `sources.φ`-Block; Lizenz-Klärung vor CDN.
- **MAVEN-TNF-Shards** — offen: TRK-2-34-SFDU-TNF-Record-Parser + PDS4-XML-Label-Leser
  in `extract.rs` (`pds-ppi.igpp.ucla.edu/.../tnf/`). (Schritt: Parser-Gap-Auftrag.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

## NSE/Haug — Route offen, Rohdaten zugesagt (`wartend`)

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days" (Mail
  2026-09-17 09:29Z gemessen). **Trigger = Dateieingang.** (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.;
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- §4-fsky-Deutung an `research-max` (hartes Atom: Zuordnung der Reduktionsstufe
  im ODP/light-time-Kontext), Docs-Pendings-Zugangsmessung an `grind-pro`
  (Urteil: Quellen-Zustandsklasse). Beide lieferten vollständige, gemessene
  Ergebnisse; kein flash/pro-Doppellauf — die Klasse „ODP-Stufen-Zuordnung" hat
  noch keinen registrierten Sieger. Extraktion (Artefakt-Download + Lesen) lief
  in der Session selbst.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/paper/twenty-second-band-ground-chain.md`,
  `docs/handover/handover-2026-09-17-forschung-folge64.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge63.md`). Fremde uncommittete Arbeit (nicht
  anfassen): `docs/zustand/external-state.md`, `docs/handover/post.md`,
  `phi/sources.φ`, `src/archivar/extract.rs`, `src/archivar/main_flow.rs`,
  `src/archivar/tests.rs`, die vier `.opencode/command/*.md`, die gestagten
  Renames `handover-2026-09-16-*` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
