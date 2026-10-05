<!--
  title: Auftrag — Das universelle Vlies (alles gegen alles)
  class: auftrag
  date: 2026-10-05
  sha256: dd133266b34f7f90de9023c34ac4241be96ab5ebcb9488e8c3ab593a5f34e3e0
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

## Die Astrologie-Prüfung (eigener Teil der Vision)

Die uralte Behauptung — *„der Himmel beeinflußt die Erde"* — wurde verlacht, aber nie
vollständig gemessen. Der Operator will sie **messen, nicht spotten**: die Himmelskörper
gegen die irdischen Kanäle, mit Familien-Schwellen, Phasen-Nullen, Präregistrierung.
Das ist ein **benannter Teil** des universellen Vlieses (nicht das Ganze — „Himmel gegen
Erde" ist der erdbias-behaftete Ausschnitt; das Vlies weitet ihn auf „alles gegen alles").

Die Behauptungen, die auf dem Prüfstand stehen: Mond → Gezeiten (**gemessen**, gravitativ)
· Mond → Verhalten · Planeten → Erdbeben · Planeten → Wetter · Sternbilder → Charakter ·
Himmel → Krankheit · kosmische Strahlung → Leben. Die Maschine trägt bereits beide Seiten
(Himmelskörper: DE/INPOP/EPM + Gaia DR3 + 2MRS; irdische Kanäle: Erdbeben, Wetter, Magnet,
Ozean, GIC, EEG, Blitz).

**Ehrliche Grenze:** die Astrologie behauptete **Kausalität**; das Vlies mißt **gerichteten
Informationsfluß** (schwächer als Kausalität, stärker als Korrelation). **Zwei Ausgänge,
beide Ergebnis:** überwiegend Stille → die Astrologie ist **mit Messung** beerdigt (das
größte Negativ-Ergebnis); ein verbleibender Pfeil → neue Physik.

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
- **Die Bias-Kurve über n ist nicht gemessen.** `TE_BIAS_MK` trägt sechs Benchmark-Punkte
  (800…10000); produktive Paar-n (z. B. 8546) fallen dazwischen. Das n-Floor-Gate + der
  exakte Lookup sind verdrahtet (`bias_column` in `field_te_query.rs`, Rat 2026-10-05;
  `adjusted`/`unadjusted_below_floor`/`off_table`, Roh-TE unberührt, keine Interpolation).
  Offen: `te_bias_n_probe` über den produktiven n-Bereich erweitern + `TE_BIAS_MK_EMBEDDED`
  messen — Register-Pflicht.
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
