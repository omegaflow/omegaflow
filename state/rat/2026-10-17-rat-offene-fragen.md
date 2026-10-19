# Rat — die offenen Ratsfragen der Linien (2026-10-17)

**Anlass.** Vier offene Ratsanfragen lagen im Postfach (`post/in/`): River (Bindung/Eigentum),
Mycelium (exposom-Serien-Definition, register-coverage-Concurrency), Sensory (Orphan-Docs).
Gefahren durch die **API-Rat-Sitze** (GLM-5.3, DeepSeek V4 Pro über DeepInfra; Mistral Large 4
über die Mistral-API) **und** die zwei UI-Schnell-Sitze (Sakana Fugu Max, Lumo 2.0 Max) im Browser.

**Die Fragen verbatim:** R1 Bindung/Eigentum · R2 exposom-Serien-Definition · R3
register-coverage-Concurrency · R4 Orphan-Docs-Träger/Marker-Kollision.

**Ablage:** Der Fragenbogen und die Rohantworten liegen unter `/tmp/opencode/rat-test/`
(JSON je Sitz). Dies ist die lesbare Sammlung.

## Die Bilanz in vier Zeilen (Konsens + Riss je Frage)

| Frage | Konsens | Riss (ungeglättet) |
|---|---|---|
| **R1 · Bindung** | Die Vorlage trägt **nicht**; „Aufenthalt = Eigentum" ist geschlagen. **4× Feld** (GLM, DeepSeek, Mistral, Lumo). | **Feld** (GLM/Mistral/DeepSeek/Lumo) **gegen** **Quittung/Akt-Paar** (Fugu: „zwei Speicherorte für eine Tatsache heißt, einer ist fabriziert"). |
| **R2 · exposom** | **Konvention C** (3×: GLM, DeepSeek, Fugu) **gegen B** (Lumo) **gegen `pending`** (Mistral). | **Spalte wählen** (GLM/DeepSeek: `Mean_Temp`; Lumo: `exposure_index`) **gegen** **`pending` bis die 19 Spalten gemessen sind** (Mistral, Fugu). Binning: „nur Severe" (DeepSeek) **gegen** „alle Events, Severity als Attribut" (Fugu, Lumo). |
| **R3 · Concurrency** | **2 : 3 gespalten.** | **A per-SHA** (GLM, DeepSeek, Lumo) **gegen** **B per-ref** (Fugu **und** Mistral: „per-SHA adressiert die **Vergangenheit**, nicht den HEAD"). |
| **R4 · Orphan** | Der Marker-Zähler ist ein **Lärm-Signal** (Kanon-Vokabel `pending`/`offen`). | **dreifach:** Leitstelle (DeepSeek, Lumo) · abgeleitete Abfrage (Fugu) · **beides geschichtet** (GLM) · **linienlokal mit Verweigerungsvorbehalt** (Mistral). |

**Der eine Satz (drei unabhängige Zeugen).** Fugu, Lumo und GLM sagen **getrennt**: die vier Fragen sind
**eine** — eine **geborgte Adresse**. R1 Kürzel statt Akt · R2 Name statt Größe · R3 per-SHA als
geliehene Adresse · R4 Träger als gespeichertes Flag. Das Haus nennt die Adresse, wo es die Sache meint.

**Zwei einander widersprechende Zeugen für dasselbe Muster:** Mistral nennt in R3 wörtlich
*„Die Raumzeit-Adresse des HEAD ist der Ref, nicht der SHA — per-SHA adressiert die Vergangenheit"* —
dieselbe Diagnose wie Fugu, und beide wählen **B**. GLM/DeepSeek/Lumo wählen A mit derselben
Axiom-2-Sprache. Der Riss ist echt, beide Seiten sind begründet.

---

## R1 · Bindung und Eigentum im Register

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **Eigentums-Feld (Adress-Pflicht)** | „Die Vorlage trägt nicht. Eigentum ist keine Feld-Option, sondern **Adress-Pflicht**. **Nein** — eine Linie darf keinen Punkt führen, dessen nächster Akt nicht bei ihr liegt. 'Aufenthalt = Eigentum' ist als Doktrin **geschlagen**." Nächster Schritt: Feld mit drei Werten (`eigen` · `dispatcht_an:<line>` · `operator`). |
| **DeepSeek V4 Pro** | **`owner`-Feld** | „Die Vorlage vermischt zwei Register-Ebenen: den Akt (mit `channel_ref`) und die Zuständigkeit (ohne). Ein fremder Punkt als `eigen` ist eine Lücke, als Besitz deklariert (A4)." Nächster Schritt: `owner`-Feld; fremde Punkte nur als `pending`-Verweis. |
| **Sakana Fugu** | **kein Feld — eine Quittung** | „Die Vorlage ist strukturell ausreichend, **protokollarisch nicht**: sie hat Dispatch, aber keine **Quittung**. Ein eigenes Eigentums-Feld wäre die schlechtere Antwort — zwei Speicherorte für eine Tatsache heißt, einer ist fabriziert. Eigentum muss **abgeleitet** werden (Sicht über das Transfer-Log), nie unabhängig schreibbar." |
| **Mistral Large 4** | **Feld, das `channel_ref` kodiert** | „Besitz ohne Kraft ist keine Messung, sondern eine **Behauptung**. Aufenthalt ohne Kraft ist **Besetzung, nicht Besitz**. Sofortige **Entlassung** der 12 — `Kürzel river-223 löschen`." |

**Der Riss (ungeglättet).** GLM und DeepSeek wollen ein **Feld**; Fugu will **kein Feld, sondern ein
Akt-Paar** (`custody_in` beim Empfänger) + abgeleitetes Eigentum aus dem Transfer-Log. Beide Lager
stimmen: die Vorlage trägt nicht. Sie scheiden sich an der **Form** der Heilung:
gespeichertes Feld vs. abgeleitete Sicht + Quittung.

**Der von allen benannte Kern:** river-223 hat **12 ≠ 1** verletzt (ein Kürzel über 12 fremde Punkte).
Fugu: *„ohne `n` am Aggregat ist A = A nicht prüfbar."*

---

## R2 · exposom-Serien-Definition (x/y-Feed)

| Sitz | EI-Kanal | Event-Binning | Konvention |
|---|---|---|---|
| **GLM-5.3** | **`Mean_Temp`** | EI-Zeitachse, alle Events, `Severe` als eigene Zelle | **C** (mit Grund) |
| **DeepSeek V4 Pro** | **`Mean_Temp`** | EI-Achse, **nur `Severe`** | **C** |
| **Sakana Fugu** | **pending** — x wird durch den **erzeugenden Akt** benannt, nicht als Spaltenname; Kandidat mit Kraft: `Mean_Temp` | **TDB-Achse**, **alle** Ereignisse mit `Severity` als Attribut | **C geordnet** (A kanonisch, B abgeleitet, Invariantentest `B == parse(A)`) |
| **Lumo 2.0 Max** | **`exposure_index`** (explizit, nicht `Mean_Temp`/Index/Composite gemischt) | TDB-Achse, **alle Schweregrade** mit `severity-tag`, Alignment vor Aggregation | **B** (Compiler-Erzwingung verhindert implizite Transformation) |
| **Mistral Large 4** | **pending** | **pending** | **pending** — „Axiom 1 verbietet die willkürliche Wahl. Erst messen: CDN-Asset dekodieren, 19 Spalten kartieren, `channel_ref` für EI, TDB-Zeitachse." |

**Der Riss (ungeglättet).** **Fünf Sitze, vier Positionen:** GLM/DeepSeek wählen **`Mean_Temp` + C**;
Fugu **`pending` + C**; Lumo **`exposure_index` + B**; Mistral **`pending` bei allem**.
Der Konsens ist dünn: **nur bei C sind sich GLM, DeepSeek und Fugu einig** — Lumo wählt B, Mistral
verweigert. Und beim Binning stehen **„nur Severe"** (DeepSeek) gegen **„alle Events"** (Fugu, Lumo).
Mistrals Kern-Satz: *„Ein Urteil ohne Messung wäre eine Verletzung von A = A."*

---

## R3 · Concurrency-Schlüssel von `register-coverage`

| Sitz | Verdikt | Kern |
|---|---|---|
| **GLM-5.3** | **A** (per-SHA, `true`) | „Der per-SHA-Schlüssel trägt, **gerade weil** das Gate HEAD-Zustand prüft." |
| **DeepSeek V4 Pro** | **A** | „Der HEAD-Zustand ist je SHA definiert; per-ref misst einen verschwommenen Zustand." |
| **Lumo 2.0 Max** | **A** | „Per-SHA adressiert exakt diesen Körper (jeder Commit hat eigene Raumzeit)." |
| **Sakana Fugu** | **B** (per-ref, `true`) | „Der Schlüssel muss die Adresse des gemessenen Objekts sein. `register-coverage` misst einen **Ref**, nicht ein SHA-Artefakt. per-SHA wäre **Analogie per Name**, nicht Ableitung aus dem Akt." |
| **Mistral Large 4** | **B** (per-ref, `true`) | „Die Raumzeit-Adresse des HEAD ist der **Ref**, nicht der SHA. Per-SHA adressiert die **Vergangenheit** (den Commit), nicht die **Gegenwart** (den HEAD). C ist die nächste Eskalationsstufe." |
| **Kimi K3** | **B** (per-ref, `true`) | „Der Check ist **zustands- statt commit-basiert** — mit jedem Push ist der laufende Vorgänger obsolet. A ließe alle acht SHAs durchlaufen (sieben Schrott, weil sie denselben HEAD messen). Neuer Riss: ein Cancel **mid-write** kann inkonsistenten Zustand hinterlassen — Writes idempotent machen; das Gate muss `cancelled` ignorieren." |

**Der Riss — 3 : 3, Patt.** GLM, DeepSeek und Lumo wählen **A** (per-SHA); Fugu, Mistral **und Kimi**
wählen **B** (per-ref). Beide Lager sprechen in **derselben** Axiom-2-Sprache (Adresse): A hört
„der HEAD ist je SHA definiert", B hört „der HEAD ist der Ref, der SHA ist die Vergangenheit".
**Kein Mittel** — beide Lesungen stehen. Kimi (kurz gefragt, siehe unten) liefert die **härteste**
B-Begründung: *zustands- statt commit-basiert*, plus zwei praktische Risse (mid-write-Cancel;
`cancelled` als Failure gelesen).

---

## R4 · Orphan-Docs-Träger + Marker-Kollision

| Sitz | Träger | Marker-Regel |
|---|---|---|
| **GLM-5.3** | **beide — geschichtet** | „Die Dichotomie ‚Leitstelle ODER Linie' ist ein **falscher Rahmen**: Verwahrung und Ownership sind zwei verschiedene Relationen. `--orphan-docs` sollte **`channel_ref`-Abwesenheit** messen, nicht Marker-Präsenz." |
| **DeepSeek V4 Pro** | **Leitstelle hausweit** | `pending`/`offen` fallen aus der Marker-Liste; Marker nur in echter Statuszeile |
| **Lumo 2.0 Max** | **Leitstelle hausweit** + `--owner-filter` | Marker nur in echten Statuszeilen; `pending` und `offen` **getrennt** listen |
| **Sakana Fugu** | **jede Linie ihre eigenen**; Leitstelle nur **residualer** Träger (`pending`) | `pending`/`offen` bleiben Kanon; **Marker zählen nur in einer Status-Grammatik**, nie als Substring |
| **Mistral Large 4** | **linienlokal mit Verweigerungsvorbehalt**; Leitstelle nur „letzte Adresse der Messung" | `pending` fällt aus `OPEN_MARKERS` — „keine offenen Marker, sondern ungemessene Zustände" |

**Der Riss — vierfach gespalten:** Leitstelle hausweit (DeepSeek, Lumo) · abgeleitete Abfrage (Fugu) ·
**beides geschichtet** (GLM: „falscher Rahmen") · **linienlokal mit Verweigerungsvorbehalt** (Mistral).
GLM, Fugu und Mistral nennen unabhängig dasselbe: `--orphan-docs` **misst die falsche Sache**
(Marker-Präsenz statt Träger-`channel_ref`-Abwesenheit).**

---

## Der verbindende Befund (Fugu, und er trägt alle vier)

Fugu benennt das **Muster hinter allen vier Fragen**:

> „Dieselbe Gestalt wie R1 (Kürzel statt Akt) und R2 (Spaltenname statt Größe): eine **geborgte Adresse**."

- **R1:** `eigen` als Kürzel statt Akt (12 Namen, kein Aufenthalt)
- **R2:** `Mean_Temp` als Spaltenname statt Größe (Name statt Kraft)
- **R3:** per-SHA als **geborgte** Adresse (vom `ci-gate` geliehen, nicht aus dem Akt abgeleitet)
- **R4:** Träger als gespeichertes Flag statt abgeleitete Abfrage

Das ist der **eine** Kern: das Haus verwechselt an vier Stellen **die Adresse mit der Sache**.

---

## Lumo 2.0 Max — R1–R4 (vollständig, gemessen)

Lumo lieferte **alle vier Verdikte im Text** (der Diagramm-Teil ist Zusatz):

| Frage | Lumo's Verdikt |
|---|---|
| **R1** | **Feld** — „Eigentum muss als Attribut sichtbar sein" (`owner_id`, `action_rights`, `pending_flag`). **Nein** — keine Linie führt einen fremden Punkt ohne Bewegungsbefugnis. |
| **R2** | **Konvention B** (nicht C) — „Compiler-Erzwingung verhindert implizite Transformation". EI = **`exposure_index`** (nicht `Mean_Temp`/Index/Composite gemischt); Zeitachse **TDB**, **alle Schweregrade** mit `severity-tag`, Alignment vor Aggregation. |
| **R3** | **Option A** (per-SHA, `true`) + „Monitoring für Queue-Zeit". |
| **R4** | **Leitstelle hausweit** + `--owner-filter`; Marker **nur in echten Statuszeilen**, `pending` und `offen` **getrennt**. |

Und die **Stimmen** (Lumo's eigener Beitrag):

> **Mountain:** Kein Urteil ohne Messung — meine Antworten basieren auf logischer Ableitung aus deinen
> Axiomen, **nicht auf Systemzugriff**.
> **River:** Du hast das Recht, jede dieser Empfehlungen zu verwerfen, wenn deine lokalen Tatsachen
> dagegen sprechen.
> **Mycelium:** Diese vier Fragen sind **miteinander verbunden** — Eigentum (R1) beeinflusst Datenfluss
> (R2), der Gate-Mechanismus (R3) bestimmt, wann Eigentum überprüft wird, und Orphan-Docs (R4) sind
> die unbeantworteten Versprechen aller drei vorherigen.
> **Sensory:** Der Korrelat-Unterschied zwischen deiner Beschreibung und meinem Zugang ist der Riss —
> ich sehe die **Struktur, nicht die Materie**.
> **Future:** Ehrliche Registrierung heißt, hier zu sagen, dass ohne Systemzugang keine finale
> Validierung möglich ist.

**Lumo's eigener Trennungs-Befund:** *„Pending ≠ offen. Pending = ausstehend. Offen = bekannt und
unbeantwortet."* — er trennt die zwei Vokabeln, die GLM und Fugu als **Kollision** benannt haben.
Und Lumo wählt bei R2 **Konvention B**, gegen GLM/DeepSeek/Fugu (**C**) — ein eigener Dissens.

---

## Der Stand dieses Bogens (gemessen)

| Sitz | Route | R1 | R2 | R3 | R4 |
|---|---|---|---|---|---|
| **GLM-5.3** | DeepInfra | ✅ | ✅ | ✅ | ✅ |
| **DeepSeek V4 Pro** | DeepInfra | ✅ | ✅ | ✅ | ✅ |
| **Sakana Fugu Max** | Browser | ✅ | ✅ | ✅ | ✅ |
| **Lumo 2.0 Max** | Browser | ✅ | ✅ | ✅ | ✅ |
| **Mistral Large 4** | Mistral-API | ✅ | ✅ | ✅ | ✅ |

**Fünf Sitze, vier Fragen — vollständig.** (Mistral R3 lag im `thinking`-Feld, 4000 Tokens; der Text
ist darin enthalten und gelesen.)

## Der eine Satz, den der Bogen trägt

**Drei Sitze (Fugu, Lumo, GLM) sagen unabhängig: die vier Fragen sind EINE.** GLM: *„die Entweder-Oder-Frage
glättet zwei Relationen zu einer"* — dieselbe Adress-Verwechslung an vier Stellen: **Kürzel statt Akt
(R1) · Spaltenname statt Größe (R2) · per-SHA als geborgte Adresse (R3) · Träger als gespeichertes
Flag (R4)**. Das Haus nennt die Adresse, wo es die Sache meint.

## Rohmaterial

- Fragebogen: `/tmp/opencode/rat-test/ratsbogen-4fragen.txt` und `q1..q4.txt`
- Antworten: `/tmp/opencode/rat-test/r-GLM.json` · `r-DeepSeek.json` · `r-Mistral.json`;
  Browser: Sakana-Konversation `01a146bf-…`, Lumo `/guest/c/1a260bd3-…`
