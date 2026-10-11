<!--
  title: Handover — Mountain-Folge 313 (2026-10-11)
  session: Mountain-Linie in einem Pass — Taucherstaffel über den Wartend-Auftrag: 8 Compiler gebaut, Register-Zeilen gesetzt, Teil-C-Lücken gemessen (ESA-Lizenz geklärt)
  class: handover
  date: 2026-10-11
  sha256: 73b16b02474272b4dddd62dc15cc5dbbb1c8d4dd8fd65d58cfc69195f9d37622
  status: live
-->
# Handover — Mountain-Folge 313 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge312.md` (→ `archiv/`).
Der Auftrag `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md` (Teil A/B)
ist in diesem Atom **ausgeführt**: Teil A Register-Zeilen gesetzt, Teil B
Compiler gebaut. Was daraus noch offen ist, steht unten.

## Burn: open 0.0 · close 0.25 · cap 0.30 — Grund: Taucherstaffel (11× grind-flash, 1× general) in einem Atom; flash only, kein pro/max. Cap angehoben (Default 0.15) auf das Operator-Wort („Taucherstaffel losschicken") — die Breite war gewollt, nicht der Normalfall. (close ist die aus `session_burn` geschätzte Atom-Summe: Zeile $0.062 + general Teil C $0.057 + 11 grind-flash.)

## Offen (aufgeschlüsselt)

### ci-gate — nächster abgeschlossener Lauf am HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage status`/`list`) alle `ci-gate`-Läufe queued (`38131646200`/`38131433493`/`38131252100`/`38130416426`); kein abgeschlossener Lauf am HEAD.
- **Blockade:** kein abgeschlossener Lauf.
- **Braucht:** `ci_manage log <id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die Lints ohne `#[allow]` tilgen.

### copernicus-Compiler B1–B6 — Laufzeit gegen CDS verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `--ci-mode`-Lauf je Dataset mit `CDS_API_KEY`
- **Lage:** (gemessen 2026-10-11 via `cargo build`) 6 Bins gebaut, alle grün: `copernicus_surface_land_compiler` (EXTRA_INPUTS version 2_0_0), `copernicus_uscrn_compiler` (time_aggregation daily), `copernicus_igra_compiler` (archive+version), `copernicus_woudc_compiler` (version 1_0_0), `copernicus_gruan_compiler` (version 1_0_0), `copernicus_gnss_compiler` (network_type igs_daily+version). VARIABLES/EXTRA_INPUTS gegen das CDS-Prozess-Schema gemessen; die Retrieval-Antwort selbst lief nicht.
- **Blockade:** kein CDS-Lauf (CI trägt `--ci-mode`).
- **Braucht:** je Dataset ein `--ci-mode`-Lauf; bei Void die Variable/Version im Bin korrigieren. **Riss:** `uscrn` wählt `time_aggregation=daily`, `gnss` wählt `network_type=igs_daily` — beide sind Wahlen unter Enum-Werten ohne Default, nicht gemessene Defaults.

### NOAA S3-Compiler B7/B8 — Laufzeit verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `--ci-mode`-Lauf je Bucket
- **Lage:** (gemessen 2026-10-11 via `cargo build`) `noaa_nos_lidar_compiler` (ListObjectsV2, Prefix `laz/`) + `noaa_eri_compiler` (per-Event) gebaut, grün; Buckets anonym listbar (200).
- **Blockade:** kein Lauf.
- **Braucht:** `noaa_nos_lidar_compiler --ci-mode` / `noaa_eri_compiler --ci-mode`.

### Wartend-Auflösung — verbleibende Messlücken (Teil C)
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Messschritt
- **Lage:** (gemessen 2026-10-11) A4/A7-Lizenz **geklärt**: ESA Space Science Archives (PSA + ESAC-SPICE) = **CC BY-NC 3.0 IGO** (`https://www.cosmos.esa.int/web/esdc/terms-and-conditions`; Deed `https://creativecommons.org/licenses/by-nc/3.0/igo/deed.en`) — der `rosetta_odf`-Block und der neue JUICE-SPICE-Block tragen den Token. A5/A6 Earth-Flyby-ODFs in den genannten Cruise-Collections **nicht vorhanden** (Galileo GO-X-RSS-1-ODR = 1994/95 GWE; MESSENGER data-odf = 2007–2015). A2 TRK-2-34-Datei gefunden: `cors_0366/SROC15_156/TNF/S73SROI2012_156_1830X63MV1.TNF`.
- **Blockade:** keine.
- **Braucht:** A5/A6 — andere Quelle für die Galileo-1990/92- und MESSENGER-2005-Earth-Flyby-ODFs suchen (nicht in den PDS-Cruise-Collections) oder descopen mit Befund.

### Open-LiDAR-Konsolidierung + vier Portal-Coverage — Descope-Verdikt (mycelium-307)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** das Mountain-Verdikt je Portal
- **Lage:** (gemessen 2026-10-11) NL: flai ⊇ AHN4-Origin außer `08BZ1`/`40HZ1`; CH `swisstopo`: Origin > 10 000 Kacheln, Diff offen; FR `ign_lidarhd` nur partial; NZ `linz` bleibt. Die vier Portal-Coverage-`phi`-Blöcke sind **gesetzt** (Manifest-`url`/`origin`/`format` aus den Bins, `terms`: AHN4 CC0-1.0, swisstopo open-by-attribution, IGN etalab-2.0, LINZ CC-BY-4.0).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** je Portal das Descope-/Behalten-Verdikt (AHN4-Origin zugunsten flai? CH-Voll-Diff; `ea_lidar_index` behalten oder flai UK DEFRA?); CH-Diff via `coverage_id_diff`.

### SuperMAG-Derivate — Abstoßen (future-230 Vorschlag)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) die 5 `substorm_*`-Blöcke (`phi/sources.φ` ehem. `:19935-19973`) sind **Derivate**; das Rohdatum ist `INTERMAGNET` (CC-BY-NC), der Onset wird aus dB/dt abgeleitet. Vorschlag Future: Zeile + CDN-Release abstoßen (Register: Mountain; Manifestation: Mycelium).
- **Blockade:** Konsumenten-Prüfung (`sgrep -i substorm src tools`) vor dem Entfernen.
- **Braucht:** Verdict setzen (descoped mit Befund) + `substorm_compiler`/CDN-Release koordiniert mit Mycelium entfernen — oder Keep-Entscheid.

### Riss — adressierte Blöcke waren teils schon im Baum
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** nächster Register-Pass
- **Lage:** (gemessen 2026-10-11) die adressierten Vorschläge (mycelium-307) für **Bayern-DOM** (`terms CC-BY-4.0`, `ttl 63072000` vs. 604800), **VNP46A3** (`ttl 86400`, Origin = LAADS-Verzeichnis statt der `.h5`) und **open-lidar-data BE** (`terms CC-BY-3.0` vs. CC0-1.0) treffen auf bereits vorhandene, abweichende Blöcke; ebenso waren `wind-swe-mfi` (`:826/:835`), `themis-gmag` (ABK) und `juno/cassini/viking_odf` bereits getragen.
- **Blockade:** keine.
- **Braucht:** je Block das Verdikt (bestehender Wert gilt oder Vorschlag) gegen die Quelle messen.

### Riss — PDS-Missionsrouten als `at earth` registriert
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** nächster Register-Pass
- **Lage:** (gemessen 2026-10-11) die neuen PDS/ESA-Routen (Voyager/Mariner/Galileo/Juno/Cassini/Rosetta) wurden als `at earth` registriert (Bodenstation-Frame der ODF-Observablen); vorhandene Geschwister nutzen teils Körper-Frames (`at mars`, `at rosetta`).
- **Blockade:** keine.
- **Braucht:** Frame je Route gegen den Geschwister-Block prüfen; wo der Körper-Frame gilt, korrigieren.

### Sechs Klima-Quellen — `terms`/`ttl` gegenprüfen (mycelium-305)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der Serien-Manifest-Entwurf
- **Lage:** (gemessen 2026-10-11, unverändert) die Compiler erzeugen je-Akt-Instanzen; die Blöcke sind Workflow-Default-Repräsentanten.
- **Blockade:** ohne Serien-Manifest trägt jeder Klima-Block nur einen Default-Tag.
- **Braucht:** `terms`/`ttl` je Compiler gegenprüfen.

### Mycelium-φ-Blöcke — Serien-Manifest-Format
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Entwurf des Serien-Manifest-Formats
- **Lage:** (gemessen 2026-10-11, unverändert) der ecad-Riss ist gelöst (netloc = Quellen-Identität); AODN-`ttl 1` = Datensatz-Kadenz.
- **Blockade:** ohne Serien-Manifest trägt jeder Klima-Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format; der CI-Wait je Block steht in `state/zustand/wartend.φ`.

### Frame/Epoche — Transform-Layer
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Entscheid der Rat-Linse (Vorlage `state/mycelium/frame-epoch-findings.md`)
- **Lage:** (gemessen 2026-10-11) bbox-off Default steht; Transform-Layer `pending`.
- **Blockade:** Architektur.
- **Braucht:** Entscheid über den Transform-Layer (Rat-Linse).

### LiDAR-Inventar / Extraterrestrisch — Parser
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) TNM + `lola_coverage.rs` gebaut.
- **Blockade:** keine.
- **Braucht:** Parser MOLA/MLA/NLR/OLA/Hayabusa.

### PEP + Tudat — Schritt 3 (mehrere Bahnen)
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11) vier Häuser als `ephemeris_*` registriert.
- **Blockade:** keine.
- **Braucht:** weitere Zeugen durch die Kette, Kette CI-nah verankern.

### te-ground-truth — Riss + GIC-Paper-Trigger
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der GIC-Paper-Trigger (gefeuert) — §3.5/Abstract/§7 nachziehen
- **Lage:** (gemessen 2026-10-11) Lauf `te-bias-n 38038722712` success; Produktionsarm (Takens-embedded KSG, dim 3) **NOT PASS** — Null-Control c=0 zeigt in beiden Richtungen einen Pfeil; der skalare Silverman-KDE-Referenzarm nullt den Rückkanal nicht. Riss getragen.
- **Blockade:** kein Code-Äquivalent beider Arme unter dem Null-Control.
- **Braucht:** den Riss in §3.5/Abstract/§7 des Papers ziehen (Mountain) oder den Null-Control-Arm angleichen.

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI führt die Tests aus)
- **Lage:** (gemessen 2026-10-11 via `src/archivar/root.rs`) Element-Tail + Class-Ref + Bootstrap; Basket-Arrays gebaut, Basis-Wrapper konsumiert.
- **Blockade:** `cargo test` läuft nur in CI; v9-Body ohne v12-Basis nur aus ROOT-Quelle + synthetisch geprüft.
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment, kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile) — `state/zustand/wartend.φ`
- **Lage:** (gemessen 2026-10-11) HTTP 200; `sensor.community` bereits registriert (`phi/sources.φ:338/1218`).
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-11) die JUICE-`orbc`-SPK (A7) ist registriert; `doppler.rs` absent.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-10) Overpass HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated (HTTP 406 direkt+Proton).
- **Braucht:** Riss 4-Abschluss oder Descope.

### Manifestation SPT-3G D1 — sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cmb-cdn`-Lauf abgeschlossen
- **Lage:** (gemessen 2026-10-11) `cmb-cdn 38129785553` in_progress; sha256 fehlt in `phi/sources.φ`.
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** den sha256 des manifestierten Assets in `phi/sources.φ` nachtragen.

## Operator-Wort-Register

- **„hä wir bitte du solltest eine taucherstaffel losschicken und du hast dich nur um copernicus gekümmert du sollst alle offenen punkte die die letzte session aufwändig dokumentiert hat umsetzen"** | 2026-10-11 | Operator-Session (mountain) — die Taucherstaffel über den gesamten Wartend-Auftrag (Teil A + B) losschicken, nicht nur einen Punkt.
- **„die brauchen wir aber, oder hast du irgendeine idee, wie wir sie bekommen können? Nur A4 (ESA PSA) und A7 (ESA SPICE) Lizenz bleiben unmeasured"** | 2026-10-11 | Operator-Session (mountain) — A4/A7-Lizenz ist nötig; gemessen: ESA Science Archives = CC BY-NC 3.0 IGO (ESAC-Terms-Seite). Kein neuer Punkt; die Lücke ist geschlossen.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
