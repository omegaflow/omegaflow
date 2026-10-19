# P7.1-Kopplung + Grenzflächen + Regime-Namensschnitt — Rat + Roster (2026-10-12, river-200, dritte Runde)

Operator-Wort (2026-10-12): „council fragen müssen mit search all und max roster besprochen werden."
Die P7.1-Frage lief durch alle drei Kanäle: `archive_search --all`
(`state/river/p71-coupling-all-2026-10-12.txt`, 238 Zeilen) · Rat (fünf Stimmen) · Roster (UI-Seats).

## Die drei Fragen

**(c)** P7.1-Kopplung `Plasma ⊕ Fluid`: wo lebt die sparse Paar-Matrix (Bilinearform über Kanäle)?
**(d)** Grenzfläche zwischen zwei Pfadsegmenten: der Deskriptor trägt nur **einen** `boundary` —
welche Form für die Paar-Relation?
**(e)** Namenskollision: `enum Regime` trägt heute das PDE-Regime; das Plasma-Regime
(`ν/ω, ω_c/ω, ω_p/ω`) ist etwas anderes.

## Rat (fünf Stimmen) — Entwurf

**(c) Kopplung:** zwei neue Typen **neben** `ChannelRegistry`, nicht darin:
```
enum CouplingKind { IonNeutralFriction, Thermoelectric, Hall, Pedersen }
struct Coupling { kind, from: u64, to: u64, coefficient: Option<f64>, unit, region: Option<String> }
struct CouplingRegistry { pairs: Vec<Coupling>, id: HashMap<(u64,u64,CouplingKind), usize> }
```
- **Sparse per Konstruktion** (die Liste *ist* die Matrix, kein N×N).
- **Nicht im `ChannelDescriptor`, nicht als Frame-Slot, nicht im 26×f64-Record.**
- `coupling_term(frame, reg) = Σ_pairs C_k · φ_from · φ_to`, additiv im ω()-Loop; **`channel_reduce` bleibt unberührt** (Test sichert das). Teiler bleibt `Σ_active wᵢ`.
- `schema_hash` faltet die Paar-Hashes (append-only, Tombstones); Koeffizient `None` = absent/pending, **nie 0.0**.

**(d) Grenzfläche:** gleiche Sparse-Paarform, eigener Typ (Bedingung statt Koeffizient):
```
enum InterfaceCond { Matched, Reflecting, Transmitting, Dirichlet, Neumann, Robin }
struct Interface { from: u64, to: u64, cond: InterfaceCond, region: Option<String> }
```
Direktive (Mountain) + Parser-Arm (River): `interface <from_ref> <to_ref> <cond>`. Der Deskriptor trägt weiter genau seinen `boundary`; die Grenzfläche ist das **Paar** der beiden Ränder + Durchlassbedingung.

**(e) Namensschnitt:**
| heute | künftig | warum |
|---|---|---|
| `enum Regime` (PDE) | `enum PdeRegime` | abgeleitet aus `(op,pde_type)`; Name=Implementation; hash-neutral (`as u8`) |
| Plasma `{ν/ω,ω_c/ω,ω_p/ω}` | `struct PlasmaParameters { n_e, b0, nu, …: Option<f64> }` | stetiges Datum, reitet **außerhalb** `hash`/`PartialEq` wie `body` |
| nacktes `Regime` | **gestrichen** | zwei Dinge unter einem Wort = Fabrikationsquelle |

## Roster (UI-Seats, read-only)

| Seat | (c) | (d) | (e) |
|---|---|---|---|
| **Qwen3.7-Plus** | strukturell im **Deskriptor** (Sparsity-Pattern/Operator-Topologie), **materialisiert im ω()-Loop** | **Transmission-Condition (Hyperedge)**, referenziert zwei Boundary-Instanzen | PDE-Regime → `Formulation`/`Approximation`; Plasma → `PlasmaState`/`Collisionality` |
| **Claude Sonnet 5.5 Extra** | Deskriptor als **deklarierter Kopplungsblock** (Muster + Feldreferenzen); Einträge erst im ω()-Loop materialisiert; Frame nur Cache, Manifest nur Spiegel | **eigenes Pfadobjekt** (Kante Segment i↔i+1, geordnetes Paar + Kanal-Matching); `boundary` bleibt für Außenränder | `Regime` bleibt diskreter PDE-Enum; Plasma = `PlasmaParams` (dimensional) + `PlasmaRatios` (pro ω); `regime_of(medium, ratios)` |
| **DeepSeek V4 Pro** (tryingopen) | `pending` (noch „Thinking…", keine Antwort) | — | — |
| **Z.ai GLM-5.3 Deep Think Max** | `pending` („peak hours"/capacity) | — | — |

## Die getragenen Risse (nicht geglättet)

1. **Einträge nicht unabhängig** (Claude): Impulserhaltung bindet `ρ_i·ν_in = ρ_n·ν_ni` → der Deskriptor darf **kein freies Matrixfeld** sein, sondern `ν_in` (Skalar) + Dichten, aus denen beide folgen.
2. **Kanal ohne Gegenstück** (Claude): der Plasma-Kanal endet an einer neutralen Seite — Reflexion oder Energiesenke müssen deklariert werden.
3. **Scharfes Interface vs. dünnes Segment** (Claude): die Ionosphäre ist graduell; sobald die Gradientenlänge ~λ wird, wird aus der Kante ein Segment — wer entscheidet das?
4. **Regime-Abbildung nicht 1:1** (Claude): `ω<ω_p` ohne Stöße = Cutoff/evaneszent passt in keine der fünf Klassen; Anisotropie braucht den Winkel zwischen `k` und `B` → `ω_c/ω` ist ein Tensor-, kein Skalarfeld → Regime = Funktion von `(Segment, ω)`.
5. **Nichtlinearität** (Qwen): hängt der Reibungstensor von der lokalen Ionisationsrate ab, verschwimmt die Deskriptor/Frame-Grenze.

## Bounded (die nächsten Agenten-Schritte, je `cargo check` 0/0)

1. Rename `Regime` → `PdeRegime` (nur `channel.rs`, hash-Bytes unverändert).
2. `struct PlasmaParameters` (nur `Option`-Felder, nicht in `hash`) + `PlasmaParameters::absent()`.
3. `Coupling`/`CouplingRegistry` + `is_coupled`/`coupling_of`/`coupling_term` + Test „Kopplung ändert `channel_reduce` nicht".
4. `Interface`/`InterfaceRegistry` + `is_interface`.
5. `phi/sources.φ`-Direktiven `coupling`/`interface` + Parser-Arm — Mountain (Verdikt) + River (Parse), cross-eigen.

**`pending`:** die Koeffizienten `C_k` (IRI-2020/NRLMSISE-00/IGRF); die Segmentierungsachse (λ vs. Gradientenlänge).
