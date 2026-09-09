<!--
  title: Handover — Weberin-Folge: CDN-Dispatch-Fixes, NED-REST-API-Kegel-Tiling, 2MRS-z-Feldwert
  class: handover
  date: 2026-09-09
  sha256: fe31b7919911d1559c435f531450cd99f4ccebcd91a1660bbe56dbaccf7cb191
  status: live
  see-also: docs/TODO.md docs/auftrag/auftrag-ned-objdir-zugang.md phi/sources.φ docs/concepts/archivar-mathematikerin.md
-->

# Handover — Weberin-Folge: CDN-Dispatch-Fixes, NED-REST-API-Kegel-Tiling, 2MRS-z-Feldwert

Übergabe der Session, die die vier Weberin-III-Posten (ned/meteo/argo/REVIEW) schloss
und daraus die NED-z-Frage weitergetrieben hat. Parallel lief eine sehr aktive
Fremdlinie (Seismik/Neptun/vo-tap/te-PCMCI) — deren Arbeit steht getrennt und ist
nicht von dieser Session.

## Was gebaut + committet ist

**meteo-cdn (geschlossen):** der Manifestator trug 51 Variablen, `sources.φ` 30.
Der `HOURLY_KATALOG` ist gestrichen; der Manifestator liest die Variablen jetzt
aus dem Event (`variables`-Array in `config/meteo/tibet-flut-2026.json` = genau
die 30 registrierten). 65 Orphan-Assets (21 Extra-Variablen × 3 + 2 Scratch)
gelöscht — der Release `archive-api.open-meteo.com` trägt exakt 90 Assets = 30×3.

**argo_bgc (gelandet):** der sequentielle Fetch sprengte den 240-min-Timeout.
Paralleler Fetch (`std::thread::scope`, `available_parallelism`) + Timeout 360 +
`--max-profiles 1500`. `argo_bgc.bin` (18,7 MB) liegt auf der CDN
(`data-argo.ifremer.fr`).

**REVIEW-CDN (geschlossen):** die 21 REMOVE-Hosts sind vom CDN gelöscht,
`zenodo.org` (SuperDARN-Feldblock) bleibt.

**2MRS-z-Feldwert (gebaut + gelandet):** die stabile z-Quelle. `twomrs_compiler`
→ `twomrs.bin` (Magic `2MRS`, Records `[ra, dec, cz, e_cz]`, absent cz
übersprungen — nie 0-Sentinel) via `tapvizier.cds.unistra.fr`
(`J/ApJS/199/26/table3`, 44.599 Galaxien, cz). Feld in `sources.φ`:
`field cz twomrs_cz_kms gaussian-inverse-square em km/s`. Getrennt von `ned`
(Rat 2026-09-09, einstimmig: zwei getrennte ehrliche Messungen derselben Achse,
nie gemischt; cz→z bleibt ein benanntes Gesetz an der Lesestelle).

**NED-Saga (die Kernlinie):**
- Der objid-Keyset-Walk starb reproduzierbar bei objid ~3,6 Mio (NED-Sync
  `executionDuration default 60`, dokumentiert — kein Config-Fehler; die Adresse
  war korrekt, VOSI-verifiziert).
- Async staut sich unter Last; der eigene stündliche Schedule hielt NED dauerhaft
  down.
- **Der übersehene Schlüssel:** `Docs::API` dokumentiert eine **REST-API**
  (`/NED::API/ConeSearchByPosition`, IVOA Simple Cone Search), getrennt vom TAP —
  mit `Z_CONSTRAINT=Available` (nur Objekte mit z) und `MAXREC`. Zuverlässig
  (kein `void` wie TAP); verlässlicher Radius ≤ 30' (SR 60' grindet >240 s).
- Gebaut: `ned_cone_compiler` (ein Kegel je Aufruf, VOTable-Parse, Lattice 1024²,
  Dedup 4/Zelle) + `ned-cdn.yml` als Autoresume über ein Hex-Kegel-Gitter.
- Unterabtastung: `--spacing-deg` verdrahtet, Default 5° → ~1.900 Kegel (~1 Tag)
  statt der vollen 63.500 (Monat).

## Harte Grenzen / 0 honored (gemessen)

Kein Bulk-z-Export des NED-objdir existiert („NEDL" kein Format; einziges
Dateiprodukt NED-LVS, kuratiertes 2-Mio-Sample). Der NED-objdir-Weg bleibt
dienst-begrenzt; die stabile z-Ernte läuft über 2MRS-via-CDS.

## Offen / für die nächste Session

- **`fa5467c` (Spacing-Default 5°) ist committet, aber NICHT gepusht** — blockiert
  durch die Fremdlinie (8 untracked Dateien mit anderem Inhalt im Baum:
  `tools/vo-tap/*`, `astrometry.rs`, `depth_phase_field_probe.rs`,
  `neptune_apparent_chain_probe.rs`). Nächste Session: pushen, dann
  `gh workflow enable ned-cdn.yml` + Dispatch — der unterabgetastete Crawl
  läuft dann ~1 Tag autonom.
- **`ned-cdn.yml` ist derzeit `disabled_manually`** (Voll-Crawl gecancelt,
  Schedule pausiert). Erst nach Push von `fa5467c` wieder aktivieren.
- **`docs/auftrag/auftrag-ned-objdir-zugang.md`** (externer Rechercheauftrag,
  `status: pending`): die menschliche IPAC-Rückfrage (Dienst-Erholung, Bulk-z,
  schonende Route) ist offen. Ergebnis-Datei der Messung liegt lokal:
  `/home/johannes/Schreibtisch/auftrag-ned-objdir-zugang_results.md`
  (NED `degradiert`, 2MRS-via-CDS `live`, IPAC-Kontakt `pending`).
- **TODO.md**: die Register-Einträge dieser Session (ned/2MRS/meteo) stehen; die
  Fremdlinie editiert TODO.md parallel — nicht anfassen.

## Andere Linien (nicht diese Session)

Sehr aktiv im geteilten Baum: Seismik/ak135/`depth_phase_probe`, Neptun-astrometrie
(`neptune_apparent_chain_probe`, FK4-App-Reduktion), ein neues `tools/vo-tap`-Crate,
te-PCMCI (`te.rs`), AllWISE-Footprint. Deren Commits (4c28bcc, cb53b5b, 693cb18,
9e3c46b, 95fd825, bf7e5de, d41a945 u. a.) und untracked Arbeit NICHT committen/
anfassen — das ist deren Linie.

## Verifikation

`cargo check -p omegaflow-harvest` 0/0 für die eigenen Compiler
(`twomrs_compiler`, `ned_cone_compiler`, `argo_bgc_profile_compiler`,
`meteo_cache_manifest`, `tap_compiler`). Das Gesamt-`cargo check` trägt die
Fremdlinien-Warnungen/-Fehler. `twomrs` und `ned_cone_compiler` Roundtrip-/Unit-
Tests grün. Die CI-Läufe argo/meteo/2MRS sind `success`.
