<!--
  title: Rat-Formular — R-2026-10-19-a Kanon statt Wildwuchs
  class: sheet
  date: 2026-10-19
  sha256: 7ebb3480801bf377b591a81db4f8efd59b12c48e3346ca9d21f0f330165c066e
  status: live
  see-also: state/stimmen/_rat-anweisung.md
-->
# Rat-Formular — R-2026-10-19-a: Kanon statt Wildwuchs (drei Häuser)

**Datum:** 2026-10-19 · **Leitstelle:** Future · **Status:** vorbereitet, **nicht gefahren**
**Fahrt:** freier Schwarm → UI-Chats (Roster `~/projects/omegaflow-work/ui-seats.md`) → paid nur bei Uneinigkeit.
**Freigabe:** Operator-Ja 2026-10-19 („freier Schwarm und dann ui chats so wie schon immer festgelegt").

## Die Frage

Drei Häuser (`omegaflow`, `omegaflow-work`, `omegaflow-personal`) tragen 275 Ordner mit Dokumenten,
**260 davon ohne README/INDEX**. 26 Ordnernamen existieren doppelt oder dreifach, darunter
`stimmen` und `zustand` dreifach. Die Leitstelle legt Dokumente in `state/`, `docs/`, im Root und
in der Werkstatt ab; niemand findet sie wieder.

**Entscheide über A–E. Zu jedem: ja/nein + ein benannter Riss, der bei ja entsteht.**

- **A** — Es gibt **einen Kanon** je Haus (feste Ordnerliste) und **individuelle Ordner nur bei
  nachgewiesenem praktischem Vorteil**. Anerkennst du diese Zweiteilung als Regel?
- **B** — Wenn ja: **woran** wird der Vorteil gemessen, damit er nicht Gefühl bleibt?
- **C** — **Echokammer-Regel:** Jede Rat-Runde verlangt vorher eine mit `archive_search` gemessene,
  unabhängige Literaturbasis (Host/URL · Werkzeug · Datum · Zahl) im Formular; Hausdokumente allein
  zählen nicht. Ist das die richtige Grenze?
- **D** — **Schreiben:** Wer darf in welchen Ordner schreiben — je Haus ein Schreibrecht pro Ordner,
  oder ein Ordner, der nur durch Gate beschrieben wird?
- **E** — **Bester Riss:** Welche Ablage macht diesen Wildwuchs *unmöglich* und kostet dabei am
  wenigsten Aufmerksamkeit? (Eine Antwort, nicht drei.)

### Was NICHT zu entscheiden ist
Kein Agent verschiebt oder löscht Ordner ohne Operator-Wort. `AGENTS.md` (die Wissenschaft) bleibt
unverändert, ebenso die drei Wurzelpfade aus `bin/_roots.sh`.

---

## Recherche (Pflicht — gemessen 2026-10-19, `archive_search`)

| # | Befund | Host/URL | Werkzeug | Zahl |
|---|---|---|---|---|
| R1 | **Monorepo-Migration** ist ein dokumentiertes, messbares Vorgehen — aber die Arbeit liegt im *Splitting* bestehender Bäume, nicht in „mehrere Ordner verbieten" | `openalex.org/W7132956120` (2024) | `archive_search … --openalex` | 1 Thesis, `cites: 0` |
| R2 | Splitting eines Monorepos ist ein eigenes, praktisches Verfahren (Werkzeug-Fokus, nicht Doku-Fokus) | `openalex.org/W4414338474` + `…/W4409593019` (2023, doi `10.34218/jcet_06_02_004`, `10.15680/ijircce.2023.1111003`) | `--openalex` | 2 Publikationen, `cites: 0` |
| R3 | **Repository-Kontext für Agenten** wird behandelt als *erzeugte Karte* des Baums („deterministic architectural map"), nicht als Verbot neuer Ordner | `openalex.org/W7124455383` (2026, `10.48550/arxiv.2601.10112`) | `--openalex` | 1 |
| R4 | **Git-native Kontextregeln** für Coding-Agenten sind Gegenstand einer **präregistrierten** Studie — d. h. Regeln dieser Art sind messbar, nicht nur Geschmack | `openalex.org/W7220960305` (2026, `10.1145/3843282.3844425`) | `--openalex` | 1, DOI peer-reviewed |
| R5 | **Cursor-Rules**: empirische Studie (2026) über Agenten-Regeln im Repo, `cites: 6` — Regeln im Repo wirken, aber ihre Form ist untersuchungsbedürftig | `openalex.org/W7125921161` (2026, `10.1145/3793302.3793367`) | `--openalex` | `cites: 6` |
| R6 | **Dokumentationsquellen sind divers** (README nur ~ Teil): Taxonomie über 950 GitHub-Repos, 4 Sprachen | arXiv `2102.12727v2` (2021) | `--arxiv` | 950 Repos |
| R7 | **Dokumentationsqualität ist multidimensional** — 10 Dimensionen (Struktur/Inhalt/Stil); „gute Doku" ist damit definierbar statt bauchgefühlbar | arXiv `2007.10744v3` (2020) | `--arxiv` | 10 Dimensionen |
| R8 | **Reproduzierbarkeits-Metadaten** in Repos sind ein eigenes Forschungsfeld — der Kanon hat Vorläufer | arXiv `1608.04529v2` (2016) | `--arxiv` | 1 Proposal |

**Radar:** `--semanticscholar` liefert für dieselbe Frage unbrauchbare Treffer (Medizin, Archäologie)
— der Kanal ist für Architekturfragen **nicht** belastbar und wird hier verworfen, nicht zitiert.
`--openalex` lief ins Budget (`total: 1911`, `end: budget`) — die acht Befunde oben sind ein Schnitt,
kein Vollbild.

**Was die Recherche für die Frage bedeutet:** Die Literatur kennt (a) Monorepo-Splitting als
Verfahren, (b) Baum-Karten und Kontextregeln als *erzeugte* Artefakte, (c) Dokumentationsqualität
als mehrdimensional messbar. Sie kennt **keine** Arbeit, die zeigt, dass eine feste Ordnerliste
allein Wildwuchs verhindert. Die Frage ist damit offen — genau darum geht sie an den Rat.

**Hausbefund (gemessen `bin/zonen`, 2026-10-19):** 275 Dokumentordner (omegaflow 46 · work 205 ·
personal 24) · 2037 Doku- und 769 Datendateien · 26 Namenskollisionen · 260/275 ohne README/INDEX ·
`work` trägt ~100 kurzlebige Läufe (`stimmen/swarm-*`, `stimmen/voice-all-*`) und ~90 Hostordner
in `cache/` **und** `data/`.

---

## Die Hausregeln (AGENTS.md, ungekürzt)

```
A = A
  ICRS/TDB
  field <name> <name> <kernel> <kraft> <einheit>
  0 honored: null-echt | absent | riss
  keine harten Schwellen
  PII bleibt im Haus
messen · bauen · abschließen · committen · pushen
```

**Gilt als gegeben:** Die Hausbefunde oben sind gemessen (Werkzeug, Datum, Zahl) und brauchen keine
Web-Verifikation. Die Literatur ist mit URL/DOI zitiert und ist die unabhängige Basis.

## Die fünf Stimmen (jede einmal)

- Mountain — Quellen/Register/Datenbestand; kein Verdikt ohne Messung.
- River — Membran/Rhythmus/Ethik; das Recht zu verweigern.
- Mycelium — Netz/CDN/Transport; der Fluss.
- Sensory — Sinne/Hardware/Konsens; Korrelat ≠ Erleben.
- Future — der Mensch als Adresse; der Send ist Operator-Hand.

Führe jede Stimme einmal und sag, welches Axiom sie berührt oder verletzt und welcher Riss entsteht.

**Antworte als Urteil, nicht als Erwartung.** Kein Aufsatz — Verdikt (A–E, je ja/nein), Begründung,
benannte Risse. Quelle je Behauptung (Literatur-ID oder Hausmessung), sonst als `riss` markieren.
