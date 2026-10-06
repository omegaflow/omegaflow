<!--
  title: Free-Voices — der kostenlose Recherche-Schwarm
  class: concept
  date: 2026-10-02
  sha256: 4fa2e75a03fb947c77d40baa6dc41e24892eb1afd5d587723bc88ad8a66311b8
  status: live
  see-also: docs/concepts/tools-map.md docs/concepts/tool-forms.md state/stimmen/reviewer-roster-2026-10-01.md
-->
# Free-Voices — der kostenlose Recherche-Schwarm

**Was wir können** (gemessen 2026-10-02, Mycelium): die freien LLM-Modelle laufen als
**`voice`-Rolle** und **messen selbst** das öffentliche Netz über
`./bin/archive_search_public` — kein lokaler Baum, kein Register, keine Secrets, kein
edit, kein Playwright. Ihre Ausgabe ist ein **Claim** (URL + HTTP-Code + Datum); die
Session prüft ihn am Baum gegen, bevor etwas ins Register geht.

## Aufruf (ein Modell je Lauf)

```
opencode run --pure -m <provider/model> --agent voice "<frage>"
```

`--pure` (ohne externe Plugins) + `--agent voice`; die Rolle bringt die
`archive_search_public`-Allowlist mit. Für die Web-UI-Stimmen (claude, z.ai, chatgpt,
qwen, deepseek, kimi, arena, grok, mistral, duck, together, tryingopen — Composer +
Send-Trigger in `state/stimmen/README.md`) das Rezept in `state/stimmen/ui-stimme.js` +
`state/stimmen/2026-10-01_frontier-kanaele-ohne-login.md`; für rohe API-Stimmen ohne
Tools `state/stimmen/stimme.sh`.

## Roster (gemessen)

- **Trägt:** `nvidia` (nemotron-3-super/ultra/nano, kimi-k3), `google`
  (gemini-3.x-flash/-flash-lite; `gemini-2.5-flash` ist pensioniert), `kilo`,
  `openrouter`, `deepseek`.
- **Tot / meiden:** `groq` — 8k TPM < ~37–41k Agenten-Kontext (alle 4 Modelle);
  `nvidia/z-ai/glm-5.3` hängt (nvidia-GLM-Endpunkte); `gemini-3.8-flash` → „Requests
  ending with a model turn". Roster-Quelle: `tools/measure/free_models.tsv` +
  `state/stimmen/reviewer-roster-2026-10-01.md`.

## Grenzen

- **Kontext:** ein Agenten-Lauf trägt ~37–41k Token (AGENTS + Rollen-Prompt + Tools).
  Das ist die TPM-Untergrenze — Groq fällt genau daran.
- **Kadenz:** **1 Frage je Lauf** ist der tragfähige Modus (7/7 sauber). **8 Fragen in
  einem Prompt überfordert die meisten Stimmen** (gemessen 2026-10-02: von 29 nur 2
  vollständig, der Rest am 360-s-Cap geschnitten — viel echte Messarbeit, kein
  Abschluss).
- **Status-Falle:** ein Statusfeld `tpm-limit` feuert, sobald *irgendwo* im Output ein
  Provider-Limit auftaucht — auch wenn danach eine gültige Antwort kam. Verlässlich ist
  nur der komplette Antworttext, nicht das Flag.
- **Read-only:** die Stimme kann keine lokalen Dateien/Register/Secrets lesen, kein
  Playwright — eingeloggte/Key-Routen bleiben bei einem DeepSeek-Agenten bzw. Operator.
- **Kein Verdikt:** jede Stimme liefert Rohmaterial (like a witness), nie die Messung.

## Voice-Swarm — `arch`-Modus, Synthese-Gate, Kern-Roster

Das **dritte Bein** neben den flash-Tauchern und dem Council: N unabhängige freie
Stimmen auf **eine** Frage, dann ein Synthese-Gate. Kanon und Details:
`state/mycelium/voice-swarm.md`; Wrapper `state/mycelium/voice-swarm.sh`.

- **`--mode research`** — die `voice`-Rolle misst über `./bin/archive_search_public`
  (`--verdict/--sniff/--wayback/…`); 1 Frage je Lauf (8 überfordern).
- **`--mode arch`** — dieselbe Rolle **ohne Tool-Zwang**: die Frage ist ein
  Design-/Kritik-Prompt; die nötigen Fakten stehen **im Prompt eingebettet** (die
  Stimmen lesen den Baum nicht). Ausgabe: Design-Positionen, kein Routen-Claim.
- **Kern-Roster** — `state/mycelium/voice-roster-core.tsv`: **24 deduplizierte
  Stimmen**, **eine Route je Modell** (unkorreliert; die 85 Dropdown-Modelle tragen
  viele Spiegel desselben Modells über kenari/kilo/openrouter). Quelle
  `tools/measure/free_models.tsv`.
- **Synthese-Gate:** (1) nur belegte Zeilen (research) bzw. Design-Positionen (arch)
  sammeln; (2) Top-Claims am Baum gegenmessen (`archive_search --verdict/--sniff`,
  `git`/Register); (3) `@council` oder ein starkes Modell wägt ab — Riss benennen,
  nie glätten; (4) Verdikt/Register trägt allein die Session.
- **Agent-Route:** die gemessenen Stimmen sind als read-only Subagenten definiert
  (`voice-<name>`), ein Schwarm = eine Nachricht mit N `task`-Calls — kein
  `opencode run`-Prozess-Fan-out (schont die Maschine).

## Architektur-/Ethik-Adressierung (Operator-Wort 2026-10-06)

Architektur-/Ethikfragen werden mit **den fünf Stimmen und den vier Axiomen** addressiert —
gleichermaßen für **UI-Chats** (`state/stimmen/ui-stimme.js`, Prompt
`state/stimmen/prompt-arch-ethik.txt`; die fünf arbeitenden UIs stehen in `state/stimmen/README.md`)
und **API-Modelle** (die `voice-*`-Subagenten, das Basis-`voice`-Rollenprompt in `opencode.json`
und der arch-PROLOG von `state/mycelium/voice-swarm.sh --mode arch`). Die Stimmen liefern je Frage
ein Verdikt aus den fünf Stimmen und tragen die vier Axiome; das Verdikt/Register trägt allein die
Session.

## Dateien

- `state/mycelium/voice-run-all.sh` — Roster-Fan-out (ein Lauf je Stimme, Erreichbarkeit
  in `reachability.tsv`). `state/mycelium/voice-roster.tsv` — die Modell-Liste.
- `state/stimmen/` — Roh-Outputs (`voice-all-<ts>/<provider>__<model>.json`), Synthesen,
  Reviewer-Roster.
- `.opencode` Rolle `voice` (read-only, `archive_search_public`).

## Kampagne 2026-10-02 (Muster)

8 offene Quellen-Routen wurden über 29 Stimmen sondiert. Bestätigte Leads (Session am
Baum gegengeprüft, `archive_search --verdict`):

- **M3-ENVI-Spiegel:** Wayback `…/web/20160603190919/http://pds-imaging.jpl.nasa.gov/data/m3/CH1M3_0003/DATA` 200; ODE `ode.rsl.wustl.edu/…/indexProductSearch.datasetFiles.aspx` 206; SBN `sbn.psi.edu/` 206; `pds-imaging.jpl.nasa.gov/volumes/m3.html` 206 (nennt die Spiegel).
- **MESSENGER 2005:** SPDF `spdf.gsfc.nasa.gov/pub/data/messenger/` 206; NAIF `naif.jpl.nasa.gov/pub/naif/MESSENGER/` 200; WUSTL `…/missions/messenger/` 206 (2005-spezifisch bleibt offen).
- **Tianwen-1 MoRIC:** `alasky.cds.unistra.fr/…/Norder6/Dir0/Npix0.png` 206.
- **Danuri ShadowCam:** `pds.shadowcam.im-ldi.com/derived/` 200 (13117 B).
- **PDS-Imaging-Spiegel:** ODE/SBN 206. **`pdsimage.wr.usgs.gov` direkt tot** — nur Wayback 1996 (Voice-Fehltreffer, nicht übernehmen).
- **Shandong:** Wurzel 206, `/data/` 404, kein `/data/`-Snapshot.
- **GOSAT-GW / BepiColombo-Pre-release:** `unbelegt`.

Das Muster: der Schwarm liefert breite, gemessene Route-Kandidaten in Minuten — die
Session verifiziert die Top-N und trägt sie als Verdikt (Mountain) bzw. `url` (Mycelium).
