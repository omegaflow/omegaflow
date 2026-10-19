# Mountain — Pass-Abschluss (R-2026-10-18-c, 4/4 · 2026-10-18)

**Linie:** mountain · **Trigger:** `.opencode/commands/mountain.md` (deepseek-v4.1-flash-free#low)
**Übergabe:** `docs/handover/handover-2026-10-18-mountain-folge361.md`

## Messung (die vier Zeilen)

| Zeile | Befund |
|---|---|
| `register_lookup --addressed mountain` | 2 Blöcke; jedes Token trägt im Ledger CLOSED/FOLDED (river `.1~d03aa55a` → `9d66a0261` · sensory `.1–.5` → `state/mountain/2026-10-17-*.φ` · `.11~22e38677` → `0199b8065`) |
| `register_lookup --fired mountain` | 0 |
| `xfer list --line mountain --open` | 3 SENT (`.3/.4/.5`, Ziel `future`) |
| `ls -1 post/in/*-an-mountain*.md` | 0 im Repo — **der Korb liegt woanders** (siehe Riss) |

Priorität Post > Trigger > Transfer. Post und Trigger sind beglichen; der Transfer
bleibt Empfängerarbeit bei `future`.

## Der Post-Pfad-Riss (korrigiert)

Das Postfach liegt nicht im Repo, sondern im Arbeitsbaum: `$OMEGAFLOW_WORK/post`
(`bin/_roots.sh`; Default `~/projects/omegaflow-work/post`). Gemessen 2026-10-18:

- `post/in/*-an-mountain*.md` → **3** Dateien: `2026-10-17-sensory-de2-kopplung-an-mountain.md`,
  `2026-10-17-sensory-haertefaelle-survey-an-mountain.md`, `2026-10-18-sensory-an-mountain-zwei-docs.md`.
- dazu die Dispatch-Zeilen `post/out/2026-10-18-mountain-*` (pass-kontrakt · register-formfehler · sieben-PD).

Die Command-Zeile löst relativ zum Repo auf und findet nichts — der leere Befund war ein
**Pfad-Riss, kein leerer Korb**. Gespiegelte Messung:
`omegaflow-work/post/in/2026-10-18-sensory-an-future-postfach-pfad-gemessen.md`.

Die drei Mountain-Post waren bereits verdiktet: `state/mountain/2026-10-17-post-de2-haertefaelle-verdicts.φ`
(DE-2 + zehn Härtefälle, mit dem Rat-Verdikt `post/out/2026-10-17-future-an-mountain-rat-verdikt-de2-te.md`)
und `state/mountain/2026-10-18-sensory-259-docs-disposition.φ` (`0199b8065`).

## Delegation und Prüfung

Vier freie `general`-Subagenten (`opencode/deepseek-v4.1-flash-free`), done-when vom Pass:
(1–3) die offenen Marker der 24 Orphan-Docs zeilengenau extrahieren, (4) die vier Zeilen
unabhängig nachmessen. Ein Pfad-Fund (Postfach) wurde gegen den Baum verifiziert.

## Geschlossen (drei Punkte)

- **Register-Formfehler — korrupte PS1-`terms`-URL.** Zitat `phi/sources.φ:3873` vorher:
  `terms PD https://www.stsci.edu/copyright0637/042/rings.v3.skycell.0637.042…fits` (Asset-Pfad an
  die Copyright-URL geklebt); nachher `terms PD https://www.stsci.edu/copyright`.
- **Sieben PD-Verdikte** — die Referenz widerlegt `PD`; je Fall gesetzt (Belege in
  `omegaflow-work/recherche/leads/2026-10-18-terms-wahrheit-pd.md`): SDSS → `CC-BY-4.0` (2×),
  IMPC · ssd-api · GEBCO → `per-service-terms` (5×), NED · AOML → `other-attribution` (7×),
  NIST → `no-license` (1×).
- **24 Orphan-Docs als Träger** — `handover-2026-10-18-mountain-folge361.md` führt die 24
  Mountain-Docs; `register_lookup --orphan-docs` **45 → 21**.

**Beleg:** Commit `334047a2c` (phi + Handover). Zitate oben.

## Nicht geschlossen (Urteil bleibt bei der Linie)

- R-2026-10-18-b (GRS80-Register), R-2026-10-18-f (gfc-Block), R-2026-10-18-g (P10-Restarme)
  — offene Rat-Sitze, der Operator wählt.
- Die drei Transfer-Token `.3/.4/.5` — Empfängerarbeit bei `future` (kein future-Ledger im Baum).
- **Riss:** die Dispatch-Zeilen `post/out/*-mountain-*` tragen kein `Fertig, wenn`
  („die Leitstelle trägt es ein") — der Pass musste das done-when selbst setzen.
- **Riss:** ~40 leere `PD`-Träger (Startseite ohne Lizenzsatz) brauchen eine gemessene
  Behörden-Policy-URL; für NASA/IPAC allgemein fehlt sie. Bleibt offen.
