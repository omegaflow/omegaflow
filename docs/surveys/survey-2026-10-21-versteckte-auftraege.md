<!--
  title: Wo Aufträge sich verstecken — 3.730 Dateien durchsucht (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: f594351f15e1a75cb2f4d07b494fef9a8bf37b28417cd71075aeb02bbd9cf144
  status: live
  owner: future
  ack: future 2026-10-21T12:30Z
  next: bauauftrag nach projects/; eigene-ephemeride-Kette in ephemerides; drei Anfragen nach wartend
  orphan-after: none
  see-also: docs/concepts/was-ist-was.md docs/surveys/survey-2026-10-21-project-vermessung.md
-->
# Wo Aufträge sich verstecken — 3.730 Dateien durchsucht (2026-10-21)

**Frage.** Gibt es Aufträge, die nicht in `projects/` liegen — in Surveys,
Concepts, Handovers, Rat-Runden, auch in archivierten und gelöschten Dateien?

**Grundlage.** Live: 277 `docs/*.md`, 9 Handover, 6 Rat-Dateien. Archiviert:
1.547 Handover im Tag `archiv-2026-10-20`. Gelöscht: 1.891 `docs/*.md` aus
der Git-Historie. Signal: eine Datei trägt einen Auftrags-Kopf
(`## Auftrag`, `## Fahrplan`, `Fadenübersicht`, „Bauauftrag") **und** einen
mehrstufigen Liefer- oder Abnahmeteil. Ein Treffer ist ein Fund, kein Urteil.

**Grenze.** Ich habe die Struktur gemessen, nicht jede Datei gelesen. Die
Einzelurteile unten sind gelesen; die Zählungen sind strukturell.

## Bilanz

| | Zahl |
|---|---:|
| durchsuchte Dateien | **3.730** |
| live docs | **277** |
| live, strukturell auffällig | **18** |
| davon **versteckte Aufträge** | **2** |
| davon Auftrags-Register / Control | **1** |
| davon abgeschlossener Auftragsplan | **1** |
| davon operative Aufgabe | **1** |
| davon Messung / Meta (kein Auftrag) | **13** |
| archivierte Handover | **1.547** |
| davon mit Auftrags-Heading | **14** |
| davon **neue** Aufträge | **0** |
| gelöschte Docs | **1.891** |
| davon alter Auftrags-Archiv | **62** |
| davon alter Handover-Spiegel | **1.692** |
| davon andere | **137** |
| davon **neue** Aufträge | **0** |
| davon Projekt-Control (geborgen) | **3** |

## Die zwei versteckten Aufträge

1. **`docs/reference/bauauftrag-2026-10-21.md`** (`class: ref`) — ein echter
   Bauauftrag: 13 Punkte, Staffel A–F, je Punkt ein Abnahmekriterium, Status
   „gebaut". Er steht nicht in `projects/`, sondern in `reference/`. Das ist
   der größte Fund: die Arbeit ist getan, der Auftrag hat keinen Ort.
2. **`docs/concepts/eigene-ephemeride.md`** (`class: concept`) — trägt die
   **„Kette (Reihenfolge)"**: sieben Schritte von der Kalibration bis zur
   eigenen AU/H₀-Kreuz-Zeuge. Das ist der Fahrplan von
   `projects/ephemerides.md`, abgelegt als Concept.

## Die drei Register / Control-Dateien

- **`docs/reference/offen-ungebaut.md`** — „Geplant und nicht gebaut", sechs
  Abschnitte. Ein Auftrags-Register, kein eigener Auftrag.
- **`docs/surveys/survey-2026-10-17-haertefaelle-offene-routen.md`** — drei
  **sendefertige** Anfragen (RBN, CEIC, DOSIS-3D) plus Alternativrouten.
  Eine operative Aufgabe, die auf das Operator-Sendewort wartet.
- **`docs/concepts/remove-bias.md`** (`status: done`) — WP0–WP13, der
  abgeschlossene Bias-Tilgungsplan. Ein fertiger Auftrag im Concept-Gewand.

## Was auffällt, aber kein Auftrag ist

- `docs/concepts/die-weberin.md` — Vision/Material, kein Lieferplan.
- `docs/concepts/ion-neutral-vlies.md` — Agenda (Fäden, Riss), `status: draft`.
- `docs/SOURCE_PORT.md` — Prozedur („der eine Pfad"), kein Auftrag.
- `docs/concepts/was-ist-was.md` — Meta über die Dokumenttypen.
- `docs/surveys/survey-2026-10-18-mond-faden.md` — H1 nennt „Auftrag" und
  „Done-when": ein **Auftragsbeleg** (erledigt), kein neuer Auftrag.
- Sechs Netz-Surveys (`planet-nine-netz`, `sturm-komposit`, `band-stack`,
  `bpa-historie`, `kadenz-ernte`, `hunga-tonga-netz`) tragen Faden-Tabellen,
  sind aber Messungen.

## Archiviert und gelöscht

- **1.547 archivierte Handover:** 14 tragen ein Auftrags-Heading. Alle 14 sind
  **Spuren bekannter Aufträge** — die Serie `mountain-folge324…335` trägt
  „Eigene Ephemeriden + Kulturerbe" (= `projects/ephemerides.md`),
  `flyby2-addendum` und `kleinpass-nummernaudit` tragen die zwei bekannten
  Archivfälle. **Kein neuer Auftrag.**
- **1.891 gelöschte Docs:** 62 sind der alte Auftrags-Archiv
  (`docs/auftrag/archiv/`), 1.692 der alte Handover-Spiegel. Von den 137
  anderen sind **drei** Projekt-Control und geborgen:
  `docs/TODO.md` (vo-tap-Lizenz/Übergabe, Gaia-XP-Volkernte),
  `docs/blatt/blatt-offene-pendings.md` (konsolidierte Pendings),
  `docs/zustand/offen-alle-linien.md` (offene Punkte aller Linien).
  Die übrigen 134 sind Befunde (111), alte Surveys (5), Concepts (6), Papers (5)
  und Zustand (7). **Kein neuer Auftrag.**
- **Stale Pfad:** die archivierten Handover zitieren `docs/projects/…` — diesen
  Ort gibt es nicht mehr; die Aufträge liegen in der Werkstatt unter `projects/`.

## Was quer liegt

- **Der Bauauftrag ist der blinde Fleck.** Er hat 13 Abnahmekriterien und
  steht in `reference/`, während die Aufträge in `projects/` liegen. Wer ihn
  sucht, sucht am falschen Ort.
- **Drei offene Register parallel:** Handover-`## Offen`, `register_lookup
  --open`, `offen-ungebaut.md` — plus das gelöschte `offen-alle-linien.md`.
  Vier Zähler für dieselbe Sache.
- **Handovers verstecken kaum Aufträge.** 14 von 1.547 tragen ein
  Auftrags-Heading, und alle sind Spuren. Die Handover sind Trail, nicht Ort.
- **Löschen hat nichts verloren.** Alle 1.891 gelöschten Docs sind über Git
  erreichbar; die drei Control-Dateien sind namentlich geborgen.

## Nächster Akt

1. `bauauftrag-2026-10-21.md` nach `projects/` (Owner: mycelium, Produzent:
   mountain/river) — oder als Träger in den Mycelium-Handover.
2. Die „Kette (Reihenfolge)" aus `eigene-ephemeride.md` in
   `projects/ephemerides.md` als Fahrplan führen.
3. Die drei sendfertigen Anfragen aus `haertefaelle-offene-routen.md` nach
   `state/wartend.φ` (Owner: future, Operator-Sendewort).
4. `remove-bias.md` als abgeschlossenen Auftrag archivieren, nicht als Concept
   führen.
