<!--
  title: Kanal-Ontologie — kompletter Bau (feste 9 → Kapazität 2ⁿ + lebendiges n)
  class: concept
  date: 2026-10-09
  sha256: bccd06f162efe7492a671f6a4588c2382fa0e9a35edce9a010eacf5c342cfd18
  status: live
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/tool-forms.md state/stimmen/2026-10-09-river-kanalzahl-frontier.md
-->
# Kanal-Ontologie — kompletter Bau

Das Verdikt der Frontier-Runde (Rat + 8 Open-Weight + 5 UI, `state/stimmen/2026-10-09-river-kanalzahl-frontier.md`)
ist einstimmig: die feste Kanalzahl 9 ist **A≠A**. Dieser Plan ist der **komplette** Umbau
auf die A=A-Form. Kein „erster Bau" — die ganze Strecke, in Abhängigkeitsreihenfolge,
jede Phase ein bounded dispatch mit `cargo check`-Gate (Ein Dispatch = ein begrenzter Schritt).

## Der Ziel-Kontrakt (A=A)

- **Kanal = Descriptor** `(q, 𝒯, M+Rand, u)` **⊗ Tri-State**, Identität = `hash(q,𝒯,M,u)`
  (nie der Slot, nie der Index).
  - `q ∈ {Masse, Impuls, Energie, Ladung, [Entropie]}` (Erhaltungsgröße).
  - `𝒯 ∈ {Flux(Fick/Fourier/Ohm/Newton-viskos), Advektion, Welle(Elastodynamik), Poisson}` (Transport-Operator).
  - `M+Rand ∈ {Vakuum, Fluid, elastischer Festkörper, freie Oberfläche, …}` (Medium + Randbedingung).
  - `u` = SI-Dimensionsvektor, **folgt** aus `q·𝒯` (keine freie Achse).
- **Tri-State** `{present, pending, absent}` ist **orthogonal** zur Identität, nicht Teil von A;
  `pending` ist identitätsneutral (Wert+Einheit stehen, Gewichtung 0), `absent` steht nicht im
  Nenner, **kein** `0.0`.
- **Moden sind Projektionen, keine Achsen:** acoustic/seismic-body/seismic-surface sind eine
  Zerlegung EINES elastischen Feldes; `Σ Pᵢ = 1` über Projektoren. `electric` = Alias(em);
  `thermal` = Zeile von `diffusion` (Fourier). Als getrennte Kanäle addiert zählen sie dieselbe
  Energie doppelt (Hy4-Riss).
- **Zulässigkeit statt Produkt:** das Tupel ist ein dünn besetzter Graph (Fick↔Masse, Fourier↔Energie;
  Masse×Fourier leer). Eine **Zulässigkeitsrelation** legalisiert Kombinationen, kein nacktes ⊗.
- **Kopplung zwischen Zellen:** Onsager/Seebeck, thermal⊗advective, em⊗Ladung — eine **sparse
  Paar-Matrix** (Bilinearform) über Kanäle, nicht über das Produkt.
- **Zahl:** feste **Kapazität CAP = 2ᵏ** (nur Alignment/Behälter) + **lebendiges n** als Datum;
  Reduktion `Σ_active wᵢφᵢ / Σ_active wᵢ` — der Teiler ist **nie** eine Konstante.
- **Transitionssemantik** (MiniMax, der tiefste Riss): `Active ↔ Vacant ↔ Drained ↔ Pending` braucht
  eine deklarierte Übergangsregel (Trigger + Hysterese), sonst Chatter an Grenzflächen.

## Phasen (Abhängigkeitsreihenfolge)

### P0 — Kontrakt & Register (Docs)
- **P0.1** `docs/concepts/archivar-mathematikerin.md` um den neuen Kanal-Kontrakt erweitern
  (Descriptor, CAP/n, Tri-State, offsets, Reduktion) — der Wire/GPU-Force-Abschnitt.
- **P0.2** Dieses Konzept (`kanal-ontologie-komplettbau.md`) als Träger in die Übergabe.
- **P0.3** `phi/sources.φ`: neue Direktive `channel <conserved>:<op>:<medium>:<boundary>:<unit>`
  je Quelle (Ersatz des flachen `force`-Tokens); Parser-Arm in `src/archivar/parse.rs`
  analog `fanout`/`fanout_center`. **Mountain-Domäne** (Verdikt-Zeilen) + River (Parse).
- **P0.4** `phi/canon.φ` falls eine neue Registerdatei entsteht.
- **P0.5 — Ton-Relation (Operator-Wort 2026-10-09: „es muss doch alles zusammenpassen").**
  Das Ton-Modell wird **vor** P1/P2.2 entschieden (nicht erst in P6.2): der Klang ist **keine
  Zuordnung `𝒯→Klangfamilie`**, sondern eine **Ableitung**: `f_j = ω_𝒯(k_j^M)/2π` — Dispersion
  aus `𝒯`, Wellenzahl-Quantisierung aus `M` (inharmonisch nur, wo `M` es erzwingt: 2D-Kreis →
  Bessel `1·1,59·2,14·2,30·2,65·2,92·3,16·3,50·3,60`; 1D-Saite → harmonisch). Parabolisch →
  Relaxationsspektrum (Debye), kein Pitch; erster Ordnung → eine Charakteristikenfamilie, reine
  Laufzeit, Doppler = Pfad zwischen zwei **deklarierten** Weltlinien; elliptisch → **kein
  Zeitmodus, kein Ton — aber nicht Stille** (Durchreicher: Quellzeitverlauf × räumlicher Gain,
  Eigenfarbe null); der erhaltene Anteil (k=0, ω=0) ist in **jedem** `𝒯` stumm. Der Deskriptor
  trägt das **Gesetz**; die **Anregung** (Quelle+Anfangsdaten) ist ein eigener Zustand. Die
  **Receiver-Schicht** (ERB/Bark) ist nicht aus `(q,𝒯,M)` ableitbar — sie ist die deklarierte
  `ReceiverAperture` oder in `M` zu absorbieren; ERB als reine Organ-Funktion. Protokoll:
  `state/stimmen/2026-10-09-river-tonmodell-deskriptor.md` (Wissenschaft + Rat + Duck/Claude/Qwen/
  DeepSeek Chat/GLM-5.3/Gemini/Mistral + DeepSeek V4 Pro/GLM 5.3/Inkling/Hy4).
  **Offene Risse (Roster-Erweiterung, zu tragen):** (i) der **fehlende Instabilitäts-Quadrant**
  Re λ>0 — die vier Familien sind stabile Propagatoren (Färbung), die **Quelle/Stimme**
  (Bogenstrich/Anblaskante/Feedback, nichtlinear gesättigt) ist ein eigener Zweig; (ii)
  **Superposition ↔ Treue** — A⊕B verliert die Einzel-Identität, der Ausweg ist die Projektion
  mit deklariertem Kopplungsoperator C + Anregungs-Verfolgung; (iii) **Steifigkeit höherer
  Ordnung** (biharmonisch, B) wird vom 2.-Ordnungs-Operatorensatz verfehlt; (iv) die
  `𝒯`-Liste ist uneinheitlich (Ordnung vs. Typ; „Flux" = Fick-Diffusion, sonst fehlt parabolisch).

### P1 — Ontologie-Kern (`src/mathematikerin/force.rs` + neu `channel.rs`)
- **P1.1** `enum Conserved { Mass, Momentum, Energy, Charge }`.
- **P1.2** `enum TransportOp { Flux(FluxKind), Advective, Wave, Poisson }`.
- **P1.3** `enum Medium { Vacuum, Fluid, ElasticSolid }`, `enum Boundary { None, FreeSurface, … }`.
- **P1.4** `struct ChannelDescriptor { conserved, op, medium, boundary, unit }` + `hash()`.
- **P1.5** `fn is_admissible(q, op, medium) -> bool` — die **Zulässigkeitsrelation** (der Graph).
- **P1.6** `enum TriState { Present, Pending, Absent }` + Übergangsregel (P7.2).
- **P1.7** `struct ChannelRegistry { cap: usize, descs: Vec<ChannelDescriptor>, id: HashMap<u64, usize> }`;
  Moden als **eine** abgeleitete Instanz; `electric` = Alias(em); `thermal` = diffusion-Zeile.
- **P1.8** Tests: `thermal==diffusion` dedupliziert; Modus-Projektor `Σ Pᵢ = 1`; Zulässigkeitsgraph
  (Masse×Fourier verboten); `ChannelId` = Hash, stabil.

### P2 — Frame & Reduktion (`src/mathematikerin/actuators.rs`)
- **P2.1** `struct PresenceFrame { n: u16, omega: [f32; CAP], aperture: [f32; CAP],
  state: [u8; CAP], pan_ms, tilt_ms, tau_ticks }` (CAP = 2ᵏ).
- **P2.2** **Der A=A-Kern zuerst:** `channel_intensity` → `Σ_active wᵢφᵢ / Σ_active wᵢ`
  (Monoid `(Σ, W)`), Teiler nie konstant; `Pending`/`Absent` korrekt (kein 0.0-Mittel).
- **P2.3** `acoustic_partials`/`acoustic_amplitude` über `n`/active, nicht 9; **deskriptorgetrieben
  nach P0.5** — kein Slot-Harmonisches (k+1), sondern `f_j = ω_𝒯(k_j^M)/2π`; das Regime wird
  **pro Mode** über die Diskriminante der Dispersionsrelation entschieden (nicht pro Kanal);
  elliptische Kanäle als Durchreicher (kein Ton), der k=0/ω=0-Anteil stumm.
- **P2.4** Alle `[f32;9]`-Signaturen → `&[f32]` + `n` (oder `ChannelVec`).

### P3 — Wire & Relay (`src/archivar/relay.rs`, `main_flow.rs`)
- **P3.1** Wire-Kopf `{ n: u16, schema_hash: u32 }` + `offsets[n+1]` + `f32[n]` + 2-Bit-State-Maske.
- **P3.2** `TcpRadiator`: n Aperturen mit offsets, nicht 9.
- **P3.3** selbstbeschreibend: Deskriptor-Manifest beim Handshake/Szenen-Load.

### P4 — GPU / WGSL (`src/mathematikerin/shaders.rs`, `omega.rs`)
- **P4.1** Storage-Buffer **runtime-sized array** + `arrayLength`; `n` als Uniform; **kein `const N=9`**.
- **P4.2** `force_type`-Switch → **Dispatch über den Operator** (Kernel je 𝒯), nicht über den Slot.
- **P4.3** `field_permeability`/`probe_omega`/`force_ref` → dynamisch; `integral/9.0` strukturell raus.

### P5 — Browser (JS) (`static/constants.js`, `static/radiator.js`)
- **P5.1** `parseKinetic` liest `n` + `offsets` (nicht 9).
- **P5.2** `radiator.js` projiziert über `n` Kanäle (per-Senke-Projektion).

### P6 — Senken (je Organ-Invariante)
- **P6.1** **Bild/Luminanz**: erst auf ein gemeinsames dimensionsloses Maß normieren, dann gewichtete
  Projektion (Organ kollabiert legitim — Trichromatie).
- **P6.2** **Audio**: ERB-/Kochlea-Bänder bzw. Partiale über aktive Kanäle; Kollaps nur als
  Schalldruck-Superposition (gleiche Größe) — der Ton ist deskriptorgetrieben nach **P0.5**
  entschieden (Ableitung `f_j = ω_𝒯(k_j^M)/2π`, kein Slot-Harmonisches). ERB/Bark sind die
  deklarierte Receiver-Apertur (Organ-Funktion), nicht der Kanal.
- **P6.3** **Vibration/Haptik**: 1:1 Aktuator, **bleibt kanalaufgelöst**.
- **P6.4** **Serial/HID**: struct-of-fields, dimensional; bit-exakt.
- **P6.5** **Relay/Netz**: Verbatim, verlustfrei, n-fach.
- **P6.6** **Neue Senken** (übersehen, Claude): **Archiv/Replay** (content-addressed, verlustfrei) +
  **Analyse/Kohärenz** (Kreuzkanal-Kopplung als Bilinearform). Optional thermohaptisch für `thermal`.
- **P6.7** Gemeinsames Aggregationsmaß (Leistungsdichte/Normalisierung) für Cross-Family.

### P7 — Kopplung & Transition
- **P7.1** sparse Paar-**Kopplungsmatrix** über Kanäle (Onsager/Seebeck etc.); Kopplungsterme
  zwischen Zellen, nicht in ihnen.
- **P7.2** **Tri-State-Übergangssemantik** erklären (Trigger, Hysterese); Tri-State pro
  (Kanal×Operator) bei Mehrfach-Operatoren.

### P8 — Gates & Tests
- **P8.1** 0-Kanon-/Gate-Fixtures: verboten sind `[f32; 9]`, `/9.0`, `const N=9`,
  `unwrap_or(0.0)`; in `src/gate/commit_gate_vocab.json` + Test im selben Atom.
- **P8.2** Einheiten-`q·𝒯`-Ableitung testen; Reduktion über aktive Kanäle testen.
- **P8.3** `cargo check` grün; die CI trägt den Rest.

### P9 — Register & Kanon
- **P9.1** `phi/sources.φ`-Descriptor-Direktiven je Quelle (Mountain) + Parse (River).
- **P9.2** Der Record bleibt das *Gesicht* (26×f64) — die Zahl **im** Record wird zur deklarierten
  Kanalzahl; `color_index`/`force_type`-Semantik neu binden.

### P10 — `phi`-Register: `force` → Quantity | Mechanism | Medium (Register-Physik)
Gemessen 2026-10-09 (`state/stimmen/2026-10-09-river-register-physik.md`): die `force`-Spalte
in `phi/sources.φ` (7972 `field`-Zeilen; em 6088 …) trägt **Kategorie-Etiketten**, keinen
Ausbreitungsweg. Die Idee trägt, die Materialisierung ist falsch.
- **P10.1 (erster, autonomer Schritt — der lesende Lint):** ein **rein lesendes** Werkzeug über
  `phi/sources.φ` (und die weiteren `field`/`force`-Register `declined_sources.φ`,
  `pipeline/ledger.φ`, `pipeline/library.φ`, `witnesses.φ`, `bindings/bands.φ`), **ohne Formatänderung**.
  Es gruppiert `force × kernel`, zählt, nennt je Gruppe 2–3 Beispielzeilen und meldet
  Geometrie-/Quellparameter (Tiefe, Distanz, Höhe, Magnitude) sowie die `em`/`electric`-Überlappung —
  die **Messung**, woher die 6088 `em` stammen. Kein Verdikt, keine Schreiboperation.
- **P10.2a — Normalize-Schema (Rat-Entwurf 2026-10-09, additiv, noch nicht geschrieben):**
```
field <selector> <quantity> <kernel> <pde_type> <medium> [<interaction>] <role> <unit> <tau> [abs] [adv]
quantity <id> <quantity> <role> <medium> <unit> <tau>     # force_type=255, tritt nie in Σω ein
```
  - `force` → **n:m-Tag/FK** auf eine Lookup-Tabelle `mechanisms`; nie mehr die Reichweiten-Achse.
  - `pde_type` ∈ {elliptic, parabolic, hyperbolic, advective, mixed} — trägt den Charakter.
  - `medium` ∈ {vacuum, atmosphere, ocean, solid-earth, ionosphere, …}; `interaction` optional ∈ {gravity, em};
    `role` ∈ {primary, derived, geometry, source-parameter}.
  - **Abbildung der 9 Labels:** `em` → elliptic (quasi-statisch, In-situ) | hyperbolic (strahlend,
    **regime-abhängig**); `gravity` → elliptic (Poisson-Constraint); `acoustic`/`seismic-body`/
    `seismic-surface` → hyperbolic; `thermal` → parabolic; `diffusion` → parabolic; `advective` →
    advective; `electric` → elliptic (⊆ em quasi-statisch).
  - **Zulässigkeitsrelation (Regel, nicht Liste):** Flux-Operatoren koppeln an ihre Erhaltungsgröße —
    Fick↔Masse, Fourier↔Energie, Ohm↔Ladung, Newton-viskos↔Impuls; legal nur, wo `operator × medium`
    einen definierten konstitutiven Tensor hat. Kopplungsterme (Onsager: Thermodiffusion, Seebeck,
    Lorentz) liegen **zwischen** Kanälen (sparse Paar-Matrix), nicht in ihnen.
  - **Migrationsregel je top-Gruppe:** Geometrie/Quellparameter (Tiefe, Distanz, Höhe, Magnitude) →
    `quantity role=geometry|source-parameter`; In-situ-Zustand (`electron_density`, `field_intensity`,
    `density`, `humidity`, `temperature`, `pressure`) → `quantity role=primary` mit statischem
    `pde_type`; echte Propagatoren (wind/CME-`speed` → advective) bleiben mit `pde_type`.
  - **Reihenfolge:** `gravity`(118)/`seismic`(41)+Wellenhöhen zuerst, dann `acoustic`/`diffusion`,
    **`em`(6012) zuletzt** als reine Umbenennung.
  - **Gate/Fixture:** kein `force`-Feld ohne `quantity`; eine `quantity`-Zeile trägt keine Σω-Wirkung.
  - **Risse (getragen, nicht geglättet):** Moden-Partition (`Σ Pᵢ=1`); Kopplung; Einheiten folgen;
    `gravity` = Constraint.
- **P10.2 (Mountain-Domäne, nach Rat + Operator-Wort):** **Zulässigkeits-/Normalisierungsschema**:
  getrennte Achsen **Quantity** (Was/Kind: primär/abgeleitet/Geometrie/Quellparameter) |
  **Mechanism** (`pde_type`: elliptisch/parabolisch/hyperbolisch/Advektion; optional Wechselwirkung
  gravity/em) | **Medium** (Wo); die Relation ist **1:N/n:m**. Die `force`-Spalte wird zum
  Kategorie-Label/FK; Geometrie/abgeleitet wandert auf den `quantity`-Arm (`force_type=255`).
- **P10.3** Migration nach Fehlergröße: **gravity (118) · seismic-body/-surface (41) + Wellenhöhen**
  zuerst, dann `acoustic`/`diffusion`, **`em` zuletzt** als reine Umbenennung. Jede Zeile muss
  „welche Quantity + welcher `pde_type` + welches Medium + welcher Sensor" unabhängig beantworten.
- **P10.4** Gate/Fixtures: kein `force`-Feld ohne `quantity`; `quantity`-Zeile trägt keine
  Kraft-Summen-Wirkung; `commit_gate_vocab.json`-Fixture im selben Atom.

**Wahrheits-Prinzip (bindend für alle Phasen):** kein Fabrikat, kein `[[derive(Default)]]`,
kein `unwrap_or(0.0)` für physikalische Werte, kein stilles Weglassen. Was fehlt, ist `pending`;
was sich widerspricht, ist ein **Riss** (beide Zeugenzeilen, nie geglättet); was nicht gemessen
ist, wird gemessen oder als `unverified` benannt. Die Zahl ist Kapazität, nie Ontologie.

## Erster-A-Bau-Kern (was zuerst grün sein muss)
1. **P2.2** — Reduktion auf `(Σ, W)`, Teiler = Σ aktive Gewichte (vor jedem Layout; sonst lügt jedes
   spätere Register weiter).
2. **P2.1/P3.1** — `n` + `offsets[n+1]` tragen (Layout).
3. **P1.7** — Registry ChannelId→Descriptor, Moden als **eine** Instanz; die heutigen 9 darauf abbilden.

## Riss-Inventar (offen zu tragen, nicht zu glätten)
- Moden-Energie-Doppelzählung (Σ Pᵢ=1) · Zulässigkeitsrelation statt ⊗ · Kopplungsmatrix ·
  Einheiten folgen (keine Achse) · `gravity` = elliptische Constraint · Tri-State-Transition ·
  gemeinsamer Wire ⇒ gemeinsamer Zeitschritt (Subcycling/implizit) · der Riss wandert in jede
  kanonische Enumeration.
