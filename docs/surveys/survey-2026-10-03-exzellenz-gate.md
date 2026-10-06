<!--
  title: Exzellenz-Gate — der Maßstab auf die Papers angewendet (Stand 2026-10-03)
  class: survey
  date: 2026-10-03
  sha256: d055d9a16dbfd3b1ec4b872af2ca1f0927f4bf385dbada9e406b5ff7e6276681
  status: live
  see-also: docs/concepts/exzellenz-konzept.md
-->
# Exzellenz-Gate — der Maßstab auf die Papers angewendet (Stand 2026-10-03)

Anlass: `docs/concepts/exzellenz-konzept.md` §4 — der Maßstab wird auf die
Papers angewendet, jede Verletzung benannt, nichts geglättet. Methode: zwei
read-only Taucher (flash, 2026-10-03) über `docs/paper/*.md` + `docs/blatt/*.md`;
`sha256` via `omega_sh sha`, siehe-auch/Codepfade via `glob`, Verdiktswörter via
`sgrep`. Gemessen am Arbeitsbaum um HEAD `29993494b`.

## Klassifikation

- **Messpaper/-Sheet (Gegenstand des Maßstabs):** `big-bang-echo-sheet-12`,
  `broken-null-control`, `corona-heating-ladder`, `cross-screening-tibet`,
  `causal-arrow-preregistration`, `dark-flow-sheet-8`, `depth-phase-echo-fleet`,
  `eclipse-clock-worldlines`, `flyby-path-1-cold-cases`, `flyby-path-2-*`,
  `galileo-rotor-spin-era-floor`, `gic-causal-driver`, `ground-sources-20s-band`,
  `h0-lines-register`, `jwst-disequilibrium-survey`, `laic-arrow-direction`,
  `lead-geometry-direction`, `nadel-v-fresh-area-dip-scan`, `neptune-rift-ephemerides`,
  `planet-nine-kbo-residue`, `probe-front-dark-matter`, `signal-cone-audit-sheet`,
  `solar-cycle-dynamo`, `solar-seconds-matrix`, `sturzflut-tibet-pfeil`,
  `text-as-data-pioneer`, `tonga-lamb-crosscheck`, `twenty-second-band-ground-chain`,
  `uranus-rift-ephemerides` + die `docs/blatt/*.md` (sonne-erde, der-grat,
  kreuz-screening-gyirong, solar-seconds, thuan, anderson-flyby, pioneer-floor,
  te-externer, fruehwarnsystem).
- **Externe Referenz-Abstracts (kein omegaflow-Messpaper):**
  `armstrong-1998-phase-scintillation-abstract`, `armstrong-woo-estabrook-1979-…`,
  `asmar-2005-…`, `woo-armstrong-1979-jgr-abstract`,
  `yu-tong-fang-hu-2022-…`, `terminologie-der-gegenstroemung` (trägt `class: paper`,
  ist aber Begriffskanon, kein Verdikt).
- **Fremd-Volltexte außerhalb des Genres:** `pioneer-anomaly-lrr-2010-4.txt`,
  `armstrong-woo-estabrook-1979-….txt`.

## Wahrheits-Schwelle (§2) — bestätigt

`omega_sh sha` stimmt bei allen geprüften Papers exakt mit dem Header-`sha256`;
alle Titel ≤75 arxiv-Zeichen, alle gelesenen Abstracts <200 Wörter; die
siehe-auch-Pfade lösen auf (außer unten); §2.2–2.6 tragen (stille Befunde sind
als Ergebnisse getragen, §3.3; Unsicherheiten ausgewiesen, §3.4; Ton ruhig, §3.5).

## Verstöße

| Paper | Stufe | Verstoß (file:line) |
|---|---|---|
| `jwst-disequilibrium-survey` | 2.8 | `:7` Kopf-Verweis leer — kein Survey-/Handover-Träger |
| `jwst-disequilibrium-survey` | 2.9 | `:14`/`:39` „cannot reproduce/make it" |
| `planet-nine-kbo-residue` | 2.8 | Header ohne `see-also` |
| `planet-nine-kbo-residue` | 2.9 | `:54` `fallback`, `:105`/`:106` `must`, `:194` `expected` |
| `planet-nine-kbo-residue` | 2.10 | `:113` nennt `tools/work/src/bin/te_ground_truth.rs` (real `tools/measure`) |
| `lead-geometry-direction` | 2.10 | `:206` nennt `tools/work/src/bin/mitdb_sweep_probe.rs` (real `tools/measure`) |
| `lead-geometry-direction` | 2.9 | `:16`/`:38`/`:135`/`:173` „cannot …" |
| `solar-cycle-dynamo` | 2.10 | `:523–531` nennt `tools/work/src/bin/*` (real `tools/harvest`/`tools/measure`) |
| `solar-cycle-dynamo` | 2.9 | `:53` `failed`, `:44` `should`, `:366` `expected`, `:388`/`:400`/`:459` `cannot` |
| `flyby-path-2-falsification-metric-addendum` | 2.7/3.2 | `:18` nennt den Pre-Reg-Header-sha `502e06c3…`; der Baum trägt `0ed15f81…` |
| `flyby-path-2-falsification-metric-addendum` | 2.9 | `:140` `failed`, `:66`/`:74` `cannot` |
| `flyby-path-2-preregistration-revised` | 2.9 | `:102` „Expected physics:" |
| `flyby-path-2-addendum-2026-09-29` | 2.9 | `:204` „cannot be applied" |
| `gic-causal-driver` | 2.9 | `:179`/`:182`/`:183` `must …`, `:413` `failed`, `:116`/`:138`/`:244`/`:379`/`:465`/`:578`/`:691` `expected`, `:60` `cannot` |
| `broken-null-control` | 2.9 | `:58` `must not`, `:92` `must pass`, `:191` `expected` |
| `corona-heating-ladder` | 2.9 | `:365` `failed`, `:525` `must`, `:30`/`:287` `cannot` |
| `causal-arrow-preregistration` | 2.9 | `:30` `cannot` |
| `probe-front-dark-matter` | 2.9 | `:122`/`:300`/`:580` `cannot`, `:86`–`:700` `must`, `:63`–`:761` `error` |
| `twenty-second-band-ground-chain` | 2.9 | `:262` `error` |
| `uranus-rift-ephemerides` | 2.9 | `:36` `must`, `:13`/`:23` `error` |
| `text-as-data-pioneer` | 2.9 | `:15`/`:43` `expected` |
| `blatt-kreuz-screening-gyirong` | 2.8 | kein `see-also` |
| `exzellenz-konzept` | 2.7 | `:20`/`:189` „15 Papers" gegen 38 `docs/paper/*.md` (im selben Atom geheilt) |

## Träger der Verstöße (am lebenden Bestand gemessen)

- **River (eigen, nicht fremd):** `lead-geometry-direction` (2.10 geheilt 2026-10-03;
  2.9 offen), `gic-causal-driver`, `corona-heating-ladder`,
  `flyby-path-2-preregistration-revised`, `flyby-path-2-addendum`,
  `flyby-path-2-falsification-metric-addendum` — `handover-2026-09-29-river-folge65.md:55-59`
  (Rivers Papiere) + die Flyby-Kette.
- **Sensory:** `jwst-disequilibrium-survey` (`handover-2026-10-03-sensory-folge224.md:295`),
  `planet-nine-kbo-residue` (descoped, `:297`), `blatt-kreuz-screening-gyirong` (`:266`),
  `broken-null-control` (`:299`), `causal-arrow-preregistration` (`:257`),
  `probe-front-dark-matter` (`:301`), `uranus-rift-ephemerides` (`axiom-gate-*`,
  sensory-folge168–170).
- **Mountain (Zuordnung über see-also/Archiv, am lebenden Träger nicht direkt gemessen):**
  `solar-cycle-dynamo` (see-also `corona-heating-ladder`; forschung-folge134),
  `twenty-second-band-ground-chain` und `text-as-data-pioneer` (Pioneer-Front,
  `handover-2026-10-03-mountain-folge227.md` Burn).
- **Geheilt 2026-10-03:** `flyby-path-2-falsification-metric-addendum` (2.7-Hash),
  `lead-geometry-direction` (2.10-Codepfad), `exzellenz-konzept` (2.7-Zähler) und die
  2.9-Sprachheilung in Rivers eigenen Papern (`lead-geometry-direction`, `gic-causal-driver`,
  `corona-heating-ladder`, `flyby-path-2-preregistration-revised`, `flyby-path-2-addendum`,
  `flyby-path-2-falsification-metric-addendum`; Header-shas neu gesetzt).
- **Adressiert:** die Verstöße reisen als `## An sensory` / `## An mountain`-Zeilen
  in `handover-2026-10-03-river-folge84.md`.

## Verdikt — geheilt am 2026-10-03

Alle oben benannten Verstöße wurden noch am 2026-10-03 in ihren Träger-Linien geheilt:
River seine eigenen (`lead-geometry-direction`, `gic-causal-driver`,
`corona-heating-ladder`, `flyby-path-2-preregistration-revised`, die zwei Flyby-Addenda,
plus die 2.7-Hash- und 2.10-Pfad-Risse), Sensory die seinen (2.8 `see-also`, 2.10
`planet-nine`-Pfad, 2.9 in sechs Papern; `planet-nine-kbo-residue` bleibt `descoped`),
Mountain die seinen (`solar-cycle-dynamo`, `twenty-second-band-ground-chain`,
`text-as-data-pioneer`). Verifikation am Baum (2026-10-03): `sgrep` findet
`cannot`/`failed`/`expected`/`should`/`must` nicht mehr; verbleibende `error`-Treffer sind
Messgrößen (`error bars`, `digitization error`). Alle Header-`sha256` sind nachgezogen.

Kein offener Punkt aus diesem Gate. Jedes neue Paper läuft vor Veröffentlichung erneut
durch §2–§3 — der Maßstab bleibt lebend.
