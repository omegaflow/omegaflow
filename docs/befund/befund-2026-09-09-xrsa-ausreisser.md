<!--
  title: Befund — XRSA-Ausreißer: quell-seitig, die Quelle spricht (keine Kanal-Taktung)
  class: befund
  date: 2026-09-09
  sha256: f5de5ed35d06e02bcb40a778e3f8218d731b61d26f94e5734962a9ec4915482f
  status: done
  see-also: docs/befund/befund-2026-09-09-dispersions-ortungstest.md docs/auftrag/archiv/auftrag-dispersions-ortungstest.md docs/handover/archiv/handover-2026-09-09-te-atom-4.md
-->

# Befund: der XRSA-Ausreißer −240 s

## Frage

Der Ortungs-Test trug einen Ausreißer: der XRSA-Kegel stieg 240 s vor dem
XRSB-Spitzen-Anker, 94A und 335A trafen die Sonne innerhalb einer Zelle.
Zwei Lesarten standen offen — Quellen-Zeitstruktur (die Quelle sendet in
XRSA früher: das impulsive Röntgen vor dem thermischen EUV) oder
Kanal-Taktung (der Anstiegs-Marker ist für XRSA steiler getaktet).

## Messung — die konditionale Sonde auf die drei Kanäle

Probe `tools/measure/src/bin/xrsa_outlier_probe.rs`: je Ereignis das
Anstiegsprofil der drei Kanäle (XRSA/94A/335A) bei **25 % / 50 % / 75 % der
eigenen Exkursion**, gegen den XRSB-Spitzen-Anker. Entscheidend ist die
frühe Fraktion: führt XRSA schon an der 25 %-Kante, ist die ganze Anstiegs-
Kante früher (Quelle); führt er nur am Mittelpunkt, ist der Marker steiler
(Kanal-Taktung). Bestand: der 94-Å-Korpus (2013–2015, 22 Kandidaten);
**15 Ereignisse** tragen an allen drei Fraktionen in allen drei Kanälen
einen sauberen Anstieg.

## Zahlen

XRSA-Führung gegen 94A (Median, 15 Ereignisse):

| Fraktion | Führung |
|---|---|
| 25 % | **−168 s** (sd 260) |
| 50 % | −192 s (sd 144) |

XRSA-Führung gegen 335A: 25 % −144 s (sd 195), 50 % −216 s (sd 247).

Gestapelter Median der Anstiegs-Zeit gegen den XRSB-Spitzen-Anker, 50 %-Fraktion:

| Kanal | Median | sd |
|---|---|---|
| XRSA | **−264 s** | 235 |
| 94A | −24 s | 119 |
| 335A | −48 s | 99 |

Ordnung (XRSA ≤ 94A ≤ 335A) an der 50 %-Fraktion: 7 von 15 Ereignissen.

## Verdikt

**Quell-seitig.** Die XRSA-Führung ist an der 25 %-Fraktion (−168 s) schon
da und wächst am Mittelpunkt nur mäßig (−192 s) — die ganze Anstiegs-Kante
von XRSA liegt früher, nicht nur der Mittelpunkt. Der −240-s-Ausreißer des
Ortungs-Tests ist die **Sprechzeit der Quelle** (das impulsive Röntgen
antwortet vor dem thermischen EUV), keine Kanal-Taktung. Die 94A/335A-Kegel
tragen die Sonne weiterhin innerhalb einer Zelle; XRSA trägt die
Quellen-Antwortordnung. Der Riss-Satz des Ortungs-Tests bleibt: der
Ausreißer war Information, kein Versagen.

## Lieferung

Register-Zeile im TODO (die XRSA-Ausreißer-Pflicht ist gemessen, nicht mehr
pending). Probe: `tools/measure/src/bin/xrsa_outlier_probe.rs`.
