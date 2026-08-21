<!--
  title: Handover: Source-Port — die offene Pipeline (Zustandsmaschine, Grind, Kuration)
  class: handover
  date: 2026-08-21
  sha256: 0784307095ec770e9e6ee294bcc23275a54de96bd024d6745819ad1fbc6ba591
  status: live
  see-also: docs/SOURCE_PORT.md TODO.md phi/pipeline/ledger.φ docs/handover/handover-2026-08-21-katalog-luecken.md
-->

# Handover: Source-Port — die offene Pipeline

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Alle Source-Arbeit läuft über den einen Pfad: `docs/SOURCE_PORT.md` ist das
selbsttragende Protokoll (Zustandsmaschine, Workflow, Referenz-Karte,
Pfadkarte). Arbeitsfläche: `phi/pipeline/` (queue/, park/, stage/,
ledger.φ, prompt.φ). Das Detail-Register ist `phi/pipeline/ledger.φ` —
dieses Dokument ist die Karte der offenen Posten, keine Kopie des Ledgers.
Die Katalog-Lücken (Wellen I–III, Zeitkritisch, Kuration Unit-Arme) tragen
eine eigene Übergabe (`handover-2026-08-21-katalog-luecken.md`).

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "entdeckt\|kompiliert\|disponiert" phi/pipeline/ledger.φ | head
```

Referenzen (stehend): `docs/SOURCE_PORT.md` (der eine Pfad),
`phi/pipeline/ledger.φ` (Zustands-Register), `phi/pipeline/queue/`,
`phi/sources.φ` + `phi/dead_sources.φ` + `phi/blocked_sources.φ`,
`/home/johannes/projects/archive/archeology/` (Vorräte), TODO.md.

## Die Einheiten

### A. Kompilat-Pfad in die Zustandsmaschine

Der Weg tap_index → kernel_flatten.yml → tap_compiler → CDN → sources.φ
läuft außerhalb der Zustandsmaschine (SOURCE_PORT §4) — kein
ledger-Eintrag, kein Pfadkarten-Eintrag. Deshalb zerfleddert ein großer
Katalog in Queue/Metadaten/Weights/Stage, ohne je aufgelöst zu werden.
Vereinheitlichung: eine Kompilat-Stufe (`entdeckt → kompiliert →
disponiert`) in ledger.φ + Pfadkarte; `disponiert` räumt die
Discovery-Reste. Berührt SOURCE_PORT.md + ledger.φ + ggf. main.rs
(--fish-Flag).

### B. S3-Harvester

xml_harvester löst den ListBucketResult-Namespace nicht (0 records,
getestet 2026-08-20) — die NOAA-NODD-S3-Buckets (sea-ice/GDP-Drifter/
cors/bathymetry) sind geparkt (ledger.φ parser-gap); braucht
Namespace-Handling oder einen s3_harvester.

### C. Probe-Stufe — nächste Welle

Neue Kandidaten aus den Katalogen in batches/ nachrücken; der
Probe-Batch `queue/grind_dataverse.φ` (136 Blöcke, Harvard 90 + Borealis
46, Gewicht ≥ 16, Dataverse-API je DOI live verifiziert) wartet. Die
Linse: Folgewelle — NASA-CMR-Keywords + GBIF-Tags downloaden, Library
feinwägen; --port ersetzt --gold.

### D. Queue

10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/exotic/
candidate-staging) — Port durch die Prozedur; astro-Korpus: 28 Blöcke →
manueller Port.

### E. Grind-Einbau

32 ArcGIS-Drafts (thermal/seismic/diffusion/em/advective/gravity);
ARI GCNS (331.312 Sterne ≤100pc) + MWSC (3.006 Haufen) als
Kompilat-Kandidaten; 8 VirES-Drafts (CHAMP/GRACE/GOCE/CryoSat
MAG/DNS/WND/TEC/KBR); archeology-gaps 77 Kandidaten (AERONET, IERS-EOP,
Fireball/Sentry, Xamin-TAP, GONG2, GIRO-Ionosonde, e-CALLISTO …) als
nächster Grind; FRB-Union-Merge mit TNS-Namens-Normalisierung
(FRB121102 ↔ FRB20121102A) + frbcat.org-CSV als Quelle.

### F. Nachlauf

VirES-Vollprobe (64 Drafts, Datei ABSENT) + DONKI-Familie (CME-Draft,
Datei ABSENT) — in Blöcken.

### G. Park

Pegelonline, USGS-Geomag, GWOSC/GraceDB (Skymap), DSN, CENC,
JMA-Quake (cod-String), SDSS-SkyServer.

### H. Harvester-Binaries (Rats-Befund)

kafka_harvester/fdsn_harvester als eigenständige Binaries zulässig —
std-only bindet den Archivar-Runtime, nicht die Produktions-Tools.
Reihenfolge: (1) Force-Gate zuerst — Alert-Ströme ohne Feldwert am Punkt
fallen (ANTARES, dead_sources.φ); (2) REST-Pull zuerst — GCN circulars/
notices, IceCube, GraceDB, MPC tragen REST → rest_harvester deckt sie
(lebt schon als src/bin/rest_harvester.rs); (3) nur ZTF (Kafka+Avro,
IRSA-Auth declined) und FDSN dataselect (miniSEED-Zeitreihe) brauchen
echte Decoder — beide AUSSTEHEND hinter dem Gate; miniSEED-Frage: eine
Waveform zerfällt in Samples (TESS-Muster) ODER in Bins (Spektral-Atom)
— das Instrument deklariert seine Basis; (4) Hand-Client vs. Crate:
AUSSTEHEND — fällig erst, wenn ein Kafka-only-Feed das Gate passiert.

### I. SuperMAG

Leading-line „OK"-Strip lebt; Positions-Join + station-Filter bleiben
server-blockiert — db-get-Fault, phi/-Zugang logon-only.

### J. Kraft-Abdeckung

acoustic/electric/thermal/advective/diffusion-Kuration offen — electric:
GIC-Netze + Live-E-Feldstärke (kein Feed); GLM ist em (Ratsurteil);
WWLLN radio-em vs. Entladung-electric bleibt Force-Gate-Frage.

### K. Die drei Ports der Nadeln — offene Reste

IONEX-GIM — der `format ionex`-Parser lebt, der Kanal ist AUSSTEHEND
(CDDIS verlangt Earthdata-OAuth, GFZ/BKG/IGN-Routen 404/000 am
19.8.2026); kein Block im Register, bis eine Route anonym lebt oder der
Earthdata-Account existiert. WARTEND: SuperMAG (I), Gaia DR4
(2.12.2026 — Recompiler der 44-Byte-Records).

### L. Teleskop-Inventar (ledger.φ geparkt)

GCN-API v0.1 tot (Einstein-Probe-Blöcke master.φ stale, kein
Listen-Feed → SVOM-Block nicht baubar); NRAO = Angular-SPA; CHIME =
CANFAR-DOIs statt API; svom.ac.cn = HTML, Zertifikat abgelaufen;
ESA-AMA-TAP-Basis ungefunden (nur EAS/Euclid lebt); Keck/KOA unkuratiert;
eROSITA DR2 = HTML-Landing; MAGIC/HAWC = HTML+FITS-Portale, TLS-Kette
unvollständig; LHAASO = News-Seite 2021 → Decline. IRSA spherex.obscore =
VOTableJSON-Atom; Euclid mer_catalogue = SpaltenAusMetadata-Atom; ESO
tap_obs = echte CSV, aber probe_csv klassifiziert Header-CSV nicht
(Probe-Limitierung); Pan-STARRS dr1 mean = Endpoint lebt, Probe-Env
kennt {ra}/{dec}/{radius} nicht (Nachweis im Register-Lauf offen).
Befunde: phi/pipeline/research/agent_output/verify_astro{,_b}_2026-08-19.φ.

### M. Sensor-Kategorien-Welle

18 live-Kandidaten geparkt (ledger.φ — Port ausstehend: AMeDAS, ECCC
GeoMet, BfS-ODL, GTMBA, EMODnet, EMSO, IOOS-Glider, SmartBay,
USGS-Grundwasser, NRCS-AWDB, IGRA, Wyoming, Iowa-RAOB, SondeHub,
AWC-PIREP, COSMIC-2, IMO, GeoNet, meteo.lt); 14 blocked
(blocked_sources.φ — key-needed; 3 ip-blocked lokal nachprüfen:
Meteomatics, CelesTrak, MeteoSwiss-Pollen); 5 dead/declined
(dead_sources.φ: Saildrone, SatNOGS-API, TreeTalker, OSDR, WindBorne,
IGRAC, AOML); 13 angekündigt (MTG-I2 27.08.2026, MetOp-SG B1,
Sentinel-3C, C-130J, NASA-777, Axiom, Orbital Reef, Starlab, SOFF,
ITER, SPARC, DUNE, EMSO-SMART-Cable). Befunde:
phi/pipeline/research/agent_output/*_2026-08-19.φ + classify_2026-08-19.φ.

### N. Parser & Spec

VOTableJSON (ausstehend, ledger.φ) — IRSA-TAP liefert
VOTable-serialisiertes JSON (s_ra/s_dec nur als FIELD-Metadaten);
SpaltenAusMetadata (ausstehend, ledger.φ) — Euclid/EAS-TAP antwortet
{metadata:[{name:…}], data:[[…]]}; Hapi-FieldConfig — die deklarierten
kernel/force/tau der HAPI-Blöcke erreichen den Oszillator nicht
(synthetisch {0,0,0}).

### O. Host-Kuration

CENC (Keyed Object No1..NoN), JMA-Quake (Position im cod-String),
Pegelonline (Fanout-Block steht aus — P09), GWOSC/GraceDB (Position nur
via Skymap), DSN (statische Dish-Positionen), USGS-Geomag
(Komponenten-Timeseries).

### P. Enrichment

Name-basierter Ersatz-Join — offen.

### Q. Vorräte (unter /home/johannes/projects/archive/archeology/)

sources/sources_gold_pre-cdn_27k (2572 Blöcke) +
sources_recovery_pre-cdn_25k (1924) — Migration nach Protokoll
(docs/SOURCE_PORT.md); sources_new_untested_14k (873) +
sources_astro_untested (30) + sources_exotic_untested (16) +
sources_earth_untested (3) — UNTESTED_index.txt nicht archiviert,
per-Domain-Index rekonstruieren; sources_recovery_cdn-merged_60k
lost-blocks (5701 urls, 0 field-Tokens) — Extract-Parameter aus
history/recovery zuordnen; arena/ (batch_01–21, ungeprüft); foundation/
(APIs/collection/gaps).

### R. Port-Migration ohne τ (S2)

Die pre-cdn-Grammatik trägt kein τ-Token — port_field_synth verweigert
Felder ohne kuratiertes τ; felderlose Konvertate werden nicht
übernommen (flush_port_block). Die Alt-Blöcke
(phi/pipeline/research/batches/ 283 + probe_batches/ 242) bleiben
unkonvertiert-pending, bis τ je Feld kuratiert ist (Register:
phi/pipeline/queue/).

### S. Bestands-Pflichten im Register

- Zwei Bestands-Blöcke in phi/sources.φ deklarieren `on earth 52.5 13.4`
  ohne alt — seit S2 refused; alt deklarieren oder die Blöcke bleiben
  dunkel.
- Fanout-Stationen ohne Höhe (stations_lat/lon ohne stations_alt-
  Direktive): alt-Slot 0.0 = fehlende Messung bis die v3-Maske das Bit
  trägt; eine `stations_alt`-Direktive steht aus.
- mpcobs: das Bin (src/bin/mpcobs_compiler.rs) hat keinen Konsumenten im
  Archivar (Integration pending) — der 0.0-Slot bleibt Wire-Pad bis die
  Konsum-Kette existiert; die Autorität liegt dann beim Konsumenten:
  `mag > 0.0`-Gate (blank → kein Messwert), die Vega-Kollision (mag=0
  ist ein physikalischer Wert) ist benannt (D1-Verdict).
- v8-Präsenz-Maske: der color_index-Slot bleibt bis v8 das
  0.0=absent-Wire-Pad (Weiß); BP−RP=0 (A0V) kollidiert — die v8-Maske
  (Rats-Urteil-1-Muster) trägt den Farb-Slot als Bit (D2-Verdict).
- INTERMAGNET-Fanout (154 Observatorien live): Ausbeute-Feintuning offen
  (best-avail-Aktualität variiert je Observatorium).

### T. Struktur-Reader

netCDF-3 (CDF-1 + CDF-2, std-only) in src/netcdf.rs lebt; CDF-5 bleibt
pending (eigenes Atom); offen: Parquet/Arrow, OPeNDAP, CDF, GRIB-2,
GeoParquet, OGC-SensorThings. **Register-Drift benannt (2026-08-21):**
die TODO-Zeile führt auch „FITS-Binärtabellen" und „netCDF-4/HDF5" als
offen — beide sind erledigt (src/fits.rs trägt BINTABLE; src/hdf5.rs
liest netCDF-4/HDF5, NCEI-SSI-Atom). Beim ersten Schnitt in diesem
Abschnitt die Register-Zeile korrigieren. Offene FITS-Reste: Spaltencodes
'A'/'X'/'P'/'Q', CONTINUE/HIERARCH-Cards, Nicht-TAN-Projektionen
(SIN/STG/ZEA) — kein konkreter Katalog-Bedarf steht an; der erste Bedarf
(Gaia/SDSS/2MASS-Ernte) holt sie.

### U. Befund-Zeilen ohne Reparatur-Auftrag

- ω-Loop-Fetch-Sturm (TODO-Zeile): die Reparatur ist ERLEDIGT
  (Fetch-Sturm-Reparatur, 2026-08-21 — In-Flight-Guard, 2ⁿ-Backoff,
  Budget 2³). Offen bleibt nur: Budget-Messungen brauchen einen
  begrenzten/drosselbaren Lauf — das trägt `handover-2026-08-21-offene-
  atome.md` (Atom 1). Beim Schnitt die TODO-Zeile auf den Rest schärfen.
- health-Label-Befund: erledigt (Prepare-Step legt das Label an) —
  Register-Zeile beim Schnitt entfernen.

## Gates

- Force-Gate je Kanal (Litmus: könnte ein nicht-menschlicher Organismus
  ein Sinnesorgan für diese Messung evolvieren?); τ-Gate: ohne deklariertes
  τ kein Sample. Beide Gates verweigern — kein Default.
- Jede Einheit endet mit ledger.φ + Register-Zeilen (sources.φ oder
  blocked/dead_sources.φ) im selben Commit.
- cargo check 0/0 für alles, was src/ berührt.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Katalog-Lücken (eigene Übergabe), offene-atome (Sample-Budget/GLADE+/
grüner Lauf), die Sphären, spektraler Oszillator, Validation/CI,
Stern-/Asteroiden-Physik.
