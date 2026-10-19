# Medium-Kinds des Kanal-Deskriptors — Rat + Roster (2026-10-12, river-200)

Operator-Wort (2026-10-12): „warum sollte ich die vokabeln im kopf haben … wenn nicht dann
recherchier rat und roster". Vorarbeit: `archive_search --all` (Forschungslandschaft
`state/river/medium-kind-all-2026-10-12.txt`, 255 Zeilen); Rat (fünf Stimmen); Roster (UI-Seats).

## Die Frage

Der Kanal-Deskriptor `channel <conserved>:<role>:<op>:<pde_type>:<medium>:<domain>:<boundary> <unit>`
trägt `medium ∈ {Vacuum, Fluid, ElasticSolid}` (`src/mathematikerin/channel.rs`). Die §D-Zeugen
des Ionosphären-Antwortkanals (Riometer · Ionosonde foF2 · ISR Ne/Te/Ti · GNSS-TEC · ELF/VLF ·
Airglow/FPI) liegen in Plasma/Ionosphäre — es gibt keine passende Klasse. Soll `Plasma` eine
eigene Klasse sein, oder deckt `Fluid` das ab?

## Das Rat-Verdikt (fünf Stimmen, einstimmig)

**`{Vacuum, Fluid, ElasticSolid, Plasma}`** — `Plasma` wird additiv ergänzt, sonst nichts.

- **Vacuum** bleibt (gatet Maxwell/Poisson; `c` ist Naturkonstante).
- **Fluid** bleibt und **deckt die neutrale Atmosphäre** — acoustic-gravity sind **Moden** eines
  Fluids unter Schichtung+Gravitation, keine eigene Tensorform (konsistent „Moden sind
  Projektionen, keine Achsen").
- **ElasticSolid** bleibt (elastische Wave-Zweige).
- **Plasma** neu: eigene Modenfamilie (MHD: Alfvén, fast/slow magnetosonisch), eigener
  konstitutiver Tensor (gyrotropes ε(ω,k; nₑ,B₀,ν), anisotrop durch B), dispersives Regime
  (Plasmafrequenz, Refraktionsindex < 1). `Fluid`+`Maxwell` kann das nicht legal tragen.

**Konsequenzen (kein Edit, nur Verdikt):** `Flux(Ohm)`+`Advective` müssen `Plasma` zusätzlich
zulassen (Sonnenwind, FAC); `Flux(Fourier)`+`Wave`/`Maxwell` in `Plasma` legalisieren
(Spitzer-Wärmeleitung; MHD-Welle; em-Welle im dispersiven Medium). Magnetisierung ist **kein**
zweiter Klasse-Wert, sondern das B-Datum.

**Die Unterfragen:**
1. Plasma eigene Klasse? — **Ja.**
2. Ionosphäre/Thermosphäre eigenes Medium? — **Nein**; benannter Region-Wert (Datum) innerhalb
   Plasma (ionisiert) bzw. Fluid (neutrale Thermosphäre). Nie im `medium`-Token.
3. Klasse vs. Region vs. domain? — **Drei verschiedene Dinge:** Klasse (Tensor) · Region (Datum) ·
   domain (Geometrie, die `k_j^M`-Achse).
4. Deckt Fluid die neutrale Atmosphäre? — **Ja.**

## Der Roster (UI-Seats, read-only)

| Seat | (1) Klassen | (3) Riss |
|---|---|---|
| **Qwen3.7-Plus** | `{Vacuum, Fluid, Plasma, ElasticSolid}`; Region als topologisches Datum | Teilweise ionisiertes Gemisch: `Plasma` und `Fluid` koexistieren in derselben Region, gekoppelt über Ionen-Neutral-Reibung (ambipolare Diffusion); die Ontologie zerreißt, wenn sie die Region zum homogenen Medium macht. |
| **Claude Sonnet 5.5 Extra hoch** | `{Vacuum, Fluid, ElasticSolid, Plasma}`; Ionosphäre = Region-Datum (D/E/F mit nₑ, ν, B₀) | Ein skalares `medium` pro Deskriptor trägt den Ionosphärenkanal nicht (schwach ionisiert, ν_en/ν_in gekoppelt). **Zwei offene Entscheidungen:** (a) **Regime** — Parameterfeld der Plasma-Klasse oder Unterklassen? (b) **Granularität** — `medium` pro Kanal oder pro **Pfadsegment** (Vakuum→Plasma→Neutralgas→Boden, z.B. ELF/VLF-Wellenleiter, GNSS-Strecken); pro Segment ist physikalisch ehrlicher, kostet ein Segment-Modell. Zwei Fäden sehen verschiedene Komponenten: Airglow/FPI das Neutralfluid, Riometer-Absorption das Plasma. |
| **DeepSeek V4 Pro** (tryingopen) | `{Vacuum, Fluid, ElasticSolid, Plasma}`; Ionosphäre Region/Datum | Plasma ist physikalisch auch ein Fluid; die disjunkte Aufteilung bricht an der teilweise ionisierten, kollisionsdominierten Unterkante (D-/E-Region, Neutralwind-Drag): ein Kanal braucht gleichzeitig Fluid- und Plasma-Parameter → Misch-/Mehrkomponenten-Regel. |

**`pending` (gemessener Grund, keine stille Null):** Z.ai/GLM-5.3 Deep Think Max — „Model is
currently at capacity … Currently in peak hours" (keine Antwort); Duck.ai — keine verwertbare
Antwort.

## Der konvergente Stand

**Rat + Roster einig:** `Plasma` ist eine **eigene Medium-Klasse**; **Ionosphäre/Thermosphäre sind
Region-Datums-Werte**, nie Klasse; `domain` bleibt Geometrie.

**Getragener Riss (drei unabhängige Stimmen, nicht geglättet):** die Ionosphäre ist ein **teilweise
ionisiertes Gemisch** — `Plasma` und `Fluid` koexistieren und sind über **Ionen-Neutral-Kopplung**
(ambipolare Diffusion, Pedersen-/Hall-Leitfähigkeit, Neutralwind-Dynamo) verbunden; ein **skalares
`medium` pro Kanal** reicht nicht. Zwei offene Entscheidungen (Claude): (a) Regime als
Parameterfeld vs. Unterklassen; (b) `medium` pro Kanal vs. **pro Pfadsegment**.

## Die nächste Messung (schließt das Verdikt)

Eine **gemessene Region-Parameter-Tabelle** keyt Region → konstitutive/geometrische Daten:
**IRI-2020** (Ne, Te, Ti, ionische Zusammensetzung vs. Höhe) · **NRLMSISE-00** (Neutraldichte/
-temperatur) · **IGRF** (B). Form wie `media_params.tsv`, ein Key je Region mit SI-Werten + einem
`measured`-Stempel. Dann: `archive_search` auf IRI/IGRF → Registrierung in `phi/sources.φ`
(Mountain) → CDN (Mycelium) → Parser-Arm `Medium::parse("plasma")` + Zulässigkeits-Erweiterung +
Region-Datum (River, bounded dispatch nach Operator-Wort — Wire/Kontrakt-berührend).

**Kleinster Schritt, der den Riss sofort schließt:** die §D-Fäden mit Deskriptor `…:plasma:…`
schreiben und die Region als **Datum** anhängen — die Klasse ist entschieden (`Plasma`), die Region
bleibt `pending`, bis IRI/NRLMSISE/IGRF die Zahl liefern.
