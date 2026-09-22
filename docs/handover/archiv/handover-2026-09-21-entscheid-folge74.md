<!--
  title: Handover — Entscheid-Folge 74 (open_points_check repariert: Exec-Bit + Pfad-Parser; offene Punkte aufgeschlüsselt) (Stand 2026-09-21)
  session: Entscheid-Folge 74
  class: handover
  date: 2026-09-21
  sha256: 3f6aed9afab34ea06980fa429d7532957ecbe9c104dc3682e9eddc78a126afaf
  status: live
-->
# Handover — Entscheid-Folge 74 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt** geführt: **Lage**
(Zustand, gemessen) / **Blockade** (woran es hängt, oder „keine") / **Braucht**
(was es löst: Werkzeug, Datei, URL, Anfrage, Operator-Wort). Jeder Punkt trägt
seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 74)

- **HEAD** `ae822fe7` (research folge128) — die research-Linie hat während dieser
  Session committet; `origin/main` == HEAD. Arbeitsbaum trägt nur eigenen,
  noch nicht committeten Anteil.
- **Postfach** — `mail_ledger.φ`: kein neuer Eingang. `external-state.md`
  Postfach-Zeile nachgezogen.
- **CI** — `external-state.md` CI-Zeile trägt die Messung der research-Linie
  (Forschung-Folge 128 @`da03beb4`); zusätzlich `ci_manage list` 2026-09-21 ~09:2x:
  `tools-build` `35572559276` in_progress; `ci-check` `35572602793` pending;
  `te-gate` `35572569205` pending; `hyperscanning-te` `35572559304` **failure**;
  `free-model-bench` `35567266519` + `free-model-agent-bench` `35567268692`
  in_progress; `rpw-cdn` `35571724517` success. Kein Poll.
- **`register_lookup --open`** — 3 `[entscheid]`-Registerpunkte
  `phi/blocked_sources.φ:21/60/65` (SuperDARN / solar-system-open-data / Amentum)
  → Operator-Queue; 2 fremde OPEN (`handover-2026-09-20-operator-entscheidungen.md`).
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD, nichts zu sichern.

## Messung dieses Atoms (kein offener Punkt)

- **`bin/open_points_check` war nicht ausführbar** (Mode 664 statt 775;
  `git ls-files -s` führte `100644`) — der Wrapper lief nie, `~/.local/bin`-Symlink
  ins Leere. Fix: Mode `100755`, Werkzeug läuft.
- **Pfad-Parser meldete falsche `ABSENT`-Treffer** (gemessen: ein als
  `` (`.github/workflows/hyperscanning-te.yml:41`). `` eingebetteter Pfad wurde mit
  Trailing-`` ` ``/`)` als Pfad getestet → ABSENT, obwohl die Datei getrackt ist).
  Fix in `tools/register/src/bin/open_points_check.rs` `normalize`: erst
  führende Wrapper-Zeichen strippen, `:line`-Suffix nur bei Ziffernfolge
  abschneiden, dann Trailing-Satzzeichen inkl. `.` entfernen (führender `.` bleibt
  erhalten für `.github/`). Vierter Testfall ergänzt; `cargo check` grün.

## Offen (aufgeschlüsselt)

### Free-Model-Bench (P13 + P2–P4)
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Zwei CI-Läufe sollen das Ranking der kostenlosen Modelle liefern —
  `free-model-bench` `35567266519` und `free-model-agent-bench` `35567268692`.
- **Blockade:** Beide Läufe sind noch `in_progress` (gestartet 06:09); es gibt noch
  kein Ergebnis-Artefakt.
- **Braucht:** Run-Abschluss abwarten, dann die Artefakte **einmal** lesen
  (`ci_manage view 35567266519` / `35567268692`) und das Ranking oder `pending`
  als Zeile tragen. Kein Poll.

### PINE64-Dokumentationspflicht (Ox64 / Mantis-Shrimp)
- **Status:** `blockiert` | **Bindung:** `linie:bau`
- **Lage:** Pine64 hat Ox64-Hardware zugesagt (Hardware beidseitig geschlossen);
  die Presence-Hardware „Mantis-Shrimp" ist ungebaut.
- **Blockade:** Die Hardware existiert nicht. Bau gehört zur **bau-Linie**, nicht
  zur Entscheid-Linie.
- **Braucht:** Die bau-Linie baut einen minimalen Mantis-Shrimp (Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen vor) und dokumentiert danach den
  Ox64. Nachricht liegt in `docs/handover/post.md` (`An bau:`).

## Operator-Queue (Stand folge74; einfache Sprache, je Eintrag Lage/Blockade/Braucht, mit Alter)

3. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   **Lage:** Sicherheitsereignis, App hat Kontozugriff. **Blockade:** offener
   Zugriff. **Braucht:** App unter `github.com/settings/connections/applications`
   widerrufen? (Alter: seit 2026-09-20)
4. **SuperDARN** (`blocked account`, `phi/blocked_sources.φ:21`) — **Lage:**
   HF-Radar-Ionosphären-Konvektion, Route über Globus + PI-Vereinbarung.
   **Blockade:** kein Konto, keine PI-Vereinbarung. **Braucht:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
5. **solar-system-open-data REST** (`blocked key`, `phi/blocked_sources.φ:60`) —
   **Lage:** HTTP 401 (Bearer-Token). **Blockade:** kein Token. **Braucht:**
   Konto/Token anlegen? (Alter: seit 2026-09-20)
6. **Amentum Developer** (`blocked account`, `phi/blocked_sources.φ:65`) —
   **Lage:** geomagnetisch/aviation-radiation/gravity (trial). **Blockade:** keine
   Registrierung. **Braucht:** `developer.amentum.io/register`? (Alter: seit 2026-09-20)
7. **Split-Routing-Verifikation** — **Lage:** 8 `000`-Hosts ungemessen.
   **Blockade:** braucht sudo + Netz. **Braucht:** Operator-Wort/Route für
   `./bin/proton-exit.sh ca` + direct↔tunnel-Nachmessung. (Alter: seit Ernte folge12–17)
8. **Cookie-Transfer** — **Lage:** `operator-gebunden`, Auslöser „Bedarf".
   **Blockade:** kein Bedarf. **Braucht:** nichts — wartend. (Alter: seit 2026-09-16)

## Benchmark

- **Delegation (Entscheid-Folge 74):** 1 × `general` (flash) für die unabhängige
  Verifikation (Handover-Pfade + Parser-Edge-Cases, read-only) — Routine-Klasse,
  kein Doppel-Lauf gegen pro/max (Klasse geschlossen, gemessen 2026-09-16: flash
  gewinnt). Ergebnis: alle genannten Pfade existieren, Parser-Fälle bestätigt.

## Geteilter Baum — eigener Pfad-Satz

- `tools/register/src/bin/open_points_check.rs` (Parser-Fix + Test)
- `bin/open_points_check` (Mode 100644 → 100755)
- `docs/handover/handover-2026-09-21-entscheid-folge74.md` (neu)
- `docs/handover/_template.md` (Punkt-Form Lage/Blockade/Braucht)
- `AGENTS.md` (Tafel-/Punkt-Regel um Lage/Blockade/Braucht erweitert)
- `docs/zustand/external-state.md` (Postfach-Zeile)
- Move `handover-2026-09-21-entscheid-folge73.md` → `archiv/` (eigene Linie, atomar)
- **nicht** angefasst: fremde `phi/*`, `tools/harvest`, fremde CI-/Zustand-Zeilen,
  die research-folge128-Arbeit, die fremde uncommittete Änderung
  `tools/register/src/bin/register_lookup.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push werden
`tools-build.yml` (neuer Wrapper/Parser) und `ci-check.yml` dispatcht; kein Poll —
das Ergebnis einmalig lesen (`ci_manage view <id>`).
