# Rat — R-2026-10-17-j: Ist `K` mit der 1981–83-WDC-Reihe identifizierbar?

**Frage:** `state/stimmen/2026-10-17-k-identifizierbar-formular.md`
**Gefahren:** 2026-10-17 — API-Sitze (Subagenten, Modell direkt); UI-Sitze (Fugu · Lumo) **ausstehend**.
**Kosten:** DeepSeek V4 Pro (`#low`) · Kimi K3 (`#low`) · Mistral Large 4 (`#none`) — Cap 0,15 nicht berührt.

## Die Antworten

**DeepSeek V4 Pro** (`deepinfra/deepseek-ai/DeepSeek-V4-Pro#low`, `ses_eb44bf194ffeGQz9YpyK3WtwMZ`):
> Die WDC-Reihe bringt Bodenindizes (AE, Dst, Kp) — das sind Treiberproxies, keine direkten Kraftgrößen. `K` verlangt mindestens `∇p` und `B/j`, beides aus einem einzigen Satellitenorbit; daran ändert die WDC-Reihe nichts. Die Reihe stützt ausschließlich die bedingte TE `TE(NVE→IVE | Z)`; das ist ein Indikator, kein `K`. Ein Regressionsmaß gegen AE/Kp wäre eine neue, benannte Definition von `K`, die erst beschlossen werden müsste.
> **VERDIKT: Option A** — `K` bleibt nicht identifizierbar; `K = pending`, Indikator eigener Punkt.
> **STÄRKSTER RISS:** kein `∇p`, kein `B/j`, Ein-Satellit — WDC-Reihe ist Treiberproxy, kein Kraftmaß.

**Kimi K3** (`deepinfra/moonshotai/Kimi-K3#low`, `ses_eb44bf1b6ffejGntbrEZz5oiUn`):
> AE/Kp/Dst messen globale Treiberintensität am Boden und in Äquivalentströmen, nicht die lokale Impulsbilanz auf der DE-2-Bahn. Ein Ein-Satellit kann keinen Gradienten liefern; kein Bodenindex ersetzt ihn. Korrelation gegen AE wäre Regression des Ionspeeds auf einen globalen Proxy — Indikator, nicht `K`. Option B wäre nur mit einer neuen, schwächeren Definition von `K` ehrlich: Definitionswechsel, keine Messung. Als Ratssitz: **Beleg zweiter Hand** — die Binärdateien nicht selbst eingesehen.
> **VERDIKT: A** — `K` bleibt pending; WDC-Reihe stützt ausschließlich die bedingte TE als eigenständigen Indikator-Punkt.
> **STÄRKSTER RISS:** kein lokaler Gradient messbar (Ein-Satellit, kein `∇p`/`B/j`) — Bodenindizes ersetzen das strukturell nicht.

**Mistral Large 4** (`mistral/mistral-large-4#none`, `ses_eb44bf151ffe80spKuCJLVpgrH`):
> `K` war nie als Index definiert, nie ein Proxy. Eine Korrelation Kp↔AE wäre die Korrelation zweier Indizes, keine Identifikation von `K`. Ohne benannte Definition, was `K` dann *ist*, wäre Option B Etikettenschwindel. Option C wäre voreilig: die Ernte läuft, die Lücke ist nur nicht geschlossen.
> **VERDIKT: A** mit Register-Zeile `K = pending`.
> **STÄRKSTER RISS:** kein `∇p`, kein `B/j`, Ein-Satellit — die Impulsbilanz-Lücke ist durch Index-Ernte nicht zu schließen.

## Bilanz

**3/3 API-Sitze: Option A.** Kein Widerspruch. **UI-Sitze ausstehend.**

## Der Riss (nicht geglättet)

Kein Sitz widerspricht dem Verdikt — aber **zwei Risse** stehen:
1. **Beweislast, ungetragen:** Kimi sagt ausdrücklich, es habe die Binärdateien **nicht selbst eingesehen** („Beleg zweiter Hand"). Die 1 634 952 records / 0 Lücken / 810 720 AE-rows stammen aus dem Fragebrief, nicht aus einer Messung des Sitzes. Der Riss ist benannt, nicht geglättet.
2. **Die Definitionsfrage bleibt offen, wer sie stellt:** alle drei sagen, Option B wäre ein **Definitionswechsel** — aber keiner sagt, **wer** `K` neu definieren dürfte. Das ist kein Mess-, sondern ein Verfassungsakt (Rat + Operator), und er ist hier nicht entschieden, nur benannt.

**Lumo 2.0 Max** (`lumo.proton.me/guest`, UI, 2026-10-17):
> AE, AL, AU, AO, Dst und Kp sind Indizes des Antriebs und der Dissipation, nicht die Kraftgröße `K` selbst — sie messen Integrale über andere Volumina, Bänder und Kadenzstufen als die Satellitenmessung. Eine Konditionierung erzeugt bedingte Abhängigkeit, und genau das ist TE. Der Indikator braucht eine **eigene Falsifikationsbedingung**, sonst wird `pending` zur ewigen Schuld ohne Ablaufdatum.
> **VERDIKT: A** — `K` bleibt nicht identifizierbar, Register `K = pending`, die bedingte TE ist ein eigener Indikator-Punkt **mit benannter Falsifikationsbedingung**.
> **STÄRKSTER RISS:** Der Kadenz- und Bandbreiten-Mismatch (Kp 3 h, Dst 1 h, AE 1 min, 13 Observatorien gegen einen Satelliten) könnte die **Decke** setzen, nicht die Physik — dann misst die bedingte TE die Auflösung der Indizes und nicht die Kopplung, und `pending` bliebe trotz perfekter Daten bestehen.

**Fugu Max High** (`chat.sakana.ai`, UI, 2026-10-17):
> Kein Index liefert `T`, `∇p` oder `B/j` — `K` wird nicht identifizierbar, nur besser kontextualisierbar. Indizes sind **Prädiktoren, nicht der Term**. Option B wäre nur mit einer benannten, extern definierten `K`-Definition zulässig; C wäre eine Überreaktion.
> **VERDIKT: A** — `K = pending`, bedingte TE als eigener Indikatorpunkt.
> **STÄRKSTER RISS:** Die reichere Z-Besetzung verleitet, den Indikator als `K`-Ersatz zu lesen — genau der Kategorienfehler, den der Rat am 2026-10-17 schon einmal zurückgewiesen hat.

## Bilanz

**5 von 5 Sitzen: Option A** (DeepSeek · Kimi · Mistral · Lumo · Fugu). Kein Widerspruch.

## Der Riss (nicht geglättet)

1. **Beweislast, ungetragen:** Kimi sagt ausdrücklich, es habe die Binärdateien **nicht selbst eingesehen** („Beleg zweiter Hand"). Die Zählungen stammen aus dem Fragebrief, nicht aus einer Messung des Sitzes.
2. **Der neue Riss (Lumo): der Kadenz-/Bandbreiten-Mismatch.** Kp 3 h, Dst 1 h, AE 1 min, 13 Observatorien — gegen **einen** Satelliten. Wenn die Indizes gröber sind als die Kopplung, misst die bedingte TE die **Auflösungsgrenze der Indizes**, nicht die Physik. Dann bliebe `pending` wahr, auch mit perfekten Daten. Das ist der schärfste Einwand der Runde und er ist **nicht** beantwortet.
3. **Die Definitionsfrage bleibt offen, wer sie stellt:** alle sagen, Option B wäre ein Definitionswechsel — keiner sagt, **wer** `K` neu definieren dürfte. Verfassungsakt (Rat + Operator), hier nicht entschieden.

## Stand

**entschieden** (2026-10-17). Adressat gezählt aus dem Verdikt.

## An mountain

Origin: rat-2026-10-17-j
- Register-Zeile `K = pending` setzen (Ernte da, Wert nicht); der TE-Indikator wird **nicht** als `K` verbucht.
- Beleg: dieser Verdikt-Abschnitt + `2026-10-17-k-identifizierbar-formular.md`.

## An river

Origin: rat-2026-10-17-j
- Gebundener Schritt: bedingte TE `TE(NVE→IVE | AE/Kp/Dst/LT/Höhe)` als **eigener Indikator-Punkt mit benannter Falsifikationsbedingung**.
- Der Kadenz-Mismatch (Lumo) ist vor dem Bau zu beantworten: misst die bedingte TE die Kopplung oder die Auflösung der Indizes? Bis dahin bleibt der Punkt `pending`, nicht `gebaut`.
- Grenze: kein Register-Schreibakt ohne Mountains `K`-Zeile.
