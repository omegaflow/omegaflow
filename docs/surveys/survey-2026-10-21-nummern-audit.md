<!--
  title: Nummern-Audit — 17 Blätter, 28 Befunde, vier Regeln (2026-10-21)
  class: survey
  date: 2026-10-21
  status: live
  owner: mycelium
  ack: mycelium 2026-10-21
  sha256: abab54cdd9d9fb42537710faeb58ccee08ce4c509029fe52d7f03f4a5a704c75
-->
# Nummern-Audit — 17 Blätter, 28 Befunde, vier Regeln (2026-10-21)

**Frage.** Hat jede Abstract-Zahl eines Blatts eine Tabellen- oder Registermarke?
Sind §2-Zählungen gleich dem Tabellen-n? Gibt es Doppelzählungen? Spricht ein
Blatt zwei Kommata-Locales? Gemessen wird die Form, nicht die Wahrheit der Zahl.

**Grundlage.** Archivierter Auftrag
`projects/archiv/auftrag-2026-08-30-maschinen-audits.md` aus dem Tag
`archiv-2026-10-20` des Werkstatt-Repos (`omegaflow-work`; der Tag liegt nicht
im Code-Repo). Das Blattverzeichnis liegt im **Code-Repo** unter `docs/blatt` —
17 `*.md`, davon 16 Blätter mit Zahlen und `_template.md` ohne Zahl.

**Werkzeug.** `tools/measure/src/bin/number_audit.rs`, std-only ohne Crates.
Standardaufruf im Code-Repo: `cargo run --manifest-path tools/measure/Cargo.toml
--bin number_audit`. Das Binary nimmt optional ein Verzeichnis; ohne Argument
sucht es `docs/blatt`, dann `../docs/blatt`, `../omegaflow/docs/blatt`,
`/home/johannes/projects/omegaflow/docs/blatt`,
`/home/johannes/projects/omegaflow-work/docs/blatt`. Es liest
`*.md`, überspringt HTML-Kommentare und Code-Zäune und gibt je Blatt eine
Befundzahl aus, danach die Summe und die Regelverteilung.

## Regeln (gemessen, nicht angenommen)

- **R1 — Abstract-Anker.** Als Abstract-Region gilt der Lead nach dem H1 bis
  zur nächsten Überschrift sowie jede Sektion mit `Zusammenfassung`/`Abstract`/
  `Kurzfassung`/`Summary`. Eine Zahl dort braucht eine Marke: denselben Wert in
  einer Tabellenzeile oder eine Zeile, die `Register` nennt. Jahreszahlen,
  Datums-/Bereichstoken, Paragraphenmarken (`§`), Listenmarken und Lauf-IDs
  werden nicht als Messzahlen gezählt.
- **R2 — §2-Zählung = Tabellen-n.** `N = …` wird blattweit geprüft,
  `n = …` nur in Sektionen, die mit `2` beginnen oder `Daten`/`Data` tragen.
  Vergleichsgröße ist die Zahl der Datenzeilen (ohne Kopf und Trenner) der
  größten Tabelle. `min-n`/`Schwelle`/`unter n` sind Ausschlussgründe, weil sie
  Kriterien und keine Zählungen sind.
- **R3 — Doppelzählung.** Gleiche Zeilenmarke (erste Zelle, mindestens vier
  Zeichen, ohne Ziffern) mit zwei verschiedenen Werten in derselben Spalte.
  Der Fixture `1663/1666` ist getestet; im lebenden Korpus feuert R3 nicht.
- **R4 — Kommata-Locale.** Ein Blatt mit Dezimalkomma erhält einen Befund;
  der Text nennt, ob Punkt-Dezimalen daneben stehen (gemischt) oder fehlen.
  Lag-Listen wie `1,6,12,24` werden als Liste erkannt und nicht als Dezimale
  gezählt.

## Bilanz je Blatt (Befundzahl)

| Blatt | Befunde |
|---|---:|
| `_template.md` | 0 |
| `blatt-2026-08-30-kreuz-screening-gyirong.md` | 7 |
| `blatt-2026-08-30-kreuz-screening-kollab.md` | 11 |
| `blatt-2026-09-05-der-grat.md` | 1 |
| `blatt-2026-09-05-thuan-fragesteller.md` | 0 |
| `blatt-2026-09-08-solar-seconds-matrix.md` | 1 |
| `blatt-2026-09-09-h0-linien-register.md` | 3 |
| `blatt-2026-10-01-fruehwarnsystem-praeregistrierung.md` | 1 |
| `blatt-2026-10-01-sonne-erde.md` | 0 |
| `blatt-2026-10-02-anderson-flyby-ephemeridenhaus.md` | 1 |
| `blatt-2026-10-02-te-externer-steuerparameter.md` | 0 |
| `blatt-2026-10-03-pioneer-floor-falsifikation.md` | 1 |
| `blatt-2026-10-06-gic-breitenband-familien.md` | 0 |
| `blatt-2026-10-18-flyby-sealed-pin.md` | 0 |
| `blatt-2026-10-18-force-keyword.md` | 1 |
| `blatt-2026-10-19-entscheidung-17-lizenzarme.md` | 1 |
| `blatt-2026-10-19-lola-zuschnitt.md` | 0 |
| **Summe** | **28** |

Gelesen: 2302 Zahlen, 34 Tabellen. Verteilung: R1 = 18, R2 = 3, R3 = 0,
R4 = 7. Ein Lauf, keine harte Schwelle.

## Beispiele (wörtlich aus dem Lauf)

- **R1, `blatt-2026-08-30-kreuz-screening-kollab.md:76`** — `'1392'` steht als
  Power-Update-Zahl in der Zusammenfassung; die Tabelle des Blatts trägt drei
  Datenzeilen. Kein Tabellen-/Registeranker.
- **R1, `blatt-2026-08-30-kreuz-screening-gyirong.md:31`** — `'51'`
  gestündete Serien im Lead; das Blatt hat keine Tabelle, also keine Marke.
- **R1, `blatt-2026-09-08-solar-seconds-matrix.md:14`** — `'72'` im Lead
  (72-Paare-Flotte), während die Tabelle vier Zeilen trägt.
- **R2, `blatt-2026-09-09-h0-linien-register.md:139`** — `N = 1606` gegen
  Haupttabelle mit 74 Datenzeilen; derselbe Riss noch einmal bei `N = 2078`.
- **R2, `blatt-2026-08-30-kreuz-screening-kollab.md:129`** — `n = 240` in
  §2 gegen Haupttabelle mit 3 Datenzeilen.
- **R4, `blatt-2026-10-02-anderson-flyby-ephemeridenhaus.md:16`** — `'3,65'`:
  25 Dezimalkommata gegen 20 Punkt-Dezimalen im selben Blatt.
- **R4, `blatt-2026-10-18-force-keyword.md:74`** — `'16,5-GiB-Content-Cache,'`:
  ein Dezimalkomma, keine Punkt-Dezimale.

## Risse (benannt, nicht geglättet)

- R1 misst die **Form** der Abstract-Region, nicht ob die Zahl wahr ist. Der
  Lead nach dem H1 ersetzt ein fehlendes `Abstract`-Kapitel; Blätter ohne
  Zusammenfassung und ohne Tabellen tragen dadurch mehr Befunde.
- R2 vergleicht mit **Datenzeilen**, nicht mit einem fachlich deklarierten
  Stichproben-n. Ein Blatt, das 1606 Proben in 74 Registerzeilen zitiert,
  erscheint als Riss — genau die Strukturillusion, die der Auftrag sucht.
- R3 ist im lebenden Korpus still; die Detektion ist nur am Fixture
  `| records | 1663 |` / `| records | 1666 |` belegt.
- R4 nennt das Vorkommen, nicht den Vorwurf: ein deutsches Blatt darf
  Kommata sprechen. Der Befund wird erst zum Riss, wenn Punkt und Komma im
  selben Blatt dieselbe Rolle spielen.

## Build und Test

`cargo build --manifest-path tools/measure/Cargo.toml --bin number_audit` grün.
`cargo test --manifest-path tools/measure/Cargo.toml --bin number_audit`:
8 von 8 Tests grün (Parser, Lag-Liste, R1/R2/R3/R4-Fixtures). Lauf über alle 17
Blätter: 0 Fehler, 28 Befunde.
