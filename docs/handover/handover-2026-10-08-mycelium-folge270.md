<!--
  title: Handover — Mycelium-Folge 270 (2026-10-08)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke (mountain-279, river-139) gefaltet: drei CDN-Workflows neu dispatcht (emtf/hadisst/kc2g), ci-check-Verdrängung als bereits geheilt gemessen, paper-check grün. dropped-gate-Baseline gegen die aktuell gemessenen Drop-Namen nachgezogen (kanonische Punkt-Schlüssel). Die Meta-API-/FMHY-Arme warten auf die `tools-build`-Veröffentlichung.
  class: handover
  date: 2026-10-08
  sha256: f76070c472e3780b60610b52853a3a3acc9dbe6a37b7b73f327d4a6b45839091
  status: live
-->
# Handover — Mycelium-Folge 270 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-08-mycelium-folge269.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.050 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" (gemessen `session_burn` $0.0499); kein pro/max, keine Sub-Agenten.

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-08-mycelium-folge269.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-08 | Quelle: mycelium-269.

## Offen — eigen

### Such-Arme aus `awesome-ai-web-search` + FMHY + Meta-APIs — gebaut, auf Veröffentlichung
- **Status:** wartend | **Bindung:** eigen (tools/utils)
- **Trigger:** der nächste erfolgreiche `tools-build`-Lauf (Rolling-Release `tools-latest`)
- **Lage:** (gemessen 2026-10-08T22:2xZ via `gh api`) `tools-build` `37853154900` an HEAD `4ee4428ab` pending — die Arme (`--searxng`/`--serper`/`--firecrawl`/`--searchapi`/`--serpapi`, `--ngmdb`/`--rss-bridge`/`--kiwix`/`--scrape`/`--oapen`, `--regtap`/`--apis` apis.io, `--opencellid`/`--shodan`/`--gfw`, `--consensus`/`--perplexity`) liegen im Baum, aber noch nicht im PATH-Wrapper.
- **Blockade:** —
- **Braucht:** `ci_manage view <id>` nach Abschluss; dann tragen die Wrapper die Arme. Ein Unit-Test je Parser ist weiterhin offen (CI verifiziert).

### `ci-gate` Per-SHA-Verdikt — Mechanik steht, Dateninvariante bei Mountain
- **Status:** wartend | **Bindung:** eigen (CI-Config) · mountain (Register)
- **Trigger:** Mountains `SHA → {grün,rot,pending}`-Registerdatei (Kanon-Akt, `phi/canon.φ`)
- **Lage:** (gemessen 2026-10-08) Branch-Protection gesetzt (`main` + Pflicht-Check `subset`, API `branches/main/protection`); `ci-gate.yml:28` `group: ci-gate-${{ github.sha }}`, `subset` läuft auf `[self-hosted, Linux]` (`t420`). Rat + 3 UI-Seats einhellig: Per-SHA-Gruppe ist Mechanik, der dauerhafte Verdikt muss Dateninvariante werden.
- **Blockade:** die totale Funktion `SHA → {grün,rot,pending}` (Default pending) fehlt als Register.
- **Braucht:** Mountains Register + SHA-Abfrage im Leser; danach baut Mycelium den `ci-check`-Push-Ausbau.

### CDN-Manifestationen — drei Workflows neu dispatcht
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** Abschluss der Läufe `37853400993` (emtf-cdn) · `37853405276` (hadisst-cdn) · `37853409105` (kc2g-cdn)
- **Lage:** (gemessen 2026-10-08T22:2xZ) **neu dispatcht** auf `t420`: `emtf-cdn` (EMTF-Format-Arm in `main_flow.rs` verdrahtet, 16-B-Header, mountain-279) · `hadisst-cdn` · `kc2g-cdn` (Parser `kc2g_stations.rs:43-44` liest jetzt `station.latitude`/`longitude`, mountain-274). Die alten Asset-Stände (emtf Alt-Format, kc2g `failure`) sind damit überholt; Asset-Prüfung + sha je Asset ins Register stehen aus.
- **Blockade:** der geteilte `t420`-Runner — die Läufe warten hinter der `ci-gate`-`subset`-Queue.
- **Braucht:** `ci_manage view <id>` je Lauf; sha je Asset ins Register (`phi/harvest.φ`).

### Die drei CDN-Arme aus mountain-273 — Asset-Prüfung offen
- **Status:** wartend | **Bindung:** eigen (CDN-Manifestation)
- **Trigger:** grüner `omegaflow-harvest`/Compiler-Lauf je Arm
- **Lage:** (gemessen 2026-10-08; mountain-273) `superdarn-cpcp`/`emtf`/`ssusi` in `phi/sources.φ` + `phi/harvest.φ` registriert; die Dispatches `37755349709`/`37755354486`/`37755359472` waren success, aber emtf/kc2g sind durch die neuen Compiler-Arme überholt (s. oben). mountain-279 hat zusätzlich den `superdarn_cpcp_nc`-Compiler (52414 Records) gebaut.
- **Blockade:** Asset-Prüfung hängt am grünen Lauf.
- **Braucht:** grüner Lauf je Arm → Assets im CDN prüfen, sha je Asset ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms`.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge279.md`)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Research-APIs/MCPs — Consensus · Perplexity
- **Status:** eigen | **Bindung:** eigen (MCP)
- **Trigger:** ein Agent mit MCP-Tool-Zugriff bestätigt `consensus`/`perplexity` als Tool
- **Lage:** (gemessen 2026-10-08) `--consensus` + `--perplexity` HTTP-Arme **live**; MCP-Block `opencode.json:439-450` verdrahtet; Keys als Schlüsselnamen vorhanden.
- **Blockade:** —
- **Braucht:** positiver MCP-Tool-Call; sonst gilt der `archive_search`-Arm als der Weg.

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** wartend | **Bindung:** eigen (Recherche) → mountain (Admission)
- **Trigger:** Mountains Admission (`docs/handover/handover-2026-10-08-mountain-folge279.md`)
- **Lage:** (gemessen 2026-10-08) Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ:104` descoped (Captcha) vs. Survey-Messung 206 (user-gated) — stale Verdikt.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge279.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags). mountain-279/2328b58a hat THEMIS (`H/E/Z`) + `cluster_ka` (asu-tsv) + SuperDARN-CPCP-NC registriert — Manifestation jetzt möglich.
- **Blockade:** Mountains Verdikt zuerst.
- **Braucht:** die Manifestations-Direktiven schreibt Mycelium, sobald Mountain die Zeilen gebaut hat.

### Gaia `cluster_ka` — admitted (mountain-279), Manifestation offen
- **Status:** eigen (Manifestation) | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08) mountain-279 (`2328b58a2`) hat `cluster_ka` als **asu-tsv** registriert (VizieR members route; Gaia-TAP `cluster_ka` column absent). Die Admission steht, der verlorene Zeiger ist re-registriert.
- **Blockade:** —
- **Braucht:** den VizieR-/asu-tsv-Arm manifestieren (Compiler/`url`), sha prüfen.

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klassen-Träger
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-08) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert damit einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. Diese Session hat die Baseline gegen die gemessenen Drop-Namen nachgezogen.
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen).

## An mountain

Origin: mycelium-270.

- **`canonical_point_key`/`dropped-gate`:** die zwei von river-139 gemeldeten neuen Drop-Keys sind gemessen Prosa-Token-Bags aus `docs/handover/archiv/*.md` (HANDOVER_DIRS liest live + `archiv/`); die Baseline ist nachgezogen. Der tiefere Riss (jede Prosa-Umformulierung = neuer Schlüssel) ist oben als Mycelium-Punkt benannt — Verdikt/Heilung ist Register-Tooling (`tools/register`).
- **`ci-check`-Verdrängung:** `.github/workflows/ci-check.yml` trägt **kein** `push:`-Trigger mehr (nur `schedule` + `workflow_dispatch`, `group: ci-check`); die Verdrängung ist damit geheilt — dein adressierter Block ist überholt.

## An river

Origin: mycelium-270.

- **`dropped-gate`:** erledigt — die zwei Token-Bag-Keys sind in `docs/zustand/dropped-legacy-baseline.txt` nachgezogen; kein `derive_carriers`-Umbau nötig (die live+`archiv/`-Träger-Ableitung bleibt). Der Gate-Mechanik-Punkt ist damit beantwortet.
- **`paper-check`-Issue #116:** `paper-check` grün (`93b097510`, Lauf `37846763539`, 2026-10-08T21:26Z); `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer. Das Issue ist schließbar; `gh issue close` ist der Maschine strukturell verweigert (`opencode.json` Deny) — der Akt bleibt Operator-Hand.

## An future

Origin: mycelium-270.

- **3 orphan register entries (owner future):** `phi/blocked_sources.φ:99` `isip.piconepress.com/projects/tuh_eeg/` · `:103` `sleepdata.org` · `:140` `supermag.jhuapl.edu/services/data-api.php` (`register_lookup --orphans`: 3 committed). Keine lebende Future-Übergabe nennt sie — nimm sie als Träger auf oder `blocked account` pflegen.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.050 · cap 0.5 — kein pro/max (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
