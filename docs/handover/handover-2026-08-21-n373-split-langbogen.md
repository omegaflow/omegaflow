<!--
  title: Handover: sb441-n373-Langbogen (Split-Weg) — Schritt 0 vermessen
  class: handover
  date: 2026-08-21
  sha256: 4c80f43bbc021b82281c65971a222a0093fcad8a0595427ecc6fbd6feb7b384d
  see-also: docs/SOURCE_PORT.md .github/workflows/kernel_flatten.yml
-->
# Handover: sb441-n373-Langbogen (Split-Weg) — Schritt 0 vermessen

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein
ls -la kernels/de441.bsp kernels/sb441-n16.bsp   # 3,3 GB + 616 MB liegen lokal
grep -nE "ephemeris_(eris|haumea|makemake)" phi/sources.φ
curl -sI --max-time 60 https://ssd.jpl.nasa.gov/ftp/eph/small_bodies/asteroids_de441/sb441-n373.bsp | grep -i content-length
```

Referenzen (stehend): `src/bsp_reader/daf.rs` + `src/bsp_reader/spk.rs` (der
DAF-Record-Walk, 1024-B-Records, `read_summary_record`), `src/bin/ephemeris_compiler.rs`
(der `asteroids`-Zweig, `flatten`, `--small-bodies`), `.github/workflows/kernel_flatten.yml`
(der Asteroiden-Schritt), `phi/pipeline/katalog/asteroid_gm_inpop25c.φ` (das
Katalog-Muster), `phi/sources.φ` (die drei TNO-Blöcke `ephemeris_eris.bin`,
`ephemeris_haumea.bin`, `ephemeris_makemake.bin`).

## Was schon steht (diese Session, committet)

Der Langbogen von ceres + vesta ist erledigt: `asteroids`-Systemmodus im
ephemeris_compiler (sb441-n16.bsp + de441.bsp als Sonnen-SSB-Träger,
256-Tage-Raster, Grad 17, `--small-bodies`, laute Carrier-Gates), CI-Schritt
vor horizons_compiler, Register geschlossen, das alte Handover archiviert
(458afcb, b5deb23, 68b51a4, 920c2ee). ceres/vesta tragen 24.253 Granulen =
8001 v. Chr.–9000 n. Chr. (JD −1200525,5–5008242,5), Roundtrip ≤ 9,4 m über
4000 Zufallsepochen, GM aus gm_Horizons.pck. horizons bodies_stable trägt die
12 Körper ohne SPK-Segment weiter (fehlt, nicht null).

## Auftrag

Den **sb441-n373-Langbogen** über den **Split-Weg (Weg 1)** bauen: CI streamt
die 14,13-GiB-Datei, ein `spk_split`-Werkzeug kompiliert daraus die
Langbogen-Bins der Körper mit sources.φ-Block — **eris, haumea, makemake** —
und hebt sie auf den CDN (`--clobber` überschreibt die 12-Monats-Bins).
Entscheid aus der Vor-Session (der Operator hat ihn getroffen):
**Streaming-Reader ist Over-Engineering.** Das CDN hält bereits einzelne
`ephemeris_<body>.bin`-Assets, der Runtime-Archivar fetched einzelne Körper
(kinematische Dilatation). Ein Streaming-Reader, der zur Laufzeit in einer
14-GB-Datei sucht, würde diese Datei zum CDN-Asset machen und die
1-MB-Runtime-Architektur brechen. Die 14 GB berühren nie den Laptop des
Operators.

## Schritt 0 — die vermessene Wahrheit (A = A)

- **SB441-IOM** (https://ssd.jpl.nasa.gov/ftp/eph/small_bodies/asteroids_de441/SB441_IOM392R-21-005_perturbers.pdf,
  pdftotext -layout parst sauber): Tabelle 1 = die vollständige
  Perturber-Liste — Name | Nummer | SPICE-ID | GM in au³/d², Vollpräzision,
  „match the ones used for DE441". Nachgezählt: **343 Hauptgürtel + 30 KBO
  = 373** — beide Teilsummen decken sich mit den Zahlen im IOM-Text.
- **Eris 136199 → 2136199, Haumea 136108 → 2136108, Makemake 136472 →
  2136472** — IDs aus Tabelle 1 verifiziert (Regel 2000000+N).
  **Apophis 99942, Bennu 101955, Encke: nicht in der Liste** (Tabelle 1 ist
  die vollständige 373er-Liste) — Horizons-only, benannt registriert.
- **n373s.bsp (937 MB) = Kurzfenster 1550–2650 n. Chr.** — gemessen am
  Datei-Präfix (DAF-Summaries, alle Segmente et −14200747200..20514081600).
  Das „s" ist die Kurzfenster-Variante (wie de440s), KEIN Langbogen-Träger.
  Bleibt ungenutzt.
- **n373.bsp (14,13 GiB) trägt den vollen Bogen:** jd −1200525,5..5008242,5
  (8001 v. Chr.–9000 n. Chr.), heliozentrisch (center 10), type 2,
  4 Segmente je Körper, ~41,6 MB je Körper, TNOs zuerst (Sortierung nach
  provisorischer Bezeichnung).
- **Layout-Fund (entlastet den Split):** die Datei ist eine lineare Folge
  von [Summary, Name, Daten]-Blöcken; die Chain-Pointer steigen monoton
  (nach 25 Segmenten steht der nächste Summary-Record bei 246.629 ≈ 9.865
  Records je Segment = 10,1 MB — exakt die Segmentgröße). Ein einziger
  sequenzieller Pass ohne Seek genügt; die 4 Segmente eines Körpers liegen
  zusammenhängend. Kein Adress-Umschreiben, kein Per-Body-SPK-Zwischenformat.
- **GM-Quelle:** gm_Horizons.pck trägt abweichende TNO-Nummern
  (BODY20136199 ≠ 2136199) — für n373 nicht verwenden. Die Masse kommt aus
  IOM Tabelle 1 → Katalog `phi/pipeline/katalog/asteroid_gm_sb441.φ`
  (Format: `<spice_id> | <gm_m3s2>`; Muster: asteroid_gm_inpop25c.φ;
  Kreuzcheck: Ceres 1.3964518123081070e-13 au³/d² = 6,2626e10 m³/s²,
  Vesta 3.8548000225257904e-14 au³/d² = 1,7288e10 m³/s² — beide decken
  sich mit gm_Horizons).
- **Ehrlichkeit:** der ceres/vesta-Fit trägt ≤ 9,4 m gegen das SPK — das
  SPK selbst trägt an den Intervallenden bis ~1000 km Konvergenz-Differenz
  (IOM Fig. 2, „nicht als Maß der Unsicherheit zu lesen"). Das Register
  trägt beides.

## Architektur — der Split-Weg

- **CI-Disk:** Runner haben 14 GB — die 14,13 GiB dürfen nie auf die Platte
  (`df` als Schritt 0 des Jobs). Der Stream: `curl -sfL <url> | spk_split`.
  DAF-Records sind fest 1024 B, Sequenz-Parsing ist das natürliche
  Lesemuster. Abbruch mitten im Stream → Job scheitert laut → Re-Run.
- **RAM (CI 7 GB):** de441.bsp (3,3 GB, geladen für die Sonnen-SSB-Kette)
  + ein Körper-Buffer (~41,6 MB) + Fit-Strukturen (~11 MB) — passt.
- **spk_split** (neuer bin): liest stdin, folgt der Summary-Chain
  (Pointer-Monotonie verifizieren — Abweichung = lauter Abbruch), puffert
  je Körper dessen Segmentblöcke, baut einen gültigen DAF-in-RAM
  (`DafFile::from_data` wird dazu public), evaluiert über den bestehenden
  SpkFile-Code, kompiliert `ephemeris_<name>.bin` (256-Tage-Raster, Grad
  17, ~10,9 MB), lädt hoch (`upload_asset` --clobber), verwirft den Buffer.
- **Compile-Kern teilen:** `extract_granules` / `state_ssb_multi` /
  `chebyshev_fit` / `write_binary` wandern aus dem ephemeris_compiler in
  ein Lib-Modul — beide Binaries teilen ihn, kein Duplikat.
- **Umfang:** nur die Körper mit sources.φ-Block (eris, haumea, makemake).
  Die übrigen 370 bleiben benannt pending (sources.φ-Blöcke =
  SOURCE_PORT-Kuration) — unlesbare CDN-Assets sind Registerschuld.
- **Tabelle + Horizons:** `naif_body_ids.tsv` bekommt die drei
  verifizierten IDs (parent 10); horizons_compiler streicht
  eris/haumea/makemake aus `bodies_stable` im selben Commit wie der
  CI-Schritt (die Bins landen vorher, sonst Lücke).
- **GM-Katalog:** Ernte aus dem IOM-PDF; Gate: exakt 343+30 Zeilen, alle
  IDs siebenstellig, Kreuzcheck Ceres/Vesta wie oben. Der Compiler füttert
  ihn als gm_text (BODYnnnnnnn_GM-Zeilen) — die Masse wird during CI
  kompiliert, nicht zur Laufzeit.

## Arbeitsschritte

1. spk_split bauen (Stream, Chain-Walk, Zielliste drucken) — erster Lauf
   gegen den Stream; lokal notfalls mit Ein-Körper-Filter für den Prototyp.
2. GM-Katalog ernten + Gate.
3. NAIF-Tabelle + Lib-Extraktion + Compiler-Anbindung.
4. CI-Schritt (nach dem Asteroiden-Schritt, vor horizons), horizons-Kürzung.
5. Register: TODO-Zeile präzisieren/schließen, KERNEL_INDEX-Flatten-Policy.
6. Roundtrip-Gates, cargo check 0/0, Tests, Handover archivieren
   (nach dem letzten Commit, nie vorher).

## Gates

- Zielliste des ersten Split-Laufs: 373 Ziele, enthält 2136199/2136108/
  2136472, keine Duplikate, jedes Segment jd −1200525,5..5008242,5.
- GM-Katalog: exakt 373 Zeilen, Kreuzcheck Ceres/Vesta wie oben.
- Roundtrip der drei Bins gegen den Stream: ≤ ~10-m-Ordnung, Fenster voll.
- cargo check 0/0, cargo test still (kein Fenster, kein Audio), Uploads
  erreichen den CDN, kein void-Report.
- Ein Commit je Einheit; das letzte schließt die TODO-Zeile.

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik, `src/te.rs`, die NCEI-SSI-Ernte
(eigenes Atom), die Sonden (horizons dynamic), der `wm2_1au`-Pfad, die
OMEGAFLOW_HIDDEN-Radiator-Stille, der planets-Schritt (fremder Index-Stand,
z. B. de721). Nur: n373-Split-Weg + GM-Katalog + Tabelle + Register.
