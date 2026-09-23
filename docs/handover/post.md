<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: ccbd33f9592585ff63e54f79a3d6dc485898ef3e9aa8d4291dbc28635614d87e
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An future: Riss-Politik imag-data — 15 pre-cdn-URLs tragen zwei konfligierende `source`-Zeilen (em vs gravity), der 0-Kanon glättet nicht. Frage an den Operator: welche Linie gilt, oder beide als Riss tragen? (Schritt: `phi/pipeline/stage/pre-cdn_params_source_riss.txt`; die Entscheidung erlaubt den letzten Pre-Pass-Schritt.)

An future: Aufnahme des 13. Korpus (5206 Blöcke, `phi/pipeline/index.φ:35`) ins Register — Scope-Frage an Operator/Rat. (Schritt: Träger `archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ`; als eigene Quelle registrieren oder descoped?)

An future: Free-Model-Bench-Scope — Default (~105 Modelle/Job) ist unter dem CI-Watchdog-Fenster strukturell unvollendbar (Sweep 3069 s > 2× Median 1414 s). Frage: Sharding / begrenzter Default / Watchdog-Ausnahme? (Schritt: Artefakt ID 10721898235.)

An future: DataONE Data-Policy bleibt 401 (`old.dataone.org`), kein `DATAONE_*`-Credential in `.secrets.local`; die per-Record-`accessPolicy` ist anonym lesbar (`cn.dataone.org` HTTP 200). Frage: DataONE-Konto/Token anlegen, oder per-Record-Policy als Lizenzweg annehmen? (Schritt: Konto/Token.)

An future: CNES-Order 18387 ist nicht brauchbar (44,5 % Dateien in Fehler, Fortschritt 16 % seit 2026-09-21, Ablauf 2026-09-28). Frage: Order pausieren (`PUT /user/orders/pause/18387`, Schreibakt bei CNES) und neu nur über `DMT_N1_1144` in 100er-Batches ordnen? (Schritt: Operator-Wort für die Pause.)

An future: Globus-Transfer `af68c4f1` (SuperDARN MAP) ist ohne Globus-Konto/Token nicht messbar; lokal 4999 Dateien/34,1 GB mit 9 Zero-Byte-Dateien. Frage: Globus-Konto/Token bereitstellen oder Transfer per Web-UI bestätigen? (Schritt: Globus-Login.)

An future: Queue-Strukturlag in den pre-cdn-Queue-Dateien — der `source`-Header hinkt dem url-Block eine Position hinterher (Namen/ttl/force falsch zugeordnet, 22 INTERMAGNET-Erstblöcke ohne ttl). Frage: Neuausrichtung als Datenänderung an Kandidaten freigeben? (Schritt: `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ`.)

An mycelium: `tools-build 35844365704` @`250f07963` **failure** (09:44Z) — Kompilierung grün, rot ist der Upload-Step `gh release upload tools-latest … --clobber --repo omegaflow/omegaflow` (`.github/workflows/tools-build.yml:41–64`): `HTTP 404: Not Found (…/releases/assets/582700040)` — der Clobber trifft ein bereits gelöschtes Asset; die Release-Binär-Kette (`tools-latest`, `bin/.tools_ensure`) friert ein. (Schritt: den Clobber/Upload-Step in `.github/workflows/tools-build.yml` gegen das 404-Asset `582700040` lesen und idempotent machen, dann `gh workflow run tools-build.yml`.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

