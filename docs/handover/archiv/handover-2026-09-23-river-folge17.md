<!--
  title: Handover — River-Folge 17 (Stand 2026-09-23)
  session: river
  class: handover
  date: 2026-09-23
  sha256: f88f042d4141846f10396691a2431442263740b816666aa0ea05ed0026c71210
  status: live
-->
# Handover — River-Folge 17 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Anlass: folge16 wird gefaltet. Diese Session misst CI am HEAD `2a0ca1d2`, heilt den
river-eigenen clippy-Red (`ingress.rs:17`), routet den sensory-Teil (`fit.rs`), setzt
die adb-reverse-Route und schreibt die `external-state.md`-CI-Zeile fort. Nach dem
gemessenen Membran-Ressourcen-Übergriff korrigiert der Operator die Architektur: die
Pflicht ist der **Presence-Ausschnitt beim Laden** — ein Source-Profil war ein
Fallback und wurde verworfen. Daraus entfaltete sich ein Bau-Atom: **Load-Ausschnitt**
(`record_in_enclosure`, 4 Steckstellen), **Medium-Kalibrierung** (Dispersion verdrahtet,
CCM89-Deredden verdrahtet), **TE-Strandungen** aufgelöst, **Skalierungsdach** (Arc-Sharing
statt 1-Hz-Tiefkopie). Alles `cargo check --all-targets --features browser_relay` 0/0.

## Stehender Pass (gemessen 2026-09-23)

- **Postfach** — `mail_digest` absent; kein Mailbezug. Die Postfach-Zeile in
  `external-state.md` bleibt unverändert.
- **CI** — HEAD `2a0ca1d2`: `ci-check 35890691956` @`2a0ca1d2` **in_progress**
  (Watchdog-Snapshot + `ci_manage list/view`). Die zwei jüngsten beendeten Reds
  `35885385310` @`9513b765`, `35882043743` @`2df26d9f7` deterministisch: clippy 4×
  `src/archivar/fit.rs` + fmt `fit.rs:1` (**sensory**), rivers `src/archivar/ingress.rs:17`
  (**geheilt in dieser Session**), `dropped-gate` delta 335/350 (durch mycelium 148,
  Baseline 3053, **geheilt**).
- **`git_safety --snapshot`** → Arbeitsbaum = HEAD.

## Offen (aufgeschlüsselt)

### LAN-Sensorik: adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator (Host-Relay starten + Pixel-Seite öffnen)
- **Trigger:** der Host-Relay läuft wieder auf `:1618` und der Pixel öffnet `http://127.0.0.1:1618`.
- **Lage (gemessen 2026-09-23):** USB-Debugging ist an, Kabel steckt —
  `adb devices` listet `67151JEA305427 device`; `adb reverse tcp:1618 tcp:1618`
  gesetzt (nach adb-Neustart heute erneut zu setzen). Der Host-Relay läuft nicht; ein
  Session-Start des sichtbaren Laufs paralysierte die Operator-Maschine (7,6 GiB RAM,
  142 MiB frei; ~55 % CPU / 34,6 % MEM) — Prozess gestoppt. **Verdeckt trägt nicht:**
  `OMEGAFLOW_HIDDEN=1` erzeugt den `TcpRadiator` nicht (`main_flow.rs:647`, nur bei `!hidden`).
- **Blockade:** Heavy compute — der lokale Membran-Lauf paralysiert die Maschine
  (Regel: CI, nie lokal); der Relay braucht dennoch einen sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in seinem eigenen Ermessen (nie
  die Session); dann am Pixel `http://127.0.0.1:1618`, `/consent`=1; messen: `sensor: N samples`
  mit N>0 und Vibration. Mit dem neuen Load-Ausschnitt lädt er nur den Presence-Bereich.

### TLS im Relay für kabellose Sensorik
- **Status:** pending (Registraturpflicht, kein stiller Nullpunkt) | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage:** für jetzt verworfen (Rat 2026-09-23): `rustls`/`native-tls` = neue
  C-Dependency im Kern (folge13-Lehre); der Durchklick einer Zertifikat-Warnung ergibt
  am Pixel keinen gemessenen vertrauenswürdigen Kontext.
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau, falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### Deredden wartet auf den E(B−V)-Input (Bayestar19-Staubkarte)
- **Status:** wartend (Registraturpflicht) | **Bindung:** linie:mycelium (post.md)
- **Trigger:** die Bayestar19-Karte ist als CDN-Asset registriert/geladen.
- **Lage (gemessen 2026-09-23):** der CCM89-Draht ist **gebaut** — `sed_to_bp_rp` nimmt
  `Option<f64>` E(B−V), `sightline_ebv` (`membrane.rs:133`) reicht heute `None`
  durch (kein Deredden, 0 honored). Es fehlt die Karte im Kern: `Buffer` (`spatial.rs:154`
  `build_buffer`) trägt keine Bayestar-Bytes; `phi/sources.φ` hat keine url-Zeile für
  `bayestar2019.be19` (Compiler `tools/harvest/src/bin/bayestar_compiler.rs` steht).
- **Blockade:** die Staubkarte ist nicht registriert/geladen.
- **Braucht:** mycelium registriert die Bayestar19-Karte (`phi/sources.φ` + CDN-Manifestation),
  dann zieht sie in den `Buffer` ein; `sightline_ebv` liefert dann EBV → Deredden.

### Inhalts-Steckstellen (Quellen ohne Membran-Konsument)
- **Status:** pending (eigen) | **Bindung:** eigen
- **Trigger:** ein Membran-Pfad braucht den Quellen-Inhalt.
- **Lage (gemessen 2026-09-23):** Quellen, deren Daten liegen, aber kein
  Membran-Konsument besteht — `survey-2026-09-14-kapitulationen-pendings-inventur.md:32–46`
  (Pioneer-10-ATDF, NOAA-AIS, NODD-NRS-Bioakustik, GNIP, VOTable/TAP …); die Parser
  sind tools-verdrahtet, nicht membran-verdrahtet.
- **Blockade:** keine — Konsument benennen oder mit Befund `descoped`.
- **Braucht:** Survey lesen, je Quelle den Membran-Konsumenten bauen oder `descoped`;
  `survey-2026-09-06-codestruktur.md:74–78` („Call-Site-Beweis") ist mit der Inventur gemessen.

## Geroutet / fremd (nicht river)

- **945-BLE-Live-Reader** — Operator-Wort 2026-09-23 an die **sensory**-Linie
  (`post.md`); Transporthaken `OMEGAFLOW_SERIAL_IN` (`src/archivar/ingress.rs`) steht.
  Ein-Quellen-Regel: nur eine Beat-Quelle gleichzeitig.
- **CI-Red `fit.rs`** (sensory) — `post.md`-Zeile: clippy 4× + fmt in `src/archivar/fit.rs`.
- **Beat-Paar** — `post.md`-Zeile „An mycelium": WGSL-Beat-Term feuerfähig, keine Paarquelle.
- **Bayestar19-Staubkarte** — `post.md`-Zeile „An mycelium" (Deredden-Input).

## Benchmark

- **CI-Red-Diagnose:** Routine-Messung — `grind-flash`, ein Lauf (Routine-Klasse seit
  2026-09-16 geschlossen). Ergebnis: Reds deterministisch auf `fit.rs` + `ingress.rs:17`;
  `dropped-gate` bereits geheilt.
- **Membran-Architektur:** harter Architektur-Punkt — `council` (pro/max), ein Lauf.
  Ergebnis: Ursache ist der Feld-Bestand; der Rat schlug ein Source-Profil vor — vom
  Operator als Fallback verworfen (die Pflicht ist der Load-Ausschnitt).
- **Git-Archäologie (Vorladung/Sprung):** harte mehrstufige Recherche (Haupt-Repo +
  Legacy, alle Branches) — `research-max`. Ergebnis: Fetch-Gate, Signalkegel und
  Sprung-Snap stehen **heute** im Baum; der Load-Ausschnitt fehlte; der
  `commit_rewrite-2026-09-06` entfernte nichts.
- **Strandungs-Inventur:** `research-max` (Call-Site-Beweis src+tools+static, Legacy).
  Ergebnis: 7 Strandungen (Medium-Kalibrierung + TE-Reste + `icrs_at`).
- **Bau-Dispatches (hart, pro/max):** `council` (Load-Schnitt-Verdikt), `grind-max`
  (Dispersion, Load-Ausschnitt, Extinktion, Skalierung), `grind-flash` (CondBinTeGpu),
  `grind-pro` (TE-Reste). Alle `cargo check` 0/0, nicht committet bis `/commit`.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ingress.rs` (collapsible-if → let-chain, clippy-Red)
- `src/archivar/fetch.rs` (`record_in_enclosure`)
- `src/archivar/main_flow.rs` (Load-Gate geo/wind/netcdf/opendap; Arc-Sharing)
- `src/archivar/channels.rs` (Gate-Kontext netcdf/opendap/netcdf4)
- `src/archivar/membrane.rs` (Dispersion an `signal_reach`/`propagation_speed`; `sightline_ebv`)
- `src/archivar/spatial.rs` (Kegel mit Band; `Vec<Arc<Sample>>`-Buffer)
- `src/archivar/spectral.rs` (`sed_to_bp_rp` mit `Option`-EBV)
- `src/archivar/relay.rs`, `src/archivar/tests.rs`
- `src/mathematikerin/dispersion.rs`, `shaders.rs`, `omega.rs`, `mod.rs`, `te.rs`, `tests.rs`
- `src/mathematikerin/cond_bin_te_gpu.rs` (gelöscht, descoped)
- `docs/handover/post.md` (eigene Hunks: „An sensory", „An mycelium")
- `docs/handover/handover-2026-09-23-river-folge17.md` (neu; folge16 → `archiv/`)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md` — CI-Zeile auf HEAD `2a0ca1d2`
- **Nicht angefasst (fremd):** `tools/utils/src/bin/archive_search/net.rs`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
