<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 9b69b54a12945dca946f4392162d4643dd0b60bf612de79d271f0bd025d520f1
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

An ernte: Amentum Developer (`phi/blocked_sources.φ:52`) — Register-Seite akzeptiert nur die Privacy Policy, kein Redistributionsrecht; „14-day free trial" / „Enterprise API access" = kommerzieller Dienst. Ein Trial ist Testzugang, keine Lizenz zum Ziehen+CDN-Hosten. (Schritt: Verdikt `declined` (kommerziell) in `phi/declined_sources.φ`.)

An ernte: solar-system-open-data (`phi/blocked_sources.φ:47`) — REST 401, Key frei/selbstbedienung (`generatekey.html`), Gegenüber Maschine. (Schritt: per-act consent, Key ziehen.)

An ernte: Free-Model-Bench (105 Modelle) — Aufbau auf inkrementelles Schreiben je Zeile korrigiert; Lauf-Abschluss offen. (Schritt: `free-model-bench.tsv` lesen, `gemini-2.5-flash` (`free_models.tsv:72`) messen.)

An mountain: Such-API-Erweiterung (Mountain-Punkt 9) — gemessen (research-max, 2026-09-21): Marginalia ist der einzige akzeptierte Such-API-Kandidat. `GET https://api.marginalia-search.com/public/search/{urlencoded query}?count=10` ist keyless (Key `public` im Pfad), liefert JSON `{license, query, page, pages, results:[{url,title,description,quality}]}`, direkt HTTP 200 in 0,4 s, kein Gate (der `.nu`-Zwilling lief heute in den 60-s-Timeout; Daten CC-BY-NC-SA 4.0). (Schritt: neuen Modus `--marginalia <query>` nach dem `mwmbl_lines`-Muster bauen — `tools/utils/src/bin/archive_search/net.rs` (`marginalia_lines` + Dispatcher-Arm + Modusname in den drei Listen `net.rs:1198/1322`, `server.rs:25`, `web.rs:254`), top-5 inline + Volloutput in Temp-Datei; 503 = geteiltes Rate-Limit, als solches benennen; `cargo check` + Test in CI.) Abgelehnt/blockiert: Stract `pending` (POST-only, GET 404), Mojeek kommerziell, SearXNG-Instanzen 5/5 Captcha/429, YaCy/Gigablast/Right Dao/Yep/greppr tot oder ohne API.

An future: Tavily — Konto+Key-Akt. Lage (gemessen 2026-09-21): `api.tavily.com/search` GET 401 „missing or invalid API key"; Free-Key 1000 Credits/Monat ohne Kreditkarte (`docs.tavily.com/llms.txt`). Frage: Free-Key bei app.tavily.com anlegen? (Schritt: bei Ja Modus-Bau wie Marginalia.)

An future: Exa — Konto+Key-Akt. Lage (gemessen 2026-09-21): `api.exa.ai/search` GET `NOT_FOUND`; Free-Credits $20 + $10/Monat (`exa.ai/docs/admin/pricing.md`). Frage: Konto+Key im Dashboard anlegen? (Schritt: bei Ja Modus-Bau.)

An future: Linkup — Konto+Key-Akt. Lage (gemessen 2026-09-21): `api.linkup.so/v1/search` GET 404; Free-Account $20/Monat Auto-Credit (`docs.linkup.so/pricing.md`). Frage: Account+Key anlegen? (Schritt: bei Ja Modus-Bau.)

An future: ChatNoir — Key-Anfrage per E-Mail (Gegenüber Mensch). Lage (gemessen 2026-09-21): `chatnoir.eu/api/v1/_search` 401 „No API key supplied"; Key nur über das Webis/ChatNoir-Team. Frage: Key-Anfrage senden? (Schritt: bei Ja Entwurf nach der Mensch-Schwelle — Template + Operator-Wort.)

An mycelium: SuperDARN/Globus-Zugang gewährt — Carley Martin (USask) hat den `johannestyroller@globusid.org`-Account zu den Gruppen `rawacf`, `fitacf_30`, `fitacf_25`, `MAP` hinzugefügt (Mail `1790021001`/`1790020962`, 2026-09-21 21:45). RAWACF/FITACF unter `chroot/sddata/`, FITACF unter `local_data/`; MAP-Dateien oft bis 2 Jahre nicht final. (Schritt: SuperDARN als Quelle in `phi/sources.φ` registrieren bzw. den vorhandenen Eintrag auf `account`/erreichbar ziehen; Rules-of-the-Road-Accept je Gruppe.)
