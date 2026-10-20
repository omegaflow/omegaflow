<!--
  title: Signal-cone audit — Positivkontrolle im selben Lauf (2026-10-19)
  class: survey
  date: 2026-10-19
  sha256: 26192cd26dc304b5107d5babcf8c43914ffa29cf2df913e56c7d0f6a564eebd1
  status: live
  see-also: docs/paper/signal-cone-audit-sheet.md docs/surveys/survey-2026-10-19-paper-landscape.md
-->
# Signal-cone audit — Positivkontrolle im selben Lauf

Token: `xfer:future.ses_ec618129fffeLFawr4V3pQKpOl.3~420fe9cd`
(Lücke `signal-cone`, Linie river).

## Die Frage

Die Leere des `signal-cone-audit` ist erst ein Nachweis, wenn dieselbe Maschine
einen bekannten Pfeil mit gemessenem τ und gemessenem d/c in **einem** Lauf
durch den Kegel-Test bringt. Zwei getrennte Läufe erfüllen das Kriterium nicht.
Der frühere Lauf (n ≈ 3824) fand keinen fam-Pfeil; die Audit-Tabelle blieb leer,
und die Leere war deshalb nicht als Nachweis lesbar.

## Der Lauf

- Werkzeug: `signal_cone_audit_probe` (release), Lauf `signal-cone-audit`.
- Fenster: 730 Tage des gemeinsamen Fensters, **n = 730 Tagzellen**.
- Maschine: 90 gerichtete Paare × 8 Lags (0–7 d), 10 phasen-randomisierte
  Surrogate pro Zelle, fam = Maximum aller Surrogate des Laufs,
  paarweiser Schwellwert = mean + 2σ der Surrogate.
- **fam = 2,8826e-1**, Paar-Schwelle der Positivkontrolle (lag 2 d)
  = **2,1053e-1**.
- Kommando (lokal, Assets aus dem CDN `omegaflow/sources`):

```sh
cargo build -p omegaflow-measure --release --bin signal_cone_audit_probe
./target/release/signal_cone_audit_probe --days 730 \
  --goes-bin /tmp/opencode/sac/goes_xrs.bin \
  --f107-bin /tmp/opencode/sac/f107_penticton.bin \
  --omni2-bin /tmp/opencode/sac/omni2_serie.bin \
  --euvs-bin /tmp/opencode/sac/goes_euvs.bin \
  --wind-waves-bin /tmp/opencode/sac/wind_waves.bin \
  --sun-eph /tmp/opencode/sac/ephemeris_sun.bin \
  --earth-eph /tmp/opencode/sac/ephemeris_earth.bin \
  --wind-orbit /tmp/opencode/sac/wind_orbit.bin
```

- Rohlog: `/tmp/opencode/sac/signal_cone_audit_730d.log`,
  sha256 `ab8419c78916e2810f690e50ec5a5beb49ac23acce81062709c1622343fd2553`
  (275 Zeilen, exit 0).
- Nachlauf 2026-10-21 (release, dieselben Assets): dieselben Werte Zeile für Zeile,
  inklusive `fam = 2,8826e-1`, `τ = 2 d`, `d/c = 4,893e2 s` und
  `Positivkontrolle: BESTANDEN`; Log-SHA wie oben.
- Eingaben (sha256, CDN-Assets): `goes_xrs.bin` `2a2ed6ab…` ·
  `f107_penticton.bin` `55e2ad1d…` · `omni2_serie.bin` `6dcbed64…` ·
  `goes_euvs.bin` `45b9c0ae…` · `wind_waves.bin` `fe907190…` ·
  `wind_orbit.bin` `e30025ea…` · `ephemeris_sun.bin` `75010b82…` ·
  `ephemeris_earth.bin` `4c708301…`.

## Die Positivkontrolle

Gepflanzte Kopplung im **selben Prozess**: ein AR(1)-Treiber am Sonnenanker
(AR 0,0, Rauschen 1,0) und seine um **2 d** verzögerte Kopie am Wind-Anker
(AR 0,7, Kopplung 0,9, Zielrauschen 0,3), Seed `0x50524F4245DB1E01`.
Die Kopplung ist bekannt; τ wird von derselben TE-/Lag-Maschine zurückgemessen,
d und d/c kommen aus den Ephemeriden desselben Laufs (t_mid). Die Kontrolle
läuft durch dieselben Lags, dieselbe Surrogat-Regel und dieselbe
`signal_reach`-Rechnung wie die Audit-Tabelle.

| Größe | Wert |
|---|---|
| τ gepflanzt | 2 d |
| **τ gemessen** | **2 d = 1,728e5 s** |
| TE (Treiber → Ziel, lag 2 d) | 5,4413e-1 |
| Paar-Schwelle (mean + 2σ) | 2,1053e-1 |
| fam (ganzer Lauf) | 2,8826e-1 |
| Pfeil | ja — TE > fam |
| **d(sun↔wind)** | **1,467e11 m = 0,9806 AU** |
| **d/c** | **4,893e2 s = 8,155 min** |
| reach(τ) = c·τ | 5,180e13 m |
| **Kegel-Urteil** | **Pfeil innerhalb des Kegels** (reach ≥ d, τ ≥ d/c) |
| PE-Gate | no statement (n = 730 < 8 × 256 = 2048) |

Der Lauf druckt wörtlich:

```
driver (sun) → target (wind) | planted τ 2 d | measured τ 2 d = 1.728e5 s | TE 5.4413e-1 | pair thr 2.1053e-1 | fam 2.8826e-1 | PFEIL
d(sun↔wind) = 1.467e11 m | d/c = 4.893e2 s = 8.155 min | reach(τ) = 5.180e13 m | τ ≥ d/c: ja | PE gate: no statement | Positivkontrolle: BESTANDEN (Pfeil within the cone)
```

**Positivkontrolle: BESTANDEN.** τ und d/c stammen aus demselben Lauf wie der
Kegel-Test; der Pfeil liegt innerhalb des Kegels.

## Der Rest desselben Laufs

- Die reale Matrix trägt genau einen fam-Pfeil: `Vwind → Density`, lag 0 d
  (TE 2,9922e-1 > fam 2,8826e-1). Beide Anker sind „at sun“, also d = 0.
- Die Audit-Tabelle bleibt leer; das Verdikt sagt „1 fam arrows, all hold the
  cone“. Das ist in diesem Lauf **kein** Advektiv-Kegeltest, siehe Riss 3.
- Kein em-Pfeil in den realen Paaren dieses 730-Tage-Fensters.

## Offene Risse (ehrlich)

1. **Die Kontrolle ist synthetisch.** Sie validiert Detektor, Lag-Rückgewinnung
   und Kegel-Arithmetik der Maschine, nicht einen physikalischen Kanal. Sie
   ersetzt nicht den Positivkanal mit realem τ < c, den die Paper-Landschaft
   als Datenlücke nennt.
2. **Tagesauflösung.** Bei d = 0,98 AU ist d/c = 8,2 min ≪ 1 d. Jeder gemessene
   Lag ≥ 1 d besteht den Kegel nahezu zwangsläufig; die Kontrolle diskriminiert
   keine Sub-Tag-Verletzungen. Ein τ = 0 (gleiche Tageszelle) ergäbe reach = 0
   und würde selbst einen echten kausalen Tag-Pfeil als „VIOLATED“ markieren.
3. **Kraft-IDs.** `FORCE_ADVECTIVE = 7` löst über `descriptor_for_channel_ref`
   auf `FIXED_CHANNELS[7] = "thermal"` auf, `FORCE_DIFFUSION = 6` auf
   `"electric"` (advective ist 4, diffusion ist 3). Für advective Paare liefert
   `signal_reach` deshalb `None` und die Audit-Zeile wird still übersprungen;
   diffusion würde mit c statt √(2·D·τ) gerechnet. Die em-Kontrolle
   (`FORCE_EM = 0`) ist davon nicht betroffen — ihr Kegel ist c.
4. **PE-Gate ohne Aussage.** n = 730 reicht nicht für acht 256-Zellen-Segmente
   (n ≥ 2048). Die Richtungs-Kaveat der Kontrolle bleibt ungeprüft.
5. **Fenster.** 730 d statt n ≈ 3824 des Papierlaufs. Dieser Lauf ist eine
   Maschinen-Kontrolle, kein Ersatz für den vollen Audit.
6. **EUVS-URL.** Der im Probe-Code hart kodierte CDN-Pfad
   `ssd.jpl.nasa.gov/goes_euvs.bin` antwortet 404; das Asset liegt unter
   `ncei.noaa.gov/goes_euvs.bin` (hier per `--euvs-bin` übergeben). Der
   Workflow lädt den ncei-Tag, der Code-Fallback bleibt aber falsch.

## Was der Lauf belegt

Für die Maschine ist die Lücke geschlossen: Ein Paar mit gemessenem τ und
gemessenem d/c besteht im selben `signal-cone-audit`-Lauf den Kegel-Test
(Pfeil innerhalb des Kegels). Die Leere der realen Paare ist damit als
Messergebnis der validierten Maschine lesbar — vorbehaltlich der genannten
Risse, insbesondere der Tagesauflösung und der Kraft-IDs.
