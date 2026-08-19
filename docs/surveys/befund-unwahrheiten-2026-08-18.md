# Befund der Unwahrheiten — Archäologische Säuberung (2026-08-18)

Vermessung des Repositories auf Lügen, Widersprüche und Karteileichen nach den
Session-Abbrüchen, Hardresets und der Umbenennung `phi/port` → `phi/pipeline`.
Methode: read-only (cargo check, URL-Abgleich dead↔live, grep-Inventar, zeilenweises
Lesen der Register). Nichts wurde getilgt — die Räumung entscheidet der Operator.

## 1. Register (die `.φ`-Dateien)

### WAHR befunden (kein Widerspruch, nicht erneut gräben)

- **Keine exakte URL steht in beiden Registern** (dead ∩ live = ∅).
- Der vermutete netCDF-Tombstone existiert so nicht: `dead_sources.φ:6` ist die
  `ftp.bom.gov.au`-Route (curl 78, tot). Die live netCDF-Quelle
  `data-argo.ifremer.fr` (`sources.φ:1052-1062`, `format netcdf`) hat **keinen**
  Tombstone. Die emodnet- (`dead_sources.φ:2149-2151`) und temis-
  (`dead_sources.φ:3906`) netCDF-Einträge sind echte 404s anderer Routen.
- openaq-Tombstone (`dead_sources.φ:533-535`, „integrated fanout-p09") und die
  wheretheiss-Tombstones (`:29-31`, `:621-627`, „superseded-by-ephemeris")
  benennen die Live-Quelle selbst — Dispositions-Notizen.
- 40 gemeinsame Hosts zwischen den Registern: alle Einträge sind durch
  „decline variant"-Notizen oder Routen-Unterschiede erklärt (z. B. BOM:
  dead = ftp-Mirror + IDD60901-Darwin, live = IDN60901 `sources.φ:134`).

### Befunde

**B1 — `staging_verified.φ` trägt zwei decline-URLs.**
`phi/pipeline/stage/staging_verified.φ:13-16` (qrng.anu.edu.au — im Register
`dead_sources.φ:2257` *decline randomness*) und `:20` (query1.finance.yahoo.com —
`dead_sources.φ:2277` *decline no-physical-force*, zusätzlich TODO:926 *Rejected/DROP*).
Die „verified"-Datei widerspricht dem Register. `index.φ:77` markiert sie zwar als
„artefakt" — der Dateiname bleibt eine Lüge.

**B2 — `index.φ` referenziert durchgehend `port/`-Pfade.**
`phi/pipeline/index.φ:6-15, 77-79, 87-99` zeigen auf den alten Baum
(`port/master_urls.txt`, `port/ledger.φ`, `port/stage/staging_verified.φ` …) —
nach der Umbenennung ins Leere; die Dateien leben heute unter `phi/pipeline/…`.
Auch die Blockzahlen sind veraltet (`:46` „ausstehend 0 queue/grind_domain_coverage.φ"
— die Datei trägt ~255 Host-Zeilen; `:36` grind_esa_full, `:44-48` mehrere 0er).

**B3 — `ledger.φ` trägt 14 tote Queue-Pfade.**
`phi/pipeline/ledger.φ:2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58`
zeigen auf `queue/sources_*_untested_*.φ`-Korpora, die dort nicht mehr liegen
(im Queue existieren nur noch `master.φ` und das Join-Paar
`sources_potential_pre-cdn_{9k_richest,params}.φ`). Die Korpora liegen im Archiv
(TODO:847-853 bestätigt die Archiv-Pfade) — das Register wurde nie nachgezogen.

**B4 — `park/` existiert nicht.**
TODO:616 und `docs/SOURCE_PORT.md:56,165` führen `park/` als Arbeitsfläche,
TODO:638-639 parkt Pegelonline, USGS-Geomag, GWOSC/GraceDB, DSN, CENC, JMA-Quake,
SDSS-SkyServer dorthin — der Ordner fehlt (geparkt wird vermutlich in
`blocked_sources.φ`). Doku-Drift oder verlorener Ordner.

## 2. Rust-Code (`src/`)

- **`cargo check`: 0 Fehler, 0 Warnungen.** Keine toten Funktionen — die
  `pack_deep_pt`/`pack_deep_ex`/`pack_deep_directions`-Reste sind wirklich getilgt.
- **Keine** `#[allow]`, `todo!`, `unimplemented!` in src/. **Keine** `phi/port`-
  Referenzen im Code. Die Umbenennung ist im Code vollzogen.

### TODO-Urteile gegen den Code verifiziert (alle WAHR)

transfer_entropy (`main.rs:16045`), surrogate_threshold (`:16097`), tile_cull
(`:463`), star_cull (`:609`), hud_raster (`:17415`), aberration (`:82`),
hill_radius_m (`dastcom.rs:204`), parse_star_record exakt 44 Byte
(`main.rs:2301-2331`, Legacy-40/48-B wird abgelehnt — Test `:13364-13366`),
MAST_TOKEN (`kernel_flatten.yml:200`), pastel/wds/mktypes/denis-Sync aus
kernel_flatten.yml entfernt (keine Treffer), n_sections=3
(`ephemeris_compiler.rs:442`), chunk_master.py existiert, Deep-Sprite-Pässe
(deep_pt_vs/deep_vs/near_pt_vs) sind aus main.rs getilgt.

### Befunde

**B5 — TODO-Selbstwiderspruch deep_pt_vs/deep_vs.**
`TODO.md:329-330` (Funktionsinventar) sagt „deep_pt_vs/deep_vs UNWAHR
(orthografische Scheibe — P3)" — aber TODO:138-141 (Subpixel-Wahrheit) und
TODO:385-386 sagen, die Pässe seien getilgt. Der Code bestätigt die Tilgung.
Die Urteilszeilen 329-330 sind Karteileichen.

**B6 — Stale Funktionsinventar.**
`TODO.md:368` listet `pack_deep_pt(stars:` / `pack_deep_ex(stars:` — existieren
nicht. `TODO.md:372` `pack_deep_directions()` — existiert nicht. `TODO.md:373`
`force_ref_medians_relaxes_absent_channels_to_zero()` — heißt heute
`force_ref_medians_holds_reference_on_absence` (`main.rs:19455`).

**B7 — P2 „Relay-Rest spd/hdg" möglicherweise erledigt.**
`TODO.md:209-211` markiert „SurfaceFlow für spd/hdg (sensor_config hat keinen
spd/hdg-Zweig für die Browser-Station)" als offen — aber `frame_motion`
(`main.rs:3014-3042`) trägt exakt den als fehlend markierten Zweig
(`(Some(s), Some(h)) if s > 0.0 → surface_motion`). Entweder ist der Punkt
erledigt, oder der TODO-Eintrag beschreibt den falschen Rest. Verifizierung
gegen den Browser-Relay-Ingress nötig.

**B8 — `unwrap_or`-Brücken (Fabrikationskandidaten):**

| Stelle | Befund |
|---|---|
| `tycho2_compiler.rs:560,738`; `tap_compiler.rs:554,1324` | `rv = unwrap_or(0.0)` — rv 0 m/s ist nicht „unbekannt". Von TODO:173-176 als Council-Frage registriert, aber die 0.0 ist eine Fabrikation. |
| `cometels_compiler.rs:256` | `H = unwrap_or(0.0)` — H=0 ist astronomisch unmöglich hell. |
| `mpcobs_compiler.rs:33,82` | `parse().unwrap_or(0)` — mag=0 wäre ein echter Sternwert. |
| `main.rs:1592` | `flattening = 0.0` wenn `radii_c` fehlt — sphärische Annahme statt Absenz (nur Rotationsmatrizen, Grauzone). |
| `main.rs:8355/8363` | Referenz-Sphäre (`CELESTIAL_SPHERE_RADIUS_M`) bei absent z — registriertes Verhalten (`test_extract_cmap_no_distance_reference_sphere`), keine versteckte Fabrikation. |
| `fk.rs:82,93`; `fits.rs:107-114`; CLI-Defaults (dataverse/oai/sexagesimal/horizons/tap) | Parser-Mechanik / Struktur / Argument-Defaults — WAHR. |

## 3. Pipeline (`phi/pipeline/`)

**B9 — Leere Karteileichen (0 Bytes):**
`phi/pipeline/probe_survivors.φ`, `phi/pipeline/probe_jina.txt`,
`phi/pipeline/probe_url_void.txt`. Dazu `phi/pipeline/probe_all_survivors.φ`
(1 Zeile Header „kumulative Survivors aller Batch-Proben", 0 Einträge) — die
Datei behauptet per Name und Header den Bestand und trägt nichts.

**B10 — `staging_empty.txt` ist nicht leer.**
3713 Zeilen void-URLs — der Name beschreibt das Inhaltsregister („fetch returned
empty"), keine Lüge im engeren Sinn, aber eine Name↔Inhalt-Kippung.

**B11 — `probe_all_void.txt`: 2.723.263 Zeilen.**
Der Riesen-Void-Ledger — Speicher-Karteileiche, Kandidat fürs Archiv.

**B12 — Queue-Drafts mit integrated-Dispositionen (historisch, kein Lügenfund):**
`grind_nasa.φ:104-105, 170-171`, `grind_vires_full.φ:151-566` (integrated-Notizen),
`grind_terrapulse_a/b.φ` (Dispositionen laut TODO erledigt gespiegelt),
`grind_domain_coverage.φ` (Host-Klassifikationen „live in sources.φ"). Sie sind
Protokoll-Arbeitsfläche (SOURCE_PORT) — Räumung ist Operator-Urteil.

## Entscheidungspunkte für den Operator

1. **Löschen (offensichtlich):** die 4 leeren probe-Dateien (B9); TODO-Zeilen
   329-330 (B5); die toten Inventarnamen in TODO:368/372/373 (B6).
2. **Register-Urteil:** rv-`unwrap_or(0.0)` (B8, Council-Frage aus TODO:173-176
   fällig); cometels H; mpcobs mag.
3. **staging_verified.φ:** qrng/yahoo-Blöcke entfernen oder die Datei als
   artefakt umbenennen (B1).
4. **Pfad-Wahrheit:** index.φ port/-Pfade auf pipeline/-Pfade umschreiben (B2);
   ledger.φ-Queue-Pfade auf das Archiv umschreiben (B3); TODO:627-628
   „Queue: 10 Untested-Korpora" an die Archiv-Realität anpassen.
5. **park/:** Verzeichnis anlegen oder SOURCE_PORT.md/TODO umschreiben (B4).
6. **P2:** spd/hdg-Rest gegen den Browser-Relay-Ingress verifizieren, dann
   TODO-Eintrag schließen oder präzisieren (B7).
7. **Räumung:** probe_all_void.txt archivieren (B11); integrated-Grind-Drafts
   nach Katalog/Archiv (B12) — nur nach Operator-Urteil.
