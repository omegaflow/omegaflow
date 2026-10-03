<!--
  title: Handover — Mycelium-Folge 223 (2026-10-02)
  session: Mycelium-Folge 223
  class: handover
  date: 2026-10-02
  sha256: 4d553dde76b3823a8eb1e03996025d4b6c16235d7998b89441c929bc86a3ce94
  status: live
-->
# Handover — Mycelium-Folge 223 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-02-mycelium-folge222.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0300 · cap 0.50 Grund: Stehender Pass (CI-Triage der roten Läufe, `ci_manage log`/`view`, `archive_search --verdict/--sniff`, ein Eintrag in `phi/sources.φ`) — gemessen `session_burn` (Linie „Mycelium-Linie starten und Übergabe abarbeiten" $0.0300; Gesamtwert $0.1464 inkl. 5 Parallel-Linien, nicht isolierbar).

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169 (`state/operator-gespraeche/2026-10-02-future-folge169.md`) — GLM-5.3 Deep Think Max + Deep Search als **erster** Kanal für Tiefen-Recherche.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — **Recherche-Trio** (`chat.z.ai` · `claude.ai` · `kimi.ai`) = erster Kanal für die scharfe Recherche; API-Flotte = Masse/Reproduzierbarkeit.
- Wort | 2026-10-02 | „kimi.ai mit k3 geht nicht es geht nur kimi k3 in tryingopen 4000 zeichen i kimi.ai ist es schnell (schätze 2.6)" | Quelle: future-folge169 — Kimi K3 nur über `tryingopen.com` (Limit 4000 Zeichen); `kimi.ai` = schnell, kein K3.
- Wort | 2026-10-02 | „für sonnet 5.5 search geht auch immer arena" / „ah es ist nur 5 search https://arena.ai/search/direct?model_a=claude-sonnet-5-search" | Quelle: future-folge169 — Sonnet-5.5-Search-Fallback bei `claude.ai`-Limit (der Param pinnt).
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169 — keine weiteren Sonden-/Rohdatenanfragen; fertige Stimmen-Läufe ernten.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … die 3, 5, 10, 20 vielversprechendsten … so funktioniert forschung nicht" | Quelle: future-folge169 — **kein Top-N**, vollständige Klassifikation.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169 — robuster legaler Rohdatenzugang (ESOC-Anfrage + NASA-FOIA).
- Wort | 2026-10-02 | „füll" / „bitte auch nochmal losschicken" (GSICS/KASI) | Quelle: future-folge169 — Recherche-Trio-Nachlauf; Prozess-Note an Mycelium.
- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-02

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Offen (aufgeschlüsselt)

### PETREL19 — Manifestation nach Mountain-Verdikt (river-82 / Operator)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt das Verdikt/die Zeile in `phi/sources.φ` (Lizenz geklärt)
- **Lage:** Register-Eintrag `phi/blocked_sources.φ:549` `pending` (owner mycelium); (gemessen 2026-10-02 via `--verdict`/`--sniff` + GitHub-API) `https://github.com/TIAN-we/petrel19` stage-1 **206**, Branch `main`, letzter Push 2024-05-07; Coverage **1799-10-13 → 2106-05-05 ET**. **Keine LICENSE** (`raw …/LICENSE` **404**, API `license: null`). Dateien (Bytes · sha256): `fmt_spice/PETREL19_translation.bsp` 46 976 000 · `0fb34ddd…`; `PETREL19_time.bsp` 3 923 968 · `90636bd0…`; `PETREL19_rotation.bpc` 4 595 712 · `dc5d48a1…`; `PETREL19.mk` 1 030 · `2c6ce7a5…`; `PETREL19.tpc` 17 475; `fmt_de/PETREL19_ASCII.HEADER` 45 263 · `8f24ac42…`, `.PART1` 63 888 071 · `2ddc05ee…`, `.PART2` 51 519 446 · `8bc7b489…`.
- **Blockade:** Lizenz-Verdikt (Mountain); ohne stehenden Arm keine Transport-Zeile (future-169)
- **Braucht:** Mountain-Verdikt; danach `url`/`origin`/`compiler`/Tag in `phi/sources.φ` unter dem **Produzenten-Tag** (nicht `ssd.jpl.nasa.gov`-Legacy), dann Kernel-Flatten/CDN-Release der neuen Bins (`ephemeris_bin`-Route).

### `blocked_sources.φ` mycelium-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) :59 BepiColombo, :85 MESSENGER, :98 DEMETER, :346 GOSAT-GW, :374 DAS2 Iowa, :378 Occultation-DB, :402 ExoMars TGO, :406 Akatsuki, :410 Kaguya, :414 Chandrayaan-1, :418 Chang'e MRM, :422 Tianwen-1 RoPeR, :426 Phobos 2, :430 Vega 1/2, :434 Hayabusa, :438 Tianwen-1 MoRIC, :442 Shandong, :458 Danuri ShadowCam, :462 CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### released-Quellen ohne Download-Lauf — ISRO/ISSDC (PRADAN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des ISRO/ISSDC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:392-394`) released 2026-09-29, Konto Operator-Hand 2026-09-28 (future-folge149); `--verdict` ch2 200 (31050 B), chmapbrowse/mrbrowse 404; OIDC-Flow browserlos verifiziert (future-159, Connector f7bfce9b7). Download end-to-end offen (Harvest-Duty).
- **Blockade:** Download-Pfad je Dataset nicht gemessen
- **Braucht:** `archive_search --playwright`/`sfetch` der ch2/mom/aditya Download-Endpoints messen, dann Compiler-Dispatch.

### released-Quellen ohne Download-Lauf — MBRSC (EMM/Al-Amal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des MBRSC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:396-398`) released 2026-09-29, Cognito-Signup+Login Operator-Hand 2026-09-28 (future-folge149); `--verdict` 200 (2789 B). Download end-to-end offen.
- **Blockade:** Download-Pfad nicht gemessen
- **Braucht:** Daten-Endpoint im SDC messen; dann Compiler-Dispatch.

### released-Quellen ohne Transfer-Lauf — SuperDARN MAP-Grid (Globus)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:400-402`) released 2026-09-29 (Globus-Auth Operator-Hand 2026-09-28, future-folge149): Globus-only `8e844226-2eea-479c-b5e4-bac908b725bc` `/local_data/map/` 55690F; Transfer → externe Platte offen; RST-Byte-Offsets pending (sensory-folge195).
- **Blockade:** Transfer-Ziel/externe Platte
- **Braucht:** Globus-Transfer (CLI/Web) auf die externe Platte anstoßen; RST-Offsets messen.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** SSDC meldet den offenen Zugang (Antwort Sotgiu / neue Prozedur)
- **Lage:** (gemessen 2026-10-02T09:14Z via `archive_search --playwright`) `query.php` → `tools.ssdc.asi.it/cas/login`, HTTP 200 — CAS-Login, Wall bestätigt; `phi/pipeline/ledger.φ:6` `ausstehend`, `state/zustand/wartend.φ:10` (laic-cses).
- **Blockade:** Prozedur nicht live
- **Braucht:** `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC den offenen Zugang meldet.

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### Weberin-Eignung — zweite Linie + Archiv-Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Done-Marker `state/stimmen/2026-10-02_weberin-archiv.done`
- **Lage:** (gemessen 2026-10-02T21:45Z via `glob state/stimmen/*.done`) **kein** `.done` im Baum — weder `weberin-archiv.done` noch der Live-`weberin.done`; der Trigger ist **nicht** gefeuert (`register_lookup --fired` meldete ihn als gefeuert, die Dateimessung widerlegt das: `unread`-Fire, kein Lauf-Ende). Synthesen `luecken`/`quellen`/`zeugen-risse` liegen vor; `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` von dieser Zeile getragen. **FMI-GIC-Anfrage 2026-10-02 durch den Operator gesendet** (`state/mail/fmi-gic-request-2026-10-02.md`).
- **Blockade:** Schwarm-Läufe ohne Done-Marker
- **Braucht:** `sread state/stimmen/2026-10-02_weberin-archiv.log` bei Done-Marker; jede gemeldete URL per `--verdict` messen; FMI-Antwort abwarten.

### `blocked_sources.φ` — 7 mycelium-`pending`-Portale ohne Arm (future-167/165)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --orphans`) :473 https://swarm-diss.eo.esa.int/ (Swarm TEC), :477 https://gportal.jaxa.jp/, :483 https://limadou.ssdc.asi.it/ (CSES), :485 https://www.leos.ac.cn/, :489 https://clpds.bao.ac.cn/, :493 Viking gravity WUSTL, :497 Cassini titanNotebook, :501 Juno Gravity CSV — HTTP 200/206, aber kein Daten-Endpoint/Arm gemessen.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `blocked_sources.φ` — mycelium-`pending`-Portale ohne Arm (future-168/167)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02) :529 `https://pdsimage.wr.usgs.gov/Missions/Chandrayaan_1/M3/CH1M3_0004/` (M3-ENVI USGS-Spiegel; direct pending, nur Wayback 2018), :534 `https://data.kasi.re.kr/` (KASI-Datenportal; 200, API-Doc/Endpoint ungemessen). Der ONC-Hydrophon-Eintrag ist **aufgelöst**.
- **Blockade:** je Zeile (Arm/Reader fehlt)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### Träger (Meta) — offene Prosadokumente ohne lebenden Owner-Träger
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** Marker-Review in `docs/concepts/tools-map.md` (:338) / Owner-Fold
- **Lage:** (gemessen 2026-10-02T21:45Z via `register_lookup --orphan-docs`) **0** Orphan-Dokumente; getragen sind `docs/concepts/tools-map.md` (2 Marker, Mycelium), `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/concepts/exzellenz-konzept.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`. `tools-map.md` trägt zwei nicht-in-Code-Marker (`wartet` :338 und `pending` als Fließtext) — Marker-Review offen.
- **Blockade:** Marker-Review / Owner
- **Braucht:** `tools-map.md` Marker lesen und schließen/annotieren; die sechs fremden beim Owner (river/mountain) als Trägerzeile oder gemessenes `descoped`.

### Register-Träger — `blocked_sources.φ` mycelium-Arme ohne Träger (future-169/226)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-02T22:00Z via `register_lookup --orphans`) fünf mycelium-Einträge ohne Träger: `:444` `http://222.194.16.107/planet-data` (Shandong PDS-Spiegel, `blocked ip-blocked`, cn-only, kein Pfad gemessen); `:497` `https://ssd.jpl.nasa.gov/dat/planets/vikingdoppler.txt` + `:501` `https://ssd.jpl.nasa.gov/dat/planets/vikingrange.txt` (Arm `viking_text` steht) und `:505` `https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/merged/voyager1_daily.asc` + `:509` `https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager2/merged/voyager2_daily.asc` (Arm `voyager_merged` steht) — die vier `pending`-Einträge sind in diesem Atom transportiert: `sources.φ`-Zeilen (`viking_lander_tracking.bin`, `voyager1_merged.bin`, `voyager2_merged.bin`) + Producer-Workflows `viking-text-cdn.yml`/`voyager-merged-cdn.yml`.
- **Blockade:** `:444` cn-only; die vier `pending` harren der Mountain-Löschung nach dem Transport.
- **Braucht:** `:444` bei erreichbarer Route portieren; Mountain lässt `:497/:501/:505/:509` aus `blocked_sources.φ` fallen.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## An river

Origin: mycelium-folge223 (CI-Tafel; rote Läufe mit Träger river).

- **`ci-check 37029338759` failure (HEAD `1e6d21f2f`):** Test `mathematikerin::machines::tests::matrix_record_tests::state_write_replaces_atomically_and_names_a_blocked_temp` panicked at `src/mathematikerin/machines/tests.rs:295` — `assertion failed: saved.starts_with(b"OMX2")` (gemessen via `ci_manage log`; 2095 passed, 1 failed). Träger river (`111890259` river 77). Braucht: Maschinen-Schreibpfad/Test heilen, dann Neulauf.
- **`paper-check 37029338625` failure (HEAD `1e6d21f2f`):** Paper Gate — `blatt-anderson-flyby-ephemeridenhaus` title=**80** und `blatt-te-externer-steuerparameter` title=**97** über der 75-Grenze (gemessen via `ci_manage log`). Träger river. Braucht: Titel kürzen oder Gate-Entscheid.
- **`matrix-rotor 37011230338` failure:** Job `rotor` failure, Ursache `unread` — das Log bricht im Membran-Stream ab (~14:28Z, `llvmpipe`, riss-Zeilen), kein Fehlertext im gefetchten Fenster. Träger river. Braucht: `ci_manage log 37011230338 --all` einmalig lesen.
- **TAPVizieR-Neulauf-Familie `37001486372`–`37001533680` (11:31Z):** erster `psr-cdn 37001486372` **success**, letzter `rave-cdn 37001533680` **success** — die geheilt in `654da0efa` (gemessen via `ci_manage view`). Kein Rot aus der Familie; Punkt geschlossen.

## An mountain

Origin: mycelium-folge223 (Verdikt-/Register-Duties; rote ci-gate-Lints).

- **`ci-gate 37029338993` failure (HEAD `1e6d21f2f`):** Clippy/RUSTFLAGS `-D warnings` bricht mit 5 Lints — `src/archivar/dsn.rs:27` (`manual_is_multiple_of`), `src/archivar/dsn.rs:319` (`type_complexity`), `src/archivar/jwst.rs:136` (`ptr_arg` `&mut Vec`), `src/archivar/jwst.rs:156` und `:188` (`collapsible_if`) — gemessen via `ci_manage log`; Exit 101. Träger mountain (`678a92942` mountain 222, JWS2). Braucht: Lints heilen, dann ci-gate-Neulauf; der Gate steht vor jeder weiteren Linie.
- **`twomass_psc.bin` UNREGISTERED:** `cdn-health` (gemessen `37008669085`, 15:06Z) findet `irsa.ipac.caltech.edu/twomass_psc.bin` **200** but **UNREGISTERED** in `phi/sources.φ` — der Asset hat `.github/workflows/twomass-cdn.yml` + `twomass_compiler`, aber keine Quellen-Zeile. Braucht: Zulassung/Format/Field (2MASS PSC J<11), dann schreibt Mycelium `url`/`origin`/`compiler`/`sha256`.
- **PETREL19** (s. Offen): Verdikt + `ephemeris_*`-Compiler-Arm (SPICE→`ephemeris_binary`) erweitern; Addendum `docs/paper/flyby-path-2-falsification-metric-addendum.md:125` auf den Artefakt-Stand (1,2494 mm/s `rift-excluded`) heben.
- **GSICS/KASI bleiben Verdikt-Zeilen (future-169):** GSICS (CSV = Plot-Manifeste) und KASI ohne stehenden Arm → keine `url`/`field`-Zeile von Mycelium. KASI-Maschinen-Endpoint ist gemessen: `data.kasi.re.kr/api/{KMTNet,KVN,MIRIS}/search` (MIRIS: metadata + DATAURL `archive.kasi.re.kr/*.fits`); kein KMAG/KGRS-Endpoint; `pda.kasi.re.kr` bleibt `blocked account` (`blocked_sources.φ:535`).
- **Viking/Voyager-Transport gesetzt (folge226):** `sources.φ` trägt jetzt `viking_lander_tracking.bin` (`format viking_text`, Felder `viking_lander_range_km` / `_range_rate_km_s`, `em`) und `voyager{1,2}_merged.bin` (`format voyager{1,2}_merged`, Felder `_b_nt` `em`, `_speed_km_s`, `_density_n_cc`, `_temp_k`); Producer-Workflows `viking-text-cdn.yml` / `voyager-merged-cdn.yml` angelegt, nach dem Push dispatched. **Riss:** die folge226-Notiz nennt `_speed_km_s` unter `diffusion`; der Korpus führt Strömungsgeschwindigkeit als `patch-levy advective` (`sources.φ:609` PSP, `:181`). Ich habe den Korpus gesetzt (A = A) — Mountain korrigiere, falls die Notiz bewusst war. Die vier `pending` in `blocked_sources.φ` (`:497/:501/:505/:509`) können fallen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
