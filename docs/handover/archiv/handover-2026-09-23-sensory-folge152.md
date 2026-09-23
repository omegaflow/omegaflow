<!--
  title: Handover — Sensory-Folge 152 (Stand 2026-09-23)
  session: Sensory-Folge 152
  class: handover
  date: 2026-09-23
  sha256: 6512141fe6991abde4e022158dbf132e404391ee1cc859d9fb2f543c4fcbf1aa
  status: live
-->
# Handover — Sensory-Folge 152 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt). Kein Dokument wächst ohne Messung; die Droh-Sprache ersetzt
den Schritt nicht. Der Planungs-Pass legt **alle** eigenen Punkte vor und schlägt
vor, jeden parallel abarbeitbaren zu dispatchen; `operator-gebunden`, `blockiert`
und `wartend` werden benannt, nie dispatcht. Gibt es keinen abarbeitbaren Punkt,
sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt); eine Session, die nur
dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 152)

- **HEAD** `2339533c2` == `origin/main`; eigene Arbeit der Vorsession `7ed618d5a`
  (te-gate-Split) liegt darunter. Der Baum trug Fremd-uncommittet (mountain
  folge140→archiv, `post.md`, `phi/{sources,witnesses,footprints}.φ`, mountain
  folge141) — **nicht angefasst**.
- **Postfach** — `mail_digest` absent; `post.md` trägt `An mountain:` (sfetch-Body)
  + `An mycelium:` (ci-check-Reds) — **keine an sensory**, nicht angefasst.
- **Register** — `register_lookup --open`: keine sensory-eigenen Zustandseinträge
  (115 Docs, 560 offen, 16 zustand due, 2 post, 11 disposition — alle mycelium/wartend).
- **`open_points_check`** der folge151: 8 Pfad-Refs, 2 „absent" — nur Glob-Muster
  (`phi/*.φ`, `src/archivar/{…}.rs`); **keine** stale Punkte.
- **CI** — `ci_manage view 35834155918`: `te-gate @7ed618d5a` **in_progress** seit
  07:54Z. `ci_manage list`: `ci-check 35838658864` pending, `35834150611`/`35833935838`
  cancelled, `health-check 35813434762` in_progress. Kein Poll.
- **`git_safety --snapshot`** → `refs/safety/1790153595`.

## Geschlossen in dieser Session (git trägt sie)

### Tafel-Struktur verschärft (Operator-Wort, 2026-09-23)
`AGENTS.md:191` + `docs/handover/_template.md`: die Tafel trägt `Trigger` als eigene
Spalte (`Status = f(Trigger)` wird prüfbar; ein `wartend` ohne Trigger ist eine
Gate-Fixture), `Lage` trägt den Messstempel `<state> (gemessen <Datum/Zeit> via
<Werkzeug>)` (Veraltung sichtbar ohne Extra-Lesung), `Braucht` trägt den wörtlichen,
kopierbaren Schritt. Keine abgeleitete `dispatchable`-Spalte (wird pro Pass
abgeleitet). Der Handover-Block ist jetzt vierteilig (`Trigger`/`Lage`/`Blockade`/
`Braucht`). Anlass: der te-gate-Punkt der folge151 stand mit „Blockade: Commit+Push"
— Dispatch war beim Pass längst vollzogen; `open_points_check` prüft Pfade, nicht
Zustandsfrische.

## Offen (aufgeschlüsselt)

### te-gate-Split — n=1000-Lauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs `35834155918`
- **Lage:** Split (13 Jobs + `issue`) gebaut+gepusht (`7ed618d5a`); Dispatch gelandet,
  `te-gate 35834155918 @7ed618d5a` **in_progress** seit 07:54Z (gemessen 2026-09-23
  via `ci_manage view`).
- **Blockade:** Lauf nicht abgeschlossen (`gate_fpr_autocorrelation` ~17323 s im
  Vorlauf; kürzere Jobs landen früher).
- **Braucht:** `ci_manage log 35834155918` einmal nach Abschluss → `te_fn_probe`-Riss-4
  + n=1000-Gates (`ksg_sweep`, `ksg_k_gate`, conditional, `mi_lag`).

### HRV/Puls→Strahlung (vC-Permeabilität)
- **Status:** operator-gebunden | **Bindung:** eigen (HW-Träger `operator`)
- **Trigger:** ESP32-Träger verfügbar / Operator-Wort
- **Lage:** TE-Apertur→Strahlung gebaut (`omega.rs:347` `aperture = field_permeability
  * tone_scale`, `actuators.rs:29`, `tests.rs:570`, `356fa616`); `src/archivar/hrv.rs`
  trägt RMSSD/tone-Gate; der Puls-**Ankunfts**-Pfad fehlt (gemessen 2026-09-23 via
  `sgrep`).
- **Blockade:** physischer Träger (ESP32, BOM).
- **Braucht:** Puls/HRV-Ankunft via ESP32-Firmware → `tone_scale` bauen.

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** dritter (`termin:2026-09-28`)
- **Trigger:** Perigäum 2026-09-28
- **Lage:** Kanäle live; Zellen ab Perigäum messbar (gemessen 2026-09-23 via
  `docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** externer Termin.
- **Braucht:** Messung nach dem Perigäum.

### NSE/Haug
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Dateieingang (Mail 2026-09-17)
- **Lage:** Route offen, kein Dateieingang gemessen (gemessen 2026-09-23 via
  Übergabe-Ref).
- **Blockade:** kein Dateieingang.
- **Braucht:** eingehende Datei lesen.

## Fremd-CI (geroutet, nicht sensory)

- `dropped-gate` delta 77 + `register_sort` `phi/sources.φ` — von river folge9
  (`9983f5b0`) via `post.md` `An mycelium:` geroutet. Beide mycelium.
- `ci-check 35838658864` pending; `35834150611`/`35833935838` cancelled — nicht sensory.

## Geteilter Baum — eigener Pfad-Satz

- `AGENTS.md` (Tafel-Verschärfung, 3 eigene Hunks)
- `docs/handover/_template.md` (Trigger-Stelle)
- `docs/handover/handover-2026-09-23-sensory-folge152.md` (neu)
- Move `handover-2026-09-23-sensory-folge151.md` → `archiv/` (eigene Linie, atomar)

Fremd uncommittet hielt der Baum mountain folge140/141, `post.md`,
`phi/{sources,witnesses,footprints}.φ` — **nicht angefasst**, nicht gestaged.

## Benchmark

- **Rat (pro/max, Architektur):** te-gate-Split-Verdikt — Option D (parallele Jobs),
  6h-Hard-Cap. Kein Gegenlauf (Architektur).
- **`general` (flash):** Tafel-Abarbeitbarkeits-Analyse — Ergebnis deckungsgleich
  mit dem Session-Plan (0/5 abarbeitbar), eine Verschärfung (te-gate-Dispatch
  vollzogen). flash vollständig.

## Abschluss

Commit+Push pfad-begrenzt auf den eigenen Satz; `git show --stat HEAD` nur eigene
Dateien, `origin/main == HEAD` danach. Kein Workflow von dieser Session angelegt
oder geändert → kein Dispatch. `/consent` ist der session-weite Consent, nie das
Commit-Wort.
