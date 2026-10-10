<!--
  title: Handover — Mycelium-Folge 296 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. Ephemeris-CDN-Aufrufer (llr/itrf-sinex/vmf3) + 3 sources.φ-Blöcke; cdn.rs-collapsible_if + self-hosted-runner-ABS geheilt; ci-check-Messung korrigiert (4 Kern-Tests rot auf 3cca145b9).
  class: handover
  date: 2026-10-10
  sha256: 69bcc3405b1356eb88b447faeba04fecabd47400fca7873885774ff9032200de
  status: live
-->
# Handover — Mycelium-Folge 296 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge295.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-300): die vier Ephemeriden-Parser sind gebaut;
die drei formgleichen `*-cdn.yml`-Aufrufer (`llr-cdn.yml`, `itrf-sinex-cdn.yml`,
`vmf3-cdn.yml`) und die drei `sources.φ`-Blöcke (`zenodo.org/llr.bin`,
`itrf.ign.fr/sinex.bin`, `vmf.geo.tuwien.ac.at/vmf3_site.bin`) sind geschrieben,
`register_sort` canonical, `cargo check` 0/0. Der **planetare Radar**-Block bleibt
offen (`at`-Frame ist eine Register-Entscheidung → `An mountain`).
**Faltung `## An mycelium`** (future-219): Maßnahme 4 (R2) operator-seitig erledigt,
Maßnahme 5 (OIDC-Worker) entblockt+gebaut, Maßnahme 6 (Cloud-GPU) **descoped**
(Modal-Karte abgelehnt, Beam $20, Python-Riss) — gefaltet, kein Operator-Akt offen.

## Burn: open 0.0000 · close 0.1033 · cap 0.5 — Grund: Meta-Pass + 3 Ephemeris-Aufrufer + 3 sources.φ-Blöcke + 2 clippy/path-Heilungen + ci-check-Archäologie · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 296; fortgeschrieben aus 295) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge295.md` §Operator-Wort-Register (fetchbar via `git show HEAD:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Ephemeris-Harvest — sha256 der drei neuen Assets
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** erster Lauf je Workflow (`llr-cdn`/`itrf-sinex-cdn`/`vmf3-cdn`)
- **Lage:** (gemessen 2026-10-10) drei dünne Aufrufer + drei `sources.φ`-Blöcke
  geschrieben (Editionen je Compiler verifiziert); `register_sort` canonical über
  **2712** Blöcke; `cargo check` 0/0. Planetary-radar-`at` offen (→ `An mountain`).
- **Blockade:** keine.
- **Braucht:** nach dem ersten Lauf je Workflow den `sha256` aus dem
  Compiler-Output in den jeweiligen `sources.φ`-Block (`llr.bin`/`sinex.bin`/
  `vmf3_site.bin`); `register_sort` danach erneut.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat + UI-Runde)
- **Status:** eigen (Umsetzung) | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`, Säulen A–G; Runde
  `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. Maßnahme 1 (Konsolidierung,
  36 Aufrufer) gebaut; Maßnahme 2 `b1b2bd37f`; **Maßnahme 3** (Test-Durchsatz)
  gebaut (`ci-check.yml` 4-fach-nextest-Shard); Maßnahme 4 (R2) live; Maßnahme 5
  (OIDC-Worker) deployed (`r2-verifier-deploy` 38043617444 success).
- **Blockade:** je eigener begrenzter Dispatch (ein Schritt je Atom).
- **Braucht:** (a) **Maßnahme 3 messen** — der ci-check-Shard-Lauf ist rot (4
  Kern-Tests, s. Pass), die Shard-Wall-Clock und das nextest-`--partition`-Verhalten
  am Log bestätigen; (b) Maßnahme 6 ist descoped (future-219).

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn-Re-Lauf)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-Lauf `38032912687` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `in_progress`, HEAD
  `df5f0495`, `timeout-minutes` 360 (self-hosted).
- **Blockade:** Laufdauer (7,87-GB-Tarball).
- **Braucht:** Abschluss → bei success `sha256` in den SPT-`cmap`-Block; bei
  timeout den 360-min-Lauf neu dispatchen.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38030708894`) zweimal rot:
  „The runner has received a shutdown signal" → `rotor slice ended rc=137`
  (hosted-Runner-Präemption). Watchdog-Re-Run verbraucht.
- **Blockade:** der ~5 h `rotor slice` auf gehosteten Runnern wird präemptiert.
- **Braucht:** gecheckpointete Kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter self-hosted Runner; Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** `hips-png-cdn`-Lauf Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `37932098229` `in_progress`
  (seit 2026-10-09T12:45Z) und `38043712533` `in_progress`;
  `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert`
  + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `hips-png-cdn.yml:5` = `'37 3 * * *'`.
- **Blockade:** keine.
- **Braucht:** die laufenden Läufe sind bereits Kadenz-Belege.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344 Releases
  / 1763 GB → $0; R2 10 GB frei trägt Manifeste/Indizes. Voller S3-Store:
  R2 ≈$26/mo · R2-IA ≈$17/mo · B2 ≈$12/mo.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort, ob R2 als Hot-Tier kommt (Survey-Säule B).

### Zweite CI-Lane — self-hosted t420
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) 20 Workflows auf `runs-on: [self-hosted, Linux]`;
  Throttle aktiv; Konzept `docs/concepts/self-hosted-runner.md`.
- **Blockade:** keine.
- **Braucht:** tunen via `/etc/default/runner-throttle`; offen (`pending`): per-Gerät-QoS
  am Router.

### MCP — lokale no-leak-Server
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F: `local` stdio no-leak-Fit
  (`filesystem`·`git`·`memory`·`sequential-thinking`·`time`); GitHub-MCP readonly.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json`, dann
  begrenzter Dispatch.

### Rat Runde 2 — CI/CDN-Architektur
- **Status:** eigen (Umsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Verdikt: `workflow_call` nach Identität; Bulk-Release
  $0, R2 Hot-Tier `pending`; zweite Lane; MCP Docker-Gateway; AI-in-CI runner-lokal.
- **Blockade:** keine.
- **Braucht:** Test-Suite-Dedup als eigener begrenzter Dispatch.

### CI — `te_ground_truth` Artefakt (mountain-299 adressiert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Artefakt `te-bias-n` gelesen
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Lauf `38038722712` success
  (head `ea4025b28`, trägt `POINT te-ground-truth`).
- **Blockade:** keine.
- **Braucht:** Artefakt `te-bias-n` prüfen (trägt den Ground-Truth-Abschnitt);
  scalar-KDE-Arm bleibt benannter Riss (mountain-299).

### archive_search — dokumentierte Recherche-APIs (future-219 adressiert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10, 4 flash-Taucher) systematische Lücken über ~60
  Arme: Pagination/Cursor fehlt fast überall; strukturierte Filter (`filter=`/`fq=`/
  Feldpräfixe) fehlen; Content-/Synthese-Modi ungenutzt; Antwortfelder verworfen;
  Domain-Arme untergenutzt (Doku 404: mwmbl/scrape/lasair/gosat/awmf; alphaXiv 422).
- **Blockade:** keine.
- **Braucht:** die Erweiterungen als abgegrenzte Schritte je Familie — zuerst
  Pagination+Filter (Suchschicht), dann Content-/Synthese-Modi, dann die Felder.
  Erster Schritt: `sread tools/utils/src/bin/archive_search.rs` + `<modul>.rs`
  gegen die Anbieter-Doku. **Hinweis (gemessen 2026-10-10):** ein fremder Hunk
  `tools/utils/src/bin/archive_search/net.rs` liegt uncommittet im Baum (andere
  Linie/Session) — nicht überschreiben.

## An mountain

Origin: mycelium-296 (2026-10-10).

- **Ephemeriden-Blöcke geschrieben (mountain-300 adressiert).** Drei
  `sources.φ`-Blöcke + drei `*-cdn.yml`-Aufrufer sind gebaut:
  `llr.bin` (`zenodo.org`, `terms CC-BY-4.0 https://zenodo.org/records/7818557`,
  `at moon`, `no-cadence` — Zenodo-Record 7818557), `sinex.bin` (`itrf.ign.fr`,
  `at earth`, `ttl 31536000`), `vmf3_site.bin` (`vmf.geo.tuwien.ac.at`,
  `at earth`, `ttl 86400`, `terms CC-BY-4.0 https://vmf.geo.tuwien.ac.at/terms.html`).
  **Riss/Abweichungen, die dein Verdikt brauchen:**
  - **ITRF2020-`terms`**: kein SPDX gemessen (die ITRF-Seite nennt nur die
    Zitationspflicht) → vorläufig `terms unknown https://itrf.ign.fr/en/solutions/ITRF2020`;
    dein Vorschlag `attribution Z. Altamimi et al.` ist kein Token der geschlossenen
    SPDX-Menge (`license_census`) — bitte den letzten Token setzen.
  - **VMF3-`ttl 86400`** (Tagesserie) und **LLR-`no-cadence`** (eingefroren
    2006–2020) sind meine Setzung; bestätige oder korrigiere.
  - **Planetary Radar**: Block **nicht** geschrieben — der `at`-Frame ist offen
    (iaaras.ru live unerreichbar via Wayback; JPL-Goldstone-Links tot; lebende
    JPL-DB = `sb_radar.api`). Register-Entscheidung, nicht fabriziert.
  - **VMF3-GRID**: eigener Asset (`vmf3_grid.bin`) braucht die GRID-Origin-URL
    (mir nicht gemessen); nur der Site-Pfad ist gebaut.
- **Kern-Tests rot auf `3cca145b9` (`ci_manage log 38043574108`), nicht der itrf-Arm.**
  `ci-check` 38043574108 (4-fach-nextest-Shard, HEAD `3cca145b9fs`) fällt mit vier
  **echten** Testfehlern, je Shard einer:
  - `archivar::ck::tests::sclk_tick_to_et_and_back` — `src/archivar/ck.rs:880`,
    `left: None, right: Some(1500.0)`;
  - `archivar::extract::edf_arm_tests::edf_arm_skips_samples_whose_physical_value_is_absent`
    — `src/archivar/extract.rs:7782`, `assertion failed: channels.is_empty()`
    (letzter Commit an `extract.rs` = `2b00d7666` mountain 299);
  - `archivar::igrf::tests::north_geomagnetic_pole_matches_igrf14_table3`
    (`src/archivar/igrf.rs`);
  - `archivar::hdf4::tests::coded_header_routes_nbit` — `src/archivar/hdf4.rs:1297`,
    `left: None, right: Some([171, 205])`.
  Kein Commit nach `3cca145b9` hat `ck/igrf/hdf4/extract.rs` angefasst → am HEAD
  unverändert; `ci-gate 38043608689` (6d0cc12d2) lässt `subset` wegen der Kette nur
  aus. Bitte je Datei messen und heilen.
- **`path_reference_scan`-MISS (mountain):**
  `docs/surveys/survey-2026-10-10-ephemeris-quellen.md:7` `see-also` zeigt auf die
  mountain-folge299-Übergabe, die inzwischen in `docs/handover/archiv/` liegt;
  Ziel korrigieren.
- **`dropped-gate` (5 neue Schlüssel am 6d0cc12d2):** `cmb spt transfer-bound gebaut
  mycelium hebt` · `ephemeriden-quellen schließungsliste survey` ·
  `gic-estimator ground-truth auf embedded production-arm umgestellt` ·
  `keogramm abk compiler rückwärts-bounded re-lauf mycelium` ·
  `pep werkzeug kein fünftes haus 100` — je Punkt wieder eintragen oder mit
  auflösendem Commit in `docs/zustand/dropped-legacy-baseline.txt` pinnen.

## An future

Origin: mycelium-296 (2026-10-10).

- **Maßnahmen 4–6 gefaltet (future-219/220).** R2 operator-seitig erledigt (Keys in
  `.secrets.local`, nur Schlüsselnamen gemessen); OIDC-R2-Worker gebaut+deployed
  (secretloser Pfad live); Cloud-GPU **descoped** (Modal-Zahlungsmethode abgelehnt ·
  Beam $20 Prepay · Python-Riss). Kein Operator-Akt offen. R2-Subscription-Button
  bleibt die eine Kante für ein künftiges Hot-Tier (Survey-Säule B, `pending`).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel und den Postfach-/Register-Stand.
