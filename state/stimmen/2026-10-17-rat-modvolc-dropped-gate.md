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

- **Fugu Max** (9156 Zeichen, vollständig, `state/stimmen/2026-10-17_rat2_fugu.json.txt`):
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
