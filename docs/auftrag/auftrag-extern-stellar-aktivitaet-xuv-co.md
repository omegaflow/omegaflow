<!--
  title: Auftrag (extern) — Stellare Aktivität/XUV und C/O für 30 Exoplaneten-Wirte
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: 8cbf117eccf9c7d0e75f9dd669cde2b6c9b5d0880eef41b91327b988618a53d1
  see-also: docs/paper/jwst-disequilibrium-survey.md
-->

# Rechercheauftrag (extern): Stellare Aktivität, XUV und C/O von 30 Exoplaneten-Wirten

Dieser Auftrag ist für eine externe Person oder ein externes Team gedacht. Er
ist selbsttragend — es ist kein Vorwissen über das omegaflow-System nötig. Das
Ziel ist reine Literatur-/Katalog-Recherche: keine Beobachtung, keine Analyse,
nur das Zusammenstellen **öffentlich publizierter** Messwerte.

## Warum (der eine Satz)

Wir haben in 30 Exoplaneten-Atmosphären chemische Signale gemessen (JWST-
Transmissionsspektren), die nicht im thermochemischen Gleichgewicht liegen
(z. B. Schwefeldioxid SO2, Kohlendioxid CO2). Um zu entscheiden, ob diese
Signale echte „Lebens-/Techno-Signaturen" sind oder nur normale **Stellar-
Aktivität** (UV/XUV-Strahlung des Wirtssterns, die Photochemie antreibt),
brauchen wir pro Wirtsstern Messwerte seiner Aktivität und Elementhäufigkeit.

## Die 30 Ziel-Sterne (Wirtssterne)

GJ 3090 · GJ 9827 · HAT-P-12 · HAT-P-14 · HAT-P-18 · HAT-P-26 · HAT-P-30 ·
HIP 67522 · K2-18 · L 98-59 · LP 791-18 · LTT 3780 · TOI-1130 · TOI-199 ·
TOI-270 · TOI-421 · TOI-5205 · TrES-4 · V1298 Tau · WASP-107 · WASP-121 ·
WASP-127 · WASP-15 · WASP-166 · WASP-17 · WASP-39 · WASP-43 · WASP-52 ·
WASP-94 A · WASP-96

(Die Sternnamen sind die gängigen Exoplaneten-Wirts-Bezeichnungen; SIMBAD
löst sie auf. Es sind fast alle helle bis mittelhelle FGK- und M-Zwerge.)

## Gesuchte Daten — pro Stern, so viele der folgenden wie auffindbar

1. **Chromosphärische Aktivität `log R'HK`** (dimensionslos, Mount-Wilson-Skala)
   oder ein anderer Aktivitätsindex (z. B. S-Index, H-alpha-Äquivalentbreite).
2. **Röntgen- bis EUV-Leuchtkraft / Fluss** (`L_X`, `L_X/L_bol` in der
   Röntgenbande 0,1–2,4 keV und/oder XUV 1–912 Å oder 6–1200 Å), aus
   Röntgen-/UV-Katalogen oder abgeleiteten XUV-Abschätzungen.
3. **Rotationsperiode** `P_rot` (Tage), wenn publiziert.
4. **Elementhäufigkeiten:** Eisen `[Fe/H]`, sowie **Kohlenstoff/Stickstoff/
   Sauerstoff** (`[C/H]`, `[N/H]`, `[O/H]` oder daraus das **C/O-Verhältnis**),
   wenn publiziert (hochauflösende Abundanz-Kataloge).

**Für jeden Wert:** Wert + Einheit + Quellenangabe (bibcode/DOI) + das
Instrument/Katalog (z. B. „XMM-Newton", „Brewer & Fischer 2016").

## Bitte zu prüfende Quellen (öffentlich; keine Vermutung, nur was real existiert)

- **Aktivität/XUV:** MUSCLES / Mega-MUSCLES (Röntgen-UV-Spektren von
  Exoplaneten-Wirten, Loyd et al.), XMM-Newton / ROSAT / Chandra-Kataloge
  (L_X), GALEX (NUV), die Mount-Wilson-HK-Projekt-Sammlung, SONG, TESS
  (Rotationsperioden), asterochromologie-Kataloge (Alter aus Rotation).
- **C/O und Abundanzen:** Brewer & Fischer 2016 (ApJS 225, 32) und 2018
  (ApJS 237, 38), die Hypatia-Katalog-Erweiterungen, SWEET-Cat, das Exoplanet
  Archive (NExScI, `pscomppars`), VizieR-Kataloge (A/J/…-Abundanz-Tabellen).
- **Auflösung der Sternnamen:** SIMBAD / VizieR-Crossmatch.

**Wichtig:** Bitte prüfen Sie jede Quelle wirklich an (URL/ADS-Eintrag offen),
nicht annehmen. Ein Stern, für den Sie in einer geprüften Quelle keinen Wert
finden, wird als **„in <Quelle> nicht gefunden"** eingetragen — niemals
erfunden oder geschätzt.

## Lieferung

Eine Tabelle (CSV oder XLSX) mit den Spalten:
`Stern | log R'HK | Quelle_RHK | L_X [erg/s] | Quelle_LX | XUV | Quelle_XUV |
P_rot [d] | Quelle_Prot | [Fe/H] | [C/H] | [O/H] | C/O | Quelle_Abund |
Bemerkung`

Für jeden der 30 Sterne eine Zeile; jede Zelle, die Sie nicht belegen können,
bleibt leer und in „Bemerkung" steht, welche Quelle Sie geprüft und leer
gefunden haben. Dazu ein kurzer Absatz: welche der 30 Sterne überhaupt in
XUV/Aktivitäts-Katalogen vorkommen (Vollständigkeit).

## Regel (0 honored)

Ein leerer Wert, den Sie in einer geprüften Quelle nicht finden, ist ein
ehrliches Ergebnis — **nicht** durch Schätzung/Extrapolation auffüllen.
Die Recherche ist die Grenze: was nicht publiziert ist, bleibt „nicht
publiziert gefunden".
