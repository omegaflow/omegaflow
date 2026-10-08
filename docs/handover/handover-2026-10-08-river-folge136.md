<!--
  title: Handover — River-Folge 136 (2026-10-08)
  session: River-Folge 136
  class: handover
  date: 2026-10-08
  sha256: 1a06f6abd81291fd1edc165c0422243189ad4c527a81b27a631bd74e6b5e8fd9
  status: live
-->
# Handover — River-Folge 136 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127) — alle offenen UI-Seats, nicht einer
„es kommen doch keine sterne oder die sonne oder der mars an der presence an es kommen die kräfte also die kanäle/oszillatoren an ich glaube ihr habt irgendwann wieder die objektophilie eingeführt und euch vom agnostizismus wegbewegt" | 2026-10-07 | Operator (Session, River 127) — Kraft-/Kanal-Agnostik statt Objekt-Render
„es gibt keine sonne erde mond die presence kann sich frei durch das 4d block universum bewegen … sie spawnt nur am SSB weil euer bias sonst noch größer wäre von da kann sie sich völlig frei bewegen" | 2026-10-07 | Operator (Session, River 127) — freie Presence-Weltlinie, SSB-Spawn, keine Objekte
„Die Förder-Bewerbungen bleiben LOCK … Send bleibt deine Hand" | 2026-10-07 | Operator (Session, River 127) — Prototype Fund (30.11.) + EMAP (06.11.) bleiben LOCK
„auf jeden fall agnostoisch dein vorgänger hat doch schon eine umfangreiche gibt und bias untersuchung gemacht ist die schon wiedre vergessen?" | 2026-10-07 | Operator (Session, River 129) — Wort für den agnostischen Membran-Edit
„bitte ratsfragen auch vor ALLE UI chats bringen" | 2026-10-07 | Operator (Session, River 130) — Ratsfragen vor alle UI-Seats; als Regel in `AGENTS.md` eingetragen
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge133.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad geheilt (`:7`).
- `docs/paper/gic-causal-driver.md` — §4.7/§6 NUR-Asset-Fakten.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — Träger der Flyby-Kette (Offen: Zell-Fortschreibung).
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt; angewandt 2026-10-07 (River 129).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### span-Apertur — Rat-Verdikt (b): Empfänger-seitig, Record-`extent` quellen-eigen
- **Status:** eigen | **Bindung:** eigen · mountain (Doc-Benennungs-Riss)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08, Rat der fünf Stimmen) Verdikt **(b)**: die Empfänger-Apertur (`state/operator-gespraeche/2026-10-06-river.md:70`, „es geht um alle radiatoren") wirkt **empfangs-seitig**, nie im Record-`extent`; der Record-`extent` bleibt quellen-eigen (`src/archivar/channels.rs:1245` `extent_eff`, `:1359`). Gerade weil der Record von *allen* Radiatoren gelesen wird, trüge eine Empfänger-Apertur im Draht N Blicke für eine Quelle (Observer-as-vantage). Gebaute Empfangs-Orte: `static/membrane.html:467-494` (`state.lvl[ft·2+ap]`, Bild), `src/mathematikerin/omega.rs:361` (`frame.aperture = field_permeability · tone_scale`, Ton/Vibration), `src/mathematikerin/actuators.rs:73-79` (`Σω·aperture`). Der **HID-Pfad ist ungemessen** (kein HID-Aktuator im Baum) → `pending`.
- **Riss (nicht gemittelt):** `docs/concepts/archivar-mathematikerin.md:33` nennt `span` „receiver-side aperture override"; der Code backt ihn als **quellen-deklarierte Selbstkappung** in `Sample.extent` (`channels.rs:1359`) und schreibt ihn auf den Draht (`spatial.rs:817/975`). Linie A (Mountain): `span` = quellen-deklarierte Selbstkappung, der Doc-Satz ist die Fehlbenennung. Linie B (River): `span` genuin empfangs-seitig, dann darf er den Draht nicht berühren. Messbar, sobald eine Quelle `span < medium_reach` deklariert.
- **Blockade:** die Doc-Benennung `:33` (Mountain, Contract).
- **Braucht:** Mountain entscheidet die Doc-Formel `:33` (A oder B). Der HID-Aktuator-Term bleibt `pending`.

### Membran — Kraft-/Kanal-Agnostik (wartend auf den Render)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste nicht-schwarze Render gegen den deployten `static/membrane.html`.
- **Lage:** (gemessen 2026-10-08) Der Contract `docs/concepts/archivar-mathematikerin.md:33` trägt den `span`-Override (Mountain-Direktive); der `span`-Konsument `extent_eff` ist gebaut. `pages-deploy 37685135772` success an `c28ce137d`. `ci-gate 37817248866` läuft (in_progress, 18:42Z).
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) — das Feld braucht einen Anker mit finiter `extent`.
- **Braucht:** den deployten Render prüfen (`archive_search --playwright <pages-url>`), sobald die `ci-gate`-Kette grün ist.

### Membran-Instrumentierung — gebaut (dieser Atom)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Membran-Render (`static/membrane.html`).
- **Lage:** (gemessen 2026-10-08 via `sread static/membrane.html`) `record.count` wird in `updateLvl` nach `extent > 0.0` in `state.anchors`/`state.stars` aufgeschlüsselt (`static/membrane.html:467-475`); die Loop-Statuszeile trägt `anchors N · stars M` (`:622`); die `absent:`-Liste liegt in `state.absent` und wird im Loop-Status getragen (`:622,693`) — kein Überschreiben mehr durch `loop()`. Das alte „verschluckte boolean" existiert nicht (`:666-671` schützt `load_ephemeris`). Die alten Zeilenverweise (578/594-599/544) waren stale.
- **Blockade:** der Render (s.o.) fehlt zur Sichtprüfung.
- **Braucht:** den ersten Render — dann die Statuszeile gegen `anchors`/`stars`/`absent` prüfen.

### `cors_compiler`-Disposition (aus E0061)
- **Status:** wartend | **Bindung:** eigen · mountain
- **Trigger:** Mountains Register-Disposition (`phi/sources.φ`).
- **Lage:** (gemessen 2026-10-08) Der E0061-Fix ist committet (`e53b19300`); End-zu-End gemessen: das CDN-`ephemeris_earth.bin` (HTTP 206) parst mit `radius 6378137 m flattening 0.003353` (`cargo run -p omegaflow-measure --bin ephemeris_structure_probe -- <bin>`), `body_ellipsoid_of` liefert `Some` → die geodätische Position der 6 RINEX-Bins ist da, nicht absent. `cors_compiler` hat keine Registerzeile und laut mountain-274 keinen Workflow (`cors-cdn.yml:61` ruft `cors_rinex_compiler`; CRX1-Arm allein `cors_rinex`).
- **Blockade:** Mountain (Register-/Workflow-Entscheidung).
- **Braucht:** Mountains Disposition (eigener Arm/Workflow oder `descoped` mit Befund).

### `em nmgy`-Riss
- **Status:** wartend | **Bindung:** eigen · mountain
- **Trigger:** eine `band … pivot …`-Registerzeile auf `flux_g` (`phi/sources.φ`).
- **Lage:** (gemessen 2026-10-08 via `sgrep src/archivar/parse.rs`) `parse.rs:1113` liest `nmgy`; `:2454` zeigt die Syntax `quantity flux_g g_flux inverse-square scale nmgy 31536000 0.0 0.0 band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom`; `:2450` verlangt eine Band-Referenz (ohne → refused, nie eine 0.0-Band). `phi/sources.φ:19611-19618` `flux_g` trägt keine Band-/pivot-Direktive → `scale` bleibt gültig, ein `em` wäre `Physics Mismatch`. 1 nMgy = 3.631e-32 W m⁻² Hz⁻¹ (AB; pivot-λ 4808.49 Å, SVO FPS `CTIO/DECam.g`). BASS-90Prime-g-pivot `pending`.
- **Blockade:** fehlende `band`/`pivot`-Registerzeile (Mountain).
- **Braucht:** Mountain setzt `band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom` auf `flux_g`; dann `quantity scale nmgy` → `em` prüfen.

### CI-Verifikation — Receiver/em-Apertur, ozzy, Membran
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate` grün am HEAD.
- **Lage:** (gemessen 2026-10-08T18:4xZ via `ci_manage list`) `ci-gate 37817248866` (17:31Z) ist **in_progress** (18:42Z); `ci-gate 37826520140` queued (18:43Z). Kein grüner am HEAD; kein lokaler Schritt.
- **Blockade:** `register`/`dropped-gate` = Mountain/Mycelium.
- **Braucht:** `ci_manage status`; einen grünen `ci-gate` lesen.

### dropped-gate — Ursache gemessen (Token-Bags aus archivierten Handovers)
- **Status:** wartend | **Bindung:** eigen · mycelium
- **Trigger:** ein `ci-gate`-Lauf, der den `dropped-gate`-Job auswertet.
- **Lage:** (gemessen 2026-10-08) Die zwei neuen Drop-Keys sind **Token-Bags aus Handover-Prosa**: `für presence-hull- river schritt span-direktive star-grid-apertur` = `docs/handover/archiv/handover-2026-10-07-mountain-folge271.md:150`; `--lpf 200 … fef1238b6 … start steht` = `docs/handover/archiv/handover-2026-10-07-mycelium-folge267.md:108`. Mechanik: `dropped_gate --carrier` (`tools/register/src/bin/dropped_gate.rs:364-409`) liest als Träger nur **live** `docs/handover/*.md` + `git log --pretty=format:%B --name-only` + `phi/*.φ`; wandert ein Handover nach `archiv/`, verlieren seine Token-Bags den Träger → neuer Drop. Der Baseline `docs/zustand/dropped-legacy-baseline.txt` trägt die Prosa-Bags (Erzeuger `register_lookup --dropped`, full history).
- **Blockade:** Gate-Mechanik (Träger nur aus live-Handovers) + Baseline-Erzeugung.
- **Braucht:** Mycelium prüft, ob `derive_carriers` auch `docs/handover/archiv/*.md` lesen soll (oder ob der Baseline-Erzeuger keine Prosa-Token aufnehmen darf); dann `dropped-legacy-baseline.txt` neu ziehen.

### Flyby-Kette — OMNI2-Trigger gefeuert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit. Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-08) **OMNI2-Trigger gefeuert:** die HAPI-URL (OMNI2_H0_MRG1HR, 2026-09-26..29) liefert `HTTP 200` (33251 B). Der Fill lief lokal (`cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice`): Trajectory `official: placed` (sha `aeb3c82…`), 26 Zellen, Register `data/flyby2/tube-juice-2026-09-28.json`; die Zellen tragen RTSW-bt/bz/v/n/T/p + `omni2 bz` weiter `pending`, kp/Swarm/ACE Werte. **kp `def`: HTTP 500** (nicht gereift). **JUICE-recon** absent (Wiedervorlage 2026-11-01).
- **Blockade:** Zell-Füllung (RTSW-Alignment/OMNI2-Parameter) + kp `def`.
- **Braucht:** das Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md` um die gemessenen Zellen fortschreiben; kp `def` erneut prüfen.

## An mountain

Origin: river-136.

- **INTERMAGNET-HAPI-Route:** die korrigierte Form ist bestätigt (`id` lowercase + `time.min`/`time.max` Pflicht) — live gemessen 2026-10-08: `…/hapi/data?id=abk/best-avail/PT1M/xyzf&time.min=…&time.max=…&format=json` → **HTTP 200** (103778 B); ohne Zeitgrenze 400. `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs:6` trägt weiter die zeitlose Vorlage. **Braucht:** die Form in den Compiler (und ggf. spiegelnde Register-`url`-Zeilen) einsetzen.
- **`cors_compiler`-Disposition:** siehe Offen — kein Workflow, keine Registerzeile; Vorschlag `descoped` mit Befund oder eigener Arm.
- **`span`-Doc-Riss:** siehe Offen (Rat-Verdikt b) — Entscheidung über `docs/concepts/archivar-mathematikerin.md:33` (A oder B).
- **`em nmgy`:** `flux_g` braucht eine `band … pivot …`-Zeile (`parse.rs:2454` ist die Form).

## An mycelium

Origin: river-136.

- **`dropped-gate`-Ursache:** siehe Offen — die zwei neuen Drop-Keys sind Token-Bags aus archivierten Handovers (`mountain-folge271:150`, `mycelium-folge267:108`); `dropped_gate --carrier` liest Träger nur aus live `docs/handover/*.md`. **Braucht:** entscheiden, ob `derive_carriers` auch `archiv/` liest oder der Baseline-Erzeuger keine Prosa-Token aufnimmt; dann Baseline neu ziehen. (Hinweis: auch das Archivieren *dieser* Handover kann Träger verschieben.)

## An future

Origin: river-136.

- **span-Fork (future-201) gefaltet und um das Rat-Verdikt geschlossen:** kein neues Operator-Wort; die Empfänger-Apertur ist Receiver-Eigenschaft, `span` bleibt quellen-eigen. Träger `state/zustand/wartend.φ` (`span-aperture-membran`) — wird in diesem Atom als beschieden geführt.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 136):

- `static/membrane.html` (Instrumentierung)
- `docs/handover/handover-2026-10-08-river-folge136.md`
- `docs/handover/archiv/handover-2026-10-08-river-folge135.md` (Move, bereits committet)

## Burn: open 0.0000 · close 0.0824 (deepseek-flash, `session_burn`, gemessen 2026-10-08) · cap 0.15 Grund: Ein-Pass-Atom (Fold der adressierten Blöcke + E0061-End-zu-End + Membran-Instrumentierung + Rat-Verdikt + Flyby-OMNI2 + dropped-gate-Ursache) · kein pro/max
