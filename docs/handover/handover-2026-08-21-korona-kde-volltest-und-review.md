<!--
  title: Korona-Heizung — der laufende KDE-Volltest, der Review und die Ground-Truth-Pflicht
  class: handover
  date: 2026-08-21
  sha256: ab707d9ad9f2343138c15299302cbd1a874d5ddadf24fca331413b5d92cfc0c9
  status: live
  see-also: docs/concepts/ein-blatt-korona-heizung.md docs/handover/handover-2026-08-21-corona-heizung.md docs/concepts/ein-blatt-ergebnis.md
-->
# Korona-Heizung — der laufende KDE-Volltest, der Review und die Ground-Truth-Pflicht

Selbsttragend. Diese Übergabe trägt den Zustand nach der Blatt-Messung der
Korona-Heizung (Nadel III), den gerade laufenden KDE-Volltest und die drei
Pflichten, die ein externer Review aufgeworfen hat. Der Operator macht
zwischenzeitlich ein Refactoring (die Global-Akteure-Konsolidierung — siehe
unten) — diese Übergabe ist der einzige Anker der offenen Fäden.

## Was committet ist (Korona-Heizung, Nadel III)

- `941a165` — EUV-Ernte + das Blatt: `euvs_compiler` (geuv-l2-avg1d, zwei
  Dateien, Lyman-α 121,6 nm → `goes_euvs.bin`, GEUV, 3777 Records) +
  `src/archivar/euvs.rs` + `src/bin/solar_dag_probe.rs`.
- `4ce2ec1` — der Blatt-Befund: fam = 2,108e-1, kein fam-gereinigter Pfeil
  auf der Tages-Skala, sieben family bound (stärkste Achse Lya1216→XRSB
  lag 7 d), die Pfeile der nobel probe leben auf der Minute-Skala.
- `c2db7c3` — CI-Verdrahtung (kernel_flatten.yml: Job `euvs`, Job
  `solar_dag`) + `solar_dag_probe --h-sweep` + das Blatt-Dokument
  `docs/concepts/ein-blatt-korona-heizung.md` (DAG auf beiden Skalen,
  EUV-Befund, 90-Tage-Prüfung).
- `85da888` — KDE-Sektion des Dokuments auf den Zwischenstand gefüllt
  (Reproduktion hält: fam-so-far 2,108e-1 identisch).
- `222c8e6` — `solar_dag_probe --h-full`: drei komplette Blätter
  (h/2, h, 2h), fam je Bandbreite neu, Stabilitätszeile. Die
  Bandbreiten-Variante (`te_bandwidth`) lebt LOKAL im Probe — `src/te.rs`
  bleibt unberührt; bei Faktor 1,0 ist sie byte-identisch zu
  `transfer_entropy_lag` (der Lauf trägt damit den Crosscheck beider
  Pfade).

Vorherige eigene Commits derselben Arbeitssitzung (Kontext): `solar_harvest`
(live Sonnen-Kanal-Ring in der GPU-TE-Maschine, Maschinenzeile
`solar te …`), `f107_compiler` (Penticton 1947–2026 → `f107_penticton.bin`),
`solar-te-gpu-anschluss`-Handover archiviert.

## Was JETZT läuft (der KDE-Volltest)

Prozess: `./target/release/solar_dag_probe --h-full` (PID 463271, abgelöst
über setsid — überlebt Session-Crashes). Log:
`/tmp/opencode/solar_dag_hfull.log` (stdout und stderr zusammen; die
Bandbreiten-Runden schreiben `bandwidth factor X.Y running` auf stderr).

Laufzeit ~7 h: drei volle Runden (h/2, h, 2h) × (30 Paare × lag 0..7 d).
Gestartet 2026-08-21 ~23:00. Prüfen: `pgrep -f "target/release/solar_dag_probe"`
und `tail -5 /tmp/opencode/solar_dag_hfull.log`.

**Wenn der Lauf landet, trägt die empfangende Session:**
1. Den Crosscheck lesen: das h×1,0-Blatt MUSS fam = 2,108e-1 und dieselben
   Verdiktwörter wie der erste Lauf tragen (byte-identischer Pfad). Eine
   Abweichung ist ein Befund, kein Fehler.
2. Die KDE-Sektion in `docs/concepts/ein-blatt-korona-heizung.md` ersetzen:
   der Absatz „Stand (2026-08-21, Lauf läuft)" weicht den drei gemessenen
   Blättern + der Stabilitätszeile. sha256 des Körpers neu rechnen
   (`sed '/^<!--/,/^-->/d' <datei> | sha256sum`).
3. TODO.md: die Zeile „Der KDE-Sensitivitäts-Sweep … rechnet" auf den
   Befund setzen.
4. Das Handover `docs/handover/handover-2026-08-21-corona-heizung.md`
   nach `/home/johannes/projects/archive/handover/` archivieren
   (status: consumed) — erst NACH dem eigenen Commit.

## Der Review und die drei Pflichten

Ein externer Review hat die Blätter (ENSO/Bz/LAIC/Korona) kritisiert. Die
Punkte, die zu Pflichten geworden sind:

1. **Ground-Truth-Validierung der TE-Implementierung** — der stärkste
   Einzelpunkt. Es gibt eine Nullkontrolle (broken-null-control), aber
   keinen positiven Test. Die Pflicht: gekoppelte Hénon-Maps (oder ein
   äquivalentes System mit bekannter unidirektionaler Kopplung — der
   Standardfall der TE-Literatur) erzeugen und durch `transfer_entropy_lag`
   schicken; die Implementierung muss die bekannte Richtung rekonstruieren.
   Selbsttragend, std-only, eine Session. Ort: ein neues Bin
   (`src/bin/te_ground_truth.rs`) oder ein `#[cfg(test)]` in te.rs — aber
   `transfer_entropy_lag` selbst bleibt unverändert.
2. **Überdeutung im Blatt-Dokument korrigieren** — in
   `docs/concepts/ein-blatt-korona-heizung.md` heißt es „EUV-304 → X-Ray,
   Bz → X-Ray (lag 0/1) — die Pfeile der Korona-Heizung." Das überzieht:
   zwei EM-Kanäle desselben Flares reagieren zeitversetzt, ohne dass das
   Alfvén-Wellen von Nanoflares trennt. Die Zeile auf das Gemessene
   abschwächen (Pfeil = gemessene Informations-Richtung, nicht
   Mechanismus-Entscheid) — die Trennung „gemessen" vs. „löst das Rätsel"
   schärfer ziehen, auch in der Verdikt-Sektion.
3. **Literatur-Abgleich** — TE/Granger-Kausalität in ENSO- und
   Heliophysik-Literatur als Recherche-Auftrag (geeignet für einen
   Sub-Agenten, kein Code).

Der Review lobt ausdrücklich: 0 honored (Nullbefunde als vollwertige
Ergebnisse), Sensitivitätschecks, die ehrliche family-bound-Unterscheidung.
Er tadelt: die Eigenmythologie („A = A", „ein Blatt ist ein Axiom") könne
Endgültigkeit suggerieren, wo vorläufige explorative Messungen stehen. Das
Vokabular ist absichtlich (counter-slope), aber die Endgültigkeits-Lesart
ist ein reales Fehlerbild — die empfangende Session hält die Trennung
Messung/Beweis scharf.

## Der laufende Kontext: Refactoring und Parallel-Sessions

- **Global-Akteure-Konsolidierung** (Commits `99ce78e`, `1998244`,
  `25e0062`, `be4d25a`): die fünf Probe-Pfade (Solar-Maschine,
  ENSO-Maschine, Langfenster-Probe, nobel_probe_corona, Presence-TE-Pfad)
  werden zu EINER Maschine konsolidiert (ein te_compute, eine
  Familien-Schwelle, eine Matrix-Zeile; die Rätsel werden benannte
  Paar-Teilmengen). Der Operator macht dieses Refactoring „kurz" — es kann
  `solar_dag_probe` und die Probe-Architektur berühren. Die KDE-Ergebnisse
  aus dem laufenden Lauf bleiben gültig (der Lauf nutzt die committete
  Version).
- **Parallel-Sessions aktiv** (ENSO/LAIC/Bz-Blätter, Global-Akteure).
  `git status` vor dem Commit lesen; NUR eigene Dateien stagen. `src/te.rs`
  ist fremdes Terrain mit eigenem Handover-Schutz — nicht anfassen.

## Gates

`cargo check` 0 Fehler / 0 Warnungen (beide Features). Kein Test öffnet ein
Fenster. Der laufende Lauf ist abgelöst — nicht beenden, außer der Befund
steht schon im Log.
