<!--
  title: Survey — Asservatenkammer-Zensus (Surveys und Doks ohne Bearbeiter)
  class: survey
  date: 2026-10-10
  sha256: db57d4d0d045102632b77ad37ccb4ae015dcd89166c5767056381720b4bbf1fb
  status: live
  see-also: docs/handover/archiv/handover-2026-10-10-mountain-folge302.md docs/concepts/eigene-ephemeride.md
-->
# Survey — Asservatenkammer-Zensus (2026-10-10)

**Frage.** Wie viele Surveys/Doks wurden mit Recherche-Aufwand erzeugt und liegen
danach unbearbeitet — formal getragen, aber von keiner nächsten Session abgearbeitet?
Welche tragen ein handelbares offenes Versprechen, welche sind Messreihe/Regel/Kanon?

**Durchgeführt.** 2026-10-10, Mountain. Zensus am Baum (Marker-Scan, `git log -1` je
Pfad, Träger-Scan des Basenamens/Stamms gegen die flachen lebenden
`docs/handover/handover-*.md`). `register_lookup --orphan-docs` meldet **22**
Marker-Doks ohne lebenden Träger (gemessen 2026-10-10).

## Befund

- Der Gate `commit_check` `doc-carrier` existiert, greift aber nur bei **offenen
  Markern** und verlangt nur „**irgendeine** lebende Übergabe" — ein Dok ist damit
  formal getragen, ohne daß der nächste Bearbeiter es kennt.
- Fast alle „Träger" hängen an **einer** breiten Zitat-Liste in
  `docs/handover/handover-2026-10-10-sensory-folge250.md` — kein offener Punkt, keine
  Handlung. Formal `ja`, funktional `pending`.
- **Neu gebaut (selber Atom, `c0441466d`):** `commit_check` **own-handover** — ein
  **neu hinzugefügtes** `docs/*`-Dok muß im selben Commit in der **eigenen** Übergabe
  genannt sein. Das schließt den Riß, daß ein Dok in `docs/` landet und keine nächste
  Linie es je erfährt.

## Wiederbelebungs-Kandidaten — handelbares offenes Versprechen

Markerzahl aus dem Zensus; Träger-Vorschlag ist ein **Vorschlag**, keine Zuweisung
(die Owner-Session entscheidet).

| Dok | Offen | Kern-Faden | Träger-Vorschlag |
|---|---|---|---|
| `docs/surveys/survey-2026-10-04-exposom-matrix.md` | 82 | Quellen-Landkarte somatisch/psychosomatisch, Zeile je Krankheitsklasse = eine TE-Messung | Mountain (Quellen) · Weberin |
| `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | 32 | Register-Inventur aufgegebener/offener Quellen | Mountain |
| `docs/surveys/survey-2026-10-08-open-sources-delta.md` | 18 | Gegen-Audit zum Open-Sources-Audit | Mountain |
| `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | 15 | 156 Orphan-Releases warten auf Disposition (Step 3) | Mountain |
| `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` | 11 | fehlende Fäden + verifizierte Beschaffungsrouten | Mountain · River |
| `docs/surveys/survey-2026-10-09-redistribution-alternativen.md` | 10 | freie Alternativen für 41 `redistribution`-declined Quellen | Mountain |
| `docs/surveys/survey-2026-10-09-domaenen.md` | 8 | Domänen-Landschaft am Baum, Lücken als Lücken | Mountain |
| `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | 6 | fremde Parser-/Compiler-Sammlungen (Vergleich) | Mountain |
| `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` | 5 | FMHY/Awesome-Mining, nächster Schritt je Kandidat | Mountain |
| `docs/surveys/survey-2026-10-07-fmhy-forschungsschicht.md` | 2 | FMHY-Wiki noch zu vermessen | Mountain |
| `docs/surveys/survey-2026-10-08-research-api-mcp.md` | 3 | Consensus/Elicit/SciSpace/Perplexity — Auth-Route | Mycelium |
| `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` | 2 | verlorene, heute entblockbare Konzepte | Mountain · Rat |

## Bewußt nicht handelbar (Messreihe/Regel/Kanon — 0 honored, bleibt)

`system-directive`, `the-counter-slope`, `docs-naming`, `tools-map`, `ui-seats`,
`die-vier-schilde`, `4d-membrane`, `arxiv-api`, `positive-maske`, `parser-magic`,
`mirror-research`, `te-literatur-matrix`, `ein-blatt-*`, `bindings-dust-maske-gebco`,
die `axiom-gate-*`-Export-Gates, `survey-2026-09-02-code-te-drift`, `survey-auswertung`,
`survey-geometric-ground-truth`, `survey-raetsel-bestand`/`-zensus` (stehende Messreihe),
`die-weberin` (draft). Sie tragen keinen Bearbeiter, sondern eine Meßreihe oder eine
Regel — sie bleiben, ohne Trägerpflicht.

## Nächster Schritt

Jeder handelbare Kandidat bekommt einen **Träger-Punkt** in der Übergabe seiner
Owner-Linie (Lage · Blockade · Braucht), der genau den nächsten begrenzten Schritt
nennt — nicht die ganze Survey, sondern die erste Zeile, die sich in einem Atom
arbeiten läßt. Ist ein Dok tatsächlich erledigt, wird es `descoped` mit Befund, nicht
liegengelassen.
