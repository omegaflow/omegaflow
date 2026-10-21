<!--
  title: Bauauftrag — die offenen Arme zu Ende bauen (2026-10-21)
  class: ref
  date: 2026-10-21
  sha256: 30245d564e914b404961e510df4a82dab81765fa24c438a74ff48e62bce00c50
  status: live
  see-also: docs/reference/offen-ungebaut.md docs/reference/koeder-kette.md
-->
# Bauauftrag — die offenen Arme zu Ende bauen

> **Kanonischer Auftrag:** `projects/bau-der-arme.md`. Dieses Blatt trägt die
> Staffel, die Assets und die Key-Prüfung als Beleg.

**Regel:** Ein Punkt ist abgehakt, wenn der Code/das Register gebaut und geprüft
ist. Ein externer HTTP-Code (402/429/400), ein fehlender Token oder „keine
Daten" ist **kein** Haken — er ist eine eigene Spalte `Zugang` und ändert den
Bau-Status nicht. Kein `absent`/`blockiert` als Ersatz für Bau.

## Staffel

| Agent | Auftrag | Dateien (nur diese) | Status |
|---|---|---|---|
| **A** | `find → weave`: `--jsonl`, `--from-find`, `--terms`-Pflicht, `channel_ref`, Tests | `bin/find`, `tools/utils/src/bin/weave.rs` | **gebaut**, 9 Tests grün (uncommitted) |
| **B** | `archive_search`: `--alerce`, `--fink`, `--dust` (IRSA), Hilfe, `QUERY_MODES`, Tests | `tools/utils/src/bin/archive_search.rs` | **gebaut** — ALeRCE 200, IRSA 200, Fink-Route; 612 Tests |
| **C** | Compiler Kitt Peak KPVT + Mount Wilson (MP/DP) + Register-Snippets | neue `tools/harvest/src/bin/*.rs`, `/tmp/*.φ` | **gebaut** — echte FITS-Läufe, Assets hochgeladen |
| **D** | Compiler COSMIC-1 + STEREO/EUVI + Register-Snippets | neue Compiler, `/tmp/*.φ` | **gebaut** — COSMIC 200/Tarball, EUVI 206/Range, Assets hochgeladen |
| **E** | 6 key-gebundene, 5 request-only, 3 neue Arme, 24 Compiler — bauen/verdrahten | Report + `/tmp/newarms_snippet.φ` | **gebaut** — 46 Zeilen, 4 Blöcke im Register |
| **F** | offene Zwillinge für brave/exa/tavily/serpapi/consensus/googlecse/fink/demeter | `archive_search.rs` | **gebaut** — `--4get` (200), `--antares` (200); 616 Tests |

## Die Punkte und ihr Abnahmekriterium

| # | Punkt | Gebaut wenn | Status |
|---|---|---|---|
| 1 | Verdikt A, 2. Hälfte: 8 Key-Arme + 4 Sonden erneut absuchen | je Arm ein Lauf mit Signatur/URL belegt | **gebaut** (E; 6 Key-Arme + 5 Sonden, `--verdict` je URL) |
| 2 | `find → weave` | `find --jsonl` + `weave --from-find` laufen, Tests grün | **gebaut** (A) |
| 3 | maschinenlesbare `find`-Ausgabe | JSONL mit `url/header/value/hex/verdict/terms` | **gebaut** (A) |
| 4 | `weave --terms` Pflicht | ohne `--terms` Exit 2, `--no-terms` explizit | **gebaut** (A) |
| 5 | `channel_ref` Pflichtglied | Spalte in `.map`, `--require-channel-ref` | **gebaut** (A) |
| 6 | drei neue Arme: `ds004100`, `10.17632/6mmd96j5bg`, `edges_raw` | Register-Snippet + Compiler/Route | **gebaut** (E; Blöcke im Register, Assets 200) |
| 7 | 6 key-gebundene: babamul, demeter, ieeg, ilrs_npt, pradan_ch2, tns | Arm im Code/Register, Token-Route | **gebaut** (E; alle 6 Routen + Tokens) |
| 8 | 5 request-only: Voyager 1/2, Viking 1/2, Mariner 10, Juno pre-EFB | erreichbare Datenroute gebaut | **gebaut** (E; 5/5 Register+Compiler) |
| 9 | 2 ohne Block: erbq, fugin | Register/Route gebaut | **gebaut** (E; JMA + Fugin-Compiler, Fugin im Register) |
| 10 | 24 `unregistered` Compiler | verdrahtet (Direktive) oder Binary entfernt | **gebaut** (E; 13 retired, 9 variant, 2 wire — beide im Register) |
| 11 | vier Quellen-Compiler: KPVT, MWO, COSMIC-1, EUVI | Compiler baut + läuft + Register-Snippet | **gebaut** (C/D; echte Läufe + Assets) |
| 12 | 5 Funken, `archive_search`-Teil | `--alerce`, `--fink`, `--dust` gebaut | **gebaut** (B; ALeRCE/IRSA 200, Fink-Route) |
| 13 | 5 Funken, `measure`-Teil (TE, TDB, Forced Photometry) | **nicht** `archive_search` — eigene `measure`-Route | abgegrenzt |

## Offene Zwillinge (Agent F, gemessen)

| Blockierter Arm | offene Alternative | Beleg | gebaut |
|---|---|---|---|
| `--brave` (402) | `--4get` (Metasearch, keyless) + `--searxng` | 4get `200`, 10 Treffer; Brave-Doku 403 | **ja** |
| `--exa` (402) | `--4get` + `--openalex`/`--semanticscholar` | 4get 200; Exa-API 404/402 | **ja** |
| `--tavily` (432) | `--4get` | 4get 200; Tavily 401 | **ja** |
| `--searchapi` (429) | `--4get` | 4get 200; searchapi 000 | **ja** |
| `--serpapi` (429) | `--4get` | 4get 200; serpapi 401 | **ja** |
| `--googlecse` (400) | `--4get` | 4get 200; CSE 403 | **ja** |
| `--consensus` (429) | `--openalex`, `--semanticscholar`, `--europepmc` | bereits im Baum | kein Neubau |
| `--gemini` (429) | keine belastbare offene Entsprechung | Pollinations 200→402; Groq/DeepInfra 401 | nein (Befund) |
| `--fink` (000) | `--antares` (NOIRLab) + `--alerce` | ANTARES 200, `ANT2018hocaw` | **ja** |
| demeter (kein Key) | keine Such-API-Entsprechung | `api.demeter.space` 000 | nein (Befund) |

Nicht gebaut, weil gemessen unbrauchbar: DDG Lite (Challenge), Mojeek (403/Captcha),
Startpage („Blocked"), Yandex (Captcha), LibreX (301/000), 4get.ca (503).

## Live-Arme (GOES / Hunga Tonga)

`--live <listing-url>` ist der allgemeine Arm: er liest **jede** S3-ListObjectsV2-
XML oder jeden HTML-Verzeichnisindex und gibt das neueste Objekt aus (Key,
LastModified, Size; keyless). `--goes-live <g18|g19> [FDCF|AODF|GLM]` baut
daraus nur die G18/G19-URL — ein Preset, kein Sonderfall. Live gemessen:
S3 FDCF `05:40:14Z` / 1.172.966 B; KPVT-Index `m2007f.fits`.

Die Hunga-Tonga-Probe (`tools/measure/src/bin/hunga_tonga_netz_probe.rs`)
bleibt der fachliche Konsument; die S3-Listung selbst gehört nicht mehr in
einen Einzel-Probe-Silo. `--live` dort kann den Arm aufrufen.

## Veröffentlichte Assets (2026-10-21, `gh release upload`)

| Tag | Asset | Bytes | sha256 |
|---|---|---:|---|
| `openneuro.org` | `sub-HUP060_…interictal…run-01_ieeg.bin` | 70.800.669 | `be750a43…` |
| `data.mendeley.com` | `class_xrf.bin` | 32.812 | `ff76ca3e…` |
| `downloads.sws.bom.gov.au` | `sws_ionosonde.bin` | 9.385.708 | `9eb32cae…` |
| `nispdata.nso.edu` | `kpvt_synoptic_grid.bin` | 3.888.008 | `64a62aff…` |
| `sha.stanford.edu` | `mwo_mag_mp.bin` | 11.835.848 | `5b49ac56…` |
| `sha.stanford.edu` | `mwo_mag_dp.bin` | 11.835.848 | `135c8db0…` |
| `data.cosmic.ucar.edu` | `cosmic1_temp.bin` | 70.388 | `7c0187d2…` |
| `stereo-ssc.nascom.nasa.gov` | `stereo_euvi.bin` | 48 | `ec1b4a8a…` |

## Schlüssel — geprüft (Namen, nie Werte)

`bin/secrets_keys`: **190 Key-Namen** mit Wert. Die für die Arme relevanten:

| Arm | Key-Name | vorhanden | letzter Messwert |
|---|---|---|---|
| `--serper` | `SERPER_API_KEY` | **ja** | **200, Treffer** (Schumann-Lauf) |
| `--brave` | `BRAVE_API_KEY` | **ja** | 402 Abrechnung |
| `--exa` | `EXA_API_KEY` | **ja** | 402 |
| `--tavily` | `TAVILY_API_KEY` | **ja** | 432 Plan-Limit |
| `--searchapi` | `SEARCHAPI_API_KEY` | **ja** | 429 |
| `--serpapi` | `SERPAPI_API_KEY` | **ja** | 429 |
| `--consensus` | `CONSENSUS_API_KEY` | **ja** | 429 |
| `--gemini` | `GEMINI_API_KEY` | **ja** | 429 |
| `--googlecse` | `GOOGLE_CSE_API_KEY` + `GOOGLE_CSE_CX` | **ja** | 400 „key not valid" |
| `--lasair` | `LASAIR_LSST_TOKEN`, `LASAIR_TOKEN` | **ja** | Arm vorhanden |
| babamul | `BABAMUL_API_TOKEN` | **ja** | Arm fehlt noch |
| ieeg | `IEEG_USER`, `IEEG_PASS` | **ja** | Arm fehlt noch |
| pradan | `PRADAN_USER`, `PRADAN_PASS` | **ja** | Arm fehlt noch |
| tns | `TNS_API_KEY`, `TNS_UA` | **ja** | Arm fehlt noch |
| dust/IRSA | `IRSA_USER`, `IRSA_PASS` | **ja** | Arm fehlt noch |
| ilrs_npt (CDDIS) | `EARTHDATA_EDL_TOKEN` | **ja** | Route prüfen |
| `--fink` | — | nein | öffentliche API prüfen |
| `--alerce` | — | keyless | API prüfen |
| demeter | — | nein | Zugang/Registrierung prüfen |

**Konsequenz:** „Key-gebunden" war eine falsche Sammelzeile. Die Keys sind da;
die 402/429/400 sind Konto-/Quota-/Key-Gültigkeits-Probleme, keine fehlenden
Schlüssel. Die Arme werden trotzdem gebaut (Bau ≠ Zugang).

## Abnahme je Punkt

```sh
# Kette
sh -n bin/find
cargo test -p omegaflow-utils --bin weave

# Arme
cargo test -p omegaflow-utils --bin archive_search

# Compiler
cargo build --release -p omegaflow-harvest --bin kpvt_synoptic_compiler \
  --bin mwo_mag_compiler --bin cosmic1_ro_compiler --bin euvi_compiler

# Register
target/release/source_validate
target/release/register_lookup --compilers
```
