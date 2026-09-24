<!--
  title: Handover — River-Folge 18 (Stand 2026-09-23)
  session: river
  class: handover
  date: 2026-09-23
  sha256: 72c8260a3246fb62a660279e3c72b3921aaa985306b868ebff93017d7ae751db
  status: live
-->
# Handover — River-Folge 18 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Anlass: folge17 wird gefaltet. Die Session misst CI am HEAD und findet `ci-check
35915810580` @`486b8e6c` **failure** — rivers **eigener** clippy-Red aus `7a269ef1a`:
`fetch.rs:467` too-many-args (11/7), `dispersion.rs:71` negierter Partial-Ord-Vergleich
+ `:74`/`:181` `chunks_exact` konstante Größe, `spectral.rs:673`/`:674` needless-borrow.
Geheilt: `record_in_enclosure` 11→6 Argumente (`EnclosureField<'a>` + `AnchorEnvelope`,
alle 5 Load-Steckstellen + Tests umgestellt), `freq.is_finite() && freq > 0.0` als
Positiv-Gate, `as_chunks::<SHELF_GPU_ROW_FLOATS>().0`, `passband_at(table, …)`;
`cargo check --all-targets --features browser_relay` 0/0. Sensory hat `fit.rs`/`ble.rs`
in `48ae5e734` geheilt. Der Punkt **Inhalts-Steckstellen** ist per Messung geschlossen
(folge17-Inventur): alle 11 Survey-Einträge sind heute konsumiert oder bereits
`declined`/`dead`. Die `An river:`-Post-Zeile (Membran-/Sensor-Punkte) ist gefaltet.

## Stehender Pass (gemessen 2026-09-23)

- **Postfach** — `state/mail/`/`mail_ledger.φ` in diesem Baum absent, `mail_digest`
  pending (CI-Bau); keine Mail. Die Postfach-Zeile in `external-state.md` bleibt.
- **CI** — HEAD `d87a0c9b3`; `ci-check 35915810580` @`486b8e6c` **failure** (rivers
  clippy, in folge18 geheilt), jüngster Lauf `35923601360` @HEAD **pending**;
  `dropped-gate` delta +118 (baseline 3053 | current 3171) — an mycelium geroutet.
- **`git_safety --snapshot`** → Arbeitsbaum = HEAD.

## Offen (aufgeschlüsselt)

### CI-Verifikation: clippy-Heilung folge18
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check` des folge18-Commits läuft nach `/commit`+Push aus.
- **Lage (gemessen 2026-09-23):** river-Red aus `7a269ef1a` geheilt
  (`cargo check --all-targets --features browser_relay` 0/0); nur die CI-Messung fehlt.
- **Blockade:** keine.
- **Braucht:** `ci_manage view <id>` bzw. Watchdog-Snapshot im nächsten Pass; grün =
  Load-Ausschnitt + Folgemedium functional verifiziert.

### LAN-Sensorik: adb-reverse-Route (Rat-Verdikt 2026-09-23)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Host-Relay läuft auf `:1618` + Pixel öffnet `http://127.0.0.1:1618`.
- **Lage (gemessen 2026-09-23):** `adb devices` listet `67151JEA305427 device`;
  `adb reverse tcp:1618 tcp:1618` gesetzt (nach adb-Neustart erneut zu setzen);
  Host-Relay läuft nicht; `OMEGAFLOW_HIDDEN=1` erzeugt keinen `TcpRadiator`
  (`main_flow.rs:647`, nur bei `!hidden`). Ein Session-Start paralysierte die Maschine.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — der Relay braucht dennoch
  einen sichtbaren Lauf.
- **Braucht:** der **Operator** startet `bin/omegaflow` in eigenem Ermessen; dann am
  Pixel die URL, `/consent`=1; messen `sensor: N samples` >0 + Vibration. Mit dem
  Load-Ausschnitt lädt er nur den Presence-Bereich.

### Sensor-Bindung vC-Permeabilität (Smartwatch + Mantis-Shrimp)
- **Status:** termin | **Bindung:** operator (Hardware/versteckter Lauf)
- **Trigger:** Smartwatch + Mantis-Shrimp-Sensoren sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage (gemessen 2026-09-23):** der Permeabilitäts-Pfad steht (vC → `fieldPermeability`
  in `src/mathematikerin/omega.rs`); der HRV-Reader ist gebaut (`src/archivar/ble.rs`,
  external-state Zeile BLE-HR-Reader); `docs/handover/handover-2026-09-20-operator-entscheidungen.md:26`
  hält den `termin` (hidden sensor run + `perm_target_probe --live`).
- **Blockade:** Hardware fehlt (Sensoren nicht angeschlossen).
- **Braucht:** Operator führt `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run
  --release` aus, dann `perm_target_probe --live <pfad>`.

### Sensor-Hardware: Mantis-Shrimp Node / ESP32-S3 + HRV-Teile (BOM)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator beschafft/an schließt die Sensor-Hardware an.
- **Lage (gemessen 2026-09-23):** `esp32-firmware`-Workflow existiert (CI); der
  Mantis-Shrimp-Node und die HRV-Teile (BOM) sind nicht beschafft/gemessen. Die
  Geräte-Anbindung ist in `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md`
  (See-also) inventarisiert.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber).
- **Braucht:** Operator beschafft die Teile; Session kann die BOM/den Node bauen,
  sobald die Teile gemessen vorhanden sind.

### 945-FIT-Datei (Watch onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad wird gemessen nötig.
- **Lage (gemessen 2026-09-23):** die Garmin 945 liefert onboard nur FIT/CIQ; der
  Live-Reader ist als BLE-FIT-Pfad an **sensory** geroutet (`post.md`, folge17,
  Transporthaken `OMEGAFLOW_SERIAL_IN` `src/archivar/ingress.rs`). Ein Onboard-Datei-
  Reader ist nicht gebaut/gemessen.
- **Blockade:** keine — bewusst `pending`; Ein-Quellen-Regel (nur eine Beat-Quelle).
- **Braucht:** erste Messung: wer hält den FIT/CIQ-Datei-Pfad (sensory vs river);
  Ein-Quellen-Regel gegen `OMEGAFLOW_BLE_HR`/`OMEGAFLOW_SERIAL_IN` prüfen.

### TLS im Relay für kabellose Sensorik
- **Status:** pending | **Bindung:** eigen (Rat/Bau)
- **Trigger:** ein kabelloser Sensor wird gemessen nötig.
- **Lage (gemessen 2026-09-23):** für jetzt verworfen (Rat 2026-09-23): `rustls`/
  `native-tls` = neue C-Dependency im Kern (folge13-Lehre).
- **Blockade:** keine — bewusst `pending`.
- **Braucht:** Rat/Bau falls kabellos nötig; dann privates CA-Zertifikat am Gerät.

### Deredden wartet auf den E(B−V)-Input (Bayestar19-Staubkarte)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** Bayestar19 ist als CDN-Asset registriert/geladen.
- **Lage (gemessen 2026-09-23):** der CCM89-Draht ist gebaut (`sed_to_bp_rp` nimmt
  `Option<f64>` E(B−V), `sightline_ebv` `membrane.rs:133` reicht `None` durch = 0
  honored); `Buffer` (`spatial.rs:154`) trägt keine Bayestar-Bytes; `phi/sources.φ`
  hat keine url-Zeile.
- **Blockade:** die Staubkarte ist nicht registriert/geladen.
- **Braucht:** mycelium registriert Bayestar19 (`phi/sources.φ` + CDN-Manifestation),
  dann zieht sie in den `Buffer` ein.

## Geroutet / fremd (nicht river)

- **dropped-gate delta +118** — `post.md` an **mycelium**: die φ-Register-Drops
  (`5d4655e24`, folge149) brauchen den Baseline-Bump ({3053 → gemessen}) im annehmenden
  Commit; river fasste kein φ an.
- **CI-Red `fit.rs`/`ble.rs`** (sensory) — durch `48ae5e734` geheilt; Post-Zeile gefaltet.
- **945-BLE-Live-Reader** — Operator-Wort 2026-09-23 an **sensory** (`post.md`);
  Transporthaken `OMEGAFLOW_SERIAL_IN` (`src/archivar/ingress.rs`) steht.
- **Beat-Paar** — `post.md` an mycelium (WGSL-Beat-Term feuerfähig, keine Paarquelle).
- **Bayestar19-Staubkarte** — `post.md` an mycelium (Deredden-Input).
- **SSDC Limadou** — `post.md` an mycelium (Konto/prozedur).

## Benchmark

- **CI-Red-Diagnose:** Routine-Messung — `build` (flash), `ci_manage log 35915810580`
  (exakte clippy-Zeilen) + `ci_manage view` (head_sha) — ein Lauf. Ergebnis: der Red
  ist rivers eigener `7a269ef1a`-clippy, nicht fremd.
- **Inhalts-Steckstellen-Verdikt:** Judgment — `grind-pro` (pro), ein Lauf. Ergebnis:
  alle 11 Survey-Einträge (a) konsumiert oder (c) bereits `declined`/`dead`; keine
  Register-Edits nötig; Punkt geschlossen.
- **Clippy-Heilung (hartes Bau-Atom):** `grind-pro` (pro), ein Lauf. Ergebnis:
  `EnclosureField`/`AnchorEnvelope`-Restruktur (`fetch.rs`), `as_chunks`/Positiv-Gate
  (`dispersion.rs`), needless-borrow (`spectral.rs`); `cargo check --all-targets
  --features browser_relay` 0/0.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/fetch.rs` (`EnclosureField`/`AnchorEnvelope`, 6-Arg-Signatur)
- `src/archivar/main_flow.rs` (2 Load-Steckstellen auf Struct-Form)
- `src/archivar/channels.rs` (3 Load-Steckstellen auf Struct-Form)
- `src/archivar/tests.rs` (`record_in_enclosure`-Aufrufe auf Struct-Form)
- `src/archivar/spectral.rs` (`passband_at` needless-borrow)
- `src/mathematikerin/dispersion.rs` (Positiv-Gate + `as_chunks`)
- `docs/handover/post.md` (eigene Hunks: „An river" gefaltet, „An mycelium" dropped-gate)
- `docs/handover/handover-2026-09-23-river-folge18.md` (neu; folge17 → `archiv/`)
- **lokal, gitignored (nicht im Commit):** `docs/zustand/external-state.md` — CI-Zeile auf HEAD `d87a0c9b3`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
