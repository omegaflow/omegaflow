<!--
  title: Status-Audit 2026-10-21 — vier Zustände, der Riss-Dreischritt, stille Lücken
  class: survey
  date: 2026-10-21
  sha256: 9ad0d77dafcb7e429db411a86d1e775793bef681c292d2d997289b516c2426df
  status: live
  owner: future
  ack: pending — Operator
  next: die zehn Riss-Kandidaten itemweise prüfen; stille Lücken in `pending` überführen; Operator schließt
  orphan-after: none
  see-also: docs/concepts/messzustaende.md docs/surveys/survey-2026-10-21-riss-zensus-nach-test.md docs/surveys/survey-2026-10-21-offen-ungebaut.md docs/concepts/zeugnis.md
-->
# Status-Audit 2026-10-21 — vier Zustände, der Riss-Dreischritt, stille Lücken

**Frage.** Werden `null-echt`, `absent`, `riss` und `pending` in den nicht
archivierten Dokumenten und in den Laufzeitmeldungen sauber getrennt — und wo
steht Offenheit ohne `pending`?

**Grenze.** Dieses Audit ändert **Dokumente** in `docs/surveys` und
`docs/concepts`, keine Laufzeitmeldungen und keinen Code. Die Papiere
(`docs/paper`), Blätter (`docs/blatt`) und Specs (`docs/specs`) werden geprüft,
aber nicht umgeschrieben; ihre Riss-Kandidaten bleiben dort als Verdacht
stehen. Geschlossen wird nur vom Operator.

## 1 · Die Regel

- **`null-echt`** — gemessen, **0 über n**; n wird genannt. Eine definitionale
  Null (Punktquellen-`bin_width`, Pad) ist keine Messung und heißt `0 honored`,
  nicht `null-echt`.
- **`absent`** — Nicht-Existenz ist belegt (Reichweite, Instrument, Auswahl).
  Nicht „nicht gebaut“, nicht „nicht gefunden“, nicht „nicht zugänglich“.
- **`riss`** — nur nach drei Schritten: (1) **Vergleichbarkeit** (dieselbe
  Sache, Rahmen, Epoche, Einheit, Kadenz, Auswahl, Methode), (2) **Kettenfehler
  ausgeschlossen** (Parser, Einheit, Zeitskala, Interpolation, Aggregation,
  Version, Auswahl), (3) **Daten wirklich verschieden** jenseits der
  Unsicherheit. Fehlt ein Schritt: `pending` mit dem Verdacht im Text.
- **`pending`** — nicht gemessen; offen. `pending ⇒ offen`; `offen ⇏ pending`:
  stille Lücken (offen ohne Token) und „geliefert, nicht geschlossen“ sind
  ebenfalls offen.
- **HTTP.** `pending — … HTTP 4xx/5xx` bleibt `pending`; der HTTP-Status ist der
  Verdacht, keine Messung. Diese Meldungen sind **korrekt** und werden nicht
  angefasst.

## 2 · Messung der nicht archivierten Treffer

`git grep` (literal, ohne `**/archiv/**`), Stand 2026-10-21 nach den
Korrekturen. „Dateien“ = Dateien mit mindestens einem Vorkommen; „Vorkommen“ =
alle Treffer.

| Zustand | Dateien | Vorkommen |
|---|---:|---:|
| `null-echt` | 13 | 26 |
| `absent` | 100 | 378 |
| `riss` | 38 | 129 |
| `pending` | 161 | 795 |

Nach Verzeichnis (Dateien):

| Verzeichnis | `null-echt` | `absent` | `riss` | `pending` |
|---|---:|---:|---:|---:|
| `docs/blatt` | 0 | 5 | 2 | 14 |
| `docs/concepts` | 5 | 15 | 10 | 27 |
| `docs/paper` | 0 | 22 | 9 | 22 |
| `docs/reference` | 0 | 2 | 0 | 4 |
| `docs/specs` | 2 | 8 | 2 | 18 |
| `docs/surveys` | 6 | 48 | 15 | 76 |

Die Wurzeldateien `docs/*.md` (`SOURCE_PORT.md`, `granit.md`) liegen außerhalb
des Glob `docs/**/*.md`; sie tragen `absent` 1× und `pending` 2×.

Der Arbeitsbaum `omegaflow-work` (kein `docs/`; `*.md` ohne
`research/models/raw/**`) trägt: `null-echt` **12** Dateien / 93 Vorkommen,
`absent` **73** / 293, `riss` **30** / 103, `pending` **116** / 829.

**Zur Zahl 22/30.** Die work-Zahl **30** ist mit `git grep -l riss` über
`*.md` ohne `research/models/raw/**` reproduzierbar. Die main-Zahl **22** ist
es nicht: derselbe Lauf findet heute **38** Dokumente (26 mit `riss` als
ganzem Wort; 15 davon statusartig, 11 in `surveys`/`concepts`). Die Abweichung
ist selbst ein Befund; dieses Audit prüft die **statusartigen** Fälle und die
zehn Cluster des Riss-Zensus, nicht jedes Wortvorkommen (Dateinamen,
`Grundriss`, `larissa`, `riss_knoten_probe`, Definitionen).

## 3 · Riss-Prüfung gegen die drei Schritte

„nein“ heißt: der Schritt ist **nicht** erfüllt; dann ist der Fall `pending`.
`—` heißt: nicht anwendbar, weil der Fall schon an Schritt 1 scheitert.

| Fall (Datei) | (1) vergleichbar? | (2) Kette ausgeschlossen? | (3) Daten verschieden? | Ergebnis |
|---|---|---|---|---|
| GIC-Kausaltreiber (`docs/paper/gic-causal-driver.md`) | nein — yearly-round vs. hardened-quarterly, zwei Estimator | nein | offen | **`pending`** (Verdacht: Estimator) |
| GIC-Breitenband (`docs/blatt/blatt-2026-10-06-gic-breitenband-familien.md`) | teilweise — 133 Stationen, globale vs. Familien-FDR | nein — Methode/Provenienz | offen | **`pending`** |
| GALAH (`survey-2026-10-16-taucherschwarm-quellen-funde.md`) | ja | nein — `ra_dr2`/`dec_dr2` vs. TAP/Schema/Parser | offen | **`pending`** |
| Auger (`ebd.`) | ja | nein — EeV vs. eV | nein | **`pending`** |
| CDN-404 (`ebd.`) | ja | nein — Transport | offen | **`pending`** |
| MODVOLC/OpenTopography/MAGION (`survey-2026-10-15-riss-alternativen.md`) | nein — Lizenz/Namensraum | — | — | **`pending`** (Metadaten) |
| WINDMI/MOSWOC (`survey-2026-10-16-gap-zensus-riss-pending.md`) | nein — Modell vs. Messung; restricted | — | — | **`pending`** (Definition/Zugang) |
| Flyby-2 (`docs/paper/flyby-path-2-addendum-2026-09-29.md`) | nein — ungeeichter Bogen vs. Gate | nein — Kalibrierung | — | **`pending`** |
| v2-Sandbox-Filter (`survey-2026-10-16-gap-zensus-riss-pending.md`) | nein — Codeverhalten vs. Erwartung | nein — Code | — | **`pending`** |
| Kanal-Ontologie (`docs/concepts/kanal-ontologie-komplettbau.md`) | ja — Definitionen | ja | ja, **semantisch** | **Begriffsriss** (bleibt; kein Datenriss) |
| ABK: INTERMAGNET-Boden vs. SWARM-Überflug (`docs/concepts/die-weberin.md`) | nein — Instrument, Höhe, 1,98° Versatz | nein | — | **`pending`** (korrigiert) |
| Hubble 67 vs. 73 (`ebd.`) | teilweise | nein — zwei Ketten/Kalibrierung | offen | **`pending`** (korrigiert) |
| TE: KDE vs. KSG (`docs/concepts/archivar-mathematikerin.md`) | ja — eine Größe | nein — zwei Estimator-Familien | offen | **`pending`** (korrigiert) |
| Kanal/Katalog: Index als `quantity` vs. `em` (`ebd.`) | ja — Definition | ja — Entscheid | ja, **semantisch** | **Begriffsriss** (korrigiert benannt) |
| Köder-Konflikt (`docs/concepts/koeder-formen.md`) | nein — Konflikt, keine Messung | — | — | **`pending`** (korrigiert) |
| CGrav/CEIC gated (`survey-2026-10-15-riss-alternativen.md`) | nein — Zugang gated | — | — | **`pending`** (korrigiert) |
| Mantis-Modulzahl 34 vs. 35 (`docs/specs/mantis-shrimp-build.md`) | nein — Dokumentzählung/Version | offen | — | **`pending`** (Verdacht: Zählfehler) |
| DE/INPOP/EPM, UTC/TAI, 1m/1h (`r 0,9388`), ICRF3/Gaia-CRF3 | nein — Modell/Skala/Kadenz/Epoche | — | — | **kein Riss**, `pending` |
| 173 Code-Zeilen `pending — … HTTP` (Auftrag: 178 Meldungen) | nein — keine Messung | — | — | **`pending`** (korrekt, unverändert) |
| Phantom-Island, exakte Koordinaten | nein — nicht gemessen | — | — | **`pending`** (geliefert-offen) |

**Bilanz.** Von den geprüften Fällen besteht **kein Datenriss** den Test. Zwei
Fälle bleiben als **Begriffsriss** (Kanal-Ontologie; Index `quantity` vs.
`em`) — dort widersprechen sich Definitionen, nicht Messungen. Alles andere
ist `pending` mit benanntem Verdacht. Die zehn Cluster des
`survey-2026-10-21-riss-zensus-nach-test.md` (9 `pending`, 1 Begriffsriss)
sind damit bestätigt.

**Die 30 work-Dateien.** Statusartige Riss-Behauptungen stehen in
`research/gic/notes/2026-09-27-gic-againstlesung.md`,
`…-09-30-gic-againstlesung.md` und `projects/gic.md` — derselbe
GIC-Estimator-Fall (yearly vs. hardened quarterly) → **`pending`**. Die
übrigen sind Dateinamen (`gic-riss-154…`), Definitionen
(`rat/2026-10-21-rat-messzustaende*.md`), Code-/Strukturmetaphern
(`rat/2026-10-20-rat-ordnerstruktur.md`: Provenienz-/Schwellen-/Denylist-Riss)
oder Rohnotizen — kein Datenriss. Editiert wurde dort nichts; der Arbeitsbaum
liegt außerhalb dieses Repos.

## 4 · `absent` für „nicht gebaut“ → `pending`

| Datei | Stelle | Änderung |
|---|---|---|
| `docs/concepts/zeugnis.md` | §11, §13, §14 (3×) | „unbekannter Schwellenwert / fehlender Wert / ungebauter Name“ → `pending`; `absent` nur bei belegter Nicht-Existenz |
| `docs/concepts/_template.md` | „Der Riss“ | Default „was fehlt“ → `pending`; `absent` nur bei belegter Nicht-Existenz |
| `docs/surveys/survey-2026-10-20-keyless-blöcke.md` | „Was quer liegt“ | MEEO-NetCDF-Felder → `pending`, bis Objekt-Fetch/Compiler gebaut ist |

**Geprüft und bewusst `absent` gelassen:** Geräte-Inventar
(`survey-2026-09-23-geraete-anbindung-radiatoren.md`: Hardware nachweislich
nicht vorhanden), H₀-Linien (`blatt-2026-09-09-h0-linien-register.md`: BAO
allein/PTA-Parallaxe tragen kein H₀), `tools-map` („kein Eintrag“ = Quelle
trägt den Wert nicht), Sieben-Sphären (Δz in **n = 2** geprüften Katalogen
nicht vorhanden), Hunga-Tonga („Daten nicht absent“ bei Key-Fehler).

**Sonderfall, kein `absent`-für-Bau:** Mars-96 stand als `null-echt` im
Sonden-Zensus. Es ist keine gemessene 0, sondern belegte Nicht-Existenz
(Startfehler 1996) → **`absent`**.

## 5 · `null-echt` ohne n

| Datei | Stelle | Befund / Änderung |
|---|---|---|
| `docs/surveys/survey-2026-09-16-sonden-flotte.md` | Mars-96 | war `null-echt` ohne n; keine Messung → **`absent`** |
| `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` | `bin_width=0.0` | Punktquelle: 0 **gesetzt**, nicht gemessen → `0 honored`, nicht `null-echt` |
| `docs/concepts/archivar-mathematikerin.md` | Punktquellen-Extent / `bin_width` | definitionale Null → **`0 honored`** (nicht Messung) |
| `docs/concepts/glossar.md` | 0-Kanon | `null-echt` = **0 über n gemessen, n genannt** |
| `docs/concepts/kybernetische-ethik.md` | 0-Kanon | `null-echt` = **0 over n measured, n named** |
| `docs/surveys/survey-2026-10-21-messzustaende-wissenschaft.md` | Definition + Synthese | **0 über n** ergänzt |

**Schon korrekt:** `docs/concepts/messzustaende.md` („0 über n gemessen“),
`survey-2026-10-20-asi-stationshoehen.md` (PGM `0.0` nominal, 17 Stationen).
**Definitional, kein n nötig:** `ausrede-untersuchung.md`,
`fuenf-funken-anomalie-suche.md`, `koeder-formen.md`, `spectral-oscillator.md`
(Spec, außerhalb der Korrekturfläche), das Perplexity-Zitat in
`survey-2026-10-08-fmhy-research-landscape.md`.

## 6 · Stille Lücken (offen ohne `pending`)

| Stelle | Befund | Behandlung |
|---|---|---|
| `survey-2026-10-21-offen-ungebaut.md` §1 | „acht Key-Arme + vier Sonden erneut absuchen“ = nicht gebaut, ohne Token | jetzt `pending` benannt |
| `ebd.` §3 | fünf Funken + Klasse 3 = ungebaut, ohne Token | jetzt `pending` benannt |
| `survey-2026-10-15-riss-alternativen.md` | „Neu zu registrieren (offen)“ | jetzt `pending` benannt |
| `survey-2026-10-21-riss-zensus-nach-test.md` | „104 Dokumente itemweise prüfen“ | jetzt `pending` benannt |
| `ebd.` | **33 orphan-Docs** (offene Marker ohne Träger) | `pending`; nicht itemweise geprüft |
| `survey-2026-10-16-gap-zensus-riss-pending.md` | private Register (`state/*`, `phi/pipeline/*.φ`) nicht im Baum | `pending` — nicht gemessen |
| Auftrag `projects/archive-search-faehigkeiten.md` | Punkt I selbst | `pending`, bis dieser Commit + Review |

## 7 · „Geliefert, nicht geschlossen“

- **`phantom-island`** (`survey-2026-10-21-phantom-island.md`): vier Felder
  geliefert; die exakten Inselkoordinaten bleiben `pending`. Geliefert ≠
  geschlossen.
- **`archive_search` A–H**: gebaut und getestet (zuletzt 743 Tests), aber nicht
  vom Operator geschlossen; Punkt I schließt sie nicht mit.
- **Riss-Alternativen**: die offenen Ersatzwege sind gemessen, aber vier
  Blöcke sind noch nicht registriert (`pending`).
- **Riss-Zensus**: die zehn Cluster sind geprüft; neun `pending`, einer
  Begriffsriss. Die Prüfung ersetzt keine Schließung.

## 8 · Fix-Liste

| Datei | Änderung |
|---|---|
| `docs/surveys/survey-2026-10-21-status-audit.md` | **neu** — dieses Audit |
| `docs/concepts/zeugnis.md` | 4× `absent`/„kein pending“ für Ungebautes → `pending` |
| `docs/concepts/_template.md` | Default `absent` → `pending` |
| `docs/concepts/die-weberin.md` | 2 Riss-Behauptungen (ABK/SWARM, Hubble) → `pending` mit Verdacht |
| `docs/concepts/koeder-formen.md` | `riss: null-echt` → `conflict`/`pending`; fünf „Risse“ als Kandidaten |
| `docs/concepts/archivar-mathematikerin.md` | Estimator-Riss → `pending`; definitionale Nullen → `0 honored`; Index-Riss → Begriffsriss |
| `docs/concepts/kybernetische-ethik.md` | `null-echt` mit n; `riss` mit Dreischritt |
| `docs/concepts/glossar.md` | `null-echt` mit n; `riss` nach Dreischritt |
| `docs/concepts/the-seven-spheres.md` | `absent` mit n = 2 Katalogen |
| `docs/surveys/survey-2026-09-16-sonden-flotte.md` | Mars-96 `null-echt` → `absent` |
| `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` | `bin_width`-`null-echt` → `0 honored` (Definition) |
| `docs/surveys/survey-2026-10-15-riss-alternativen.md` | `riss`/`pending` → `pending`; „offen“ → `pending` |
| `docs/surveys/survey-2026-10-16-gap-zensus-riss-pending.md` | Dreischritt-Nachtrag; `absent`-Definition; „Riss-Register“ → Kandidaten |
| `docs/surveys/survey-2026-10-20-keyless-blöcke.md` | `absent` für Ungebautes → `pending` |
| `docs/surveys/survey-2026-10-21-messzustaende-wissenschaft.md` | `null-echt` mit „0 über n“ |
| `docs/surveys/survey-2026-10-21-offen-ungebaut.md` | Befund 1/3 korrigiert; Nachmessung; stille Lücken → `pending` |
| `docs/surveys/survey-2026-10-21-riss-zensus-nach-test.md` | 104-Dokumente-Prüfung als `pending` benannt |

## 9 · Grenzen

- **Zählbasis.** Die 38 main-Dokumente mit `riss` enthalten Definitionen,
  Dateinamen und Prosa; nur 15 sind statusartig. Die 22 aus dem Auftrag ist
  nicht reproduzierbar; die 30 im work-Baum ist es.
- **Definitionen.** Ein `null-echt` in einer Definition („der Wert IST 0“)
  nennt kein n; erst die Messaussage braucht n. Solche Stellen sind oben
  einzeln ausgewiesen.
- **HTTP.** Die Code-Meldungen wurden nicht geändert: 173 literale
  `pending — … HTTP`-Zeilen in `tools/utils/src/bin/archive_search/**`
  (Auftrag: 178 Meldungen; Zählweise nicht identisch). Alle bleiben `pending`.
- **Nicht geprüft.** Jedes einzelne der 38/30 Dokumente itemweise; die
  Papiere/Blätter/Specs wurden nur auf statusartige Riss-Behauptungen
  angesehen. Die 33 Orphans und die privaten Register bleiben `pending`.

## Nächster Akt

1. Die `pending`-Riss-Kandidaten (GIC, Flyby, Tauchbefunde, ABK/SWARM, Hubble,
   KDE/KSG) je auf ihren Kettenfehler prüfen — erst dann darf `riss` wieder
   stehen.
2. Die 33 Orphan-Docs und die 104 Kandidaten itemweise prüfen.
3. Der Operator schließt; dieses Audit schließt nichts.
