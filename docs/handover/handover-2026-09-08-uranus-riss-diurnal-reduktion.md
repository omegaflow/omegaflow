<!--
  title: Handover — Uranus-Riss: die Diurnal-Reduktion (der absolute Offset) + der roemer_fold-Knoten
  class: handover
  date: 2026-09-08
  sha256: e24833976598472ba7f52ca64d9dedc5882a9b80cd1f155f9e1a1edc24885f84
  status: live
  see-also: docs/handover/handover-2026-09-08-uranus-riss-schiedsspruch.md docs/befund/befund-2026-09-08-uranus-riss-schiedsspruch.md docs/concepts/die-weberin.md docs/TODO.md
-->

# Handover — Uranus-Riss: die Diurnal-Reduktion + der roemer_fold-Knoten

Übergabe für die nächste Sitzung. Das Atom: die **volle topozentrische +
Aberrations-Reduktion** bauen, den ~200 mas Diurnal-Term aus dem absoluten
Satelliten-Residuum entfernen und damit den absoluten Baryzentrum-Offset
sichtbar machen — erst dann ist der Eisriesen-Schiedsspruch entscheidbar.

## 0. Die abgebende Sitzung ist ein abgeschlossenes Atom

Die Schiedsspruch-Arbeit ist gebaut, gemessen, registriert, committet —
Abschlüsse: `e4e6e59` (Blatt `uranu_j`), `7108bab` (Wurzel: fünf
Satelliten-Tabellen), `a55ce3d` (Mondmodell-mas-Niveau-Befund + Manifestor).
Drei gemessene Befunde stehen:

1. **Der Riss ist echt und durch die Wurzel sauber gemessen: ~32–47 mas**
   (DE−INPOP 32.1, DE−EPM 39.5, INPOP−EPM 47.0; Standardfehler ~1,3 mas).
2. **Das Mondmodell ist mas-Niveau** — ura111 ≈ ura184_part-3 auf 5–25 mas,
   der anfängliche Verdacht „ura111-Fehler" war falsch (grind-pro gemessen).
3. **Der absolute Offset liegt unter einem ~200 mas common-mode Diurnal-Term**
   (Parallaxe/Aberration), der sich in den paarweisen Differenzen kürzt, aber
   den absoluten Baryzentrum-Offset („welche Linie ist richtig") verdeckt.

## 1. Das Atom — den Diurnal-Term entfernen

Das absolute Satelliten-Residuum (geozentrisch astrometrisch, Lichtzeit) trägt
RMS ~220 mas, Mittel ~165 mas, mit einer **~140 mas-Schwankung über ~1,2 h**
(Tageszeitskala). Das ist die Signatur eines Diurnal-Terms — Parallaxe,
Diurnal-Aberration, oder beides. Gemessen, aber **noch nicht zerlegt**.

Die drei Linien (DE441/INPOP19a/EPM2021) stimmen auf ΔRMS ~6 mas überein; der
Diurnal-Term ist common-mode (alle Linien gleich). Der Riss (32–47 mas) ist
deshalb in den paarweisen Differenzen bereits sauber — aber die **absolute**
Aussage braucht den Term weg.

## 2. Die konditionierte Messung — vorab genagelt

- **(i)** Nach der Reduktion liegt eine Ephemeride im absoluten Residuum messbar
  näher bei null → Schiedsspruch.
- **(ii)** Alle drei liegen innerhalb der Beobachtungs-Unsicherheit (~88 mas) →
  der Riss ist kein beobachtbarer Widerspruch im absoluten Sinne.
- **(iii)** Alle drei bleiben ≥ Unsicherheit entfernt → Systematik, die kein
  Modell trägt.

## 3. Der technische Pfad

**Vorhanden (alles committet):**
- `tools/measure/src/bin/uranus_satellite_schiedsspruch_probe.rs` — die
  Wurzel-Probe (geozentrisch astrometrisch, `roemer_fold`-Lichtzeit,
  `--spk <name>` wählt das Mondmodell). Der Riss steht im Report
  („Pairwise mean-residual differences").
- `tools/measure/src/bin/uranus_riss_schiedsspruch_probe.rs` — die Blatt-Probe.
- Daten lokal: `data/vizier.cfa.harvard.edu/camargo_{uranu,ariel,umbri,titan,obero,miran}_j.tsv`;
  `data/naif.jpl.nasa.gov/ura111.bsp` (169 MB) + `ura184_part-3.bsp` (386 MB,
  die Major Moons 701–705, center 7, Type 2, baryzentrum-relativ — der
  mas-Niveau-Mond). **Nicht verwenden:** `ura117.bsp` (irreguläre Monde 716–724).
- Manifestor: `tools/harvest/src/bin/camargo_uranus_manifestor.rs` +
  `.github/workflows/camargo-uranus-cdn.yml` (gebaut, CI-Lauf steht aus).

**Gemessen offen (die eigentliche Arbeit):**
- Die Observatorien-Geodäten stehen in der Probe (MPC 874: λ −45.5825°,
  φ −22.534444°, h 1810.7 m — Camargo+ 2015 §2).
- **Die naive Topozentrik ist gemessen falsch:** `station = geocenter + topoff`
  (mit `topoff = body_fixed_to_icrs("earth", …) − body_barycenter_position("earth", …)`)
  macht das Residuum **schlechter** (598 mas statt 220 mas). Die Sitzung
  prüfte es und entfernte es wieder. Der erste Zug der Folgesitzung: den
  Diurnal-Term **zerlegen** — Parallaxe gegen Diurnal-Aberration — statt blind
  eine Topozentrik draufzulegen. Referenz ist Horizons:
  - `q1` (astrometrisch) = geozentrisch, Lichtzeit, keine Aberration;
  - `q2` (apparent) = topozentrisch, Lichtzeit + Parallaxe + Aberration (der
    Aberrations-Anteil ist ~20″, die Parallaxe ~0,45″ — die Zerlegung muss den
    Diurnal-Anteil isolieren, nicht das Ganze).
- Die Quelle Camargo+ 2015 reduziert mit SOFA/NOVAS (Geozentrum→Topozentrum) und
  nennt die Positionen „astrometrisch" — aber die **Satelliten-Tabellen** tragen
  empirisch einen Diurnal-Rest (daher der Term). Diese Diskrepanz ist der Kern:
  was genau tragen die publizierten Satelliten-Positionen (Parallaxe? Diurnal-
  Aberration? beides?), muss an der Quelle (NOVAS/SOFA-Konvention, Paper §4)
  gemessen werden, bevor eine Reduktion gebaut wird.

## 4. Register-Disziplin

Jede Zeile trägt die Reduktions-Konvention (geozentrisch/topozentrisch,
astrometrisch/apparent, welche Aberration). Der Verdict-Satz lautet
„Ephemeride X trägt die Beobachtungen näher (ungewichtetes RMS)", nie
„korrekter". Die paarweisen Mittelwert-Differenzen (der Riss) bleiben der
Eichanker: sie müssen unter jeder Reduktions-Änderung auf 32/39/47 mas stehen
bleiben — kippt der Riss, ist die Reduktion falsch, nicht die Messung.

## 5. Der zweite offene Punkt — der roemer_fold-Knoten

`roemer_fold`/`light_time_sc_pos` existiert **sechsfach** probe-lokal:
`topocentric_coupling_probe`, `pioneer_navio_residuum`, `pioneer11_odf_residuum`,
`pioneer_link_correction_probe`, `uranus_riss_schiedsspruch_probe`,
`uranus_satellite_schiedsspruch_probe`. Heben in die Archivar
(`src/archivar/motion.rs`) beim nächsten Template-Griff. **Aber:** die
Parallel-Session arbeitet gerade aktiv in `src/` (ihre uncommitteten Änderungen
an `src/mathematikerin/te.rs` brechen aktuell `nobel_probe_laic`). Das Heben
**erst**, wenn `src/` wieder ruhig ist — sonst vermischt sich der Refactor mit
fremden Hunks.

## 6. Arbeitsregeln für die bauende Sitzung

- **Bauen, nicht registrieren.** Der Diurnal-Term ist ein Bau, keine
  Register-Zeile — die Zerlegung (Parallaxe vs Diurnal-Aberration) wird an der
  Quelle gemessen, die Reduktion gebaut, der absolute Offset gemessen.
- **Geteiltes Repo:** parallele Sessions committen gleichzeitig. Nur eigene
  Dateien stagen, parallele Hunks nie committen, `.git/index.lock` kurz abwarten.
- **Commit-Gate:** `cargo check` null Fehler/null Warnungen; `commit_check`
  blockt `unwrap_or_else(`, `unwrap_or_default(`, `unwrap_or(0.0)`,
  `#[derive(Default)]`, Deutsch-in-Code. Code/Diagnostik ist Englisch.
- **Sub-Agenten erwünscht** (die abgebende Sitzung hat Manifestation +
  Mondmodell-Split an grind-flash/grind-pro gegeben — mit messbarem Gewinn).
  Die Messung selbst (Zerlegung, Verdict, Register) ist das Urteil der Sitzung.
- **0-Kanon:** der Diurnal-Term ist eine gemessene Größe — `absent` bleibt
  absent, ein unbekannter Aberrations-Anteil ist `pending`, nie ein fabrizierter
  0.0.
