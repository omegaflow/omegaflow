<!--
  title: Befund — Konsolidierungs-Blatt der Galileo-Floor-Deduktionskette: die lauten Pässe sind anhaltende per-Pass-Empfangs-Out-of-Lock-Zustände der Galileo-Empfangskette dieser Station in diesem Pass (Klarstellung A: 7/7 flag-bedeckte anhaltende Episoden out-of-lock, anhaltend-bei-Lock n=0) — nicht Signal, nicht Himmel; Deduktion 27 als Empfangskette benannt (Simultanität, Open-Loop sauber, Sende-Klasse n=0), Uplink-Pfad gemessen leer
  class: befund
  date: 2026-09-06
  sha256: 5e6aba6c17f04208bfe526de4dca5c81661ab0afd32155fb0c549ff144544c60
  status: draft
  see-also: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-1996-rest-kontrast.md docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-floor-ort-himmelskoerper.md docs/befund/befund-galileo-floor-subtages-recurrenz.md docs/befund/befund-galileo-receiver-je-pass-floor.md docs/befund/befund-galileo-dreiweg-sendempfang-floor.md docs/befund/befund-galileo-dsn-passplan-uplink.md docs/befund/befund-galileo-doppler-odf-beschaffung3.md docs/befund/befund-galileo-odr-uplink-swse-anchor.md docs/befund/befund-galileo-goj-odr-ded31-sameday.md docs/befund/befund-galileo-gwe-odr-zweiter-zeuge.md docs/befund/befund-galileo-pioneer-stationsfloor-kreuz.md docs/befund/befund-galileo-pioneer-passfenster-subtaeglich.md docs/befund/befund-galileo-simultan-intrapass-trk225.md docs/befund/befund-galileo-h1-receiver-regression.md docs/befund/befund-galileo-beide-laut-simultan.md docs/befund/befund-galileo-cycle-slip-disziplin.md docs/befund/befund-galileo-elevation-ausbruchdauer.md docs/befund/befund-galileo-odr-goj-beschaffung2.md docs/befund/befund-galileo-odf-sender-uplink.md   docs/befund/befund-galileo-dsn-passplan-uplink-beschaffung1.md docs/befund/befund-galileo-ruhige-basis-drift.md docs/reference/133A.pdf docs/reference/19930020414.pdf   docs/reference/96-1505.pdf docs/reference/93-0778.pdf docs/reference/recherche-extern-galileo-ruck-borduhr-modell.md /home/johannes/Schreibtisch/rechercheauftrag-extern-galileo-floor-ded27.md /home/johannes/Schreibtisch/rechercheauftrag-extern-galileo-floor-ded27_results.md /home/johannes/Schreibtisch/rechercheauftrag-extern-galileo-floor-ded27_results_2.md
-->
# Befund: Konsolidierungs-Blatt der Galileo-Floor-Deduktionskette — die lauten Pässe sind anhaltende per-Pass-Empfangs-Out-of-Lock-Zustände der Galileo-Empfangskette dieser Station in diesem Pass, nicht Signal, nicht Himmel

## 1. Phänomen

Die Floor-Lautheit der Galileo-Boden-Ära (AGC-Klemmwert −2560, Zell-RMS um den
Zellen-Mittelwert, laut = RMS ≥ 1 Hz, Lock |resid| > 1000 Hz vor dem Rauschen
getrennt) liest auf den 70-m-Stationen 14/43/63:

- **400 robuste (Mode, Station, Tag)-Zellen (n ≥ 30): 207 laut / 193 ruhig**,
  Summe über M1/M2/M3; die station-gebundene Tages-Reihe wechselt in **105
  Nachbar-Tages-Flips** (18/14/15 + 9/12/11 + 10/9/7 je Serie). Die laut-Zellen
  der Modi 1/2 tragen **547 Boden-Pässe (314 laut / 233 ruhig)**.
- Die Flips sind **isolierte 1–3-Tage-Gipfel**: 133 von 137 lauten Episoden
  dauern 1–3 Tage; die acht längsten zusammenhängenden Fenster (8–15 Tage)
  tragen keine anhaltende Stufe.
- Anker-Tage (Tag-Zellen exakt reproduziert, n je Zelle in den Quell-Befunden):
  1995-11-24 st14 25,85 Hz laut (st43/st63 ruhig 0,031/0,091 Hz) · st14-Dreiweg
  1995-12-04 0,044 Hz ruhig → **1995-12-05 52,9 Hz laut** → 1995-12-06 0,058 Hz
  ruhig · 1996-06-26 st63 20,64 Hz laut (Opposition) · 1997-02-26 st14 ~29 Hz
  laut (closed-loop; GOJS-Window-RMS 84 Hz).
- Die Dreiweg-Zellen der Ära (Mode 3): 94 robuste Tage, 51 laut (st14 40/25,
  st43 33/16, st63 21/10).
- Die Ausbruchs-Episoden auf den lauten Tagen sind zweipopulig (Schwelle 10 Hz,
  gap 30 s): ~42 % unter ~2 s (165 Einzel-Samples — die Cycle-Slip-/Neugreif-
  Klasse) neben anhaltenden Episoden (Mehr-Sample-Median 70 s, bis ~9,4 h), die
  den Großteil der Laut-Zeit tragen.

## 2. Deduktionskette — je Hypothese: Test, Ergebnis (✗ mit n), Commit

Jede Zeile: Hypothese über den Sitz der station-gebundenen Tages-Lautheit,
getestet mit der benannten Probe (Mess-ID, reproduzierbare Definition in den
Quell-Befunden) und der gemessene Ausgang. ✗ = die Hypothese als Mechanismus
der Lautheit ist nicht getragen; n zuerst, 0 geehrt.

| # | Hypothese (Sitz = …) | Messung (Probe / Mess-ID) | Ergebnis ✗ (n) | Commit |
|---|---|---|---|---|
| A1 | kalendarische Betriebs-Ära der Stationen (BVR 9/95, 5.12.95-Wechsel, DGT 5/96, Full-Array 11/96) als Stufen | `galileo_ops_era_station_step_probe` (Richtung A): Ein-Schnitt + ankergebundene Fenster | ✗ BVR/Wartung/DGT auf der Serie unbesetzt (vor n = 0); einzige Stufen-Konzentration 5.12.95 datendünn (links n 2–13) und konjunktions-konfundiert; dominante Struktur: **114 Nachbar-Tages-Flips über alle Boden-Tage inkl. dünner Zellen** (76 % ohne Anker in ±7 d; davon **105 auf der robusten n≥30-Teilmenge** — die §1-Zählung) | `dae5062` |
| A2 | 1996er-Rest 1,9×/2,4× (Opposition gegen Konjunktion) = kleine station-stabile Geometrie-Magnitude | `galileo_1996_rest_split`: Tages-/Stations-/Pass-/Ausreißer-Zerlegung | ✗ hängt an 1–2 episodischen Pass-Tagen (ohne die zwei lautesten Mode-2-Tage: 2,79 → 0,04 Hz = 0,03×; Mode 3 ohne den einen Tag 2,40× → 1,20×); Modi dekorrelieren je Station/Tag bis 10⁴ (st63 06-28: 0,012/11,4/123,9 Hz); kein Nicht-Oppositions-Boden im 30-d-Umkreis (0 Zellen) | `4a33e3c` |
| A3 | Wochen-Treppe/Telegraph der Tages-Serie (mehrere Tage verharrendes Niveau) | `galileo_floor_stair_te` (Richtung C): Permutations-Segmentierung der 8 Fenster 8–15 d | ✗ 0 anhaltende Stufen in allen 8 Fenstern (beste Schnitte p 0,10–0,77; Kontrolllauf detektiert eine 7/8-d-Stufe mit p 0,0005); 133/137 laute Episoden 1–3 d; laute Amplitude kontinuierlich gestreut (bis 530 Hz) | `505e5e7` |
| A4 | gerichtete Kopplung: Zustand einer Station/ eines Treibers treibt den Floor (TE) | derselbe Lauf (Messung 2/3): `transfer_entropy_lag`, Phasen- und Block-Null, Reverse-Kontrolle | ✗ TE station→station nur im Mode-2-Fenster Nov–Dez 1995 messbar (n 12–14) und dort unter beiden Nullen; ε- und konstruierter 5.12.95-Schritt-Treiber ohne Kopplung über der Null (n 12–14, ein Fenster) | `505e5e7` |
| A5 | getaktetes Steuer-/Kalibrier-/Update-Signal (Sub-Tages-Kadenz, phasen-kongruente Tageszeit) | `galileo_floor_subday_clock_probe` (Richtung E): UTC-Stunden-Faltung, Lomb-Scargle mit Raster-Kontrolle, Epoch-Faltung | ✗ laut-Onset über alle 24 h verstreut (Rayleigh-R 0,304 gegen 0,250 der ruhigen Kontrolle — die Kongruenz ist die stationseigene Pass-Tageslage); keine Wiederkehr-Periode über dem Surrogat-p95 (8/9 Serien); 133/137 Episoden 1–3 d ohne feste Stations-Reihenfolge | `52866ed` |
| A6 | Sichtlinien-Ort (RA/Dec) + Himmelskörper-Distanz (Erde rE, Jupiter J, Perijove) | `galileo_floor_sky_body_probe`: Flip-Schritte, Tag-Mischung bei fester Geometrie, J-/rE-Achsen, 1996er-Fenster | ✗ alle 105 Flips wechseln bei < 1° Sichtlinien-Schritt (Median 0,18°) und 0,0052 AU; an identischer Geometrie/J je Tag gemischt laut/ruhig (M1 31/61, M2 16/27, M3 17/28 geteilte Tage); laute Zellen über Bogen ~50° und J 6,7–168 R_J verstreut; Perijove trennt die lauten von den ruhigen 1996er-Tagen nicht (06-28 gegen 06-29 bei 11,5/10,9 R_J reassigniert) | `99bac54` |
| A7 | die lauten Pässe sind die flachen (Elevations-Decke / Schwellen-Physik) | `galileo_floor_elevation_burst` (Richtung E2, Messung 1): Decken-Verteilung laut/ruhig, Flip-Decken | ✗ Decken-Verteilungen überlappen je Station vollständig (st14 med 31,82 gegen 32,93°, p 0,121; st43 76,55/77,00°, p 0,744; st63 27,67/27,62°, p 0,964); die zenitale st43 ist auf 53/80 Tagen laut wie die flachen st14/st63; 61 Flips bei Δ-Decke median 0,02–0,03° | `f6b33cd` |
| B1 | Empfänger-Generation: die Laut-Inzidenz springt an den dokumentierten Empfänger-Meilensteinen (H1) | `galileo_h1_receiver_regression`: Fisher-exakte (laut, ruhig)×(vor, nach) je Meilenstein, Station-Konkordanz, Rollout-Suche | ✗ kein Station-Tag-Test p < 0,05 an 18.9.95 / 5.12.95 / 23.5.96 / 1.11.96 (12 Tests; Grenze 18.9.95 vor n = 0); Zell-Zuwächse (gepoolt p 0,0002 bzw. 0,002) sitzen auf dem Era-Öffnungs-/SWS-Fenster und verschwinden auf Station-Tag-Ebene; kein stations-versetzter Rollout-Sprung | `d0adf13` |
| B2 | kodierte Receiver-Identität (Zell-Ebene) trennt laut von ruhig | Vorbefund Receiver-Ursache (Zell-Ebene, done) | ✗ DOPPLER_RCVR_REF uniform 5 über alle Stationen/Tage, RCVR_NUMBER/AMP_NUMBER/AMP_TYPE absent (0); die Identität flippt laut/ruhig nicht | `3c95c2a` |
| B3 | kodierte Receiver-Identität pro Pass (letzte je-Pass-Achse) trennt laut von ruhig | `galileo_receiver_pass_cross` (Richtung F): Pass-Zensus rx-Felder gegen laut/ruhig | ✗ alle 547 robusten Boden-Pässe (314 laut, 233 ruhig) tragen vollständig die Konfiguration ref 5 rcv 0 amp 0 atype 0 (RX_REF konstant 5 über 4 244 313 Boden-Samples; rcv/amp/atype 0 in allen 8 536 371 klassierten Samples); das einzige variierende Wort (starker Zustand, ref 0/4/5) trennt nicht (ära-durchziehende ref 5 beides; ref 0 in 1990-Tagen, ref 4 je Station gemischt) | `94b4719` |
| C1 | Pioneer-10-Tages-Koinzidenz an derselben Station (Sitz = die Bodenstation selbst) | `galileo_pioneer_stationsfloor_kreuz` (Richtung K): (Station, Tag)-Koinzidenz laut–laut im Fenster 1995-11-23..1997-02-28 | ✗ Koinzidenz laut–laut n = 0 (154 Galileo-laut-Zellen × 3 Pioneer-laut-Zellen, 6 gemeinsame Zellen; Pioneer 11 n = 0 im Fenster); die eine gemeinsame robuste Zelle mit Galileo-laut (st14 1995-11-26) liest gegenläufig (Pioneer −0,008 Hz ruhig, Galileo m2 ruhig bei m1/m3 laut) | `18d5156` |
| C2 | die Lautheit füllt den gemeinsamen Station-Tag (station-weiter Tageszustand über Pass-Zeiten) | `galileo_pioneer_pass_slices` (Richtung L): Pass-Zeitscheiben der gemeinsamen Zellen | ✗ alle 6 gemeinsamen Station-Tage tragen Pioneer- und Galileo-Samples in nicht-überlappenden Zeit-Scheiben (0 s Überlapp); auf st14 1995-11-26 ist der Tag nicht durchgehend laut (Pioneer-Fenster und Galileo-m2-Fenster ruhig); innerhalb Galileo: laut-Tag = laut-Pass in 139/207 laut-Zellen, 42 laut-Zellen mit ruhigem Pass am selben Tag | `d81cf6e` |
| D1 | gemeinsame Ursache bei simultanem Empfang (zwei Stationen gleichzeitig laut auf demselben Signal) | `galileo_ded27_simultan` (Ded-27): 55 simultane Zellen, 50 robust (Mode 1) | ✗ Muster 16 eine-laut / 2 beide-laut / 32 beide-ruhig — die Lautheit sitzt an der lauten Empfangsstation; in den Uplink-tragenden Modi 2/3/4 ist die Simultanität n = 0 (Mode 2 nur eine n = 7-Zelle) | `d4853fc` |
| D2 | die 2 robusten beide-laut-Zellen = gemeinsame Ursache auf Sample-Ebene | `galileo_beide_laut_simultan` (Richtung B-L): matched Paare auf dem gemeinsamen 1-s-Gitter | ✗ same-sample beide-laut 0 von 4633 bzw. 0 von 1043 matched Paaren (Unabhängigkeits-Erwartung 46,3 bzw. 10,3); Pearson r 0,037 bzw. −0,55 — zwei zeitlich getrennte per-Station-Ausbrüche im geteilten Zellfenster; die einzige sample-koinzidente Zelle (12/1996, n 7) ist gegenphasig (r −0,9998) | `7eba007` |
| D3 | gleichmäßige stationäre Schleifen-Bandbreite über den Pass (H2) | `galileo_ded27_intrapass`: 12-Fenster-Struktur der 7 lauten Anker-Pässe | ✗ 7/7 laut-Pässe nicht gleichmäßig; 5 end-konzentriert, 1 front-lastig (M2 st14, front20 0,78), 1 mehrere Burst-Fenster (M3 st63), 1 mittleres Lärmband (M1 st14); Muster = Lock-Grenz-Transienten/Cycle-Slips (H3-Familie) | `d4853fc` |
| D4 | die Transienten sind Sende-Predict-/Rampen-Sprünge der Uplink-Seite | `galileo_cycle_slip_discipline` (Richtung CS): Roh-TDF-Flags (RCVR_LOCK0/DOPPLER_GOOD0/SLIPPED_CYCLE) gegen die Ereignisse | ✗ Send-Predict-/Rampen-Klasse n = 0; 12/12 flag-bedeckte Transient-Ereignisse tragen eine Empfangs-Signatur (Out-of-Lock-/Good-bad-Flag oder Slipped-Cycle; M2 st14 19:15:50 = Counter-Slip im gelockten Lauf); die |resid| > 1000-Lock-Marker koinzidieren mit dem Out-of-Lock-Flag (M1 st14 4065/4065) | `a6f50f2` |
| E1 | TRK-2-25-Dreiweg-Record trägt die Sende-Station (Sample/Pass) | `galileo_threeway_txrx_split` (Tor 1, Ded-27): Item-Zensus über 83 614 Dreiweg-Samples aus 4 Roh-TDF | ✗ item 10 das einzige Stations-Wort (Empfang); XMTR_POWER/XMTR_FREQ/XMTR_POWER_IND überall 0, kein zweiter Stations-Wert; XMTR_ON0 mischt; atdf2ascii-Xmtr = Ramp-Record-Verknüpfung (in zwei der vier Caches zwei-deutig), 2-Weg = Empfangs-Duplikat — Ded-27 aus dem Record nicht schließbar | `5ae7743` |
| E2 | ein Doppler-tragendes ODF/TRK-2-18 der Floor-Ära existiert (trägt item 7 Sender) | `galileo_odf_sender_uplink` + Beschaffungs-Sweep (Beschaffung 3/3) | ✗ einzige ODF-Volumen GO-J-RSS-1-ODF VLBI-only (136 Orbit-Daten-Wörter, Sende-Wort 0, 0 Doppler, 0 Dreiweg; Fenster-Schnitt 24 ODF-Tage ∩ 61 Floor-Tage = 0); 5 TDF-Bände führen 0 ODF; Kandidaten anderer Bände HTTP 404 | `b185e2a` + `70ad543` |
| E3 | die Galileo-ODR-SFDU trägt TRACKING_MODE/Uplink-DSS | `galileo_odr_header_census_probe` (Richtung J): RSC-11-11-Wort-Plan + Byte-Zensus über 10 ODR-Dateien | ✗ kein TRACKING_MODE-Wort, kein Uplink-Feld; byte 6 (Prime-FEA) einzige Stations-Identität, byte 7 (Secondary FEA) = 0 in allen Records — das Uplink-DSS-Feld der Grail-RSR-SFDU ist eine spätere Recorder-Generation | `a48fb1c` |
| E4 | ein öffentlicher DSN-Tracking-/Pass-Plan der Floor-Ära nennt die Uplink-Station je Tag | Quellen-Such-Ledger (PDS-RSS-Volumina, JPL-Report-Archive, NSSDC) + HANDBK5-Appendix-A | ✗ keine erreichbare Quelle führt den per-Pass-Plan (PDS-DOCUMENT/INDEX/LBL/GEOMETRY leer; tda/ipnpr HTTP 403; HANDBK5-App-A-Pläne enden 1995-06-28, Lücke 148 d zum Floor-Beginn; USO-Log-Einträge der Ära auf Test-Tagen) | `dffc7b4` + `686a7d9` |
| E5 | ein open-loop Record bezeugt die Lautheit als empfangene Spektral-Störung (zweiter Zeuge upstream) | `galileo_odr_overlap_witness` (GWE-ODR) + GO-J/GO-JS-ODR open-loop-Proben (Fenster- und 300-s-Phasen-Auflösung) | ✗ GWE-ODR: n = 0 überlappende (Tag, Station), ODR endet 1995-06-28 (Lücke 148 d), 207 laut-Tage beginnen 1995-11-23; GO-J/GO-JS-ODR: der open-loop Trägerlinien-Ton (segsnr) bricht in den laut-markierten Sub-Phasen nicht ein (Same-Day-Splits: st14 1997-02-26 128,2 gegen 110,4, n 3/7; zwei dünne n 1/1) — die Floor-Lautheit erscheint nicht als empfangene Spektral-Störung | `f887b88` + `ce7fd58` |
| F1 | Telemetrie-Arraying/Combiner (DGT/FSR/Parkes) als per-Pass-Ursache | strukturell + zeitlich + in-Ära gegen die Architektur (133A, 96-1505, Parkes — extern) und die Messung (H1, beide-laut, Cycle-Slip) | ✗ strukturell: der TRK-2-25-Doppler entsteht je Empfangs-Station in der Closed-Loop-Radiometrie-Kette (BVR→MDA), FSR/DGT/Combiner speisen die Telemetrie, nicht die Radiometrie — der Floor-Pfad durchläuft den Combiner nie *(architektonische Aussage aus 133A/96-1505; Lese-Register der zwei Quellen als `pending` benannt, nicht in einem Quell-Befund zitiert — das ✗ ruht unabhängig davon auf dem zeitlichen und in-Ära-Pfeiler)*; zeitlich: DGT ab 23.05.1996 (96-1505), Wide-Area-Arraying/Parkes ab Nov 1996 (Parkes) — die größten Anker 1995-11-24 (25,85 Hz) und 1995-12-05 (52,9 Hz) liegen vor beidem, 1996-06-26 (20,64 Hz) nach dem DGT-Start (23.05.96) und vor dem Wide-Area-Arraying; in-Ära: die beiden-laut-Zellen der Arraying-Ära 1996-11-06 und 1997-02-27 sind sample-getrennt (7eba007), kein Combiner-Event; die DGT-Umstellung am 23.05.1996 liegt in der bodenleeren Spanne Feb–Mai 96 (0 Floor-Samples, 0 geehrt) — die H1-Laut-Inzidenz-Prüfung (d0adf13) trägt dort keine Stufe, aber der Tag selbst ist auf der Boden-Serie unbesetzt | `d0adf13` + `7eba007` + `a6f50f2` |
| F2 | ein säkularer Drift der ruhigen Resid-Basis (unmodellierte Rampen-Beschleunigung à la Pioneer) | `galileo_floor_basis_drift` (Richtung D2): Tages-Niveau-Reihe (Tages-Mittel/Median, NICHT das RMS der laut-Gipfel) + linearer Trend + ruhige Decke je Station/Mode über 1995-11-23..1997-02-28 | ✗ ruhige Basis-Niveau flach in 7/9 Serien (die Niveau-flachen Serien tragen p_t 0,12–0,90: M1 st14 0,26, M2 st14 0,156, M3 st43 0,119, M3 st14 0,900 …); nur Mode 1 langsam gesenkt (st43 −8,8e-4 Hz/d p 0,0017, st63 −9,0e-4 p 0,0004 — ~0,4 Hz über 27 Monate) als saisonale Lage-Änderung (1995-11 ~+0,7 Hz → 1997 ~0 Hz), nicht monoton; Modi 2/3 flach; die Rausch-Decke driftet nicht systematisch (7/9 flach, p_t 0,25–0,68; M1 st43 0,041 und M3 st43 0,0045 positiv); Resid-Offset klein-positiv (Serien-Median +0,06…+0,17 Hz); 3 isolierte Versatz-Tage (|Niveau| > 1 Hz bei ruhigem RMS) — Größe ≤ 0,4 Hz/27 Mon unter der 1-Hz-Laut-Schwelle: Boden-/Reduktions-Eigenschaft, kein Anomalie-Drift | `fad752b` |
| F3 | ein „Ruck" (plötzliche, anhaltende Niveau-Stufe) in der ruhigen Resid-Basis | `galileo_floor_basis_ruck` + `galileo_ruck_zeugen` + `galileo_ruck_nachmessen_f3` (Richtung D3–D5): Zwei-Segment-/Change-Point + ref_hz-Trennung + Nach-Messung + externe Borduhr-vs-Modell-Recherche (`docs/reference/recherche-extern-galileo-ruck-borduhr-modell.md`) | **EIN Ruck gemessen — er beseitigt einen Versatz**: abrupter, anhaltender Mode-1-Niveau-Sprung am Era-Übergang 1995-11-30/12-01 (−0,80…−0,82 Hz Nachbar-Tag; st14/st43/st63 −0,615/−0,637/−0,700 Hz, p_perm 0,0030/0,0095/0,0045), der das +0,75-Hz-Öffnungs-Plateau (je Station +0,775/+0,719/+0,741) auf ≈ 0 senkt (Dez-95-Folge-Ruhig-Tage +0,03/+0,03/+0,03 Hz) — der Öffnungs-Versatz war der transiente Öffnungs-Versatz der ersten Messwoche (Ursache `pending`), der Sprung entfernt ihn; nur Mode 1, simultan über alle drei Stationen; die Mode-1-Argument-Deckung am Übergang (M2/M3) ist benannt: teils echte Gegen-Daten ohne Ruck-Abfall (M2 st14 Δ−0,018, M2 st63 Δ−0,054, M3 st43 Δ−0,004 Hz), teils Lücke/laut (M2 st43 12-01..03 laut, M3 st14 11-30..12-02 laut, M3 st63 11-30 n0) — das Mode-Argument ruht auf der Gegen-Daten-Menge, die Lücken sind benannt, nicht als Gegen-Daten verkauft; resid-gebunden (ref-Hz-Spur glatt weiter), datengetrieben (drei unabhängige M1-Serien am selben Datum, 6–7 d nach Daten-Beginn, nicht an der Kante); der Meilenstein 23.05.1996 (DGT) liegt in der bodenleeren Spanne Feb–Mai 96 (0 Floor-Samples, 0 geehrt) — ein Sprung ist dort nicht beobachtbar; relativer Schritt 0,8 Hz/2,295 GHz ≈ 3,5e-10 (diskriminiert USO-Nachkalibrierung gegen Predict-Referenz-Wechsel nicht — Zahl ist für beide Kandidaten dieselbe Größenordnung); zeitliche Koinzidenz mit den dokumentierten Bord-Kommandos JOE-A/B (Sequenzladungen 30.11./1.12., Probe-Oszillatoren 27.11., TDA 42-125) — kausal offen, kein Referenzfrequenz-Event dokumentiert; Sitz (Borduhr-Event vs sub-Hz-Modell-/Predict-Korrektur) `pending` nach öffentlichem Material | `b8226ab` + `a435c56` + `bbefd67` |

Mess-IDs und reproduzierbare Definitionen stehen je Zeile in den zitierten
Quell-Befunden (Bindungen: Zell-/Pass-/Episoden-Definitionen, Schwellen,
Gap-Regeln, Surrogat-Nullen — unverändert zur Referenz-Linie; n zuerst).

## 3. Positiver Charakter

Der von Klarstellung A gestützte Befund (neue Messung
`galileo_floor_sustained_lock_state`, Report
`/tmp/opencode/galileo_floor_sustained_lock_state_report.txt`, `cargo check`
0/0):

**Anhaltender Out-of-Lock-Zustand pro Pass.** Über die 7 lauten Anker-Pässe der
Referenz (Tag-Zellen exakt reproduziert) tragen die 5 Roh-TDF-bedeckten Pässe
7 anhaltende Episoden (Spanne ≥ 60 s, Schwelle 10 Hz, gap 30 s —
Episoden-Definition siehe Abschnitt 5), alle mit voller Flag-Kreuzung:
**7/7 sitzen in Out-of-Lock-geflagten Strecken** (im engen
±2-s-Fenster je Episode 100 % der AGC-Boden-Flag-Records mit RCVR_LOCK0 =
Out-of-Lock: 133/133, 95/95, 142/142, 939/939, 857/857, 91/91, 66/66;
DOPPLER_GOOD0 = bad und SLIPPED_CYCLE > 0 durchgehend, |resid| > 1000-Marker
eingebettet und an den Rändern), Gesamt-Spanne 2700 s; **anhaltend-bei-Lock:
n = 0**; die achte anhaltende Episode der Serie (M3 st63
1995-11-27, 381 s) liegt auf dem cache-freien Pass und bleibt am Flag 0 geehrt
(flag-void). Die eine in-Lock-geflagte laute Episode der bedeckten Pässe (M2 st14
1995-11-24 19:15:31–19:16:02, 34/34 in-lock, good, SLIPPED 29) ist ein
Counter-Slip im gelockten Lauf und liegt mit 31 s Spanne unter der
anhaltenden Klasse. Auf dem lautesten Anker ist der
ganze 2-h-Lauf eine durchgehend Out-of-Lock-geflagte Strecke (M1 st14
1995-11-24: 6106/6106 AGC-Boden-Records des Laufs lock-out); die anhaltende
Laut-Zeit sitzt damit in per-Pass-Out-of-Lock-Strecken, nicht als Rauschen in
einem gelockten Loop mit Slips nur an den Rändern.

**AGC-Kontrolle der Register-Fläche (Signalstärke laut gegen ruhig, neue
Messung `galileo_floor_agc_loud_quiet`, Report
`/tmp/opencode/galileo_floor_agc_loud_quiet_report.txt`, `cargo check` 0/0).**
Über die reproduzierten 400 robusten Floor-Zellen des Registers (207 laut /
193 ruhig, 105 Flips) liegt das Signalstärke-Feld (AGC in 0,1-dBm-Einheiten)
der Zell-Mitglieder in **100 % beider Klassen am Klemmwert −2560** (laut:
207 Zellen, n = 2 106 313 Mitglied-Samples; ruhig: 193 Zellen, n = 2 539 919;
je Zelle Spanne 0, kein Mitglieds-Sample auf einem anderen Registerwert). Die
laut/ruhig-Trennung des Registers lebt damit vollständig **innerhalb eines
einzigen AGC-Werts** — ein Signalstärke-Gradient trennt die Klassen nicht, die
AGC unterscheidet laut nicht von ruhig. Auch die Tages-Komposition trennt
nicht: der mediane in-track-Anteil der Tage am Klemmwert ist in beiden Klassen
1,0000 (laut-Zell-RMS-Median 23,08 Hz gegen 0,064 Hz ruhig bei gleichem AGC);
ganz-geklemmte Tage (einziges AGC-Registerwort des Tages = −2560) zählen
97/207 laut gegen 128/193 ruhig, Tage mit einem starken Ausflug (≥ −1750)
85/207 laut gegen 45/193 ruhig. Die Gegen-Prüfung auf der starken Population
(≥ −1750) stützt keine Bindung der Lautheit an den Klemmwert allein: 322
robuste stark-Zellen der Ära tragen 85 laut / 237 ruhig (26 % laut; 75
stark-Zellen auf laut-Floor-Tagen, davon 26 selbst laut) — laut und ruhig
existieren in beiden AGC-Regimen. Der laute Floor-Zustand sitzt damit bei
**identischem (geklemmtem) Empfangs-Signalpegel** als erhöhtes Rauschen der
Empfangskette — die Morabito-Klasse „gleiches Signallevel, erhöhtes Rauschen →
Boden-Gerät" ist auf der AGC-Achse gestützt (Abschnitt 8, Präzedenzfall).

Die Konsolidierung der Negativ-Kette setzt das Positive in Kontrast: Pioneer 10
liest dieselbe Station am selben Tag ruhig, während Galileo laut ist (die
einzige gemeinsame robuste Zelle, st14 1995-11-26, gegenläufig); der
open-loop-Kanal ist auf den deckenden Tagen sauber (kein Einbruch des
Trägerlinien-Tons); die 12/12 flag-bedeckten Transient-Ereignisse tragen eine
Empfangs-Cycle-Slip-/Lock-Verlust-Signatur, die Sende-Klasse ist n = 0. Die
Lautheit ist ein Zustand der Empfangskette des Galileo-Passes dieser Station.

## 4. Deduktion 27 — Empfangskette, nicht Uplink

Drei gemessene Zeugen lokalisieren den Sitz auf die Empfangskette:

1. **Simultanität:** von 50 robusten simultanen Mode-1-Zellen (One-Way-Downlink,
   kein Boden-Uplink im Signalweg) sind 16 eine-laut/eine-ruhig und 32
   beide-ruhig, nur 2 beide-laut — und die 2 beide-laut-Zellen sind auf
   Sample-Ebene zwei zeitlich getrennte per-Station-Ausbrüche (same-sample
   beide-laut 0/4633 und 0/1043). In den Uplink-tragenden Modi 2/3/4 ist die
   Simultanität n = 0. Dieselbe Downlink-Quelle liest an einer Station laut, an
   der anderen ruhig → der Sitz ist der Empfangsort, nicht das gemeinsame
   Signal.
2. **Open-Loop sauber:** der unabhängige open-loop-Trägerlinien-Ton der
   deckenden Tage bricht in den laut-markierten Sub-Phasen nicht ein. Dieser
   Zeuge deckt nur **Mode-1-Tage** und ist **n-dünn**: Same-Day-Splits auf drei
   laut-Tagen (st14 1997-02-26 128,2 gegen 110,4, n 3/7; st14 1996-12-21 und
   st43 1996-12-19 je n 1/1) — in jedem liegt der Ton der laut-Phase in der
   Spanne der ruhigen Phase desselben Tages, kein systematischer Einbruch. Die
   Lautheit erscheint nicht als empfangene Spektral-Störung vor dem
   Closed-Loop-Abgriff; in den Modi 2/3 erreicht dieser Zeuge n = 0 (kein
   zeitgleicher open-loop-Record der betreffenden Tage, 0 geehrt).
3. **Sende-Flags n = 0:** über die Roh-Caches tragen 12/12 Transient-Ereignisse
   eine Empfangs-Signatur (Out-of-Lock/Good-bad/Slipped-Cycle), kein Ereignis
   die Sende-Predict-Signatur; die Lock-Cut-Marker (> 1000 Hz) koinzidieren mit
   dem Empfangs-Out-of-Lock-Flag.

Der Uplink-Pfad selbst ist als Achse gemessen leer: TRK-2-25 trägt die
Sende-Station nicht, ein Doppler-ODF/TRK-2-18 der Ära existiert in keiner
erreichten Quelle, die ODR-SFDU führt kein Uplink-Feld, der DSN-Pass-Plan ist
nicht öffentlich erreichbar. Ded-27 ist damit auf Galileo nicht als
Sender/Empfänger-Zerlegung ziehbar — aber über die drei Zeugen als
**Empfangskette, nicht Uplink** benannt. **Asymmetrie des Schlusses:** „nicht
Uplink" ist dort ein *gezogener* Schluss, wo der Zeuge n trägt — die
Simultanität (Mode 1, 16 eine-laut/32 beide-ruhig bei Uplink-Modi n = 0), der
Open-Loop-Ton (Mode-1-Tage, n-dünn) und die Send-Predict-Klasse (D4, n = 0,
durch Abwesenheit über 3 Anker-Pässe definiert). Für die lauten Zellen der
Modi 2/3 (106 von 207) trägt gegen den Uplink allein die Send-Predict-Klasse
n = 0; Simultanität und open-loop erreichen dort n = 0 (keine zeitgleichen
Records). Verbleibend (`pending`, kein Ersatz):
das konkrete **Bauteil/der Parameter der Empfangskette je Pass** (Loop-
Bandbreite, DGT-/Array-Konfiguration, Geräte-Zustand — die kodierten
Receiver-Felder tragen es nicht; die per-Pass-Geräte-Zuweisung ist auf dem
Resid-Record absent).

## 5. Klarstellung B — Episoden-Definition (dokumentiert, reproduzierbar)

- **Schwelle:** |resid| > 10 Hz über dem Zellen-Mittelwert (Floor-Sample =
  Stärke exakt −2560, |resid| ≤ 1000 Hz; die Elevation-Probe nennt die 7 lauten
  Tage, deren Lautheit im Band 1–10 Hz liegt, als eigene Klasse).
- **Mindest-Sample-Zahl:** keine eigene Mindestzahl — ein Einzel-Sample ist eine
  Episode der Spanne 0 s (eigene Klasse, τ = 0 auf dem 1-s-Gitter, nicht als
  Sekundenwert gefüllt).
- **Merge-Regel:** zusammenhängende über-Schwellen-Samples, Lücke zwischen
  aufeinanderfolgenden über-Schwellen-Samples ≤ 30 s, innerhalb eines Laufs
  (Lauf-Grenze = tdb-Lücke > 600 s). Episoden-Dauer = Spanne letztes − erstes
  Sample.
- **Klassen:** anhaltende Episode = Spanne ≥ 60 s (Minuten-Klasse);
  Kurz-Transiente = Spanne < 60 s (die < 2-s-Klasse wird separat gezählt).
- **Sensitivität (aus der Elevation-Probe, Schwelle/gap-Variation):** 10 Hz/30 s
  → Median der Mehr-Sample-Spannen 70 s (n 335); 1 Hz/60 s → 68 s (alle 158
  lauten Tage); 100 Hz/2 s → 21 s (137 Tage, 4 Episoden ≥ 10 min). Die
  Verteilungsform (große Kurz-Population + anhaltender Schwanz) und der
  anhaltende Schwanz bis Stunden sind über die drei Parametersätze stabil; die
  100-Hz-Schwelle blendet die kleineren anhaltenden Episoden aus und lässt nur
  die stärksten langen stehen.
- Reproduzierbarkeit: Daten `data/galileo_resid.bin` + die vier Roh-TDF-Caches
  `galileo_tdf_cache_*.TDF`; Tag-Zell-Definition (Mittags-Konvention), die
  Anker-n und -RMS reproduzieren die Referenz auf alle ausgegebenen Dezimalen.

## 6. Klarstellung C — Provenienz-Caveat (Juni-1996-g1-ATDF)

Die Juni-1996-g1-ATDF-Dateien (die Oppositionstage 1996-06-26..30) stammen aus
**derselben RSST-Bandquelle** wie die TDFs der Floor-Serie (das 2025er
Restaurierungs-Bundle `gll.rss.raw`, Autoren Buccino & Barbini, RSSN/332K —
externe Recherche `…_results_2.md`). Sie sind damit **als unabhängiger zweiter
Kanal nicht geeignet** (gleiche Provenienz, kein unabhängiger Zeuge); als
**Reproduktion** sind sie es — die Resid-/Anker-Zahlen der Juni-1996-Zellen
werden aus den TDF-Caches exakt reproduziert. Ein echter unabhängiger Zeuge
für Juni 1996 bleibt `pending` (Parkes/VLA als benannte externe Kandidaten;
ODR-Volumina ohne Juni-1996-Eintrag, 0 geehrt).

## 7. Bekannte Grenzen

- **Fehlende 2-Weg-Basis der Dreiweg-Kontingenz:** die Dreiweg-laut/ruhig-Zellen
  (Mode 3, 94 robuste Tage) haben keine zeitgleiche 2-Weg-Basis derselben
  Station am selben Tag als Bezugsgröße — je gewählter Bezugsgröße
  (Mode-3-Tages-Serie, Station-Tag-mergiert, geteilte Station-Tage) ergibt sich
  eine andere Zell-Menge und Signifikanz; die Dreiweg-Kontingenz ist damit
  beschreibend, nicht als Kontrast gegen 2-Weg messbar.
- **Frühe Anker ohne Open-Loop-Zeugen:** 1995-11-24 und 1995-12-04/05/06 tragen
  im GO-SUN-ODR-Archiv n = 0 ODR-Dateien (erste archivierte Datei 1995-12-08);
  die frühen lauten Anker (st14 25,85 Hz, st14-Dreiweg 52,9 Hz) sind belegt
  (closed-loop, TDF), aber ohne zeitgleichen open-loop-Zeugen.
- **Juni-1996-Opposition ohne Open-Loop:** die lauten Oppositionstage
  1996-06-26..30 liegen in keinem GO-J/GO-JS/GO-SUN-ODR-Volumen (0 geehrt) und
  in keinem weiteren ODR/Open-Loop-Bestand einer erreichten Quelle.
- Zwei cache-freie laute Anker-Pässe (M3 st63 1995-11-27, M1 st43 1996-11-04)
  sind am Flag nicht entscheidbar (resid-Alone-Maß: Lock-Nachbarschaft gemessen,
  Flag-Klasse `pending`, 0 geehrt).
- Die Mond-Vorbeiflug-Distanz (Ganymed an G1) ist offline nicht messbar (kein
  Jupitermond-Ephemeriden-Asset); der messbare Stellvertreter Jupiter-Distanz/
  Perijove trennt die lauten von den ruhigen Tagen des 1996er-Fensters nicht.

## 8. Präzedenzfall — Morabito et al., TDA Progress Report 42-113 (1993)

Der externe Präzedenzfall `docs/reference/19930020414.pdf` (Morabito et al.,
TDA Progress Report 42-113, 1993, Galileo-USO-Doppler-Stabilität) dokumentiert
dieselbe Phänomen-Klasse in der Cruise-Ära: **drei Pässe, 900330 (90-089)
DSS-43, 911019 (91-292) DSS-63 und 911130 (91-334) DSS-14, mit unerklärter
Doppler-Degradation, „possibly due to ground equipment problems"** — dieselben
drei 70-m-Stationen wie der Floor, dieselbe Klasse (per-Pass-Degradation des
Empfangs; die AGC korreliert dort mit der Spacecraft-Range) in einer früheren
Ära (Cruise/USO, 1/sec-Doppler, 10-Hz-Loop-Filter). Die Analyse lief im Rahmen
der Galileo-Radio-Wissenschaft und wurde laut Dokument fortgeführt („will
continue for the duration"). § V „Data Products" des Dokuments bestätigt den
**Passfolder-Inhalt** der DSN-Passfolder: „a pass folder from the DSN
containing copies of frequency predictions, operator logs and related
material" — die Passfolder-Achse (Abschnitt 9) ist damit als externer
Präzedenzfall belegt, ihr Inhalt deckt die gesuchte per-Pass-Achse ab
(Predictions/Logs/Material je Pass). **Table-1-Format** des Dokuments
(per-Pass-Tabelle Year/DOY/DSS/AGC/geschätzte Frequenz) ist das Format, das für
die Orbital-Phase als Nachfolge-/Anfrage-Analogon relevant ist (Abschnitt 9).

## 9. Pending (extern, nicht blockierend)

- **RSSN/332K (Buccino & Barbini, 2025er `gll.rss.raw`-Bundle):** Nachfrage bei
  den Autoren, ob auf denselben RSST-Bändern überlebt haben: Keyword-Files/
  Pass-SOE, **TRK-2-15** (Closed-Loop-Configuration-Monitor, der
  Konfigurations-Mitschrieb je Pass) und **MON-5-Systemmonitor-Bänder**
  (Antennen-/Rauschtemperatur-Status je Pass).
- **Passfolder-Archiv der JPL Radio Science Library, Building 264-325**
  (HANDBK6 §1.2/§5.1.5: Logsheets/Displays je Pass, „part of the permanent
  record of the track") — das physische Tagebuch des gesuchten per-Pass-
  Empfangs-Zustands; der Inhalt der Passfolder ist durch den Morabito-
  Präzedenzfall (Abschnitt 8) als „frequency predictions, operator logs and
  related material" extern belegt.
- **NTRS 19950010764 (SOE/Readiness-Reports der Ära)** — extern benannt als
  zu prüfende Quelle für die Sequence-of-Events-/Readiness-Achse.
- **Morabito-Fortsetzungs-Suche für die Orbital-Phase:** die Nachfolge-Berichte
  der TDA-Serie für 1995–1997 (Suche „Morabito Galileo USO orbital", TDA
  42-1xx), die die Doppler-Stabilitäts-/Ground-Equipment-Analyse des
  Präzedenzfalls (Abschnitt 8) auf die Floor-Ära 1995-11-23..1997-02-28
  fortführen.
- **Per-Pass-AGC/DSS-Tabelle der Orbital-Phase:** Anfrage an RSSN/JPL um ein
  Table-1-Analogon (Year/DOY/DSS/AGC/geschätzte Frequenz, Abschnitt 8) für die
  Floor-Ära — die AGC-Achse des vorliegenden Befunds ist auf dem Resid-Record
  ein einziger Klemmwert; eine per-Pass-AGC-Tabelle der Orbital-Phase wäre der
  externe Vergleich auf der Signalstärke-Achse.

Diese fünf Achsen sind extern und blockieren die vorliegende Konsolidierung
nicht; sie sind als offene Beschaffungs-/Nachfrage-Punkte registriert, nicht
als gemessene Werte.

## Register-Satz

*Die Galileo-Floor-Lautheit der 70-m-Ära ist in ihrer Deduktionskette
konsolidiert: 207 laut / 193 ruhig der 400 robusten (Mode, Station, Tag)-Zellen
als 105 Nachbar-Tages-Flips isolierter 1–3-Tage-Gipfel (133/137 Episoden),
nicht als Stufen, Kopplung, Takt, Ort oder Himmelskörper-Distanz — alle
Hypothesen der Tabelle mit ✗ und n gemessen (Betriebs-Ära `dae5062`,
1996er-Rest `4a33e3c`, Stufen/TE `505e5e7`, Sub-Tages-Achse `52866ed`, Ort/
Himmelskörper `99bac54`, Elevations-Decke `f6b33cd`, H1-Meilensteine `d0adf13`,
Receiver-Identität Zell/Pass `3c95c2a`/`94b4719`, Pioneer-Koinzidenz/-Scheiben
`18d5156`/`d81cf6e`, Simultanität/Intra-Pass `d4853fc`, beide-laut-Sample-Ebene
`7eba007`, Cycle-Slip-Sende-Klasse `a6f50f2`, Dreiweg-Tor-1 `5ae7743`,
ODF/ODR-Uplink-Pfad `b185e2a`/`70ad543`/`a48fb1c`, Pass-Plan `dffc7b4`/
`686a7d9`, Open-Loop-Zeuge `f887b88`/`ce7fd58`). Klarstellung A (neu gemessen,
`galileo_floor_sustained_lock_state`): die anhaltenden Episoden (Spanne ≥ 60 s)
der flag-bedeckten lauten Pässe sitzen 7/7 in Out-of-Lock-geflagten Strecken
(je ±2-s-Fenster 100 % AGC-Boden-Flag-Records lock-out, SLIPPED_CYCLE und
Lock-Marker eingebettet/am Rand; Gesamt-Spanne 2700 s), anhaltend-bei-Lock
n = 0, die eine in-Lock-Episode ist eine 31-s-Kurz-Transiente (Counter-Slip im gelockten Lauf) —
der positive Befund lautet: anhaltender per-Pass-Empfangs-Out-of-Lock-Zustand.
Ded-27 ist über drei Zeugen als Empfangskette benannt (Simultanität 16
eine-laut/32 beide-ruhig bei Uplink-Modi n = 0; Open-Loop-Ton ohne Einbruch;
Sende-Klasse n = 0); verbleibend pending das Bauteil/der Parameter der
Empfangskette je Pass. Die AGC-Kontrolle der Register-Fläche
(`galileo_floor_agc_loud_quiet`): 100 % der Mitglied-Samples beider Floor-
Klassen am Klemmwert −2560 (laut 207 Zellen / n 2 106 313, ruhig 193 / n 2 539
919, je Zelle Spanne 0), der mediane in-track-Anteil am Klemmwert 1,0000 in
beiden Klassen — die AGC unterscheidet laut nicht von ruhig; die starke
Population ist nicht systematisch ruhig (322 robuste stark-Zellen der Ära: 85
laut / 237 ruhig), die Lautheit bindet an keine AGC-Stufe. Präzedenzfall
Morabito et al., TDA 42-113 (1993): drei Cruise-Pässe 90-089 DSS-43, 91-292
DSS-63, 91-334 DSS-14 mit unerklärter Doppler-Degradation „possibly due to
ground equipment problems" — dieselbe Phänomen-Klasse; der Passfolder-Inhalt
(„frequency predictions, operator logs and related material", § V) ist extern
belegt; die AGC-Achse stützt die Boden-Gerät-/Loop-Klasse bei identischem
geklemmtem Signallevel (nicht Signal-Schwelle). Episoden-Definition (10 Hz, gap
30 s, anhaltend ≥ 60 s, Einzel-Samples eigene 0-s-Klasse; Median stabil 70/68 s
bei 1–10 Hz) und der Provenienz-Caveat (Juni-1996-g1-ATDF = dieselbe
RSST-Bandquelle, als unabhängiger Kanal ungeeignet, als Reproduktion geeignet)
sind dokumentiert. Das Telemetrie-Arraying/Combiner-Pfad (DGT/FSR/Parkes,
Zeile F1) ist ausgeschlossen: strukturell (der TRK-2-25-Doppler durchläuft die
per-Station-BVR/MDA-Radiometrie-Kette, nie den Combiner), zeitlich (die größten
Anker 1995-11-24/1995-12-05 liegen vor DGT- und Wide-Area-Arraying-Beginn,
1996-06-26 nach dem DGT-Start 23.05.1996 und vor dem Arraying, ohne H1-Stufe an
23.05.96) und in-Ära (beide-laut-Zellen der Arraying-Ära sample-getrennt). Die
ruhige Resid-Basis trägt keinen säkularen Drift (Zeile F2): flach in 7/9 Serien,
nur Mode-1-Senkung ~0,4 Hz/27 Mon als saisonale Lage-Änderung, Rausch-Decke
flach, Resid-Offset klein-positiv, 3 isolierte Versatz-Tage — kein
Anomalie-Drift, Boden-/Reduktions-Eigenschaft. Die ruhige Basis trägt genau
EINEN „Ruck" (Zeile F3): einen abrupter, anhaltender, Mode-1-nur-Niveau-Sprung
am 1995-11-30/12-01 (−0,8 Hz, st14/43/63 simultan, p_perm 0,003–0,0095), der
das +0,75-Hz-Öffnungs-Plateau auf ≈0 senkt (der Sprung beseitigt einen Versatz,
das Plateau war der transiente Öffnungs-Versatz der ersten Messwoche — Ursache
`pending`, Anfangs-Predict-Fehler oder vor-Ära-Zustand); M2/M3 am Übergang ohne
Ruck-Klasse-Abfall (teils echte Gegen-Daten, teils Lücke/laut benannt);
datengetrieben, drei Stationen, nicht an der Daten-Kante; der Meilenstein
23.05.1996 (DGT) liegt in der bodenleeren Spanne Feb–Mai 96 (0 Floor-Samples,
0 geehrt) — ein Sprung ist dort nicht beobachtbar; der Sitz (Borduhr-Event vs
sub-Hz-Modell-/Predict-Korrektur) bleibt
nach öffentlichem Material `pending` (externe Recherche in
`docs/reference/recherche-extern-galileo-ruck-borduhr-modell.md`): der relative
Schritt 0,8 Hz/2,295 GHz ≈ 3,5e-10 diskriminiert die zwei Kandidaten nicht,
zeitlich koinzident (kausal offen) sind die Bord-Kommandos JOE-A/B (30.11./1.12.,
Probe-Oszillatoren 27.11.), die Einweg-Reduktionen über die Grenze sind
methodisch blind gegen einen konstanten Sendefrequenz-Versatz. Bewusste
Register-Option (kein Blocker): FOIA-Anfrage an JPL/DSN nach Stations-Logs und
Predict-Updates um den 30.11.1995.
Extern-pending ohne Blockade: RSSN/332K-TRK-2-15/MON-5-
Bänder, Passfolder-Archiv JPL 264-325, NTRS 19950010764,
Morabito-Fortsetzungs-Suche 1995–1997 (TDA 42-1xx), per-Pass-AGC/DSS-Tabelle
der Orbital-Phase (RSSN/JPL-Anfrage).*

## Status

`draft` (Konsolidierungs-Blatt für die Haupt-Session und den Rat). Neue Probe
`tools/measure/src/bin/galileo_floor_sustained_lock_state.rs` (Klarstellung A,
`cargo check -p omegaflow-measure --bin galileo_floor_sustained_lock_state`,
RUSTFLAGS `-D warnings`, 0/0), Report
`/tmp/opencode/galileo_floor_sustained_lock_state_report.txt`. Blatt und Probe
in einem Commit (pfad-beschränkt); `phi/`, `docs/handover/handover-thematisch-mechanische-reste.md` und fremde Dateien
nicht angefasst. Die Deduktionskette trägt je ✗ eine Mess-ID (Probe) und den
Mess-Commit. AGC-Kontrolle additiv neu (`galileo_floor_agc_loud_quiet`, `cargo check` 0/0, Report `/tmp/opencode/galileo_floor_agc_loud_quiet_report.txt`), Morabito-Präzedenzfall (Abschnitt 8) und die zwei neuen extern-pending-Achsen (Abschnitt 9) in diesem Blatt-Stand.
