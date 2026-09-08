<!--
  title: Befund — TODO gegen den echten Code: was offen ist und welche Leichen wir tragen
  class: befund
  date: 2026-09-07
  status: done
  sha256: f88ea856ad159152c5cf390139cd1cb974d261e5a0edf745f2f48b5ac89af1f0
  see-also: docs/TODO.md
-->

# Befund: die TODO gegen den echten Code — offene Pflicht und getragene Leichen

**Datum:** 2026-09-07 · **Axiom:** A = A · **Verdikt-Ordnung:** 0 honored — die Leiche wird benannt, nie geglättet; was nicht gemessen werden kann, ist `unverifiziert`, nie ein Urteil.

Dieses Blatt führt den Code-Abgleich der `docs/TODO.md` (2688 Zeilen, Stand der Messung) gegen den realen Baum. Die TODO ist ein lebendes Register und wurde während der Messung uncommittet weiter editiert — jede Referenz hier ist daher **formfrei** (Dateiname/Behauptung, keine feste Zeilenzahl), damit der Befund morgen noch trägt.

**Mess-Methode:** grep/glob über `src/**`, `tools/*/src/bin`, `.github/workflows`, `phi/*.φ`; `gh api` gegen `omegaflow/sources` (CDN); git-Historie (747 Commits). Keine Spekulation — jeder Fund trägt einen Beleg. Wo der Beleg fehlt, steht `unverifiziert`.

---

## Kurzfassung — die Bilanz

Die TODO lügt an den meisten Stellen **nicht**: die große Mehrheit der als „gebaut" behaupteten Rust-Bins, te.rs-Symbole, Identitäts-Gates (`FeldIdentitaet`/`zeugen_gate`/`serien_gate`), die gbco-Thread-Integration und die 49 Format-Module existieren real und wie behauptet. Die echten Leichen konzentrieren sich auf **eine große Ecke — die Browser-Station** — plus wenige einzelne Namen und Pfade. Zwei CDN-Behauptungen der TODO sind durch die CDN-Messung korrigiert.

---

## Klasse 1 — die große Leiche: die Browser-Station (kopfloser Rumpf)

**Fund (gemessen):** `index.html`, `constants.js`, jede `.wgsl`-Datei und jedes Browser-JS (DataView-Parser, wheel/touch, `windowMedianExtent`) existieren **in keinem** der 747 Commits. `static/` trägt nur `static/landing.html` (eine reine Landing-Seite, ohne fieldShader/WebGPU/DataView). Was lebt, ist der Rust-Relay `src/archivar/relay.rs` (955 Zeilen, WS+HTTP, Port 1618, feature `browser_relay`), aber er ist ein **Kopf-Loch**:

- `src/archivar/main_flow.rs` (Asset-Lesepfad) loggt `static/index.html absent — serving 0 bytes` und `static/constants.js absent — browser protocol empty`, übergibt leere `Vec<u8>` an den `TcpRadiator`.
- `relay.rs` (Request-Zeit `resolve_asset`) fällt auf die eingebettete leere Kopie zurück.
- **Ein Browser, der sich heute verbindet, erhält ein leeres Dokument.** Port 1618 + WS-Server existieren; das Auge fehlt.

**TODO-Behauptungen ohne Code-Beleg (rein dokumentarisch):** GRID_TO_ANGLE = 2^62, `windowMedianExtent()`→tanh(Ω·median) (M03), Wheel-Divisor 128 / Touch 512 (M04), `getRto` in constants.js, `color_lut_rgba`-als-GPU-Textur, Bindings 9+12, `EMOscillator`/`hsl_to_rgb`/`temperature_to_rgb` — alle nur in der TODO und in Spezifikations-Texten, **null Code-Vorkommen**.

**Urteil:** Die Abschnitte der TODO zu Browser-Relay / Membran-Rendering / „Farbe-Render-Bein" beschreiben einen toten Zweig. `color_lut_rgba` ist im Rust zwar vorhanden, aber ohne Textur-/Render-Pfad.

**Bereinigungsvorschlag:** Der Browser-Sensor ist ein benannter toter Zweig, kein offener Bau mehr in der Form, in der die TODO ihn führt. Operator-Entscheid: (a) als eigenen Auftrag „Browser-Sensor wiederbeleben — index.html + constants.js + Textur-Pfad neu bauen" registrieren, oder (b) den Zweig als archiviert markieren und die render-spezifischen Membran-Zeilen aus dem offenen Register nehmen. Die TODO-Zeilen, die Dateien adressieren, die es nie gab, sind keine offenen Pflichten.

---

## Klasse 2 — verwaiste/namenlose Code-Funde (einzelne Leichen)

| Fund | Beleg | TODO-Behauptung | Urteil |
|---|---|---|---|
| `color_lut_rgba` (256 Bins) | `src/archivar/spectral.rs:203`; einzige Konsumenten zwei `#[cfg(test)]` (295, 310); Bindings 9+12 existieren nicht | „in der Membran gebacken (Bindings 9+12)" | Orphan — LUT ohne Render-Pfad. |
| `tools/work`-Crate | existiert nie (auch nicht in git-Historie) | „`tools/work/src/bin/smail.rs` ist gebaut" | Pfad-Leiche — smail lebt in `tools/service/src/bin/smail.rs`. |
| `tools/live`-Crate | existiert nie | als grep-Wurzel der Wahrheitsfindung zitiert | Pfad-Leiche. |
| `cd_reconcile` | kein Bin dieses Namens | „Steps 1-2 committet (cd_reconcile, …)" | Namens-Drift oder fehlender Bin — echt vorhanden ist `tools/register/src/bin/cdn_reconcile.rs`. |
| `ionex.rs` Duplikat | `src/archivar/channels.rs:390` `build_ionex_channels` im main_flow-Pfad; `ionex.rs` (`parse_gim`/`tec_at`) nur von 2 Proben | „der Kanal lebt" | Zwei Parser für einen Kanal — die TODO sagt „lebt" (wahr), trägt aber zwei Implementierungen. |

**Nähe-Leichen (einzelne Konsumenten, getragen, kein Total-Leichnam):** `matfile` (nur `crystal_compiler`), `lzw` (nur `gong_series_compiler`), `bison_velocity` (nur `bison_compiler`), `cdf25` (nur `wind_orbit_compiler`). Keines ist unerreichbar; keines ist ein offenes TODO-Versprechen.

---

## Klasse 3 — echte offene Pflicht (TODO ehrlich — Gegenprobe grün)

Die folgende TODO behauptet „gebaut" und **ist gebaut** — hier trägt die TODO keine Leiche, die Gegenprobe bestätigt sie:

- **te.rs-Maschine:** `te_compute` (WGSL `@compute`, DIM 3/ORDER 3), `topological_te_phase`, `transfer_entropy_lag`, `conditional_te_stats(_lagged)`, `residual_surrogate_conditional_lagged`, `ols_fit_lagged`, `solve_linear`, `transfer_entropy_lag_h`, `cycle_phase_shift_surrogate`, `flare_envelope_*`-Gates, `synthetic_dag_recovers_known_direction` — alle vorhanden (`src/mathematikerin/te.rs`).
- **Identitäts-Register:** `FeldIdentitaet`, `magic_identity`, `zeugen_gate`, `serien_gate` — `src/archivar/zeuge.rs`; `phi/witnesses.φ` trägt die Zeugen.
- **gbco-Thread:** `parse_gbco` (`geo.rs`), `gestalt_surface_threads` (`motion.rs`), `load_gestalt_surface_threads` (`main_flow.rs`) — verdrahtet.
- **49 Format-Module** in `archivar/mod.rs` (Zeilen 10–58) — real.
- **Ehrliche PENDING-Marken:** `C_LIGHT` noch nicht konsolidiert (3 Duplikate + `solar.rs`-Literal), `temporal_ring`, `hill_radius_m` nur als Gate (`spatial.rs`), `frame_motion` bei `membrane.rs:212`.
- **Reverse-Check:** kein Format-Modul ist ein Total-Leichnam (alle ≥1 externer Verweis). Voll verdrahtet in den main_flow-Fetch/Pars-Pfad: `bl_narrowband`, `wind_orbit`, `mitdb`, `goes`, `rpw`, `omni2`, `fk`, `netcdf`.

---

## Klasse 4 — CDN-Manifestations-Befunde (gh api gemessen, omegaflow/sources)

Diese TODO-Abschnitte waren lokal nicht als Code prüfbar — sie sind gegen das CDN gemessen:

| Asset | CDN-Host (Release-Tag) | Größe | TODO-Behauptung | Befund |
|---|---|---|---|---|
| `qbo_30hpa.csv` | `cpc.ncep.noaa.gov` | 23.205 B (2026-09-07) | „geschlossen — auf dem CDN" | **wahr** — manifestiert. |
| `d20_thermocline.csv` | `data.pmel.noaa.gov` | 78.899 B (2026-09-07) | „geschlossen — auf dem CDN" | **wahr** — manifestiert. |
| `omni2_serie_1h.bin` | `ssd.jpl.nasa.gov` | 54.379.048 B (2026-09-07) | „CDN-Upload endet 401 … Asset auf dem CDN gemessen absent … pending" | **korrigiert** — das Asset **liegt** auf dem CDN (Upload 2026-09-07T15:11Z, real 54 MB). Der 401-Befund gilt einem früheren Moment; die TODO-Zeile ist veraltet. |
| `gebco_bathymetry.gbco` | `opentopodata.org` | **56 B** (2026-09-07) | „das Release opentopodata.org existiert nicht (404) … Manifestation CI-pending" | **zweifach korrigiert** — (1) das Release **existiert** (Tag `opentopodata.org`), (2) aber das Asset ist ein **56-Byte-Stub**. gbco = Magic(4) + len(4) + 24-B-Records → 56 B = genau 2 Records (lat/lon/elev). Echte Bathymetrie wäre MB. Der gemessene 404-Stand ist überholt; der Stub-Inhalt ist `unverifiziert` gegen die erwartete Messung. |
| `bayestar2019.be19` | `ssd.jpl.nasa.gov` | 2.090.178.751 B (2026-09-06) | „liegt auf dem CDN" | **wahr** — 2,09 GB real. |
| `bidsleep_mehrnacht.bin` | `physionet.org` | 10.263.728 B (2026-09-05) | „manifestiert (verified HTTP 200)" | **wahr** — manifestiert. |

**Bereinigungsvorschlag:** (a) die TODO-Zeile zu omni2 von „401 / absent / pending" auf „manifestiert 2026-09-07" stellen; (b) den gebco-Eintrag von „404 / CI-pending" auf „Release existiert, Asset 56-B-Stub, Inhalt unverifiziert" stellen und den Stub-Inhalt gegen den `gebco_bathymetry_compiler`-Ausgabe nachmessen.

---

## Klasse 5 — Dispatch-/Manifestations-Schulden (keine Code-Leichen)

Die CI-Workflow-Dateien existieren real (`.github/workflows/`, ~100+): `d20-cdn.yml`, `qbo-cdn.yml`, `fmi-gic-cdn.yml`, `iss-lis-cdn.yml`, `vlies-density-cdn.yml`, `amon-cdn.yml`, `auger-cdn.yml`, `gebco-bathymetry-cdn.yml`, `physionet-cdn.yml`, `omni2-cdn.yml`, `kernel-flatten.yml`, `jwst-cdn-watch.yml`, `de441-cdn-watch.yml` u. a. — alle auffindbar. Die TODO-Einträge „Manifestation offen / CI-dispatch-pending / Token-Rotation" sind **echte Zustands-Schulden** (Dispatch nicht ausgeführt, Token fehlt), keine fehlenden Dateien. Das Blatt führt sie als offen, nicht als Leiche.

---

## Bereinigungsvorschlag (gebündelt)

1. **TODO: Browser-Relay / Membran-Rendering / Farbe-Render-Bein** — als toten Zweig benennen (oder als Wiederbelebungs-Auftrag umschreiben); render-spezifische Zeilen, die nie-existente Dateien adressieren, aus dem offenen Register nehmen.
2. **TODO: Pfad-Leichen korrigieren** — `smail` nach `tools/service`; `tools/live` als nie-existente grep-Wurzel ersetzen oder streichen; `cd_reconcile` → `cdn_reconcile`; `ionex`-Duplikat (zwei Parser) benennen.
3. **TODO: CDN-Zeilen auf den gemessenen Stand stellen** — omni2 „manifestiert", gebco „56-B-Stub, Inhalt unverifiziert".
4. **Code: `color_lut_rgba`** — entweder in einen Textur-Pfad heben oder als Orphan benennen; nicht als „gebaut in der Membran" führen.
5. **Kein totaler Modul-Leichnam vorhanden** — die getragenen Einzel-Konsumenten (`matfile`/`lzw`/`bison_velocity`/`cdf25`) sind Nah-Leichen, kein Sofort-Handlungsdruck.

Die Leichen konzentrieren sich auf die Browser/Membran-Ecke; der Rest der TODO ist überwiegend echte offene Pflicht. Kein Fund hier ist eine gelöschte Datei, die noch lebt (Klasse-C-Leiche fehlt) — die historischen Löschungen (Python-Scratch, `probe-sweep.yml`-Cron) sind sauber vollzogen.
