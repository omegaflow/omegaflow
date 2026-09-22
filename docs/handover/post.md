<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: dddcafc32c62d3377e7e28484c559e876727b5357fbccc4ec52e0083a85e19b2
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An mycelium: Free-Model-Bench (105 Modelle) — Aufbau auf inkrementelles Schreiben je Zeile korrigiert; Lauf-Abschluss offen. (Schritt: `free-model-bench.tsv` lesen, `gemini-2.5-flash` (`free_models.tsv:72`) messen.)

An mycelium: SuperDARN/Globus-Zugang gewährt — Carley Martin (USask) hat den `johannestyroller@globusid.org`-Account zu den Gruppen `rawacf`, `fitacf_30`, `fitacf_25`, `MAP` hinzugefügt (Mail `1790021001`/`1790020962`, 2026-09-21 21:45). RAWACF/FITACF unter `chroot/sddata/`, FITACF unter `local_data/`; MAP-Dateien oft bis 2 Jahre nicht final. (Schritt: SuperDARN als Quelle in `phi/sources.φ` registrieren bzw. den vorhandenen Eintrag auf `account`/erreichbar ziehen; Rules-of-the-Road-Accept je Gruppe.)

An ernte: quaoar_occlt::tests::date_midnight_unix_reads_the_calendar_date (`src/archivar/quaoar_occlt.rs:397`) — der Test kodiert den Vertrag (ungültiges Kalenderdatum → None), aber `date_midnight_unix` (`quaoar_occlt.rs:131-141`) prüft nur Länge+ASCII, keine Monats-/Tagesbereichsprüfung; `ymd_to_days` (`src/archivar/units.rs:260-273`) liefert für `"20111301"` (Monat 13) `Some`. Die **Funktion** ist die unvollständige Seite. Beide aus `5c077e9fe` (ernte folge130); `3e319087` (river folge3) hat den zweiten Fehler freigelegt. Rot im `ci-check`-Testjob. (Schritt: Monats-/Tagesbereich in `date_midnight_unix` validieren.)

An mountain: Such-API-Modi bauen — Keys für Tavily/Exa/Linkup liegen in `.secrets.local` (`TAVILY_API_KEY`/`EXA_API_KEY`/`LINKUP_API_KEY`, Future-Folge 89); je ein Modus `--tavily`/`--exa`/`--linkup` nach dem Muster `--marginalia`. (Schritt: `archive_search`-CLI-Arm + Parser, Endpunkte `api.tavily.com/search`, `api.exa.ai/search`, `api.linkup.so/v1/search`.)

An mycelium: solar-system-open-data Key liegt in `.secrets.local` (`SOLAR_SYSTEM_OPEN_DATA_KEY`, Future-Folge 89); Quelle in `phi/sources.φ` registrieren und `blocked key` (`phi/blocked_sources.φ:47`) auflösen. (Schritt: REST `api.le-systeme-solaire.net/rest/bodies/` mit `Authorization: Bearer` testen, dann Quell-Zeile setzen.)
