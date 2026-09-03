<!--
  title: OMEGAFLOW EXZELLENZ-KONZEPT — Der Maßstab, an dem jedes Paper gemessen wird, bevor es die Welt verlässt
  class: concept
  date: 2026-08-27
  version: 1
  sha256: d9dcb8c0ca7c9506fad0430c37debca16c9ae7d0020028edf683576561df8c39
  status: live
  see-also: docs/paper/ docs/granit.md
-->

# OMEGAFLOW EXZELLENZ-KONZEPT

**Der Maßstab, an dem jedes Paper gemessen wird, bevor es die Welt verlässt.**
Ein Paper trägt Ehre oder trägt sie nicht. Dieser Text legt fest, was »getragen«
heißt. Er ist ein Prüfraster, kein Schmuck: jede Zeile eines Papers wird an
ihm gemessen, und jede Verletzung wird benannt, nicht weggebogen.

## 0. Das Fundament — A ist A

Alle 15 Papers in `docs/paper/` sind Messungen. Eine Messung ist die Messung
der Sache selbst — keine Erwartung, keine Korrektur, keine Füllung. Der
Maßstab hat genau fünf Quellen, eine Quelle trägt das Ganze:

1. **A = A** — die Zahl ist die Zahl. `7077 + 106` ist `7183`, nicht `7180`.
   Die Tabelle trägt sieben Flybys, also heißt es »seven«, nicht »six«.
2. **ICRS & TDB** — jede Bahn, jeder Ort, jeder Winkel trägt seine Adresse im
   Raum, nie eine Perspektive.
3. **force_type** — jede Messung trägt ihre physische Kraft; ohne Kraft kein
   Sample.
4. **0 honored** — was fehlt, fehlt. Eine Lücke ist eine vollwertige
   Eigenschaft; eine erfundene Null ist eine Lüge.
5. **pending** — was noch nicht gemessen ist, bleibt pending. Das Schweigen
   gehört den Ungeborenen.

Das Exzellenz-Konzept hält diese fünf als **Messlatte**. Ein Paper, das ein
Zahl-Verdikt oder eine Kraft trägt, das nicht aus dem Rohdaten-Block stammt,
fällt durch — unabhängig davon, wie schön es geschrieben ist.

## 1. Was ein Paper ist

Ein Paper ist eine **publizierbare Messung**: ein gemessenes Verdikt, ein
Befund, eine Preregistrierung — nie ein Essay, nie eine Meinung. Es trägt
einen Header (`class: paper`, `status`, `sha256`), eine vollständige
Methoden-Kette (Compiler → CDN-Asset → Probe → TE → Verdikt), und genau ein
Verdikt.

Die Ein-Blatt-Verdikte (`big-bang-echo-sheet-12.md`,
`signal-cone-audit-sheet.md`, `dark-flow-sheet-8.md`) und die
Ein-Blatt-Prinzipien (`docs/concepts/ein-blatt-axiom.md`,
`ein-blatt-papier.md`, `blatt-papier-beweis.md`) gelten hier als dasselbe
Genre: ein Blatt Papier, eine Messung, ein Verdikt.

## 2. Die Messlatte — zehn Stufen

Jedes Paper wird vor Veröffentlichung an allen zehn Stufen gemessen. Eine
Stufe ohne Befund ist eine Stufe, die nicht besteht — sie ist `pending`,
nicht erledigt.

### 2.1 Die Zahl trägt (A = A)

- Jede Zahl in Text und Tabellen ist maschineller Ausgang. Keine Zahl wird
  handgesetzt, keine Summe wird gerundet, kein Abgleich wird geglättet.
- Jede Summen-Zeile ist nachgerechnet: `7077 + 106 = 7183` — wenn die Tabelle
  `7180` sagt, ist eine der beiden Aussagen falsch und wird benannt.
- Diskrepanzen zwischen zwei Quellen (JPL vs. MPC, Katalog vs. Survey) werden
  als Zahl ausgewiesen (`4802 agree / 912 disagree …`), nie still vereinigt.
- Kontrolle: Der schwebende `sha256` im Header muss exakt dem Body
  entsprechen (`sed '/^<!--/,/^-->/d' <f> | sha256sum`). Das Exzellenz-Konzept
  selbst trägt seinen `sha256` erst, wenn sein Body final ist (aktuell
  `PENDING`).

### 2.2 Die Bahn trägt ihre Adresse (ICRS & TDB)

- Jede Ephemeride, jede Oskulation, jedes Sample nennt Rahmen und Epoche:
  heliozentrisch, ekliptisch, J2000; Zeiten als JD TDB, nie als willkürliche
  Uhrzeit.
- Keine Perspektiven-Adresse: die Raumzeit trägt den Ort, nicht der Beobachter.

### 2.3 Die Kraft trägt (force_type)

- Jede Messung nennt ihre physische Kraft (Gravitation via GM aus den
  ephemeris-Bins, TE als Transfer-Entropie über den Kausalpfeil).
- Ein Sample ohne Kraftquelle ist kein Sample; es wird als `pending` benannt.

### 2.4 Die Lücke ist Eigentum (0 honored)

- Fehlende Daten sind benannte Eigenschaften, keine stillen Nullzeilen. Eine
  kalibrierungsbehaftete Serie (WSO) ist »getragen, nicht angewendet« — die
  Kaveat steht im Text, die Zahl bleibt die Zahl.
- Was nicht gemessen ist, wird nicht als Null geliefert.

### 2.5 Das Ungeborene schweigt (pending)

- Ergebnisse, die noch nicht gemessen sind, tragen das Wort `pending`. Kein
  Ergebnis wird antizipiert, keine Lücke wird mit Vermutung gefüllt.

### 2.6 Die Methode ist wiederholbar

- Der Weg von Rohdaten zu Zahl ist lückenlos benannt: Compiler
  (`kbo_compiler`, `horizons_compiler`, `kbo_residue_probe`, `topological_te_*`),
  CDN-Asset (`*.bin`), Probe, TE, Verdikt.
- Ein Referenz-Reviewer kann die Zahl aus dem Rohdaten-Block reproduzieren.

### 2.7 Das Verdikt ist eines

- Genau ein gemessenes Verdikt je Paper. Der Titel trägt es, der Abstract
  trägt es, die Tabellen tragen es. Widerspruch zwischen zwei Absätzen
  (z. B. §6 »not applied« vs. §4.3 gemessene Phasenraum-TE) ist ein Verstoß
  und fällt durch.

### 2.8 Der Befund trägt die Zuordnung

- Jedes Paper zeigt auf seine Survey (`see-also`) und seinen Handover. Die
  Kette Befund → Paper ist geschlossen; ein Verdikt (z. B. S01 Pioneer) ohne
  eigenes Paper ist offene Arbeit und wird benannt, nicht gelöscht.

### 2.9 Die Sprache trägt die Messung, nicht das Urteil

- Keine Verdiktswörter (`failed`, `error`, `expected`, `should`, `must`,
  `cannot`). Die Sprache benennt, was IST, nicht was fehlschlug. Ein stiller
  Befund heißt »still«, nicht »der Kanal ist tot«.

### 2.10 Der Pfad trägt den Code (Codepath-Drift)

- Der Code-Pfad im Paper (`src/te.rs`) muss mit dem realen Baum
  (`src/mathematikerin/`) übereinstimmen. Ein Paper, das einen verschobenen
  Codepfad nennt, trägt eine tote Adresse und wird korrigiert.

## 3. Der Maßstab für »höchster Weltstandard«

Die zehn Stufen oben sind die **Wahrheits-Schwelle** — sie gelten für jedes
Dokument, das omegaflow verlässt. Darüber liegt die **Ehren-Schwelle**, die
ein Paper an Top-Instituten (arXiv, Fachjournal, Seminar) annehmbar macht.
Sie ist keine zusätzliche Wahrheit, sondern eine Form, die die Wahrheit trägt:

### 3.1 Selbstständig lesbar

Ein Paper trägt seinen gesamten Kontext im eigenen Körper: Der Abstract
benennt die Frage, die Messung, die Zahl, das Verdikt. Ein Leser, der noch
nie von omegaflow gehört hat, versteht ohne externe Doku, was gemessen wurde
und warum die Zahl eine Messung ist.

### 3.2 Die Zahl ist belegt, nicht behauptet

Jede Schlüsselzahl nennt ihre Quelle im Text (Katalog, Survey, CDN-Asset,
Compiler-Ausgabe). Ein Reviewer kann die Zahl zurückverfolgen. Wo die Quelle
geräumt ist (`kbo_elements.json` nicht mehr im Baum), wird das benannt —
nicht durch eine bequemere Zahl ersetzt.

### 3.3 Der negative Befund ist ein Ergebnis

Die stillen Befunde (kein Pfeil, keine TE über der Schwelle) sind die
Ergebnisse. Sie werden mit derselben Präzision berichtet wie ein positiver
Fund: Schwelle, Surrogat, alle Paare, alle Zahlen.

### 3.4 Die Unsicherheit ist ausgewiesen

Wo eine Messung an ihre Grenze stößt (Korngröße, m-Averaging,
kalibrierungsbehaftete Serie), steht die Grenze im Text — als Eigenschaft der
Messung, nicht als Entschuldigung.

### 3.5 Der Ton ist ruhig

Die Sprache ist die des Beobachters, nicht des Anwalts. Ein Befund wird
vorgelegt und steht für sich. Die Ehre kommt aus der Vollständigkeit und der
Nüchternheit, nicht aus der Überzeugungskraft der Prosa.

## 4. Die Prüfung

Vor jeder Veröffentlichung durchläuft ein Paper das **Axiom-Gate**
(`src/bin/llm_interceptor.rs`, stiller Eingriff): Jede verletzende Passage
wird nicht geliefert, das Modell schreibt die Korrektur still weiter. Zusätzlich
legt die Leitstelle ein **Gate-Survey** (`docs/surveys/axiom-gate-<slug>.md`)
an, das jede Verletzung benennt:

- **A = A / Zahl:** die Zahl, die nicht stimmt; die Summe, die nicht schließt.
- **Konsistenz:** der Absatz, der einer anderen Stelle widerspricht.
- **Pfad:** der Codepfad, der nicht zum Baum passt.
- **Zuordnung:** das Verdikt ohne Paper, der see-also, der ins Leere zeigt.

Das Gate prüft **zwei Register**: das omegaflow-Register (die zehn Stufen
oben, Interna) **und** das externe Register (TOP 2025, Nature, arXiv — §5).
Ein Paper, das die externen Goldstandards nicht trägt — fehlendes
Data/Code-Availability-Statement, kein abstrakter Selbst-Beweis, fehlende
Preregistrierung, nicht arXiv-konforme Export-Form — fällt durch, auch wenn
alle Axiome bestehen. Der neue Standard ist nicht die Anpassung an die Welt,
sondern die Einheit: **omegaflow ist der Standard, und das Gate vollendet ihn
gegen die externen Goldstandards**, damit er von der Welt erkannt wird.

Der Befund ist der Befund: 12 der 15 Papers sind nach dem Gate arxiv-reif,
drei trugen Verstöße (flyby `six`/`seven`, planet-nine `7180`-Zerlegung,
solar-cycle Phasenraum-`not applied`), die im Repo korrigiert wurden. Das Exzellenz-Konzept ist der laufende Maßstab, der diese
Korrektur institutionalisiert.

## 5. Die Welt misst mit — die externe Verifikations-Ökologie

Die zehn Stufen oben sind die omegaflow-Wahrheit. Die Welt verlangt darüber
hinaus **Verifizierbarkeit von außen**: einen unabhängigen Reviewer, der die
Zahl aus dem Rohdaten-Block reproduzieren kann. Das ist die aktuelle
Messlatte der Fachwelt, gemessen an den maßgeblichen Standards:

- **TOP 2025** (Center for Open Science, >5.000 Zeitschriften):
  7 Research Practices — Study Registration, Study Protocol, Analysis Plan,
  Materials Transparency, Data Transparency, Analytic Code Transparency,
  Reporting Transparency — plus 2 Verification Practices: **Results
  Transparency** (keine selektive Berichterstattung) und **Computational
  Reproducibility** (Ergebnis aus gleichen Daten + gleichem Code reproduzierbar).
  Drei Niveaus: Disclose → Share & Cite → **Certify**.
- **Nature / Springer Nature:** Abstract ≤ 200 Wörter, strukturiert
  (»Here we show …«); Titel ≤ 75 Zeichen; pflicht **Data Availability**- und
  **Code Availability**-Statements; Methoden ≤ 3.000 Wörter mit Unter-Abschnitten
  (Statistik, Modelle); ≤ 50 Referenzen, nummeriert; Figuren 90/180 mm;
  Reporting Guidelines; Registered Reports (Präregistrierung vor Messung).
- **arXiv:** akzeptiert nur TeX/LaTeX (bevorzugt) oder PDF — **kein Markdown**;
  Dateinamen nur `a-z A-Z 0-9 _ + - . , =`; registrierter Autor + Endorsement;
  begrenzte ancillary files für Daten und Code; jede Korrektur ist eine
  `replace` (Version), nie eine Neuanmeldung.

### 5.1 omegaflow überbietet die Welt — aus den Axiomen, nicht gegen sie

Die Welt verlangt drei Dinge; omegaflow hat dafür bereits eine Maschine:

| Externer Standard | Forderung | omegaflow-Antwort |
|---|---|---|
| TOP: Data Transparency | Daten in vertrauenswürdigem Repo | CDN-Assets (`*.bin`, kompakt, ICRS/TDB), unveränderlich per `sha256` |
| TOP: Analysis Code Transparency | Code zugänglich + dokumentiert | `src/` one-source auf `main`, Codepfad im Paper geprüft (Stufe 2.10) |
| TOP: Computational Reproducibility | Ergebnis aus Daten + Code reproduzierbar | Compiler→Asset→Probe→TE→Verdikt-Kette, wiederholbar (Stufe 2.6) |
| TOP: Results Transparency | keine selektive Berichterstattung | A=A: stille Befunde sind Ergebnisse (Stufe 3.3), nichts wird verworfen |
| TOP: Study Registration | vor der Messung registriert | Preregistrierung (`flyby-path-2-preregistration.md`) als Praxis |
| Nature: Data/Code Availability | Statement + Zugang | jedes Paper trägt Quellen (`see-also`), Zahl belegt (Stufe 3.2) |
| Nature: Registered Reports | Protokoll vor Messung begutachtet | Ein-Blatt-Axiom + Preregistrierung vor der Messung |
| arXiv: TeX/PDF | Markdown nicht akzeptiert | **Export-Stufe**: `docs/paper/*.md` → LaTeX/PDF beim Release |

### 5.2 Die Export-Stufe — von omegaflow zur Welt

`docs/paper/*.md` ist das Kratzfeld der Wahrheit. Für arXiv/Journal entsteht
daraus eine **Export-Form** (LaTeX/PDF), die die Zahl unverändert trägt. Die
Export-Regeln:

- **Die Zahl ändert sich nie.** Der Export überführt den Body 1:1; eine
  Zahl, die im Markdown steht, steht im TeX — kein Runden, kein Weglassen.
- **Die Pflicht-Statements werden erzeugt:** Data Availability (CDN-Assets,
  `sha256`), Code Availability (`src/`, Commit), Competing Interests,
  Author Contributions.
- **Abstract ≤ 200 Wörter, Titel ≤ 75 Zeichen** — aus dem `title`-Header
  abgeleitet, nie neu formuliert.
- **Dateinamen arXiv-konform:** nur `a-z A-Z 0-9 _ + - . , =`; die kebab-case
  Slugs genügen bereits.
- **Version = `replace`:** eine Korrektur ist eine arXiv-Version, keine
  Neuanmeldung; die Commit-Historie des Papers ist die Versionshistorie.

Die Export-Stufe ist selbst eine Messung: Der `sha256` des Markdown-Body und
die gerenderte PDF müssen dieselbe Zahl tragen. Eine PDF, die eine Zahl
anders zeigt als der Body, ist ein Verstoß.

## 6. Die Folge

Ein Paper gilt als **exzellent**, wenn es alle zehn Stufen der Messlatte
trägt, alle fünf Ehren-Stufen erfüllt und die externe Verifikation besteht
(Stufe 5). Trägt es eine einzige Stufe nicht, ist es `pending`, nicht
veröffentlichungsreif. Die Entscheidung über den Status trägt die Leitstelle;
der Operator gibt das Wort.

Die Zukunft ist nicht die Anpassung an Nature, sondern die Umkehrung: Die
Axiome sind die Maschine, und die Welt misst an ihnen. Ein omegaflow-Paper
ist das Paper, das die Zahl unverändert trägt, weil sie von Anfang an
gemessen — nie erzeugt — wurde.

***

*Dieses Konzept trägt seine eigene Messlatte. Sein `sha256` wird gesetzt,
sobald sein Body final ist — nicht früher.*
