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
| **M1 · MODVOLC-Verteilung** | **4× einig** (GLM, DeepSeek, Qwen, Mistral): `terms PD` deckt die LANCE-Route **nicht** → **`decline-redistribution`**; die **produktspezifische** LAADS-Absage trägt über die allgemeine Politik; das veröffentlichte `modvolc_thermal.bin` **zurückziehen**; die Gate-Lücke (Existenz statt Wahrheit einer `terms`-Zeile) **verdient eine Wand**. | **Kein Widerspruch im Verdikt.** Die Risse liegen im Vollzug: Der Rückzug heilt die **Zukunft**, nicht die **Vergangenheit** (GLM: „bereits geladene Kopien sind nicht revidierbar"); der **Derivat**-Status ist selbst messpflichtig (GLM/Mistral: „Transformation wäscht keine Lizenz"); der **Kanal-Konflikt** (LANCE vs. LAADS, derselbe Produzent) hat keine Haus-Regel (Mistral). |
| **M2 · dropped-gate** | **4× einig** (GLM, DeepSeek, Qwen, Mistral): **Option C** — Ledger bauen **und** Baseline frieren — aber **sequentiell**: erst das Ledger und die Einzel-Auflösung, **dann** der Freeze; **Operator/Rat** friert, **nicht** CI. | **Kein Widerspruch im Verdikt.** Die Risse: die Baseline selbst trägt **89 ungeprüfte** Keys (GLM: „niemand weiß, ob Legacy-Lücke oder Legacy-*Beleglose*"); die **CI/lokal**-Diskrepanz **11 vs. 6** ist selbst ein Mess-Riss (Sensory: Korrelat ≠ Erleben); `--dropped` **konfundiert** zwei Messungen in eins (alle vier). |

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

## Was die Linien damit tun (Vorschlag — der Akt bleibt bei ihnen)

- **Sensory (M1):** das veröffentlichte `modvolc_thermal.bin` **zurückziehen** (Operator-Hand am CDN);
  die `terms`-Zeile der LANCE-Route auf **`decline-redistribution`** / `local-only` setzen; die
  LAADS-Absage als `source_ref` registrieren (mit Ticket/Dokument, nicht bloß als Notiz). Die
  Gate-Lücke (`license_census`: Existenz ≠ Wahrheit) als **Rat-getragene Wand** benennen.
- **Mycelium (M2):** `state/zustand/ledger/` anlegen; die 6 lokalen / 11 CI-Drops einzeln reconcilen
  (3× `m10` → Mountain, 2× `mcpherron` → River, 4× φ-Blöcke → Mycelium); `--dropped` in **zwei**
  Messungen splitten; **danach** den Baseline-Freeze dem Operator-/Rat-Wort vorlegen. `dropped-gate`
  **rot lassen**, bis Belege oder offene Lücken sauber registriert sind.
