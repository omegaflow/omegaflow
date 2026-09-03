<!--
  title: Handover — flyby-2-Addendum: Auftrag gegen Bestand abgleichen, vor dem 28.09.
  class: handover
  date: 2026-09-03
  sha256: b296d1f343e80aaef4edac02c58ea7454ce30922dcacc86e7689eafb6f07b1d2
  status: live
  see-also: docs/auftrag/auftrag-flyby2-addendum.md docs/paper/flyby-path-2-falsification-metric-addendum.md docs/paper/blatt-pfeil-sturzflut-tibet.md docs/paper/flyby-path-2-preregistration.md
-->

# Handover — flyby-2-Addendum vor dem 28.09.2026

Diese Session übernimmt einen Auftrag und findet einen Teil schon erfüllt
vor. Lies die gemessene Ausgangslage zuerst — nicht, um neu zu schreiben,
sondern um zu entscheiden, was wirklich noch offen ist.

## Auftrag

`docs/auftrag/auftrag-flyby2-addendum.md` (2026-08-30, pending): ein Addendum
mit **σ-Metrik + Vorhersagekette** nach dem Muster des Trishuli-Blatts, mit
**Siegel-Zeile vom Operator**. Harte Frist: **vor dem 28.09.2026** (JUICE-
Erd-Vorbeiflug 28./29.09.).

## Gemessene Ausgangslage (2026-09-03)

- `docs/paper/flyby-path-2-falsification-metric-addendum.md` existiert bereits
  (2026-08-28, `status: live`, sha256 9ad59892…): es **versiegelt die
  σ-Metrik** — normalisierter Kanal-Residuum RMS über die Perigäums-Röhre von
  (gemessen − präregistriert)/σ, Schwelle = fam (phasenrandomisierte
  Surrogate, Mehrfachvergleichskorrektur). Die ursprüngliche
  Präregistrierung (`flyby-path-2-preregistration.md`, sha 6f24f98a…) bleibt
  unangetastet. Zwei Trajektorien-Siegel: JUICE 28.09. (aeb3c82f…),
  Europa Clipper 03.12. (dae553fb…).
- Das Addendum trägt die σ-Metrik, aber **keine „Vorhersagekette"** nach
  Trishuli-Muster und **keine Siegel-Zeile vom Operator** (gemessen: kein
  Treffer für Vorhersagekette/Trishuli/Siegel im Dokument).
- Das Trishuli-Muster (die Vorlage): `docs/paper/blatt-pfeil-sturzflut-tibet.md`
  — ein Kausalpfeil-Blatt mit messbarer **Vorhersage-/Kausalkette**
  (Zeitstempel, nicht erzählt) und Surrogat-Schwelle (mean + 2σ).

## Die eine Entscheidung dieser Session

**Ist das bestehende Addendum die vollständige Lieferung des Auftrags
(Auftrag schließt), oder fehlt ihm die Vorhersagekette nach Trishuli-Muster
plus Operator-Siegel-Zeile (Addendum ergänzen)?** Nicht blind duplizieren —
die σ-Metrik ist da. Abgleichen gegen den Auftragstext Wort für Wort, dann
Verdikt. Fehlende Teile benennen als `pending`, nie erfinden (0 honored).

Falls ergänzt wird: die Vorhersagekette nach dem Trishuli-Blatt-Muster
bauen (präregistrierter Feldzustand → in-situ-Messung je Kanal, mit
Zeit-Transit-Korrektur), die σ-Metrik aus dem Addendum als Maß referenzieren;
die Siegel-Zeile **setzt der Operator**, nicht die Maschine (Kernregel).

## Regeln

- Kein Siegel ohne Operator-Wort. Die Metrik wird gemessen, nicht behauptet.
- Die ursprüngliche Präregistrierung und die zwei Trajektorien-Siegel bleiben
  unangetastet; header-sha nach jedem Edit nachziehen.
- 0 honored: pending bleibt pending, nie 0.0; keine mm/s-Zahl (0 honored).

## Lieferung

Addendum committet (σ-Metrik vorhanden, ggf. Vorhersagekette + Operator-
Siegel), mit dem Abgleich-Verdikt im Commit-Text: entweder „Addendum war
vollständig, Auftrag geschlossen" oder „Vorhersagekette + Siegel ergänzt".
Vor dem 28.09. committet = Frist gehalten.
