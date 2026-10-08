<!--
  title: Handover — Mountain-Folge 272 (2026-10-08)
  session: Mountain-Folge 272
  class: handover
  date: 2026-10-08
  sha256: b5a72993e37a26b0fef7bb2470b54577077d2dcf5ecf72bb5557b361959b522e
  status: live
-->
# Handover — Mountain-Folge 272 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-07-mountain-folge271.md` (→ `archiv/`). Kein
pro/max. Gefaltet: die adressierten Blöcke `## An mountain` (river-133, future-199).

**272-Arbeit (dieses Atom; Operator-Wort „bitte kümmer dich drum"):** die fünf
genannten Punkte durch Recherche-Schicht + Rat + UI-Runde gefahren.
- **3 GIC-Parser-Arme gebaut** (`cargo check -p omegaflow-harvest --bin <name>` je 0/0):
  `tools/harvest/src/bin/superdarn_cpcp_compiler.rs` (DMap `.map2`, Zenodo 10374021,
  `pot.drop` kV) · `tools/harvest/src/bin/emtf_compiler.rs` (EMTF-XML, USArray.CAO01.2010,
  Z in Ω) · `tools/harvest/src/bin/ssusi_compiler.rs` (HDF5, CDAWeb dmspf16, GW); plus
  `.github/workflows/superdarn-cpcp-cdn.yml` · `emtf-cdn.yml` · `ssusi-cdn.yml`.
- **blocked_sources-Abgleich (16 Einträge, `docs`-Kette):** gebaut (A) = `:167 themis_mag`
  (`harvest.φ:513`, `MAGIC THGM`), `:175 THA_L2_FGM`, `:179 MMS1_FGM`, `:183 poes19_meped`,
  `:199 soho_lasco_cme`, `:203 wdc_ae`, `:219 vlf_awesome`; offen (B) = `:159` Kellerman/
  Zenodo 4444068, `:163` DLR ROTI, `:171` Wind SWE, `:191` OMTI/Abisko, `:195`
  Substorm-Onsets, `:215` USGS E-Feld — SSUSI/CPCP/EMTF durch 272 gebaut.
- **river-133 gegenstandslos (gemessen):** `cgm_source bgs-quasi-dipole` + `cgm_lat`
  CPL/TTB stehen (`phi/sources.φ:6328-6329`, `:7673-7674`); `kepler.rs`-PROD-POISON ist
  `descoped` (`state/future/giftkarte-klassifiziert-src-2026-10-07.md:133`).
- **Rat (5 Stimmen) + Recherche-Schicht (`archive_search`) gefahren** — Verdikte je Punkt
  unten. **UI-Runde:** die konsolidierte 5-Fragen-Frage an 9 Seats (`mountain-ui`) gestellt.
  **Antworten auf die exakte Frage** (gemessen aus den Tabs, 11 Seats): Duck.ai (GPT-6 Luna),
  Gemini 3.1 Pro (AI Studio), Qwen, Claude (Sonnet 5.5 Extra hoch), DeepSeek-Chat, Mistral,
  MiniMax M3, Lumo 2.0 Max, Z.ai (GLM-5.3), plus Open-Weight **Inkling** und **DeepSeek V4 Pro**
  (tryingopen). **Offen:** Open-Weight **GLM 5.3** (753B) — „Thinking…" nach >2 min, kein
  Endverdikt = `pending` (Lock gesetzt/entfernt, Gruppe JIT geschlossen). **Konvergenz (11/11):**
  Q1 Bootstrap-Cache-Load = Verletzung; Receiver = zwingende per-Record-Weltlinie
  (`Receiver(WorldlineRef)` / `Receiver<RecordId,Worldline>` / `Future::Worldline<Continuity>`);
  `admit` → `Refusal` ohne Receiver. Q2 Apertur = Record-`extent`, `span`-Direktiv **streichen**
  (alle). Q3 nmgy nur mit Filter-/pivot-λ-Referenz als `em` (Inkling: ~477 nm SDSS g; Z.ai:
  `band_id` aus dem Band-Register, keine Code-Konstante). **Riss (nicht geglättet, mehrheitlich):**
  Q4 — `pending`-mit-Trigger: Qwen, DeepSeek-Chat, Lumo, DeepSeek-V4-Pro; **`measured descoped`:
  Claude, Duck, Inkling, Mistral, MiniMax, Z.ai (6:4)**. Q5 — **nicht wirebar / Gegenlinie steht:
  DeepSeek-Chat, Duck, Inkling, Mistral, MiniMax, Lumo, DeepSeek-V4-Pro (7)**; wirebar-mit-
  Coverage: Qwen, Claude, Z.ai (3). **Gewichtung (Roster-Tier = Frontier + ≥750B open-weight):**
  Q4 `descoped` 4:1 (Claude, Duck/GPT-6, Inkling 975B, GLM-5.3 753B gegen DeepSeek V4 Pro 1.7T);
  Q5 nicht-wirebar 3:2 (Duck/GPT-6, Inkling, DeepSeek V4 Pro gegen Claude, GLM-5.3) — die
  ungewichtete Zahl **unterschätzt** den Riss. **Der Rat hält Q4 `pending` und Q5 Member-Pool-Träger
  (Route C) → gewichtete UI-Mehrheit und Rat divergieren auf beiden Fragen = der eigentliche Riss.**
  Vorbehalt: nicht jeder Seat lief die stärkste Variante (Qwen-Tab = `Qwen3.7-Plus`, nicht 3.8-Max;
  Z.ai erst `GLM-5.3-Flash`, dann GLM-5.3). **Gemessene Nicht-Antwort:** Z.ai/GLM-Frontier war am Peak
  „Model currently at capacity" (2×), antwortete später als GLM-5.3. Max-Thinking/Deep-Search je
  Seat, wo ein Toggle gemessen wurde.

  **Max-Settings-Runde (2026-10-08, erneut, je stärkste Variante + Max):** Claude **Sonnet 5.5
  Maximal** (neu verfügbar), Gemini 3.1 Pro **Thinking High**, Duck **GPT-6 Luna + Begründung**,
  Qwen **3.8-Max**, DeepSeek **DeepThink + Search**, Z.ai **GLM-5.3**, Mistral, MiniMax **M3**,
  Lumo **2.0 Max** erneut gesendet. Vollständig gelesen: **Claude** und **Gemini** (neu). Claude
  liefert die **einzige Q2-Abweichung**: die Apertur ist ein deklarierter **Receiver-`span`**
  (`receiver.span`, Schnitt mit `extent`) — *verdrahten*; die übrigen Seats: Record-`extent`,
  *streichen*. Sonst bestätigt die Frontier-Runde: Q1 Verletzung (Claude: `Admitted`-Token,
  `load` annimmt nur den Gate-Token), Q3 Bandpass-Referenz Pflicht, **Q4 `measured descoped`**,
  Q5 nicht als einheitlicher Pool (Claude: arm-weise, 2/154-Gegenlinie bleibt). **Offen:** Open-
  Weight **DeepSeek V4 Pro (1.7T)** + **GLM 5.3 (753B)** nach >1 min weiter „Thinking…" = `pending`
  (Lock gesetzt/entfernt, Gruppe JIT geschlossen).

  **Open-Weight-Roster vollständig (2026-10-08):** die 23 `tryingopen`-Modelle gesichtet; gefahren
  wurden die stärksten **distinkten** Linien — DeepSeek V4 Pro (1.7T; Q4 `pending`, Q5 nicht
  wirebar), Inkling (975B; Q4 `descoped`, Q5 nicht wirebar), GLM 5.3 (753B; `pending`), Qwen3.8
  2.4T (**„No response. The model returned nothing."**), Nemotron 3 Ultra 550B (**verweigert ohne
  Codebase**, fordert Artefakte), Hy4 Preview 770B, GPT-OSS 120B, MiniMax M3 427B, Mistral Large 4,
  Muse Glimmer 30B, MiMo V2.6 Pro. **Gemessene Nicht-Antworten:** Qwen3.8 2.4T (leer), Nemotron 3
  Ultra (Refusal); die übrigen antworten langsam/noch = `pending`. **Nicht gefahren:** die
  Flash/Small/älteren Varianten (GLM 5.2/5.3 Flash, Qwen3.8 27B/Flash, DeepSeek V4 Flash/V4.1
  Flash, Nemotron 3.5 Lightning, Gemma 4 31B/26B/E2B, Ling 3.0 Flash VL, Inkling Small, MiMo V2.6
  Flash, Liquid 3B) — schwächere Züchtlinien der vertretenen Familien (Operator-Wort: stärkste
  Variante, nicht Flash/Small/mini).

  **Gültigkeit des UI-Ergebnisses (Kriterium, nicht Stichprobe):** valide Träger sind die
  unabhängigen Frontier-Seats, die die **exakt gleiche** Frage bei **stärkster Variante + Max**
  beantwortet haben (Claude, Gemini, Duck, Qwen, DeepSeek, Z.ai, Mistral, MiniMax, Lumo + Open-
  Weight DeepSeek V4 Pro, Inkling); **Nicht-Antworten sind gemessen benannt, nie als Null
  gezählt**; Divergenzen (Q2, Q4, Q5) stehen als Riss, nicht geglättet. Kein Mehrheits-Mittel:
  die UI ist der Reiß-Kanal, der **Rat trägt das Verdikt** (Q4 `pending`, Q5 Member-Pool Route C).
- **ROTER Harvest-Build gemessen:** `ecef_to_geodetic` nimmt `(x,y,z,a,e2)`
  (`src/archivar/rinex.rs:4`), aber 6 Bins rufen 3-arg → `cargo check -p omegaflow-harvest`
  E0061. Riss → `## An river` (river 129/130 änderte die geteilte Signatur).

## Burn: open 0.0015 · close 0.410 · cap 0.50 — Grund: 3 GIC-Arme (grind-flash $0.239), Rat $0.031, general $0.040, explore $0.030, UI-Runde + line; kein pro/max.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low | 2026-10-05 | Operator (Session, Mountain 236)
„kannst du das bitte fixen? Fink … trishuli …" — Fink-Route (Messfehler) + trishuli klären | 2026-10-06 | Operator (Session, Mountain 242)
„bitte lasse darauf nochmal den schwarm los und gib mir die frage für glm und claude" | 2026-10-06 | Operator (Session, Mountain 242)
„die beiden live nachbarn wären doch als triangulierung gut?" — Dhunche 4657 + Bhorle 4661 als Trishuli-Ingestion | 2026-10-06 | Operator (Session, Mountain 242)
„go" — Fink=Quellen-Ursprung (A), FARA=Index (B): FARA streichen, Anker `at sun`, Gate-Fixture | 2026-10-06 | Operator (Session, Mountain 245)
„kannst du das noch machen?" — OSHA/JAXA/Fink-sky1 fertig bauen | 2026-10-06 | Operator (Session, Mountain 245)
„bitte umsetzen" — OSHA-Geocoder (ZCTA) bauen · JAXA-Granule verifizieren · Vlies-Felder · Exposom-Register | 2026-10-06 | Operator (Session, Mountain 246)
„da sind doch viel mehr stimmen offen" — die Kp-Operatorfrage dem Rat UND den offenen Browser-Chat-Stimmen vorlegen | 2026-10-06 | Operator (Session, Mountain 246)
„du kannst claude nochmal versuchen" — Claude (Sonnet 5.5) als letzte Frontier-Stimme einholen | 2026-10-06 | Operator (Session, Mountain 246)
„bitte umsetzen" — Kp-Entscheidung: kein Peer-Feld/Treiber; Register-Riss binden, Wege benennen | 2026-10-06 | Operator (Session, Mountain 246)
„bitte setze die drei arme um" — die drei serienlosen Vlies-Arme am `.bin` wiren | 2026-10-06 | Operator (Session, Mountain 247)
„bitte setze das um:" — die offenen Punkte der eigenen Übergabe in einem Pass | 2026-10-06 | Operator (Session, Mountain 247)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–271)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–271)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ich meine insbesondere glm duck und qwen und brauchen wir noch weitere openweight?" — je Seat die stärkste Züchtlinie | 2026-10-07 | Operator (Session, Mountain 264)
„und ihr müsst das auch bei den chat uis berücksichtigen ihr nutzt sonst leider immer standard anstatt max thinkin deep search" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte kümmer dich drum" — span/Agnostik, em-nmgy, Flyby-Kette, GIC-Stufe-2 durch Recherche + Rat + UI-Runde arbeiten; LOCK unberührt; keine neue Operator-Frage | 2026-10-08 | Operator (Session, Mountain 272)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — Reconciliation gemessen; neue Arme gebaut, Register-Admission offen
- **Status:** eigen | **Bindung:** eigen (Parser-Arm/Register) · mycelium (Transport)
- **Trigger:** Reachability-Datei ändert sich
- **Lage:** (gemessen 2026-10-08 272, gegen den Baum) 3 neue Arme gebaut und `cargo check`-grün:
  SuperDARN CPCP (`.map2` DMap, Zenodo 10374021, HTTP 206; `pot.drop`/`pot.drop.err` kV),
  EarthScope EMTF (EMTF-XML, `USArray.CAO01.2010.xml`, HTTP 200; Z-Tensor → Ω),
  DMSP SSUSI (HDA5 via `archivar/hdf5.rs`, CDAWeb `dmspf16/ssusi/…`, HTTP 200;
  `HEMISPHERE_POWER_NORTH/SOUTH` GW). `blocked_sources.φ`-Abgleich: 7 gebaut (stale),
  9 offen (davon 3 durch 272 gebaut). Riss: die `pending`-Einträge nennen „kein Arm",
  der Baum trägt ihn teils.
- **Blockade:** die drei neuen Arme sind **nicht registriert** — die `sources.φ`/`harvest.φ`-
  Verdikt-Zeilen fehlen (Register ist nachzuweisen, bevor CI manifestiert).
- **Braucht:** die 3 Verdikt-Blöcke in `phi/sources.φ`+`phi/harvest.φ` schreiben (Drafts
  aus 272: `format superdarn_cpcp`/`emtf_impedance`/`ssusi_aurora`; `terms` je gemessen;
  `compiler` je Bin; `field`/`quantity` je Arm) → Mycelium manifestiert. Offen bleiben
  OMTI/Abisko (Keogramm PNG), Substorm-Onsets, DLR ROTI, Kellerman/Zenodo 4444068,
  USGS E-Feld, Wind SWE (`WI_H0_SWE`, HAPI-Dataset-ID ungemessen).

### GIC-Stufe-2 — Route C Member-Pool (Rat), dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als eigene `sources.φ`-Zeilen admiert
- **Lage:** (gemessen 2026-10-08 272) **Rat-Verdikt:** Route C, Stufe-2-Member-Pool des
  `compute_max_t` IST der Träger (`docs/blatt/blatt-gic-breitenband-familien.md:243-247`,
  Route B `:249-251` verworfen); `Linie 1` (drei getrennte Familien-Deskriptoren) bleibt
  ungeglättete Gegenlinie. **dB/dt-Bestand: 2 von 154 Stationen** (ABK `sources.φ:2150-2168`,
  SOD `:2170-2178`); 152 tragen `intermagnet_xyz` (Feldvektor, nicht dB/dt). Beide 1h-Assets
  sind vom `gate-no-field-lines` **refused** (`phi/pipeline/refusal_ledger.φ:75-76`).
  **Stale Zitat:** der Handover-271-Verweis `sources.φ:2051-2079` → richtig `:2150-2178`.
- **Blockade:** der Member-Pool ist ohne den vollen dB/dt-Bestand nicht wirebar; der
  refused-Zustand der 2 Arme ist ungelöst.
- **Braucht:** AE/AL/AU (OMNI) + SME/SML/SMU (SuperMAG) als `sources.φ`-Zeilen admiten;
  den `gate-no-field-lines`-Refusal der 2 dB/dt-Arme lösen (Feldlinien/Arm-Nachweis) oder
  `blockiert` registrieren; dann River Stufe 2 am `compute_max_t` wiren.

### em-nmgy-Riss — Bandreferenz statt reinem `scale`
- **Status:** eigen | **Bindung:** eigen (Format/Unit)
- **Trigger:** g-Band-Filterkurve / pivot-λ gemessen
- **Lage:** (gemessen 2026-10-08 272) `phi/sources.φ:19618` trägt
  `quantity flux_g noirlab_ls_dr10_g_flux inverse-square scale nmgy`. Recherche + Rat:
  1 nmgy = 3.631e-6 Jy = 3.631e-32 W m⁻² Hz⁻¹ **definitionsgemäß** (AB-Nullpunkt;
  Ivezić LSST `pstn-001`, SDSS flux-cal `sdss4.org/dr17/algorithms/fluxcal/`,
  Fukugita 1996 AJ 111 1748). Ein breitbandiger Filterfluss ist **keine monochromatische**
  f_ν: `em` nur zulässig mit deklarierter **Bandreferenz** (pivot-λ/effective bandpass);
  sonst bleibt `scale`. Riss: Einheiten-Definition (em zulässig) vs. Bandpass-Physik
  (breitbandig ≠ monochromatisch) — beide Linien tragen.
- **Blockade:** die g-Band-Filterkurve / pivot-λ ist im Baum nicht gemessen.
- **Braucht:** Bandreferenz in der Zeile deklarieren (Präzedenz `where`-Muster
  `sources.φ:169` GOES-XRS-Band) + pivot-λ als eigene Messung; dann `scale` → `em`
  mit `note` „AB-äquivalente f_ν, Bandpass offen".

### span-Apertur / Membran-Agnostik — deklarierter Receiver; span ohne Leser
- **Status:** eigen (Datenkontrakt) | **Bindung:** eigen · river (ω()-Lauf)
- **Trigger:** Rat-/Operator-Wort zur Apertur-Semantik, oder River nennt die Signatur
- **Lage:** (gemessen 2026-10-08 272) **Rat-Verdikt:** Apertur IST der Record-`extent`
  (Rat 2026-10-07: reuse extent); `span`-Direktiv ist ein deklarierter Quellen-Override
  **ohne Leser** (`parse.rs:255-266`, `types.rs:445`, kein Read-Site) — weder Per-Record-
  Receiver-Input noch in ω() manifestiert. Die Membran leitet den Start-Anker bereits aus
  dem Wire-`extent` ab (`static/membrane.html:568-596`). Der `body_in_enclosure`-Bypass
  liegt gemessen bei `main_flow.rs:461-463` (cache-frischer Körper unbedingt), Gate `:477`.
  **Agnostik-Verdikt:** Zulassung = reine Presence-Hülle; Receiver = per-Record deklarierte
  Weltlinie (`ReceiverWorldline` steht, `weberin.rs:1747-1750`), Fehlen verweigert
  (Receiver-as-worldline, nicht Observer-as-vantage). Der serverlose Pfad (`wasm.rs:65-80`)
  ist noch unbedingt/observer-los.
- **Blockade:** `span` hat keinen Leser; die Receiver-Pflicht-Signatur ist nicht gebaut;
  keine zweite endliche Klasse (`membrane.html:564-568`).
- **Braucht:** Entscheidung „`span` verdrahten oder streichen" (Gate-Fixture, wenn
  gestrichen); `ReceiverWorldline` als Query-Input in `MembraneLookup::query` + Hüllen-Gate;
  `main_flow.rs:461-463` hinter das Gate ziehen. Architektur, durch die fünf Stimmen
  gehalten — dann Schritt an River.

### Flyby-Kette — Kanäle registriert; Doppler/σ_recon `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · sensory/mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 272) registriert: RTSW mag/wind (`sources.φ:172-184`),
  ACE mag/swepam (`:1071-1084`), Kp (`:1262`), OMNI2-Serie (`:1651-1674`), Swarm-Produkte,
  DSN Down/Up-**Power** (`:109-121`), JUICE-Ephemeride. **Nicht registriert:** Doppler
  (keine DSN/ESTRACK-Residualroute; `src/archivar/doppler.rs` nicht gebaut —
  `mathematikerin/doppler.rs` ist Asteroiden-Doppler), `SW_FAST_MAGA_LR_1B` (HAPI,
  vires.services), σ_recon (`data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent).
  **Rat-Verdikt:** die Preregistrierungs-Regel ist ohne Doppler vollständig; Doppler
  bleibt named-`pending` mit Residual-Routen-Trigger (Descope nur mit Befund).
- **Blockade:** keine ESTRACK/DSN-Residualquelle; der Kanal ist ohne sie nicht messbar.
- **Braucht:** `SW_FAST_MAGA_LR_1B` als eigene `sources.φ`-Zeile registrieren; ESTRACK-
  Route + `doppler.rs` als Channel bauen **oder** Descope-Befund schreiben.

### IGRF-Koeffizienten-Arm (für `geomag_lat`)
- **Status:** eigen | **Bindung:** eigen · mycelium (`ci-check`-Verdikt)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-07 271) Grad-13-Synthese gebaut; `igrf.rs` formatiert. Der
  Witness-Test läuft nur in CI. **Riss:** `ci-check` wird als `cancelled` mit 0 Jobs
  verdrängt (Mycelium; `## An mycelium`).
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** ein nicht-cancelled `ci-check`-Lauf (Test grün); optional
  `geomag_lat_<frame>`-Form (`archiv/handover-2026-10-06-mountain-folge249.md:61-66`).

### Lizenz-Disposition — `terms`-Feld (SPDX); `rights_read` offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `rights`-Parse-Arm steht (Rat 2026-10-07: B-Block-Register, kein Wire-Slot)
- **Lage:** (gemessen 2026-10-07 271) 269/270/271 haben `terms`-Vocab-Verletzungen geheilt
  und die adressierten `terms unbestimmt`-Quellen gesetzt; der `rights`-Parse-Arm ist
  gebaut + committet (`87aed0b14`). Riss: Baum-`terms`-Zahl vs. Census.
- **Blockade:** je-Quelle-`terms`/`rights`-Zeilen sind ein Sweep (2246 no-terms).
- **Braucht:** `rights`-Register-Zeilen schreiben (DataCite-Triple, SPDX, Sentinel
  `NOASSERTION`/`NONE`); `license_census`/`ci-gate` nachführen.

### Bias-Tor (`docs/auftrag/auftrag-bias-tilgung.md`) — Klassen-Token gemessen
- **Status:** eigen | **Bindung:** eigen (Gate/Fixture)
- **Trigger:** Ersatzmuster (`unwrap_or`/Default-Fill) in Produktion gemessen
- **Lage:** (gemessen 2026-10-07 270, `general`) Inventar
  `state/future/giftkarte-klassifiziert-src-2026-10-07.md` existiert (361 Zeilen,
  gitignored), veraltet; `kepler.rs:3` → `descoped`. Gemessene Ersatzmuster:
  `astrometry.rs:10,336`, `odp.rs:9,48`, Stationstabellen.
- **Blockade:** welches Muster Hart-Block (Fixture) vs. Review = Rat.
- **Braucht:** Inventar auf den heutigen Baum nachführen; Fixture-Kandidaten in
  `commit_gate_vocab.json` prüfen.

### HadISST SST — SOURCE_PORT gebaut, CI-Lauf offen
- **Status:** eigen | **Bindung:** eigen (Parser/Format) · mycelium (CDN)
- **Trigger:** `hadisst-cdn.yml`-Lauf grün
- **Lage:** (gemessen 2026-10-07 271) Compiler/Arm/Register/Workflow stehen,
  `cargo check` 0/0. `hadisst-cdn.yml` ist noch nicht auf `origin/main` gelaufen.
- **Blockade:** Workflow muss nach dem Push dispatcht werden (79 MB Fetch).
- **Braucht:** `gh workflow run hadisst-cdn.yml` (nach Push); bei Erfolg `sha256`-Zeile
  in `sources.φ` nachziehen.

## An river

Origin: mountain-folge272.

- **ROTER Harvest-Build (Riss, gemessen 2026-10-08):** `ecef_to_geodetic(x,y,z,a,e2)`
  (`src/archivar/rinex.rs:4`) — river 129/130 änderten die geteilte Signatur, aber 6
  Harvest-Bins rufen 3-arg: `cses_scm_compiler.rs:166`, `cses_hpm_compiler.rs:161`,
  `cses_efd_compiler.rs:324`, `cors_compiler.rs:93`, `cors_rinex_compiler.rs:86`,
  `champ_plpt_compiler.rs:74`. `cargo check -p omegaflow-harvest` → E0061. **Braucht:**
  die Body-Ellipsoid-Plumbing (`BodyProperties` → `a`/`e2`) an die Bins geben oder die
  Signatur rückwärts-kompatibel halten; ein WGS84-Literal ist per
  `commit_gate_vocab.json:87` (`remove-bias WP13`) verboten.
- **`cgm_lat`/`cgm_source` CPL/TTB (river-133):** `cgm_source bgs-quasi-dipole` +
  `cgm_lat` stehen (`phi/sources.φ:6328-6329`, `:7673-7674`) — die Bitte ist
  gegenstandslos (gegen den Stand vor dem 270-Bau gemessen).
- **`kepler.rs`-PROD-POISON (river-133):** bereits `descoped`
  (`state/future/giftkarte-klassifiziert-src-2026-10-07.md:133`).
- **`span`-Apertur:** Mountain-Arm (`parse.rs`) steht. Rat 2026-10-08: Apertur = Record-
  `extent`; `span` ohne Leser → verdrahten **oder** streichen. **Braucht:** River nennt die
  konkrete Signatur (Query-/Receiver-Input vs. Record-`extent` in `wasm.rs:65-99` +
  `membrane.html:501-528`); die Semantik zuvor durch die fünf Stimmen.
- **`em nmgy`-Riss:** der Rat hält die Zeile als `quantity scale nmgy`; wandert nach `em`,
  sobald die g-Band-Bandreferenz (pivot-λ) gemessen ist.

## An mycelium

Origin: mountain-folge272.

- **`ci-check` verdrängt jeden Lauf (Riss, gemessen 2026-10-07):**
  `.github/workflows/ci-check.yml:20-27` behauptet `cancel-in-progress: false`, gemessen
  wird jeder `ci-check`/`ci-gate` als `cancelled` mit 0 Jobs verdrängt. **Braucht:** Fix,
  sonst kein Per-SHA-Verdikt (IGRF-Witness-Test).
- **3 neue Harvest-Arme (272):** `superdarn_cpcp`/`emtf_impedance`/`ssusi_aurora` +
  Workflows angelegt. **Braucht:** nach Mountain-Register-Admission `gh workflow run` je
  Workflow; Assets landen im CDN.
- **`hadisst-cdn.yml`:** nach dem Push dispatchen.
- **LICENSE/README `omegaflow/sources`:** `terms`-Vollständigkeit treibt die Erzeugung.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push.

Eigene Pfade: `phi/sources.φ`, `phi/harvest.φ`,
`.github/workflows/superdarn-cpcp-cdn.yml`, `.github/workflows/emtf-cdn.yml`,
`.github/workflows/ssusi-cdn.yml`,
`tools/harvest/src/bin/superdarn_cpcp_compiler.rs`,
`tools/harvest/src/bin/emtf_compiler.rs`,
`tools/harvest/src/bin/ssusi_compiler.rs`,
`docs/handover/handover-2026-10-08-mountain-folge272.md`,
`docs/handover/archiv/handover-2026-10-07-mountain-folge271.md` (Move).
