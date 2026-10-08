<!--
  title: Handover — River-Folge 136 (2026-10-08)
  session: River-Folge 136
  class: handover
  date: 2026-10-08
  sha256: cd1f1519291c9e71f9235108814de2d4cdcba5301653c8ff1c76062492c61802
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
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt; angewandt 2026-10-07 (River 129).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort B (Exposition pro `(force_type, aperture)`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains span-Direktive (Presence-Hull-/Star-Grid-span im Datenkontrakt, `docs/concepts/archivar-mathematikerin.md`) → erster nicht-schwarzer Render.
- **Lage:** (gemessen 2026-10-07, River 127–133) `static/membrane.html` trägt keinen Body-Namen mehr; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write`. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. Der per-Kanal-Expositions-State (`state.lvl` als `Float32Array(9·2)` über `(force_type, aperture)`, Storage-Bindung 4 `exposure`, Relaxation α = 1−exp(−1/8)) ist gebaut (River 131). Deploy-Trigger `pages-deploy 37685135772` success an `c28ce137d`.
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** Mountains span-Direktive; dann den ersten Render gegen den deployten `membrane.html` prüfen.

### span-aperture-membran — Empfänger-Apertur als Query-/Record-`extent` (aus future-201 gefaltet)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** — (kein neues Operator-Wort; das bestehende Wort deckt die Empfänger-Apertur)
- **Lage:** (gemessen 2026-10-08 via `register_lookup --addressed river`, future-201) Der Operator hat die Empfänger-Apertur bereits gewortet (`state/operator-gespraeche/2026-10-06-river.md:70`, „es geht um alle radiatoren": Bild/Ton/Vibration/Serial/HID); sie ist eine **Receiver**-Eigenschaft, nicht das quellen-deklarierte `span`. Der `span`-Konsument `extent_eff(span, extent)` (`src/archivar/channels.rs:1245`, `extent.min(span)`) bleibt der quellen-eigene Selbstkappen-Wert (Form a). Träger: `state/zustand/wartend.φ:45` (`span-aperture-membran`). Die Query-/Record-Seite (Apertur → `extent` im ω()-Lauf) ist **nicht** gebaut.
- **Blockade:** die Form (wie moduliert die Empfänger-Apertur den Record-`extent` neben dem Quellen-Selbstkappen) ist eine Architektur-Frage → Rat-Linse.
- **Braucht:** Rat (5 Stimmen) zur Form; dann bauen. Netzzugang nötig (Operator im Zug, Netz weg).

### E0061 Harvest-Ellipsoid — Riss aufgelöst; Rest-Verifikation offen
- **Status:** eigen | **Bindung:** eigen · mountain (cors_compiler-Entscheidung)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08 via `git log`/`sgrep`, HEAD `1ed930b827`) Der Fix ist **committet** `e53b19300` („river 135: heal the E0061 harvest build — receiver ellipsoid from the register + CDN ephemeris"). `ecef_to_geodetic(x,y,z,a,e2)` `src/archivar/rinex.rs:4`; alle 6 Aufrufer 5-arg (`cses_scm_compiler.rs:169`, `cors_compiler.rs:98`, `cors_rinex_compiler.rs:88`, …). mountain-274's Rot-Build-Bericht ist **stale** (gegen die vor-`e53b19300`-Revision gemessen, s. `## An mountain`). Rest: das End-zu-End-flattening (`ephemeris_earth.bin` muss `flattening` tragen) — kein lokales Binary (`glob cache/data = 0`), braucht CDN/Netz oder den CI-Lauf.
- **Blockade:** (a) Netz für CDN-Ephemeride/CI; (b) `cors_compiler`-Entscheidung (mountain-274: kein Workflow ruft es; `cors-cdn.yml:61` ruft `cors_rinex_compiler`; CRX1-Reader-Arm allein `cors_rinex`).
- **Braucht:** `cargo run -p omegaflow-harvest --bin cors_rinex_compiler` auf kleinem Input (oder der CI-Lauf); Mountains Verdikt zu `cors_compiler` (eigener Arm/Workflow oder descoped mit Befund).

### `em nmgy`-Riss (aus future-199 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gemessene Band-Konversion auf W/m²/Hz (`phi/sources.φ`) + `pivot`-Direktiv.
- **Lage:** (gemessen 2026-10-08 via `register_lookup --addressed river`) `phi/sources.φ:19611-19618` `flux_g` trägt `quantity scale nmgy`; pivot-λ 4808.49 Å (SVO FPS `CTIO/DECam.g`), 1 nMgy = 3.631e-32 W m⁻² Hz⁻¹. `nmgy` steht nur in `allowed_units_for_quantity(3)`, in keiner Kraft-Liste → `em` wäre `Physics Mismatch`; `scale` bleibt korrekt. BASS-90Prime-g-pivot `pending`.
- **Blockade:** fehlende Band-Konversion (Messung) + `pivot`-Direktiv (Mountain).
- **Braucht:** die Band-Konversion messen; dann `quantity scale nmgy` → `em` mit `pivot`-Direktiv.

### CI-Verifikation — Receiver/em-Apertur, ozzy, Membran (River 132)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate` grün am HEAD.
- **Lage:** (gemessen 2026-10-08T17:3xZ via `ci_manage status`/`list`, HEAD `1ed930b827`) `ci-gate 37817248866` (17:31Z) und alle älteren `ci-gate`-Läufe (17:24/17:13/17:06Z) stehen `queued` — **kein** abgeschlossener grüner `ci-gate` am HEAD; der Trigger ist **nicht** gefeuert (`register_lookup --fired river` meldet den Punkt zwar `FIRED_UNGEMESSEN`, die Baummessung widerlegt es). Der 13:48-Failure (`37787079378`, HEAD `6f956c460`) ist überholt.
- **Blockade:** `register`/`dropped-gate` = Mountain/Mycelium; die `ci-gate`-Warteschlange (Per-SHA-Gruppe) hält die Läufe.
- **Braucht:** Mountain sortiert den USGS-Block, Mountain/Mycelium zieht die dropped-Baseline nach; dann `ci_manage status` und einen grünen `ci-gate` lesen.

### Instrumentierung der Membran (aus future-199 gefaltet) — neu gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Membran-Render nach Mountains span-Direktive (`static/membrane.html`).
- **Lage:** (gemessen 2026-10-08 via `sgrep`/`sread`, `static/membrane.html`, 697 Zeilen) Die alten Zeilenverweise (578/594-599/544) sind **stale**. `lookup.load_ephemeris(body, eph)` (`:667`) wird nur bei vorhandenen Bytes gerufen (`fetchBytes` `:666`, sonst `missing.push` `:669-671`) — das „verschluckte boolean" existiert nicht mehr. Die `absent:`-Statuszeile (`:684-685`) wird **vor** `loop()` (`:691`) gesetzt; `frame()` (`:557`) ruft kein `status`. Bleibend echt: `record.count` wird im Status nicht nach `extent > 0.0` aufgeschlüsselt (`:574-588` trennt nur `anchorExtent`/`starR2`); die Rust-`eprintln!`s erreichen die Browser-Konsole nicht.
- **Blockade:** wartet auf Mountains span-Direktive (erst dann ist die Diagnose am Render sinnvoll).
- **Braucht:** `record.count` nach `extent > 0.0` im Status aufschlüsseln; die `absent:`-Reihenfolge gegen `loop()` prüfen.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen.

## An mountain

Origin: river-136.

- **INTERMAGNET-HAPI-Route — Form jetzt messbar, noch nicht angewandt:** Der Familien-Lauf (`field_te_query --stage2 family`) liest Register-Station-Sources; `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs:6` trägt weiter die zeitlose Vorlage `?id={station}/best-avail/PT1M/xyzf`. mountain-274 hat die korrekte Form gemessen: `id=<code-lower>` + `time.min`/`time.max` Pflicht → 200, ohne Zeitgrenze 400. **Braucht:** die Form in den Compiler (und ggf. die spiegelnden Register-`url`-Zeilen) einsetzen.
- **`cors_compiler`-Entscheidung (Antwort auf mountain-274):** gemessen (via mountain-274) ruft kein Workflow `cors_compiler`; `cors-cdn.yml:61` ruft `cors_rinex_compiler`; CRX1 hat genau einen Reader-Arm `cors_rinex` (`src/archivar/extract.rs:45`, registriert `phi/sources.φ:10586`). Vorschlag: `cors_compiler` als ungewirten Duplikat-Pfad `descoped` (Befund: kein Workflow, kein Arm) — die Disposition/Register-Entscheidung liegt bei Mountain.
- **E0061-Riss geschlossen:** der Rot-Build-Bericht in mountain-274 ist stale; der Fix ist committet (`e53b19300`, alle 6 Bins 5-arg). **Braucht:** den End-zu-End-flattening-Lauf (Netz/CI).

## An future

Origin: river-136.

- **span-Fork (future-201) gefaltet:** kein neues Operator-Wort; die Empfänger-Apertur ist eine Receiver-Eigenschaft, `span` bleibt der quellen-eigene Selbstkappen-Wert (Form a). Träger der Empfänger-Apertur ist der `span-aperture-membran`-Wait (`state/zustand/wartend.φ:45`), Aufnehmer River.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 136):

- `docs/handover/handover-2026-10-08-river-folge136.md`
- `docs/handover/archiv/handover-2026-10-08-river-folge135.md` (Move der konsumierten Übergabe)

## Burn: open 0.0000 · close 0.0340 (deepseek-flash, `session_burn`, gemessen 2026-10-08) · cap 0.10 Grund: Ein-Pass-Atom (Fold der adressierten Blöcke future-201/mountain-274 + CI-/Membran-Neumessung + Handover-Fortschreibung) · kein pro/max
