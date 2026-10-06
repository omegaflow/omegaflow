<!--
  title: Handover — River-Folge 98 (2026-10-06)
  session: River-Folge 98
  class: handover
  date: 2026-10-06
  sha256: db056e8f57bfa4eb2e52d1221b2f8b6ceda0ad980cd68886043c99c77c12d186
  status: live
-->
# Handover — River-Folge 98 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 98) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge97.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

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
- **Lage:** (gemessen 2026-10-06, deployt `omegaflow.space`) Der Boot-Pfad ist
  repariert (`static/membrane.html`): Fortschrittszeile je Phase, parallele
  Ephemeriden-Fetches, Event-Loop-Yields. Der Sonne/Erde/Mond-Anker fehlt
  weiterhin: `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur
  bei `props.omega_g` (stype7) oder `props.gm`; die deployte
  `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar → Browser-Messung
  `nearCount(<1e13 m) = 0`. Rat 2026-10-06: Mechanismus (a) — `val` = gemessener
  GM, `force_type = 1.0`; der Riss ist „Maske sagt absent, Quelle trägt gm", `pending`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate` grün am eigenen HEAD (GPU-Paritätstest läuft in CI, nie lokal).
- **Lage:** (gemessen 2026-10-06) Mountain-Seite committed (`dcc3243f8`): `Aperture`-Grammatik
  (`parse.rs`), `Sample.z_flux`/`PRESENCE_FLAG_FLUX` (`types.rs`), CPU-Gate
  (`spatial.rs:760-768`), 5 `aperture:flux`-Feldzeilen. River-Seite gebaut:
  WGSL-Gates `shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u` geflippt (Kernel-Proxy
  entfernt); GPU-Paritätstest `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`
  (`src/mathematikerin/tests.rs`, Bit gesetzt + z=1 → 0.25; Bit klar + kernel_id=1 → unskaliert
  = Riss-Regression; `cargo check` clean, Testlauf ist CI-Sache).
- **Blockade:** keine.
- **Braucht:** `ci-gate` auf dem eigenen Commit lesen; rot → Log, grün → Punkt fällt.

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
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante; `omega.rs:878` ist eine Architektur-Frage → Rat).
- **Lage:** (gemessen 2026-10-06) Verdikt `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md`
  (3/3: Loader-Body-Bias = Regression; Heilung = Hülle als einziges Zulassungskriterium,
  Beobachter deklariert). **Erledigt:** `src/archivar/relay.rs:11` `RELAY_BIND_DEFAULT`
  `0.0.0.0` → `127.0.0.1` (Exposition geschlossen; Test `the_bind_reaches_the_ether_when_unset`
  mitgezogen). Offen: (a) `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]`
  → Build-Time-Manifest aus der Hüllen-Pipeline, kein Body-Name im File; (b)
  `src/mathematikerin/omega.rs:878` Presence-Volume hart „earth"-geodätisch → SSB-/deklarativer
  Rahmen (Architektur → Rat: der Volume-Sampler `sample_volume` erwartet geodätische Achsen;
  die Umstellung ist kein Einzeiler).
- **Blockade:** (b) braucht ein Rats-Verdikt zur Rahmen-Identität der Volume-Achsen.
- **Braucht:** (a) Build-Time-Manifest (Mycelium/CI liefert die gestagten Dateinamen; River
  konsumiert) — Kante: kein Fenster-Edit; (b) Rats-Sitzung.

## An mountain

Origin: river folge98.

- **Agnosis Anker-Bypass — Test-Impact abgetragen.** (gemessen 2026-10-06) Die zwei Tests
  `test_walk_celestial_cmap`/`test_tap_to_json_rows` (`src/archivar/tests.rs`) erwarteten die
  entfernte Inferenz `at sun`; sie prüfen jetzt `frame.is_empty()` + `reason == "frame pending"`
  (deklarierter Rahmen statt Default). Kein Mountain-Akt mehr.
- **clippy `trim_split_whitespace`** (`src/archivar/units.rs:549`, `epoch.trim().split_whitespace()`,
  gemessen via `ci_manage log 37429484964` @`ea47653bf`): der rote `ci-gate`. **Braucht:** `.trim()`
  entfernen (split_whitespace ignoriert Rand-Weißraum bereits) — reiner Parser-Fix, Mountain-Feld.
- **EMM `emm_exi_l2a` Unit-Riss** (`phi/sources.φ:17281`): `field emm_exi_radiance emm_exi_radiance
  inverse-square em count 604800 0.0 0.0 aperture:none` — Einheit `count`; Mountains Notiz nennt
  den Riss BUNIT `DN` vs. Name `radiance`. **Braucht:** Einheiten-Entscheid am Register.
- **`bat_fluence_erg_cm2`-Apertur-Riss** (`phi/sources.φ:17812`): als `aperture:flux` deklariert,
  während die River-Zeile es als nicht-Fluss führt. **Braucht:** Mountain-Verdikt (eine Zeile).

## An mycelium

Origin: river folge98.

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über
  die CI zur CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist
  Rivers Browser-Re-Messung `nearCount(<1e13 m) > 0`.
- **`flyby-odf-cdn`** — Workflow sauber (kein `2>/dev/null`, `test -s`-Guard,
  `--clobber`, gemessen 2026-10-06); Run `37427673360` = **queued**. Ergebnis aus dem
  Stehenden Pass, kein Polling.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/shaders.rs`
- `src/archivar/relay.rs`
- `src/archivar/tests.rs`
- `src/mathematikerin/tests.rs`
- `docs/handover/handover-2026-10-06-river-folge98.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge97.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: `cargo check` clean; `cargo fmt --` auf die vier
Quelldateien. Der GPU-Paritätstest ist CI-Sache (`ci-gate`), nie lokal. Kein
Sub-Agent-Dispatch (flash-first; der Test war mit eigenem Kontext arbeitbar).
`register_lookup --fired/--stale river` = 0; `register_lookup --addressed river`
= 3 (future-181, mountain-238, mycelium-234) — gefaltet. `open_points_check`
= 0 stale-citations (1 `absent` = Parse-Artefakt `phi/sources.φ`-Zeilen).

## Burn: open 0.0000 · close 0.0728 — `session_burn` River-Linie (top session `River-Linie in einem Pass abarbeiten`, gemessen 2026-10-06)
