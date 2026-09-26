<!--
  title: Handover — Mountain-Folge 175 (Stand 2026-09-27)
  session: Mountain-Folge 175
  class: handover
  date: 2026-09-27
  sha256: 7c8b7871a393bca4dc9de89522a3109e749babfbb8429b7c25000a57ed1742d8
  status: live
-->
# Handover — Mountain-Folge 175 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Jeder Punkt aufgeschlüsselt: Trigger / Lage / Blockade /
Braucht.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — Quelle auf ALMA-inklusiven `sgra`-Pass gewechselt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn`-Lauf **36276702701** (queued, ausgelöst 2026-09-27 via `gh workflow run eht-uvfits-cdn.yml`); bei success die Archiv-sha256 und die echte AA–AP-`df` aus dem Vollauf lesen.
- **Lage:** (gemessen 2026-09-27 via `sfetch`/`curl`/Listing `2016.1.01404.V/`) die `na`-Pässe (u.a. `e17a10-7-hi-na-1921-293`) tragen kein AA — `.pclist`-Header `AA AP AZ LM PV JC SM SR SP`, AA-Spalte leer; der feste `--pair AA AP` schlug daher strukturell fehl (Lauf 36272668544, „no AA-AP beat member"). Die `sgra`-Pässe sind ALMA-inklusiv; `e17b06-7-hi-sgra-J1924-2914-fits.tgz` (3,9 G, FITS-`ANNAME` = AA, AP, AZ, JC, LM, SM, SP, SR) ist die kleinste hi-Band-Quelle. `.github/workflows/eht-uvfits-cdn.yml:24-31` auf diesen Dateinamen/URL umgestellt (neue URL HTTP 200), `--pair AA AP` unverändert; kein Architektur-Akt (Feldnamen `alma_aa_phase`/`apex_ap_phase` bleiben). (gemessen 2026-09-27 via `ci_manage list`) Lauf 36276702701 queued; ein älterer Lauf 36276234178 (in_progress, an `cfdf84bfb`) läuft parallel.
- **Blockade:** keine.
- **Braucht:** bei success die Archiv-sha256 und die echte AA–AP-`df` aus dem Vollauf in `phi/sources.φ` (Block um `:7952`) nachtragen (nicht pollen — Lauf-Id steht).

### P5 CI-Verify + GIO2 am neuen HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` **36276394710** am HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) HEAD ist inzwischen `6659f9366` (river-38, fremde Linie; `origin/main == HEAD`). `register-coverage` @ HEAD **success** (36276394760) — der Orphan-Fix hält; `ci-check` @ HEAD **pending** (36276394710, erstellt 22:28:36); **keine Fehlläufe am HEAD** (der alte `ci-check` 36273526554 hing an `517d587e`). GIO2 `galileo-ionocal-cdn` 36272046933 success.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36276394710` nach Abschluss (nicht pollen); prüfen, dass `ci-check` grün wird.

### P6 Sony RX100 V Luminanz — CI-absent-tolerant, K-Beschaffung auf LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort 2026-09-27 — K wird erst nach Förderung beschafft (`## Operator-Wort-Register`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) `harvest` **success** (36272685581) — der absent-Arm (`no camera answered M-SEARCH`, Compiler exit 0) greift; `phi/harvest.φ:233` `asset fehlt` bleibt wahr.
- **Blockade:** keine (bis zur Förderung).
- **Braucht:** Förderung; danach K-Beschaffung + K-Messung gegen kalibriertes Luminanzmeter (river), dann K verdrahten (`freq`/`bin_width`, `river-folge38.md:109-114`).

### UI-Chat-Stimmen zum `epochrange`-Befund (vier von fünf)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort 2026-09-27 — derzeit nicht gebraucht.
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `claude.ai`, `chat.deepseek.com`, `kimi.ai`, `arena.ai` stehen an Login-/Consent-Wänden; `chat.z.ai` (GLM-5.3-Flash) lief und trägt `descoped` (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`).
- **Vorbereitung (eigen):** (gemessen 2026-09-26) die Prompt-Datei liegt bereit; dieselbe Datei geht an alle vier Sessions.
- **Blockade:** keine (derzeit nicht gebraucht).
- **Wort:** UI-Stimmen derzeit nicht gebraucht (zuvor: externe Stimmen über den Voice-Runner fragen, nicht chatgpt.com) | 2026-09-27 | Operator (Session).
- **Braucht:** nichts bis zum Operator-Wort.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung ("on hold") | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
