# Plasma-Regime + Medium-Granularität — Rat + Roster (2026-10-12, river-200, zweite Runde)

Operator-Wort (2026-10-12): „dann noch mal recherche roster rat". Frage an Rat + Roster:
die zwei offenen Entscheidungen aus der Medium-Kinds-Runde.
Vorarbeit: `archive_search --all` (`state/river/plasma-regime-granularity-all-2026-10-12.txt`, 250 Zeilen).

## Die zwei Fragen

**(a) Plasma-Regime** (kollisionsfrei ↔ kollisionsdominiert ↔ MHD; gesteuert durch `ν/ω`, `ω_c/ω`,
`ω_p/ω`): Parameterfeld **innerhalb** der `Plasma`-Klasse, oder **Unterklassen**?

**(b) Medium-Granularität**: `medium` **pro Kanal**, oder **pro Pfadsegment** (Kette Vakuum→Plasma→
Neutralgas→Boden; ELF/VLF-Wellenleiter, GNSS)?

## Die Verdikte (Rat + Roster, einig)

### (a) → **Parameterfeld innerhalb der `Plasma`-Klasse**, keine Unterklassen
- Der konstitutive Tensor ist **stetig** in `(nₑ, B₀, ν)`; eine Unterklasse ist eine binäre Grenze in
  einem stetigen Raum = **Fabrikation** (A=A). Landschaft: „transition from collisionless to
  collisional MHD", Knudsen-Zahl/Hall-Parameter als Verhältnisse.
- Regime gehört zum Tripel **(Medium, ω, Ort)** — dasselbe Plasmavolumen ist bei `ω₁`
  kollisionsfrei, bei `ω₂` kollisionsdominiert; entschieden **pro Mode** (`mode_regime`, P2.3).
  Appleton–Hartree deckt das Kontinuum mit `X=ω_p²/ω², Y=ω_c/ω, Z=ν/ω` einheitlich.
- **Präzedenz im Code:** Materialdaten reiten als Datum (`body`, `media_params_of`) und **nicht** in
  `hash`/`PartialEq` — genau so reiten `nₑ,B₀,ν,T,Z,Ionisationsgrad`. Ein Regime-Subclass wäre ein
  fünfter diskreter Schnitt auf einem stetigen Feld; `hash` hinge sonst an `nₑ`, das sekündlich driftet.

### (b) → **pro Pfadsegment**, als **Kette von Kanal-Deskriptoren** (Query entlang der Weltlinie), **nicht** als Segment-Vektor im Deskriptor
- Ein Medium = ein konstitutiver Tensor = **eine** Kanal-Identität. Ein Pfad durch mehrere Medien ist
  eine **Folge** von Kanälen, gekoppelt an den Grenzflächen (die `Boundary`-Achse als Paar-Relation).
- Der gefürchtete Preis ist vermeidbar: der Frame trägt bereits `n` Kanäle + `offsets[n+1]` (P3.1);
  ein Pfad aus `k` Segmenten sind `k` Kanäle. Der Deskriptor ist der **Typ an diesem Query-Punkt**,
  kein wachsender Vektor → **kein `hash`-Zuwachs, kein Wire-Bruch**.
- **Der getragene Riss der Vorrunde löst sich hier:** die teilionisierte Ionosphäre ist kein
  gemischtes Medium, sondern ein `Plasma`-Kanal **und** ein `Fluid`-Kanal in derselben Region,
  gekoppelt über Ionen-Neutral-Reibung (ambipolare Diffusion, Pedersen-/Hall-Leitfähigkeit); die
  Kopplung gehört in die **sparse Paar-Matrix** (P7.1, `kanal-ontologie:142`, `pending`), nicht in
  einen dritten `medium`-Wert.

## Die Quellen (Roster)

| Seat | (a) | (b) | eigener Riss |
|---|---|---|---|
| Qwen3.7-Plus | Parameterfeld (Zustandsobjekt) | pro Pfadsegment (Kanal = geordnete Kette) | Kopplungsoperatoren an den Schnittstellen (Reflexions-/Transmissions-/Modenkonversionsmatrizen), besonders bei verschmierten/dynamischen Grenzen |
| Claude Sonnet 5.5 Extra | Parameterfeld; Regime gehört zum Tripel (Medium, ω, Ort) | pro Pfadsegment, Region = Quelle der Wahrheit | **ν gehört der Plasma-Neutral-Kopplung**, nicht dem Plasma (`ν_en`/`ν_in` hängen am co-lokalisierten Neutralfluid) → Parameterfeld hat eine Referenz nach außen; **Segmentierungsachse**: der ELF/VLF-Wellenleiter ist **quer** zur Ausbreitung geschichtet (Moden gehören dem ganzen Stapel); Strahl/Moden/Vollwelle hängt von λ vs. Gradientenlänge ab; kontinuierliche Profile haben keine natürliche Segmentgrenze |
| DeepSeek V4 Pro (tryingopen) | Parameterfeld / berechnetes Regime-Attribut | pro Pfadsegment (geordnete Kette) | Dispatch auf Dispersions-/Stoßmodelle in Misch-/Übergangsregimen (`≈1`); Segmentierungsregel vs. Region-Daten-Sync |
| Z.ai GLM-5.3 Deep Think Max | `pending` („at capacity / peak hours", keine Antwort) | — | — |

## Getragene Risse (nicht geglättet)

1. **Teilionisiertes Gemisch** — `Plasma` ⊕ `Fluid` in derselben Region, Ion-Neutral-Kopplung in
   P7.1 (`pending`, nicht gebaut). Beide Zeugenzeilen stehen, keine wird gemittelt.
2. **`ν` hat eine Referenz nach außen** (Claude) — `ν_en`/`ν_in` hängen an der Neutraldichte des
   co-lokalisierten Fluid; das Parameterfeld ist nicht geschlossen. Entweder `ν` als Region-Datum
   oder abgeleitet aus einem `Coupling(Plasma, Fluid)`-Objekt.
3. **Entartungsgrenzen / Klassenschluss** — die Bedingung, an der `ε` seine Tensorform wechselt
   (isotrop ↔ gyrotrop: `B₀→0`, `ν→∞`, Ionisationsgrad→0), ist nicht deklariert; `Plasma` ist nach
   unten nicht geschlossen (`pending`).
4. **Kinetische Schließung** — Vlasov/Landau ist in `TransportOp {Wave, Maxwell}` nicht darstellbar;
   für die kollisionsdominierte Ionosphäre trägt der Fluid-Operator, die kinetische Klasse bleibt
   `pending`. Jedes Modell braucht einen expliziten Gültigkeitsbereich.
5. **Namenskollision „Regime"** — `Regime` (`channel.rs:187`) trägt das **PDE**-Regime
   {QuasiStatic, Radiating, Diffusive, Advective, Constraint}; das **Plasma**-Regime
   {`ν/ω, ω_c/ω, ω_p/ω`} ist etwas anderes. Zwei Dinge unter einem Wort = Fabrikationsquelle; der
   Namensschnitt ist offen.
6. **Grenzflächen-Paar-Relation** — der Deskriptor trägt *einen* `boundary`; die Grenzfläche
   zwischen zwei Segmenten ist eine Paar-Relation, die es noch nicht gibt (P7.1, `pending`).
7. **Segmentierungsachse** (Claude) — Strahl/Moden/Vollwelle hängt von λ vs. Gradientenlänge ab;
   kontinuierliche Profile haben keine natürliche Segmentgrenze → die Diskretisierung ist selbst
   eine Modellwahl.

## Die nächste Messung

- **(a):** eine **gemessene Regime-Parameter-Tabelle** in `media_params`-Form (Key → SI-Werte +
  `measured`): **IRI-2020** (`nₑ,Tₑ,Tᵢ`) · **NRLMSISE-00** (Neutraldichte/-temperatur → `ν`) ·
  **IGRF** (`B₀`). **Schließen:** zwei verschiedene Parameterzeilen (D-Region vs. F-Region) auf
  **denselben** `Plasma`-Deskriptor-Hash abbilden und nachweisen, dass der Hash unverändert bleibt.
- **(b):** eine **gearbeitete Segmentkette** eines §D-Fadens (ELF/VLF Blitz→D-Region→Wellenleiter→
  Boden, oder GNSS Ionosphäre→Troposphäre): je Segment die Medium-Klasse + `boundary`, je
  Grenzfläche die Bedingung, und der Nachweis, dass kein neues Deskriptorfeld nötig ist (die Kette
  läuft über die `n`-Kanal-`offsets` + die P7.1-Kopplungszeilen).

**Kleinster Schritt, der beide Verdikte schließt:** die §D-Fäden mit Deskriptor schreiben, Klasse =
`plasma` (entschieden), Regime/Datum = `pending` (keine erfundene `nₑ`), Region als Datum anhängen,
und die Segmentkette + die Plasma⊕Fluid-Kopplung als P7.1-Zeilen anlegen.
