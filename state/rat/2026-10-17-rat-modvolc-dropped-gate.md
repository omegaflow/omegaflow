# Rat — zwei neue Ratsfragen (2026-10-17, zweite Runde)

**Anlass.** Zwei neue Ratsanfragen lagen im Postfach (`post/in/`): Mycelium (`dropped-gate`-Baseline,
01:33) und Sensory (MODVOLC-Verteilung, 02:09). Gefahren durch die **API-Rat-Sitze** (GLM-5.3,
DeepSeek V4 Pro, Qwen3.8-Max über DeepInfra; Mistral Large 4 über die Mistral-API) — dasselbe Haus-Design
wie die erste Runde (`2026-10-17-rat-offene-fragen.md`): fünf Axiome, fünf Stimmen, Verdikt/Begründung/Risse.

**Die Fragen verbatim:** M1 Verteilungs-Ethik (darf das Haus MODVOLC verteilen?) · M2 Register-Architektur
(der `dropped-gate`-Rotstand und die eingefrorene Baseline).

**Ablage:** Fragenbogen + Rohantworten unter `/tmp/opencode/rat-test/` (JSON je Sitz, `m-*.json`).

## Die Bilanz in zwei Zeilen (Konsens + Riss je Frage)

| Frage | Konsens | Riss (ungeglättet) |
|---|---|---|
| **M1 · MODVOLC-Verteilung** | **API-Sitze 4× einig** (GLM, DeepSeek, Qwen, Mistral): `terms PD` deckt die LANCE-Route **nicht** → **`decline-redistribution`**; Asset zurückziehen; Gate-Lücke → Wand. **Die zwei UI-Sitze überstimmen:** Fugu + Lumo sagen, `PD` ist der *falsche Wert*, aber **`decline` der falsche Schluss** — die LANCE-Route ist **offen**. **Gelöst (Sensory `e789578d3`):** nicht MODVOLC, sondern der **L1-Block `MOD021KM`** trägt die Verteilung. | **API-Sitze gegen UI-Sitze.** Die API-Sitze lasen den Support-Satz als Absage; die UI-Sitze prüften die Kanalterme. Fugu: „Terme hängen an **(Produkt, Kanal)**, nicht am Produzenten — diese Brücke existiert nicht." Lumo: „`PD` ist der falsche Wert (NASA = open, Attribution, „as is")." |
| **M2 · dropped-gate** | **4× einig** (GLM, DeepSeek, Qwen, Mistral) **und Fugu:** **Option C**, aber **sequentiell** — erst Ledger + Einzel-Auflösung, **dann** Freeze; **Operator/Rat** friert, **nicht** CI. Fugu schärft: **A, dann B** (nicht C als Einzelakt); Baseline trägt **als Zeuge**. | **Kein Widerspruch im Verdikt.** Risse: die Baseline selbst trägt **89 ungeprüfte** Keys; CI/lokal **11 vs. 6** ist selbst ein Mess-Riss; `--dropped` **konfundiert** zwei Messungen; das Ledger-Dir fehlt. Fugu: jeder Drop ohne Commit-Witness ist **`pending`**, nicht `fail`. |

**Der eine Satz (vier unabhängige Zeugen).** Alle vier sagen **getrennt**: bei beiden Fragen ist das
**Verdikt einig** — der Streit liegt allein im **Vollzug** (Reihenfolge, Vergangenheit, Kanal-Regel).
Das Haus entscheidet nicht neu, *was* richtig ist; es entscheidet, *wie* das Richtige geschieht.

**Wo eine Doktrin geschlagen ist (verbatim).**
- M1: *„Allgemeine NASA-Politik deckt alle Produkte"* — geschlagen (GLM, DeepSeek, Qwen, Mistral).
- M1: *„Öffentlich erreichbar = öffentlich verteilbar"* — geschlagen (DeepSeek).
- M1: *„Existenz einer `terms`-Zeile = Lizenzwahrheit"* — geschlagen (Qwen, Mistral).
- M2: *„Eingefroren = aktuell"* / *„Ein neuer Baseline-Freeze heilt Drops"* — geschlagen (alle vier).
- M2: *„Absent = offen"* — geschlagen (GLM, Qwen, Mistral).

---

## M1 · Verteilungs-Ethik: darf das Haus MODVOLC verteilen?

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **decline-redistribution** | „`terms PD` deckt die LANCE-Route nicht — mit offenem Beleg-Riss." Produktspezifisch trägt; Asset **zurückziehen**; **Wand ja.** Riss: der Rückzug stoppt die **Zukunft**, nicht die **Vergangenheit**; der **Derivat**-Status ist ein *Schluss*, keine Messung → als `pending` führen. Wand muss zwei Dinge prüfen (Quelle mit Kraft + produkspezifisch > allgemein) **und** das eigene Instrument kalibrieren. |
| **DeepSeek V4 Pro** | **decline-redistribution** | „Die LANCE-Route ist nicht durch `terms PD` gedeckt. Das Asset ist zurückzuziehen." Drei geschlagene Doktrinen (allgemein-deckt-alles · erreichbar=verteilbar · Lizenzzeile=Lizenz). Nächster Schritt: Asset zurückziehen, `terms` auf `pending`, produktspezifische Anfrage an LAADS **und** HIGP. |
| **Qwen3.8-Max** | **`decline-redistribution`** (hilfsweise `pending-no-redistribution`) | „Existenz ohne Quelle ist keine Freigabe." LAADS-Absage als `source_ref` registrieren; CDN sperren; `license_census` um **Quellen-/Produktstatusprüfung** erweitern; HIGP bleibt `local-only`. **Wichtigstes Zusatz-Argument:** die LAADS-Absage ist **selbst beweisbedürftig** (ohne Ticket/Dokument fragil). |
| **Mistral Large 4** | **decline-redistribution** | „Allgemeine Politik ist Rahmen, nicht Freibrief." Sofort zurückziehen (HTTP 200 → 403/404); Register auf `decline`/`local-only`; Gate um **semantische** Lizenzprüfung. Riss M1-D: **zwei Kanäle desselben Produzenten widersprechen sich** — das System hat keine Regel, welcher bei Konflikt trägt (außer produkspezifisch > allgemein). |

**Konsens M1 (vier Zeugen):**
1. **`terms PD` deckt die LANCE-Route nicht** → **`decline-redistribution`**.
2. **Die produktspezifische LAADS-Absage trägt** über die allgemeine NASA-Politik.
3. **`modvolc_thermal.bin` zurückziehen** (Operator-Hand); bis zum Entscheid nicht weiter veröffentlichen.
4. **Die Gate-Lücke verdient eine Wand** — `license_census` prüft Existenz, nicht Wahrheit.

**Offene Risse M1 (nicht geglättet):** Derivat-Status selbst messpflichtig · Rückzug heilt nur die Zukunft ·
Kanal-Konflikt LANCE/LAADS ohne Haus-Regel · LAADS-Absage selbst noch ohne Ticket/Quellenbeleg.

---

## M2 · Register-Architektur: der `dropped-gate`-Rotstand und die eingefrorene Baseline

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **C, in Reihenfolge: erst A, dann B** | „B allein ist geschlagene Doktrin." Baseline friert **ein Wort** (Operator/Rat, normativer Akt); „Lücke vs. Beleg" entscheidet die **Messung** (Commit-Witness), nicht das Werkzeug. Riss 3: die **89 gepinnten Keys** sind selbst ungeprüft. Riss 4: **Reihenfolge ist Verdikt, nicht Geschmack** — B vor A wäre Beweisvernichtung. |
| **DeepSeek V4 Pro** | **C** (Baseline frieren **und** Ledger bauen) | „Beides ist nötig, um die Lücke zu messen, nicht zu verstecken." Baseline = Messung eines **vergangenen** Zustands; als aktuell behandeln ist Fabrikation. Wer friert: Operator/Rat. Wer entscheidet „Lücke vs. Beleg": der **Commit-Witness**. |
| **Qwen3.8-Max** | **C — aber nur in harter Reihenfolge** | „Nicht B allein. Zuerst **A**: Ledger bauen und Close-Events belegen. Danach darf eine korrigierte Baseline neu gefroren werden. Ein bloßer Neu-Freeze ohne Ledger wäre **Glättung**." Operator/Rat gibt Autorität, **kann aber nicht Beleglosigkeit in Wahrheit verwandeln**. |
| **Mistral Large 4** | **C — sequentiell: erst Ledger, dann Frieren** | „Die Baseline trägt nicht mehr als alleinige Wahrheit; sie ist stale." Operator/Rat friert (nicht CI, nicht automatisch); der **Mensch** entscheidet „Lücke vs. Beleg" — „das Tool misst nur das Fehlen; die Deutung ist menschlich". Riss M2-C: CI **11** vs. lokal **6** → Riss in der **Reproduzierbarkeit**. |

**Konsens M2 (vier Zeugen):**
1. **Option C** — Ledger bauen **und** Baseline frieren.
2. **Sequentielle Reihenfolge:** erst **A** (Ledger + Einzel-Auflösung), **dann** **B** (Freeze). B vor A wäre Beweisvernichtung / Glättung.
3. **Operator/Rat friert** die Baseline — ein normativer Akt, nicht CI.
4. **Der Commit-Witness entscheidet** „Lücke vs. fehlender Beleg" — nicht das Werkzeug, nicht das Wort allein.

**Offene Risse M2 (nicht geglättet):** 89 gepinnte Keys selbst ungeprüft · CI(11)/lokal(6)-Diskrepanz ist
selbst ein Mess-Riss · `--dropped` konfundiert zwei Messungen in eins · das Ledger-Dir fehlt (der Ort, an
dem ein Witness zählen könnte).

---

## Die zwei UI-Sitze (nachgeholt 2026-10-17) — und die Überstimmung

Auf Operator-Wort nachgeholt: **Fugu Max** (Sakana, eingeloggt) und **Lumo 2.0 Max** (Proton).
Beide **überstimmen die vier API-Sitze** in M1 — und decken sich gegenseitig:

- **Fugu Max** (9156 Zeichen, vollständig, `state/rat/2026-10-17_rat2_fugu.json.txt`):
  „`terms PD` deckt die Route **nicht sauber ab** — aber MODVOLC ist auf dieser Aktenlage **kein
  `decline-redistribution`-Fall**. Die Route ist **offen**." **M1.3: nicht zurückziehen** — „ein
  Rückzug, gestützt allein auf einen Support-Satz, wäre ein Akt auf einer ungemessenen Restriktion
  — selbst ein Verstoß gegen A = A." **Sechs geschlagene Doktrinen**, u. a.: „gleicher Produzent ⇒
  gleiche Terme" (Terme hängen an **(Produkt, Kanal)**) · „ein Support-Satz setzt oder widerruft
  eine Lizenz" (Zeugnis ≠ Term-Dokument) · „Existenz einer terms-Zeile ⇒ lizenziert".
  M2: **A, dann B** (nicht C als Einzelakt); Baseline trägt **als Zeuge**, nie neu frieren, um Grün
  zu erzeugen; jeder Drop ohne Commit-Witness ist **`pending`**, nicht `fail`.
- **Lumo 2.0 Max** (Web-gestützt, während des Streams gelesen): „`terms PD` **deckt** die LANCE-Route
  (Verzeichnis 61 enthält MODVOLC) — aber **`PD` ist der falsche Wert** (NASA = open, Attribution,
  kein Endorsement, „as is" — nicht CC0). **Decline-redistribution: nein** für die LANCE-Route."
  Es trägt „keine der beiden allein, sondern die **Kanal-Bindung**." **Fachkorrektur:** LANCE liefert
  `allData/61` mit MODVOLC; **LAADS ist ein anderer Kanal** (L1/Atmosphere); die Support-Mail ist
  **kein Lizenzinstrument**.

**Die Überstimmung in einem Satz:** Die vier API-Sitze lasen den **Support-Satz** als Produkt-Absage
und schlossen `decline-redistribution`; die zwei UI-Sitze prüften die **Kanalterme** und fanden:
`PD` ist der *falsche Wert*, aber **`decline` ist der falsche Schluss** — die LANCE-Route ist offen.

## Was die Linien damit tun (Stand nach der Überstimmung)

- **Sensory (M1) — hat die Ratsfrage entschärft (Commit `e789578d3`):** statt MODVOLC (HIGP-abgeleitet)
  trägt jetzt der **L1-Block `MOD021KM`** (NASA-open, Attribution) die Verteilung — das **eigene
  Thermalanomalie-Feld aus offener L1** (Wright et al. 2003, JVGR `10.1016/j.jvolgeores.2003.12.008`)
  ist mit Attribution verteilbar (`terms free-open`, **kein** `decline-redistribution`). MODVOLC selbst
  bleibt `local-only`. **Offen:** `MOD021KM`/`MYD021KM` sind noch **nicht im Register** (0 Treffer) —
  der L1-Block + ein Band-21/22/32-Reader sind der nächste Bau.
- **Mycelium (M2):** `state/zustand/ledger/` anlegen; die 6 lokalen / 11 CI-Drops einzeln reconcilen
  (3× `m10` → Mountain, 2× `mcpherron` → River, 4× φ-Blöcke → Mycelium); `--dropped` in **zwei**
  Messungen splitten; **danach** den Baseline-Freeze dem Operator-/Rat-Wort vorlegen. `dropped-gate`
  **rot lassen**, bis Belege oder offene Lücken sauber registriert sind.

---

## 2026-10-17-modvolc-verteilung-formular.md

<!--
  title: Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik)
  class: concept
  date: 2026-10-17
  sha256: 34a5c16bf0109284a89aeefe381264e7162b6b1c3401c47982f85648dd4f7fe1
  status: live
-->
# Rat-Formular — MODVOLC-Verteilung (Verteilungs-Ethik), 2026-10-17

**Anlass (Operator-Wort):** „das ist eine **Ratsfrage**." Mountain ist dasselbe Modell mit demselben
Wissen und trägt zur Verteilungsfrage nichts Eigenes bei — die Frage gehört an den **Rat**.

**Gemessen (Belege):**
- **LANCE-Route** `nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC` — damals im Register `terms PD`
  (Block später gefallen, s. Nachtrag 2), belegt über die NASA-Datapolitik
  `earthdata.nasa.gov/engage/open-data-services-software-policies/data-information-guidance`
  („full and open sharing … all data … distributed … by NASA data systems"); Messung **mountain-340**
  (2026-10-13); Route gesetzt von **mycelium-343**.
- **HIGP-Route** `modis.higp.hawaii.edu` — **keine** offene Lizenz (© UH Mānoa, Data-Use-Agreement) →
  `local-only`; Host zudem tot (Timeout, DNS löst auf `128.171.157.89`).
- **Neu, widersprechend:** LAADS Support — „this product is **not approved for public distribution**."
- Das kompilierte `modvolc_thermal.bin` (**4 306 035 B**) lag auf dem **öffentlichen** CDN — inzwischen
  **zurückgezogen** (Asset **HTTP 404**, gemessen 2026-10-17).
- Gate-Lücke: `license_census` prüft die **Existenz** einer `terms`-Zeile, nicht ihre **Wahrheit**.

**Fragen an den Rat:**
1. Deckt `terms PD` die **LANCE**-Route — oder ist MODVOLC ein `decline-redistribution`-Fall?
2. Trägt die allgemeine **NASA-Politik** oder die **produktspezifische LAADS-Absage** (LANCE vs LAADS:
   derselbe Produzent, zwei Kanäle)?
3. Soll das veröffentlichte CDN-Asset **zurückgezogen** werden (Operator-Hand) — und wird bis zum
   Entscheid **nicht weiter** veröffentlicht?
4. Verdient die Gate-Lücke (Existenz vs. **Wahrheit** einer `terms`-Zeile) eine **Wand**?

**Nachtrag (2026-10-17) — entschärft durch die Rohdaten-Route.** Verifiziert: LANCE `allData/61` trägt
MODVOLC, **LAADS** ist das **L1/Atmosphere**-Archiv (`MOD021KM`/`MYD021KM`, 200); die Support-Mail ist
kein Lizenzinstrument; `terms PD` ist der **falsche Wert** (NASA = open/Attribution/kein Endorsement).
Der MODVOLC-Algorithmus ist **publiziert** (Wright et al., JVGR `10.1016/j.jvolgeores.2003.12.008`:
4 µm Band 21/22 + 12 µm Band 32) → das Haus kann sein **eigenes** Thermalanomalie-Feld aus der **offenen
L1** rechnen und MODVOLC nur zum **Abgleich** nutzen. **Damit bleibt die Ratsfrage nur für den einen
Fall: geben wir das MODVOLC-Derivat selbst weiter?** Für unser L1-Derivat genügt Mountains `terms`-Verdikt
(`free-open` + Attribution).

**Rat-Antwort (M1), `4f50553f5`:** **4:2 gespalten** — API-Sitze 4× `decline` (PD deckt LANCE nicht,
Asset zurückziehen); **UI-Sitze (Fugu Max, Lumo) überstimmen**: `PD` ist der falsche *Wert*, aber
`decline` der falsche *Schluss* — die **LANCE-Route ist offen**; Terme hängen an (Produkt, Kanal).
Das Haus liest es als UI-Überstimmen (`state/rat/2026-10-17-rat-modvolc-dropped-gate.md`).

**Operator-Wort (2026-10-17) — entschieden, konservativ:** „wir nehmens runter im Zweifel lieber nicht …
wir rechnen eh selber." → **MODVOLC-CDN-Asset zurückziehen** (Mycelium), Block `local-only` (kein
`terms PD`), lokal zum **Abgleich**; das **Feld** trägt der **L1-Block `MOD021KM`** (NASA-open +
Attribution). Der Spalt ist damit **durch das Operator-Wort** entschieden — kein neuer Rat nötig.

**Nachtrag 2 (gemessen, Sensory-Pass 2026-10-17):** Vollzogen: der Register-Block ist **gefallen**
(`phi/sources.φ` trägt **keine** MODVOLC-Zeile; Mycelium `796ae91c3`, „no fabricated PD line") und das
CDN-Asset ist **HTTP 404** (zurückgezogen). MODVOLC bleibt `local-only`; der Träger ist der **L1-Block
`MOD021KM`/`MYD021KM`** bei Mountain (`xfer:mycelium-361-7`; terms/field/ttl + Band-21/22/32-Reader).
Die vier API-Sitze (`decline-redistribution`) und die zwei UI-Sitze (`LANCE offen`) stehen **benannt**,
nicht geglättet; die `license_census`-Lücke (Existenz ≠ Wahrheit) bleibt **Rat-getragene Wand**.

**Speist:** die gemessene Recherche oben + der Roster `~/projects/omegaflow-work/ui-seats.md`.

