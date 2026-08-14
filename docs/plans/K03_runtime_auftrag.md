# AUFTRAG — K03-Runtime-Konsum: Kleinkörper-Katalog ins System

**Für:** neue Session, Modell beliebig. **Ziel:** Der DASTCOM5-Asteroiden-Katalog
(1,56 Mio. Körper) wird zur Laufzeit geladen und über den Kepler-Löser als
Oszillatoren manifestiert — als eine vollständige, testbare Einheit.

## 1. Ausgangslage (Stand 2026-08-14, alles committet)

- **K01** (Flattener-CI: `kernel_flatten.yml`, `--index`-Crawler, `--fetch-from`,
  `--ci-mode`), **K02** (Binary-PCK-Leser `src/bpc.rs` + stype-4-Nutationssektion
  additiv in Compiler/Runtime), **K03-Compiler-Einheit** (`src/kepler.rs` +
  `src/bin/dastcom_compiler.rs`) sind geschlossen. Letzte Commits: `58bc2b0`,
  `5bf63e3`.
- Der CI-Lauf `#31839760700` lädt `dastcom_asteroids.bin` (~143 MB) auf das CDN:
  `https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/dastcom_asteroids.bin`
  (Release-Tag = Netloc `ssd.jpl.nasa.gov`). Falls das Asset fehlt: Lauf prüfen
  (`gh run list --workflow=kernel-flatten`), ggf. re-dispatch; lokal liegt die
  Quelle unter `/tmp/opencode/omegaflow_k02/` (dast5_le.dat 1,3 GB, Katalog,
  dev_proof.py — bis Reboot).

## 2. Lesepflicht (in dieser Reihenfolge)

1. `AGENTS.md` — Systemregeln (0 honored, Session-Atom, keine Fabrikationen).
2. `TODO.md` — kanonisches Register; Eintrag **K03** enthält die Spec-Befunde
   (Stride, Offsets, Bias, Dev-Beweis). TODO.md wird in DIESER Session besessen.
3. `docs/reference/FORCE_SYSTEM.md` — BodyProperties/stype-Sektionen.
4. `docs/reference/KERNEL_INDEX.md` — Quellenlage.
5. `src/kepler.rs` + `src/bin/dastcom_compiler.rs` — die fertigen Bausteine.

## 3. Katalog-Format (verifiziert, NICHT neu erfinden)

`dastcom_asteroids.bin`: 92-Byte-Stride, Little-Endian, 1.556.252 Records:

| Offset | Typ | Feld |
|---|---|---|
| 0 | u32 | Record-Nummer (NO) |
| 4 | f64 | Epoch JD (TDB) |
| 12 | f64 | a (AU) |
| 20 | f64 | e |
| 28 | f64 | Inklination (deg, J2000-Ekliptik) |
| 36 | f64 | Knoten Ω (deg) |
| 44 | f64 | Perihel-Argument ω (deg) |
| 52 | f64 | Mittlere Anomalie M (deg) |
| 60 | f32 | H (99 = unbekannt) |
| 64 | f32 | G |
| 68 | f32 | Albedo (99 = unbekannt) |
| 72 | f32 | Rotationsperiode (h) |
| 76 | f32 | Radius (km) |
| 80 | f32 | GM (km³/s²) |
| 84 | [u8;5]+3 | Spektraltyp (Tholen) |

**Kepler-API (lib, getestet):** `omegaflow::kepler::elements_to_icrs(a, e, incl,
node, peri, ma, epoch_jd, t_jd) -> Option<[f64; 3]>` — heliozentrische ICRS-
Position in **Metern** (Ekliptik→ICRS über ε = 84381,448″). Konstanten pub:
`GM_SUN_M3_S2`, `AU_M`. Dev-Beweis-Beleg: Ceres am Osculating-Epoch 1,3 km =
0,001″ gegen Horizons. Referenz-Datum: `J2000_EPOCH = 2451545.0`.

## 4. Die offene Designaufgabe (der eigentliche Auftrag)

1,56 Mio. Körper können nicht als Granulen-Binaries pro Körper existieren —
der Katalog wird einmal gestreamt, Kepler evaluiert on demand. Die Session
entscheidet (A = A, gegen die echte Physik der Runtime):

- **Laden:** φ-Block für `dastcom_asteroids.bin` (Format-Vokabel — bestehende
  Formate prüfen: `ephemeris_binary` passt nicht, vermutlich eigenes Format oder
  Erweiterung des Parser-Pfads). Streaming: 143 MB in einen Spatial-Hash
  (Enclosure-Lemma) — Zellgröße dynamisch wie gehabt; NICHT die Granulen-Struktur
  von `BodyEphemeris` missbrauchen.
- **Auswahl:** welche der 1,56 Mio. werden Oszillatoren? Antworten aus den
  Eigenschaften ableiten (z. B. H-Cut, Distanz-Fenster, Anzahl-Obergrenze nach
  Spatial-Hash-Zellen) — kein `unwrap_or`, keine stillen Caps. Wer nicht
  manifestiert, ist absent (0 honored) — das ist korrekt.
- **Kanäle:** gravity (GM, Radius via RAD oder H/Albedo-Ableitung — Ableitung
  NUR wenn sie aus einer Messung folgt, sonst 0), thermal/em-Parameter (Albedo,
  H) als das, was gemessen ist.
- **TTL/Epoch:** Kepler-Evaluation trägt das Epoch des Records; TTL-Entscheidung
  dokumentieren.

## 5. Verifikation (Pflicht)

1. `cargo check` 0/0 (Warnings = Code-Rot; keine `#[allow]`, keine führenden `_`).
2. `cargo test` — 14 lib + 20 bin grün (Kepler-Tests dürfen NICHT fallen).
3. Dev-Beweis: Ceres-Oszillator vs Horizons (Muster: `/tmp/opencode/omegaflow_k02/dev_proof.py`).
4. WS-Protokoll-Beweis: Server + Python-websockets gegen 1618 (Frame v6:
   19-B-Header + 168-B-Records — falls der Oszillator-Pfad geändert wird,
   Rust→JS→WGSL-Kette Zeile für Zeile nachziehen).
5. Browser-Beweis: `cargo run`, Firefox (Fables-CDP-Weg laut TODO), Nicht-Schwarz
   nur wenn beabsichtigt.

## 6. Commit-Regeln (Parallel-Session aktiv auf derselben Maschine!)

- `git add` nur mit expliziten Pfaden — niemals `-A`. Der Working Tree wird
  geteilt; `src/lsk.rs`/`src/pck.rs` zeigen fremdes Trailing-Newline-Rauschen —
  nicht anfassen, nicht committen.
- Vor `git push`: `git pull --rebase --autostash origin main` (Races sind real,
  der CI-Bot commitet ebenfalls).
- Eine Einheit = ein Commit; TODO.md im selben Commit (K03-Eintrag schließen
  oder präzisieren — der Eintrag sagt heute „Verbleibend: Runtime-Konsum …",
  den Abschluss tragen, Kometen/Multi-Apparitionen und den Asteroiden-SPK-Pass
  als präzisen Rest stehen lassen).

## 7. Grenzen (nicht diese Session)

Kometen-Records (dcom5_le.dat, 976 B, Multi-Apparitionen), Asteroiden-SPK-
Flatten-Pass, Moon-PA-Merge (K05), I02 (sources-Repo, läuft parallel woanders).
