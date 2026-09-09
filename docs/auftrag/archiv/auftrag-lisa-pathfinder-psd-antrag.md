<!--
  title: Anfrage (extern) — LISA Pathfinder: Δg-Zeitreihe oder PSD-Tabelle
  class: auftrag
  date: 2026-09-08
  sha256: 0b17bcd9c967ae0048c264dc7de5320cad11bf7c5e0a1d31cd4b1306b8be8e20
  status: archived
  see-also: docs/auftrag/auftrag-lisa-pathfinder-psd.md docs/TODO.md
-->

# Anfrage — LISA Pathfinder: Δg-Zeitreihe oder PSD-Tabelle

Gesendet am 2026-09-08 (Operator).

Empfänger (gemessen 2026-09-08, aus der ESA-Pressemeldung zur Archiv-Eröffnung):
Michele Armano (michele.armano@esa.int) — Erstautor PRL 116, 231101 und zugleich
„LPF Operations and Archive Scientist"; Kopie an Paul McNamara
(paul.mcnamara@esa.int), LPF Project Scientist.

## Text (englisch, sendfertig)

Subject: LPF differential-acceleration PSD — tabulated data or Δg time series

Dear Dr. Armano,

We are reconstructing the LISA Pathfinder differential-acceleration power
spectral density in machine-readable tabular form. The published results
(PRL 116, 231101 and PRD 110, 042004) give the PSD only as figures
(Fig. 1 / Fig. 3), not as a table, and the catalogue reference
VizieR J/PhRvL/116/231101/table1 does not resolve.

The Legacy Archive (lpf.esac.esa.int/lpfsa/) holds the Δg time series, but its
TAP/ADQL interface is disabled and the archive is interactive-only, so we
cannot retrieve the series by machine.

Could you provide either
(a) the tabulated PSD (frequency, S_Δg, Brownian noise floor, cross-spectral
    phase), or
(b) the L1/L2 Δg time series (10 Hz) for the noise runs together with the
    magnetic-field, temperature and thruster channels — so we can compute the
    PSD and the cross-spectral phase ourselves?

We note the archive data is CC BY-NC 3.0 IGO; our use is non-commercial
research, and we will credit the collaboration.

With thanks,
[operator]

## Register-Pflicht

Die Antwort ist die Messung. Bis dahin bleibt der Kanal `pending` und der
Gesamtstatus `not-published` (kein offener Tabellen-Bestand). Die Modell-PSD
(Armano+ 2016, S_Δg = S_Brown + S_IFO·(2πf)⁴) ist in `lpf_psd_probe` als
publiziertes Modell ausgewertet — sie ersetzt die gemessene Reihe nicht.
