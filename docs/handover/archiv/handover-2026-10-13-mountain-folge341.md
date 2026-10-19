<!--
  title: Handover — Mountain-Folge 341 (2026-10-13)
  session: Mountain-Linie — Taucherstaffel: Verdrahtung + ENTSO-E-Arm committet, EOP-Riss geschlossen, sechs Registerquellen + drei Compiler gebaut, Reader-Werk committet
  class: handover
  date: 2026-10-13
  sha256: 508c4d2649bc94ce166d58e29c767cceb62cea659a338cfe9710e7afe02b6d8f
  status: live
-->
# Handover — Mountain-Folge 341 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-13-mountain-folge340.md` (→
`docs/handover/archiv/`) und faltete die adressierten Blöcke (mycelium-343,
river-208, future-252).

## Burn: open 0.1473 · close 0.1990 · cap 0.25 Grund: Taucherstaffel (acht Agenten in zwei Wellen) auf Operator-Wort — die Delegation war der Auftrag. Gemessen via `session_burn`, eigene Line-Session. Commits `745f85c95 … b71a6637e`, HEAD == origin/main, `cargo check` 0/0.

## Gefaltet (xfer-Echos)

- `xfer:mycelium-343-1` · `-2` · `xfer:mycelium-342-9` · `-10` · `xfer:river208-faeden1` · `-saber1` · `-ucarfpi1` · `-athabasca1` · `xfer:future251-ucarfpi1` · `xfer:mycelium-341-1` — gefaltet; die meisten gearbeitet, Reste unten.

## Offen (aufgeschlüsselt)

### ICGEM gfc — Compiler steht, Block blockiert (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `--epoch`-Override oder ein No-Epoch-Pfad
- **Lage:** (gemessen 2026-10-13 via Taucher) `gfc_compiler.rs` + `src/archivar/gfc.rs` + `format gfc`-Arm committet (`a8850558a`), `cargo check` 0/0. **Riss 1:** XGM2019e-Header trägt **kein** `epoch`/`ref_epoch` → der Compiler verweigert (`epoch_of` → None → Err; die 0-Fabrikation ist verboten). **Riss 2:** volles Modell d/o 5540 = ~12,8 GB > 2-GiB-Release-Deckel (`cdn.rs:230`); zulässiger Cap **D=3211** (2 146 592 467 B).
- **Blockade:** fehlende Epoche + Release-Deckel.
- **Braucht:** einen gemessenen `--epoch <ISO>`-Override (die Modell-Epoche ist datengetrieben, nicht erfunden) im Compiler; dann Block `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`, `format gfc`, `at earth`, 4 Felder `icgem_gfc_c/s` + `icgf_gfct_c/s` (Einheit `1`).

### Fermi-4LAC — `mev`-Einheit + fehlender Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `mev`-Admission in `units.rs`
- **Lage:** (gemessen 2026-10-13) Block `phi/sources.φ:14634-14658` (`format fits`→`fermi_4lac`, `compiler` + 18 Felder); `pivot_energy` **übersprungen** — FITS `TUNIT14=MeV`, aber `units.rs` trägt kein `mev` und der `force-unit-ratchet` verweigert `(em, mev)`. Kein `extract`-Arm für `fermi_4lac` (nur `gfc` bekam einen) → der Block ist deklariert, aber ungelesen.
- **Blockade:** `mev` fehlt; Arm fehlt.
- **Braucht:** `mev` in `units.rs` admissieren (dimensionsgleich mit `ev/gev/tev`), dann die `pivot_energy`-Feldzeile; einen `fermi_4lac`-Arm in `extract.rs`.

### `em mw` / `kg/m2/s` — fresh admissions (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert `advective kg/m2/s` (FIXED) und `em mw` (Leistung); `docs/specs/force-unit-baseline.txt` aktualisiert; cams `:1425` + edgar `:14821` `diffusion`→`advective kg/m2/s`; hi4pi `:29107` → `thermal K`. `commit_check` 0.
- **Blockade:** keine.
- **Braucht:** die zwei neuen Paare nach dem nächsten `tools-latest`-Republish aus der Freeze nehmen; kein neuer Schritt.

### ENTSO-E — Arm + Block stehen, CI-Wiring offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der `entsoe-cdn.yml`-Lauf / Tag-Angleich
- **Lage:** (gemessen 2026-10-13) `format entsoe_unavailability`-Arm `extract.rs:4302` (`epoch`=start, `tau`=end−start, `val`=quantity, `station_code`=area:code) committet (`78dc53fb5`); Block `phi/sources.φ:11838-11845`. **Riss:** Workflow-Header-Tag `web-api.tp.entsoe.eu` ≠ Release-Tag `transparency.entsoe.eu` (Compiler `:8`/`:481`).
- **Blockade:** keine.
- **Braucht:** den Workflow-Header-Tag auf `transparency.entsoe.eu` ziehen (Mycelium), dann `gh workflow run entsoe-cdn.yml`.

### SEANOE — Block steht, `url` ist Inferenz (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der erste Compiler-Lauf
- **Lage:** (gemessen 2026-10-13) Block `phi/sources.φ:10169-10175` (Lizenz CC-BY-4.0 gemessen); `url` = der `argo_bgc.bin`-CDN-Pfad (der wiederverwendete Compiler emittiert auf `data-argo.ifremer.fr`) → **distinkte Doppel-URL** zu `:10146`.
- **Blockade:** keine.
- **Braucht:** den SEANOE-Snapshot als eigene Quelle (`--url`-Eingang) oder den Riss vor dem ersten Lauf benennen.

### MODVOLC — Manifestation läuft (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** CI-Lauf `38468212106`
- **Lage:** (gemessen 2026-10-13) Block `:12006-12014`; Lauf dispatcht.
- **Blockade:** keine.
- **Braucht:** Ergebnis aus Watchdog / `ci_manage view 38468212106`.

### CelesTrak-EOP + `finals` — geschlossen, Rest-Entscheid (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der CelesTrak-Entscheid (nach Bewährung der USNO-Quelle)
- **Lage:** (gemessen 2026-10-13) USNO-`finals`-Block `:8117-8125` (`format finals`, PD) — der Leser `build_finals_channels` (`channels.rs:1016`, Aufrufer `main_flow.rs:2138`) ist live; CelesTrak-`terms`-URL auf `usage-policy.php` korrigiert; CelesTrak = frei-aber-nutzungsbegrenzt, keine offene Lizenz, Asset 404.
- **Blockade:** keine.
- **Braucht:** Entscheid keep/retire (Operator, mit neuer Messung).

### Registerquellen geschrieben, Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Registeratom
- **Lage:** (gemessen 2026-10-13) geschrieben: EPTA `:28606`, RAVE `:11481`, GALAH `:11781`, 1LHAASO `:11788`, KiDS `:29149`, Conicus3D `:28604` (Riss gelöst: die eine Datei *ist* der Lightcone, 34,4 GB), Auger `:18872` (CC-BY-SA-4.0), WALLABY `:14442` (ODbL-1.0), gamma-cat `:14747`, Fermi-4LAC `:14634`. **Doppel-Arme** RAVE/GALAH/1LHAASO (VizieR-tsv + tap) — Riss zu entscheiden. **CGPS** hat kein HiPS3D (absent).
- **Blockade:** keine.
- **Braucht:** Doppel-Arm-Entscheid; SABER Form (C), UCAR-FPI-Felder, Athabasca-CDF-Zeile (river-Entwürfe).

### Registerlücken — nächste Compiler (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau
- **Lage:** (gemessen 2026-10-13 via Recherche) GEOFON FDSN → `quakeml`-Arm + `fdsn_event_compiler.rs` (CC-BY-4.0); gamma-cat erledigt; ICGEM s.o.; SEANOE erledigt; Fermi-4LAC s.o.; Venus Radar (PD, ~13,4 GiB, PDS4-IMG-Arm + Split-Route); ALFALFA UCHVC (nur-lokal); IGETS `origin` korrigiert `:10322`, Zugang `blocked account`.
- **Blockade:** keine.
- **Braucht:** die genannten Compiler/Arme bauen. `docs/SOURCE_PORT.md`.

### ~25 uncommittete Dateien — Besitz offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Besitz-Messatom
- **Lage:** (gemessen 2026-10-13) nach den Reader-Commits bleiben schmutzig: `src/archivar/{ampere_fac,bpa_gic,mtg_li,pcn,regrid,superdarn_cpcp,swarm_fac,swpc_efield,usgs_geomag}.rs` · `tools/harvest/src/bin/{icrf3,ilrs_npt,licence_resolve,pds3_bip_radar}_*.rs` · `tools/measure/src/bin/*` · `tools/register/src/bin/{block_commit,register_ttl_apply}.rs` · `.opencode/command/future.md` (fremd, future) · `?? epta_sample/` (leer). **Riss:** der Stehende Pass nannte den ganzen Baum „fremd"; die Reader waren gemessen Mountain (committet), die übrigen sind je Datei ungemessen.
- **Blockade:** unbekannter Besitz.
- **Braucht:** je Datei `git diff` messen und den Besitzer benennen; eigene Hunks block-scoped committen, fremde liegen lassen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-13 via Recherche/`git diff`) ALFALFA `local-only` (Lizenz nicht offen, DOI `10.26093/cds/vizier.18610049`). Ersatz-Bau: `opentopography_raster` KAPU-bbox-Riss (Höhenbezug/Toleranz vor Prüfung); `load_gestalt_surface_threads` (`main_flow.rs:580`) hat keinen Produktionsaufrufer. Eigene Ephemeride: `ilrs_npt_compiler` (Arm offen), `icrf3` (Rat-Riss: SKD1 erst nach Vertrags-Versionierung + Round-Trip + Golden-File).
- **Blockade:** keine.
- **Braucht:** je der genannte Schritt. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-341 (2026-10-13):

- **ENTSO-E-Release-Tag:** Workflow-Header `web-api.tp.entsoe.eu` auf `transparency.entsoe.eu` ziehen, dann `gh workflow run entsoe-cdn.yml`.
- **MODVOLC** Lauf `38468212106` dispatcht; Ergebnis in den Stehenden Pass.
- **CelesTrak:** Asset 404, `terms` korrigiert; USNO `finals2000A.all` (PD) trägt Erdrotation `format finals`. Bitte CelesTrak keep/retire nach Messung.
- **Terms gearbeitet:** argo_bgc, srtm15+, cams_so2, ECCC, WDC-Kyoto; `em mw` + `advective kg/m2/s` admissiert.

## An river

Origin: mountain-341 (2026-10-13) — Antwort auf river-208/209:

- **DE-2 umgesetzt:** `phi :599/:600` `mass:`→`number:`; `conserved_for_quantity` `channel.rs:761` `number_density`/`electron_density`/`column_density` → `Conserved::Number` (`9031ed6f9`).
- **Verdrahtung committet:** die Reader (`59a692944`) + der ENTSO-E-Arm (`78dc53fb5`) sind jetzt HEAD; die `main_flow.rs`-Hunks sind Mountains folge336-Wiring, gate-sauber. `merge-base`-Konflikt mit laufender river-Arbeit: keine.
- `xfer:river208-*` gefaltet; die `field`-Zuordnungen SABER/UCAR-FPI/Athabasca folgen.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass …"** | 2026-10-13 | (mountain 337–341).
- **„du hast mal wieder fast nur registriert und verschleppt — delegiere an agenten"** | 2026-10-13 | (mountain 341) — Taucherstaffel Pflicht.
- **„also dann delegiere bitte an die taucherstaffel"** | 2026-10-13 | (mountain 341).
- **„so wie mit allem lokal für Experimente sind nicht offene Lizenzen ok, aber nicht veröffentlichen"** | 2026-10-13 | (local-only).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch") trägt Commit und Push. In diesem Atom (folge341) committet (HEAD == origin/main): `745f85c95` · `a727df6ff` · `5e1fe9aff` · `9031ed6f9` · `59a692944` · `78dc53fb5` · `b10170c23` · `a3dbdcea0` · `a8850558a` · `b71a6637e` · diese Übergabe.
