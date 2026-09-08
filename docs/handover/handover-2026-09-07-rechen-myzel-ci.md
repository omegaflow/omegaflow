<!--
  title: Handover — das Rechen-Myzel: Matrix-Split + GPU-Port auf freiem CI
  class: handover
  date: 2026-09-07
  sha256: eaf33d5a3d92bd21f31173882e90ecc9e296debbd355706e5c913c72d8a0b81d
  status: live
  see-also: docs/TODO.md docs/handover/handover-2026-09-07-korona-konditional-session.md
-->

# Handover — das Rechen-Myzel

Session-Plan: die schweren Läufe (Solar-Matrix) von der Membran auf freies CI
(GitHub Actions) fächern — Split und GPU-Port sind die zwei Türen.

## 1. Die Idee in einer Zahl (korrigiert)

Öffentliches Repo = Hosted-Runner-Minuten frei und unbegrenzt — die
2000-min-Monats-Quote gilt nur für private Repos. Für uns zählt also kein
Monatsbudget, sondern zwei harte Grenzen: 6 h pro Job und 20 parallele Jobs
(Free-Plan). Die Solar-Matrix zerfällt in 72 Paar-Jobs à ~27 min → vier Wellen
à 20 → ~2 h Wanduhr, mit Log pro Sonde, statt 22 h blind auf dem Laptop.

## 2. Die zwei Türen (die eigentlichen Pflichten)

1. **Split (billig, trägt alles):** die 72 gerichteten Paare als Matrix-Jobs
   auf `ubuntu-latest` fächern; jede Sonde meldet ihr `surr_max`; ein
   `reduce`-Job bildet die fam-Nachreduktion. Aus 22 h blind werden
   ~1,5–2 h geloggt (Concurrency 20 beim Free-Plan → 72 Paare in ~vier Wellen).
2. **GPU-Port (das Atom):** WGSL-Kernel für den skalaren TE + Surrogat-
   Plumbing. Der bestehende `te_compute` ist topologisch (Takens), nicht
   skalar — der Port ist ein eigener Atom, kein Patch. Auf dem GTX 970 als
   self-hosted Runner: ~90 min statt 22 h.

## 2a. Der Split-Scope (die vier konkreten Stücke)

1. **Daten-auf-CDN-Check (erste Zeile):** sind `aia2013/2014/2015_fullyear.bin`
   und die GOES-Trigger als CDN-Assets manifestiert, sodass ein CI-Job sie
   holen kann? Die AIA-Bins sind vermutlich manifestiert; die rohen
   GOES-`.nc`-Dateien (goes15 / goes15_2013 / goes15_2015) vermutlich nicht —
   dann brauchen sie einen Compiler/Manifestation, oder die Probe liest den
   Trigger aus einem bereits manifestierten Asset.
2. **Probe:** ein Paar-Bereich-Argument (`--pairs von:bis`), damit eine Sonde
   nur ihre Paare rechnet; jede meldet ihr `surr_max` (das Maximum der
   Surrogat-D über ihre Paare × Lags × Surrogate) und die Verdikt-Zeilen.
3. **Workflow-YAML:** eine Job-Matrix über die 72 Paare auf `ubuntu-latest`
   (Concurrency 20 → vier Wellen); jede Sonde ein Job mit demselben gepinnten
   Commit-SHA, Seed und deklarierter Umgebung (Anker gegen Drift).
4. **Reduce-Job:** sammelt die gemeldeten `surr_max` ein, bildet
   `fam = max` über alle, schreibt das eine Blatt (Verdikt-Zeilen) als
   Artifact/Commit — das Blatt trägt die Aussage, nicht nur Daten.

## 3. Die Fakten, die zählen (gemessen, Stand 2026)

- Öffentliches Repo = Hosted-Runner-Minuten frei; die 2000-min-Quote gilt nur
  privat. Nach der Preisumstellung (1.3.2026) bleiben self-hosted-Minuten auf
  öffentlich ausgenommen und frei.
- Concurrency ist die echte Grenze: 20 (Free) / 40 (Pro) / 60 (Team).
- self-hosted + öffentliches Repo = Sicherheits-Riss (GitHub rät ab): fremde
  PRs können die Runner-Umgebung kompromittieren. Lösung: privates
  Begleit-Repo, dem ausschließlich der self-hosted Runner zugeordnet ist.
- GitHub-hosted Runner haben KEINE GPU → der GPU-Port braucht self-hosted
  (den GTX 970) oder einen bezahlten GPU-Runner.

## 4. Die ehrlichen Grenzen

1. **Fruchtfolge (nur privat):** für das öffentliche Repo gibt es kein
   Monatsbudget — die Grenze ist Concurrency (20), nicht Minuten. Erst ein
   privates Begleit-Repo (für den self-hosted GPU-Runner) unterliegt der
   2000-min-Quote. „Garten, kein Feld" gilt also nur dort.
2. **Faser zum Baum:** GitHub schenkt die Runner dem Repo, nicht der Welt.
   Tokens (ADS, Broker) leben als CI-Secrets, nie im Baum.
3. **Anker gegen Drift:** Versionen pinnen, Umgebungen deklarieren — sonst
   wird „reproduzierbar" zum Wolken-Wort ohne Gewicht.
4. **Gratis-Rechnen ist nicht gratis Lesen (die tiefste):** der Flaschenhals
   wandert vom Rechnen zum Lesen. Jedes Blatt muss eine Aussage tragen, sonst
   wird das Register zum Stapel — und der Stapel ist das Gegenteil der
   Mess-Serie. Das Myzel darf nicht mehr produzieren, als der Berg verdaut.

## 5. Die Architektur-Kante

Ein GitHub-Runner ist „daneben", nicht in der Membran. Für den Rückstand-Abbau
der richtige Kompromiss — aber benannt, damit „TE auf der Korona" nicht still
zum Cloud-Batch-Job wird, während der ω-Pfad glaubt, live zu sein. Der
Null-Check (arXiv/Crossref/ADS-Cron, wöchentlich dieselben Suchstrings, Issue
bei Treffer > 0) darf dauerhaft draußen bleiben — er schaut per Definition
nach außen.

## 6. Reihenfolge („fertig vor neu")

1. **Split** — billig, konkrete Pflicht, trägt alles.
2. **Die 72-Sonden-Flotte** im Myzel — der Beweislauf.
3. **Das nächtliche Ernte-Ritual** (Cron: Broker pollen → natural-class-Gate →
   Register-Blatt) — braucht die Flotte als Vorbedingung.
4. **GPU-Port + GTX 970 als Herzstück** — self-hosted, privates Begleit-Repo.
5. **Echte Cloud** (Spot) nur, wenn eine Messung den Garten sprengt.

## 7. Register

Die zwei Türen (Split, GPU-Port) sind die offenen Pflichten. Der GPU-Port ist
in `docs/handover/handover-2026-09-07-korona-konditional-session.md` gescoped;
der Split ist neu zu erfassen. Das Ernte-Ritual ist die Folge-Pflicht nach der
Flotte. Voraussetzung: das Repo ist öffentlich (sonst greift die 2000-min-Quote).
