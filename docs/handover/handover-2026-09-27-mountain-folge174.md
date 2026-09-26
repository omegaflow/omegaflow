<!--
  title: Handover — Mountain-Folge 174 (Stand 2026-09-27)
  session: Mountain-Folge 174
  class: handover
  date: 2026-09-27
  sha256: d2c4c36024342511364294a9ace5d97bf3bbcdd31bf5bc9c81f764ebe5c47c85
  status: live
-->
# Handover — Mountain-Folge 174 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Jeder Punkt aufgeschlüsselt: Trigger / Lage / Blockade /
Braucht.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — Quelle auf ALMA-inklusiven `sgra`-Pass gewechselt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit`+Push, dann `gh workflow run eht-uvfits-cdn.yml`.
- **Lage:** (gemessen 2026-09-27 via `sfetch`/`curl`/Listing `2016.1.01404.V/`) die `na`-Pässe (u.a. `e17a10-7-hi-na-1921-293`) tragen kein AA — `.pclist`-Header `AA AP AZ LM PV JC SM SR SP`, AA-Spalte leer; der feste `--pair AA AP` schlug daher strukturell fehl (Lauf 36272668544, „no AA-AP beat member"). Die `sgra`-Pässe sind ALMA-inklusiv; `e17b06-7-hi-sgra-J1924-2914-fits.tgz` (3,9 G, FITS-`ANNAME` = AA, AP, AZ, JC, LM, SM, SP, SR) ist die kleinste hi-Band-Quelle. `.github/workflows/eht-uvfits-cdn.yml:24-31` auf diesen Dateinamen/URL umgestellt (neue URL HTTP 200), `--pair AA AP` unverändert; kein Architektur-Akt (Feldnamen `alma_aa_phase`/`apex_ap_phase` bleiben).
- **Blockade:** keine.
- **Braucht:** nach `/commit`+Push `gh workflow run eht-uvfits-cdn.yml`; nach success die Archiv-sha256 und die echte AA–AP-`df` aus dem Vollauf in `phi/sources.φ` (Block um `:7952`) nachtragen.

### P5 CI-Verify + GIO2 am neuen HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD nach dem Push.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) `register-coverage` **success** (36273430680, 36275819584) — der Orphan-Fix (2 mountain + 1 mycelium) hält; `ci-check` 36273526554 in_progress / 36275819606 pending am 22:18Z-Push; GIO2 `galileo-ionocal-cdn` 36272046933 success.
- **Blockade:** keine.
- **Braucht:** nach Push `ci_manage view/log <id>` am neuen HEAD; prüfen, dass `register-coverage` grün bleibt und `ci-check` die Mountain-Fixes trägt.

### P6 Sony RX100 V Luminanz — CI-absent-tolerant, K-Kalibrierung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** K-Messung gegen kalibriertes Luminanzmeter im Smart-Remote-LAN — `docs/handover/handover-2026-09-26-river-folge38.md:109`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) `harvest` **success** (36272685581) — der absent-Arm (`no camera answered M-SEARCH`, Compiler exit 0) greift; `phi/harvest.φ:233` `asset fehlt` bleibt wahr.
- **Blockade:** physische Kamera + K-Kalibrierung (River).
- **Braucht:** river-Messung; K=`12.5` ungemessen, `freq`/`bin_width` nicht verdrahtet (`river-folge38.md:109-114`).

### gll_rss_odr.manifest — Prüf-Artefakt, keine Datensatz-`url`
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** keiner — kein offener Schritt (die 13 Shards tragen ihre `sha256` selbst).
- **Lage:** (gemessen 2026-09-27 via `curl`) `gll_rss_odr.manifest` → HTTP 200; es ist die sha256-Liste der 13 Shards (`gll_rss_odr.bin.000`–`.012`), kein Datenformat — je Shard trägt die `url`-Zeile ihre `sha256` bereits selbst. Kein `url`-Präzedenzformat.
- **Blockade:** keine.
- **Braucht:** nichts.

### UI-Chat-Stimmen zum `epochrange`-Befund (vier von fünf)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator-Wort (Anmeldung in den vier Tabs).
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `claude.ai`, `chat.deepseek.com`, `kimi.ai`, `arena.ai` stehen an Login-/Consent-Wänden; `chat.z.ai` (GLM-5.3-Flash) lief und trägt `descoped` (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`).
- **Vorbereitung (eigen):** (gemessen 2026-09-26) die Prompt-Datei liegt bereit; dieselbe Datei geht an alle vier Sessions.
- **Blockade:** Login/Consent (nicht umgangen).
- **Wort:** externe Stimmen über den Voice-Runner fragen (nicht chatgpt.com) | 2026-09-26 | Operator (Session).
- **Braucht:** vier Sessions im `chrome-devtools`-Browser anmelden (Operator), dann die Prompt-Datei senden.

## Benchmark

- EHT-Baseline-Quellenwahl (2026-09-27, `grind-flash` vs `grind-pro`): `grind-flash` maß „no AA-AP member" korrekt und benannte den Urteilsbedarf, ließ aber die Datensatz-Semantik offen; `grind-pro` bestimmte die `na`/`sgra`-Pass-Konvention und den exakten ALMA-inklusiven Datensatz (`e17b06-7-hi-sgra-J1924-2914`) → **pro gewinnt** (vollständig). Burn: `session_burn`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
