<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 570c219ec42802a0abe6f7a36d8873686312eadc98b833d397d53583967bfa93
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

An mountain: Such-API-Erweiterung (Mountain-Punkt 9) — gemessen (research-max, 2026-09-21): Marginalia ist der einzige akzeptierte Such-API-Kandidat. `GET https://api.marginalia-search.com/public/search/{urlencoded query}?count=10` ist keyless (Key `public` im Pfad), liefert JSON `{license, query, page, pages, results:[{url,title,description,quality}]}`, direkt HTTP 200 in 0,4 s, kein Gate (der `.nu`-Zwilling lief heute in den 60-s-Timeout; Daten CC-BY-NC-SA 4.0). (Schritt: neuen Modus `--marginalia <query>` nach dem `mwmbl_lines`-Muster bauen — `tools/utils/src/bin/archive_search/net.rs` (`marginalia_lines` + Dispatcher-Arm + Modusname in den drei Listen `net.rs:1198/1322`, `server.rs:25`, `web.rs:254`), top-5 inline + Volloutput in Temp-Datei; 503 = geteiltes Rate-Limit, als solches benennen; `cargo check` + Test in CI.) Abgelehnt/blockiert: Stract `pending` (POST-only, GET 404), Mojeek kommerziell, SearXNG-Instanzen 5/5 Captcha/429, YaCy/Gigablast/Right Dao/Yep/greppr tot oder ohne API.

An mycelium: SuperDARN/Globus-Zugang gewährt — Carley Martin (USask) hat den `johannestyroller@globusid.org`-Account zu den Gruppen `rawacf`, `fitacf_30`, `fitacf_25`, `MAP` hinzugefügt (Mail `1790021001`/`1790020962`, 2026-09-21 21:45). RAWACF/FITACF unter `chroot/sddata/`, FITACF unter `local_data/`; MAP-Dateien oft bis 2 Jahre nicht final. (Schritt: SuperDARN als Quelle in `phi/sources.φ` registrieren bzw. den vorhandenen Eintrag auf `account`/erreichbar ziehen; Rules-of-the-Road-Accept je Gruppe.)
