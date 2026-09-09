<!--
  title: Befund — Feldstandard Seismik: die gebaute Seismik ist kein Neubau — jede Komponente trägt ein Feld-Äquivalent, neu ist nur die Einbettung (ICRS, t_ref, Signal-Kegel)
  class: befund
  date: 2026-09-09
  sha256: beeb98e41a03a2e1d74e5f792b508944299be56882fb168ae165f83f46c17c64
  status: done
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-seismische-ortung.md docs/befund/befund-2026-09-09-seismische-ortung-ak135.md docs/befund/befund-2026-09-09-seismische-ortung-tiefe.md docs/befund/befund-2026-09-09-tohoku-pegsel.md docs/befund/befund-2026-09-09-tohoku-vorhersage.md docs/befund/befund-2026-09-09-tohoku-gsn-geblockt.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md
-->

# Befund: der Feldstandard der Seismik

## Frage & Bindung

Operator-Verdacht (2026-09-09): „ihr erfindet die Seismik gerade komplett neu."
Gemessen per lokaler und externer Recherche (vier Taucher, 2026-09-09): jede
gebaute Komponente hat ein Feld-Äquivalent; neu ist nur die Einbettung.

## Die Gegenüberstellung

| Gebaut | Feld-Standard (Zitat) | Zustand |
|---|---|---|
| ak135 + τ(p)-Tracer | Buland & Chapman 1983; Modell Kennett/Engdahl/Buland 1995; Referenz TauP (Crotwell et al. 1999) | identisch, <0,15 s gegen TauP |
| 3D-Gitter-Ortung | NonLinLoc (Lomax et al. 2000); linearisiert = Geiger 1910 | Äquivalent; Lücke: kein Posterior/Fehler-Ellipsoid |
| STA/LTA-Picker | Allen 1978 | Äquivalent |
| Σ Segment/√(g·d) | Laufzeitkarten der Warnzentren (Murty 1977; TTT-Software, Wessel) | identisches Verfahren |
| M9-Ersteinsatz-Picker | Feld nutzt W-Phase-CMT (Kanamori & Rivera 2008) / GCMT | Picker ist das falsche Werkzeug für M9 — benannt |

## Was wirklich eigen ist

- Die Einbettung: Weltlinien über `body_fixed_to_icrs` zur gemeinsamen t_ref
  (die Rotation kürzt die Sehne, das Medium rotiert mit); der Signal-Kegel
  (v_or_d·age). Das Physikalische ist Feldstandard — die Befunde trugen nur die
  Herkunft nicht.

## Prüffall-Pin

Test-Event der Positivkontrolle: USGS `us6000tkt2` (2026-08-14T21:58:21.505Z,
mww 7.8, reviewed, „64 km NNW of Ende, Indonesia"), Katalog −8,3514/121,3478/
10 km. Die Rayleigh-Probe (`rayleigh_dispersion_probe.rs`) trägt die ID schon;
die Ortungs-Befunde trugen nur die Koordinaten.

## Datenwege (gemessen 2026-09-09)

- EarthScope-FDSN liefert 2011-Tōhoku-Wellenformen (IRIS-legacy 410).
- Eikonal-Gitter: ETOPO 2022 60s bedrock (469 MB, anonym, NCEI THREDDS) als
  erste Stufe; GEBCO 2023 15s (7 GB, CEDA) als CDN-Manifestations-Stufe.
- Hi-net (NIED): Registrierung `hinetwww11.bosai.go.jp/nied/registration/`,
  2011 archiviert, WIN32-Format.
- ISC-FDSNWS offen (QuakeML; Parser-Gap in `phi/blocked_sources.φ`); GCMT-NDK
  offen (NDK-Parser-Gap ebenda).

## Folge (Register)

- Die sechs Seismik-Befunde tragen jetzt eine Feld-Herkunft-Zeile.
- Offen (unverändert): Eikonal über das volle Gitter, pP/sP für scharfe Tiefe,
  W-Phase-Mw für M9, Stromboli.
