<!--
  title: Riss-Zensus nach dem Test — zehn Cluster, ein Begriffsriss, kein Datenriss (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 9804de4853a367ffc55b7242ce0a030b377ac894e552e248edb6d457ecad2bb9
  status: live
  owner: future
  ack: future 2026-10-21T13:10Z
  next: die neun pending-Fälle je auf ihren Kettenfehler prüfen; kein Etikett ohne Test
  orphan-after: none
  see-also: docs/surveys/survey-2026-10-16-gap-zensus-riss-pending.md docs/concepts/glossar.md
-->
# Riss-Zensus nach dem Test — zehn Cluster, ein Begriffsriss, kein Datenriss

**Frage.** Welche der im Gap-Zensus (`survey-2026-10-16-gap-zensus-riss-pending.md`)
als „Riss" geführten Fälle bestehen den Test: **vergleichbar**, **Kettenfehler
ausgeschlossen**, **Daten wirklich verschieden**?

**Test.** Ein Riss ist erst echt, wenn (1) beide Zeugen dieselbe Sache unter
denselben Bedingungen messen (Rahmen, Epoche, Einheit, Kadenz, Auswahl,
Methode), (2) Parser-, Einheiten-, Interpolations-, Aggregations- und
Auswahlfehler ausgeschlossen sind, und (3) die Daten jenseits der
ausgewiesenen Unsicherheit differieren. Fehlt ein Schritt: `pending`.

**Grenze.** Geprüft sind die **zehn Riss-Cluster** des Zensus, nicht jede der
118 Dateien, die das Wort „Riss" tragen. Die 104 Dokumente mit Abschluss- und
Offen-Markern sind eine Kandidatenmenge, kein Item-Beweis.

## Bilanz

| | Zahl |
|---|---:|
| Riss-Cluster im Zensus | **10** |
| bestehen den Test als **Datenriss** | **0** |
| bestehen als **Begriffsriss** | **1** |
| sind **`pending`** (Kette/Methode/Zugang) | **9** |
| Dateien mit „Riss" | **118** |
| davon mit Vergleichbarkeits-/Kettenfehler-Wort | **5** |

## Die zehn Cluster

| Cluster | Zeugen | Vergleichbar? | Kette ausgeschlossen? | Daten verschieden? | Urteil |
|---|---|---|---|---|---|
| GIC-Kausaltreiber | yearly-round vs. hardened-quarterly | ja (gleiche Daten) | **nein** — zwei Estimator | offen | **`pending`** |
| GIC-Breitenband | global-FDR vs. Familien-FDR; 133 Stationen | teilweise | **nein** — Methode/Provenienz | offen | **`pending`** |
| Tauchbefunde: GALAH | `ra_dr2`/`dec_dr2` vs. TAP | ja | **nein** — Schema/Parser | offen | **`pending`** |
| Tauchbefunde: Auger | EeV vs. eV | ja | **nein** — Einheit | nein | **`pending`** |
| Tauchbefunde: CDN | drei registrierte `url` = 404 | ja | **nein** — Transport | offen | **`pending`** |
| Quellen: MODVOLC/Namen | Lizenz/Namensraum | nein | — | — | **`pending`** (Metadaten) |
| WINDMI/MOSWOC | Modell vs. Messung; restricted | nein | — | — | **`pending`** (Definition/Zugang) |
| Flyby-2 | ungeeichter Bogen vs. Gate | nein | **nein** — unkalibriert | — | **`pending`** |
| v2-Sandbox-Filter | Codeverhalten vs. Erwartung | nein | **nein** — Code | — | **`pending`** |
| Kanal-Ontologie | Active/Vacant/Drained/Pending | ja (Definitionen) | ja | **ja, semantisch** | **Begriffsriss** |

**Der eine Riss** ist die Kanal-Ontologie: dort widersprechen sich die
Definitionen selbst (ein Kanal kann nicht zugleich `Active` und `Drained`
sein). Das ist ein **Begriffsriss**, kein Datenriss — die Daten sind nicht
verschieden, die Sprache ist es.

**Die neun anderen** sind das, was die Vergangenheit zeigt: die Kette. Zwei
Estimator, ein Parser, eine Einheit, ein 404, eine Lizenz, ein ungeeichter
Bogen. Kein Fall trägt zwei vergleichbare Messungen, die sich wirklich
widersprechen.

## Was ein echter Datenriss wäre

Ein echter Datenriss wäre zum Beispiel:

- dieselbe Ephemeride, zweimal aus denselben Rohdaten gerechnet, mit
  verschiedenem Ergebnis (dann ist die Kette kaputt, nicht die Physik);
- dieselbe Quelle, zweimal geparst, mit verschiedenem Wert (Parser);
- zwei **verschiedene** Instrumente, dieselbe Größe, dieselbe Epoche,
  dieselben Einheiten — und ein Unterschied jenseits der Fehlerbalken.

Die verschiedenen Ephemeriden (DE, INPOP, EPM) sind **kein** Riss: es sind
verschiedene Modelle, keine verschiedenen Messungen derselben Sache.

## Kandidatenmenge (nicht Urteil)

- **104** Dokumente tragen Abschluss- **und** Offen-Marker. Das heißt nur, dass
  in einem Dokument beides vorkommt — nicht, dass dasselbe Item beides ist.
- **33** Docs sind `orphan` (offene Marker ohne Träger) — die stillen Lücken.
- **855** offene Registerzeilen; **0** Registerwaisen.

## Nächster Akt

1. Die neun `pending`-Fälle je auf ihren Kettenfehler prüfen (Parser, Einheit,
   Transport, Estimator, Kalibrierung) — erst dann darf „Riss" wieder stehen.
2. Die Kanal-Ontologie als **Begriffsriss** führen, nicht als Datenriss.
3. Die 104 Dokumente itemweise prüfen; bis dahin bleiben sie Kandidaten.
