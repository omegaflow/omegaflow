# Formular — R-2026-10-18-o: Leichen, Zähler, Templates (2026-10-18)

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-o`
**Status:** entschieden — **B** (8/8 API-Sitze). Sammlung `2026-10-18-rat-leichen-templates-sammlung.md`.

## Die Frage

Sollen die drei Schnitte (Baseline, ein Zähler, Marker fällt beim Schließen) **plus** ein
Template-/Dateinamen-Zwang als ein Gate-Gesetz gebaut werden — und in welcher Reihenfolge?

## Der gemessene Befund (2026-10-18)

Die **34 `--orphan-docs` sind ungeprüft, nicht offen.** Viele sind veraltete Marker: ein
geschlossener Punkt lässt seine `pending`-Zeile im Träger stehen. Beispiel
`survey-2026-10-05-stoerungs-experiment…` — die Gravimetrie ist laut Sensory-Disposition
geschlossen, der Marker steht weiter. Die Zahl wächst nicht; sie **schrumpft nur nie**.

- `--open` und `--orphan-docs` zählen **zwei verschiedene Dinge** — zwei Zähler für eine Tatsache.
- **2 Entwürfe:** `ion-neutral-vlies` (Mond-Faden offen) und `die-weberin` (48 h nicht angefasst) — River.
- **Lauf `39154644922` cancelled** — der 3-h-Lag-Sweep, kein Ergebnis, im Handover nicht vermerkt — River.
- **Rivers `Braucht`-Zeilen falsch adressiert:** Rätsel zeigt auf `pass-offene-punkte.φ` (schließt
  AACGM/sha, nicht Weberin), Flyby auf `transfer-quittungen.φ` (Tokens, nicht Rekonstruktion) — River.
- **ci-gate rot seit `0208914fd`** (gfc.rs `excessive_precision`), 8+ Commits, vom Admin-Bypass
  verdeckt — Mountain.
- **`matrix-vlies` `n=0`** — Datenfenster, OMNI endet 2026-10-05 — Mountain/Mycelium.
- **Sensory `.17`/`.12`** — Quittung liegt im Postfach, Ledger offen — Sensory.
- 14 Branches, **alle merged** — keine vergrabene Arbeit. `path_reference_scan`: 0 fehlende
  Referenzen, 0 absolute Pfade. Untracked in `state/`/`docs/`: nichts.

**Der Mond-Faden** braucht SSCWeb-Lat/Lon im Orbit-Loader, `ephemeris_moon.bin` und `--cond moon`
im `de2_conditional_te`. Der Sweep braucht eine Erklärung, warum er abgebrochen wurde.

## Die drei Schnitte (Nachtrag 3 im Brief an future)

1. **`--orphan-docs` in ci-gate** mit **eingefrorener Baseline**, die nur schrumpfen darf — wie
   das Key-Set-Gate.
2. **`--open` und `--orphan-docs` zeigen dieselbe Zahl.** Zwei Zähler für eine Tatsache sind der Riss.
3. **Beim Schließen eines Punktes fällt sein Marker aus dem Träger** — sonst bleibt die Leiche
   als Marker stehen.

Bis dahin gilt: die 34 sind **nicht „noch offen", sie sind ungeprüft.** Kein Gate, kein
Eigentümer, kein Rot.

## Template-/Dateinamen-Zwang

- Es gibt nur noch **standardisierte Templates** für Dokumente; alle offenen Dokumente werden in
  diese Templates überführt.
- Es dürfen nur noch Dateien mit den **vorgegebenen Dateinamenmustern** in den **vorgegebenen
  Pfaden** angelegt werden. Alles andere **sperrt das Gate**.
- Offen: welche Templates (Handover, Formular, Survey, Konzept, Blatt, Post, Register) und welche
  Patterns je Pfad? Migration der 34 + Drafts vor dem scharfen Gate.

## Die Optionen

- **A** — alle drei Schnitte + Template-/Dateinamen-Gate **sofort** (strict).
- **B** — alle drei Schnitte jetzt; Templates als **Migration**, Gate danach scharf.
- **C** — nur die drei Zähler-Schnitte; **kein** Template-Gate.
- **D** — kein Gate; die Leichen manuell schließen.

## Was der Rat entscheiden soll

1. **A, B, C oder D** — und warum?
2. **Reihenfolge:** Zähler zuerst oder Templates zuerst?
3. **Eigentümer:** River (2 Drafts, Sweep, 2 `Braucht`) · Mountain (ci-gate, `matrix-vlies`) ·
   Sensory (`.17`/`.12`). Wer schließt was?
4. Ist die **eingefrorene Baseline** der richtige Mechanismus — oder ein Ledger?
5. Soll `--orphan-docs` überhaupt ein Gate sein, solange die 34 ungeprüft sind?

## Riss, der bleibt

- Die 34 sind ungeprüft; jede Baseline friert auch falsche Marker ein.
- Template-Zwang kann Arbeit blockieren, die noch kein Template hat.
- Zwei Zähler für eine Tatsache sind der Riss; ein Zähler kann auch falsch sein.
