<!--
  title: Wo Aufträge sich verstecken — 3.730 Dateien durchsucht (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 6cc9e30f6067f9bf2d466709684e7dacb7124844ee5c91ef9cd7323c9f37b9c1
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

## Die zwei versteckten Aufträge — jetzt kanonisch

1. **Bau:** kanonisch `projects/bau-der-arme.md`. Die zwei Belege
   `docs/surveys/survey-2026-10-21-bau-der-arme-beleg.md` (Staffel, Assets, Keys) und
   `docs/surveys/survey-2026-10-21-offen-ungebaut.md` (Messdetails, jetzt `status: superseded`)
   tragen nur noch Beweise und verweisen nach oben.
2. **Ephemeriden-Fahrplan:** kanonisch in `projects/ephemerides.md`, Abschnitt
   „Fahrplan (Kette, 2026-10-21)". Das Concept
   `docs/concepts/eigene-ephemeride.md` trägt nur den Verweis.

## Die drei Register / Control-Dateien — kanonisch

- **Bau-Register:** `projects/bau-der-arme.md` (oben). `offen-ungebaut.md` ist
  Beleg, nicht Register.
- **Offene Anfragen:** `state/wartend.φ` — `rbn-lizenz`, `ceic-d18000`,
  `dosis-3d`. Der Survey `haertefaelle-offene-routen.md` trägt die Entwürfe.
- **Abgeschlossener Plan:** `docs/concepts/archiv/remove-bias.md`
  (`status: archived`) — WP0–WP13, nicht mehr im Live-Baum der Concepts.

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

- **Der Bauauftrag war der blinde Fleck.** Er hatte 13 Abnahmekriterien und
  stand in `reference/`; jetzt liegt er als `projects/bau-der-arme.md` am
  kanonischen Ort.
- **Offene Register:** Handover-`## Offen`, `register_lookup --open` und das
  Bau-Register. Das ist jetzt im Auftrag konsolidiert; die Belege tragen nur
  noch Messdetails.
- **Handovers verstecken kaum Aufträge.** 14 von 1.547 tragen ein
  Auftrags-Heading, und alle sind Spuren. Die Handover sind Trail, nicht Ort.
- **Löschen hat nichts verloren.** Alle 1.891 gelöschten Docs sind über Git
  erreichbar; die drei Control-Dateien sind namentlich geborgen.

## Erledigt (2026-10-21)

1. Bau kanonisch: `projects/bau-der-arme.md`; beide Belege verweisen dorthin.
2. Fahrplan kanonisch in `projects/ephemerides.md`; das Concept verweist.
3. Anfragen kanonisch in `state/wartend.φ` (`rbn-lizenz`, `ceic-d18000`,
   `dosis-3d`).
4. `remove-bias.md` → `docs/concepts/archiv/`, `status: archived`.
5. Offen bleibt nur die **Abarbeitung**, nicht der Ort.
