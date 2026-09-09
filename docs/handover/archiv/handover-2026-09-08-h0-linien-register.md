<!--
  title: Handover — H₀-Linien-Register: Wurzeln statt Zeugen (das nächste Atom: die eigene Leiter-H₀)
  class: handover
  date: 2026-09-08
  sha256: 55e9618e7061e97527c93529650a39a2a8960dd0f748be337f53dad84f0391c1
  status: archived
  see-also: docs/blatt/blatt-h0-linien-register.md docs/concepts/die-weberin.md docs/TODO.md docs/concepts/docs-naming.md
-->
# Handover — H₀-Linien-Register (das Atom der nächsten Sitzung)

Übergabe für die nächste Sitzung. Das Atom: die **eigene Leiter-H₀** — aus
Cepheiden-Perioden-Leuchtkraft + Pantheon+-SN-Ia selbst rechnen, damit die
Leiter-Wurzel des H₀-Linien-Registers von „teils gewogen" auf „ganz gewogen"
rückt.

## 0. Die abgebende Sitzung ist ein vollständig abgeschlossenes Atom

Das H₀-Linien-Register ist gebaut, gewogen und committet — **null offene
Punkte**. Commits: `dc89786` (Blatt + Probe), `38c1553` (Nachtrag I),
`366c8b6` (DCEP-Filter), `d45450b` (Nachtrag II), `706ab5a` (MNRAS-Druck
bestätigt). Diese Übergabe eröffnet das **nächste** Atom; nichts aus der
abgebenden Sitzung ist offen.

## 1. Was gebaut wurde

- **Das Blatt** `docs/blatt/blatt-h0-linien-register.md` (class: sheet):
  ~40 publizierte H₀-Messungen in 10 Wurzel-Familien + Rücken, jede Zeile mit
  Wurzel-Kette, Trennstufe, Route. Weißes Feld („gemeinsame Wurzel: absent" —
  kein Schiedsspruch möglich), Asymmetrie (Leiter teils gewogen / CMB zitiert),
  Trennstufen-Karte (früh/spät/keine), Verdikt („noch kein Schlichter").
- **Der Probe** `tools/measure/src/bin/cepheid_parallax_weigh.rs`: Gaia-TAP,
  `type_best_classification = 'DCEP'`, N = 1606, gewichtetes
  Parallaxen-Mittel 0.2619 ± 0.0004 mas.
- **Zwei Nachtrag-Wiegungen** (alles `[c]`): JWST-Ära (Riess 2024/2025,
  Freedman/CCHP 2025), DESI DR2, GWTC-5.0 (71.7), TDCOSMO 2025, TRGB-SBF,
  Pantos-88-Kompilation; CMB-Klassen-intern (ACT DR4/DR6, WMAP9), HST Key
  Project, Fundamentalebene (Said, MNRAS 539, 3627), baryonische Tully-Fisher.
- **Alle `pending` geschlossen:** BBN-Herkunft = Quasar-D/H (Cooke 2018),
  `vari_cepheid`-Enum, VizieR-ID = `absent` (Tabelle lebt im arXiv-Paket
  2012.08534 `bigtable_redux3.tex`), Freedman-Erratum kosmetisch, MNRAS-Druck
  bestätigt.

## 2. Das Atom — die eigene Leiter-H₀

Die Leiter-Wurzel end-to-end im Haus wiegen, nicht zitieren. Zutaten stehen
bereit:

- **Gaia-TAP-Leg**: `gea.esac.esa.int/tap-server/tap/sync` (sources.φ).
- **DCEP-Parallaxen** schon gewogen (N = 1606, s. Probe).
- **Die 75 Riess-Cepheiden mit Photometrie**: arXiv-Quellpaket **2012.08534**
  (`bigtable_redux3.tex`, F555W/F814W/F160W + π_EDR3).
- **Pantheon+** (SN-Ia-Standardisierung + Hubble-Fluss): Scolnic 2021 /
  Brout 2021, SH0ES-Datarelease auf GitHub (y/L/C-Kovarianzmatrizen).

Der Weg: eigene Cepheiden-PL-Anpassung im Parallaxenraum → eigene
SN-Ia-Kalibration → eigene H₀. Ergebnis: die Asymmetrie-Zeile des Blatts rückt
von „teils gewogen (Klasse)" auf „ganz gewogen (Leiter)". Das ist ein
Wochen-Atom, in Reichweite (Rat, 2026-09-08).

## 3. Die ehrliche Grenze — benannt, kein Pending

Die CMB-Seite bleibt zitiert: die Planck-Likelihood ist eine
Forschungsmaschine, keine Session. Die eigene Riss-Messung zwischen den
**eigenen** Linien (die-weberin §8) bleibt offen, bis die CMB-Linie in den
Bestand einzieht — das ist der gemessene Zustand, nicht ein Versäumnis. Auch
nach der eigenen Leiter-H₀ bleibt die Asymmetrie eine Zeile im Blatt: eine
Seite gewogen, eine zitiert.

## 4. Die Disziplin-Lektion (an die nächste Sitzung)

Ein externer Nachtrag (Chat zweier Fremd-Sessions) behauptete eine „Korrektur"
(H₀ = 76.6) mit fabrizierten Zitat-Markern (`【turn…】`). Die Wägung (direkter
Fetch von arXiv:2509.04348) zeigte: **75.4 ist richtig, die 76.6 kommt nicht
vor.** Regel, die trägt: **nicht das Modell ist der Schiedsrichter, die Quelle
ist es.** Ein „ich habe gefetcht" ohne Fetchen ist Fabrication — egal welches
Modell es schreibt. Dasselbe gilt für `【turn0search…】`-Marker: sie sind keine
Quelle.

## 5. Zeiger

- Blatt: `docs/blatt/blatt-h0-linien-register.md`
- Probe: `tools/measure/src/bin/cepheid_parallax_weigh.rs`
- TODO: `docs/TODO.md` (Weberin-Bau-Linie, Hubble-Illustration → zeigt aufs Blatt)
- Die Wägung läuft über die Kaskade: `docs/SOURCE_PORT.md` §Agenten-Rezept.
