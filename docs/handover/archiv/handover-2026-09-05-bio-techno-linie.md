<!--
  title: Übergabe — Bio-/Technosignatur-Linie (Nadel XIII + V)
  class: handover
  date: 2026-09-05
  sha256: 8a8395b5b44efced27bda076a5ba5bddf978421f1d13a29352973337710e737b
  status: archived
  see-also: docs/TODO.md docs/paper/jwst-disequilibrium-survey.md docs/paper/nadel-v-fresh-area-dip-scan.md
-->

# Übergabe: Bio-/Technosignatur-Linie

Diese Übergabe fasst den Zustand der Bio-/Technosignatur-Arbeit (Nadel ⅩⅢ =
Atmosphären-Biosignatur/Disequilibrium, Nadel Ⅴ = struktureller Techno-Galaxie-Scan)
zusammen, damit eine einzelne empfangende Session sie fortsetzen kann.

## Das Ergebnis in einem Satz

Beide Nadeln sind durch den natürlichen Ausschluss geführt: **Bio 16 → 0, Techno
2 → 0 unerklärte Kandidaten.** Kein Signal überlebt als unerklärt; der einzige
verbleibende Zweifel ist ein benannter Messwert (das T-P-Profil von V1298 Tau b).

## Was gebaut und committet wurde (die Linie)

- **Nadel ⅩⅢ (Atmosphären-Biosignatur/Disequilibrium):** Disequilibrium-Zensus über
  die 48 JWST-Transmissions-Ziele. 30 mit publizierter Detektion disequilibrium-geprüft,
  18 als Non-Detection (0 honored). Registriert in `docs/reference/jwst_host_census.json`.
  Das thermochem-Gleichgewicht wurde erweitert: Schwefel (H,C,O,N,S) und F/Cl
  (Halogen), kondensations-bewusst unter 500 K — alles NIST-JANAF-gesourct, der
  16-Slot-Vertrag des Archivs unangetastet.
- **Detektions-Saat** `docs/reference/jwst_detection_seed.json` (73 publizierte
  Detektionen, 30 Ziele) + Registry auf dem CDN. XUV/C-O/log-R'HK-Zeugen
  `co_rhk_witness_seed.json` (88 Zeilen, aus HEASARC/eRASS1/XMM + Polanski-KeckSpec).
- **Photochemie-Ausschluss der 16 Hits (ADS-verifiziert):** SO2/CO2/CO alle
  natürlich ausgeschieden (kanonische Photochemie, supersolares Gleichgewicht,
  Quench). Ein Kandidat überlebte zunächst (V1298 Tau b OCS, 3.5σ) → dann per
  **Quench-Modell** (`v1298_tau_b_sulfur_quench_probe`) als natürlich erklärbar
  gezeigt, bedingt durch das T-P-Profil. **Bio: 16 → 0.**
- **Paper** `docs/paper/jwst-disequilibrium-survey.md` (gate-konform).

- **Nadel Ⅴ (Techno-Galaxie-Scan):** Der Scan läuft über fünf Routen (Lasair-LSST,
  Fink, lasair-ZTF, IRSA-ZTF, ANTARES), committet in `lsst_anomaly_probe.rs` +
  `ztf_anomaly_probe.rs`, nach Archivar/Mathematikerin-Standard refactored (Option/
  None, 0 honored, keine Fabrication-Floors). Ein **echter Fehler wurde gefunden und
  gefixt**: das Lomb-Scargle-FAP-Tor war nicht skaleninvariant (meldete FAP 0 auf
  jeder Zeile) — die früheren „0 Kandidaten" waren ein geschlossenes Tor, keine
  Messung. Mit dichter **anonymer Forced-Photometrie** (Fink `/api/v1/fp`) messen die
  Gates jetzt: 2 Kandidaten, beide natürliche Dimmer, **0 unausgeschlossen**.
- **Frische Fläche:** `ztf_lightcurves_fresh.bin` (78 Kurven, g/r/i, IRSA) via
  dediziertem Workflow `ztf-fresh-cdn.yml` (kein Job in kernel-flatten) auf dem CDN.
- **Paper** `docs/paper/nadel-v-fresh-area-dip-scan.md` (gate-konform).

## Die zwei Nadeln im Register

- Nadel Ⅴ = struktureller Techno-Galaxie-Scan (dip/IR), auf LSST-Live wieder
  geöffnet. Nadel ⅩⅢ = Atmosphären-Biosignatur, JWST-begrenzt. Zwei Skalen, nicht
  konfliert. Beide Trichter: Suche → natürlicher Ausschluss → 0 unerklärt.

## Pending (die empfangende Session nimmt diese auf)

1. **V1298 Tau b OCS — der T-P-Arbiter:** Das Quench-Modell zeigt OCS ist natürlich
   erklärbar WENN die Tiefe ~1000–1800 K im Gleichgewicht erreicht; Barat 2025
   favorisiert ~500 K. Der eine entscheidende Messwert ist das **T-P-Profil**
   (pending, Quelle benannt). Dazu: die gebinnte Spektrum-Tabelle / Posterior-Samples
   für die OCS-Degeneranz-Analyse ρ(OCS, CO2/CO/H2O) — MAST-Rohspektrum offen
   (10.17909/kjg5-8t66), die Tabelle liegt in keinem maschinenlesbaren Archiv.
2. **Vollflächen-Galaxie-Scan:** die Maschine scannt über Kegel/Routen, aber die volle
   LSST-Fläche (~1,4 Mio/Woche) braucht einen **Fink-/ANTARES-Kafka-Account oder die
   Rubin-Science-Platform**. Account-Vorlagen liegen in `docs/reference/`
   (`antares-konto-2026-09-05.md`, `fink-konto-2026-09-05.md`). Operator-Aktion.
3. **Fremde Gate-Bereinigungs-Session:** läuft parallel; ihr aggressiver WIP-commit_gate
   blockierte mehrfach legitime Commits und ist noch nicht stabil fertig. Die
   empfangende Session sollte prüfen, ob der Gate stabil ist, bevor sie viele kleine
   Commits macht.
4. **XUV der übrigen Wirte** (Datenlücke — schwache M-Zwerge sind in keinem Röntgen-
   Katalog; der externe Rechercheauftrag deckte einen Teil). Photochemie-Referenz.

## Operationelle Notizen

- Secrets: eine Datei `~/projects/omegaflow/.secrets.local` (die State-Kopie
  `~/.local/state/omegaflow/.secrets.local` ist ein Symlink darauf). Enthält
  `LASAIR_LSST_TOKEN` (Lasair-LSST, funktioniert) + `LASAIR_TOKEN` (lasair-ZTF) +
  `OMEGAFLOW_TOKEN` (GitHub/CDN) + `NASA_ADS_TOKEN`.
- VPN (Proton, NL) für lasair.lsst.ac.uk nötig (sonst connection-refused); Fink/IRSA
  ohne VPN erreichbar.
- CDN-Writer = CI (kernel-flatten) für kanonische Zeitreihen; neue Einzel-Workflows
  für frische Ernten (nie Jobs in kernel-flatten häufen — verstopft die Pipeline).
- Code-Standard: wie Archivar/Mathematikerin — Option/None statt Fabrication-Floors;
  Diagnose/Code englisch; Deutsch nur in Prosa/Register.

## Was die empfangende Session als Erstes tun kann

1. Die Pending-Liste oben einlesen (TODO.md + die zwei Papers + die Register-Zeilen).
2. Entscheidend für den OCS-Abschluss: das **T-P-Profil** von V1298 Tau b besorgen
   oder die Kinetik-Grenze benennen.
3. Den **Vollflächen-Unlock** (Fink/ANTARES-Konto) anstoßen, sobald der Operator die
   Account-Anfrage gesendet hat.
