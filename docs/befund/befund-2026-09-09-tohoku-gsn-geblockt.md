<!--
  title: Befund — Tōhoku 2011 (GSN): die Ortung scheitert am Picker, nicht am Modell — STA/LTA streut die P-Picks um ±20–50 s an der langen M9.1-Quelle; das dichte Netz (Hi-net/JP) trägt Metadaten, aber die 2011-Wellenformen sind nicht offen erreichbar
  class: befund
  date: 2026-09-09
  sha256: c8cf8ae5329ae6dcd1446b9c34b9499cee3f4e4a2627c19ab3fe759298055f6c
  status: done
  see-also: docs/handover/handover-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-seismische-ortung-tiefe.md
-->

# Befund: der Tōhoku-Prüffall (GSN)

Feld-Herkunft: der M9-Ersteinsatz ist emergent — das Feld ortet M9 nie über
Ersteinsatz-Picker, sondern über W-Phase-CMT (Kanamori & Rivera 2008) / GCMT;
STA/LTA = Allen 1978.

## Frage & Bindung

TODO-Registerzeile „Tōhoku/Sumatra als Tsunami-Positivkontrolle — pending":
die Maschine soll ein historisches Katalog-Beben aus eigenen Ankünften orten.
Tōhoku 2011: M9.1, Katalog 38,297 N / 142,373 E / 29 km, unix 1299822384.120
(USGS, `official20110311054624120_30`).

## Der Lauf — gescheitert

19 GSN-Stationen, 15 tragen einen P-Pick (II.TLY/NIL, G.ATD, II.KIV absent).
STA/LTA 1 s/30 s, Schwelle 4,0.

- Geortet (alle 15): 35,46 / 133,56, rms 116,8 s.
- Geortet (14 Picks, 1 verworfen >3σ): 33,79 / 134,21, rms 36,1 s.
- Katalog: 38,297 / 142,373. **Offset ≈ 700 km** — keine Ortung.

## Die Diagnose — der Picker, nicht das Modell

Das ak135-Modell ist verifiziert (TauP < 0,15 s). Die Ankünfte sind gestreut:
II.TATO −214 s, II.CHTO +400 s (grobe Ausreißer), und die „guten" Stationen
tragen ±20–50 s. Die M9.1-Quelle ist ~150 s lang; der tele seismische
Ersteinsatz ist emergent, und der STA/LTA-Pick trifft einen beliebigen Punkt
auf der Anstiegsflanke statt des wahren Ersteinsatzes. Das M7.8-Indonesien
trug σ ≈ 2 s; M9.1 trägt σ ≈ 30 s. **Der Unterschied ist die Quelle, nicht die
Maschine.** Die 3σ-Verwerfung greift nicht, weil die Streuung selbst zu groß
ist (3σ = 350 s bei rms 116 s; die ±40-s-Picks bleiben drin).

## Die Netzmessung — gemessen, nicht geglaubt

- Hi-net (NI): nicht im EarthScope-Dataselect; NIED verlangt Registrierung.
- JP (JMA): 6 Stationen **Metadaten** im Tōhoku-Fenster (JMM/JSD/JTM/JYT/JSG/
  JHJ2, alle < 5° vom Beben — die fehlenden Nah-Stationen), aber der
  Wellenform-Abruf 2011 liefert **HTTP 204 (keine Daten)** — die Wellenformen
  sind nicht offen archiviert.
- Das dichte Netz („~800 Stationen") ist für 2011 nicht offen erntbar.

## Verdikt

Zwei ehrliche Blocker, beide benannt:
1. **Der Picker** — STA/LTA streut an M9.1; der wahre Ersteinsatz braucht
   Bandpass + Einsatz-Picker (AIC/Erstbruch), ein eigenes Atom.
2. **Der Netz-Zugang** — die dichten Netze tragen Metadaten, aber die
   2011-Wellenformen brauchen Registrierung (NIED/JMA).

Tōhoku bleibt als Positivkontrolle offen — nicht als „versagt", sondern als
„geblockt an Picker und Zugang". Nichts davon ist ein Fehler des Modells.

## Folge (Register)

- „Scharfe Tiefe / Nah-Stationen" bleibt pending, jetzt mit Adresse (Hi-net).
- Neues Atom: M9.1-Picker (Bandpass + Einsatz-Pick) — die Bedingung für den
  GSN-Tōhoku-Lauf.
