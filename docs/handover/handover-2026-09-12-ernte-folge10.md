<!--
  title: Handover — Ernte-Folge 10 (Stand 2026-09-12)
  session: Ernte-Folge 10
  class: handover
  date: 2026-09-12
  sha256: 545fedb815c04effc7f3a348c30967bbbf634c2c3ffc333a39cad5141a45a9f0
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

- Hi-net — `HINET_PASS` weiter absent; der Operator nannte Browser-Registrierung,
  die Browser-Extension ist nicht verbunden — Operator.

## Noch offene Pendings

- H₀: Gaia-TAP-Crossmatch der 75 SH0ES-Cepheiden — die 75er-Tabelle trägt keine
  ID-Spalte, die per-source-Identität des Cepheiden-Ankers fehlt
  (`blatt-h0-linien-register.md`).
- de441 mars: Vor-Fix-Matrizen (Δ 6045,3 km) — kernel-flatten-Run 34348392827
  endete `failure` (`handover-thematisch-membran-sonde.md`).

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == b054e9d (Refs gemessen). Der
  Baum trägt fremde Sessionsarbeit (uncommitted — unberührt). Eigene Hunks:
  tap_compiler.rs, igets_compiler.rs, dieses Handover. Gepusht: a764f2d (clean
  fast-forward). Redispatches: 34718761177 (eso-harps-rvcat-cdn), 34718762450
  (igets-cdn).
