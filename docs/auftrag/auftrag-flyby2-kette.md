<!--
  title: Auftrag — Flyby-Path-2-Kette vor dem 28.09.2026 (JUICE-Perigäum)
  class: auftrag
  date: 2026-09-20
  sha256: 644ea7554e9d01e12056a2659778a84b85ed59b8642826b9c386cddee5cc0606
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
| DSCOVR | — | eigene keyless JSON-Route **retired** (404); L1 lebt im RTSW-Feed |
| JUICE in-situ | — | erst **nach** dem Flyby |

`phi/pipeline/ledger.φ` trägt **keinen** Ernte-Zustand der Kette (kein
flyby/rtsw/swarm/kp/omni-Eintrag) — die Kanäle sind Live-`url`-Zeilen, nicht als
`verifiziert`/`kompiliert` geführt.

## Lieferung

1. **Bereitschaft gemessen 2026-09-20** (Addendum §"Chain readiness"): alle
   Kanalrouten HTTP 200 (stage 1), Selektoren gegen die File-Ordnung geprüft,
   Transit-Methode benannt (Lichtzeit 5,0 s; Advektion `d/v_sw`; Kp 3-h; Swarm am
   Ort). Keine Zellwerte vor dem Perigäum — jede Zelle bleibt `pending` (0 honored).
2. Das Ergebnis als Addendum zum Siegel committet (σ-Metrik angewandt).
3. `DSCOVR` geprüft (2026-09-20): die eigene keyless Route ist retired (404); der
   L1-Echtzeit-Solarwind lebt im registrierten RTSW-Feed (multi-source, `source`
   je Messwert). Keine neue `sources.φ`-Zeile — eine DSCOVR-Familie wäre ein
   Duplikat; `where source DSCOVR` bleibt `pending` Parser-Prüfung.

## Frist / Owner / nächster Schritt

- **Frist:** hart vor dem 28.09.2026 (Perigäum); nach dem Ereignis ist die
  Vorhersage post-hoc.
- **Owner:** Forschung-Linie; Kette füllen `research-max`.
- **Nächster Schritt:** die Zellen ab dem Perigäum aus den lebenden Kanälen
  füllen (RTSW/ACE Minuten, Kp ≤ 3 h, Swarm ≤ 1 d; OMNI2 ~1–2 d nur Verifikation;
  JUICE nach dem Flyby) und je Messwert `source`+`active` mitprotokollieren.

## Abschluss

Addendum mit gefüllter Kette + angewandter σ-Metrik vor dem 28.09. committed =
Frist gehalten; sonst läuft die Kette als ungemessene offene Zeile weiter
(`pending`), nie als Zahl.
