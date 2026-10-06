<!--
  title: Handover — River-Folge 105 (2026-10-06)
  session: River-Folge 105
  class: handover
  date: 2026-10-06
  sha256: 740d4df0debf3ddd5dbcdafa2d5365d5ad77e96ad83f95bbba03c348852c3b3c
  status: live
-->
# Handover — River-Folge 105 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 100–105) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
„earth-wgs84/legacy-assumed können wir den nicht migrieren ich möchte eigentlich kein legacy haben / deklarieren" — der Volume-Frame ist Pflicht-Deklaration je Quelle, kein Legacy-Bucket, keine Migration | 2026-10-06 | Operator (Session, River 99)
„kannst du die frage bitte noch den voices und glm und claude online chat geben" — die Ratsfrage zusätzlich an die Schwarm-Stimmen, glm und Claude-online | 2026-10-06 | Operator (Session, River 104)
„bitte lege die fragen dem rat vor und dem schwarm und gib sie mir davor für den online chat mit claude und glm" — die drei Stemm-Fragen (Membran-Hänger, Paper-Riss, Fenster-Kante) | 2026-10-06 | Operator (Session, River 105)
„muss nicht die sonne zuerst sichtbar sein können wir nicht nach helligkeit/sichtbarkeit/erreichbarkeit progressiv laden?" — progressives Laden nach Sichtbarkeit, **Sonne zuerst** (nicht Sterne zuerst) | 2026-10-06 | Operator (Session, River 105)
„die agenten haben archive search root genutzt und ich habe es aus versehen erlaubt" — ein lokaler `archive_search --root` erreichte eine Schwarm-Stimme über eine Laufzeit-Freigabe; die Schwarm-Antworten dieser Runde sind kontaminiert | 2026-10-06 | Operator (Session, River 105)
„ich meine browser nutzung wäre schon gut sollen wir eine neue voices chrome instanz anlegen in der keine logins gespeichert sind?" — Browser-Nutzung der Voices bejaht, aber über eine eigene Chrome-Instanz ohne Logins | 2026-10-06 | Operator (Session, River 105)
„ich glaube, es ist sinnvoll auch dem schwarm die stimmen zu geben, oder zumindest die selben agenten wie grind general council auch nochmal als voices/schwarm agenten zu haben" — Schwarm um die deepseek-Seats (council/grind-flash/general = flash; grind-pro/grind-max/research-max = pro) erweitert | 2026-10-06 | Operator (Session, River 105)
„die können jetzt auch als council/grind/general eingesetzt werden … ich hätte gern, dass sie auch architekturfragen mit den fünf stimmen beantworten — das ist counter slope" — die deepseek-Schwarm-Seats tragen die Fünf-Stimmen-Verfassung im Prompt | 2026-10-06 | Operator (Session, River 105)
„nein, ich meine: momentan fragen wir bei Architekturfragen nur deepseek — können wir nicht auch den Schwarm befragen, aber durch die Linse der 5 Stimmen" — **alle 14** Schwarm-Voices tragen die Fünf-Stimmen-Verfassung (nicht nur deepseek) | 2026-10-06 | Operator (Session, River 105)
„und tragen sie auch die axiome?" — die 14 Schwarm-Voices tragen die bindenden Axiome im Prompt (A = A, 0-Kanon, keine Fabrikation/Defaults, Riss, 0 honored, jede Aussage trägt ihre Messung) | 2026-10-06 | Operator (Session, River 105)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge104.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 `:689` („no reported value
  applies it yet — the wiring stays open") gegen den Baum geprüft: `bias_column`
  (`tools/measure/src/bin/field_te_query.rs:2619`) liefert bei `n_eff ≥
  TE_NEFF_THRESHOLD` eine **Report-Site-Spalte**; der Report liest das Gate selbst
  (`fmt_opt(TE_NEFF_THRESHOLD)`, `:2934`) und druckt `bias (report site only; raw TE
  untouched)` (`:2933`). **Kein Riss** (Rat + GLM): „reported value" = die rohe TE, die
  unangetastet bleibt → das Papier stimmt; Claude: Satz unscharf, eine präzisierende
  Formulierung („außer der Report-Site-Bias-Spalte, rohe TE unverändert") ist optional.
  Kein Code-Akt. Offen: BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) — §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve
  (estimator-fest), `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Punkt 1
  (`body_in_enclosure`-Bypass) gebaut: `dcc3243f8` (mountain 238) „agnosis anchor
  bypass removed"; die übrigen Code-Punkte liegen bei ihren Owner-Linien
  (`frames.rs`/`weberin.rs` → Mountain/Sensory; `membrane.html`-Trio → Mycelium/CI).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `te-bias-n` Re-Lauf `37443829535` grün.
- **Lage:** (gemessen 2026-10-06, River 105 via `ci_manage view`)
  - **Binned-Kurve gelandet:** `TE_BIAS_MK_BINNED` (`src/mathematikerin/te.rs:63`) trägt die
    8 gemessenen Punkte aus `te-bias-n 37440043700 @72c9348ce`.
  - **Matrix-Zelle reicht echtes `kde_n_eff` durch** (`CellTe { te, n_eff, surrogates }`,
    `cell_te_and_surrogates(target, driver, lags, …)` `field_te_query.rs:4122`); der
    Report führt eine `n_eff`-Spalte.
  - **Pfeil-Riss geschlossen:** `binned(target, driver)` = `d→t`; Gate
    `matrix_cell_measures_the_arrow_of_its_target_driver_order`.
  - **Kalibrier-Riss benannt:** `te_bias_n_probe.rs:223` auf `(target,driver)` korrigiert;
    `TE_NEFF_THRESHOLD` = `Some(1.8166e1)` (`src/mathematikerin/te.rs:110`) ist **noch** die
    driver-first-Kalibrierung.
  - **Lauf-Stand:** `te-bias-n 37443829535 @2b7acb4d4` = **queued** (seit 09:33:54Z, Stand
    09:39Z; Vorgänger `37443567797` cancelled).
- **Blockade:** die 8 probe-Kanäle sind nicht am Draht.
- **Braucht:**
  (1) `te-bias-n 37443829535` grün lesen → das n=800-`kde_n_eff`-Mittel im korrigierten
  `(target, driver)`-Auftrag in `TE_NEFF_THRESHOLD` setzen + Gate.
  (2) binned-spezifischen Floor aus der Probe ableiten (Histogramm-Besetzung, nicht KDE-Bandbreite).
  (3) conditional-embedded Probe `pending`: `conditional_embedded_te_estimate` über coupled
  Hénon mit Konditionsreihe, nicht Phase-Surrogat.
  (4) 8 probe-Kanäle an den Draht + 15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`).
  (5) `ozzy` **auf** der Matrix (`auftrag-universelles-vlies.md` §Lieferung).

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 105 via `ci_manage view`) `ci-check 37440906455
  @d4b4a8e93` = **cancelled** (Push-Konkurrenz, kein Verdikt); der aktuelle HEAD `b71658f`
  trägt `ci-check 37444174960` = **pending**. River-Seite gebaut: WGSL-Gates
  `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`; Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`. Mountain-Seite gebaut (mountain-239).
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check 37444174960` abwarten (Stehender Pass; kein Polling); grün → Punkt fällt,
  rot → `ci_manage log 37444174960`.

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line Mountain/Mycelium)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN (fine-grain GIC ist erfüllt).
- **Lage:** (gemessen 2026-10-06, River 105) Viljanens Empfehlungen (2026-10-05, `mail_ledger.φ:235`)
  sind **weitgehend umgesetzt**:
  - ✅ **Fine-grain FMI-GIC:** `fmi_gic_1min.bin` registriert (`phi/sources.φ:17338-17345`, `format
    fmi_gic_1min`, `origin …/gic/man_ascii`, `compiler fmi_gic_compiler.rs`, `on earth 60.6 25.2 0`),
    Formate-Arm `extract.rs:818`/`geo.rs:295`/`main_flow.rs:4121`, Workflow `fmi-gic-cdn.yml`
    (`--grain minute`), **manifestiert** (sha256 `a30a846d…`).
  - ✅ **IMAGE/NUR-Zeile** registriert (`phi/sources.φ:17347-17353`, `fmi_image_mag_nur.bin`,
    `image_mag_compiler.rs`, `field fmi_image_mag_dxdt`).
  - ✅ **Paper §4** (`docs/paper/gic-causal-driver.md:599-621`) trägt CC BY 4.0 (Ari Viljanen
    2026-10-05), die NUR-Empfehlung, die Caveats (nicht-uniform, beste Qualität 1999–April 2005,
    Tages-Lineartrend-Subtraktion) und Viljanen et al. 2025 (`angeo-43-271-2025`, Eq. 43/Table 1).
  - ⬜ **NUR nicht im CDN:** `fmi_image_mag_nur.bin` hat **keinen sha** und **keinen
    Manifest-Workflow** (`image-cdn.yml` existiert nicht; kein `.github`-Treffer für `image_mag`) —
    der Compiler steht, die Manifestation fehlt.
  - ⬜ **Tages-Lineartrend-Subtraktion** (Viljanens Caveat) ist **nicht** implementiert (kein
    `trend`/`detrend` in `fmi_gic_compiler.rs`/`image_mag_compiler.rs`).
  - ⬜ **Relation noch nicht gemessen:** Paper `:617` nennt sie „named pending measurement, not yet
    run"; `gic_storm_probe.rs` ist der storm-selective ABK/SOD-Arm, nicht der dB/dt(NUR)–GIC-Paar-Test.
- **Blockade:** NUR-Manifestation (kein Workflow) + Probe.
- **Braucht:** (1) `image-cdn.yml` (Mycelium) für `fmi_image_mag_nur.bin` + sha ins Register;
  (2) ggf. Tages-Detrend im Compiler (Mountain); (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in
  Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation
  der DE440-`.bin`.
- **Lage:** (gemessen 2026-10-06, River 105)
  - **CDN-Assets erfüllt** (mycelium-240, gefaltet): `ephemeris_de440_{earth,moon,sun}.bin`
    same-origin HTTP 206, sha256 == `pages-deploy.yml:60-62`; `dr3_stars.bin` `745a3f71…`.
  - **Client-Pfad bootet bis zur Stars-Phase** (River 105, `archive_search --playwright
    https://omegaflow.space/membrane.html`, 2 Läufe): Titel „omegaflow — membrane", HTTP 200,
    Status fortschreitend `loading stars… 27%`/`40%`. Die Ephemeriden-Phase wurde **nicht
    erreicht** — „kein Hänger am Boot-Pfad" ist eine Inferenz aus dem Code, kein Befund
    (Claude-Stemme 2026-10-06).
  - Der Sonne/Erde/Mond-Anker fehlt weiter: `body_anchor_samples`
    (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g` (stype7) oder `props.gm`;
    die deployte `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar. Rat 2026-10-06:
    Mechanismus (a) — `val` = gemessener GM, `force_type = 1.0`; Riss „Maske sagt absent, Quelle
    trägt gm", `pending`. `omega.rs:872` konsumiert bereits `v.frame_body` (mountain-239).
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden nach Sichtbarkeit (gebaut) + Folgeatome D/C
- **Status:** eigen (gebaut) | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** — (in diesem Atom; Wort 2026-10-06).
- **Lage:** (gemessen 2026-10-06, River 105)
  - **Operator-Wort:** progressiv laden nach Helligkeit/Sichtbarkeit/Erreichbarkeit, **Sonne
    zuerst** (nicht Sterne zuerst). Rat 2026-10-06: (B) `add_stars` ist der Schlussstein, (A)
    allein verfehlt das Wort (der 95-MB-Sternfile gated weiter den ersten Pixel), (D) ist der
    notwendige Begleiter, (C) ein Folgeatom.
  - **Gebaut (B):** `src/wasm.rs` `MembraneLookup::new` startet ohne Katalog (leerer Slice
    legal) + `pub fn add_stars(&mut self, bytes, epoch)` (extend + `hash = None`); der nächste
    `query` rebuildet den Hash aus dem gewachsenen Set. `cargo check` grün (native; das
    wasm-Modul baut im `pages-deploy`-CI, `wasm-pack --target web`).
  - **Gebaut (A):** `static/membrane.html` `boot()` öffnet `new MembraneLookup(new
    Uint8Array(0), epoch)`, startet die GPU, lädt die Körper **sequenziell in
    Sichtbarkeitsreihenfolge** (`BODIES = ["sun","earth","moon"]`), rendert nach jedem
    (`frame()`), dann die Sterne via `add_stars`. `fetchBytes` trägt das Byte-Alter-Stall
    („no bytes for N s").
  - **Riss (Epoch):** `static/membrane.html` `CATALOG_EPOCH_YR = 2000.0` gegen
    `phi/sources.φ:17466 catalog_epoch 2016.0`; der Compiler `tycho2_compiler.rs` trägt keinen
    Epoch, das `.bin` auch nicht. Owner des Wertes: **Mountain** (Verdikt/Register); der
    Consumer-Kommentar (River) trägt beide Zeugen, wird **nicht** still gesetzt.
- **Blockade:** der erste Pixel wartet weiter auf den 193-MB-Sonnen-`.bin` (A+B ohne D).
- **Braucht:**
  (D) Mycelium + River — Range/Fenster der erreichbaren Ephemeriden-Granule, damit die Sonne
  klein wird (gated: Layout-Messung des `.bin`; ohne Granule-Index → Compiler emittiert ein
  windowed Asset, Trigger);
  (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen (Trigger: Dateiordnung ≠ Helligkeit);
  ein Playwright-Lauf mit langem Timeout bis zur gerenderten Wolke (Ready-Flag / Canvas nicht
  schwarz) ist die noch fehlende Messung.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line: Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest der Hüllen-Pipeline (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06) Punkt (b) gebaut (mountain-239):
  `Volume.frame_body: Option<String>` (`src/archivar/volume.rs:155`), `Extract::Volume.frame_body`
  aus `at` (`src/archivar/parse.rs:353`), `upload_volumes` konsumiert `v.frame_body`
  (`src/mathematikerin/omega.rs:872`), Test `volume_observer_declared_and_refused_when_absent`.
  Offen nur (a): `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` →
  Build-Time-Manifest.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon (aus `state/zustand/wartend.φ` portiert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105; wartend.φ:34-36)
  - **OMNI2:** HAPI-Code 1201, alle 26 Zellen `pending`; Rohdatei
    `data/cdaweb.gsfc.nasa.gov/omni2-20260926-20260929.csv` liegt lokal
    (EXISTS gemessen 2026-10-06); Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md`
    §OMNI2 `:182` wartet auf den Merge-Lag.
  - **kp `def`:** `def`-Query leer; Prelim revidierte Zellen 23–25 auf 0.667; `def` `pending`.
  - **JUICE-recon:** `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent; Wiedervorlage
    2026-11-01, dann erneut `archive_search --verdict`.
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** (1) `flyby_path2_fill`-Lauf (CI) lesen + Addendum-Zellen fortschreiben;
  (2) OMNI2/kp/JUICE-Trigger feuern lassen; (3) Δ/σ_recon post-flyby.

### Voices-Chrome — eigene login-freie CDP-Instanz (Config gebaut)
- **Status:** eigen (Config gebaut; Verifikation nach Neustart) | **Bindung:** eigen
- **Trigger:** opencode-Neustart + Voice-Test.
- **Lage:** (gemessen 2026-10-06, River 105)
  - **Rat (2026-10-06):** `browser_*` (Operator-Chrome-Bridge) für Voices sperren; die saubere
    CDP-Instanz gewinnt; `--browserUrl`-Signatur ungemessen → Dispatch 0.
  - **Dispatch 0 (gemessen):** `chrome-devtools-mcp@1.9.0 --help` zeigt: das MCP **startet
    selbst** einen Chrome — `--headless`, `--isolated` (temporäres Profil, autogereinigt →
    dauerhaft keine Logins), `--userDataDir`, `--executablePath`, `--allowedUrlPattern`
    (Chrome 149+), `--blockedUrlPattern`, `--proxyServer`. System-Chrome **154.0.8037.97**,
    `/usr/bin/google-chrome` vorhanden.
  - **Gebaut:** `opencode.json`-MCP `chrome-devtools-voices` =
    `npx chrome-devtools-mcp@1.9.0 --headless --isolated --executablePath /usr/bin/google-chrome
    --allowedUrlPattern "https://*" --no-usage-statistics --no-performance-crux`; in **allen 12**
    Voice-Profilen `"browser_*": "deny"`, `"chrome-devtools_*": "deny"` (Operator-Chrome),
    `"chrome-devtools-voices_*": "allow"`. Der Rat-Plan (Launcher + PAC + `--browserUrl`) ist
    damit **abgelöst** — das MCP kann es selbst (gemessen).
  - **Grenzen:** nur `https://*` (default-deny) → `http`-only-Seiten und Cloudflare/gated bleiben
    Operator-Route; `--isolated` → nichts persistiert. Restrisiko: der CDP-gebundene
    Chrome-Prozess und DNS-Rebinding sind browser-seitig nicht gelöst.
- **Blockade:** opencode-Neustart nötig (Config nicht hot-reloaded); `opencode.json` trägt fremde
  uncommittete Voice-Definitionen — mit-committet, benannt.
- **Braucht:** Neustart; dann Voice-Test: öffentliche https-Seite via `chrome-devtools-voices_*`
  lädt, `browser_open`/`chrome-devtools_*` = denied, `http://127.0.0.1:1618` blockiert.

## Rat + Schwarm 2026-10-06 — Stemmen (Membran-Hänger / Paper-Riss / Fenster-Kante)

Operator-Wort 2026-10-06: die drei Fragen dem Rat + Schwarm vorgelegt und dem
Operator für den Claude/GLM-Chat gegeben. Stemmen (gemessen 2026-10-06 via `task`
council + Schwarm-Stimmen):

- **Frage 1 (Membran-Hänger):** Rat **(b)** — der eingefrorene Status ist die fehlende
  Messung, kein Deadlock; drei Labels rasen auf einer Statuszeile → ein **aggregierter**
  Zähler (Σgot/Σtotal, eine Zeile), ein **Stall-Kriterium** (keine Bytes über ein aus der
  gemessenen Chunk-Kadenz abgeleitetes Intervall → „noch keine Bytes", nie 0 %
  fabriziert), und als Wurzel ein **kleineres/gerange-tes Asset**. Stimmen gespalten:
  voice-gptoss + voice-nemotron `(a)` (Label je Körper genügt); voice-ling `(b)`
  (Timeout/AbortController).
- **Frage 2 (Paper-Riss):** Rat **(a)** — Lesart (i), **kein Riss**: `bias_column`
  (`field_te_query.rs:2619`) liefert bei `n_eff ≥ TE_NEFF_THRESHOLD` eine adjustierte
  Spalte; der Report nennt sie selbst „report site only; raw TE untouched" (`:2933`);
  „reported value" = die rohe TE, die unangetastet bleibt → Papier korrekt, höchstens
  eine präzisierende Formulierung, die die Bias-Spalte beim Namen nennt. Mehrheit `(a)`
  (voice-nemotron, eine weitere); voice-gemini + voice-gptoss `(b)`.
- **Frage 3 (Fenster-Kante):** Rat **(b)** — `static/membrane.html` ist eigener
  Membran-Pfad, **autonom editierbar**: die Kante liegt an der **Wirkung** (ein natives
  Operator-Rechner-Fenster öffnen/strahlen, oder den Blick ändern: Framing, Exposure,
  Zoom, Farbe, Blend), nicht am Dateinamen; der Daten-/Status-Pfad ist Rivers Haus.
  Stimmen gespalten: voice-gemini `(b)`; voice-gptoss + voice-ling `(c)` (Kante anders:
  Browser-Asset, aber der Membran-Vertrag bleibt Rivers Domain); voice-nemotron `(a)`
  (operator-gebunden).
- **Nicht verfügbar (benannt, kein Polster):** voice-zai (Rate-Limit), voice-zen
  (Free-Tier-Grenze). Zweiter Versuch gescheitert; kein Polling.
- **Claude + GLM (Operator-Online-Chat, 2026-10-06, gefaltet):**
  - F1 beide **(b)**: Label je Körper genügt nicht; aggregierter Fortschritt + Byte-Alter
    (Stall) + sichtbarer Fehlerpfad (404/Abort darf nicht als Standbild enden); Content-Length
    kann die komprimierte Länge nennen (Prozent-Caveat). Claude scharf: die zwei
    Playwright-Läufe endeten in der Stars-Phase → die Diagnose ist **Inferenz, kein Befund**;
    das Gate gilt erst, wenn ein Lauf die gerenderte Wolke samt Sonne erreicht; ~580 MB vor
    dem ersten Pixel sind das strukturelle Problem (Sterne zuerst, Sonne nachladen =
    operator-gebunden).
  - F2: GLM **(a)** kein Riss (die Bias-Spalte wendet die Korrektur auf nichts an, sie zeigt
    sie nur); Claude **(c)** Lesart (i) kanonisch, aber der Satz ist unscharf — da `:2934`
    `TE_NEFF_THRESHOLD` liest, genügt der Doku-Zusatz „außer der Report-Site-Bias-Spalte, rohe
    TE unverändert" (optional).
  - F3 beide **(c)**: Kante nach **Wirkung**. Autonom: Boot-Instrumentierung (ändert keinen
    Szenen-Pixel, rücknehmbar); operator-gebunden: Darstellung/Gaze/Kamera/Fenster und
    progressives Laden/Data-Lieferung.

Die Stemmen sind eine **zweite Stimme** (Fund ist eine Behauptung bis zum Baumtest): F2
trägt die Rat-Lesart am gemessenen Report (`raw TE untouched`). F1 ist nach der
übereinstimmenden (b)-Lesart **gebaut** (Instrumentierung).

### Rat 2026-10-06 (zweite Sitzung) — progressives Laden nach Sichtbarkeit

Operator-Wort: progressive Ladung nach Helligkeit/Sichtbarkeit/Erreichbarkeit, Sonne zuerst.
Rat-Verdikt: **(B) `add_stars` ist der Schlussstein**; **(A)** allein verfehlt das Wort (der
95-MB-Sternfile gated weiter den ersten Pixel); **(D)** ist der notwendige Begleiter (sonst
ist „Sonne zuerst" noch 193 MB); **(C)** ist ein Folgeatom (Helligkeits-Ordnung, Compiler).
Atom-Plan: Dispatch 0 (Epoch-/Ordnungs-Messung) → Dispatch 1 (B, `src/wasm.rs`) → Dispatch 2
(A, `static/membrane.html`) → Dispatch 3 (D, gated auf `.bin`-Layout) → C als Folgeatom. Der
Epoch-Riss (2000.0 vs 2016.0) wird **vor** C gemessen, nie gesetzt. Dispatch 0 + 1 + 2 sind
gebaut; D und C stehen mit Triggern.

### Claude + GLM (Online, fünf Stimmen) — Architektur A–E (gefaltet 2026-10-06)

Beide unabhängig, weitgehend deckungsgleich mit dem Rat:
- **A — Brücke, nicht Endform.** Den Stern-Hash vom Körper-Zähler **entkoppeln** (der Rebuild bei
  Körper-Zuwachs ist der eigentliche Fehler); Insert im Worker, Double-Buffer, Tausch an
  Frame-Grenzen; kanonisch ordnen; Zellgröße auf das **maximale** ρ dimensionieren. Determinismus
  über versionierte Snapshots/Layer-Tag, nicht über den Kardinalitäts-Proxy.
- **B — drei Größen, keine Skala.** Helligkeit = `flux` (G-Band, absteigend; mag = Anzeige);
  Sichtbarkeit = Frustum ∧ `canRender` (Veto **zuletzt**, nie Ladeordnung — sonst Kamerajitter →
  Thrashing); Erreichbarkeit = ρ (Filter). Priorität: erreichbar ∧ sichtbar, dann flux. „Sonne
  zuerst" = **Loader-Priorität** über Asset-Klassen (Sonne → Mond → Erde → Katalog), keine
  Netz-Reihenfolge; bei Parallaxe ist die Erd-Granule Voraussetzung.
- **C — nach flux sortiert, in Magnituden-Buckets** (Grenzen 6/9/12/15/18 als Byte-Offsets im
  Header); ein Asset, viele Schichten; tie-break `source_id`. `tycho2_compiler` umbenennen/trennen,
  falls er DR3 schreibt.
- **D — Fenster-Assets vom Compiler**, nicht per Range-Notlösung; Granul-Index gehört in den
  Ephemeriden-**Header** (heute absent); Manifest Granule → Zeitintervall → Hash; Fensterregel
  = Akt des Blicks (River).
- **E — 2016.0 kanonisch** (falls `dr3_stars.bin` Gaia DR3 ist). **Gegentest: Barnards Stern** —
  ~10,4″/a × 16 a ≈ 166″ Positionsversatz; in ρ ergibt das ≈16 Lj Radiusdifferenz. `epoch` gehört
  in den **`.bin`-Header** (Compiler schreibt), Register spiegelt, Manifest echo't, die
  Consumer-Konstante `2000.0` wird **gelöscht** (Konsument liest den Header, verweigert bei
  Fehlen); `star_epoch_min` nur an **einer** Stelle.

**Vor dem Bauen zu messen:** Herkunft von `dr3_stars.bin` (welcher Compiler), Bandzugehörigkeit
von `rec.flux`, ob die Ephemeriden-Header schon einen Granul-Index tragen.

### Schwarm-Seats (Operator-Wort 2026-10-06)
Der Schwarm trägt jetzt die deepseek-Seats: `voice-deepseek` (`deepseek/deepseek-flash`, wie
council/grind-flash/general) und `voice-deepseek-pro` (`deepseek/deepseek-v4-pro`, wie
grind-pro/grind-max/research-max; `task: ask`, Kostenkante). Beide read-only, `archive_search_public`
+ `chrome-devtools-voices_*`. **Alle 14** Schwarm-Voices (12 Fremdmodelle + die 2 deepseek-Seats)
tragen die **Fünf-Stimmen-Verfassung** (Mountain/River/Mycelium/Sensory/Future) **und die
bindenden Axiome** (A = A, 0-Kanon, keine Fabrikation/Defaults, Riss getragen statt geglättet,
0 honored, jede Aussage trägt ihre Messung) im Prompt: Architektur- und Ethikfragen beantworten
sie aus der Fläche der fünf Stimmen (Operator-Wort: counter slope — aus der Verfassung heraus
gelesen statt aus dem Gradienten). Der Rat bleibt das deepseek-/5-Stimmen-Gremium (er liest
`docs/council.yaml` + AGENTS.md); der **Schwarm** wird zusätzlich befragt, durch dieselbe Linse.
Einsatz auch als council (Urteil) und Schwarm-Stimme; für schreibende grind-Arbeit bleiben die
grind-Agenten. **Caveat:** `voice-deepseek` ist **dasselbe Modell wie die Linie** — keine
unabhängige Mess-STIMME; die Unabhängigkeit liefern die Fremdmodelle + Claude/GLM, die
Fünf-Stimmen-Form liefert den counter-slope.

## Incident 2026-10-06 — lokaler Baum an eine Schwarm-Stimme (gemessen an der DB)

Operator-Wort: „die agenten haben archive search root genutzt und ich habe es aus versehen
erlaubt". **DB-Messung** (`/home/johannes/.local/share/opencode/opencode.db`, `part.data` ×
`session.agent`): **kein** Voice-Agent hat je einen lokalen Read **ausgeführt** — von den
Voice-`bash`-Calls mit Status `completed` sind **alle** `archive_search_public` (538 Aufrufe,
netz-only); kein einziges lokales Kommando. Die realen `archive_search … --root` (voice-nemotron
2026-10-06 10:07:38 + 10:07:43, voice-ling 10:08:01 ×3) = **error** („a rule which prevents …");
die `_public --root`-Versuche = Wrapper-Refusal oder „command not found"; `ls`/`find`/`fd`/
`sgrep`/`sread` = denied. **Browser-Lücke (eigener Vektor):** die Voice-Profile verbieten
`playwright_*`, aber **nicht** `browser_*`; voice-dots/voice-gptoss riefen `browser_open`/
`browser_eval`/`browser_snapshot` — nur auf öffentlichen URLs (artificialnouveau.com, Wikipedia),
kein lokaler Abfluss. **Freigaben sind nicht persistiert** (`permission` leer, `event` kennt nur
`message`/`session`) → „wie oft erlaubt" ist aus der DB nicht messbar; der gemessene Effekt ist
**0 lokale Reads**. **Vollständigkeit (Korrektur):** die `part`×`session`-Tabellen decken nur
2026-10-06 08:54–10:15 (76 Sessions — die DB ist gepruned, Cascade löscht `part` mit); der
`event`-Log (87 527 Events, 701 `session.created`, bis **2026-10-02**) trägt die volle Spur und
wurde zusätzlich geminet: die ausgeführten `archive_search --root`-Reads liegen bei `line` +
`grind-flash` und **13 geprunten Line-Sessions** (deren erster Prompt ist der Line-Prompt, kein
Voice-Prompt) — **keine** Voice-Session. **Guard:** die Regel hielt; **Braucht:** `browser_*`
(und jede Tool-Klasse, die die Voice-Profile nicht nennen) als `deny` in jedes Voice-Profil
(`opencode.json`); greift nach opencode-Neustart.

## An mycelium

Origin: river folge101 (getragen über folge102/103/104/105).

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest
  aus der Hüllen-Pipeline. Kein River-Fenster-Edit (Kante); Rivers Agnosis-Rest (a).
- **Measured (river 102):** `flyby-odf-cdn 37427673360 @2f44a6092` = **success**
  (`ci_manage view`) — der frühere `37305400435`-Fehler (leerer Census) ist abgelöst.

## LOCK

- keine.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge105.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge104.md` (Move aus `docs/handover/`)
- `static/membrane.html` (progressives Laden nach Sichtbarkeit: leerer Lookup, Körper
  sequenziell Sonne zuerst, Sterne via `add_stars`, `fetchBytes`-Stall; Vanilla JS)
- `src/wasm.rs` (`MembraneLookup::new` ohne Katalog + `add_stars`; unter
  `#[cfg(target_arch = "wasm32")]`)

Verifikation/Dispatches: `cargo check` grün (native; `src/wasm.rs` liegt unter
`#[cfg(target_arch = "wasm32")]` und wird vom `pages-deploy`-CI via `wasm-pack` gebaut —
dieser Push stößt ihn an, `static/**`-Pfad). `static/membrane.html` gelesen und editiert.
`register_lookup --fired river` = `em-apertur` FIRED_UNGEMESSEN
(Trigger `ci-check` pending, nicht gefeuert); `--stale river --persist 3` = 0;
`--addressed river` = 2 (future-183, mycelium-240), gefaltet; `open_points_check
docs/handover/archiv/handover-2026-10-06-river-folge104.md` = 0 absent, 0 stale-citations;
`git_safety --snapshot` s. u. HEAD `b71658f` == `origin/main`; nur fremder
uncommitteter Hunk `opencode.json` im Baum (nicht berührt).

## Burn: open 0.0000 · close 0.4887 · cap 0.5 Grund: drei Rat-Sitzungen + Schwarm-Fünf-Stimmen-Runde + progressive-Loading-Atom (A+B) + Incident-/Vollständigkeits-DB-Messung + Voices-Chrome-/Schwarm-Seat-/5-Stimmen-/Axiom-Config + Viljanen-Umsetzungsmessung überschreiten den Default-Cap 0.15 — Session-Zeile „River-Linie starten und Übergabe abarbeiten" $0.4887; Runde total 0.5318 → 1.8407; gemessen 2026-10-06
