<!--
  title: Auftrag — docs/reference und docs/plans verteilen (Teil von main-reinigung)
  class: auftrag
  date: 2026-09-01
  status: pending
  sha256: 5720f31469242e118b560a63599be51924cfff17ecfdb08185416807aeafefab
-->

# Auftrag: die verbleibenden §401-Überschüsse von main ziehen

## Zweck

Dies ist der **Teil-Auftrag 3** des `main-reinigung`-Auftrags
(`docs/auftrag/auftrag-main-reinigung-kanonische-ordnung.md`): die zwei
dort noch offenen docs-Bestände von main entfernen und ihrer Heimat zuführen.
Die `tools`-Entflechtung (Teil 2) ist bereits erledigt — `Cargo.toml` hat
`members = ["tools/live", "tools/work"]`. Dieser Auftrag schließt die
§401-Reinigung von main ab.

## Kernregel (A = A, gemessen)

Was das Ding ist, bestimmt seine Heimat (§398/§399) — nie der Sammelort.
Heimat ist der Ordner, der das Ding inhaltlich trägt; die Wurzel trägt nur das
Release (§401).

## Gemessener Ist-Zustand (main, HEAD `fd01a980`)

`docs/reference/` = 38 getrackte Dateien, vollständig klassifiziert:

| Block | Umfang | Zweck | Zielort |
|---|---|---|---|
| Pioneer-DSN | 30 | Deep-Space-Network-Tracking, Pioneer-Anomalie, ODF-Format, Syntonization | `pioneer-korrelieren` |
| NAIF/NIST | 7 | SPICE-Kernel-/DAF-/PCK-Referenz, SI/UCUM-Einheiten, Body-IDs, `12_intro_to_kernels.pdf` | Archiv / Quelle |
| README.md | 1 | Verzeichnis-Index | bleibt auf main (§399 Katalog-Ort) |

`docs/plans/ref-auth-apis.md` (1) — stehende Auth-API-Referenzliste, Heimat
per Zweck: kein Release-Dokument → Archiv.

`docs/LICENSE` — **kein Duplikat** (Messung 2026-09-01): `docs/LICENSE` =
CC-BY-NC-SA 4.0, root `LICENSE` = PolyForm Noncommercial. Zwei verschiedene
Lizenzen, genau die AGENTS.md-Grenze. **Bleibt**, ist keine §401-Verletzung.

## Arbeitsschritte

1. **Pioneer-DSN-Block** (30 Dateien) in `docs/reference/` und
   `docs/plans/ref-auth-apis.md` von main nehmen; `pioneer-korrelieren`
   übernimmt den Pioneer-DSN-Block (§398: Erbe durch rebase, keine eigene
   Kopie).
2. **NAIF/NIST-Block** (7 Dateien) der Quelle zuordnen (Ziel
   laut Survey pending — Zug-Ziel wird vom Operator bestätigt).
3. **Verifikation:** main `cargo check` 0/0, `git ls-files docs/reference`
   == nur README.md, `docs/plans/` leer, `docs/LICENSE` unverändert.
4. **Lagebild aktualisieren:** Status auf das gezogene Ergebnis setzen.

## Lieferung

- main `docs/reference/` = 1 Datei (README.md-Index), `docs/plans/` leer.
- Pioneer-DSN-Block liegt auf `pioneer-korrelieren`.
- NAIF/NIST-Block liegt auf seiner bestätigten Heimat.
- `docs/LICENSE` bleibt (kein Duplikat).
- main `cargo check` grün.

## Abschluss

Der Auftrag ist erledigt, wenn main `docs/` gemessen auf die Katalog-Klassen
reduziert ist (§399) und die beiden Blöcke auf ihren Heimat-Strängen liegen.
Ausführung ist bau-sicher (nur docs), main ändert sich per Leitstelle-Merge
mit Doppel-Wort.
