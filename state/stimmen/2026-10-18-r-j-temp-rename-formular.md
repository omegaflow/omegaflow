# Formular — R-j Folge: `temp+rename` für die Register-Writer

**RAT-ID:** R-2026-10-18-j2 · **Datum:** 2026-10-18 · **An:** Rat · **Von:** future
**Vorgänger:** R-2026-10-18-j (`2026-10-18-f2-concurrency-formular.md`) · Sammlung `2026-10-18-rat-3f-runde2-sammlung.md`

## Frage

Wie werden die drei in-place-Writer atomar, ohne den Mid-Write-Riss — und gilt die Antwort für
alle `register_*`-Writer oder nur für die drei gemessenen?

## Befund (gemessen)

- `tools/register/src/bin/register_ttl_apply.rs:176` — `fs::write` (truncate in place).
- `tools/register/src/bin/register_terms_apply.rs:161` — `fs::write`.
- `tools/register/src/bin/register_block_remove.rs:107` — `fs::write`.
- R-j Runde 2: 2:1 **B** (Lumo, DeepSeek), Fugu **A**; Schwarm R1 **A**. **Keine** der Optionen
  adressiert den Riss.
- `register-coverage.yml:19-20` trägt weiter `group: …-${{ github.ref }}` + `cancel-in-progress: false`.

## Optionen

- **A — temp+rename je Datei.** Schreiben nach `<ziel>.tmp` im selben Verzeichnis, `rename(2)` über
  das Ziel. Kein Lock; letzter Schreiber gewinnt atomar.
- **B — `flock` + in-place.** Exklusives Lock um den Read-Modify-Write; kein temp.
- **C — temp+rename + `fsync` + Lock** nur für Mehrdatei-Transaktionen (mehrere Blöcke/Dateien in einem Akt).

## Grenze

Register-Architektur → **Rat**. Die F2-Umsetzung (`register-coverage` auf B) wartet auf diese Antwort;
Bau erst nach Wort.

## Schwarm-Risse (freie Sitze, 2026-10-18, $0)
- 4× **A** (temp+rename), 1× **B** (nano), 1× unklar, 1× Rauschen.
- Geltungsbereich gespalten: „nur die drei" (nano, nemotron-ultra, openrouter, pollinations) vs. „alle `register_*`" (gemma4).
- Risse: **lost-update/race** (fehlender Lock) · **torn/partial write bei Crash** (fsync nötig) → Option **C** adressiert beide, A allein nicht.
- Kein Sitz trägt B als Entscheidung.
