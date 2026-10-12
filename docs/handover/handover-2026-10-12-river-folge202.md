<!--
  title: Handover — River-Folge 202 (2026-10-12)
  session: River-Linie in einem Pass (river-202) — P7.1-Injektions-Ort gemessen (Slot (i) nach dem Fold, aber gated: per-Kanal-Dichte fehlt im 26-f64-Wire und im 12+16-f32-Pack; CouplingRegistry::term/exchange haben keinen Produktions-Caller, nur Tests; CPU-Referenz `exchange` steht); Newell-OMNI-Lage gegen den Baum korrigiert (Carrier registriert phi:694-710/:740; absent = align_many-Fenster-Disjunktion, nicht ungeladene Quelle — mountain-330); adressierte Blöcke gefaltet (mycelium THEMIS, sensory JUICE); Rat Dichte-Trage-Ort entschieden (ρ abgeleitet im Feld-Eval, `density_refs` = Paar live `Conserved::Mass`-Kanäle; CPU-Referenz-Resolver als erster Bau), UI 5/5 untermauert (rollenschlüssiges typisiertes `density_refs`, expliziter QN-Constraint, `K` statt `ν_in`), CPU-Referenz-Resolver gebaut (`DensityKind`/`DensityRef`/`K`/`quasi_neutral` + `exchange_resolved`, cargo check 0/0); folge201 konsumiert/archiviert
  class: handover
  date: 2026-10-12
  sha256: 165590e9d9fa568f91002b0f3fff6f8f28cd25223f4cae963273b7fc00f7dd28
  status: live
-->
# Handover — River-Folge 202 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

## Arbeitsweise (Operator-Wort 2026-10-12) — mehrere Agenten parallel

Der Operator: „schreibe in die übergabe dass ihr mehrere agenten losschickt ich hab echt
keine lust mehr auf eure verschlepperei und vergessen." Jede River-Session dispatcht **alle**
parallel arbeitbaren Punkte in **einem** Zug an Agenten (`grind-flash`/`general`/`vision`) —
nie nacheinander, nie „nächster Dispatch", nie „nächste Session". Verschleppung und Vergessen
sind der benannte Fehler; ein Punkt, den die Session selbst arbeiten kann, wird im selben Atom
gearbeitet (Sofort-Prinzip).

## Burn: open 0.00 · close 0.18 · cap 0.50 — Grund: river-202 (deepseek-flash, kein pro/max); 1 `grind-flash` (P7.1-Injektions-Ort-Messung), 2 `general` (Newell-OMNI-Carrier-Messung + Dichte-Quelle-Recherche), 1 `council` (Rat Dichte-Trage-Ort) + 1 UI-Runde (5 Seats) + CPU-Referenz-Resolver selbst gebaut. `session_burn` Schluss: River-Linien-Session ≈ $0.18; Haus-Total $3.2147. Fälligkeit: `--fired river` = 0, `--stale river --persist 3` = 0, `open_points_check` folge201 = 0 absent / 0 stale-citations / 0 format-gaps / 0 owner-drift (4 word-carried = Register-Zitate, keine Drift). 5 adressierte Tokens gefaltet (mycelium THEMIS 1 · sensory JUICE 4). Kein `smail --send`, kein Window-Edit. Die Messungen liegen in `state/research/2026-10-12-river-p71-injection-site.md` (gitignored) bzw. im Abschluss dieser Übergabe; `git_safety --snapshot` `refs/safety/1791836917`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serach alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen; (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; Audit „wo wurde der Observer-Bias wieder eingeschleust"
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172)
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172)
„bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 174)
„z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen" | 2026-10-10 | Operator (Session, River 174)
„mit archive search dem rat und max roster (alle und warten bedenke tryingopen1)" | 2026-10-11 | Operator (Session, River 181)
„ok dann ist es pending … dann warten wir bis wir zugelassen sind kannst du nochmal in state schauen wir hatten da eigentlich einige netzbetreiber" | 2026-10-11 | Operator (Session, River 188) — ENTSO-E/NERC bleiben `pending`; auf die Zulassung warten, bis dahin die Netzbetreiber aus `state` referenzieren (nicht re-suchen).
„mountain ist sehr beschäftigt und mycelium auch … kümmer dich bitte selbst drum" | 2026-10-11 | Operator (Session, River 191) — **River übernimmt den Vlies-Matrix-Alignment-Punkt selbst** (SWPC/RTSW-Archivreihe finden + registrieren + `matrix-vlies` neu), auch in Mountain- (`phi/sources.φ` url) und Mycelium-Domäne (Manifestation/Compiler).
„ja" | 2026-10-11 | Operator (Session, River 192) — (1) den SPAN-Feld-Edit bauen; (2) die vier verwaisten `cargo fmt`-Hünke (Mountain-316/Mycelium-312-Rückstand) als Haus-Dienst committen.
„ja bitte selbst mountain ist beschäftigt ich glaube wir müssen projektownerschaften einführen nicht aufgabenownerschaften das reibt so dermassen" | 2026-10-12 | Operator (Session, River 196) — (1) River setzt die Stations-Tabellen-Korrektur **selbst** (fremder Schreibakt auf Operator-Wort); (2) **Architekturfrage: Projekt-Ownerschaft statt Aufgaben-Ownerschaft** (Rat/Operator).
„aber die neun kraftmedien sind doch auch abgeschafft die hast doch sogar du selbst abgeschafft warum seid ihr gerade alle s kopflos" | 2026-10-12 | Operator (Session, River 197) — die 9 Kraft-Medien UND die flachen `force`-Tokens sind abgeschafft; die Kanal-Ontologie (`channel <conserved>:<role>:<op>:<pde_type>:<medium>:…) ist die einzige Form.
„warum legacy bridge hast du deine arbeit nicht zuende gemacht ich habe immer gesagt ich möchte keine abwärtskompatibilität" | 2026-10-12 | Operator (Session, River 197) — keine Abwärtskompatibilität; den P9.2-Bau zu Ende bringen.
„gab es da nicht schon rats gespräche und archive search all und ui seats sag mal was ist denn heute los" | 2026-10-12 | Operator (Session, River 197) — die P9.2/P10-Entscheidung ist durch Rat + `archive_search --all` + UI entschieden; **kein neuer Rat**.
„was heisst morgen glieich aber nicht in dieser session mach eine übergabe" | 2026-10-12 | Operator (Session, River 197) — P9.2 sofort als nächstes Atom, aber **nicht in dieser Session**.
„bitte setze eine taucher staffel an ich möchte dass du jetz fertig wirst Commit und Push stehen" | 2026-10-12 | Operator (Session, River 199) — Taucher-Staffel ansetzen, P9.2 zu Ende bringen; Commit + Push.
„ich habe es mountai geschickt" | 2026-10-12 | Operator (Session, River 199) — die Register-Reconciliation (`channel`-Direktiven, P9.1) liegt bei **Mountain**; River wartet auf die Faltung von `xfer:river199-1`.

Verbatim: `state/operator-gespraeche/2026-10-12-river.md`, `2026-10-11-river.md`,
`2026-10-10-river.md` und `2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`) + des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; die Intervall-Konstruktion benannt (studentisiertes Moving-Block-Bootstrap, delete-a-block-Jackknife-SE). Siegel = Operator-Wort.
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap umgestellt (`:171`).
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · `docs/paper/flyby-path-2-preregistration.md` (1) — **von sensory-255 gefaltet (JUICE/Flyby-Path-2-Trägerschaft, `An river`-Block 2026-10-11; registriertes Operator-Wort 2026-09-27 „bitte prüfe, was wirklich dir gehört"; `state/zustand/wartend.φ` Key `flyby-path2-recon` Aufnehmer river)**. Tokens (sensory-256, 2026-10-12 gefaltet): `xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.1~4844adaa` (paradigmenwechsel) · `.2~5243b5f0` (falsification-metric-addendum) · `.3~1f5f2109` (preregistration) · `.4~59f0ba87` (trägerlose Flyby-Orphans: `auftrag-flyby2-kette.md` + `flyby-path-2-addendum-2026-09-29.md`).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — die generalisierte Standort-Lokalisierungs-Landkarte (Orthofoto · DSM/DOM · Street-Level) + Methode `dsm − dtm`; Carrier dieser Survey.
- **Offene-Prosa-Träger (future-235/future-240-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`). `register_lookup --orphan-docs` = 0.
- **Mountain327-Abgabe (Marker abgearbeitet 2026-10-12 river-199):** `docs/blatt/blatt-gic-breitenband-familien.md` (4→2) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0). `xfer:mountain327-3` (echoed) · `xfer:mountain327-4` · `xfer:mountain327-5`.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### P7.1 Kopplung/Interface — Injektions-Ort gemessen: Slot (i) nach dem Fold, gated auf eine per-Kanal-Dichte
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** kein — die Ort-Messung (Q2) ist geleistet (2026-10-12 river-202); der Bau wartet auf die Dichte-Quelle.
- **Lage:** (gemessen 2026-10-12 river-201; `cargo check` 0/0, `rustfmt --edition 2024 --check src/mathematikerin/channel.rs` clean) **gebaut:** `ChannelRegistry::index_of(hash)->Option<usize>` + `CouplingRegistry::term(phi,channels)->Option<f32>` = Σ C_k·φ_from·φ_to über die Deskriptor-Hash→Index-Auflösung (None-Koeffizient/None-Hash übersprungen, kein Paar → `None`, nie 0.0) **und** der form-bewusste `CouplingRegistry::exchange` (friction-kind `IonNeutralFriction`: `from_delta=+rate·(φ_to−φ_from)`, `to_delta=−rate·(φ_to−φ_from)·(ρ_from/ρ_to)` → `ρ_from·from_delta+ρ_to·to_delta≡0` per Konstruktion; Produkt-kinds: `coeff·φ_i·φ_j`; Tests `a_friction_exchange_is_antisymmetric_through_density` · `an_absent_coupling_coefficient_yields_no_exchange` · `an_unresolved_exchange_hash_is_skipped`). `term`/`channel_reduce` unberührt. Davor (river-200): `Plasma` (`042eecc33`), `PdeRegime`+`PlasmaParameters` außerhalb `hash`/`PartialEq` (`64866d660`), `CouplingRegistry` (`8404624aa`), `InterfaceRegistry` (`599523d35`).
- **Ort-Messung (2026-10-12 river-202; `state/research/2026-10-12-river-p71-injection-site.md`):** der per-Record-**Wert** ist Wire-Slot 3 `val` (`actuators.rs:218`, WGSL `shaders.rs:177`); der per-Record-**Kanal** ist Wire-Slot 9 (`actuators.rs:221`, `shaders.rs:186`). Die per-Kanal-**Dichte** (ρ) ist **weder im 26-f64-Wire noch im 12+16-f32-Pack** — kein Slot. `field`/`props` sind `read` (`shaders.rs:19-20`); der Fold `presence_probe` (`shaders.rs:370-459`, Akkumulator `omegas[]` `:373`, `omegas[f] += c.x` `:400`) trägt den Wert, nicht die Dichte. **Beide Kanal-Werte** sind erst nach der Schleife live (`omegas[a]`/`omegas[b]`, `:373`/`:451`). `CouplingRegistry::term`/`exchange` haben **keinen Produktions-Caller** — nur Test-Literale (`channel.rs:3331/3384/3408`); `PlasmaParameters.n_e` (`channel.rs:361-365`) ist `None` und nie in ein ρ-Array gelesen. **Empfohlener Ort (i):** ein post-fold per-Zelle-Akkumulator in `presence_probe` direkt nach `omegas[]` (`shaders.rs:450`, vor dem `probe_out`-Write `:451`), eine Auswertung je ungeordnetem Paar (`+Δ`/`−Δ`); Site (ii) „separater Coupling-Pass" existiert nicht, (iii) = (i). Der Riss: die Zitat-Angabe „Fold `shaders.rs:199-233`" trägt nicht — jene Zeilen sind `osc_field`/`osc_flow`; der Fold ist `370-459` (Baum schlägt Zitat).
- **Rat (2026-10-12 river-201, Kette research > roster > rat; Blätter `state/stimmen/2026-10-12-river-p71-injection-grammar-roster.md` + `state/stimmen/2026-10-12-river-p71-injection-grammar-rat.md`):** **Slot (b)** — signed PDE-Quelle im Feld-Eval, **nicht** das lineare Aggregat `omega_sum` (`omega.rs:1835`), **nicht** das Atmungs-Target (`omega.rs:1853`; c nur read-only Diagnose-Tap). **Riss (gehalten):** der frühere Produkt-Term gilt nur für die **Reaktions**-Klasse; ion-neutral-Reibung ist **Differenz**-förmig (`ρ_i ν_in(u_n−u_i)`, equal-and-opposite; GITM/three-fluid generalized Ohm, DOI `10.1029/2012JA017637`/`10.1029/2021JA029421`) → **eigener `kind`**. Grammatik-Minimum: `kind · unordered pair {a,b} · rate ν_in|pending · frame · density_refs · region · conserves · status`. Roster konvergent (Qwen3.7-Plus · Claude Sonnet 5.5 Extra hoch · GLM-5.3 Max); Duck.ai `pending`.
- **Rat Dichte-Trage-Ort (2026-10-12 river-202, nach Recherche-Schicht `archive_search --all`; Blatt `state/stimmen/2026-10-12-river-p71-density-source-rat.md`, Digest `state/research/2026-10-12-river-p71-density-source-recherche.md`):** ρ trägt **keinen Wire-/GPU-Slot** — sie lebt **abgeleitet im Feld-Eval** (Form (i)), dieselbe Form wie das gewebte Vlies; `density_refs` = Paar zweier field-refs, jede löst auf den lebenden `Conserved::Mass`-Kanal auf, dessen φ die Dichte *ist* (`channel.rs:721`); per-Zelle `ρ_c(cell) = m_c·φ_{density,c}[cell]`, gebaut an dem Ort, der `rho[i]/rho[j]` `exchange` speist (`channel.rs:1604`). Rate/Freiparameter bleiben in `PlasmaParameters` (ii **nur** für Parameter). Erster Bau = **CPU-Referenz-Resolver** form-bewusst neben `term()`, kein WGSL-Edit.
- **UI-Untermauerung (2026-10-12 river-202, 5/5; Blatt `state/stimmen/2026-10-12-river-p71-density-source-ui.md`):** Duck.ai · Claude · Qwen · Z.ai/GLM Deep Think Max · DeepSeek V4 Pro konvergieren mit dem Rat und schärfen ihn. **Schwächste Stelle einstimmig:** das **ungeordnete, untypisierte Paar** der `density_refs` verliert die Rollen (Ion vs. Neutral) und die Speziesmassen-Asymmetrie (`m_i ≠ m_n`); Patch = **rollenschlüssig/geordnet Kanal→Ref**, **Quantity-Kind-Tag** (Anzahldichte vs. Massendichte — `ρ=m_c·φ` setzt Anzahldichte voraus), **`m_c` an den Kanal gebunden**, Wiring-Validierung. **Quasi-Neutralität explizit (5/5):** als **Regime-Constraint** deklariert (`n_e ≡ n_i`; Mehrspezies `Σ_s Z_s n_s = n_e`), **Enforcement per Ref-Alias**, **Diagnose per δn** — nicht Default, nicht Post-hoc-Projektion. **Zusatz-Riss (Claude):** deklariert wird **K (bzw. σ(T)), nicht ν_in** (`ν_in ∝ n_n`). **Riss (gehalten):** „Referenz vs. abgeleiteter Term" — die Referenz benennt den Kanal, der Wert ist der abgeleitete Term.
- **Baum (2026-10-12 river-202):** der **CPU-Referenz-Resolver** ist gebaut (`src/mathematikerin/channel.rs`): `DensityKind { Number, Mass }` + role-keyed `DensityRef { channel_ref, kind, species_mass }`; `Coupling` trägt `density_from`/`density_to`/`quasi_neutral`; `coefficient` ist jetzt der deklarierte **K** (ν_in wird abgeleitet); `CouplingRegistry::density_at` (ρ = m_c·φ für `Number`, φ für `Mass`, `None` bei fehlender Masse/unaufgelöstem Kanal/negativ) und `exchange_resolved` (single derived rate `ν_in = K·ρ_to/(m_from+m_to)`, `Σ_c ρ_c Δ_c ≡ 0` per Konstruktion). `term`/`exchange` unberührt. `cargo check` 0/0, `rustfmt --edition 2024 --check src/mathematikerin/channel.rs` clean. Tests im `#[cfg(test)]`-Block (`a_resolved_friction_exchange_is_antisymmetric_through_density` · `a_number_density_ref_without_a_species_mass_yields_no_resolved_exchange` · `a_mass_kind_density_ref_carries_the_density_untouched`) — laufen im CI (`cargo check` kompiliert `#[cfg(test)]` nicht lokal).
- **Blockade:** (1) die Grammatik des Parsers/Registers fehlt noch (role-keyed `density_refs` + `K` + `quasi_neutral` in `phi/sources.φ`/Parser — Mountain-Feder + River-Arm), damit der Resolver aus echten Direktiven gespeist wird; (2) die `quasi_neutral`-**Enforcement** (Ref-Alias, nie Post-hoc-Projektion) ist deklariert, aber noch nicht verdrahtet; (3) `K` wartet auf IRI-2020/NRLMSISE-00/IGRF (Mycelium, `pending`).
- **Braucht:** (1) Mountains `coupling`-Register-Direktiven + Rivers Parser-Arm (role-keyed `density_refs`, `K`, `quasi_neutral`); (2) die QN-Alias-Enforcement + δn-Diagnose; (3) WGSL-Parität + die Region-/Injektions-Ort-Messung (Slot (i) `shaders.rs:450`).


### P10 flache Tokens → Kanal-Deskriptor (`force_id_of`/`FIXED_CHANNELS`/Modul `force` weg) — gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeilen-Migration (`phi/sources.φ`: 7943 flache `field`-Zeilen → Deskriptor-Form, P10.3) — dann sterben der flache Parser-Branch (`parse.rs:1600`) und die `force_id_of`-Compiler, River zieht die flachen Tokens auf den Deskriptor.
- **Lage:** (gemessen 2026-10-12 river-199 am Baum) **Schritt 1 (Rename, layout-neutral) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut:** `FieldConfig.channel_ref: Option<u8>`, `Sample.channel_ref: Option<f64>`; `CHANNEL_REF_QUANTITY` und `wire_channel_ref` gelöscht; „kein Kanal" trägt allein `PRESENCE_FLAG_QUANTITY` (Bit 16, Wire-Pad über `wire_channel_ref(Option<f64>)`, Apertur über `ApertureSet::for_channel`); 57 Dateien (types/spatial/parse/fetch/extract/channels/main_flow + ~40 `declared_fields`-Compiler); `cargo check` 0/0. **Noch `force` im Baum:** die Brücke `descriptor_for_force`/`descriptor_for_channel_ref`/`channel_ref_of_descriptor` (`channel.rs:930-958`) + `ChannelRegistry::alias`/`resolve` + die zwei Legacy-Tests (`channel.rs:2004/2022`); `force.rs` (`force_id_of`/`force_name_of`/`FIXED_CHANNELS`-Re-Export, **76 `force_id_of(`-Stellen in 45 Dateien**; 6 davon in `tools/harvest/src/bin/*.rs`) + `FIXED_CHANNELS` (9 Tags, `channel.rs:873`); `gate/commit_gate.rs` + die `omegaflow::force`-Re-Exports. **Gemessener Riss:** `phi/sources.φ` trägt 9 `channel`-Direktiven (nach Hash-dedup **7 lebende**), die Registry registriert in Quell-Reihenfolge, der Parser vergibt den Handle über die feste 9er-Liste — ab dem zweiten Kanal fallen Parser-Index und Laufzeit-Registry-Index auseinander; der register-sourced Handle-Rebase ist river-199 `f2623bb72`.
- **Blockade:** `phi/sources.φ` trägt noch 7943 **flache** `field`-Zeilen, 0 Deskriptor-`field`-Zeilen (Mountains P10.3).
- **Braucht:** nach Mountains `field`-Migration den flachen Parser-Branch `parse.rs:1600` + die `force_id_of`-Aufrufe (Parser + 6 Compiler unter `tools/harvest/src/bin`) streichen und auf `descriptor_from_axes`/`ChannelRegistry::handle_of` ziehen; `FIXED_CHANNELS`/`force_id_of`/`force_name_of`/Modul `force` + `gate/commit_gate.rs`-Re-Exports auflösen (P10.4-Gate: kein `force`-Feld ohne `quantity`). Verifikation `cargo check` 0/0; CI-`ci-gate` ist das Testnetz.
- **Faltung `xfer:mountain329-3` (Sender mountain-folge330):** der Register-Nenner ist geräumt — `phi:27`→`seismic-body`-Kanal, `phi:164`→`electric`-Kanal (`7aa6fdd32`), **9 `channel`-Zeilen / 9 unique**; der Handle-Rebase auf die Register-Reihenfolge ist river-199 `f2623bb72`. Der register-sourced Handle ist baubar; er stirbt wie der flache Branch erst mit Mountains `field`-Zeilen-Migration (P10.3).

### Prosa-Träger mountain327-Abgabe — Marker abgearbeitet, Rest offen
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** kein — die Träger stehen.
- **Lage:** (gemessen 2026-10-12 river-199) die drei `--orphan-docs`-Dokumente sind gefaltet und ihre Marker abgearbeitet: `docs/blatt/blatt-gic-breitenband-familien.md` (4→2; offen: IGRF-14/AACGM-v2-Rust-Bin, Route-C-Stufe-2-Member-Pool) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2; offen: CTBTO-vDEC-Antrag, Rest) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0).
- **Blockade:** keine.
- **Braucht:** die zwei verbleibenden Marker je Blatt/Survey abarbeiten, sobald ihr Gegenstand gemessen ist (aacgm-Bin = Bau; CTBTO = Operator-Antrag).
- **Tokens (Sender mountain-folge330):** `xfer:mountain327-3` · `xfer:mountain327-4` · `xfer:mountain327-5` — gefaltet (Marker 4→2 / 4→2 / 2→0); die zwei verbleibenden Marker tragen weiter.

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur (Blatt/Antwortseite).
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) **2D (Tool `themis_asi_frame_onset`, Cube `fodg_20240510_kapu_onset.bin`):** kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45 (az 155–240, el 45–55). **Ganznacht-Overlay (Lauf `38259065347` success → CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`, 3 826 168 B sha `5a59429f…`, 7358 Frames 02:21:12–08:29:03Z, `mag_optical_overlay`):** Stundenmittel 03–06 ≈ 3310–3343 (ruhige Nacht), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; größter Optik-Wert 5623.5 @ 02:21:12Z (Rand-Kandidat, erster Frame); YKC-Spitze 52.37 @ 07:20. Der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s"-Lücke ist pick-abhängig.
- **Blockade:** keine (2D-, Tages- und Ganznacht-Messung stehen).
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit). **CDN-Duty gefaltet:** mycelium-333 meldet das Compiler-Asset registriert (`phi:21027`), Lauf `38258632187` success; Token `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` (2026-10-12 gefaltet).

### Weberin-Zeugen-Fäden — Inventar konsolidiert (kein GIC-Silo)
- **Status:** eigen (Admission offen) | **Bindung:** eigen
- **Trigger:** Operator-Wort zur Aufnahme; dann je Faden Admission (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-12 river-200; konsolidiert in `state/river/weberin-faden-inventar-2026-10-12.md`) die Zeugen-Fäden liegen über **vier** Träger: `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` §2 · `state/future/gic-riss-154-wunschliste-2026-10-06.md` §A–G (33 Fäden) · die Erweiterung `state/river/gic-wunschliste-erweiterung-2026-10-12/` (Riometer · Ionosonde/ISR · thermisch/neutral · BARREL/ELF-VLF). **Register-Stand 2026-10-12** (`sgrep -c <term> phi/sources.φ`): admitted GOES 44 · superdarn 22 · champ 14 · image 14 · carisma 8 · dmsp 7 · themis_gmag 4 · madrigal 5 · hfrnet 5 · ampere 5 · poes 5 · awesome 4 · vlf 5; **noch 0**: barrel · emfisis · riometer · ionosonde · eiscat · pfisr · kaira · norstar · cgsm · airglow · saber · pithia · jicamarca · fabry · interferometer. GIC ist **kein** Silo: dieselben Fäden tragen die allgemeine Thread-Matrix/Vlies (`docs/concepts/die-weberin.md:64`); kein GIC-eigenes Register, kein gic-eigenes Werkzeug.
- **Blockade:** keine.
- **Braucht:** je Faden die Quellen-Zeile/`channel`-Direktive (Mountain) + Arm/Manifestation (Mycelium), in der allgemeinen Channel-Etikettierung (Quantity | Mechanism | Medium, P10); GIC fährt sie als `field_te_query`-Störung, nicht als eigenes Netz.
- **Rat-Stand (river-200, Runden 1–3 + Bau):** `Plasma` ist die vierte Medium-Klasse `{Vacuum, Fluid, ElasticSolid, Plasma}`; Ionosphäre/Thermosphäre sind Region-Datums-Werte, nie Klasse; `domain` bleibt Geometrie. (a) Plasma-Regime = Parameterfeld innerhalb `Plasma` (keine Unterklassen); (b) `medium` = pro Pfadsegment (Deskriptor-Kette); (c) sparse Paar-Matrix in `CouplingRegistry`; (d) Grenzfläche = `InterfaceRegistry`; (e) `PdeRegime` + `PlasmaParameters`. Blätter: `state/stimmen/2026-10-12-river-medium-kinds-rat-roster.md` · `…-plasma-regime-granularity-…` · `…-p71-coupling-interface-regime-…`. Getragene Risse: Kopplungseinträge nicht unabhängig (`ρ_i·ν_in = ρ_n·ν_ni`); scharfes Interface vs. dünnes Segment; Regime-Abbildung nicht 1:1.

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert:** `mcpherron_onsets.bin`, 314 555 Onsets, `format mcon_event`, sha `441df5f8…`. **Register-Drift gelöst** (mycelium-318). `bpa_gic` führt nur ein **4-Tage/5-min-Fenster** (2026-10-03..06), **kein Archiv** → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
- **Blockade:** `bpa_gic` deckt den MCON-Zeitraum nicht (kein Archiv; nur FOIA).
- **Braucht:** (a) `bpa_gic`-Harvest auf 1999–2024 ausdehnen (Mycelium) — oder den MCON-Zeitraum verwerfen und die Relation auf die FMI-Reihe stützen; (b) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### wy-max-t — α als Family-Bound gemessen; Blatt-Siegel offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`.
- **Lage:** (gemessen 2026-10-12 via `gh run download 38166787414` + `wy_max_t_probe --combine`) der Westfall–Young-max-T-Null ist gemessen, α = 0.05, pooled B = 9999: (1-α)-Quantil **ABK-2024 = 2.4467 · ABK-2025 = 2.4433 · SOD-2024 = 2.4831** (Selftest 2.5856). family-clearing: `Bz→dB/dt` (3/3), `dB/dt→Speed` (3/3), `dB/dt→Density` (3/3), `dB/dt→Bz` (ABK-2024, SOD-2024), `Speed→dB/dt` (ABK-2025). α steht im Blatt (`:78-85`).
- **Blockade:** das Blatt-Siegel ist Operator-Wort.
- **Braucht:** Operator-Siegel; die gemessenen Werte im GitHub-Artefakt `38166787414` in den Bestand (Mycelium).

### Receiver-Apertur (SPAN) — Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste `ci-gate`-Lauf (Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`); danach Schritt 3.
- **Lage:** (gemessen 2026-10-12 via `cargo check` 0/0 + `git log` `5a3e629e7`) **Schritt 2 (Faltung)** gebaut und committet: `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel; `max(extent, scale)` gestrichen; Kontrakt `:34` nachgezogen.
- **Blockade:** das Paritäts-Gate ist ein Test-Target — `cargo check` kompiliert `#[cfg(test)]` nicht lokal; es läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log`), das Paritäts-Gate bestätigen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-12 via `ci_manage jobs`/`log 38164890620`) der Lauf trug 25 Stationen (`start 2000`, `end 2024`): 24 success, **`lov` cancelled** (LOV-Fetch HTTP 400 für 2023+); `38164966250` queued. Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt`; Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets der noch fehlenden Stationen (24 fertig; `lov` 400).
- **Braucht:** `ci_manage log 38164966250 --all` nach Abschluss; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`.

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde (GLM-5.3, qwen 3.8 Max, Kimi) einig: **kein** neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register); der Grammatik-Patch ist aus dem Verdikt noch nicht gebaut.
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Lage korrigiert: Carrier registriert, `absent` = `align_many`-Fenster-Disjunktion
- **Status:** eigen (offen) | **Bindung:** eigen
- **Trigger:** kein — der Lauf ist gelesen; die Lage ist gegen den Baum korrigiert.
- **Lage:** (gemessen 2026-10-12 river-198 `ci_manage log 38215224525 --all`; korrigiert 2026-10-12 river-202 gegen den Baum) der Resolver ist gebaut (`field_te_query.rs` `load_field_across_sources`). Die Zellen `newell_dphi_dt->intermagnet_dbdt` und `newell_dphi_dt_omni->intermagnet_dbdt` liefern `absent` / `summary … pending — no measured rung`. **Korrektur (Baum schlägt die frühere Prosa):** der Treiber `newell_*` ist **kein ungeladener Punkt** — er ist ein **derivierter Knoten** (`src/mathematikerin/newell.rs:1` `newell_dphi_dt(by,bz,speed)`, `descriptors/newell_geospheric{,_omni}.te` `drivers …`/`from …`), und **alle Carrier-Felder sind registriert**: `newell_dphi_dt_omni` ← `omni_imf_by_gsm_nt`/`omni_imf_bz_gsm_nt`/`omni_solarwind_flow_speed_kms` (`phi/sources.φ:740-751` live HAPI `OMNI2_H0_MRG1HR` 3600 s, **rollierendes** Fenster `{week_ago}…{now}`; `:1950-1963` archiviertes `omni2_serie_1h.bin`, CDN-Asset **404**), `newell_dphi_dt` ← `magnetosphere_imf_by_nt`/`..._bz_nt`/`solar_wind_speed_km_s` (`phi/sources.φ:694-710` `AC_H2_MFI`/`AC_H2_SWE`, 2010-01-01…2024-07-01). Die `absent`-Ursache ist **nicht** die Quelle, sondern `align_many` (`field_te_query.rs:4651-4681`, `Err("bin window carries no overlap between the arms")` `:4679-4681`): das live-OMNI-Fenster ist disjunkt vom archivierten ABK-Zeugen — deckungsgleich mit mountain-330 (`state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md:49-67`, Err `:62-64`). `sgrep -c newell phi/sources.φ` = 0 (kein Quellen-Block, korrekt — Name-ohne-Implementierung). Riss: `newell_geospheric.te:8-9` nennt eine „SWPC-rtsw-Quelle" — `sgrep rtsw phi/sources.φ` = nicht gefunden; der Kommentar ist veraltet, nicht das Register.
- **Blockade:** das CDN-Asset `omni2_serie_1h.bin` (`phi:1950`, registriert, format `omni2_serie`, netloc `cdaweb.gsfc.nasa.gov`) ist 404 — ohne es überlappt das live-OMNI-Fenster den ABK-Archiv-Zeugen nicht. Dazu Drift: `omni_imf_bz_gsm_nt` ist dreifach registriert (live `:740` · daily `:1935` · 1h `:1950`), daily-vs-1h-overlap-corr nur 0.6277 < `AGGREGATION_IDENTITY_CORR=0.99` (Mountain-Register).
- **Braucht:** (a) Mycelium manifestiert `omni2_serie_1h.bin` (`omni2-cdn.yml --decimate-min 60 --asset omni2_serie_1h.bin`) — Transport-Block `## An mycelium`; (b) Mountain löst die `omni_imf_bz_gsm_nt`-Dreifach-Registrierung; (c) danach den Lauf erneut fahren und die Zeugenwahl (`abk_1m` vs `abk_1h`) messen.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Blatt-Kalibrierung — Z-Null ABK gelesen (Z = 58 min); Signifikanz-Null offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Siegel) auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`; der Signifikanz-Null (Blocklänge) ist bestimmbar.
- **Lage:** (gemessen 2026-10-12 river-198 via `gh run download 38215420000`) **BCa benannt** (studentisiertes Moving-Block-Bootstrap; delete-a-block-Jackknife Künsch 1989 als SE). **ABK-2024 (1-min-Korn):** `Z = 58 min | r = −0.31053 | n = 523621` (Maximum bei Lag 58); Artefakt `bz-dbdt-delay-abk-2024.txt` (sha `32699e08…`-Zip). **SOD-2024:** `SOD minute |dB/dt| absent` — das CDN-Asset fehlt, keine Messung; Artefakt `bz-dbdt-delay-sod-2024.txt`. **Bz-Schwelle = pending** (braucht Station X + kalibriertes Alpha); **studentisierter Null = pending** — die Treiber-Autokorrelation dreht innerhalb 180 min nirgends ins Nichtpositive, die Blocklänge fehlt.
- **Blockade:** der Signifikanz-Null braucht eine Blocklänge (Autokorrelations-Nulldurchgang fehlt); SOD-Asset fehlt.
- **Braucht:** (a) Blocklängen-Regel für dieses Korn benennen (Rat) oder den Deskriptiv-Wert tragen; (b) SOD-`dB/dt`-Asset (Mycelium); (c) α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## An mycelium

Origin: river-202 (2026-10-12):

- `xfer:river.ses_ece32cc6effeDmjwy9OIeR9uvC.1~f03b7ea0` **`omni2_serie_1h.bin` manifestieren** — registriert in `phi/sources.φ:1950` (format `omni2_serie`, netloc `cdaweb.gsfc.nasa.gov`, 3600 s), CDN-Asset **404** (gemessen 2026-10-11, `docs/handover/archiv/handover-2026-10-11-river-folge180.md:88`). Ohne es ist das Fenster der live-OMNI-Carrier (`{week_ago}…{now}`, `phi:740`) disjunkt vom archivierten ABK-Zeugen; deshalb liefert die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` `absent` (`align_many` `Err`). Bitte `omni2-cdn.yml --decimate-min 60 --asset omni2_serie_1h.bin` fahren.
[manifest n=1 root=2bd9daea]

## Abschluss

river-202: gearbeitet ohne Pro-Solo, ohne Fenster-Edit, ohne Send. **P7.1-Injektions-Ort
(Q2) gemessen** (grind-flash, read-only; `state/research/2026-10-12-river-p71-injection-site.md`):
Wire-Slot 3 `val`/Slot 9 `channel_ref` tragen Wert und Kanal, aber die **per-Kanal-Dichte ρ fehlt
im 26-f64-Wire und im 12+16-f32-Pack** — kein Slot; `CouplingRegistry::term`/`exchange` sind
**test-only** (kein Produktions-Caller); einziger Ort, wo φ und ρ gleichzeitig live sind, ist das
gebaute CPU-`exchange` (`channel.rs:1587`). Empfohlener Ort (i): post-fold per-Zelle-Akkumulator
in `presence_probe` nach `omegas[]` (`shaders.rs:450`); Site (ii) existiert nicht. Der Bau ist
**gated auf eine Dichte-Quelle** (`PlasmaParameters.n_e`, `channel.rs:361`, `None`/ungefüttert).
**Der Dichte-Trage-Ort ist rat-entschieden** (2026-10-12 river-202, nach Recherche-Schicht):
ρ lebt **abgeleitet im Feld-Eval** (kein Wire-Slot), `density_refs` = kanonisches Paar zweier
live `Conserved::Mass`-Kanäle, deren φ die Dichte *ist*; erster Bau = **CPU-Referenz-Resolver**
(kein WGSL-Edit). Die **UI-Seats** (zweiter Kanal) haben die Runde **5/5 untermauert**
(`state/stimmen/2026-10-12-river-p71-density-source-ui.md`) und geschärft: das **ungeordnete,
untypisierte Paar** ist die schwächste Stelle → **rollenschlüssig/geordnet + Quantity-Kind-Tag +
`m_c` am Kanal**; **Quasi-Neutralität explizit** als Regime-Constraint (Alias-Enforcement, δn-Diagnose);
deklariert wird **K, nicht ν_in** (Claude). **Danach gebaut:** der **CPU-Referenz-Resolver** in
`src/mathematikerin/channel.rs` (`DensityKind`/role-keyed `DensityRef`/`Coupling.density_from|density_to|quasi_neutral`,
`coefficient` = K, `density_at` + `exchange_resolved` mit `Σ_c ρ_c Δ_c ≡ 0`) — `cargo check` 0/0,
`rustfmt --check` clean, Tests im CI-Pfad. **Newell-OMNI-Lage gegen den Baum korrigiert** (general): `newell_*` ist ein derivierter Knoten,
die Carrier sind registriert (`phi:694-710`, `:740`), die `absent` ist `align_many`-Fenster-Disjunktion
(deckungsgleich mountain-330) — die frühere „keine geladene Quellenreihe"-Prosa trägt nicht; die
Mycelium-Manifestation von `omni2_serie_1h.bin` als Transport-Block `## An mycelium` gesetzt. Die
**adressierten Blöcke sind gefaltet**: mycelium-335 THEMIS (`xfer:…~3ed6b902`) + sensory-256
JUICE/Flyby-Path-2 (4 Tokens `xfer:sensory…`) — alle im Träger-/CDN-Abschnitt ge-echot. `--fired river`
= 0, `--stale --persist 3` = 0, `open_points_check` folge201 = 0 absent / 0 stale-citations /
0 format-gaps / 0 owner-drift. Kein Pro/max-Dispatch, kein `smail --send`, kein Window-Edit;
`/consent` ist der session-weite Delegations-Consent, nie das Commit-Wort.
