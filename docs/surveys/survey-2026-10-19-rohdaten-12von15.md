<!--
  title: Rohdaten-Inventar 12/15 (2026-10-19)
  class: survey
  date: 2026-10-19
  status: done
-->
# Rohdaten-Inventar — 12 von 15 (Stand 2026-10-19)

Erarbeitet in der Sensory-Session: je Datensatz ein belegter Header (Datei, Spec oder
Abstract), dann Rohdaten-Jagd über `archive_search --all` plus gezielte Arme
(`--github`, `--sourcegraph`, `--erddap`, `--nssdc`, `--zenodo`, `--exa`).

**Belegregel:** Ein Treffer zählt nur, wenn die Datei geladen und die Kopfzeile wörtlich
im Byte-Stream geprüft wurde (byte-identisch, nicht „ähnliche Spalten"). Hashes in dieser
Datei sind in der Session selbst berechnet, außer wo „Agent" steht.

| # | Datensatz | Origin | Format | Größe | sha256 | Beleg |
|---|---|---|---|---|---|---|
| 1 | emodnet-hfr | `erddap.hfrnode.eu` `EUHFR_NRTcurrent_HFR-GoS-Total_v3.csv` (Stichprobe; 98+57 Datasets) | CSV | 138 | `13fbfc0f2aaa41a1347fe6b7bc5d7d038c4b45f697b60b2bc8145bce750afed4` | Kopf `time,depth,latitude,longitude,EWCT,NSCT` + Einheitenzeile |
| 2 | noirlab-gaia-dr4 | `raw.githubusercontent.com/sefffal/Octofitter.jl/main/docs/src/gaia4_epoch_astrometry.csv` | CSV | 133502 | `34ab4046b2d26a554da1bfb7ab160991bb9f8967f8c427de52217f17428018f9` | 10-Feld-Kopf byte-identisch |
| 2b | noirlab-gaia-dr4 | `…/gaia_bh3_epoch_astrometry.csv` | CSV | 79311 | `532a4a19b1c5eb16fb45d2de875e84fd1bdeeb70d8fa18228c446105f4ab6d2b` | dito |
| 2c | noirlab-gaia-dr4 | `…/hd114762_epoch_astrometry.csv` | CSV | 109551 | `fcbbd20947cb44043d9a2475010c0d9270fa3191c6138e5870dc5f787462a671` | dito |
| 2d | noirlab-gaia-dr4 | `anonftp.cosmos.esa.int/…/gaia-dr4-prerelease-epoch-astrometry_2026-06-26.zip` | ZIP/VOTable | 625651 | (Agent) | amtlich; Zeitspalte dort `obs_time_tcb`, nicht `epoch` |
| 3 | schumann-serie | `zenodo.org/api/records/15225820/files/NiNo_SR_params_2024.csv` | CSV | 454417 | `3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272` | = Repo-Hash; 7 835 Zeilen; **Lizenzwiderspruch** (CC-BY-4.0 vs. „only with the authors' permission") |
| 4 | voyager-uranus-neptun | `spdf…/voyager2/merged/vy2_1986.asc` | ASCII | 2820720 | `5f2cfbf761d0a6f413d62bc6eb3a32a7eea2ef7b1e154073e454351eef4504c0` | Uranus-Tag 24 geprüft |
| 4b | voyager-uranus-neptun | `spdf…/voyager2/merged/vy2_1989.asc` | ASCII | 2820720 | `bd82a73885a3adbec0ed808a35d912457fa36017bbacddce440baa6fce086b2d` | Neptun-Tag 230 geprüft; Mirror `nssdcftp` byte-gleich |
| 5 | robinson-camera-angles | `ser.im-ldi.com/M10/TXT/camera_angles.txt` (auch `/M10/M10_ISIS/`) | ASCII-Labels | 194640 | `0a65df59c8685e22004ce7f7afa21cf54f7aedc39c2ecb5965b8fcad3dae97bd` | 811 Records; 2 Wayback-Snapshots byte-gleich; **Mariner 10, nicht LROC** |
| 6 | ceic-d18000 | `zenodo.org/api/records/7112529/files/datasets.rar` | RAR | 1019033 | `d8f0e3e828cd5a3cc6c315f39bbabd02a41c15bdd738506f44a90f1a544287fe` | 21 Member; `dc201008` = `530a1439f7522f09b140548c1899853b8359b4da2faa0d90f9862f3c3d8be1fc`; Zwilling 7855090 byte-gleich |
| 7 | pds-venus-radar-raw | `pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/data/` | PDS4-IMG | 15 × 960000000 | (nicht geladen; HEAD+Range) | `<elements>12000/20000`, `IEEE754LSBSingle`, `start_date_time` |
| 8 | ctbto-vdec | `isc.ac.uk/cgi-bin/web-db-run?…out_format=ISF` | ISF/IMS1.0 | 394300 | `492bcdfa4d000c61b7869d56f480164501cb26569205115f171b8fd14090f45f` | Kopf `Azim … Slow … SNR Amp Per`; **6/7 Felder** (`duration` fehlt in ISF1.0/2.1) |
| 9 | waldo-massdaten | `zenodo.org/api/records/3975788/files/AH180724000000NWC_001A.mat` | MAT-v4 | 93690 | `bef846095cf35fca0815732e3b004dddd296388edb1f8c902e3123b92c1395e7` | 30 Variablen, `call_sign NWC`, `Fc 19800.0`; amtliche Quelle account-gated, genutzt wurde die CC-BY-Kopie |
| 10 | juice-recon | `spiftp.esac.esa.int/…/juice_orbc_000116_230414_310721_v01.bsp` | SPK | 15254528 | `e7a20a60fc53f6ce696caada4655418f3494ce0bb4cc24fd98be36f8a49bf8dd` | DAF/SPK; OEM `ORBC__000116.jui` 67 684 026 B; **1σ-Kovarianz fehlt** |
| 11 | europa-clipper | `naif.jpl.nasa.gov/pub/naif/EUROPACLIPPER/kernels/spk/trj_260808-261230-dco2610050829-EGACOLA0-FINAL-OD093-v1.bsp` | SPK | 3205120 | `1727f61f1f7786bf277745c61b04e4dc4f82a2331276eb112c52379d189126e4` | 4 Segmente, target −159, center 10, frame 1; Flyby 2026-12-03 im 3. Segment |
| 12 | m10-rohtracking | `spdf…/mariner10/…/red_tele_signal_data_venus_occlt/` (9 Tars) | NSSD1346 | 19611592 (1 Datei) | `ffe5a48bde10cc6841cd8cea31bb2a561e7f03b9512cd8f125b60ebba0046667` | 4 774 × 4 108 B; Layout in `src/archivar/mariner_occlt.rs`; Zweiweg-Tracking (`PSCM-00009`) offline |

## Venus-Korrektur: `venus_radar_level2.bin` ist ein Stub

Das Register führt den Datensatz (`phi/sources.φ:19120`, `terms CC0-1.0`, Felder
`venus_radar_inc_deg` / `_ocp` / `_scp`), aber der CDN-Asset ist:

```
308 B, Magic VNRD, sha256 863962a6e30a43614fb687bc75e7562ebaf962fa96597065d04b583121defe5a
```

Also **registriert, nicht geerntet** — 15 × 960 MB stehen auf PDS, der Workflow trägt
`asset=fehlt`, und seine Idempotenz-Prüfung würde wegen des Stubs überspringen.
Kein Sensory-Akt: Harvest ist Mycelium.

## Nicht abgedeckt (3)

- **bepicolombo-more** — PSA hat 95 712 MORE-Produkte, alle `proprietary_end_date`
  2088–2099; `retrieval_type=PRODUCT` → HTTP 403. Kein offener Ersatz.
- **hreda-rohspektren** — HREDA `data_available=false` für DOSIS_3D / EuTEF_DOSTEL /
  EXPOSE-E/R; nur passive TL/OSL-Dosis (IFJ PAN, `D,mGy`).
- **cindi-nwm** — NWM nie offen publiziert (`nwm_data.html`: „we will not be presenting
  any NWM data here"); offen ist nur **IVM** (CDF + HAPI), klar benannter Ersatz.

## Lehren

1. **Header sind versioniert.** WALDO hat zwei Ordnungen (alt `start_year … filter_taps`,
   neu/AWESOME `VERSION, station_name, …`), ACE `BGSM` vs OMNI `BZ_GSM`. Suchköder müssen
   beide Namenssätze tragen.
2. **MAT-v4 ist eine Variablenfolge, kein Struct.** 20-Byte-Kopf (`type,mrows,ncols,imagf,namelen`),
   dann Name (namelen Byte, **ohne** 4-Byte-Padding), dann Daten. `Fc` = 19800.0 steht ab
   Byte 994 im NWC-File.
3. **`archive_search --all` prüft keine Reihenfolge** — es ist Token-/Metadaten-Suche.
   Reihenfolge belegt nur die Datei oder das Spec-Dokument; seltene Header treffen
   (`EWCT NSCT`, `centroid_pos_error_al`), häufige ertrinken (`BR BT BN`).
4. **Der billigste Mistral (`ministral-3b`) erfindet Belege** — in keinem von 15 Läufen ein
   spill-belegter Spaltenkopf, dafür falsche Pfade/DOIs. Nicht wiederverwenden.

## Provenienz-Hinweis zu Commit `97cdbaaa6`

Der Commit wurde mit vollem Index abgesetzt und trägt deshalb zusätzlich fremde,
bereits gestagte Hunks:

- `.github/workflows/timed-orbit-cdn.yml`
- `phi/sources.φ`
- `state/rat/2026-10-18-mycelium-pass-kids-transport.md`
- `tools/harvest/src/bin/hmi_compiler.rs`
- `tools/measure/src/bin/field_te_query.rs`

Inhalte unverändert, nur die Commit-Message ist falsch. Kein Rewrite/Force-Push auf
dem geteilten `main`. Regel ab jetzt: `git diff --cached --stat` vor jedem Commit und
isolierter Index (`GIT_INDEX_FILE=… git read-tree HEAD`) für eigene Commits.
