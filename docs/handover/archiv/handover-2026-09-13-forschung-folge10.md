<!--
  title: Handover — Forschung-Folge X (Stand 2026-09-13)
  session: Forschung-Folge X
  class: handover
  date: 2026-09-13
  sha256: 25014de0603398176ebb3f15ea22a5da8b5300e6c373fc64fbd3c0a342ba509d
  status: live
-->
# Handover — Forschung-Folge X (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅺ Placebo — der MAT-v5-Reader liest das echte ds005034-Paar; der erste
  Live-Lauf misst: das Placebo hält (verum E1↔E2, 0 Pfeile über fam-Schwelle).
  Offen: die bedingte TE mit --c-Gemeinursache-Kanal (kein --c genannt; die
  BIDS-Sidecar trägt EEGReference = Cz — auf den Referenzkanal zu bedingen ist
  degeneriert, der Operator benennt einen Nicht-Referenz-Kanal).

## Galileo-Floor

- Rausch-Kurve — Wächter: das galileo-trk-noise-Artefakt kann nicht landen:
  Run 34726993957 failure — `galileo_odf.bin` fehlt am CDN, weil
  `galileo_odf_compiler` 23 ODF-Dateien mit je 0 kept (6 discarded) liest; der
  Format-1-vs-2-Zweig bleibt ungemessen, bis der ODF-Doppler-Extrakt etwas hält.

## Weberin

- VLBI-Winkel-Probe pending (PRIDE ΔDOR nicht publiziert).

## Tiefenphasen

- W-Phase-CMT — der --mww-Gate ist verdrahtet (raw-Tensor → ndk::rp, Zentroid-
  Anker all-or-nothing); der Feld-Lauf mit --mww ist die nächste Messung. Der
  --kalibrier-Dispatch (Run 34726993968) maß: Artefakte landen, aber der
  Kalibrier-Gate lief leer (0 Stationen invertierten eine Tiefe, 12 geskippt,
  der Δ-Gate faltet 8 als branch-unstable) — der Vergleich NDK vs mww steht auf
  einer Station aus, die die Gates freigibt.
- Stromboli — das Vorzeichen ist im Code bestimmt (apply_station_term subtrahiert,
  der Test assertet es); offen ist die physikalische Anwendung, die einen
  nicht-nullen Term braucht (Kohärenz-Scan eines Kraterquells);
  stromboli-station-term.yml steht als Instrument (--end für ein
  Paroxysmus-Fenster).
- Tonga 2022 — die Roh-Druckwellenform bleibt blockiert (CTBTO-vDEC 403), nicht
  descoped.
- Positive Maske — diese Linie misst das 36-km-Streuungs-Schrumpfen, sobald die
  Treiber stehen (slab2_compiler ist fremde Session-Arbeit, nie als eigene
  geführt). Die Treiber (DEM-Gestalt, 3D-Modelle) sind jetzt anonym erreichbar
  (Crawl 2026-09-13) — Compiler/Registratur liegen in der Bau-Linie.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check mit Commit und Push.
