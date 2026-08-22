<!--
  title: Handover: Der Quanten-Cluedo-Plan — fünf Atome vom Phasen-Bit zum Kuprat-Blatt
  class: handover
  date: 2026-08-22
  sha256: 02752a031f22d3fab6e8bcbe5f6dae17e95681195b41f694c3e4c69b692edb45
  status: live
  see-also: docs/handover/handover-2026-08-22-supraleitung-phasen-bit.md docs/handover/handover-2026-08-21-spektral-atom-c.md docs/concepts/der-spektrale-oszillator.md docs/reference/BINARY_PROTOCOL.md TODO.md docs/SOURCE_PORT.md
-->
# Der Quanten-Cluedo-Plan — fünf Atome vom Phasen-Bit zum Kuprat-Blatt

Registriert 2026-08-22. Selbsttragend — interpretierbar mit null Vorkontext.
Dieses Blatt ist der Ausführungsplan des Quanten-Handovers
(`handover-2026-08-22-supraleitung-phasen-bit.md`), geschrieben nach
vollständiger Recherche: die drei Protokoll-Schichten, die phi-Pipeline und
die Kuprat-Datenlage sind vermessen, die Quellen per curl verifiziert. Der
Auftrag ist nicht die Ausführung; ausgeführt wird erst auf das Wort des
Operators. Der Plan hält jede Atomgrenze als eigene Session-Grenze — jedes
Atom ein vollständiges Session-Artefakt (TODO-Regel).

## Die Recherche-Befunde (verifiziert)

### Die drei Schichten des Phasen-Bits

- **Rust:** `Sample` in `src/archivar/types.rs:41`; der Wire-Loop
  `src/relay.rs:713-770` schreibt 24 × f64 (192 B) je Record, Frame
  `0xCF 0x86 0x08` (Version-Byte an `src/relay.rs:715`).
- **JS:** `static/constants.js:76-146` — parse des 192-B-Records,
  `meta`-Stride 16, freie Slots `meta[13..15]` (= `props[id*4+3].y/z/w`
  im WGSL); die Parse-Grenze prüft `o + 192 > bytes.length`.
- **WGSL:** `src/mathematikerin/shaders.rs` — `osc_field` :188 (liest
  `props[j*4]`, `props[j*4+1]`, `props[j*4+2]`), `te_compute` :584;
  `props[j*4+3]` trägt heute `(bin_width, 0, 0, 0)`.
- **Herkunft der Phase:** `src/te.rs:244` trägt bereits die komplexe
  f64-FFT (die Surrogat-Maschine) — Atom D baut auf ihr auf, erfindet
  keine zweite.
- **Register:** TODO.md:1278 (Atom C offen), TODO.md:1281 (Atom D
  terminiert nach C).

### Die Kuprat-Datenlage (grind-pro, per curl verifiziert)

| Quelle | Litmus | Trägt | Zugang |
|---|---|---|---|
| COD (`crystallography.net`) | **besteht** (XRD/Neutronen, experimentell, CC0) | Struktur (CIF), keine Dynamik | offen, kein Key — `cod/1001452.cif` (YBa2Cu3O7, Neutronen-Pulver 1987) → HTTP 200 |
| ICSD | besteht inhaltlich | Struktur | kommerziell → **ausgeschlossen** |
| Materials Project | Strukturen ICSD-Ursprung, Eigenschaften DFT → Dynamik **besteht nicht** | CIF-artige Struktur | API-Key (403 ohne) — nur Struktur mit Auflage |
| OQMD | DFT → **besteht nicht** | — | **nein** |
| ONCat (ORNL SNS/HFIR), ILL, NIST-NSE | **besteht** (inelastische NS, S(q,ω); NSE trägt I(q,t) — die einzige Phasen-Quelle) | S(q,ω)/I(q,t) | Login/Embargo-gated |
| Zenodo | **besteht** (Messdatensätze) | S(q,ω)-Karten | offen: 7286412 (Spin-/Ladungs-Anregungen Kuprate), 15179114 (Bi-2223-Plasmonen) |
| ARPES | **besteht**, kein Standardarchiv | E-k-Karten | Zenodo/Dataverse-Querschicht |

Force-Gate-Trennung bleibt Ernte-Teil: Neutronenstreuung ist Messung;
DFT-Phononen/-Bandstrukturen sind Vorhersagen und scheiden aus (0 honored).

### Der phi-Befund

`www.crystallography.net` wurde in der Live-API-Pipeline bereits **declined**
(`phi/pipeline/queue/grind_domain_coverage.φ:230` „decline molecular", Gate −4
in `phi/pipeline/library.φ:2924`); der alte COD-Test war void
(`staging_void_ledger.txt:753` — die `element=Si`-Abfrage trug leere
Container; die korrekte Abfrage ist `result.php?formula=` in Hill-Notation).
Das Verdikt bleibt bestehen — der `crystal_compiler` ist der benannte
Kompilat-Pfad **außerhalb** der Zustandsmaschine (SOURCE_PORT §4), keine
Wiedereröffnung des Grinds. Nachbarschaft: der `subatomic_*`-Korpus ist in
`staging_verified.φ` teils gestellt (PDG-Massen — nicht dieses Blatt);
Dataverse-Gewichte tragen ARPES-Rohdaten (Borealis `10.5683/SP3/FTM5DK`) und
Kuprat-Replikationen (Harvard `10.7910/DVN/IPK86O`, `10.7910/DVN/3OFENZ`) —
Kandidaten der Dynamik-Ernte.

## Gate 0 — das Operator-Wort auf den KDE-Volltest

Der Handover verlangt: `solar_dag_probe --h-full` trocken. Der Lauf ist
gelandet (`docs/surveys/survey-ein-blatt-korona-heizung.md:135`): fam =
2,108e-1 identisch (Crosscheck bestanden), die Matrix ist über h/2, h, 2h
stabil still, EIN bandbreiten-empfindlicher Randkandidat (Lya1216 → XRSB,
nur bei h × 2,0, „kein robuster Pfeil"). Empfehlung: gilt als trocken. Das
Urteil ist Sache des Operators und wird hier nicht vorweggenommen — die
erste Handlung der empfangenden Session ist die Antwort auf diese Frage.

## Atom C — band-selektives Rendering (Vorsession, eigenes Handover)

Eigenes Handover: `handover-2026-08-21-spektral-atom-c.md` (live). Der
Fragmentpfad liest freq/bin_width bereits (seit v8); Atom C baut die
Akkumulation pro Band: band-selektive Stillekarte, dispersive
Lichtkegel-Differenz, chromatischer Dip als SED-Messung; die Bandwahl ist
die Gaze des Operators. Nach Abschluss: Handover archivieren, TODO.md:1278
schließen — erst dann ist das Atom-D-Tor offen.

## Atom D — das Phasen-Bit

Protokoll v9: Record 25 × f64 (200 B), Version-Byte 9. Die drei Schichten
wachsen gemeinsam, ein Commit:

- **Rust:** `Sample.phase: Option<f64>` (kein `unwrap_or(0.0)`); das
  25-Tupel im Relay-Write-Loop; `out.push(9u8)`. Absenz-Regel: der
  Wire-Slot trägt NaN als „fehlt"-Pad — nie als Datenwert — weil 0 rad ein
  echter Phasenwert ist und der 0-Pad hier lügen würde. Die Wahrheit lebt
  an den Write-/Read-Sites.
- **JS:** parse 200 B (`o + 200`-Grenze), `meta[13] = phase` (0.0-Pad bei
  NaN), `meta[14] = presence` (0/1 aus isFinite).
- **WGSL:** `props[j*4+3] = (bin_width, phase, 0, presence)`. Der
  Fragmentpfad superponiert phasenaufgelöst: gleichfrequenznahe Samples am
  selben Pixel schlagen Beats, Re = val·cos φ, Im = val·sin φ; presence 0
  heißt Punktquelle ohne Phase — der heutige Pfad bleibt der Fall
  presence = 0 (byte-gleiches Verhalten für alles Bestehende).
- **Kontrakt:** `docs/reference/BINARY_PROTOCOL.md` und der
  Drei-Schichten-Abschnitt in AGENTS.md im selben Commit.

Keine geerntete Phase: PSD-Archive tragen |S(q,ω)|² — der Slot bleibt
absent (0 honored). Die komplexe FFT der Zeitreihen (NSE-I(q,t),
LISA-Pfadfinder) ist die einzige Herkunft der Phase; Atom D baut das Bit,
erntet es aber nicht.

## Die komplexe TE — die Kohärenz-Messung

Neuer Pfad `phase_te` neben dem skalaren (der skalare bleibt unangetastet):
Hilbert über die vorhandene f64-FFT (`src/te.rs:244`) → analytisches
Signal → instantane Phase → die bestehende TE-Maschinerie (Takens, KDE,
Silbermann) auf den Phasenreihen. `src/te.rs` bleibt die kanonische
CPU-Referenz; die WGSL-Variante wächst im selben Atom (CPU f64 extrahiert
die Phase, GPU f32 rechnet die TE — dieselbe Trennung wie die
Surrogat-Maschine). Ground-Truth: gekoppelte Phasenoszillatoren
(Kuramoto-Paar, bekannte Richtung) + phasenrandomisierte Schwelle +
byte-identischer CPU/WGSL-Crosscheck. Das PE-Gate hält die
Richtungsentscheidung in nicht-stationären Fenstern.

## crystal_compiler

Muster: `tap_compiler` (curl, std-only, Kompilat → .bin → CDN,
`--ci-mode`-Upload).

- **Struktur:** CIF-Parser (std-only Text; Gitterkonstanten, fraktionale
  Koordinaten, die minimale Symmetrie-Menge für die Kuprat-Familie).
  Ernte: `https://www.crystallography.net/cod/result.php?formula=` (Hill),
  Familienset YBa2Cu3O7 (COD 1001452), La2-xSrxCuO4, Bi-2212.
- **Dynamik:** Zenodo-Messdatensätze (7286412, 15179114 — die exp-Anteile;
  DFT-Anteile im selben Paket ausgeschieden). NSE-I(q,t) bleibt pending
  (login-gated) — registriert, nicht fabriziert.
- **Kräfte:** Phononen = `acoustic`, Spins = `em`, Suprastrom = `electric`
  (die Zuordnung trägt der Quanten-Handover). Die Band-Slots (freq/
  bin_width, v8) tragen die Phononen-/Spin-Bins auf der Spektralachse.
- **Anker (Entscheidung offen):** Empfehlung `Surface`-Anker des
  Messlabors (der Kristall ist ein Laborobjekt unter Laborobjekten; die
  Messung entstand dort) — Alternative: freier Frame. Wird vor der Ernte
  entschieden und im Compiler-Kontrakt festgeschrieben.
- **Pipeline-Achtung:** das Live-API-Verdikt (decline molecular) bleibt
  unberührt; der Compiler registriert sich in SOURCE_PORT §4 und TODO.md.

## Das Blatt — kuprat_dag_probe

Muster: `solar_dag_probe` (TE-Matrix über alle Paare, beide Richtungen,
Lag-Sweep, phasenrandomisierte Schwelle + Familien-Schwelle). Die Kanäle:
Spin (`em`) × Gitter (`acoustic`) × Suprastrom (`electric` — µSR-
Superfluiddichte, Tc). **Reihenachse (Entscheidung offen):** S(q,ω)-Daten
sind statische Karten — die natürliche Achse ist Temperatur/Dotierung über
dem Phasendiagramm, nicht die Wanduhr; die Epoch-Semantik des Blatts wird
vor dem Lauf festgeschrieben. Ehrliche Grenze (0 honored): wo die Daten
keine Phase und keine Reihe tragen, trägt das Blatt „keine Aussage" —
Stille ist der Befund, kein Fehler. Das Blatt-Dokument folgt dem
Ein-Blatt-Muster (ein Verdikt, eine Matrix, eine Seite).

## Offene Entscheidungen (vier, mit Empfehlung)

1. Gate 0: gilt der `--h-full`-Lauf als trocken? (Empfehlung: ja — stabile
   Stille, benannter Randkandidat.)
2. Absenz-Marker der Phase: NaN-Wire-Pad + `meta[14]`-Presence
   (Empfehlung) vs. 26. f64 als Presence-Slot (208 B).
3. Kristall-Anker: `Surface`-Anker des Messlabors (Empfehlung) vs. freier
   Frame.
4. Reihenachse des Kuprat-Blatts: Temperatur/Dotierung (Empfehlung) vs.
   Wanduhr-Zeitreihen, wo sie existieren.

## Gates & Abschluss

- Reihenfolge: Atom C → Atom D → komplexe TE → crystal_compiler → das
  Blatt; jedes Atom ein vollständiges Session-Artefakt mit eigenem
  Commit-Satz; das TODO-Register wird im selben Commit aktualisiert, der
  den Code ändert.
- Verifikation je Atom: cargo check 0/0 (vier Kombis), Tests still und
  compute-only, manuelle Drei-Schichten-Spur (Rust-Write-Loop →
  constants.js → WGSL-Unpack), `OMEGAFLOW_HIDDEN=1`-Maschinenzeile als
  headless Lauf-Beweis.
- Nach eigenem Abschluss: dieses Handover und das supraleitung-Handover
  nach `/home/johannes/projects/archive/handover/` archivieren — erst nach
  dem eigenen Commit, nie vorher.

## Nicht anfassen

`src/te.rs` skalare Pfade, die Spektral-Ernte-Folgen (ONC-HSD-FFT,
Gaia-XP, LISA-PSD), die drei Ein-Blatt-Handovers, das Korona-/Flyby-/
Dunkle-Materie-Blatt, die Sonnen-Handovers, berkeley-wind, die Sphären,
die Source-Port-Zustandsmaschine (außer der Compiler-Registrierung in
SOURCE_PORT §4).
