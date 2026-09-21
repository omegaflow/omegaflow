<!--
  title: Handover — Ernte-Folge 134 (Stand 2026-09-21)
  session: Ernte-Folge 134
  class: handover
  date: 2026-09-21
  sha256: 2b25de774c69a75d775611d4b6b1733eed5978f2c1a56c6125c80b4f26311323
  status: live
-->
# Handover — Ernte-Folge 134 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Folge 134)

- **HEAD** `b3ec7683` beim Start; Arbeitsbaum trug Fremdarbeit (`post.md` — river/
  future ci-check-Nachrichten, nicht angetastet) und die eigenen Register-Reste.
- **Postfach** — neuester Ledger-Eintrag `1789978555` (Brave-Limit, unverändert);
  kein neuer Eingang ⇒ nicht fällig.
- **CI** — Watchdog-Snapshot: `ci-check 35613891252` in_progress; `glm-l2-cdn`
  `35599198872`/`35599180155` failure (Prä-Fix, Wurzel `attr_unsigned` behoben).

## Offen (aufgeschlüsselt)

### glm-l2-cdn — Verifikation des Parser-Fixes
- **Status:** wartend | **Bindung:** termin (Lauf)
- **Lage:** Fix `attr_unsigned` (String-Form `_Unsigned`) bei HEAD `b3ec7683`; Lauf
  `35623731418` dispatcht (`gh workflow run glm-l2-cdn`).
- **Blockade:** Lauf.
- **Braucht:** `ci_manage view 35623731418`; bei success sha256 → `sources.φ`
  glm_l2-Block → `kompiliert`.

### PS1 Order-10-Final — Lauf success, Asset 404
- **Status:** offen | **Bindung:** eigen
- **Lage:** Lauf `35595711594` success (1h36m); `ps1_dr2_coverage.fp01` auf Tag
  `ssd.jpl.nasa.gov` **404** (sniff 2026-09-21). `footprints.φ:19` Asset absent.
- **Blockade:** Workflow-`success` ohne Final-Asset — Wurzel ungemessen.
- **Braucht:** `ci_manage log 35595711594 --all` lesen (final-combine-Step),
  Wurzel messen.

### DEMETER ISL — Download blockiert, Parser-Gap (mountain)
- **Status:** wartend | **Bindung:** termin (Order) + linie:mountain
- **Lage:** Order 18387 RUNNING, `availableFilesCount=0`, ZIP-Route 204, per-File
  F5/500; Parser-Gap: `demeter.rs:60` verlangt `ISL SURVEY` @[204..214],
  `DMT_N1_1143` trägt `ISL BURST` (1144 `ISL SURVEY`), 289-B-Blöcke sonst
  identisch. Ledger `parser-gap` → mountain gesetzt.
- **Blockade:** Order-Download + Marker-Prüfung.
- **Braucht:** mountain: Marker auf `ISL SURVEY|ISL BURST` erweitern; dann
  Download → `demeter_compiler` → CDN (`blocked_sources.φ:70-72`).

### Candidates-Pools — 34 stale Kandidaten-Zeilen disponieren
- **Status:** offen | **Bindung:** eigen
- **Lage:** Re-Messung 2026-09-21: alle genannten Survivors bereits disponiert
  (maia `finals.all` + `fireball.api` in `sources.φ`; cad/rapid/tsr.lol decline,
  e-callisto/coralreefwatch dead). Die 34 offenen `candidate`-Zeilen der 6
  Katalog-Pools sind URL-Varianten bereits disponierter Quellen.
- **Blockade:** Schreib-Grenze des Atoms (nur `sources.φ`/`ledger.φ`).
- **Braucht:** die 34 `candidate`-Zeilen in `phi/pipeline/catalog/*.φ` disponieren
  (declined/dead-Zuordnung), damit `register_lookup --open` 0 Kandidaten zeigt.

### EPA RadNet — FRS-Programm-Akronym ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `blocked_sources.φ:74-76`; FRS `get_facilities` flappt 503/200
  (Rate-Limit 6/min); Parameter `pgm_sys_acrnm` wird verworfen, akzeptiert
  `program_name` (RadNet leer).
- **Blockade:** FRS-Programm-Akronym ungemessen.
- **Braucht:** FRS-Programmliste messen → `get_facilities?program_name=<RadNet>`
  bzw. spatial search → lat/lon gegen `ERM_LOCATION` joinen.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:14`; `/tap/tables` HTTP 500, Backend down.
- **Blockade:** Pithia-DB (dienstseitig).
- **Braucht:** Re-Messung bei 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:26`; Portal 200, keine neue Anleitung.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### SuperDARN RAWACF
- **Status:** wartend | **Bindung:** dritter (Globus/PI)
- **Lage:** FITACF gebaut + registriert (`sources.φ:9464`, Asset 200/74 MB);
  nur full-res wide-beam RAWACF bleibt Globus+PI.
- **Blockade:** Globus-Gruppe/PI-Freigabe.
- **Braucht:** Globus-Invite (`blocked_sources.φ:21-24`).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten, BepiColombo, NRS02-10/12/13 SHAPE,
  EMODnet HFRADAR NADR (Re-Messung fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf: die Routine-Agenten-Klasse ist geschlossen (flash-Sieger,
  `tools-map.md`). Candidates lief `grind-pro` (Force-Gate, ~$0.16 gemessen),
  DEMETER `grind-flash` — keine flash/pro-Paarung auf derselben Aufgabe, kein
  neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `phi/sources.φ`, `phi/blocked_sources.φ`,
  `phi/pipeline/ledger.φ`, neues Handover `handover-2026-09-21-ernte-folge134.md`,
  Move `handover-2026-09-21-ernte-folge133.md` → `archiv/`.
- **Fremd (nicht angetastet, nicht committet):** `docs/handover/post.md`
  (river/future ci-check-Nachrichten).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
