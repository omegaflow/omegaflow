<!--
  title: Handover — Mycelium-Folge 225 (2026-10-03)
  session: Mycelium-Linie in einem Pass — Cap-Gate gebaut, zwei Cap-Bypass-Writer rotiert
  class: handover
  date: 2026-10-03
  sha256: f2bbdc72051ba8b3df6fabbc848a9ad8a88c2aeb650c96814a1bf1e9b08b6d2b
  status: live
-->
# Handover — Mycelium-Folge 225 (2026-10-03)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-03-mycelium-folge224.md` (→ `archiv/`).

## Burn: open 0.0017 · close 0.0543 (gemessen `session_burn`; Grund: Cap-Gate `cdn_reconcile --fail` + nvss/gaia/omni2-Rotation + Handover)

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

## Haus (die vier Orte) — gemessen 2026-10-03

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Offen (aufgeschlüsselt)

### Cap-Bypass-Gate gebaut, zwei Writer rotiert (Step-5(b), Teil 1)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push dieses Atom → `gh workflow run nvss-cdn.yml` / `gh workflow run gaia-cdn.yml`
- **Lage:** (gemessen 2026-10-03) `cdn_reconcile --fail` gebaut (`tools/register/src/bin/cdn_reconcile.rs`): scans `.github/workflows/*.yml` für `gh release upload <CAPPED>` und `--release-tag <CAPPED>`, exit 2 bei Drift; Unit-Test `no_workflow_writes_the_capped_release`; ci-check-Step `cargo run -q -p omegaflow-register --release --bin cdn_reconcile -- --fail` (`ci-check.yml:52`). Rotiert: `nvss-cdn.yml:46` → `gh release upload ssd.jpl.nasa.gov-nvss`; `gaia-cdn.yml:36` → `--release-tag ssd.jpl.nasa.gov-gaia`; `omni2-cdn.yml:45` Idempotenz-Read → `cdaweb.gsfc.nasa.gov` (Compiler-Tag). Lokal gemessen: `./target/debug/cdn_reconcile --fail` → `cap contract clean`, exit 0.
- **Blockade:** die zwei Family-Tags existieren erst nach dem nächsten Lauf; darum noch **kein** Registry-Rebind (kein Fenster ohne Asset).
- **Braucht:** nach Push `gh workflow run nvss-cdn.yml` + `gh workflow run gaia-cdn.yml`; nach 206 je Asset die Register-`url`s rebinden (`phi/sources.φ:10541` nvss → `ssd.jpl.nasa.gov-nvss/`, `:10512` dr3_stars → `ssd.jpl.nasa.gov-gaia/`) und die Altwriter verlieren.

### Step-5(b), Teil 2 — Register↔Workflow-Tag-Diff (offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `nvss-cdn`/`gaia-cdn` endet mit 206 auf `ssd.jpl.nasa.gov-nvss`/`-gaia`
- **Lage:** (gemessen 2026-10-03) der zweite Teil des Rats-Entscheids — `cdn_reconcile --fail` soll die erwartete Tag-Menge aus `phi/sources.φ` gegen die `gh release create/upload`-Tags in `.github/workflows` diffen (dynamische Familien modis-Jahr/ps1-Slabs/spk-Shards exempt). Noch nicht gebaut: der literale Scan sieht nur feste Tags; die dynamischen (`$tag`/matrix) bleiben exempt, brauchen also die §1-Klassen-Konstanten. Der Cap-Teil (Teil 1) steht.
- **Blockade:** die Tag-Diff braucht die 13-Netloc-Familien-Bindung (multi-Atom).
- **Braucht:** `cap_contract` um den erwarteten Registry-Tag-Satz erweitern (`cdn_tag_from_url` über die `url`-Zeilen), Drift melden; vorher Familien-Identität je `*-cdn.yml` an `sources.φ` binden.

### viking-text-cdn — Capped-Release-Fix, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `viking-text-cdn` (`37116102274`) liest aus (queued seit 2026-10-03T10:20Z, head 4a62985a7)
- **Lage:** (gemessen 2026-10-03T03:45Z via `ci_manage log 37093353915`) Lauf **failure**: `release ssd.jpl.nasa.gov is capped (1000 assets)`; Fix: `viking_text_compiler.rs` → `upload_release("ssd.jpl.nasa.gov-planets", OUT)`; `phi/sources.φ:16178` auf `…/ssd.jpl.nasa.gov-planets/viking_lander_tracking.bin`. Dispatched 2026-10-03T10:20Z, noch queued.
- **Blockade:** keiner
- **Braucht:** `ci_manage view/log 37116102274` einmalig; dann `archive_search --verdict …/ssd.jpl.nasa.gov-planets/viking_lander_tracking.bin` (206 erwartet) und `sha256` in `phi/sources.φ` nachtragen; die vier `pending` `blocked_sources.φ:497/501/505/509` fallen (Mountain).

### nvss-cdn — RA-chunked geheilt, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `nvss-cdn` (`37116107265`) liest aus (pending seit 2026-10-03T10:20Z, head 4a62985a7)
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37069595693`/`36989806823` + `--verdict`) beide früheren Läufe **failure** (async `phase ERROR`); `nvss.json` liegt auf `ssd.jpl.nasa.gov` (**206**, Altwriter). `nvss-cdn.yml` fährt 8 RA-Slices + `jq -s add`; der Upload ist in diesem Atom auf `ssd.jpl.nasa.gov-nvss` rotiert.
- **Blockade:** keiner
- **Braucht:** `ci_manage log 37116107265` einmalig; danach Register-Rebind (s. o.).

### M3-Asset — Register live, CDN-Präsenz absent (CI-403)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** runner-erreichbare M3-Route (Mountain) oder ein Mirror — Beleg: `pds-imaging.jpl.nasa.gov` 403 im CI-Lauf (2026-10-03)
- **Lage:** (gemessen 2026-10-03) `phi/sources.φ:9871` `pds3_img_m3g20081118t222604_v03_loc.bin` (`format pds3_img`, sha `5771de9891015b2ee50c436fbbfe7e87f6c5a15332d7d0505e3792505d1851c4`, 8758832 B, roundtrip) gesetzt; CDN-`--verdict` **404**. CI-Runner erhält 403 (Datacenter-IP; Proton ebenfalls 403); `pds3-img-cdn.yml` lässt M3 fallen.
- **Blockade:** runner-erreichbare Fetch-Route (Arm/Source)
- **Braucht:** Mountain misst eine runner-erreichbare M3-Route; dann `gh workflow run pds3-img-cdn.yml`.

### KPLO/KARI `blocked_sources.φ:456` — offener Datenpfad
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächste Messung des Verzeichnisses — Beleg: `phi/blocked_sources.φ:456` direct 200 (2026-10-03)
- **Lage:** (gemessen 2026-10-03) `https://www.kari.re.kr/kpds/published/KPLO/KPLO/PublicRelease/` direct 200 (2146 B), anonymes Verzeichnis (POLCAM/LUTI/KMAG/KGRS + SPICE). Offener Datenpfad.
- **Blockade:** Daten-Endpoint je Datei nicht gemessen
- **Braucht:** Verzeichnis per `archive_search --verdict`/`--playground` je Datei messen, dann Compiler-Dispatch.

### RoPeR-Familie — Sample registriert, Manifest + Familien-Register offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `roper-cdn` (`37116100285`) liest aus (queued seit 2026-10-03T10:20Z, head 4a62985a7)
- **Lage:** (gemessen 2026-10-03) `phi/sources.φ:11085` `gras_2c_roper_hx1-ro_gras_roper-hf-hv_sci_n_20210630005145_20210630005917_00046_a.bin` (`format gras_2c`, `origin` Zenodo-Record 15812343, compiler `roper_pds4_compiler.rs`, sha `d4b7397a…d6fd`) gesetzt; der Record trägt 42 `.2C`/`.2CL`-Paare. `roper-cdn.yml` angelegt.
- **Blockade:** keiner
- **Braucht:** `ci_manage log 37116100285` einmalig; dann die 42 Familien-Registerzeilen aus den Compiler-Ausgaben (Asset · sha256) nachtragen.

### vlies-density — Familien-Tag latent (nicht rot)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `vlies-density-cdn` (`37116105661`) liest aus (queued seit 2026-10-03T10:20Z)
- **Lage:** (gemessen 2026-10-03 via `sgrep`) `vlies_density_compiler.rs:348` lud auf den gekappten `ssd.jpl.nasa.gov`; Fix: Compiler → `ssd.jpl.nasa.gov-vlies`, Workflow zieht aus `ssd.jpl.nasa.gov-vlies`. Kein `sources.φ`-Eintrag (abgeleitetes CDN-Asset).
- **Blockade:** keiner
- **Braucht:** `ci_manage log 37116105661` einmalig; prüfen, dass `vlies_density.vlde` unter `ssd.jpl.nasa.gov-vlies` 206 liefert.

### PETREL19 — Manifestation nach Mountain-Verdikt (river-82 / Operator)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt das Verdikt/die Zeile in `phi/sources.φ` (Lizenz geklärt)
- **Lage:** (gemessen 2026-10-02 via `--verdict`/`--sniff` + GitHub-API) Register-Eintrag `phi/blocked_sources.φ:549` `pending` (owner mycelium); `https://github.com/TIAN-we/petrel19` stage-1 **206**, letzter Push 2024-05-07; Coverage **1799-10-13 → 2106-05-05 ET**. **Keine LICENSE** (`raw …/LICENSE` **404**, API `license: null`). Dateien (Bytes · sha256): `fmt_spice/PETREL19_translation.bsp` 46 976 000 · `0fb34ddd…`; `PETREL19_time.bsp` 3 923 968 · `90636bd0…`; `PETREL19_rotation.bpc` 4 595 712 · `dc5d48a1…`; `PETREL19.mk` 1 030 · `2c6ce7a5…`; `PETREL19.tpc` 17 475; `fmt_de/PETREL19_ASCII.HEADER` 45 263 · `8f24ac42…`, `.PART1` 63 888 071 · `2ddc05ee…`, `.PART2` 51 519 446 · `8bc7b489…`.
- **Blockade:** Lizenz-Verdikt (Mountain); ohne stehenden Arm keine Transport-Zeile (future-169)
- **Braucht:** Mountain-Verdikt; danach `url`/`origin`/`compiler`/Tag in `phi/sources.φ` unter dem **Produzenten-Tag**, dann Kernel-Flatten/CDN-Release (`ephemeris_bin`-Route).

### `blocked_sources.φ` mycelium-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) :59 BepiColombo, :85 MESSENGER, :98 DEMETER, :346 GOSAT-GW, :374 DAS2 Iowa, :378 Occultation-DB, :402 ExoMars TGO, :406 Akatsuki, :410 Kaguya, :414 Chandrayaan-1, :418 Chang'e MRM, :422 Tianwen-1 RoPeR, :426 Phobos 2, :430 Vega 1/2, :434 Hayabusa, :438 Tianwen-1 MoRIC, :442 Shandong, :458 Danuri ShadowCam, :462 CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### released-Quellen ohne Download-Lauf — ISRO/ISSDC (PRADAN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des ISRO/ISSDC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:392-394`) released 2026-09-29, Konto Operator-Hand 2026-09-28 (future-folge149); `--verdict` ch2 200 (31050 B), chmapbrowse/mrbrowse 404; OIDC-Flow browserlos verifiziert (future-159, Connector f7bfce9b7). Download end-to-end offen.
- **Blockade:** Download-Pfad je Dataset nicht gemessen
- **Braucht:** `archive_search --playwright`/`sfetch` der ch2/mom/aditya Download-Endpoints messen, dann Compiler-Dispatch.

### released-Quellen ohne Download-Lauf — MBRSC (EMM/Al-Amal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Lauf des MBRSC-Endpoints
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:396-398`) released 2026-09-29, Cognito-Signup+Login Operator-Hand 2026-09-28 (future-folge149); `--verdict` 200 (2789 B). Download end-to-end offen.
- **Blockade:** Daten-Endpoint im SDC nicht gemessen
- **Braucht:** Daten-Endpoint messen; dann Compiler-Dispatch.

### released-Quellen ohne Transfer-Lauf — SuperDARN MAP-Grid (Globus)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer-Lauf
- **Lage:** (gemessen 2026-10-02 via `phi/blocked_sources.φ:400-402`) released 2026-09-29 (Globus-Auth Operator-Hand 2026-09-28): Globus-only `8e844226-2eea-479c-b5e4-bac908b725bc` `/local_data/map/` 55690F; Transfer → externe Platte offen; RST-Byte-Offsets pending (sensory-folge195).
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
- **Lage:** (gemessen 2026-10-03T~08:40Z via `glob state/stimmen/*`) **kein** `.done` im Baum — nur die `autolauf*.sh`; `register_lookup --fired` meldet den Trigger als gefeuert, die Dateimessung widerlegt das (`unread`-Fire). Synthesen `luecken`/`quellen`/`zeugen-risse` liegen vor; `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` getragen. **FMI-GIC-Anfrage 2026-10-02 durch den Operator gesendet** (`state/mail/fmi-gic-request-2026-10-02.md`).
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

### Registry↔CDN-Reconciliation (Step 5, CDN-kanonisch) — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gemessene Tag-Menge je Netloc aus `phi/sources.φ`; Operator-Wort vor jeder destruktiven Entfernung
- **Lage:** (gemessen 2026-10-03) `docs/surveys/survey-2026-09-03-orphan-verdicts.md` trägt den 13-Netloc-Plan. **Alle 13 gemessen** (`gh release view <tag> --json assets`). **Mis-Tags gefunden + geheilt:** `vizier.cds.unistra.fr` (denis/mktypes/pastel/wds) und `rave_dr5.json`; zwei Reader (`bigbang-echo.yml`, `dark-flow-probe.yml`) auf `tapvizier.cds.unistra.fr` gebunden. **Unmanifestiertes Register-Asset:** `pds3_ring_occ.bin` (`pds-rings.seti.org`) **404**, kein Writer → Riss. **Unregistrierte Release-Assets (Orphans):** pds-ppi 3, zenodo, spdf 5, pds-rings 1, naif ~600, minorplanetcenter 1, irsa 13, pmel 5, tapvizier 10, **ssd.jpl.nasa.gov 996**. Register canonical (`register_sort`, 1570 Blöcke). **Teil 1 des Rats-Entscheids (Cap-Gate + Rotation) gebaut** (s. o.); Teil 2 (Tag-Diff) offen.
- **Blockade:** die Bindungen (Probe-Writer, Register-`url`s, Tag-Diff) stehen noch; vorher wird nichts gelöscht.
- **Braucht:** Step-5(b) Teil 2 (Tag-Diff); dann Probe-Writer-Rebindung; dann je Lösch-Klasse ein Atom mit gemessener Tabelle.

### Träger (Meta) — Prosadokumente
- **Status:** wartend | **Bindung:** eigen (Meta-Träger)
- **Trigger:** `register_lookup --orphan-docs` nennt ein neues trägerloses Prosadokument bzw. Owner-Fold der Adress-Blöcke
- **Lage:** (gemessen 2026-10-03 via `register_lookup --orphan-docs`) **1** Orphan: `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (2 offene Marker) — Träger **Mycelium**, hier gefaltet. Die vier aus folge223 sind getragen; `survey-2026-09-03-orphan-verdicts.md` trägt Mycelium selbst (Punkt oben).
- **Blockade:** keine
- **Braucht:** die 2 Marker von `daten-holdings-inventur.md` messen und je Marker den nächsten Schritt fahren (Inhalt = Daten-Holdings-Inventur der `phi/sources.φ`-Bestände).

### Register-Träger — `blocked_sources.φ` mycelium-Arme ohne Träger (future-169/226)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-03 via `register_lookup --orphans`) die fünf mycelium-Einträge aus future-169/226 sind **getragen** (`--orphans` mycelium **0**); offen bleibt `:444` `http://222.194.16.107/planet-data` (Shandong PDS-Spiegel, `blocked ip-blocked`, cn-only, kein Pfad gemessen). Die vier `pending` `:497/:501/:505/:509` harren der Mountain-Löschung nach dem Transport.
- **Blockade:** `:444` cn-only
- **Braucht:** `:444` bei erreichbarer Route portieren; Mountain lässt `:497/:501/:505/:509` fallen.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## An river

Origin: mycelium-folge225 (Cap-Bypass-Rotation nvss/gaia).

- **`nvss-cdn` — async-`phase ERROR` geheilt, Upload auf Family-Tag rotiert:** `nvss-cdn.yml:46` schreibt jetzt nach `ssd.jpl.nasa.gov-nvss` (vorher `ssd.jpl.nasa.gov`, gekappt). Der Fix der fehlenden Chunkung (8 RA-Slices, `jq -s add`) steht. Nach dem Push: `gh workflow run nvss-cdn.yml`, den Lauf lesen (`<errorSummary>`). Damit ist Rivers nvss-Punkt an der Kante — bitte beim nächsten River-Pass schließen.
- **`docs/auftrag/auftrag-flyby2-kette.md`** (3 offene Marker) trägt keinen lebenden Owner-Träger (`register_lookup --orphan-docs`): Flyby-Path-2-Kette. Bitte als Trägerzeile falten oder gemessenes `descoped`.

## An mountain

Origin: mycelium-folge225 (Register-Duties).

- **Viking/Voyager-Transport (folge226):** `sources.φ` trägt `viking_lander_tracking.bin` und `voyager{1,2}_merged.bin`; `voyager{1,2}` sind manifestiert (**206**). Viking-Fix: Release-Tag `ssd.jpl.nasa.gov-planets` (Host gekappt). Die vier `pending` `blocked_sources.φ:497/501/505/509` können nach dem Viking-Erfolg fallen.
- **JWS2-Bin registriert (folge227-`## An mycelium` gefaltet):** `phi/sources.φ:9297` `jwst_spectra.bin` (`format jwst_spectra`, sha `fb181924…5535`), CDN **206** — Punkt kann fallen.
- **M3-Zeile registriert, Manifestation offen:** `phi/sources.φ:9871` gesetzt; CDN-Asset **404** (CI-Runner-403). **Riss zur folge227-Zeile:** origin ist die **`_0003`**-Route (direct 206), nicht `_0004`.
- **RoPeR registriert (folge227-`## An mycelium` gefaltet):** `phi/sources.φ:11085` (`format gras_2c`, origin Zenodo 15812343, compiler `roper_pds4_compiler.rs`); Workflow `roper-cdn.yml` angelegt. Der Zuordnungsteil (Feld/Force je Bin) bleibt bei Mountain.
- **`docs/surveys/survey-2026-09-16-dead-sources-relevanz.md`** (1 offener Marker) trägt keinen lebenden Owner-Träger — bitte Trägerzeile oder gemessenes `descoped`.
- **`docs/surveys/survey-raetsel-bestand.md`** (6 offene Marker, u. a. „Fremde Feder (Mountain)": `witness kuprat` fehlt — `:90`) trägt keinen lebenden Owner-Träger — bitte Trägerzeile oder gemessenes `descoped`.
- **`twomass_psc.bin` UNREGISTERED:** `cdn-health` fand `irsa.ipac.caltech.edu/twomass_psc.bin` **200**, aber keine Quellen-Zeile in `phi/sources.φ`. Braucht: Zulassung/Format/Field (2MASS PSC J<11), dann schreibt Mycelium `url`/`origin`/`compiler`/`sha256`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
