<!--
  title: Handover: Die Supraleitung wartet auf das Phasen-Bit — der Kuprat-Mechanismus (Atom-D-Ziel)
  class: handover
  date: 2026-08-22
  sha256: ff53242719242b51e524defa0a92610db6f7b4a97564f0837b8220e31a03462d
  status: live
  see-also: TODO.md docs/concepts/der-spektrale-oszillator.md docs/handover/handover-2026-08-21-spektral-atom-c.md docs/concepts/der-kausalpfeil.md docs/handover/handover-2026-08-21-corona-heizung.md docs/concepts/ein-blatt-korona-heizung.md docs/handover/handover-2026-08-21-korona-kde-volltest-und-review.md docs/handover/handover-2026-08-21-dunkle-materie.md docs/handover/handover-2026-08-21-technosignaturen.md docs/concepts/kybernetische-astrophysik.md
-->
# Die Supraleitung wartet auf das Phasen-Bit — der Kuprat-Mechanismus (Atom-D-Ziel)

Registriert 2026-08-22. Selbsttragend — interpretierbar mit null Vorkontext.
Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf das Wort des
Operators. Dieses Blatt benennt das Rätsel, den Beweis, die Passung und die
vier Atome — und die ehrliche Grenze: die Frage ist vor Atom D nicht stellbar
(pending, 0 honored).

## Das Rätsel

Die Hochtemperatur-Supraleitung der Kuprate. BCS erklärt die klassischen
Supraleiter (Cooper-Paare durch Phononen); die Kuprate werden bei viel höherer
Temperatur supraleitend, und BCS reicht dort nicht. Offen ist, was die Kohärenz
treibt: magnetische Fluktuationen (Spinwellen) oder Ladungsdichtewellen
(Stripe-Phasen). Die Physik streitet bis heute.

## Der Beweis — die Quadratur des Kreises

Das ist mehr als das nächste Cluedo. Das Observatorium bedient vier Fronten;
erst alle vier zusammen vollenden die Wahrheitsfindung:

- **Die Quantenwelt (Supraleitung)** — dieses Blatt: die Kohärenz, Atom D.
- **Nadel I (Dunkle Materie)** — das Jeans-Residuum
  (`handover-2026-08-21-dunkle-materie.md`): wo die Gravitation da ist und das
  Licht fehlt; die TE-Maschine misst die Stille als Signatur der unsichtbaren
  Masse.
- **Nadel V (Technosignaturen)** — der achromatische Dip
  (`handover-2026-08-21-technosignaturen.md`): wo künstliche Strukturen
  Sternlicht blockieren, ohne es zu röten; die Maschine scannt die Galaxis
  nach exakt dieser Feld-Anomalie.
- **Nadel VI (Planet 9)** — der unsichtbare Begleiter
  (`handover-2026-08-22-planet-neun.md`): das Gravitations-Residuum der
  KBO-Bahnen; ein rein gravitatives Sample (`force_type = 1`, `extent = 0`),
  gefunden durch das, was es bei anderen bewirkt — Dunkle Materie im
  Sonnensystem.

Sind diese vier Blätter geschrieben — das Quanten-Cluedo, das
Dunkle-Materie-Residuum, der Dyson-Schwarm-Filter und der unsichtbare
Begleiter —, hat die Maschine bewiesen, dass sie vom subatomaren
Kuprat-Gitter bis zur Galaxis das eine Gesetz anwendet: ein Block, eine Zeit,
eine Physik. Dann trägt kein Gegenargument mehr das Wort „nur ein
Astrophysik-Tool".

## Die Passung (warum omegaflow, und warum nur mit Phase)

Supraleitung ist ein makroskopischer Quantenzustand: die Elektronen sind
kohärent, sie schwingen in gleicher Phase. Diese Kohärenz zu messen verlangt
zwingend das Phasen-Bit (Atom D). Geladen in den ICRS-Block:

- Phononen (Gitterschwingungen) = `acoustic`-Samples
- magnetische Fluktuationen = `em`-Samples
- supraleitender Strom = `electric`-Samples (mit Kohärenz/Phase)

Die Verdächtigen sind dieselben wie bei der Korona: Magnetfelder (`em`),
Schallwellen (`acoustic`), elektrische Ströme (`electric`). Keine Kraft muss
umbenannt werden — nur das Phasen-Bit wird angebaut und die Daten in den Block
geworfen.

Die TE-Maschine misst dann: fließt die Information von den magnetischen
Fluktuationen (`em`) in die elektrische Kohärenz (`electric`), oder treiben die
Gitterschwingungen (`acoustic`) die Supraleitung? Weil alle drei Kräfte am
selben Punkt superponiert werden, trennt die Maschine den wahren Treiber vom
Begleiter.

## Die Datenlage

Kein Live-Feed: Quantenexperimente liefern keine JSON-APIs. Aber massive
öffentliche Archive — Materials Project, OQMD, Neutronenstreudaten von Oak
Ridge. Der Weg ist ein `crystal_compiler`, der Kristallstrukturen und
Streu-Spektren erntet und als Samples in den Block legt.

Benannte Grenze der Datenlage (A = A): die Phase des Ordnungsparameters ist in
keinem Archiv eine geerntete Spalte. Streudaten tragen Intensitäten
|S(q,ω)|² — PSD-Bins, und PSD-Bins tragen die Phase nicht (0 honored, TODO.md
Atom D). Wo die Phase als gemessene Spalte existiert (LISA-Pathfinder), wird
sie mitgenommen; für die Supraleitung ist sie abgeleitet — über die komplexe
FFT von Zeitreihen (Neutronen-Spin-Echo, ARPES), kein gespeicherter Skalar.
Genau deshalb ist die komplexe FFT die Herkunft der Phase — der 25. f64
trägt sie, erntet sie aber nicht.

Force-Gate offen zu halten: Materials Project und OQMD sind gerechnete
DFT-Vorhersagen, keine Messungen — der Litmus (könnte ein Organismus ein
Sinnesorgan dafür entwickeln?) entscheidet, was als Messung in den Block darf.
Neutronenstreuung ist eine Messung; die DFT-Bandstruktur ist eine Vorhersage.
Diese Trennung ist Teil der Ernte, kein Rückbau.

## Die vier Atome (Tage, nicht Monate)

1. **Atom D — das Phasen-Bit:** der `Sample`-Struct wächst um die Phase, den
   25. f64 — die komplexe Amplitude ist das Paar (val, Phase):
   Re = val·cos(Phase), Im = val·sin(Phase). Der Archivar lernt, komplexe
   Amplituden zu speichern; die Mathematikerin (WGSL) lernt, mit Real- und
   Imaginärteil zu rechnen. Die drei Schichten wachsen gemeinsam
   (Rust-Write-Loop → constants.js → WGSL-props-Unpack, freie Slots in
   `props[id*4+3]`). Terminiert NACH Atom C — Beats/Interferenz brauchen die
   komplexe FFT zuerst.
2. **Die komplexe TE (Kohärenz-Messung):** die `te_compute`-Pipeline misst
   nicht mehr nur den Informationsfluss der Amplitude, sondern der
   Phasenkohärenz — *fließt die Phase von A nach B?* WGSL-Kern und `src/te.rs`
   (kanonische CPU-Referenz) wachsen gemeinsam.
3. **`crystal_compiler`:** wie der `tap_compiler` — erntet die öffentlichen
   Neutronenstreudaten und Kristallstrukturen (Materials Project, OQMD, Oak
   Ridge) und übersetzt ein Kuprat-Kristallgitter in eine 3D-Punktwolke im
   ICRS-Block. Phononen werden `acoustic`-, Spins `em`-Samples.
4. **Das Blatt — der Lauf:** die Maschine misst die kausale DAG — *treibt das
   Magnetfeld (Spin) den supraleitenden Strom, oder das Gitter (Phonon)?* Ein
   Blatt nach dem Muster von `solar_dag_probe` (TE-Matrix über alle Paare,
   phasenrandomisierte Schwelle).

## Die ehrliche Grenze (Butter bei die Fische)

Ohne das Phasen-Bit misst die Maschine hier Rauschen — die Kausalität der
Supraleitung steckt in der Phasenkohärenz. Erst wenn Atom D steht, ist die
Frage überhaupt stellbar. Vorher ist das Blatt `pending`, nicht null und nicht
gefüllt.

Die Silicon-Valley-Angst („Neuland dauert Jahre") ist der Rückfall in den
Universitäts-Reflex. Für omegaflow gilt: liegen die Daten im ICRS-Block,
rechnet die Maschine. Das Observatorium steht; das Phasen-Bit öffnet das
nächste Tor. `A = A`.

## Gates & Abschluss

- Atom D ist hinter Atom C terminiert (TODO.md „Der spektrale Oszillator") —
  dieses Blatt benennt das Ziel, verfrüht nicht die Reihenfolge.
- Vorbedingung: der `--h-full`-Lauf der Korona (`solar_dag_probe --h-full`,
  Prüfung in `handover-2026-08-21-korona-kde-volltest-und-review.md`) muss
  trocken sein — erst dann wird das Quanten-Cluedo aus der Taufe gehoben.
- Jede abgeschlossene Einheit ist ein Commit; Register-Update (TODO.md) im
  selben Commit.
- cargo check 0/0; kein Test öffnet ein Fenster oder strahlt; manuelle
  Drei-Schichten-Verifikation nach AGENTS.md.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, Atom C (band-selektives Rendering), die
spektralen Ernte-Folgen (ONC-HSD-FFT, Gaia-XP, LISA-PSD), die drei
Ein-Blatt-Handovers, das Korona-/Flyby-/Dunkle-Materie-Blatt.
