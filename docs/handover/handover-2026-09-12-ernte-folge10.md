<!--
  title: Handover — Ernte-Folge 10 (Stand 2026-09-12)
  session: Ernte-Folge 10
  class: handover
  date: 2026-09-12
  sha256: 0438cb20374547e076be6edc5019a7bb54841eaffe8bc5c010a9f515922d2ad9
  status: live
-->
# Handover — Ernte-Folge 10 (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

Die nächste Session schließt alle offenen Punkte dieser Linie ab — nicht
weiterschieben. Braucht sie Unterstützung, nimmt sie das Gremium (Rat) oder die
Taucher (Sub-Agenten, eigener Kontext). Fragen, die einen Webzugriff brauchen,
gehen über den opencode-Browser an den Operator.

## CDN-Manifestation (Duty)

- harps_rvcat.json — Wächter: der Redispatch 34717041472 war void — er lief auf
  cd4e963a (noch `--csv`, HTTP 400; der Void-Guard maskierte es als „success").
  Tiefer gemessen: auch `--votable` hätte still bei MAXREC=20000 gekappt
  (`OVERFLOW`, nur 20000 von 289843 Zeilen). Behoben: `tap_query_votable` trägt
  jetzt `MAXREC=<limit>`; verifiziert (25000 Zeilen fließen, alle Tests grün).
  `--votable` steht bereits auf main. Fix (tap_compiler.rs) committet + gepusht
  (a764f2d). Redispatch 34718761177 (21:01Z) — der Wächter misst das Asset im
  Release ssd.jpl.nasa.gov.
- igets.bin — Wächter: Lauf 34716901099 trägt 3 void Stationen (Mizusawa,
  Brasimone, Esashi) → merge wird geskippt. Gemessen: `parse_ggp` las
  `N Latitude  (deg)` (Doppel-Leerzeichen) nicht und ließ `gravity(mV)` ohne
  Kalibrierung als scale 1.0 durch (falscher Wert). Behoben: normierte
  Schlüssel, exakte Spalten-Zuordnung (g_fil, gravity(nm/s**2), gravity(V),
  gravity(mV)), mV-Kalibrierung `Grav.Cal (nm.S-2/mV)`, unbekannte
  Spalte/fehlende Kalibrierung → benannter Skip; verifiziert (Mizusawa → 720
  Records, alle Tests grün). Fix (igets_compiler.rs) committet + gepusht (a764f2d).
  Redispatch 34718762450 (21:01Z) — der Wächter misst das Asset im Release
  igetsftp.gfz.de.

## Ernte

- Hi-net — `HINET_PASS` steht in `.secrets.local`, Login verifiziert (Ablauf
  2027-03-31). Harvest-Verdrahtung gebaut: Compiler-Fetch auf den echten Fluss
  (auth GET→POST, Channel-Tabelle via `dlDialogue.php`, cont-Suche→Request→Poll
  →Download→Unzip), Format `hinet` im Archivar, `hinet-cdn.yml`,
  sources.φ-Registrierung. Gemessen: `cont_download.php` schneidet jede Antwort
  bei ~36 s (~11 MB) — ein 1-min-Gesamtnetz-Fenster (~15 MB) vollständet nie
  (kein Range/Resume). Offen: Fenster auf eine Station/Teilmenge verkleinern
  oder batchen, damit der Download unter die ~36-s-Grenze fällt.

## Noch offene Pendings

- H₀: Gaia-TAP-Crossmatch der 75 SH0ES-Cepheiden — die 75er-Tabelle trägt keine
  ID-Spalte, die per-source-Identität des Cepheiden-Ankers fehlt
  (`blatt-h0-linien-register.md`).
- de441 mars: Vor-Fix-Matrizen (Δ 6045,3 km) — kernel-flatten-Run 34348392827
  endete `failure` (`handover-thematisch-membran-sonde.md`).
- de441 earth/moon/sun: aus der einen Stimme gedriftet — Erdmitte 116 km von
  de440/de442, größte Finsternis 42,5 km / −90,5 s (2017) und +10,1 s (2024);
  de440/de442/epm2021 bleiben eine Stimme (4,8 km / +4,9 s). Re-Ernte
  (de_compiler) + `eclipse_shadow_probe` + Kalibrier-Gate neu laufen lassen
  (`eclipse-clock-worldlines.md`).

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == b054e9d (Refs gemessen). Der
  Baum trägt fremde Sessionsarbeit (uncommitted — unberührt). Eigene Hunks:
  tap_compiler.rs, igets_compiler.rs, dieses Handover. Gepusht: a764f2d (clean
  fast-forward). Redispatches: 34718761177 (eso-harps-rvcat-cdn), 34718762450
  (igets-cdn).
