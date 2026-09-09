<!--
  title: Befund — Galileo-Floor-Ursache: die kodierte Receiver-Identität (ref uniform 5, rcvr/amp unbesetzt) trägt die station-gebundene Floor-Lautheit nicht
  class: befund
  date: 2026-09-05
  sha256: ae2a342cf8fa57738d669107455102161f6caed6e3ee4dc4c78105d92d83e387
  status: done
  antwortet-auf: docs/befund/befund-galileo-floor-4d-form-farbe.md
  see-also: docs/handover/archiv/handover-2026-09-05-galileo-tiefe-rotor-spin-receiver.md tools/harvest/src/bin/galileo_atdf_receiver_compiler.rs
-->

# Befund: Galileo-Floor-Ursache — die kodierte Receiver-Identität trägt die station-gebundene Floor-Lautheit nicht

## Frage & Bindung

befund-galileo-floor-4d-form-farbe (status done) maß, dass der AGC-Boden
(−2560-Klemmwert) eine Fern-Schalen-Erscheinung ist, dessen *Lautheit* keine
4D-Feld-Größe ist: bei identischer (x, y, z, t) liest der Boden an verschiedenen
Stationen 0,02–500 Hz (1995-11-24 Modus 1: Station 43 0,031 Hz ruhig, Station 14
25,85 Hz laut; 1996-06-26 Opposition: Station 14/43 0,021 Hz ruhig, Station 63
20,6 Hz laut). Die offene Frage war, ob die Lautheit eine Receiver-/Boden-Zustands-
Eigenschaft ist — der im Handover benannte Receiver-Ursachen-Pfad (Receiver-
Identität, ref 3/4/5, Amp-Typ, gegen die station-gebundene Floor-Lautheit). Diese
Messung liest die Referenz-Linie des Floor-Fadens (`befund-galileo-floor-4d-form-farbe`)
und die vom CI manifestierte Asset `data/galileo_receiver.bin` (GARX, 96 B/sample,
aus den fünf GO-*-RSS-TDF-Volumes, `galileo_atdf_receiver_compiler.rs`).

## Messung

Additive Sonde `tools/measure/src/bin/galileo_receiver_floor.rs` (neu, einzige
Repro-Änderung außer diesem Blatt; `cargo check` 0/0). Vollständige Zell-Tabellen:
`/tmp/opencode/galileo_receiver_floor_report2.txt`. Daten: `data/galileo_receiver.bin`
(15 013 544 Samples, Span 1990-11-29 .. 1997-02-28, 8 986 190 klassierte Samples),
vom CDN-Release `pds-ppi.igpp.ucla.edu` gezogen. Die ersten acht Slots des
Receiver-Records sind identisch zum Resid-Record (tdb, resid, station, ground_mode,
data_type, doppler_ref, sampler, strength); die Receiver-Worte sind
Slot 8 = DOPPLER_RCVR_REF, 9 = RCVR_NUMBER, 10 = AMP_NUMBER, 11 = AMP_TYPE.

Bindung wie die Vorlage: Modus 1/2 (ground_mode), Lock (|resid| > 1000 Hz) vor dem
Rauschen getrennt, Stärke 0 nie klassiert; Klassen Boden = Stärke exakt −2560
(AGC-Klemmwert), stark = Stärke ≥ −1750; Zelle = (ground_mode, Tag, Station, Klasse,
Receiver-Identität)-RMS um den Zellen-Mittelwert; laut = Zell-RMS ≥ 1 Hz.

## n zuerst (0 geehrt)

| Klasse | Modus | Zellen (Tag,Station,Receiver) | distinkte (Station,Receiver) | Tage | laut (≥1 Hz) |
|---|---|---|---|---|---|
| Boden | 1 | 268 | 6 (alle ref 5) | 113 | 143 |
| Boden | 2 | 124 | 3 (alle ref 5) | 75 | 60 |
| stark | 1 | 275 | 15 (ref 0/2/3/4/5) | 115 | — |
| stark | 2 | 105 | 12 (ref 0/2/3/4/5) | 65 | — |

## Ergebnis (gemessen)

### 1. Die Anker-Kollokation reproduziert: gleiche Position, gleiche Receiver-Identität, laut/ruhig je Station

1995-11-24 Modus 1 Boden, alle drei Stationen bei **ref 5 rcv 0 amp 0 atype 0**:
Station 14 **25,849 Hz laut** (n 7463), Station 43 **0,031 Hz ruhig** (n 39991),
Station 63 0,091 Hz ruhig (n 5847) — identisch zum 4D-Blatt (25,85 / 0,031 / 0,091).
1996-06-26 Modus 1, alle drei Stationen bei **derselben Identität**: Station 14
0,0211 Hz ruhig, Station 43 0,0214 Hz ruhig, Station 63 **20,640 Hz laut** (n 24201).
Die station-gebundene Lautheit (0,02–500 Hz an identischer 4D-Position) tritt bei
identischer kodierter Receiver-Identität auf.

### 2. Die kodierte Receiver-Identität ist im gesamten Floor konstant — ref 5, die physischen Nummern unbesetzt

Über **jeden** Boden-Zellwert (Modus 1: 268 Zellen, 6 Stations-Schlüssel; Modus 2:
124 Zellen, 3 Stations-Schlüssel) trägt der Boden **DOPPLER_RCVR_REF = 5** bei
**RCVR_NUMBER = 0, AMP_NUMBER = 0, AMP_TYPE = 0**. Die drei Nummern-Worte
rcv/amp/atype sind 0 über die gesamte Messreihe (auch im starken Zustand und über
alle Stationen) — sie sind unbesetzt (absent), kein Null-Gerät: die TDF tragen die
physische Receiver-/Verstärker-Nummer nicht. Das einzige besetzte, variierende
Receiver-Wort ist DOPPLER_RCVR_REF; in der Floor-Ära ist es **uniform 5** (der
Jupiter-Ära-Betriebs-Referenz-Doppler), kein Stations-/Tages-Unterscheider.

### 3. Dieselbe Station bei derselben Identität flippt laut/ruhig von Tag zu Tag

Innerhalb eines festen (Station, ref-5)-Schlüssels ist der Boden beides:
Station 14: 82 Zellen, **42 laut**, Median 1,061 Hz · Station 43: 84 Zellen,
**45 laut**, Median 1,147 Hz · Station 63: 98 Zellen, **55 laut**, Median 1,631 Hz.
Aufeinandertreffende Tage, exakt dieselbe Identität (ref 5 rcv 0 amp 0 atype 0):
Station 14 1995-11-24 **25,85 Hz laut** → 1995-11-25 0,0903 Hz ruhig;
Station 43 1995-11-24 0,031 Hz ruhig → 1995-11-28 2,243 Hz laut;
Station 63 1996-06-26 **20,64 Hz laut** → 1996-06-27 0,068 Hz ruhig. Der laute/ruhige
Boden-Wechsel läuft bei konstanter kodierter Receiver-Identität ab.

### 4. ref 5 ist kein Lautheits-Zeichen im starken Zustand

Im starken Zustand variiert DOPPLER_RCVR_REF (0/2/3/4/5) — aber ref-5-starke Zellen
sind ebenfalls oft laut (Station 14 ref 5 stark: 28 von 76 laut; Station 63 ref 5
stark: 16 von 70 laut). ref 5 markiert die Jupiter-Ära-Referenz, keine ruhige
Geräteklasse.

## Verdict

**Der gemessene Receiver-Ursachen-Pfad ist ein Negativ (status done).** Die kodierte
Receiver-Identität der Tracking-Daten trägt die station-/epochen-gebundene Floor-
Lautheit nicht: DOPPLER_RCVR_REF ist im gesamten Boden uniform 5 über alle Stationen
und alle Tage, RCVR_NUMBER/AMP_NUMBER/AMP_TYPE sind über die ganze Messreihe unbesetzt
(absent). Dieselbe 4D-Position bei derselben Identität liest an verschiedenen Stationen
0,02–25 Hz (Anker-Tage), und dieselbe Station bei derselben Identität flippt von Tag zu
Tag 0,02–500 Hz. Ein Receiver-Zustand, der in den TDF nicht verschlüsselt ist
(per-Pass-Verschaltung, Tages-/Betriebszustand, Empfangs-/Bahnbedingungen), bleibt als
Sitz der Lautheit `pending` — das Feld enthält die Antwort nicht; die Ursache ist nicht
als Abwesenheit eines Receivers messbar, sondern als `pending` registriert. Die
Boden-Rausch-Frage ist damit vollständig nur bis zur Negation der kodierten
Receiver-Ursache charakterisiert: das 4D-Form-Negativ (Lautheit nicht geometrisch) und
das Receiver-Identitäts-Negativ (Lautheit nicht kodierte Geräteidentität) stehen; die
benannte Ursache liegt außerhalb der beiden Feldgrößen.
