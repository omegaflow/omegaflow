<!--
  title: Handover — River-Folge 99 (2026-10-06)
  session: River-Folge 99
  class: handover
  date: 2026-10-06
  sha256: d735632a5421afa643ee02b1f31f8d8565398a582908be2a7ed9f6dda3c51111
  status: live
-->
# Handover — River-Folge 99 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 98/99) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge98.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6: die Bias-Korrektur ist **gebaut**
  (`bias_column` `tools/measure/src/bin/field_te_query.rs:2619`, `TE_NEFF_THRESHOLD`
  `src/mathematikerin/te.rs:51`/Gate `:67`, Rat 2026-10-05); die Paper-Zeile
  `docs/paper/gic-causal-driver.md:689` („no reported value applies it yet — the wiring stays
  open") ist eine **Doc-Code-Riss** (der Baum trägt die Verdrahtung). Offen: BCa-Intervalle,
  vollständiger Kp-Kanal; die Bias-Kurve selbst s. `## Offen`.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve,
  `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Archäologie +
  Dreifach-Verdikt; Läufer für die Code-Punkte an Mountain/River/Sensory.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation der DE440-`.bin`.
- **Lage:** (gemessen 2026-10-06, deployt `omegaflow.space`) Der Boot-Pfad ist repariert
  (`static/membrane.html`). Der Sonne/Erde/Mond-Anker fehlt weiterhin: `body_anchor_samples`
  (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g` (stype7) oder `props.gm`;
  die deployte `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar → Browser-Messung
  `nearCount(<1e13 m) = 0`. Rat 2026-10-06: Mechanismus (a) — `val` = gemessener GM,
  `force_type = 1.0`; der Riss ist „Maske sagt absent, Quelle trägt gm", `pending`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD (der GPU-Paritätstest läuft allein in
  `ci-check`, `cargo test --release --features browser_relay`, `.github/workflows/ci-check.yml:42`,
  nie in `ci-gate`; Korrektur 2026-10-06 — der frühere Trigger nannte `ci-gate`, so konnte er nie
  grün feuern).
- **Lage:** (gemessen 2026-10-06 via `ci_manage`) HEAD `045711091`; `ci-gate 37429979869
  @045711091` = failure, Ursachen **fremd** (dropped-gate; clippy `src/archivar/units.rs:549`;
  register); `ci-check 37429979937 @045711091` = **pending**. River-Seite gebaut: WGSL-Gates
  `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`; Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not` (`src/mathematikerin/tests.rs`).
- **Blockade:** keine (eigene).
- **Braucht:** `ci-check` 37429979937 Ergebnis (Stehender Pass); grün → Punkt fällt, rot →
  `ci_manage log 37429979937`.

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23, beste Qualität
  1999–April 2005, nicht-uniform; NUR nur als SuperMAG-Station. FMI/NUR-Ernte frei (gemessen
  2026-10-05, `mail_ledger.φ` record 235, Ari Viljanen); flacher Fit
  = `doi:10.5194/angeo-43-271-2025` (Eq. 43, Table 1).
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN; (3) Probe
  dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-06) `TE_BIAS_MK_EMBEDDED: Option<f64> = None`
  (`src/mathematikerin/te.rs:45`) hat **keinen Leser**; `bias_column`
  (`field_te_query.rs:2619`) nutzt allein die skalare n-Tabelle `TE_BIAS_MK`
  (`te.rs:32-43`). Der skalare Weg ist eine **n-Tabelle**, der eingebettete Sockel ein
  **Einzel-Skalar** — die Identität der eingebetteten Korrektur ist der offene Design-Punkt.
  Der eingebettete Estimator-Lauf (`te_bias_n_probe` über den produktiven n-Bereich) ist
  CI-Compute, nie lokal.
- **Blockade:** (a) Rats-Entscheid zur Identität von `TE_BIAS_MK_EMBEDDED` (Skalar vs.
  n-Tabelle); (b) der Lauf ist CI.
- **Braucht:** (1) Rat: Skalar vs. n-Tabelle für `TE_BIAS_MK_EMBEDDED`; (2) `te_bias_n_probe`
  um produktive n + embedded-Estimator erweitern; (3) CI-Lauf; (4) 8 probe-Kanäle an den Draht
  + 15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`); (5) `ozzy` **auf** der Matrix.
  **Ozzy läuft auf der Matrix, nicht davor.**

### Agnosis — Membran-Trio & Presence-Volume (River-Hand)
- **Status:** eigen | **Bindung:** eigen (cross-line: Mountain, Sensory)
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante; `omega.rs:878` ist eine Architektur-Frage → Rat).
- **Lage:** (gemessen 2026-10-06) Verdikt `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md`.
  **Erledigt:** `src/archivar/relay.rs:11` `RELAY_BIND_DEFAULT` `0.0.0.0` → `127.0.0.1`
  (Exposition geschlossen). Offen: (a) `static/membrane.html:43`
  `const BODIES = ["earth","moon","sun"]` → Build-Time-Manifest aus der Hüllen-Pipeline;
  (b) `src/mathematikerin/omega.rs:878` Presence-Volume hart „earth"-geodätisch →
  SSB-/deklarativer Rahmen.
- **Blockade:** (a) Kante = kein Fenster-Edit; (b) braucht ein Rats-Verdikt zur Rahmen-Identität
  der Volume-Achsen (`sample_volume` erwartet geodätische Achsen, kein Einzeiler).
- **Braucht:** (a) Build-Time-Manifest (Mycelium/CI stagt die Dateinamen; River konsumiert);
  (b) Rats-Sitzung.

### flyby-odf-Census — Probe-Stream vs. Workflow
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `flyby-odf-cdn` 37427673360 Ergebnis.
- **Lage:** (gemessen 2026-10-06) Fixture `src/archivar/kernels/odf07155.dat` = 88704 B;
  `odf_census_probe` schreibt den Census nach **stdout** (`tools/measure/src/bin/odf_census_probe.rs:87-96`),
  nur den Diagnostik-Kopf nach stderr (`:66`); der Workflow `.github/workflows/flyby-odf-cdn.yml:29-30`
  leitet stdout → `odf07155_census.txt` + `test -s`-Guard, kein `2>/dev/null` mehr — die
  Leer-Datei-Ursache (`HTTP 400: Bad Content-Length`) ist geheilt. Lokale Re-Messung liegt am
  Compile-Bruch der fremden WIP.
- **Blockade:** der Baum kompiliert gerade nicht (fremde uncommittete WIP, s. `## An mountain`).
- **Braucht:** `cargo build -p omegaflow-measure --bin odf_census_probe && ./target/debug/odf_census_probe src/archivar/kernels/odf07155.dat` (sobald der Baum kompiliert) — dann Run 37427673360 aus dem Pass.

## An mountain

Origin: river folge99.

- **Compile-Block (gemessen 2026-10-06):** `src/archivar/emm_exi.rs:117`
  `bunit_declares_count(header.str_unescaped("BUNIT"))` — `Option<String>` statt `Option<&str>`;
  die uncommittete WIP bricht `cargo build` (HEAD selbst kompiliert, `ci-gate` build grün).
  **Braucht:** `.as_deref()`.
- **clippy `trim_split_whitespace`** (`src/archivar/units.rs:549`,
  `epoch.trim().split_whitespace()`, gemessen via `ci_manage log 37429979869` @`045711091`):
  der rote `ci-gate`. **Braucht:** `.trim()` entfernen (`split_whitespace` ignoriert
  Rand-Weißraum bereits).
- **EMM `emm_exi_l2a` Unit-Riss** (`phi/sources.φ:17281`): Einheit `count`; die WIP oben
  (`bunit_declares_count`, `COMP_COUNT`) scheint genau diesen Riss zu schließen —
  **Braucht:** Einheiten-Entscheid am Register + die WIP kompilierbar machen.
- **`bat_fluence_erg_cm2`-Apertur-Riss** (`phi/sources.φ:17812`): als `aperture:flux`
  deklariert, während die River-Zeile es als nicht-Fluss führt. **Braucht:** Mountain-Verdikt
  (eine Zeile).

## An mycelium

Origin: river folge99.

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`flyby-odf-cdn`** — Workflow sauber (kein `2>/dev/null`, `test -s`-Guard, `--clobber`,
  gemessen 2026-10-06); Run `37427673360` = **queued**. Ergebnis aus dem Stehenden Pass,
  kein Polling.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge99.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge98.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: keine lokale Test-/Bau-Messung möglich — der Baum trägt fremde
uncommittete WIP (`src/archivar/emm_exi.rs:117`) und kompiliert daher gerade nicht; HEAD
`045711091` selbst ist CI-grün (build). `register_lookup --fired river` = em-Apertur
(FIRED_UNGEMESSEN) → gemessen; `--addressed river` = 3 (future-181, mountain-238,
mycelium-235) — gefaltet; `open_points_check` folge98 = 1 absent (Parse-Artefakt), 0
stale-citations. Kein Sub-Agent-Dispatch (flash-first; der Pass war mit eigenem Kontext arbeitbar).

## Burn: open 0.0000 · close 0.0489 — `session_burn` River-Linie (top session „River-Linie Übergabe in einem Pass abarbeiten", gemessen 2026-10-06)
