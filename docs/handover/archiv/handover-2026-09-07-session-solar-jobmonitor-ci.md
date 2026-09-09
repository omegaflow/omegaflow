<!--
  title: Übergabe — Session: Solar-TE, job_monitor, CI-Reparatur (Korona/Nadel III, Werkzeuge)
  class: handover
  date: 2026-09-07
  sha256: 758aaf31947ce80be6ba3881db1bd521c1f206ce8bc6d37dfe0e9ea074ba6e09
  status: archived
  see-also: docs/handover/handover-2026-09-06-korona-ladder-kaskade.md docs/concepts/archivar-mathematikerin.md docs/TODO.md docs/paper/corona-heating-ladder.md docs/surveys/survey-ein-blatt-korona-heizung.md
-->

# Übergabe — Session: Solar-TE-Matrizen, job_monitor, CI-Reparatur

Diese Übergabe dokumentiert die Arbeit einer Session über die Korona/Solar-
Messung (Nadel Ⅲ), das job_monitor-Dashboard und zwei CI-Reparaturen. Sie
soll einer empfangenden Session den Zustand vollständig übergeben.

## Ergebnis in einem Satz

Die Solar-Messung wurde von der engen AIA-Leiter auf die **volle Akteur-
Matrix** über 2013–2015 ausgeweitet (Tages-, Stunden-ereignisweise und
Sekunden-Ereignis), ein Terminal-Dashboard für beliebige omegaflow-Jobs
gebaut, und zwei rot laufende CI-Workflows (paper-check, demeter-cdn)
repariert.

## Committet (15 Commits, englische Nachrichten)

Korona/Nadel-Ⅲ-Messung:
- `b2e9876` archivar: redshift fabrication gelöst (absent z → skip, nie 0.0)
- `28c4b10` archivar: eve_lines/aia_lines Orphan-Module entfernt
- `1f93478` AIA-2013 drittes Bestätigungsjahr (524 Ev, fam-still)
- `85b2bca` Multi-Akteur-Blatts auf den Kaskaden-Pfeil; Paper v5 (family bound)
- `e963d33` Register + Handover (Rat-Verdikt family bound)
- `07017b8` voller 3-Jahres-Ereignis-Stack (2156 Ev) + AIA-2015-Korrektur (613)
- `34fb6ba` Solar-alle-Akteure-Tages-Matrix (2013–2015)
- `de26baa` solar_hourly_event_probe (Stunden-ereignisweise)
- `0ceaacb` Register: Stunden-ereignisweise Befund (borderline Flare-Ko-Var.)

Werkzeuge + CI:
- `f015d00` job_monitor (Terminal-Dashboard lokale Jobs + CI)
- `b48532f` job_monitor polish (Selbst-Listing, Lokalzeit, Fortschrittsbalken)
- `e703f90` job_monitor vivid (Farben)
- `9493fd2` job_monitor terminalbreiten-bewusst
- `82d350b` job_monitor fancy (Frame-Puffer, System-Meter, Metriken, Spinner)
- `c2c2bdc` paper gate: corona abstract 198 W, sha-Fixes
- `3c5dd15` ci: demeter-cdn Crate-Fix

## Gemessene Zahlen (verlässlich)

- **AIA-2013** (GOES-15, 524 Ev, Volljahr): 193→211 +1.67e-1, 211→335
  +7.71e-2, 335→94 +1.01e-1 (alle ~96 s, Lag 4), fam 1.71e-1 — family bound.
- **Voller 3-Jahres-Ereignis-Stack** (2156 Ev, alle drei Jahre in einem Stack,
  aia_three_year_probe): 193→211 +1.68e-1, 211→335 +7.42e-2, 335→94 +1.19e-1,
  fam 1.79e-1 — family bound, kein Rung über fam. (Korrigiert: 2015-Volljahr
  trägt 613 Ereignisse, nicht 281 wie früher committet auf partiellem GOES.)
- **Tages-Matrix** (13 Akteure = F10.7, Lya1216, XRSA/XRSB, Bz, Density, 7
  AIA-Bänder; 156 gerichtete Paare, 2013–2015): fam 3.01e-1, kein Pfeil über
  fam, 32 family bound, 124 still. Kaskaden-Rungs auf Tages-Skala still
  (die ~96-s-Flare-Struktur ist im Tagesmittel verschmiert).
- **Stunden-ereignisweise** (11 Akteure, 636 Flare-Ereignisse): fam 2.68e-1;
  EIN Paar knapp darüber — 211A→193A (heiß→kühl, lag 0 h, D 2.70e-1, pos 86%).
  Rat-Verdikt: borderline/threshold-edge **Flare-Ko-Variation** (zeitgemittelte
  Flare-Hüllkurve), kein unabhängiger Kanal. Die 24-s-kühl→heiß-Kaskade
  (193→211→335→94 aufwärts, ~96 s) bleibt unberührt als separates Mass.
- **Sekunden-Matrix** (9 Akteure = 7 AIA + XRSA/XRSB): läuft als systemd-Unit
  `solar-seconds-matrix` (2514 Ereignisse erkannt); Ergebnis noch ausstehend.

## Offen / nächste Schritte

1. **Sekunden-Matrix auswerten** (`solar_seconds_matrix_probe`, läuft als
   systemd-Unit `solar-seconds-matrix`). Bei Abschluss den Befund registrieren.
   Log: `journalctl --unit=solar-seconds-matrix`.
2. **Source-Curation automatisieren + dokumentieren** (der Wunsch der letzten
   Chat-Runde, noch NICHT umgesetzt): der Prozess "neue Sources finden"
   (harvest → lens → probe → Disposition) lebt nur in gitignored
   `phi/pipeline/`, kein LLM kennt ihn, der Weekly-CI `probe-sweep` ist
   prinzipbedingt kaputt. Der Rat hat die Architektur bestimmt (Detail unten).
3. **job_monitor**: optional `--limit` für mehr CI-Runs, oder Filter auf
   Fehlgeschlagene; optional `bin/install-job-monitor.sh` ins Repo.
4. **Fremde parallele Session-Arbeit** (31 uncommittete Dateien: hdf5,
   Weberin-Compiler, galileo-Probes, archive_search etc.) — nicht von dieser
   Session, nicht angefasst.

## Rat-Architektur für die Source-Curation (noch zu bauen)

Konsultiert für den Automatisierungs-Wunsch. Empfehlung:
- **Doc:** Discovery-Leiter gehört in `docs/SOURCE_PORT.md` (nicht als zweite
  Wurzel) — Abschnitt "Discovery-Ladder (Ernte + Probe)" mit exakten Kommandos
  (lens → urls → draft → draft-context → probe), Herkunft von
  `probe_url_candidates.txt`, Rolle von `library.φ` (kuratiert, committen) vs
  `weights_*.txt` (abgeleitet, lokal), und der Disposition.
- **Automatisierung:** zwei neue benannte Rust-Bins — `source_url_candidates`
  (liest master_urls + positive Gewichte → URL-Liste; Schwelle als benannte
  Konstante) und `probe_sweep` (verkettet die 5 Stufen → Review-Bericht). Der
  Bericht (survivors/void) wird committet.
- **Ehrlicher CI:** kaputten Wochen-Cron `probe-sweep` entfernen (er
  fabriziert eine 14k-URL-Ernte aus nichts). Optional Smoke-Test mit
  committetem Mini-Seed. Die CDN-Pflicht bleibt kernel-flatten (greift erst
  wenn ein Kandidat in `sources.φ` aufgenommen ist).
- **Sofort-Reparaturen:** `katalog/`→`catalog/` in probe-sweep.yml +
  kernel-flatten.yml; toten awk-Filter (`frame ausstehend`→englisch) entfernen
  bzw. die Sperre in `probe_mode` verschieben; `--gold`→`--port`
  (SOURCE_PORT.md:79,233, parser-magic.md, sources-v2-spec.md);
  `|| true` in der lens-Schleife streichen (absent = pending, nie verschluckt);
  `library.φ` committen (z. B. `phi/library.φ`).
- **Maschine vs. Operator:** Maschine entscheidet Messungen (lebt/gewicht/
  probe/classify survivor/void); Operator entscheidet Disposition (Force-Gate,
  Spiegel-Gate, Duplikat, research-then-dead). Disposition wird NICHT
  automatisiert.

## Datenlage

- Alle Solar-Daten dauerhaft unter `data/`:
  `data/jsoc.stanford.edu/aia2013/2014/2015_fullyear.bin` (+ Monate),
  `data/ncei.noaa.gov/goes15_2013/`, `goes15/` (2014), `goes15_2015/`
  (GOES-15-Trigger, 2-s xr_*.nc). Reports unter `data/reports/`.
- Die AIA-Monats-Assets (2013/14/15) sind auf dem CDN manifestiert
  (jsoc.stanford.edu). 2013-Monat-11 via CI nachgeholt (Run 34042431334).
- job_monitor Binary: `target/debug/job_monitor`; Desktop-Button:
  `~/.local/share/applications/omegaflow-job-monitor.desktop` (gnome-terminal).

## Wichtige gemessene Befunde / Lehren

- **Kadenz-Grenze:** kontinuierliche O(n²)-TE über 3 Jahre ist nicht baubar
  (24-s: 1.55e13 op/Paar; Stunden: ~80 h/Jahr). Deshalb die Ereignis-Stacks.
- **Zeitachsen-Konventionen** (für jede Solar-Serie unterschiedlich):
  goes/euvs = J2000 TDB (`tdb_to_unix`); omni2 daily/1h = plain Unix; AIA =
  J2000 TT-no-leap (`t + 946728000 - 32.184`); f107 = i64 Tage seit 1970.
- **Richtungskonvention:** `transfer_entropy_lag(x, y)` — y ist der Driver;
  Label "A -> B" = "A treibt B".
- **paper-check/export_latex:** abstract ≤ 200 Wörter, sha256-Header muss mit
  dem Body übereinstimmen (bei jeder Paper-Änderung neu berechnen).
- **Git-Stand:** lokal 1 Commit voraus (397114a, parallele Session — nicht
  diese); alle 15 Commits dieser Session sind gepusht. 31 uncommittete fremde
  Dateien bleiben unangetastet.
