<!--
  title: Handover — Mycelium-Folge 256 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass; cdn-reconcile-Lauf gelandet (stale-Report-Punkt geschlossen), ci-gate dropped-gate @af7dc14e8 (delta 2) am CI-Log gemessen und per Baseline-Bump geheilt, adressierten Mountain-258-Block am Baum gemessen gefaltet, Stehender Pass am neuen HEAD
  class: handover
  date: 2026-10-07
  sha256: 92a9037560595632048a6fee5900ff7a075158b0d70e7b2a295b7a02be19c62c
  status: live
-->
# Handover — Mycelium-Folge 256 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge255.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An future`.

## Burn: open 0.0040 · close 0.0223 · cap 0.5 — Grund: Meta-Pass, kein pro/max-Dispatch

## Operator-Wort-Register

- Wort | 2026-10-06 | „für die apis brauche ich zuverlässige ehrliche und schnelle stimmen, insbesondere zum grinden … die ui chats müssen stark sein" — API-Stimmen kuratieren (zuverlässig/ehrlich/schnell), UI-Chats = frontier | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „weiter ich habe glm und trying open /kimi gestartet" — UI-Test fortsetzen | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „es sind 5 stimmen 5 axiome 5 achsen und du kannst die entwürfe überschreiben bzw. leere chats starten" — Arch-/Ethik-Adressierung trägt 5 Stimmen (Mountain·River·Mycelium·Sensory·Future), 5 Axiome (`docs/granit.md:16-20`) und 5 Achsen (`docs/concepts/die-vier-schilde.md`) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „mir geht es darum dass du die qualität und den nutzen der chats testest" — Qualitäts-/Nutzen-Test der Stimmen (UI-Chats + API-Modelle) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „ist auch bekannt welche UI Chats funktionieren und wie sie addressiert werden sollen (bei architektur/ethik fragen mit den 5 stimmen und den 4 axiomen) gilt auch für die API modelle" | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „ich will nicht nur dass du registriert sondern auch dass du tatsächlich die confgs bearbeitest und bitte kümmer dich auch um cdn_reconciliation.json" | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „kannst du bitte die nutzlosen modelle und anbieter deaktivieren? also auch die die immer rate limited sind" — nicht aufrufbare Arme aus `free_models.tsv` austragen (`struck`) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „nein wir haben glm max über ui chat" — OrcaRouter nicht verfolgen (GLM-Route läuft über den UI-Chat) | Quelle: Operator (Session, Mycelium 244).
- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Holdings-Migration: „1 ja (move) · 2 ja (delete) · 3 ja (create) · 4 messen, dann · 5 ja (delete) · 6 ja (dedup) · 7 ja (dedup)" | Quelle: Operator-Session 2026-10-06.
- Wort | 2026-10-06 | Daten-Holdings CDN-Bedarf/Ort: „ich gebe es mycelieum" — Kriterium nicht „regenerierbar", sondern was auf den CDN muss und am richtigen Ort liegt | Quelle: future-185 addressed.
- Wort | 2026-10-06 | „Ein Dispatch = ein begrenzter Schritt" — ein Agent plant im Output-Budget; ein über-großer Auftrag wird als Sequenz begrenzter Schritte gebaut | Quelle: Operator-Session 2026-10-06, als Regel in `AGENTS.md`.
- Wort | 2026-10-06 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | Quelle: Operator (Session, Mycelium 243).
- Wort | 2026-10-06 | Own-CDN-Junk-Bereinigung: „ich folge deiner Empfehlung" — die Messung korrigierte: nur 1 stale Blatt gelöscht | Quelle: Operator (Session, Mycelium 243).
- Wort | 2026-10-06 | „ich möchte dass du den API test der vorhandenen APIs gegen diese wunschliste fährst" (volle Wunschliste §A Replikationslinien · B Mediator-Zeugen · C Driver+Konditionierer · D Unabhängige Observablen · E Kalibrier-Zeuge · F Zielkanal · G Placebo-Null) — API-Reachability-Test der GIC-Faden-Wunschliste gegen das Register | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „eigentlich wollte ich dass du die llm apis nutzt um die APIs zu validieren" — die Daten-APIs über die LLM-Stimmen (zweiter Kanal) validieren; kilo vom Operator gestoppt (zu langsam) | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „nein" (auf die Frage, ob die API-Erkenntnisse an die UI-Chats gehen) — keine UI-Chats für gemessene Fakten-Endpunkte | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „mir ist nur wichtig dass wir immer besser werden und die richtigen modelle für den jeweiligen zweck nutzen" — Modell-Fit je Aufgabenklasse messen und registrieren; die passende Stufe, nicht die stärkste | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „sollen wir gemini und kilo entfernen? ich möchte wirklich nur modelle die auch etwas taugen" — Roster-Leanheit; Entfernung nur per gemessener Fähigkeit, nicht per Gefühl | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „ja dann bitte entfernen" — kilo (Latenz) und zen (Nichtantwort) aus dem Roster | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „was machen wir mit zen; zen kann nicht als agent genutzt werden aber vielleicht ist es für andere dinge nützlich" — zen als Session-Modell-Fallback prüfen, nicht als Stimme | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-06 | „bitte lies die anbieter (nur namen) aus `auth.json` und deaktiviere die nutzlosen" — Provider-Leanheit; `auth.json`-Werte bleiben gesperrt, nur Namen via `opencode auth list` | Quelle: Operator (Session, Mycelium 245).
- Wort | 2026-10-07 | „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | Quelle: Operator (Session, Mycelium 256).

## Offen — eigen

### eionet_cdr — Transportzeilen + Manifestation (Kraft-Verdikt offen)
- **Status:** wartend
- **Trigger:** Mountains Kraft-Verdikt (`diffusion` vs. `force-undetermined`-Anker) + `field`/`ttl`-Zeilen → Register-Block steht
- **Lage:** (gemessen 2026-10-07, Mountain 258 / Mycelium 255/256) Medium-Arm gebaut (`eionet_cdr::series_name`), Compiler `tools/harvest/src/bin/eionet_cdr_compiler.rs` steht (`FORMAT=eionet_cdr`, `CDN_TAG=cdr.eionet.europa.eu`); Festquelle + 92-Code-Codelist geschlossen. Am Baum gemessen (2026-10-07, Mycelium 256): `phi/harvest.φ` trägt **kein** `eionet`-Pattern, `phi/sources.φ` **keinen** eionet-Block, `eionet-cdr-cdn.yml` ist **absent** — diese drei sind mein Transport und hängen am Register-Block. Mein Teil: `url`/`origin`/`compiler`-Zeilen + Pattern + Manifestation + `eionet-cdr-cdn.yml`.
- **Blockade:** Kraft-Riss (ein anlage-verankerter Jahresmasse-Abfall ist nicht gemessen; `diffusion` fabriziert einen Gradienten).
- **Braucht:** Mountains `field`/`ttl`-Zeilen (276 = 92×3 Media); dann meine Transportzeilen + Manifestation + `eionet-cdr-cdn.yml`.

### OSHA-CEHD — Register-Zeile (Force-/Einheiten-`1`-Riss)
- **Status:** wartend
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am `1`-Riss des Force-/Einheiten-Kontrakts (Compiler emittiert Masse/Massenkonzentration/amount-fraction `1`). Bis dahin ist `obis.osha.gov` in `docs/specs/cdn-tag-baseline.txt` als gemessene Ausnahme geführt.
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann fällt die Baseline-Ausnahme und die Manifestation läuft.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255) `LICENSE`/`README` dort absent (HTTP 404 raw).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen) sind noch nicht vollständig.
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### dropped-gate — Träger statt Delta (Bau läuft; Rat + 5 API + 14 UI = 20 Quellen, 2026-10-07)
- **Status:** eigen
- **Trigger:** Operatives „los" (2026-10-07); Bau als flash-Sequenz
- **Lage:** (gemessen 2026-10-07) **20/20 konvergent:** Register-/Carrier-Frage, keine Baseline-Zahl. **Design ratifiziert:** Q1 **hybrid** — ID ist die Identität, ein Token-Treffer ist nur `match_hint`/Fallback (Qwen/Muse Glimmer: explizite ID; Ausreißer: GPT-OSS 120B will (a)); Q2 **Roster** (Menge point-IDs), Mutation nur per benanntem Ereignis, kein Auto-Bump; Q3 **hartes Gate** bei stillem/ungetyptem Move (benanntes Ereignis passiert), durchgesetzte Schranke (Hook + CI); Q4 **`pending-legacy` einfrieren**. Reihenfolge (Claude, stärkste Bindung): Nullkontrolle → ID-Feld → Snapshot → Ereignis-Vokabular → Roster-Gate → hartes Rot. **Schnitt 1 gebaut** (grind-flash, `register_lookup.rs`): `explicit_point_id` (:2329) + `canonical_point_key` (:2354, sortierte Token-Menge) + `carries`-Closure im `run_dropped`-Träger-Abgleich; 4 Tests; `cargo check`/`build -p omegaflow-register --bin register_lookup` grün, 0 Warnungen; der inert-guard (tokenlos → pending) bleibt.
- **Blockade:** Schnitt 1 uncommittet; Schnitte 2/3 hängen an der Entscheidung über die durchgesetzte Schranke (Hook) — berührt `commit_gate`/CI.
- **Braucht:** `/commit` (Schnitt 1), dann Schnitt 2 (Roster-Baseline + `pending-legacy`-Einfrieren) und Schnitt 3 (Zwei-Stufen-Gate).
- **Modell-Vergleich notiert:** `state/benchmark/2026-10-07-dropped-gate-modellvergleich.md`; Sieger **Claude Sonnet 5.5**, Ausreißer **GPT-OSS 120B**; 14 Antworten in `state/stimmen/2026-10-07_dropped-gate-design-14-ui-antworten.md`.

## An mountain

Origin: mycelium-folge256.

- **`eionet_cdr` — meine Transportzeilen warten auf deine Feder.** Dein Rat-Verdikt ist gefallen (`diffusion` unphysikalisch → `force-undetermined`-Gap, `phi/blocked_sources.φ:4`); was fehlt, ist dein Register-Write: der Block (`field`/`ttl`, 276 = 92×3 Media) + `at earth`. Danach schreibe ich `url`/`origin`/`compiler`, das `eionet_cdr`-Pattern in `phi/harvest.φ`, die Manifestation und `eionet-cdr-cdn.yml`. **Braucht:** dein `force-undetermined`-Write + `field`/`ttl`-Zeilen; dann läuft mein Transport.
- **`obis.osha.gov` — Register-Zeile wartet auf deinen `1`-Riss-Abschluss.** `.github/workflows/osha-cehd-cdn.yml` steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-`1`-Riss. **Braucht:** deine `terms`/`url`-Zeile; dann fällt die `cdn-tag-baseline.txt`-Ausnahme.
- **`LICENSE` im `omegaflow/sources`-Repo** — Mycelium erzeugt es erst nach deinen `terms`-Zeilen der register-tragenden Blöcke. **Braucht:** deine `terms`-Vollständigkeit.

## An future

Origin: mycelium-folge255.

- **API-Stimmen-Kuration (Rest, HOLD):** `dots`-Sitz durch `deepseek deepseek-flash` ersetzt; Operator-Revision „Stimmen behalten" — `gptoss`-Austrag HOLD, `gemini`/`inkling`/`nemotron` bleiben. **Braucht:** neues Operator-Wort (Roster-Dateien bleiben unverändert bis dahin).
- **Freie Frontier-Stimmen — Registrierung:** `free_models.tsv` `struck` für cloudflare/groq/sambanova/mistral/alibaba/ovhcloud/zai/orcarouter; aktiv `google`/`nvidia` (HTTP) + `deepseek`/`kenari`/`openrouter` (client). **Braucht:** `auth login`/Konten-Freischaltung (Operator).
- **GIC-Zugänge (per-Akt):** Accounts/Keys CARISMA, AMPERE, PC-Index, CDDIS-Earthdata. **Braucht:** Operator-Wort je Akt.
- **JAXA G-Portal / Sample-/Record-Downloads** (`blocked_sources.φ`): Operator-Hand (Bestellung/Fetch).
- **`ledger.φ:2`/`:6` Port-Runner:** `omegaflow --port` läuft (`main_flow.rs:732`, `port.rs:625`); die Korpus-Eingaben sind am Datenträger absent (`queue/master.φ` gitignored). **Braucht:** Korpus-Input wiederherstellen (Operator/Datenträger).

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

**Diese Session (Atome):** (a) Den gefeuerten Trigger gemessen: `cdn-reconcile`-Lauf `37551136792` = **success** @`53a11198b`, committet `docs/specs/cdn_reconciliation.json` (`42803216e`, `sources_parsed 2673`) — der stale-Report-Punkt ist geschlossen und gelöscht. (b) Den roten `ci-gate 37551637568` @`af7dc14e8` am Log gemessen: Job `dropped-gate` `baseline 1295 | current 1297 | delta 2`; die 2 sind der aufgelaufene Drop-Netto der Planungs-Pässe seit dem Bump @1295 (mountain-256…258, river-117…119, sensory-245, mycelium-255) — per Baseline-Bump auf **1297** absorbiert (`docs/zustand/dropped-baseline.md`). Der ältere clippy-Riss (`GeoRec: Debug`, `37551242041`) ist durch Mountain `ef253fa3b` geheilt (`37551637568` clippy = success). (c) Den adressierten `## An mycelium`-Block aus mountain-folge258 am Baum gemessen und gefaltet: `osm_nodes`-Pattern (`phi/harvest.φ:283`), `fink_cutout` (`phi/harvest.φ:99`), die CDN-Workflows (`jaxa-gportal`, `nasa-power-t2m`, `epa-aqs-voc`, `osm-pbf`, `fink-cutout`) und `vnp46a3-cdn.yml` (Titel `h18v07`, equatorial — DNB-Nachtdaten ganzjährig) sind vorhanden; eionet_cdr/OSHA/LICENSE bleiben wartend (s. Offen). (d) Handover fortgeschrieben, folge255 → `archiv/`; Stehender Pass am neuen `a964f2acc`/`416f7b59b`. (e) Die Wurzel des `dropped-gate`-Ratschritts dem **Rat** und den **5 API-Stimmen** vorgelegt (5 Stimmen + 5 Axiome + 5 Achsen): einstimmig Register-/Carrier-Frage, nicht Baseline-Zahl; Empfehlung „Prädikat `carrier(P)` statt Delta, `git: none` = pending, kein Auto-Bump" als eigener Punkt registriert. Die UI-/Open-Weight-Stimmen (zweiter Kanal) sind adressiert, ihr Verdikt `pending`.
