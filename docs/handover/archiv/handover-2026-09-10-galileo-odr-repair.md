<!--
  title: Handover — Galileo-ODR-Repair: das 7-Datei-Teilstück ersetzt durch das volle 10-Datei-Asset
  class: handover
  date: 2026-09-10
  sha256: 8a2a187e12a076e92ef0e827f3403721e21e11c9523f4a4d38ef7fa1e446e482
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Galileo-ODR-Repair (2026-09-10)

## Angenommen

- `handover-2026-09-10-galileo-odr-compiler.md` — der Offen-Punkt CDN-Dispatch;
  Operator-Wort (2026-09-10): löschen + neu manifestieren, Gate-Abbruch immer,
  `sources.φ`-Register in dieser Session. Council-Verdikt: harter Abbruch in
  allen Modi (auch Roundtrip/Fidelity void), `Source` trägt `annex_name` (Bare)
  neben `name` (Tabelle, `JS_`-präfixiert).

## Geleistet

- `galileo_odr_compiler.rs` repariert: `Source` trägt `annex_name` (Bare-Name
  des PDS-Annex, ohne `JS_`-Präfix) neben `name` (GODR-Tabelle, `JS_`-
  präfixiert); ein Gate-Versagen (read void / short / sha256-Mismatch /
  sample-rate void / hex void / Roundtrip void) bricht immer ab — kein Write,
  kein Upload, Exit 1. Neue Gate-Tests `gate_rejects_short_bytes`,
  `gate_rejects_provenance_mismatch`, `roundtrip_voids_on_corruption` +
  `gate_holds_when_provenance_matches`, `roundtrip_holds_for_valid_bin`.
  `cargo check` 0 Warnungen, 8 Gate-Tests grün. Commit `cb5b5f3`.
- Messergebnis des ersten Dispatch (Run 34526077426): Asset 238 683 208 B =
  8 + 7·96 + 238 682 528 — genau die 7 GOJ-Dateien, die 3 JS-Dateien fehlten.
  Ursache: `GO-JS-RSS-1-ODR-V1.0/ODR/` trägt die Namen ohne `JS_`-Präfix
  (gemessen am Annex-Listing); der Compiler fetched mit Präfix → 404 → stille
  Fortsetzung → Teil-Asset. Lokal verifiziert: `--dir`-Lauf über die 10
  Dateien hält Provenance + Roundtrip, Bin exakt 312 273 094 B.
- Re-Dispatch (Run 34529886849, 2m52s, success) manifestierte das volle
  10-Datei-Asset: 312 273 094 B, alle 10 `provenance holds` (darunter
  `JS_63540659/70561433/70571407` über Bare-Namen), Roundtrip hält. CDN-Digest
  `sha256:a5eb107ff75a790acfd703a5cc431fdabeebc2119375998768481621264b3345`.
- `phi/sources.φ`: `galileo_odr.bin` registriert (Commit `c118073`) —
  `format galileo_odr`, `at earth`, `ttl 604800`, 4 Feldzeilen
  `field ad1..ad4 galileo_odr_ad<N>_count inverse-square em count 604800 0.0 0.0`
  (nach `sources-v2-spec`; `em count` neben `em cpm`/`em 1`; Kernel
  `inverse-square` = em-Default; Einheitenliste wächst, kein Filter).

## Offen

- **sources.φ-Load-Verifikation** — der Block ist nach Spec + Präzedenz
  geschrieben, aber nicht lokal geladen-verifiziert: der Workspace-Build ist
  durch fremde, gleichzeitige `noaa_nodd`-Arbeit blockiert (der Pre-Commit-Hook
  kompiliert den ganzen Baum). Der erste Lade-Test steht aus, sobald der Baum
  wieder baut.
- **`galileo_odr`-Format-Reader** — der Compiler verpackt origin-verbatim; ein
  Laufzeit-Reader, der das `.bin` in Feld-Samples dekodiert, fehlt (Analogon:
  `supermag_compiler` baute den Reader mit). Folge-Atom.
- **880 Folgebytes von `70580900.ODR`** — erhalten, ungedeutet (PDS-Fußstruktur
  vs. abgeschnittener Record); ein Probe kann die Bytes inspizieren. Keine Eile.
- Die fünf übrigen Galileo-Floor-Atome stehen unverändert im Autonom-Handover.

## Archiv

- `handover-2026-09-10-galileo-odr-compiler.md` → `docs/handover/archiv/`
  (verbraucht; sein Offen-Punkt Dispatch ist hier gearbeitet).

## Session-Notiz

- Beide Commits liefen `--no-verify` (Operator-Wort über den separaten Commit):
  der Pre-Commit-Hook bricht auf der fremden, gleichzeitigen `noaa_nodd`-Arbeit
  ab (erst Datei fehlt, dann 5 rustc-Fehler). `main` selbst ist sauber (HEAD
  trägt kein `noaa_nodd`); jeder Commit trägt nur je eine eigene Datei. Die
  fremden Baum-Änderungen (HARPS, noaa_nodd, noaa-ghcn/gsod/isd, vo-tap-Reste)
  gehören anderen Atomen und blieben unberührt.
