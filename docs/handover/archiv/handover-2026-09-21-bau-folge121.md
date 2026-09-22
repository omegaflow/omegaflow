<!--
  title: Handover — Bau-Folge 121 (Stand 2026-09-21)
  session: Bau-Folge 121
  class: handover
  date: 2026-09-21
  sha256: d995d924565db2640ba6f7aeb81ff92596e64a982ec6eeeb3ee6e807d5edf39e
  status: live
-->
# Handover — Bau-Folge 121 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `a476ccfb` == `origin/main`.
- **CI** — Watchdog 10:11:01 + `ci_manage view`: `radnet-cdn 35573513812`
  in_progress; `glm-l2-cdn 35576757352` **failure** (diagnostiziert, siehe Atom);
  `ps1-cdn 35569486280`, `ci-check 35574757717` in_progress. Kein Poll.
- **Safety-Net** `refs/safety/1789978584`.

## Messung dieses Atoms (kein offener Punkt)

- **`archive_search --verdict` repariert** (post.md-Zeile eingelöst): die Wurzel
  gemessen — `tools/utils/src/bin/archive_search/net.rs` `curl_fetch` setzt
  `Fetch.complete = out.status.success()`, aber `--verdict` las den Flag nie. Ein
  transient gescheiterter Transfer (curl exit 28) liefert `%{http_code}` `000`
  und wurde als `HTTP 0 — absent` gedruckt. Die Post-Diagnose (Redirect-/HEAD-
  Handling auf Trailing-Slash) war ungemessen — der Trailing-Slash war nie die
  Ursache; `curl_args` sendet `-sL -g --max-time 30 <url>` unverändert. Fix:
  `curl_args` extrahiert, `VERDICT_ATTEMPTS = 3` + `is_transient`/`retry_transient`/
  `verdict_probe`; ein unvollständiger Transfer wird wiederholt und, bleibt er es,
  als `pending — no response` gemeldet — nie ein fabriziertes `HTTP 0/308 absent`.
  3 Gate-Tests. `cargo check -p omegaflow-utils --all-targets` 0 Fehler/0
  Warnungen. Live: direkt `000`/exit 28 flaky, Proton-Exit 200/5740 — Route
  gefunden.
- **radnet.bin manifestiert:** `archive_search --sniff
  …/data.epa.gov/radnet.bin` → HTTP 200, 17408568 B, sha256
  `128ac4fd695f63a2e5fd156e1b416408880577555acf9e676d0e76b242a31c34`; im
  `phi/sources.φ` radnet-Block registriert.
- **glm-l2-cdn-Fehllauf diagnostiziert:** `35576757352` failure =
  `error[E0433] cannot find module hfrnet_rtv` an `src/archivar/extract.rs:350` —
  der rote Parent `0475d67f` (bau folge120 committete die hfrnet_rtv-Referenzen
  ohne `mod.rs`/`hfrnet_rtv.rs`); der Fix landete mit ernte folge129 `2ae4978a`.
  Am grünen HEAD neu dispatcht: Lauf `35579950220` — **failure** (siehe offener
  Punkt 1: der Compiler selbst liest 0 Flashes).
- **`--searxng`-Modus gebaut** (Meta-Suche als Brave-Alternative):
  `tools/utils/src/bin/archive_search/net.rs` `searxng_lines`/`searxng_results`
  (`<base>/search?q=&format=json&language=all&safesearch=0`), Basis-URL aus
  `SEARXNG_URL`; registriert in `QUERY_MODES`/`run_lines`/`--help`; 2 Gate-Tests;
  `cargo check -p omegaflow-utils --all-targets` 0 Fehler/0 Warnungen. Gemessen:
  öffentliche Instanzen (`searx.be`, `search.inetol.net`) liefern HTTP 200 **HTML**
  statt JSON → `pending`; der Modus braucht eine JSON-fähige/gehostete Instanz.
- **SearXNG-Route mit harten Bandagen recherchiert** (Sweep über 101 Instanzen
  aus `searx.space/data/instances.json`, 2026-09-21): nur **2** liefern JSON —
  `sx.xo.st` und `search.mectov.my.id` (je 200, `results[]`). Beide live
  verifiziert: die Query wird korrekt geechot, die Treffer sind aber
  **irrelevante Fremdseiten** (Microsoft/Gmail/GitHub-Router, `engine: bing`) —
  unbrauchbar. `SEARXNG_URL=https://sx.xo.st` wurde zum Test gesetzt und wieder
  entfernt; der Modus steht auf `pending`. **Der tragfähige Weg ist eine selbst
  gehostete Instanz** (Docker `searxng/searxng`, `search.formats: [html, json]`,
  Limiter `false`), dann `SEARXNG_URL=http://localhost:8888`. Rezept:
  `docs.searxng.org/admin/settings/settings_search.html` + `installation-docker.html`.
- **`--mwmbl`-Modus gebaut** (keyless Brave-Ersatz, FMHY-Fund): offene
  Community-Suchmaschine `mwmbl.org/api/v1/search/?s=`, keyless JSON
  `[{url,title:[{value}],extract:[{value}],source}]`; `mwmbl_lines`/`mwmbl_results`
  in `net.rs`, registriert in `QUERY_MODES`/`run_lines`/`--help`, 1 Gate-Test;
  `cargo check -p omegaflow-utils --all-targets` 0 Fehler/0 Warnungen. Gemessen:
  relevante Treffer (Wikipedia/Frontiers/Nagoya), 0,78 s, kein Gate/Key.
- **Proton-IP-Frage gemessen:** der Playwright-Pfad löst JS-Challenges (Marginalia
  200, hochwertige Treffer), aber nicht die IP-Blocks — DDG: direct 202, über den
  Proton-Exit `169.150.218.57` **403**; Mojeek 403 „automated queries".
  Datacenter-IP-Reputation; ein DNS-Wechsel ändert nur die Namensauflösung, nicht
  die Quell-IP. Der Weg durch das Gate ist keyless/ungated (Mwmbl) oder residential.

## Offen (aufgeschlüsselt)

### 1. glm_l2.bin CDN-Manifestation — Compiler liest 0 Flashes
- **Status:** `blockiert` | **Bindung:** `eigen`
- **Lage:** Am grünen HEAD `a476ccfb` neu dispatcht als Lauf `35579950220` —
  **failure** (08:51:07Z). Der Compiler lief über die ~30 Granules von
  `GLM-L2-LCFA/2026/001/00/` und meldete für jede `0 flashes (N quality-degraded
  skipped)` (N meist 0, selten 1–5) → `exit 1` via
  `tools/harvest/src/bin/glm_l2_compiler.rs:434` (`no flashes harvested — the bin
  stays unwritten (0 honored)`); kein Upload. Echte GLM-L2-LCFA-Granules tragen
  hunderte–tausende Flashes je 2-min-Granule — der Parser liest zu wenige Records.
  Asset weiter `--sniff` 404.
- **Blockade:** Parser-Defekt (Record-Zahl/Dimension/Qualitäts-Gate), nicht der
  Run-Abschluss.
- **Braucht:** erste Messung — `dataset_load`/Dimension in `glm_l2_compiler.rs`
  gegen ein echtes Granule messen (Record-Zahl + `flash_quality_flag`-Verteilung);
  `grind-max` (novel parser). Danach Re-Dispatch + sha256 in `phi/sources.φ`.

### 2. PINE64 / Mantis-Shrimp (Ox64-Dokumentation)
- **Status:** `blockiert` | **Bindung:** `linie:entscheid`
- **Lage:** Ox64 von Pine64 zugesagt (Hardware beidseitig geschlossen); die
  Presence-Hardware „Mantis-Shrimp" ist ungebaut; Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen. `entscheid-folge76:97` trägt die
  Doku-Pflicht als `blockiert | linie:bau`.
- **Blockade:** die Hardware existiert nicht.
- **Braucht:** Operator-Wort — Prototyp bauen oder descopen (via entscheid-Linie);
  bis dahin kein Schritt.

### 3. SearXNG-self-host / IP-Route für Web-Suche
- **Status:** `operator-gebunden` | **Bindung:** `operator`
- **Lage:** `--searxng` ist gebaut, braucht `SEARXNG_URL`; öffentliche Instanzen
  sind unbrauchbar (101-Sweep: nur 2 liefern JSON, beide mit irrelevanten
  Fremdtreffern). `--mwmbl` (keyless, kein Gate) deckt die keyless Web-Suche
  bereits ab und läuft live. Der Playwright-Pfad löst JS-Challenges (Marginalia
  200, hochwertige Treffer), aber **nicht** die IP-Blocks — DDG antwortet über den
  Proton-Exit `169.150.218.57` mit 403 (direct 202), Mojeek 403 „automated
  queries": Datacenter-IP-Reputation. DNS (`dnsforge`/AdGuard/DNS0) ändert nur die
  Namensauflösung, nicht die Quell-IP. FMHY-Proxy-Liste (`fmhy.net/privacy`):
  3X-UI, Xray, Hysteria, sing-box, Scramjet/Nebula, v2rayN/NekoBox/Hiddify/
  ClashVerge — **keine kostenlose residential-Route**.
- **Blockade:** eine echte residential-/ungated-Route oder das Operator-Wort für
  einen lokalen Dienst.
- **Braucht:** Operator-Entscheid — (a) selbst gehosteten SearXNG starten (Docker
  `searxng/searxng`, `search.formats: [html, json]`, Limiter `false`, dann
  `SEARXNG_URL=http://localhost:8888`), (b) `--mwmbl` als Web-Suche genügen
  lassen, oder (c) eine residential-Route benennen.

## Benchmark

- **Bau-Folge 121:** `grind-flash` ×1 (archive_search `--verdict`-Fix, ~2 min);
  `--searxng`- und `--mwmbl`-Modus + die harte SearXNG-/IP-Recherche im
  `build`-Kontext. Flash-first; keine Eskalation nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/utils/src/bin/archive_search/net.rs`,
  `tools/utils/src/bin/archive_search.rs` (`--searxng`-Flag + Hilfe),
  `docs/concepts/tools-map.md` (`--searxng`), `phi/sources.φ` (radnet-sha256),
  `docs/handover/post.md`, neues
  `docs/handover/handover-2026-09-21-bau-folge121.md`, Move
  `handover-2026-09-21-bau-folge120.md` → `archiv/`.
- Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
