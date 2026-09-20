<!--
  title: Auftrag — Flyby-Path-2-Kette vor dem 28.09.2026 (JUICE-Perigäum)
  class: auftrag
  date: 2026-09-20
  sha256: 3df49a1c25c84939796fc0f49d61c927325cccb04b327052aa4f2cd3112bdf2e
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md docs/paper/flyby-path-2-falsification-metric-addendum.md
-->
# Auftrag: die Flyby-Path-2-Kette vor dem 28.09.2026

## Zweck

Die präregistrierte Kette der JUICE-Erdpassage (28./29.09.2026) wird
transit-time-korrigiert am Perigäum-Tubus gefüllt und als Auftrag getragen —
**vor** dem Ereignis; nach dem Flyby wäre die Vorhersage post-hoc. Das Siegel
(Trajektorie-Hash, Vorhersageform) und die σ-Metrik stehen; offen ist die Kette
selbst. Dieser Auftrag ist die Registrierung dieses offenen Punkts, nicht das
Füllen (das Füllen braucht die gemessene Plasmakette).

## Was steht (gemessen 2026-09-20)

- **Siegel:** `flyby-path-2-preregistration.md:21,23` — Trajektorie
  `ephemeris_juice.bin` sha256 `aeb3c82f…` (JUICE), `dae553fb…` (Europa Clipper);
  Vorhersageform **nur Feldzustand, keine mm/s-Zahl** (`:26-34`).
- **σ-Metrik:** `flyby-path-2-falsification-metric-addendum.md:13,67-73` —
  normalisiertes Kanal-Residuum `RMS (measured − pre-registered)/σ`, Schwelle
  `fam`; Operator-Siegelzeile 2026-09-03 (`:52`). Das Original-Siegel bleibt
  unangetastet (`:18`).
- **Transit-Korrektur:** RTSW an L1 führt um die L1-Transitzeit; Kp 3-h; Swarm am
  Ort (`preregistration.md:32-33`, `addendum:65`). Die Kette läuft einseitig
  upstream → Tube (`addendum:38-44`).

## Die Kanäle — Ernte-Zustand (gemessen 2026-09-20)

| Kanal | Ort in `phi/sources.φ` | Zustand |
|---|---|---|
| RTSW (bz_gsm, speed, density, temp) | `158-169` | registriert (live) |
| Kp (NOAA planetary) | `179-183`; GFZ `1336-1339` | registriert (live) |
| Swarm (EFIA-LP, FACATMS, MAGA-LR) | `6059-6078` | registriert (live) |
| OMNI2 (Plasmadruck) | `918-928` | registriert (live) |
| ACE | `1200-1213` | registriert (live) |
| WIND | `3013-3028` | nur Orbit/Waves — **kein Plasma** |
| DSCOVR | — | **fehlt** (kein Eintrag) |
| JUICE in-situ | — | erst **nach** dem Flyby |

`phi/pipeline/ledger.φ` trägt **keinen** Ernte-Zustand der Kette (kein
flyby/rtsw/swarm/kp/omni-Eintrag) — die Kanäle sind Live-`url`-Zeilen, nicht als
`verifiziert`/`kompiliert` geführt.

## Lieferung

1. Die lebenden Kanalzellen transit-time-korrigiert am Perigäum-Tubus füllen;
   jede Zelle ohne Messung bleibt `pending`, nie `0.0` (0 honored).
2. Das Ergebnis als Addendum zum Siegel committen (σ-Metrik angewandt).
3. `DSCOVR` als Kanal prüfen und, falls tragend, in `phi/sources.φ` registrieren.

## Frist / Owner / nächster Schritt

- **Frist:** hart vor dem 28.09.2026 (Perigäum); nach dem Ereignis ist die
  Vorhersage post-hoc.
- **Owner:** Forschung-Linie; Kette füllen `research-max`.
- **Nächster Schritt:** die Kanalzellen gegen die registrierten Quellen messen
  (`archive_search`/`sfetch` auf RTSW/Kp/Swarm/OMNI2), Transit-Korrektur
  anwenden, `pending`-Zellen benennen.

## Abschluss

Addendum mit gefüllter Kette + angewandter σ-Metrik vor dem 28.09. committed =
Frist gehalten; sonst läuft die Kette als ungemessene offene Zeile weiter
(`pending`), nie als Zahl.
