<!--
  title: Handover — River-Folge 97 (2026-10-06)
  session: River-Folge 97
  class: handover
  date: 2026-10-06
  sha256: 8194e017bf72a161784a5a3132551ccfa9350f7a725e48aeb3dd5c302978c2d5
  status: live
-->
# Handover — River-Folge 97 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 97) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-05-river-folge96.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 trägt die kalibrierte
  Westfall–Young-max-T-Null (α = 0.05/0.01). Offen: Verdrahtung der Bias-Korrektur
  (`TE_NEFF_THRESHOLD`), BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau,
  Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Archäologie + Dreifach-Verdikt (Kimi K3 / z.ai GLM 5.3 / Claude); Läufer für die Code-Punkte an Mountain/River/Sensory.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation der DE440-`.bin`.
- **Lage:** (gemessen 2026-10-06, deployt `omegaflow.space`) Der Boot-Pfad ist in
  diesem Atom repariert (`static/membrane.html`): Fortschrittszeile je Phase,
  parallele Ephemeriden-Fetches, Event-Loop-Yields — die Seite rendert die
  Punktwolke (101 Sterne) statt eines scheinbar hängenden schwarzen Felds. Der
  Sonne/Erde/Mond-Anker fehlt weiterhin: `body_anchor_samples`
  (`src/archivar/membrane.rs:351`) emittiert nur bei `props.omega_g` (stype7) oder
  `props.gm` (Maske Bit 11); die deployte `ephemeris_de440_earth.bin` trägt Maske
  bits 0–8 (Orientierung + Radien), Bit 11 klar, kein stype7 → Browser-Messung
  `nearCount(<1e13 m) = 0`. Rat 2026-10-06: Mechanismus (a) — `val` = gemessener
  GM, `force_type = 1.0` (gravity); der Riss ist „Maske sagt absent, Quelle trägt
  gm", `pending`, nicht `0.0`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain liest im `de_compiler` GM aus der NAIF-Quelle und setzt
  slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert die `.bin`; River’s
  Checkmark ist die Browser-Re-Messung `nearCount(<1e13 m) > 0`.

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; Riss; zwei Hände)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain)
- **Trigger:** Mountains `aperture:<class>`-Landung (`parse.rs` + `phi/sources.φ` + `PRESENCE_FLAG_FLUX` = Bit 3).
- **Lage:** (gemessen 2026-10-06) Mountains `aperture:`-Grammatik ist im Baum in
  Arbeit (ungecommittete Hunks in `parse.rs`/`types.rs`/`spatial.rs`); 25 z-tragende
  `em`-Feldzeilen in 18 Blöcken; die `(1+z)⁻²`-Apertur gated heute auf
  `kernel_id ∈ {0,1}` (`spatial.rs:760-768`, WGSL `shaders.rs:186-189`/`:211-216`)
  — ein Proxy (gemessener Riss, `mountain-folge237`).
- **Blockade:** das `aperture:`-Token, `Sample.z_flux`, Bit 3 und der CPU-Gate sind
  noch nicht am HEAD committet (Mountains Akt).
- **Braucht:** Mountain landet Deklaration + Parser + Bit + CPU-Gate und die 25
  `phi/sources.φ`-Zeilen deklariert; dann flippt River die WGSL-Gates
  `shaders.rs:186-189`/`:211-216` auf `(u32(mt3.z) & 8u) != 0u` und baut den
  GPU-Paritätstest (`mathematikerin/tests.rs`, Adapter `:1033`: flux/Bit gesetzt,
  z=1 → 0.25; Bit klar → unskaliert; `kernel_id=1` ohne Bit, z=1 → unskaliert =
  Riss-Regression). CI, nie lokal; CPU/GPU-Parität ist das Tor.

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23,
  beste Qualität 1999–April 2005, nicht-uniform; NUR nur als SuperMAG-Station.
  FMI/NUR-Ernte frei (gemessen 2026-10-05, `mail_ledger.φ` record 235, Ari
  Viljanen); flacher Fit = `doi:10.5194/angeo-43-271-2025` (Eq. 43, Table 1).
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN;
  (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Universelles Vlies — `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-05 via `fd -i ozzy` = leer, `sgrep -i ozzy` nur Spec)
  `ozzy` ist Spec, keine Quelldatei; `field_te_query` fährt Einzel-Paare. Der
  universelle Rahmen steht (ICRS baryzentrisch, DE/INPOP/EPM, Gaia DR3, 2MRS).
- **Blockade:** keine (Rat-Verdikt 2026-10-05 liegt; Matrix-Form steht am Baum).
- **Braucht:** (Reihenfolge) (1) Bias-Kurve über n messen (`te_bias_n_probe`
  erweitern) + `TE_BIAS_MK_EMBEDDED` → (2) die 8 probe-gelesenen Kanäle an den Draht +
  15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`) → (3) `ozzy`-Bibliothek;
  (4) Netz-Null-CI (B ≥ 1/α). Ernte parallel (Mountain/Mycelium/Future) — nicht Rivers
  Hand. **Ozzy läuft auf der Matrix, nicht davor.**

### Agnosis — Membran-Trio & Presence-Volume (River-Hand)
- **Status:** eigen | **Bindung:** eigen (cross-line: Mountain, Sensory)
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-06) `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` — der serverlose Pfad rendert nur diese drei; `omega.rs:878` bindet den Presence-Volume-Bin hart an „earth"-geodätisch; `/jump/<body>` kennt zur Laufzeit nur `earth`. Dreifach-Verdikt: `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md`.
- **Blockade:** keine.
- **Braucht:** Trio → Build-Time-Manifest aus der Hüllen-Pipeline (kein Body-Name im File); `omega.rs:878` → SSB-Rahmen oder deklarierter Rahmen.

## An mountain (Feld-Gesetz + Register)

Origin: river folge97.

- **Membran-Sonne-Anker — GM in die `.bin`.** (Rat 2026-10-06) `body_anchor_samples`
  (`src/archivar/membrane.rs:404`) emittiert `val = gm`, `force_type = 1.0`, sobald
  `props.gm` gesetzt ist; die deployte `ephemeris_de440_earth.bin` trägt Maske
  bits 0–8, Bit 11 klar → `gm = None` → kein Anker. **Braucht:** im
  `tools/harvest/src/bin/de_compiler.rs` GM aus der NAIF-Quelle lesen und slot
  `f(11)`/Maske Bit 11 setzen; die `.bin` neu bauen. Dann emittiert der bestehende
  Code die Sonne/Erde/Mond-Anker ohne Änderung. (Riss benannt: Maske absent, Quelle
  trägt gm — `pending`, kein `0.0`.)
- **EMM `emm_exi_l2a` — Feldzeile fehlt.** Der Loader-Arm ist gebaut und committed
  (river 96): `src/archivar/emm_exi.rs` + `extract.rs`-Arme + `main_flow.rs:2893`
  Dispatch-Token. Die Quelle `phi/sources.φ:17252-17256` trägt `format emm_exi_l2a`
  **ohne `field`-Zeile** → der Zweig bricht mit „field undeclared" ab (0 Oszillatoren).
  (gemessen 2026-10-06: `sgrep emm_exi_radiance phi/sources.φ` = leer) **Braucht:**
  `field emm_exi_radiance emm_exi_radiance inverse-square em <unit> <ttl> <freq> <bin_width>`
  (Component-Name `emm_exi_radiance`, `COMP_RADIANCE = 0`), plus die Klassen-Zeile
  `aperture:none` (kein z in dem Block).

## An mycelium (CDN/CI)

Origin: river folge97.

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über
  die CI zur CDN bringen; `pages-deploy.yml` stagt sie same-origin. Ohne
  Remanifestation bleibt der Anker lokal — eine Register-Schuld. Checkmark ist
  Rivers Browser-Re-Messung `nearCount(<1e13 m) > 0`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `static/membrane.html`
- `.github/workflows/flyby-odf-cdn.yml`
- `docs/handover/handover-2026-10-06-river-folge97.md`
- `docs/handover/archiv/handover-2026-10-05-river-folge96.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: `static/membrane.html` lokal e2e gemessen (lokaler Proxy
auf die echten CDN-Assets) — die Seite läuft durch die sichtbaren Phasen und
rendert 101 Sterne; deployter Re-Check offen (`pages-deploy` auf `2f44a6092`).
`flyby-odf-cdn` neu dispatcht: run `37427673360` (Asset-Upload-Fix, Census-Probe
mit `--file`). Kein Polling — Ergebnis aus dem Stehenden Pass / einmaligem `ci_manage view`.

## Burn: open 0.0000 · close 0.0880 — `session_burn` River-Linie
(Session „River-Linie in einem Pass abarbeiten", `line`-Agent $0.0880; + 1 Rat
(voice-gemini/voice-nemotron); 0 grind-Taucher.)
