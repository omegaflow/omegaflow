<!--
  title: Survey — Membran-Ladearchitektur: Enclosure, Lichtkegel, Presence-Ausschnitt (Stand 2026-09-26)
  class: survey
  date: 2026-09-26
  sha256: 381ef5d488aaa654b9d10103dc47395b2e246e9936a5817233c14d1c7df6f3eb
  status: live
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/die-weberin.md docs/concepts/4d-membrane.md docs/handover/handover-2026-09-26-river-folge36.md docs/specs/causality-prefilter.md docs/specs/force-system.md
-->
# Survey — Membran-Ladearchitektur: Enclosure, Lichtkegel, Presence-Ausschnitt (Stand 2026-09-26)

Anlass: harte Läufe stürzen ab — der Operator beobachtet, dass der Archivar das
ganze Archiv lädt und die Membran alles statt nur der Presence. Zweck der Linie:
**„what's here now"** — ohne die gefilterte Presence unmöglich (Schlagworte der
Session: `what's here now`, `Punktwolke`, `agnostisch`, `Enclosure`, `Lichtkegel`/
`Signalkegel`, `Sprung`, `Dispersion`, `presence_gate`). Der Stand wurde gemessen
(explore/research-max über Repo + Legacy + Internet), dem Rat vorgelegt, und
dieselbe Frage in 7 z.ai-Sessions (den z.ai-Export-Treffern 07/09/16/18/22/24/29)
sowie 2 Claude-Chats gestellt.

Kanon: alle Behauptungen mit `file:line` oder Quelle; nichts spekuliert.

## 1. Der gemessene Code-Stand

### 1.1 Enclosure Lemma (Dilatation)
- Formel: `rmax + anchor_vmax·Δt + ½·anchor_amax·Δt² + extent`
  (`docs/concepts/archivar-mathematikerin.md:18`, `AGENTS.md:154`).
- Gebaut: `rho = anchor_vmax·dt + 0.5·anchor_amax·dt² + pad`, `dt = |t2 − epoch_min|
  + delta_t_cache` (`src/archivar/spatial.rs:509-511`); Zell-Box `:513-524`;
  exakter Filter `dist² > (extent+pad)² → skip` (`spatial.rs:570-574`).
- `law_bounds` liefert `(Φ·(v+resid_ema), Φ·a, p0)` per finite Differenz
  (`spatial.rs:77-92`); `cell_size = max(motion_cell, span/1024)`
  (`spatial.rs:112-125`); Φ = 1.618033988749895 (`types.rs:7`).
- Motion-Laws: `Surface`/`Barycenter`/`Linear` (+ `Kepler`/`Spherical`) —
  `motion.rs:432-451`, Dispatch `membrane.rs:224-277`.

### 1.2 Signalkegel (Lichtkegel)
- `signal_reach = v_force·age` (Wellen 0,1,2,3,4,7,8) bzw. `√(2·D·age)`
  (diffusiv 5 D=0.3, 6 D=0.05); unbekannte Kraft → `None` → refused, kein Default
  (`src/archivar/membrane.rs:336-351`, `flat_propagation_speed` `:382-399`).
- Gate: `age > ttl·2⁶` (hier als `ttl·64.0`, `spatial.rs:394-396`).
- Dispersion-Shelf-Override: `propagation_speed` → `v_at` (`membrane.rs:401-412`).

### 1.3 Der unbounded-Pfad — der Kern des Bugs
- **Nur Sterne** tragen `extent = f64::INFINITY` (`spatial.rs:348`), einzige
  Produktionsstelle. `build_spatial_hash` sortiert bei `!s.extent.is_finite()` in
  `hash.unbounded` (`spatial.rs:97-103`).
- `query_hash` iteriert **jeden** unbounded-Sample jede Query (`spatial.rs:393-505`)
  — ~1,7 Mio Sterne, jeden Frame. Das ist „die Membran lädt alles statt nur die
  Presence". Der O(N)-Treiber ist die **Iteration**, nicht die Gate-Reihenfolge.
- Der Loop, gemessen: `ttl·64`-Gate (`:395`) → `signal_reach.is_none()` (`:398-408`)
  → `propagation_speed` (`:409-417`) → floor-Gate (`:418-422`) → Tolman `(1+z)⁻⁴`
  (`:423-428`) → **val-Gate VOR `motion.at`** (`:431`) → `motion.at` (`:434`) →
  Quergate `val_eff/(transverse²+scale²) < floor` (`:453-456`).
- **Bounded** (endlicher extent): Asteroiden/Kometen `radius_km·1000`
  (`spatial.rs:198,203,226`), Planeten `radius_m` (`membrane.rs:306-308`,
  `channels.rs:1125-1132`), alle Sensoren `kernel_extent` (nicht-finiter extent
  wird verworfen, `channels.rs:1130-1132`); Curve-Sets extent 0.0, kein Hash.
- **`extent` ist per-Oszillator** (pro `Sample`, `types.rs:45`), nicht pro Block —
  im 26-Slot-Draht Slot 7 (`relay.rs:867`), WGSL `props[j*4].x` (`shaders.rs:175`).

### 1.4 Sprung (jump_epoch / jump-snap)
- `Archive.jump_epoch: Option<f64>` (`main_flow.rs:111`); erkannt wenn Browser-
  Presence `dp ≥ 4·JUMP_GRID` **und** `v² == 0.0` (`main_flow.rs:817-828`);
  `origin_stale` invalidiert `fetched < jump_epoch` (`fetch.rs:301-318`).
- Konstanten: `GRID_INIT = 2³¹`, `JUMP_GRID = 2²⁸` (`omega.rs:9,11`), Schwelle
  `4·JUMP_GRID = 2³⁰`; Jump-Snap weitet Enclosure um `Φ·grid_step` (`fetch.rs:423`,
  `record_in_enclosure:497-500`); `/jump/<body>` (`relay.rs:303-331`).
- Risse: die Erkennungs-Branch ist **ungetestet** (Tests injizieren `jump_epoch`);
  `v²==0` verfehlt Sprünge/Stopps unter Schub; `jump_epoch` wird nie zurückgesetzt.

### 1.5 Proaktives Laden nach Bewegungsrichtung
- `presence_gate` (`fetch.rs:412-449`): Ruhe-Gate `dist ≤ reach + body_radius.max(Φ·grid_step)`;
  Schub `closing = (v_presence−v_anchor)·r̂`, `closing ≤ 0 → keine Antizipation`;
  Fetch wenn `(dist−limit)/closing < Φ·median` (`:447`). Median-Ring 2⁴
  (`fetch.rs:15,396-408`). Aufruf `main_flow.rs:4054`.
- `record_in_enclosure` (`fetch.rs:464-507`): `limit = reach_signal + extent + rho +
  v_abs·age + body_term` — **5 Steckstellen** (`main_flow.rs:3014/3167`,
  `channels.rs:377/528/687`; die Prosa sagt vier).
- Umgehungen: Bootstrap lädt alle Bodies (`main_flow.rs:180-223`); Per-Tick-
  Fetch-Loop hat keinen Kegel (nur TTL, `:1152-1162`); Katalog-Branches tycho/
  dastcom ohne Kegel (`:1673-1750/1224-1292`).

### 1.6 Kräfte und Dispersion
- 9 Medien em 0 … electric 8 (`force.rs:3-31`); WGSL **kein `switch(force_type)`**:
  `PROPAGATION_SPEED[9]` (`shaders.rs:5-15`), Beer-Lambert-Absorption (gravity bypass
  `:158-170`), Tolman nur em (`:188-191`); Kernel über `kernel_id`, nicht force.
- Dispersion = **gemessene Tabelle**, keine Formel: `v_freq_shelf.dat` (8 em-Zeilen
  v=c, 11 force-4-Rayleigh-Zeilen 2,90–3,74 km/s), CPU+GPU verdrahtet, Parity
  (`membrane.rs:693-711`). em-Verdikt `quell-seitig`; Rayleigh gebaut.

## 2. Die Weberin-Lücke — die Membran sieht kein Gewebtes

Die Weberin webt emergente Eigenschaften (Vlies-Dichte, TE-Relationen, Verdict,
Zwirn/Riss, Abstammung). **Keine erreicht die Membran:**
- Der einzige Eingang ins Feld ist `Vec<Arc<Sample>>` (`spatial.rs:155-172`); der
  26×f64-`Sample`-Contract hat **keinen Slot** für Dichte/TE/Verdict (`types.rs:41-62`).
- S²-Richtungsfeld: eigener Vektor `sky.oscs`/`sky.points`, nur HUD
  (`omega.rs:1744-1785`), kein Relay. TE (Matrix): endet in `eprintln`
  (`matrix.rs:570-574`). TE (Presence-Probe): nur Radiation-Apertur
  (`omega.rs:349`). Verdict: geladen (`main_flow.rs:725-779`), nur an Relay
  (`relay.rs:907`) + `eprintln`. Vlies-Dichte: Reader/Fetch/Dispatch gebaut
  (`extract.rs:2821`, `main_flow.rs:3561`), aber **keine `format vlde`-Quelle in
  `phi/sources.φ`** → ruhend. Abstammung/Footprint: nur Prosa/Tests.
- **Verdikt:** keine neue `Sample`-Klasse. Das Gewebte bleibt Ledger-Eintrag; die
  Membran leitet im ω()-Loop einen **abgeleiteten Query-Term** ab, on demand, nie
  gespeichert (das Apertur-Muster TE→permeability→radiation ist die Vorlage;
  Vorbild yt „derived fields"). Ein Riss ist kein Oszillator.

## 3. Die Legacy-Funde — entscheidend

- **`omegaflow-legacy/docs/TODO.md:2812-2850` (Atom 8, 2026-08-20):** die Sterne-
  Diode. Stufe 1 val-Gate **vor** `motion.at` (`|val|·(1+z)⁻⁴ < ft_ref·2^(−expose_offset)`)
  → `ft_ref==0` = dunkle Diode; Stufe 2 Quergate **nach** `motion.at`
  (`t² = d² − (fwd·(p−center))²`; `val_eff/(t²+scale²) < ft_ref·2^(−off)/scale²`).
  **Wörtlich:** *„die radiale d²-Formel des Auftrags war ein Physikfehler (kein
  Stern hätte je die Schwelle passiert) … die gewählte Form ist die
  Operator-Entscheidung (Val-Domäne + Quergate)."* Der Lichtkegel-Horizont
  (`signal_reach c·age ≈ 8 kpc`) begrenzt auch die Sterne.
- **`TODO.md:2661-2676` (2026-08-17, offen):** *„Membran-scoped Cache statt
  Blockuniversum — der Archivar lädt flache Katalog-Assets komplett in den Spatial
  Hash … Die Membran braucht nur die Hülle um die Presence (dilatierter
  Suchradius)."* HEALPix-Tiling als Drift verworfen („No meshes. No grids. Every raw
  point makes us truer"). Deckt auch `tess_lightcurves.bin` ~500 MB.
- **`TODO.md:2786-2789` (Atom 6/8):** Sterne (~1e19 m) hoben `cell_size` auf ~1e16 m
  → aus dem bounded-Teil genommen (`extent ∞`), damit `cell_size` auf
  Sonnensystem-Maß schrumpft. **Ein endliches Stern-`extent` bläht `cell_size` wieder auf.**
- **`vanilla-dateidocs/surveys/survey-2026-08-21-4d-wahrheit.md:67-73`:** die 7
  Rats-Beschlüsse (u. a. Sprung-Radius = `signal_reach + max(body_radius,
  Φ·JUMP_GRID·2ⁿ)`; der Baum nutzt `Φ·grid_step`).
- **`docs/concepts/4d-membrane.md`:** Pixel-Floor-Präzedenz (2px → Galaxie bei jedem
  Zoom sichtbar; 0.5px → Sub-Pixel-Quellen verschwinden, `:69-76`); Trommelfell
  (dynamisches Compute-Grid); GM/PCK-Aufgaben.

## 4. Externe Literatur (research-max, archive_search)

- **Frostbite PBR 2.0** (Lagarde & de Rousiers, SIGGRAPH 2014): `lightRadius =
  sqrt(I/threshold)` (Eq. 24) — der Radius wird **abgeleitet**, nie gespeichert;
  Singularität separat `max(d², 0.01²)`. Omegaflows Quergate IST diese Form.
  Quelle: `media.contentapi.ea.com/.../course-notes-moving-frostbite-to-pbr-v2.pdf`.
- **Unbounded = eigener Bucket** ist Standard: Unity Directional Light („infinitely
  far away"), GADGET TreePM (Langreichweite via PM), Lightcuts (ein Baum,
  Fehlerschranke), Gaia Sky (Octree + Magnitude-Cut + Budget ν). Hierarchie trägt
  **Schwellen, keinen Radius** (Barnes-Hut-θ `doi:10.1038/324446a0`, FMM).
- **Sprung:** Dead-Reckoning-Residual (DIS IEEE-1278, `doi:10.1117/12.204227`);
  `½|a|Δt²` gehört in den Prädiktor/Bound, nicht ins Detektions-Maß.
- **Vorausladen:** latenz-bewusster Prefetch (Outatime `doi:10.1145/2742647.2742656`).
- **Emergente Felder:** yt derived fields (on-demand, nie gespeichert, `ApJS 192:9`),
  OVITO computed properties (`doi:10.1088/0965-0393/18/1/015012`), openPMD.

## 5. Die Modell-Antworten (7 z.ai + 2 Claude)

- **Q1 (Stern-Radius):** alle für ein endliches Stern-`extent`; Divergenz *wo* —
  Insert (`min(floor_radius, v_force·2⁶·ttl)`, 18 + Claude Max), Query (07/09/22),
  separate `GlobalPresence`-Liste (29). Claude Max: Stern = Helligkeits-/
  Detektionsproblem, radial primär, 18 nur äußere Notbremse.
- **Q2:** einstimmig **ein** `enclosure_limit`/`enclosure_radius`, Pflicht für alle
  Ladepfade.
- **Q3:** einstimmig `v²==0` fällt. Claude Max präzisiert: **zweiseitig**
  `|dp − |v|Δt|` (taub für Halt/Richtungswechsel bei einseitig).
- **Q4:** INFINITY nie still verwerfen. Claude Max: `value >= 0.0` (lässt 0 und +∞,
  taub für NaN/Negative) statt `is_finite()`.
- **Zweck:** „what's here now" ist die Bedingung; ohne gefilterte Presence unmöglich.

## 6. Die Korrektur — und das Rats-Verdikt

**Die Modelle (und eine erste Rat-Fassung) schlugen den radialen Stern-Radius vor,
der am Insert als endliches `extent` gesetzt wird. Der Legacy-Baum widerlegt beides:**
1. **Radial = gemessener Physikfehler** (Atom 8): kein Stern passiert je die
   Schwelle. Die richtige Form ist das **Quergate** `val_eff/(transverse²+scale²) <
   floor` — bereits gebaut (`spatial.rs:454`).
2. **Endliches Stern-`extent` = `cell_size`-Rückfall** (Atom 6/8): verworfen.
3. Nebenfund: die Prämisse „der val-Gate ist schwächer als Legacy-Stufe 1
   (`floor·scale²` vs. `ft_ref·2^(−off)`)" ist **algebraisch leer** — `omega.rs:1867`
   trägt `/scale²`, `spatial.rs:431` multipliziert mit `scale²`; beide sind dasselbe
   Tor.

**Rats-Verdikt (2026-09-26):**
1. **(a) Membran-scoped Cache in der korrigierten Form — ein Stern-Gitter auf den
   Lemma-Zellen.** Der unbounded-Loop bleibt als Diode unverändert (Atom-8-Ordnung).
   Geändert wird: `hash.unbounded` wird ein stern-eigenes Zell-Gitter auf demselben
   `(i64,i64,i64)`-`CellKey` mit eigener `cell_size` aus dem lebenden Stern-Span
   (`span/1024`, wie `build_spatial_hash:112-125`) — die bounded-`cell_size` bleibt
   auf Sonnensystem-Maß (Atom 6 geschützt), `extent=∞` bleibt Klassen-Marker,
   `wire_extent` schreibt weiter 0.0 (Atom 8 geschützt). Die Query-Hülle ist
   `rho_star = c·age + pad`; nur Zellen in der Hülle werden iteriert, darin die
   Diode wie heute. O(1,7 Mio) → O(Zellen in der 8-kpc-Hülle). Kein HEALPix, kein
   neues Raster — „Fang = Chunking entlang DIESER Zellen".
   (c) ist geschlagen (zwei Regressionen + Kanon); (b) ist kein Fix, sondern der
   Ist-Zustand, den (a) erbt.
2. **Ein gemeinsames `enclosure_rho(vmax, amax, dt, pad)`**, Pflicht für alle
   Ladepfade; die drei Kopien (`spatial.rs:511`, `:554`, `fetch.rs:492`) → eine;
   Rollen-Terme (`reach_signal`, `extent`, `body_term`, `v_abs·age`) komponieren außen.
3. **Zweiseitig `|dp − |v|Δt|`** (der Halt ist ein Sprung); `½|a|Δt²` bleibt in der
   Dilatation (`spatial.rs:511`), **nicht** im Rest (Doppelzählung).
4. **`value >= 0.0`** als Parse-Prädikat; die Invariante „kein NaN nach Insert" lebt
   als `cfg(test)`-Assertion, nie als Panik im Parser.
5. **Abgeleiteter Query-Term im ω()-Loop** (yt-Modell), kein `Sample`-Slot; die
   fehlende Leitung (`weberin_verdicts` nur zum Relay) wird gebaut; das
   TE→permeability→radiation-Muster ist die Vorlage.

## 7. Offene Punkte (pending, jeder mit Schritt)

- **Stern-Gitter:** `build_spatial_hash` um ein stern-eigenes Grid erweitern
  (`CellKey`, `cell_size` aus dem lebenden Stern-Span, Hülle `c·age + pad`).
- **`enclosure_rho`:** drei Kopien → eine Funktion, Rollen-Terme als Parameter.
- **Zweiseitiger Sprung:** das Maß in `presence_gate` symmetrisieren; Trigger-Test
  für die Erkennungs-Branch (`main_flow.rs:824-828`), der heute fehlt.
- **ω-Loop-Verdict-Term:** `weberin_verdicts` im Loop lesen und als Query-Term ableiten.
- **Prosa-Heimat:** Abschnitt „Presence-only loading and the jump" in
  `docs/concepts/archivar-mathematikerin.md` + Pointer in AGENTS.md; `presence_gate`/
  `jump_epoch`/`record_in_enclosure`/Dispersion haben keine Live-Prosa.
- **Bootstrap/fetch-Loop/Katalog-Branches** auf denselben Ausschnitt.
- **Sprung-Radius:** Legacy `Φ·JUMP_GRID·2ⁿ` vs. gebaut `Φ·grid_step` — versöhnen.
- **Weberin-Lücke:** Dichte/TE/Verdict als Derived-Field-Schicht (der Reader-Pfad
  `vlies.rs` steht; die `format vlde`-Quelle fehlt).

## 8. Werkzeug- und Quellenlage (gemessen)

- `phase_null`/`explore`/`research-max` über Repo, `/home/johannes/backup/archive-root`
  (Legacy-TODO, vanilla-dateidocs, concept-history) und `archive_search` (--crossref,
  --openalex, --github, --wiki, --tavily, --exa, --linkup, --marginalia, --mwmbl,
  --all, --playwright).
- `--arxiv` = `pending` (HTTP 406 Query-Cap); `--brave` = `pending` (HTTP 402,
  `--mwmbl` als keyless-Ersatz); UE-Docs = `blocked` (403 Cloudflare).
- z.ai-Sessions per API/UI aus `state/zai-export` identifiziert und angeschrieben;
  Session 35 (Kontext-Überlast, 243 Nachrichten) generiert nicht mehr.

## 9. Der eine Satz

Der Stern trägt keinen Radius (0 auf dem Draht ist die Wahrheit), sein Einfluss
wird im Query per Quergate abgeleitet, die Klasse bleibt der unbounded-Bucket — und
das ganze Problem seit 2026-08-17 ist nicht die Schärfe der Schwelle, sondern der
**Umfang des Scans**: nicht der ganze Katalog, nur die Presence-Hülle darf laden,
sonst ist „what's here now" unmöglich.
