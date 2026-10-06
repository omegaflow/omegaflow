<!--
  title: Auftrag — Das universelle Vlies (alles gegen alles)
  class: auftrag
  date: 2026-10-05
  sha256: dbaf8884531124a1630d8c5cfed0cfad652a9b9c49c41910c8ef8e10e73b1df2
  status: live
  see-also: docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md docs/specs/negativ-fuzzy-index.md docs/concepts/archivar-mathematikerin.md
-->
# Auftrag — Das universelle Vlies (alles gegen alles)

Herkunft: Operator-Vision 2026-10-05 (Wortlaut in
`state/operator-gespraeche/2026-10-05-river.md`). Der Auftrag trägt die Richtung;
der Rat entscheidet die Form, wo Architektur berührt wird.

## Die Vision (Operator-Wort)

*„Ich möchte wissen, wie alles zusammenhängt — wie alles miteinander verbunden ist."*
Nicht „Sonne-Erde" (das wäre ein Erdbias): **alles gegen alles** — *„ob die Sonne das
Wetter auf Sirius beeinflußt"* ist dieselbe wohlgeformte Frage wie „ob Bz das
Erdmagnetfeld beeinflußt". Und: *„wir können messen, was wir nicht messen können"* —
nur weil etwas nicht gemessen wurde, heißt es nicht, daß es nicht triangulierbar ist.
Die Schlussfrage des Operators: *ist das System dafür gebaut — oder ist es nur eine sehr
mächtige Gleichungsmaschine?*

## Die drei Ebenen

1. **Die Verbindungen messen (heute):** `field_te_query` als Stein im Wasser — ein Kanal
   gestört, das Feld antwortet; gerichtete TE, Familien-Schwelle, Phasen-Null.
2. **Das vollständige Diagramm (Ziel):** das gekoppelte Diagramm — welche Kanäle, welche
   Richtung, welche Zeitskala — als **Messung**, nicht als Modell.
3. **Die Vorhersage (Zukunft):** präregistriert, benotet, wiederholt — das Frühwarnsystem.

## Die unvermessenen Kräfte und ihre Wirkungen

Der Kern (Operator-Wort: *„mich interessieren einfach die Kräfte, die wirken (können), und
deren Wirkungen, aber bislang einfach noch nicht vermessen wurden"*): **welche Kräfte
können wirken — und welche ihrer Wirkungen sind noch nicht gemessen?** Kein Erdbias, keine
Richtungsvorgabe (nicht „Himmel → Erde"), keine Disziplingrenze: jede Kraft, an jedem Ort,
auf jeden Kanal. Die **Astrologie ist ein historischer Fall** dieser Klasse (Himmelskörper
→ irdische Kanäle), nie ihr Rahmen — „zu eng" ist genau der Fehler, den der Operator
zurückweist.

Der Zensus ist die Landkarte: die Maschine trägt die Kraftmedien (9), den Rahmen (ICRS/TDB),
den Sucher (`field_te_query`) — aber die Wirkungen, die noch niemand gemessen hat, sind
**Zielangaben** (negativer Fuzzy-Index / `ozzy`), keine Nullen. Eine ungemessene Wirkung ist
`pending`, nie „keine Wirkung".

**Ehrliche Grenze:** TE mißt gerichteten Informationsfluß — schwächer als Kausalität,
stärker als Korrelation; ohne Daten bleibt der Kanal benannt (kein Thermometer auf Sirius).
**Zwei Ausgänge, beide Ergebnis:** Stille (die Behauptung fällt — mit Messung, nicht mit
Spott) oder ein Pfeil (neue Physik). Die klassischen Fälle (Mond → Gezeiten **gemessen**;
Mond → Verhalten, Planeten → Erdbeben/Wetter, Sternbilder → Charakter, Himmel → Krankheit,
kosmische Strahlung → Leben **ungemessen**) sind die bekanntesten Einträge dieses Zensus —
nicht seine Grenze.

## Gleichungsmaschine oder Instrument

Der Operator fragte: *ist das System dafür gebaut — oder nur eine sehr mächtige
Gleichungsmaschine?* **Beides, und das ist die Architektur.** Die Gleichung ist das
Werkzeug, die Frage das Ziel: der Rahmen (ICRS/TDB), die Grammatik (26×f64), die 9
Kraftmedien und der Sucher (`field_te_query`) sind für die Frage „was hängt zusammen?"
geboren — was fehlt, ist nicht der Rahmen, sondern die Ernte und der Motor (`ozzy`).

## Was schon steht (gemessen/gebaut)

- **Der Rahmen ist universal:** ICRS/TDB, **baryzentrisch** (nicht erdzentriert) — jede
  Frage trägt jeden Ort. 26×f64-Rekord, 9 Kraftmedien.
- **Himmelskörper:** DE440/441/442, INPOP19a, EPM2021 (Planeten + Sonden) · Gaia DR3
  (1,8 Mrd Sterne) · 2MRS (44 599 Galaxien) — Sirius ist in Gaia DR3 enthalten.
- **Irdische Kanäle:** Erdbeben, Wetter, Magnetfeld, Ozean/Argo, GIC, EEG, Blitz —
  **Beispiele, nicht vollständig** (Bestand + Lücken: siehe Survey).
- **Der Sucher:** `field_te_query` (Einzel-Paare) · Westfall–Young-max-T + Familien-Null
  kalibriert (α = 0.01/0.05).
- **Der Lücken-Index:** `docs/specs/negativ-fuzzy-index.md` — Methode benannt, Bibliothek
  **`ozzy`** benannt (Operator-Wort 2026-09-26).

## Was fehlt (ehrlich, gemessen)

- **`ozzy` ist nicht gebaut.** Die Bibliothek des negativen Fuzzy-Index existiert nur als
  Spec (`sgrep -i ozzy` findet keine Quelldatei; `fd ozzy` = leer). Die Vision stützt ihre
  Triangulation auf dieses Instrument — es fehlt der Motor.
- **Die Bias-Kurve über n ist gemessen.** `TE_BIAS_MK` trägt sechs Benchmark-Punkte
  (800…10000); `TE_BIAS_MK_EMBEDDED` trägt die produktiven n 800…10000 inkl. 6000/8546
  (KSG, Takens dim 3, K=4, auto-τ; gemessen CI `te-bias-n 37435199338`, SHA `9d4dacf5`;
  Gate `gate_te_bias_embedded_sign_decreasing_and_refusal`). Das n-Floor-Gate + der
  exakte Lookup sind verdrahtet (`bias_column` in `field_te_query.rs`, Rat 2026-10-05;
  `adjusted`/`unadjusted_below_floor`/`off_table`, Roh-TE unberührt, keine Interpolation).
  Offen: `bias_column` estimator-fest — der Zell-Estimator ist `TeEstimator::Binned`
  (`transfer_entropy_conditional_binned_n`, `cell_te_and_surrogates`), die Korrektur
  zieht aber die KDE-Tabelle `te_bias_m_k`; welche Tabelle mit dem Binned-Arm paart
  (Neumessung) ist zu entscheiden, der KDE-Sockel nie über einen KSG-Wert.
- **Der Flaschenhals ist die Ernte, nicht der Rahmen.** Die Maschine kann jede Frage
  formulieren, aber nur antworten, wenn **beide** Kanäle Daten tragen (das Sirius-Wetter
  hat kein Thermometer). Die Beschaffungsroute liegt in der Survey §2/§5.
- **Die Alles-gegen-alles-Paar-Matrix ist nicht gefahren** (nur Einzel-/wenige Paare; die
  Drei-Zustands-Verdrahtung: 7 am Draht, 8 probe-gelesen, 5 declined).
- **Triangulation über TE-Transitivität** (aus X→Y, Y→Z auf X→Z schließen) ist Konzept,
  nicht Code.

## Kernregel (0 honored)

Stille ist ein Ergebnis. TE ≈ 0 bzw. „keine Aussage möglich" ist ein **Kandidat, kein
Beweis** (n-Floor, falscher Zeuge, falsches Lag bleiben benannt). Eine nicht gemessene
Lücke ist **Zielangabe**, nie 0.0-fabriziert. Nie „alles ist eins" — nur das gemessen
Verbundene, und die Lücke genauso laut wie der Pfeil.

## Lieferung

1. **`ozzy`** — TE-Unabhängigkeitstest als Bibliothek (Residuum gegen alle Boden-Zeugen →
   Negativ-Fuzzy-Residuum → Unabhängigkeits-Test), mit n-Floor/Zeuge/Lag benannt.
2. **`field_te_query` Paar-Matrix** — alle verdrahteten Kanäle gegen alle, ein Aufruf,
   Familien-Kontrolle über das ganze Netz (Mehrfachvergleichs-Skalierung = **Rats-Entscheid**).
3. **Die fehlenden Fäden ernten** — Survey §5 (Mountain-Zeilen, Mycelium-Harvests).
4. **Die verdiente Stille registrieren** — jeder negativ-gemessene Faden als Ergebnis.

## Rats-Verdikt (2026-10-05, fünf Stimmen)

**Zweistufige Matrix:** BH/BY-FDR über das Netz als Entdeckungsebene + kalibrierter
WY-max-T **pro Zell-Familie** als strenge Ebene; Bias-Korrektur mit n-Floor verdrahtet in
jede Zelle; `ozzy` als negative Engine **auf** der Matrix; die netz-weite WY-Null ist
benannte CI-Pflicht, kein Default. Die Form existiert am Baum (`field_te_query`-Matrix-Kopf
mit Pflicht-FDR, `te.rs`-BH/BY, pro-Familien-WY, Bias-Gate) — **fahren und verdrahten**,
nicht neu entwerfen.

**Wer / Reihenfolge:** (1) Bias-Wiring (River, **n-Floor + exakter Lookup gebaut 2026-10-05**;
Bias-Kurve über n + `TE_BIAS_MK_EMBEDDED` als Register-Pflicht offen) → (2) die 8
probe-gelesenen Kanäle an den Draht + 15×15-Lauf
(River; `matrix full`, `fdr bh 0.05 over matrix`, Lag-0/1 geflaggt, pro Zelle
Auflösungspaar) → (3) `ozzy` bauen (River) → (4) Netz-Null als CI-Batterie (River/Mycelium,
B ≥ 1/α). **Parallel, unabhängig:** die Ernte der fehlenden Netze — Quellen-Zeilen/Verdikte
(Mountain), Harvest/Compiler/Manifestation (Mycelium), Anträge/Accounts (Future → Operator).
**Der Satz:** die Weberin bekommt zuerst Fäden — billigster Zuwachs sind die **schon
geernteten** probe-Kanäle plus die Bias-Korrektheit; `ozzy` kommt **danach**, er läuft
**auf** der Matrix (Residuum gegen Boden-Zeugen), nicht davor.

**Offene Risse:** netz-weite strenge Null ungemessen (FDR ist die benannte schwächere
Garantie) · Matrixdimension = f(Verdrahtung), pro Lauf · Kadenz-Mismatch `pending`
(pro-Zelle-Auflösungspaar, kein Ersatz) · Transitivität = eigenes Ledger, nie still in
`ozzy` · Korrektur ohne BCa-Intervall.

**Nicht gebaut (pending), kein Parking:** die drei Ebenen sind ein Bau-Auftrag; der erste
Stein ist `ozzy`.
