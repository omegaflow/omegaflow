<!--
  title: Archäologie des sources-Schemas (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: a6d7a6ff333bbe9f5123e55bf82b1138111a080bdbfc16e8f5511ad97ee8822d
  status: live
  see-also: docs/specs/sources-v2-spec.md docs/specs/vorlage-je-dateityp.md docs/specs/livefeed-gate.md phi/sources.φ
-->
# Archäologie des sources-Schemas

**Auftrag:** nicht ändern, sondern graben. Was ist das `sources.φ`-Schema
heute, welche Specs tragen es, wo widersprechen sie sich, und warum stehen
290 Messungen auf `on earth 0 0 0`? Kein Schema-Edit in diesem Zug.

## 0. Die drei Autoritäten (und eine vierte)

| Autorität | Datei | Was sie sagt | Was sie nicht sagt |
|---|---|---|---|
| **Format-Spec v2** | `docs/specs/sources-v2-spec.md` (656 Z.) | Direktiventabelle, Frame-Semantik, Drop-Regeln, τ-Gate, Migration; „verified against the living parser“ (`:1-11`) | `terms`, `origin`, `compiler`, `sha256` fehlen in der Tabelle; kein `frame`/`epoch` für den statischen Anker |
| **Dateityp-Vorlage** | `docs/specs/vorlage-je-dateityp.md` | Blockform mit `url`/`terms`/`format`/`origin`/`compiler`/`at`/`field`/`ttl`; Pflichtfelder (`:67-84`) | `on` fehlt in der Blockform; `at <body|earth|sun>` ist dort der einzige Anker |
| **Parser/Validator** | `src/archivar/parse.rs`, `tools/register/src/bin/source_validate.rs` | tatsächliche Arme: `at` (`:385`), `on` (`:393`); Validator: `missing_terms` hard (`:206`), `elev_zero` soft (`:428`), `anchor_missing` soft (`:533`) | keine Frame-/Epochen-/Coverage-Prüfung; `elev_zero` prüft den Frame-Anker, nicht die Zeilenposition |
| **Livefeed-Gate** | `docs/specs/livefeed-gate.md` | geo→ICRS mit `ISO-UTC` (WGS84 + GMST; Präzession/Nutation pending) (`:97-110`) | wird vom Register nicht benutzt |

Die vierte Autorität ist die **Praxis**: 2.861 Blöcke, 7.465 `field`-Zeilen,
`on earth 0 0 0` in 290 Blöcken.

## 1. Zeitleiste (gemessen)

- **2026-08-17** — SI-Konversion, Council-Ruling (`sources-v2-spec.md:589`).
- **2026-09-03** — `phi/`-Referenz (`docs/specs/ref-phi-register.md`).
- **2026-10-04/05** — `range`-Achse, zwei Ratsentscheidungen (`sources-v2-spec.md:75`).
- **2026-10-13…20** — Migrationen; `on earth` kommt am **2026-10-20** in die
  Blöcke (`git log -S 'on earth' -- phi/sources.φ`).
- **2026-10-18** — R-2026-10-18-b (GRS80/γ₀): neue Direktive vs. Block vs.
  local-only; `N` bleibt pending (`rat/entscheidungen.md:36`).
- **2026-10-21** — Lizenzmap, v3-Messmodell, `origin`-Prosa raus
  (`38fd6237a`), Anker-Höhen-Survey.

## 2. Was das Schema heute ist

**Blockgrammatik** (`sources-v2-spec.md:29-45`):

- `url` startet den Block und resettet allen Zustand; ein Block wird
  **refused** ohne `url` + `ttl` + Frame (`at`/`on`) (`:33`).
- Keine Kommentarzeilen; eine Leerzeile zwischen Blöcken; sortiert nach
  `ttl`, dann `url`.
- `at <body>` = baryzentrischer ICRS-Frame des Körpers (`:61`).
- `on <body> <lat> <lon> [alt]` = Oberflächenpunkt, WGCCRE-Rotation,
  `alt` in **Metern**, Default 0 = surface datum (`:45`).
- Datengetragene Position: `map`/`cmap`/`rows` + `lat`/`lon`/`alt` bzw.
  `ra`/`dec`/`plx`/`dist` + `epoch` (`:371-380`). Der Frame-Anker bleibt
  trotzdem Pflicht; er „gates fetching“ und rendert `{lat}`/`{lon}`.
- `field` 5/6/9 Token; 9-Token ist die Registerform; τ > 0 oder nichts
  manifestiert (`:29-120`).

**Validator** (`source_validate`):

- `missing_terms` ist **hard** (`:206`) — obwohl die v2-Tabelle `terms`
  nicht führt.
- `anchor_arity`, `anchor_range`, `anchor_body` sind hard (`:382-470`).
- `elev_zero` (`e == 0 && lat == 0 && lon == 0`) ist **soft** (`:426-435`).
- `anchor_missing` (Compiler + Observables, aber kein `at`/`on`) ist soft
  und wird ohne `--warnings` gar nicht gedruckt (`:531-547`).

## 3. Der Bock (gemessen, nicht vermutet)

290 Blöcke tragen `on earth 0 0 0`. Das ist **spec-valid** (Oberflächenpunkt
0/0/0, `alt` default 0), aber semantisch ein Platzhalter:

| Klasse | Beispiel | Was fehlt |
|---|---|---|
| globale Feeds | `earthquake.usgs.gov`, FIRMS, aviationweather | Zeilenposition (`lat`/`lon`) oder Coverage |
| Parameter-Queries | `api.open-meteo.com?latitude={lat}&longitude={lon}`, `{bbox}`, `{station}` | Position kommt erst im Aufruf |
| bewegte Plattformen | Drifter, Bojen, Argo-`profile` | Position pro Datensatz (`lat`/`lon`/`epoch`) |
| globale Mittel | `ch4_mm_gl`, `n2o_mm_gl`, `sf6_mm_gl` | gar keine Position — hängen falsch an Mauna Loa |
| Satelliten-Swath | ICESat-2, GEDI | Coverage/Swath, kein Punkt |

Die 276 Blöcke **mit** Koordinaten sind ein anderes Bild: 195 Bodenanker,
191 ≤ 300 m gegen Open-Meteo, 4 Ausreißer (Mauna Loa / globale Mittel),
76 API-Lücken (Ozean/Bojen), 5 Raumfahrt-Anker (> 9000 m). Die 2–4
Dezimalen sind Quellenpräzision, kein Formatter: die Compiler schreiben
`{}` auf `f64` (`themis_asi_timed_keogram_compiler.rs:327`,
`godas_pottmp_compiler.rs:204`). Der frühere Integer-Eindruck war ein
`awk +0` im Survey, nicht das Register (`survey-2026-10-21-anker-hoehen.md`,
Korrektur).

**Der eigentliche Widerspruch:** Die v2-Spec sagt, der Frame-Anker sei
nicht die Messposition — die Zeilen tragen sie (`:371-380`). Der Validator
prüft aber den **Frame-Anker** (`elev_zero`) und nicht, ob jede Zeile eine
Position hat. Damit ist `on earth 0 0 0` gleichzeitig „erlaubt“ und
„auffällig“, und die 290 Fälle werden als Höhenproblem missverstanden.

## 4. Die Widersprüche der Specs

1. **`terms`/`origin`/`compiler`:** `vorlage-je-dateityp.md:80-84` erklärt
   sie zu Pflichtfeldern; `sources-v2-spec.md` listet sie in der
   Direktiventabelle nicht; der Validator macht `terms` hard. Drei
   Autoritäten, drei Wahrheiten.
2. **`on`:** Die Vorlage kennt nur `at <body|earth|sun|…>`
   (`vorlage:75`); die v2-Spec definiert `on` als Oberflächenanker
   (`:45`); der Parser akzeptiert beides (`parse.rs:385,393`).
3. **Kommentare:** v2 sagt „No comment lines“ (`:43`); die
   `vorlage`-Zustandsdatei erlaubt Kopf-Kommentare — für `sources.φ` ist
   die Prosa zu Recht raus, aber `origin` braucht eine Definition.
4. **Frame/Epoche:** Die v2-Spec hat `at`/`on` und per-Zeile `epoch`
   (`:371-380`); einen expliziten Frame/`epoch` für den statischen Anker
   gibt es nicht. Das `livefeed-gate` kann geo→ICRS mit `ISO-UTC`
   (`:97-110`) — das Register nutzt es nicht.
5. **Einheiten:** v2 sagt `alt` in Metern (`:45`), die Migration soll
   km→m konvertieren (`:639`); CHUR `0.021` neben BOU `1682` zeigt, dass
   das nicht überall passiert ist. Kein Lade-Check.
6. **v3-Achsen:** Die Lizenzmap und das Messmodell (pending/LOCK/
   lebenszyklus/zugang) sind neuer als v2; `sources.φ` trägt `terms`, aber
   keine `zustand`/`lebenszyklus`/`zugang`-Felder. Die Achsen sind nicht
   im Schema verankert.
7. **Parser-Arme außerhalb der Tabelle:** `profile`, `quakeml`, `alerce`,
   `xmlcount`, `lastobj`, `lastline`, `lat_sign`, `lon_sign`,
   `epoch_scale` sind laut v2 §10 akzeptiert, aber nicht in §1
   dokumentiert (`:582-604`).
8. **Coverage/global/query/track:** Die v2-Spec kennt `map`/`cmap`/`rows`
   für Zeilenpositionen, aber keine Anker-Art für globale Mittel,
   BBox-Queries, Tracks oder Swaths. Genau diese Klassen sind die 290.

## 5. Was gut ist (nicht wegwerfen)

- Die v2-Spec ist ungewöhnlich präzise: Direktiventabelle, Force-Unit-
  Registry, Drop-Regeln, τ-Gate, Map/CMap/Rows, Migrationsalgorithmus.
- Die v3-Achsen (Messzustand/Lebenszyklus/Zugang) und die Lizenzmap sind
  eigenständig und belastbar.
- Der Parser trägt 2.861 Blöcke/7.465 Felder; die Krise ist die
  Semantik der Anker, nicht die Maschine.
- Die Anker-Höhen-Messung ist nach der Rohkoordinaten-Korrektur sauber:
  191/195 plausibel, 4 echte Ausreißer.

## 6. Offene Fragen (für Wissenschaft und Rat)

1. **Anker-Arten:** `at` (Körper) · `on` (fester Punkt) · `profile`/
   `track` (Position pro Datensatz) · `query` (Position im Aufruf) ·
   `global`/`coverage` (Fläche/Swath) — welche Arten, welche Namen?
2. **Frame/Epoche:** explizit (`frame icrs|body-fixed|wgs84`, `epoch`),
   oder aus `at`/`on` ableiten? TDB oder UTC? Welche Epoche für einen
   statischen Anker?
3. **Zeilenposition:** Muss jede manifestierende Zeile `lat`/`lon` oder
   `ra`/`dec` tragen? Was ist mit globalen Mitteln (keine Position)?
4. **Einheit:** `alt`/`elev` in Metern verpflichtend; Vorzeichen
   (Tiefe negativ)? Konversion beim Laden oder beim Schreiben?
5. **Pflichtfelder:** Gelten `terms`/`origin`/`compiler` (Vorlage +
   Validator) oder nur `url`+`ttl`+Frame (v2 + Parser)? Welche Autorität
   gewinnt?
6. **Achsen:** Werden `zustand`/`lebenszyklus`/`zugang` Teil des
   `sources`-Blocks oder bleiben sie in `state/wartend.φ`/Lizenzmap?
7. **Validator-Schwere:** Wird `on … 0 0 0` hard (ohne Anker-Art), wird
   fehlende Zeilenposition hard, bleibt `elev_zero` soft?
8. **Migration:** 290 Blöcke klassifizieren; 4 Mauna-Loa-Fälle; CHUR-
   Einheit; wer entscheidet die Klassen?
9. **Spec-Konsolidierung:** eine `sources`-Spec (v3) oder drei Specs
   (Format, Dateityp, Livefeed) mit klarer Rangfolge?
10. **Ratsbeschluss:** Der Rat entscheidet das **komplette Schema**, nicht
    nur den Anker — inkl. Pflichtfeldern, Anker-Arten, Frame/Epoche,
    Einheiten, Achsen und Validator.

## 7. Belege

- `docs/specs/sources-v2-spec.md:29-45,61,353-380,582-604,639`
- `docs/specs/vorlage-je-dateityp.md:67-84`
- `docs/specs/livefeed-gate.md:97-110`
- `src/archivar/parse.rs:385-446`
- `tools/register/src/bin/source_validate.rs:206,382-470,531-547`
- `phi/sources.φ` (290 × `on earth 0 0 0`; 276 mit Koordinaten)
- `docs/surveys/survey-2026-10-21-anker-hoehen.md` (korrigierte Messung)
- Commits: `f3f36a135` (Vorschlagsmarker), `38fd6237a` (Prosa raus),
  `3b36bd855` (wartend_validate), `332805d35`/`a1409af9d` (`on earth`).
