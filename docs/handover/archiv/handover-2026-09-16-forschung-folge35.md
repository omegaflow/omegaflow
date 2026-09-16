<!--
  title: Handover — Forschung-Folge 35 (2026-09-16)
  session: Forschung-Folge 35
  class: handover
  date: 2026-09-16
  sha256: 13fef6fe9a214feb42714d4b3896299e2cc7b31eb8082d52c38f76ad8340262f
  status: live
-->
# Handover — Forschung-Folge 35 (2026-09-16)

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

## Die Weberin — Schritte 3–9

- **Die Weberin — Schritte 3–9** (topozentrische Kopplung 3, Tafel-Abbildung 4,
  Survey-Footprints 5, GW-/Neutrino-/CR-Routen 6, CDN-Weg 7, Riss-Knoten 8,
  geliehener Sinn 9). (Schritt: `research-max`, `docs/concepts/die-weberin.md`.)

## Parser / Reader-Bauten

- **Parser COSMIC-2 / TEC-GIM-LZW / MiniSEED** — Reader-Bauten. (Schritt:
  `grind-pro`, Muster `geo.rs`.)

## Council

- **304-Å-Trigger falten vs. descopen + Blatt 2/3 fam/max-T-Bound.** (Schritt:
  Rat, vor jedem Blatt-Schreiben.)

## h0 / CMB — Reste (Nachrechnung vollzogen 2026-09-16)

- **CMB clik/CosmoMC-Eigenevaluation** — die Chain-Statistik ist gewogen (beide
  Register), der Likelihood-Code-Lauf selbst bleibt offen. (Schritt: Bau eines
  `clik`/CosmoMC-Atoms — oder descoped nach Messung; siehe
  `docs/paper/h0-lines-register.md` §„Named pending points" (1).)
- **PLA/Chain-Quelle registrieren** — der gemessene Route-Host
  (`pla.esac.esa.int/pla/aio/product-action?COSMOLOGY.FILE_ID=…`, IRSA-Spiegel
  `irsa.ipac.caltech.edu/data/Planck/release_3/ancillary-data/cosmoparams/`)
  trägt keinen `phi/sources.φ`-Eintrag. (Schritt: Register — Quelle + Reader.)

## Quellen-Pass

- **CHAMP/GFZ-ISDC PLPT** — PLPT-zip→Tabellen-Compiler `pending` (kein
  Harvest-Bin verifiziert); inner ASCII-Tabelle (`CH-ME-2-PLPT+…_1.dat`,
  519 316 B, 15-s-Kadenz) + NASA-DIF. (Schritt: `grind-pro`, Harvest-Bin.)
- **Rosetta RSI** — Register an ernte (gepostet); nur EAR2 (2007), kein
  EAR1/EAR3. (Schritt: ernte faltet.)
- **Zenodo** — sha256 `pending` (nur md5 geführt): Quaoar
  `10.5281/zenodo.21185812`, TNBFits `10.5281/zenodo.10620251`. (Schritt:
  Register an ernte.)
- **NRS-Hydrophon** — native FLAC, Compiler fehlt. (Schritt: Bau-Linie,
  FLAC-Decoder.)
- **archive_search Byte/Digest-Diskrepanz (neu gemessen 2026-09-16)** — der
  PATH-Bin liefert für binäre PLA-/arXiv-Objekte lossy-inflationierte Größen
  (Tar `909 769` vs. curl `502 226` B; FITS `65 030` vs. `46 080` B), obwohl
  `net.rs` in Folge 34 auf Rohbytes umgestellt wurde — der PATH-Bin ist der
  alte (nicht neu gebaut). (Schritt: nach dem CI-Build den neuen
  `archive_search` messen — `gh run view` des `ci-check`-Laufs.)

## Bande-Split / Sonden-ODF

- **160-Hz-Amplitudenzensus** — Probe gebaut (`band_amplitude_probe.rs`), kein
  CI-Workflow. (Schritt: `pioneer-band-amplitude.yml` anlegen.)
- **NOCC-Reduktionsvorschrift** — Integration in
  `twenty-second-band-ground-chain.md` blockiert (fremde uncommittete Hunk).
  (Schritt: Datei-Eigentum klären.)
- **7 planetare ODF** — `planetary-odf-cdn` run 35097513235: Lauf-Ende +
  Byte-Existenz. (Schritt: `gh run view`.)

## Positionslinien / Ephemeriden

- **CDN-Planetenbins** — Horizons-Rest gegen das 2026-09-11-Asset nicht
  protokolliert. (Schritt: `ephemeris_horizons_check` dispatchen.)

## CI-Läufe (dispatcht 2026-09-16, kein Poll)

- `aia-ladder-probe` run 35097506781; `corona-conditional-probe` run
  35097510109; `planetary-odf-cdn` run 35097513235; `ci-check` (net.rs
  raw-bytes). (Schritt: Artefakte in der Folgesession lesen.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`, bis
  eine Detektion samt Spektrum ins Register tritt. (Schritt:
  `docs/paper/jwst-disequilibrium-survey.md` §6.)

## Extern gebunden (kein Datum)

- Operator-Zeilen an entscheid (unverändert, `post.md`): CDDIS IONEX,
  GIC/INTERMAGNET, Fink/ANTARES/Rubin, WWLLN, Voyager/JPL-DSN + rohe
  ODF/TRK-2-34, Zhangheng-1, Babamul + GHRC, Woo/Armstrong, BiSON, DEMETER,
  CSES, Flyby-Path-2-Siegel. (Schritt: entscheid faltet.)

## Benchmark

- **h0-CMB-Atom — flash gegen pro/max (2026-09-16).** Doer `general` (flash)
  gegen `research-max` (pro/max) am PLA-Fetch/Inspect. **Ergebnis:** flash
  lieferte die saubere mechanische Extraktion (Tar-Member `plc-1.0.tar.bz2` +
  README, 4 FITS-HDUs inkl. COV-MAT, curl-sha256, Peak ℓ=233); pro/max fand die
  entscheidende Route — die 2018er Chain-ZIP
  (`COM_CosmoParams_base-plikHM-TTTEEE-lowl-lowE_R3.00.zip`, IRSA-Spiegel,
  Range-fähig), las Table I und lieferte den Nachrechen-Pfad. **Anders als die
  Routine-Klasse:** hier trug pro/max den Mehrwert (Routen-Entdeckung + Urteil),
  flash blieb bei der Oberfläche. **Burn (`session_burn`):** `research-max`
  $0.1526; `general` $0.0478 (3.2×). **Prägung:** für Routen-Discovery/Diagnose
  rechtfertigt pro/max sich; die mechanische Extraktion genügt flash.
- **Flash-Sweep 2026-09-16 (5 Läufe)** — Routine-Klasse flash-gewonnen
  (AGENTS.md); Sieger-Zeile für das h0-Atom siehe oben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
