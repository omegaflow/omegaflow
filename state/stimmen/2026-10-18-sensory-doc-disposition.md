<!--
  title: Sensory — Besitz-Disposition der 16 Docs (2026-10-18)
  class: ref
  date: 2026-10-18
  sha256: 1a79776dbb56c0eb276d4a4ce9526b2cc6b3b6df2ed90e1ea171284dcf063d88
  status: live
  see-also: docs/handover/handover-2026-10-14-sensory-folge259.md state/stimmen/2026-10-18-orphan-zuordnung.md
-->
# Sensory — Besitz-Disposition der 16 Docs (2026-10-18)

Grund: `b729a7f97` wies der Linie 16 Docs zu. Besitz ist kein Messauftrag; die
Zuordnung verlangt je Doc eine Disposition — schließen, delegieren oder mit
Begründung abgeben. `register_lookup --open` zählt Marker heuristisch (auch
Tabellen-/Prosa-Zeilen); die Linie entscheidet, was ein Punkt ist.

## Geschlossen (belegt)

| Doc | Beleg |
|---|---|
| `survey-2026-09-16-dead-sources-relevanz.md` | 4 Pending-Quellen gemessen; Commit `5e92f1e89` |
| `survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` | drei Arme gebaut: `parquet_reader.rs`, `grib2_reader.rs`, `opendap_reader.rs` + `src/archivar/{parquet,grib2,opendap}.rs` |
| `survey-2026-09-16-sonden-flotte.md` | MAVEN registriert (`sources.φ:19111`); nativer Serien-Arm gebaut (`tnf_compiler.rs:71`, PODF) |
| `survey-2026-10-08-open-sources-delta.md` (3 Reste) | SSDC Limadou NXDOMAIN/kein Ledger; LiteBIRD 301 → JAXA/keine Datenroute; CSES TLS abgelaufen, Route via `--jina` **200** / `-k` **200** |
| `survey-2026-10-08-fmhy-research-landscape.md` | MiMo NXDOMAIN; Jina-Arm gebaut; SearXNG-Arm gebaut |
| `survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` | Apple Look Around NXDOMAIN; NRCan canimage 404; Bing Streetside kein API-Doc |
| `survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (Gravimetrie) | EQUIP-G 000/unregistriert; IGETS + BGI/SEDOO registriert/live |

## Offen — mechanisch/Operator (benannt)

| Doc | Punkt | Zustand |
|---|---|---|
| `open-sources-delta` | Teilchen-/Dark-Matter-Experiment | Operator-Wort (Kuprat vs. Detektor) |
| `open-sources-delta` | Re-Audit-Zeilen 126–132 | historisch; durch „Blockierte vier — GEKNACKT" überholt |
| `stoerungs-experiment` | CTBTO VDEC | Operator-Webform; root 404, request 403, unregistriert |
| `fmhy` | SearXNG-Instanz | Arm gebaut; Instanzwahl offen |
| `sonden-flotte` | request-only-Zeilen 77/161/162 | Historie, kein Messpunkt |

## Abgegeben (Grund)

| Doc | Empfänger | Grund |
|---|---|---|
| `daten-holdings-inventur` | future/Operator | Ziel-Layout-Wort steht; Byte-Messung/Migration ist Operator |
| `grosse-raetsel-und-maschine` | future | „Aufnahme-Wort" ist Operator |
| `geraete-anbindung-radiatoren` | Operator/Hardware | physische Sensoren ungemessen |
| `die-fuenf-archaeologie` | mountain | Code-Konstanten/Axiome, kein Sensor |
| `tauchbefunde-acht-faeden` | river/mountain | force-Keyword-Entscheid |
| `verlorene-diskussionen` | mountain/code | historische Bindings |

## Geroutet (Token + Brief, 2026-10-18)

| Token | An | Brief |
|---|---|---|
| `xfer:sensory-259…10~67b095df` | future | `post/in/2026-10-18-sensory-an-future-offene-entscheide.md` — 5 Entscheide |
| `xfer:sensory-259…11~22e38677` | mountain | `post/in/2026-10-18-sensory-an-mountain-zwei-docs.md` — 2 Docs |
| `xfer:sensory-259…12~a3bf3c25` | river | `post/in/2026-10-18-sensory-an-river-force-keyword.md` — force-Keyword |
| `xfer:sensory-259…13~99e00470` | mycelium | `post/in/2026-10-18-sensory-an-mycelium-searxng-instanz.md` — SearXNG-Instanz |
| `xfer:sensory-259…14~dfc6aa3a` | future | `post/in/2026-10-18-sensory-an-future-beobachtungen.md` — Meta-/Prosa, Command-Risse, ARVO, Working Tree, Gate |

## Kein Punkt (Meta/Prosa) — an future geroutet

`asservatenkammer-zensus` (Meta-Tabelle), `survey-raetsel-bestand` (Inventar),
`das-eine-instrument` (Konzept), `ehrlich-benannt:13` (Klassifikationssatz),
`sonden-flotte:77/161/162` (request-only-Historie). Diese Beobachtung steht nicht mehr
nur hier: Token `14` an future, Brief `post/in/2026-10-18-sensory-an-future-beobachtungen.md`.

## Linien-Punkte (nicht Docs)

- `ox64-m2c`: DHL 17.10. 05:57 „in der Region des Empfängers", Fortschritt 3/5 —
  nicht zugestellt.
- `europa-clipper`: Termin 03.12.2026; der Schritt ist ausdrücklich nach dem Flyby.
- `xfer:sensory-259…9~1da7eb85` an future: `SENT`/`unbeglichen` (kein future-Handover
  im Checkout).
