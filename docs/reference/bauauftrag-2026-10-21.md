<!--
  title: Bauauftrag — die offenen Arme zu Ende bauen (2026-10-21)
  class: ref
  date: 2026-10-21
  status: live
  see-also: docs/reference/offen-ungebaut.md docs/reference/koeder-kette.md
-->
# Bauauftrag — die offenen Arme zu Ende bauen

**Regel:** Ein Punkt ist abgehakt, wenn der Code/das Register gebaut und geprüft
ist. Ein externer HTTP-Code (402/429/400), ein fehlender Token oder „keine
Daten" ist **kein** Haken — er ist eine eigene Spalte `Zugang` und ändert den
Bau-Status nicht. Kein `absent`/`blockiert` als Ersatz für Bau.

## Staffel

| Agent | Auftrag | Dateien (nur diese) | Status |
|---|---|---|---|
| **A** | `find → weave`: `--jsonl`, `--from-find`, `--terms`-Pflicht, `channel_ref`, Tests | `bin/find`, `tools/utils/src/bin/weave.rs` | **gebaut**, 9 Tests grün (uncommitted) |
| **B** | `archive_search`: `--alerce`, `--fink`, `--dust` (IRSA), Hilfe, `QUERY_MODES`, Tests | `tools/utils/src/bin/archive_search.rs` | läuft |
| **C** | Compiler Kitt Peak KPVT + Mount Wilson (MP/DP) + Register-Snippets | neue `tools/harvest/src/bin/*.rs`, `/tmp/*.φ` | läuft |
| **D** | Compiler COSMIC-1 + STEREO/EUVI + Register-Snippets | neue Compiler, `/tmp/*.φ` | läuft |
| **E** | 6 key-gebundene, 5 request-only, 3 neue Arme, 24 Compiler — bauen/verdrahten | Report + `/tmp/newarms_snippet.φ` | läuft |

## Die Punkte und ihr Abnahmekriterium

| # | Punkt | Gebaut wenn | Status |
|---|---|---|---|
| 1 | Verdikt A, 2. Hälfte: 8 Key-Arme + 4 Sonden erneut absuchen | je Arm ein Lauf mit Signatur/URL belegt | offen (E) |
| 2 | `find → weave` | `find --jsonl` + `weave --from-find` laufen, Tests grün | **gebaut** (A) |
| 3 | maschinenlesbare `find`-Ausgabe | JSONL mit `url/header/value/hex/verdict/terms` | **gebaut** (A) |
| 4 | `weave --terms` Pflicht | ohne `--terms` Exit 2, `--no-terms` explizit | **gebaut** (A) |
| 5 | `channel_ref` Pflichtglied | Spalte in `.map`, `--require-channel-ref` | **gebaut** (A) |
| 6 | drei neue Arme: `ds004100`, `10.17632/6mmd96j5bg`, `edges_raw` | Register-Snippet + Compiler/Route | läuft (E) |
| 7 | 6 key-gebundene: babamul, demeter, ieeg, ilrs_npt, pradan_ch2, tns | Arm im Code/Register, Token-Route | läuft (E) |
| 8 | 5 request-only: Voyager 1/2, Viking 1/2, Mariner 10, Juno pre-EFB | erreichbare Datenroute gebaut | läuft (E) |
| 9 | 2 ohne Block: erbq, fugin | Register/Route gebaut | läuft (E) |
| 10 | 24 `unregistered` Compiler | verdrahtet (Direktive) oder Binary entfernt | läuft (E) |
| 11 | vier Quellen-Compiler: KPVT, MWO, COSMIC-1, EUVI | Compiler baut + läuft + Register-Snippet | läuft (C/D) |
| 12 | 5 Funken, `archive_search`-Teil | `--alerce`, `--fink`, `--dust` gebaut | läuft (B) |
| 13 | 5 Funken, `measure`-Teil (TE, TDB, Forced Photometry) | **nicht** `archive_search` — eigene `measure`-Route | abgegrenzt |

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
