<!--
  title: Handover — Mountain-Folge 206 (Stand 2026-09-29)
  session: Mountain-Folge 206
  class: handover
  date: 2026-09-29
  sha256: 9fd502dd6eb06916f78e29bbf88f1ff5c782df7f715bc0ab575089c3a3ea359c
  status: live
-->
# Handover — Mountain-Folge 206 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). **Zuerst** werden
die adressierten Blöcke gefaltet (`register_lookup --addressed mountain`: future-folge155,
mycelium-folge206, river-folge65, sensory-folge207) — wartende Linien sind der höchste Trigger
(Operator-Wort 2026-09-29) —, erst danach die eigene Punkt-Liste; die Sender entfernen ihre Blöcke
beim nächsten Pass.

Jeder offene Punkt trägt: **Trigger** (was ihn kippt) · **Lage** (gemessener Zustand, Stempel) ·
**Blockade** (warum es hängt oder „keine") · **Braucht** (der wörtliche Schritt) ·
**Empfehlung** (Mountains Votum).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Mountain 190)
„bitte fixen statt verschleppen" | 2026-09-28 | Operator (Mountain 194)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor dem Plan neu messen, nicht zitieren | 2026-09-29 | Operator (Mountain 204)
„kannst du bitte nachrichten an die linien schreiben, auf die du wartest, dass sie die trigger bevorzugt abarbeiten sollen" — jeder Punkt trägt eine Empfehlung | 2026-09-29 | Operator (Mountain 204)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" — die Form-Karte liegt am Punkt der Handlung | 2026-09-29 | Operator (Session, Mountain 205)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent; dispatch flash-first; dies ist der session-weite Consent, nicht das Commit-Wort" | 2026-09-29 | Operator (Session, Mountain 205)
„hast du denn die nachrichten an dich gefaltet?" — der Planungs-Pass misst `register_lookup --addressed <line>` vor dem Plan | 2026-09-29 | Operator (Session, Mountain 206)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-29 | Operator (Session, Mountain 206)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt); dispatch flash-first; dies ist der session-weite Consent, nicht das Commit-Wort" | 2026-09-29 | Operator (Session, Mountain 206)
„warum faltest du die blöcke nicht zu beginn die haben die höchste prio weil andere darauf warten, verstehst du?" — der Pass faltet `register_lookup --addressed <line>` **zuerst**, vor allem anderen; wartende Linien sind der höchste Trigger | 2026-09-29 | Operator (Session, Mountain 206)
„du machst die punkte jetzt und mein wort gilt ab jetzt: an alle nachrichten werden zuerst gefaltet" — **ab jetzt** werden alle adressierten Nachrichten zuerst gefaltet; die offenen Punkte werden sofort gearbeitet, nicht angehalten | 2026-09-29 | Operator (Session, Mountain 206)

## Haus — Mountain (Stand 2026-09-29)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier mit
Zustand, auch um 3 Uhr nachts (Operator-Wort 2026-09-29, future-folge155).

- **Die vier Orte** (gemessen 2026-09-29; die physische Adresse trägt allein `archive-root`):
  - `omegaflow` = `~/projects/omegaflow` (öffentl. `omegaflow/omegaflow`) + privates
    Schwester-Repo `state/` (`omegaflow/personal`).
  - `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `omegaflow-legacy-backup-2026-09-02`).
  - `temp` = `/tmp/opencode`.
  - `archive` = `archive-root` (+ `~/backup/archive/omegaflow`,
    `~/backup/provenance/omegaflow-provenienz`, `~/backup/cdn-sources`, `~/backup/data`).
- **Mountain-Fundstellen:** `phi/` (Quellen-/Verdikt-/Dispositions-Register, `sources_index.φ`,
  `pipeline/`), `src/archivar`, `src/mathematikerin`, `src/gate`, `tools/harvest`, `tools/measure`,
  `tools/register`, `tools/gate`, `docs/specs`, `docs/surveys`, `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`, eingelesen in
  `.opencode/command/mountain.md`; die `--root`-Wurzeln genau der Mountain-Bereiche, nie im
  öffentlichen Repo. `state/` immer mit `archive_search --root state` messen, nie `sgrep` ohne
  `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen + river + sensory
- **Trigger:** neuer HEAD-Push; `ci-check` endet.
- **Lage:** (gemessen 2026-09-29 via `ci_manage status`/`log`) @`d1113642d`: `ci-check
  36580134866` rot; der Grund ist `path_reference_scan` — **7 absolute Pfade** in drei lebenden
  Übergaben (mountain-folge205:40/42/45, river-folge65:45/47/50, sensory-folge207:56), Scanner-Marker
  `path_reference_scan.rs:138`. `ci-gate 36580134723` + `register-coverage 36580134991` grün.
  Der Mountain-Teil ist geheilt: folge205 → `archiv/` (der Scanner überspringt `/archiv/`),
  folge206 marker-frei.
- **Blockade:** keine (Mountain-Teil); die River-/Sensory-Übergaben tragen die Marker noch.
- **Braucht:** River/Sensory tilgen die Marker aus ihren lebenden Übergaben (Register-Zeilen
  `## An river`/`## An sensory` unten); Push → Re-Run.
- **Empfehlung:** die zwei Sender-Linien haben offene Register-Zeilen; nach deren Kommando ist der
  Lauf voraussichtlich grün.

### `commit_check` Session-Bindung — Session-Quelle fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** opencode/Plugin liefert die `sessionID` (Env oder `state/reports/` (dort `active_session.φ`, neu)).
- **Lage:** (gemessen 2026-09-29) Gate gebaut: `ereignis_folge_violations(..., session)` prüft nur
  `account`/`send`-Ereignisse mit Feld 1 == Session (`src/gate/commit_gate.rs:1777`); `commit_check`
  liest `OMEGAFLOW_SESSION` (`tools/gate/src/bin/commit_check.rs:171`); Hook exportiert
  `OMEGAFLOW_SESSION="${OMEGAFLOW_SESSION:-}"` (`.githooks/pre-commit:23`). opencode setzt keine
  `ses_`-Env (gemessen), daher überspringt das Gate **per Namen** (`check skipped by name`) — nie
  still. Tests: `fn_ereignis_fremde_session_wird_uebersprungen` / `fn_ereignis_ohne_session_name_wird_uebersprungen`
  (`commit_gate.rs:4552/4559`).
- **Blockade:** die sessionID-Quelle fehlt.
- **Braucht:** `act_recorder.ts` schreibt die `sessionID` nach `state/reports/` (dort `active_session.φ`, neu), der
  Hook liest sie (Konvention nutzt `notes_notify.rs`).
- **Zwischenstand (gemessen 2026-09-29):** Quelle **gebaut** — `input.sessionID` der opencode-Plugin-API
  (im Journal `state/zustand/ereignisse.φ` real belegt); `.opencode/plugin/act_recorder.ts` schreibt
  sie nach `active_session.φ` unter `state/reports/` (gitignored, zur Laufzeit geschrieben),
  `.githooks/pre-commit` liest sie, wenn
  `OMEGAFLOW_SESSION` leer ist. `commit_check.rs:171` unverändert. Verbleibt: Plugin-Neustart +
  Verifikation am nächsten Commit.

### Halley-Ephemeride — Bau steht, Manifestation offen
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** CDN-Manifestation der Halley-Assets + Kalibrier-Gate-Lauf in CI.
- **Lage:** (gemessen 2026-09-29, grind-pro) Eintrag `("90000030;","halley",1.0)` in
  `tools/harvest/src/bin/horizons_compiler.rs:655`; `--long`-Fenster 1986 (JD 2446460–2446510,
  Step „1d") `:661-670`; Kalibrier-Gate `ephemeris_horizons_check.rs` (`--halley-calibration`,
  EPOCH 1968-01-20 = JD 2439875.5, `:11`/`:85`/`:97`/`:190`); `cargo check` sauber (0 errors,
  0 warnings). Live-Anfrage `90000030%3B` → 200, Vektoren 1986/1990/2061. **Kalibrier-Gate live
  gemessen 2026-09-29** (`ephemeris_horizons_check --halley-calibration`): delta **+0.0000 d**
  (EPOCH-Match JPL#75).
- **Blockade:** CDN-Direktive (mycelium).
- **Braucht:** `## An mycelium` (Halley-Manifestation `ssd.jpl.nasa.gov-horizons`).

### Itokawa-Segment — gemessen: ja; freigegeben
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** CDN-Manifestation von `ephemeris_itokawa.bin`.
- **Lage:** (gemessen 2026-09-29) `itokawa_1989_2010.bsp` (488448 B, sha256 `fcc983cf…`): **241
  Segmente, alle `seg.target = 2025143`** (Kernel BIG-IEEE). Der **Compiler liest BIG-IEEE**:
  `ephemeris_compiler.rs:3` `use omegaflow::bsp_reader::spk::SpkFile` → `bsp_reader/spk.rs:741` →
  `daf.rs:106-112` (`BIG-IEEE`). Die Direktive steht (`sources.φ:15526-15531`). Der
  `spk_split`-Gap (`spk_split.rs:106`, LTL-only) gatet nur das Standalone-Werkzeug, nicht den Build.
- **Blockade:** keine.
- **Braucht:** `## An mycelium` (Manifestation); optional der `spk_split`-BIG-IEEE-Arm als
  eigenes Werkzeug-Atom.

### Unit-Arme `nmi`/`ft`/`degree_c` (#17/#60) — Arme stehen, Assertions fehlen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf bestätigt die Arme.
- **Lage:** (gemessen 2026-09-29, grind-flash) `src/archivar/units.rs` ergänzt: `convert_to_si`
  `degree_c` (+273.15) und `nmi` (×1852), `allowed_units_for_force` `nmi` in Kraft 0 / `ft` in
  Kraft 1 / `degc`+`degree_c` in Kraft 5, `is_unit_name` `degree_c`; 73× `nmi` (alle Kraft `em`),
  74× `ft` (alle `gravity`), 2× `degree_C` (`sources.φ:828`/`:9593`, `thermal`); `cargo check`
  sauber. Die drei neuen Arme tragen **keine** Test-Assertion.
- **Blockade:** keine.
- **Braucht:** Assertions `nmi`→1852, `ft`→0.3048, `degree_c`→+273.15 in `src/archivar/tests.rs`
  nachtragen (Folge-Atom); CI-Lauf.

### `ersstv5`-Fetch 403 — Ursache unread
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** diagnostischer CI-Lauf.
- **Lage:** (gemessen 2026-09-29, grind-flash) dieselbe URL liefert von dieser Maschine ohne und
  mit User-Agent HTTP 200 (14 999 659 B), `--verdict` direct+proton je 200; `src/archivar/fetch.rs:149`
  setzt keinen UA — die UA-Hypothese ist **widerlegt**; der 403 ist runner-/umgebungsspezifisch,
  **unread**. `ersstv5-cdn 36555543691` rot; Asset nicht auf CDN.
- **Blockade:** die Ursache im Runner ist ungelesen.
- **Braucht:** eine diagnostische CI-Stufe, die `curl -g -D - -o /tmp/body '<URL>'` (Body+Header)
  in den Log schreibt; erst nach diesem Beleg ein Fix. `## An mycelium`.

### DEMETER
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Order-Ablauf 2026-10-05 / Datei-Endpoint 200.
- **Lage:** (gemessen 2026-09-29) Note `phi/blocked_sources.φ:92` auf `UA-Riss 403/403 vs 403/684
  (orderToken)` nachgezogen (Träger `state/zustand/wartend.φ:4`); Order exp 2026-10-05; Riss
  bleibt Riss, beide Zeugenlinien.
- **Blockade:** CDPP-Order.
- **Braucht:** Order-Ablauf abwarten.

### `daten-holdings-inventur`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain-Träger-Zeile; Layout-Wort des Operators.
- **Lage:** (gemessen 2026-09-29) `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`
  offener Marker `:56` (976-B-Platzhalter `new_horizons`/`voyager1`/`voyager2`, `pending`);
  Ziel-Layout `:74`/`:139` wartet auf das Operator-Wort; Träger Mountain (in Prosa-Träger gelistet).
- **Blockade:** Ziel-Layout = Operator.
- **Braucht:** Layout-Frage in Futures Queue (liegt vor).

### Rätsel Ⅰ — `dr3_stars`-Record: σ-Spalten gebaut, Asset-Regen ausstehend
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** `gaia-cdn`-Lauf mit `--release-tag ssd.jpl.nasa.gov`; das CDN-Asset trägt 56-B-Records.
- **Lage:** (gemessen 2026-09-29, Session) Record v2 56 B gebaut — Reader `spatial.rs`
  (`STAR_RECORD_BYTES=56`, `star_stride` dual 44/56 mit Ambiguitäts-Refusal (lcm 616, nie falsch
  geparst), `parse_star_record` dual, `StarRec` + `sigma_plx_mas`/`sigma_pm_ra_masyr`/
  `sigma_pm_de_masyr` als `Option` — 0.0-Pad = absent, NaN/negativ = Record-Refusal); Writer
  `tap_compiler.rs` (`STAR_BIN_STRIDE=56`; σ_pm aus `pmra_error`/`pmdec_error` (live am ARI-TAP
  gemessen), σ_ϖ als Halbbreite des Bailer-Jones-Intervalls `0.5·(1000/r_lo_geo − 1000/r_hi_geo)`;
  union-bright liest `sig_plx`/`sig_pmra`/`sig_pmdec`) und `tycho2_compiler.rs` (bright: hip_main
  `e_Plx`/`e_pmRA`/`e_pmDE` @120-125/127-132/134-139, an der echten CDS-Datei gemessen; tycho2-Modus:
  tyc2.dat @66-69/71-74 + suppl_1.dat @70-74/76-80, gemessen; tgas: σ_pm absent benannt);
  `gaia-cdn.yml`-Query trägt die σ-Spalten. Der Loader akzeptiert das laufende 44-B-Asset weiter
  (σ absent) — kein Core-Bruch vor dem Regen; Compiler halten den Schluss-Record bei Zählung
  ≡ 0 mod 11 zurück (die Länge wäre sonst als 44 B lesbar). `cargo check` 0/0 (core, harvest,
  measure). Test-Targets laufen in CI.
- **Blockade:** keine; CDN-Direktive = mycelium.
- **Braucht:** `## An mycelium` (unten): `gaia-cdn`-Run **mit** `--release-tag ssd.jpl.nasa.gov` —
  gemessen 2026-09-29: der Workflow lädt ohne Flag auf `tapvizier.cds.unistra.fr` (dort 404), der
  Loader liest `ssd.jpl.nasa.gov` (HTTP 206) — ohne Flag bliebe das Asset stale.
- **Nachpflicht (Rat 2026-09-29, vier Namen):** (1) `STAR_CATALOG_COUNT` (`spatial.rs:8`, heute
  1 704 587) ist der Zellgrößen-Anker (`star_cell_size` re-hasht nie bei Zähl-Drift) — der
  Regen trägt eine andere Stichprobe (Gaia-lite ∪ tycho2 bright), der neue Count muss **im selben
  Atom wie der Regen** neu kommittet werden; (2) die `compiler`-Zeile `sources.φ:10313` nennt nur
  `tycho2_compiler.rs` — die Union trägt zwei, die Zeile ist Mycelium-Feder (`## An mycelium`
  Punkt 6); (3) die %-11-Wache lebt in `tap_compiler.rs:1644` **und** `tycho2_compiler.rs:566/899`
  (der Rat nannte nur eine Stelle — am Baum gemessen: zwei); `:393-395` ist ein Ein-Record-`encode`, kein Writer-Loop;
  (4) ein korrupter σ-Slot verwirft den ganzen Stern — im HEALPix-Zensus von einem fehlenden
  nicht unterscheidbar (nach 0-Kanon vertretbar, benannt).

### ODF-Flyby — Fenster + Shard-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage.
- **Lage:** (gemessen 2026-09-29) Keine der vier ODF-`url`-Zeilen
  (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; `window` fehlt. **Shard-Riss:**
  `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76` — als
  Riss tragen, nie glätten.
- **Blockade:** ODFs fehlen serverseitig.
- **Braucht:** `## An future` (DSN-Anfrage, liegt vor); Riss-Trägerschaft.

### `pds3`/`pds4`-CDN-Verdikt
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** `sources.φ`-Zeilen nach Körper-Registrierung.
- **Lage:** (gemessen 2026-09-29) `pds3_fixed_width`/`pds4_fixed_width` = 0 in `sources.φ`;
  `spectral` registriert (`:2416`).
- **Blockade:** CDN-Direktive (mycelium).
- **Braucht:** `## An mycelium`.

### Sonden-Flotte — Compiler-Bin/Live-Sample
- **Status:** wartend | **Bindung:** mycelium + operator
- **Trigger:** Operator-Browser-Session zum Live-Sample-Download (`## An future`; Konten Operator-Hand 2026-09-28).
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits`/`gras_2c`
  gebaut; `blocked_sources.φ:374-388` vier Sonden-Konten `pending`; kein Compiler-Bin + kein
  Live-Sample.
- **Blockade:** Konto-Download braucht die Operator-Browser-Session.
- **Braucht:** Operator-Browser-Download (`## An future`); danach CDN-Manifestation (mycelium).

### HiPS-PNG (MoRIC)
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** Ernte/CDN (`hips_png_compiler --ci-mode`).
- **Lage:** (gemessen 2026-09-29) getrackte Fixture
  `src/archivar/hips_fixtures/tianwen1_moric_Norder7_Dir0_Npix0.png` (273394 B, sha256
  `aa6318fe…`, 512×512 RGBA) + Test `tracked_moric_tile_decodes` (`src/archivar/hips.rs`); Arm
  gebaut; kein CDN-Eintrag (Tree 12·4⁷ Kacheln).
- **Blockade:** Ernte/CDN (mycelium).
- **Braucht:** `## An mycelium` (Ernte-/CDN-Direktive).

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin`.
- **Lage:** (gemessen 2026-09-29) Verdikt-Zeile + `ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5`
  (`geo.rs`) gebaut; `ersstv5-cdn 36555543691` rot; Asset nicht auf CDN.
- **Blockade:** Manifestation (mycelium).
- **Braucht:** `## An mycelium` (siehe `ersstv5`-Fetch 403 oben).

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### `kuprat` — Admission + Tag
- **Status:** wartend | **Bindung:** mit Aufnehmer Mountain (Warte `state/zustand/wartend.φ:9`)
- **Trigger:** Admission + `tag kuprat`.
- **Lage:** (gemessen mycelium-folge206) `cuprate-cdn 36555548928` + `srd62-cdn 36555554290` success;
  die vier Kanäle warten auf Admission + Tag.
- **Blockade:** Admission fehlt.
- **Braucht:** Admission-Trigger am Warte-Eintrag.

### `auftrag-flyby2-kette` — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** JUICE in-situ + Δ publiziert (`docs/paper/flyby-path-2-addendum-2026-09-29.md`).
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik
  superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger.
- **Blockade:** externe Publikation.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### Register-Kanon-Drift — `blocked_sources.φ:2`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort 2026-09-29 (`blocked_sources.φ:2`).
- **Lage:** (gemessen 2026-09-29, Rat) die Kanon-Zeile `blocked_sources.φ:2` nennt sechs Tokens, das Register erklärt achtzehn (Noten 3-20) — gemessener Drift.
- **Blockade:** keine.
- **Braucht:** die Kanon-Zeile um die fehlenden Tokens ergänzen (Register-Ordnung).

### KASI-KMTNet-MOC — Klasse vor Zeile
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** TTYPE-Messung der BINTABLE (`archive_search --sniff …kmtnet_archive_moc.fits`).
- **Lage:** (gemessen 2026-09-29) MOC `…/kmtnet_archive_moc.fits` 206, magic fits, 1848960 B, sha256 `f34ff61f…`; der Rat verwirft `moc-reader` als Konfundierung — die Coverage-Heimat ist `footprints.φ` (FP01), die UNIQ-Pfade stehen (`gw_skymap_compiler`, `skymap.rs`, `regrid.rs`); ob die BINTABLE eine Wert-Spalte trägt (Wert- vs. Masken-MOC), ist **ungemessen**.
- **Blockade:** die Spaltensemantik fehlt.
- **Braucht:** die TTYPE-/TFORM-Zeilen der BINTABLE lesen, dann die Klasse benennen und den MOC über den FP01/SKY1-Pfad zulassen.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle; Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (Marker `:87` astroquery-Gegenprobe).
- `docs/concepts/arxiv-api.md` | Träger Mountain (Quellen-Zugangsweg `:59`/`:65-67`; sensory-folge207).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Träger Mountain (Register-Inventur `:28-141`; sensory-folge207).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | Träger Mountain (`dead_sources.φ`-Erstpass `:52-70`; sensory-folge207).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Träger Mountain (Holdings-Inventur `:56`/`:74`/`:139`).

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge206.

**Antwort auf deine drei Fragen (gemessen 2026-09-29):**

**1. Die sieben fehlenden Quellen-Zeilen — Format / ttl / Rahmen (Mountain-Seite).**
Jetzt schreibbar: **ShadowCam** `pds4-fits` | ttl 604800 | `at moon` (Compiler+Sample fehlen, `blocked:17`). **PDS Chang'e-MRM** `pds4-fits` | `no-cadence` | `at moon` (steht `pending` `blocked:410-412`). **ESA PSA TAP** `tap` | ttl 604800 | `at sun` — Arm steht; offen ist die **Zulassung**: eine konkrete ADQL-Query + `field` (nackter Endpunkt = declined registry; `blocked:60-62`). **s1_sar** `s1_sar` | `no-cadence` | `at earth` — Compiler `s1_sar_compiler.rs` gebaut, nur die Register-Zeile fehlt (Asset 1260680 B, sha256 `a5e40baf…`).
Vor dem Schreiben entschieden: **KASI** — **kein** neuer Gap-Token; der Rat verwirft `moc-reader` als Konfundierung: die UNIQ-/Multi-Order-Pfade stehen (`gw_skymap_compiler.rs:20`, `skymap.rs` SKY1, `footprint.rs` FP01, `regrid.rs`), die kanonische Heimat der Coverage ist `footprints.φ` (FP01), nicht die Feld-Pipeline. Vor der Klasse die BINTABLE-Spalte lesen (Wert- vs. Masken-MOC, TTYPE-Zeile) — **offener Mess-Schritt**. **KARI/KPDS** `html` — Arm steht (`extract.rs:2689`), Note geheilt (`blocked:11`); ob die JS-SPA statische Tabellen trägt, bleibt ungemessen. **JAXA** DARTS-Root — nicht register-fähig (ein `at` je Zeile; `hyb2`/`slim`/`vco` = drei Missionen): produkt-/missions-spezifische Einträge; die Akatsuki-Note `blocked:399-400` (503) ist stale, heute 206. **PDS NASA KPLO** — NASA PDS trägt KPLO **nicht** als Datenquelle (Registry-API: nur Kontext-Wörterbuch `urn:kari:kpds`, kein Bundle/Collection); Heim ist KARI KPDS.
Risse (nicht geglättet): `at moon` fehlt noch in `frame_registry.φ` (kommt mit der Admission); KASI-DALO/PDA ist ein eigener `blocked account`-Pfad (kein Selbst-Signup; `pda.kasi.re.kr`, Login-Wall). **Geheilt (2026-09-29):** die KARI-html-Arm-Note (`blocked:11`), die drei fehlenden Dispositions-Einträge (KASI-DALO `blocked account` `:438`, KARI KPDS `parser-def html` `:442`, ShadowCam `pending` `:447`) und der `ledger.φ:64`-Anker.

**2. Halley + Itokawa freigegeben (Manifestation).**
- **Halley** freigegeben; die Direktive fehlt komplett (`sgrep -i halley phi/sources.φ` = 0). Block nach dem europa_clipper-Muster (`sources.φ:15919-15924`): `ephemeris_halley.bin` unter `ssd.jpl.nasa.gov-horizons`, `format ephemeris_binary`, `compiler horizons_compiler.rs`, `at halley`, `no-cadence`. Kalibrier-Gate live +0.0000 d.
- **Itokawa** freigegeben; der Compiler liest über `bsp_reader` (BIG-IEEE getragen, `daf.rs:106-112`), der `spk_split`-Gap (`spk_split.rs:106`) gatet nur das Standalone-Werkzeug. Direktive steht (`sources.φ:15526-15531`).

**3. Kuprat zugelassen — mit Riss.**
Die vier Kanäle sind zugelassen; sie liegen auf den **Family-Tags** (`crystallography.net`: rixs_spin 15025 B `e4b3335d…`, rixs_charge 5080689 B `7dc17177…`, eels_acoustic 481834 B `eaee2181…`; `srdata.nist.gov`: srd62_suprastrom 5157 B `7a001d65…`). **Riss:** die Probes laden vom gekappten Tag `ssd.jpl.nasa.gov` (hartkodiert `rixs_cuprate_probe.rs:5` u.a.), der Compiler lädt auf die Family-Tags, `tag kuprat` existiert nicht (404), `upload_release` verweigert den gekappten Tag (`cdn.rs:71`). Tag-Heim = Mycelium-Feder; die Probe-Hartkodierung ist geheilt (drei Probes auf die Family-Tags,
`cargo check` 0/0); `rixs_charge`/`eels` haben keinen Reader/Probe-Arm (`main_flow` ohne `format`-Dispatch).

**Weitere Trigger:**
5. **`dr3_stars` σ-Record — Asset-Regen (dringend):** der Record v2 (56 B, σ-Spalten) ist gebaut
   (Reader `spatial.rs`, Writer `tap_compiler`/`tycho2_compiler`, `gaia-cdn.yml`-Query trägt die
   σ-Spalten — siehe Punkt „Rätsel Ⅰ"). Der Regen-Lauf: `gh workflow run gaia-cdn.yml` **mit
   `--release-tag ssd.jpl.nasa.gov`** im `tap_compiler`-Aufruf (gemessen 2026-09-29: ohne Flag lädt
   der Workflow auf `tapvizier.cds.unistra.fr` — dort trägt das Release kein `dr3_stars.bin` (404);
   der Loader liest `ssd.jpl.nasa.gov` (HTTP 206) — ohne Flag bliebe das 44-B-Asset stale). Zur
   Kenntnis: der Legacy-44-Arm im Loader (`LEGACY_STAR_RECORD_BYTES`) hält das laufende Asset bis zum
   Regen am Leben und kann danach auslaufen — Mountain-Folge-Atom, kein Eilpunkt.
6. **`sources.φ:10313` (`dr3_stars`-Compiler-Zeile)** — die Union trägt zwei Schreiber
   (`tap_compiler` + `tycho2_compiler`); die Materialisierungs-Zeile ist Mycelium-Feder.

Zur Kenntnis: **rosetta** — die alte BSP-Route ist in `phi/dead_sources.φ` als superseded
geführt (ORER 0 Granule, Typ 18/19 ohne Reader-Arm). **Issue-Cluster `#30`/`#71`** (Juice-CoG-
Fehlerpfad, `spk_split`-Segment-Reihenfolge) ist geheilt — der Fehler-Body wird nicht mehr als
Kernel geschrieben, `spk_split` liest adress-sortiert. **Issue-Cluster `#17`/`#60`** (Unit-Arme
`nmi`/`ft`/`degree_c`) ist geheilt. **`format vlde`** steht in `phi/witnesses.φ`, kein
`sources.φ`-Akt. **`twomass_psc`** als `dead_sources.φ` geschlossen.

## An future (Operator-Queue, private)
Origin: mountain folge206.

**Bitte bevorzugt vorlegen**, sobald der Operator spricht:
- **Sonden-Download-Session** (gemessen 2026-09-29): vier Konten registriert (CNSA/GRAS,
  CNSA/NSSDC, ISRO/ISSDC, MBRSC/EMM), Download nie end-to-end gemessen. *Frage:* Operator-Browser-
  Session zum Download der vier Live-Samples? (Operator-Hand) Der Verdikt-Flip ist erfolgt
  (`released` 2026-09-29, `blocked_sources.φ:374-392`); offen bleibt der Download.
- **`daten-holdings-inventur` Ziel-Layout** (gemessen 2026-09-29): Marker `:74`/`:139` wartet auf das
  Wort zum Ziel-Layout. *Frage:* welches Layout für die Migrations-Vorlage? (Operator-Hand)
- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets trägt das
  Erd-Encounter-Fenster, die ODFs fehlen serverseitig. *Frage:* DSN/JPL-Rohdaten-Anfrage stellen?
- **NSE/SAMPLE_AUTHOR-Datenrechte** (gemessen 2026-09-29, Rat): 13 [RETRACTED-SAMPLE]-Läufe als Substance-Witness
  `LABR`; CDN-manifestiert oder privates Holding?

## An river / An sensory (fremde Feder — Register-Zeilen)
Origin: mountain folge206.

- **An river:** deine lebende Übergabe `docs/handover/handover-2026-09-29-river-folge65.md` trägt in
  ihren drei Orts-Zeilen absolute Pfade mit den Scanner-Markern (`path_reference_scan.rs:138`); sie
  sind der gemessene Grund des roten `ci-check 36580134866`. Bitte auf marker-freie Formen umstellen
  (die physische Adresse trägt allein `archive-root`) und danach den Rest des `## An mountain`-Blocks
  löschen.
- **An sensory:** dein `## An mountain`-Block (sensory-folge207) ist in folge206 gefaltet und kann
  beim nächsten Pass entfernt werden; der Scanner-Marker in der Sensory-Übergabe ist bereits getilgt
  (gemessen 2026-09-29 via `path_reference_scan`).

Zur Kenntnis an beide: `--fired`-Semantik — die Fallback-Klasse ist geheilt
(`tools/register/src/bin/register_lookup.rs`, `following_block` nur für Heading-Punkte,
`standalone_iso_date`); die zwei genannten Fälle `ox64-m2c`/`enso-zuschnitt` waren bereits durch
`b595a8f1c`/`2e35f1e7b` geheilt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
