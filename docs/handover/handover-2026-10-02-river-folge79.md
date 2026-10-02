<!--
  title: Handover — River-Folge 79 (2026-10-02)
  session: River-Folge 79
  class: handover
  date: 2026-10-02
  sha256: 26784490bed1daf114db872ddd539707699866e22aa62ed7981756538c667916
  status: live
-->
# Handover — River-Folge 79 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" (Dispatch-Profil-Rückfrage) | 2026-10-01 | Operator (Session, River 77)
„braucht es pro?" (Wiederholung; gemessen: nein — flash löste die GIC-Kanal-Faltung identisch) | 2026-10-02 | Operator (Session, River 79)

## An future

Origin: river folge79 (Faltung aus folge78, noch nicht gefaltet).

- **GIC-Schluss-Gegenlesung / `the riss stands`:** Rivers Teil bleibt der eine Satz —
  nach dem `wy-max-t`-Lauf wird `docs/paper/gic-causal-driver.md:17` („the riss stands")
  durch die gemessene Fassung ersetzt; §3.2 offene Konstruktion, α unbenannt bis dahin.
  Kein Satz vor dem Messergebnis.
- **ENSO-Design / Flyby-σ-Metrik / TE-Novelty (nemotron-Stimmen):** gefaltet — sie
  speisen die ENSO-Positivkontrolle (Desaisonalisierung vor der TE) bzw. den
  GIC-Paper-Text; keine neue Antwort nötig.
- **Frontier-Kanäle + Kanal-Routing:** bestätigt. Kimi K3 + GLM-5.3 ohne Login
  (tryingopen/together), erste Route `chat.z.ai` (per-Akt-Wort), arena als benannter
  Fallback; die hängenden UI-Routen entfallen. Roster-Update ist deins.

## An mountain

Origin: river folge79 (Faltung aus folge78; `station ABK` gefaltet in `7fce797df`, die Namens-Schuld steht).

- **Namens-Schuld (gemessen 2026-10-02):** `field intermagnet_dbdt` ist am ABK- und am SOD-Block
  doppelt deklariert (`phi/sources.φ:1793` + `:1802`) — dieselbe Zeichenkette für zwei Stationen;
  das Register muss die Namen trennen oder die Doppelung als gewollt benennen. Funktional heute
  (der Metas-Schlüssel ist der Kanalname), aber namens-blind.
- **`jwst_spectra`-Richtung bei absentem Abstand:** der Consumer trägt extragalaktische Records
  jetzt am deklarierten `at`-Anker (`main_flow.rs`, `jwst_spectrum_motion`); der Wire hat keinen
  Richtungs-Slot ohne Abstand. Eine Richtungs-Führung bräuchte ein `z` im JWS1-Bin — deine Feder.

## Offen (aufgeschlüsselt)

### tapvizier-TAP-Klasse — Verifikationslauf (geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der Re-Dispatches `rave-cdn`/`sb9-cdn`/`first14-cdn` (dieser Atom)
- **Lage:** (gemessen 2026-10-02) Wurzel gemessen: river-77 („ADQL quoting") setzte in der
  `CONTAINS`-Join-Klausel ein literales `t.`/`j.` **vor** `alias_col` → `t.t."RAJ2000"`,
  `j.j."RA_ICRS"` → CDS 400 „unresolved identifiers"; die ADQL war sonst gültig. Beide
  Format-Positionen (`--crossmatch` und `--crossmatch-z`) korrigiert; lokal gegen
  `tapvizier` re-generiert: sb9 → 13 Spalten/5 Zeilen, rave-Slice-0 (`--where`+pm) → 12/5.
- **Blockade:** keiner
- **Braucht:** nach Push `gh workflow run rave-cdn.yml` (sb9/first14 ebenso); `ci_manage log <id>`
  bestätigt grün. `tap_compiler.rs` trägt `--fail-with-body` + `body=…` (1600 B) im Fehlerfall.

### σ-Asset `dr3_stars.bin` — falscher Release-Tag, Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn` (dieser Atom)
- **Lage:** (gemessen 2026-10-02) der 56-B-Bin (`1 704 003×56 = 95 424 168 B`) wurde vom Lauf
  `36943970102` in den **falschen** Release `tapvizier.cds.unistra.fr` hochgeladen
  (`gh api … updated 2026-10-02T00:48:55Z`); der registrierte Release `ssd.jpl.nasa.gov` trägt
  noch das **legacy 44-B**-Asset (`75 001 828 B`, updated `2026-08-23`, sha `fb9a1408…` → keine
  σ-Slots). Ursache: `gaia-cdn.yml` übergab kein `--release-tag` → Compiler-Default
  `tapvizier.cds.unistra.fr`. Fix: `--release-tag ssd.jpl.nasa.gov` gesetzt.
- **Blockade:** keiner
- **Braucht:** nach Push `gh workflow run gaia-cdn.yml`; dann `ssd.jpl.nasa.gov`-Asset snifen
  (56-B-Stride, 95 424 168 B) und den 3-Slot-σ-Zensus rechnen. Ein σ-Zensus-Bin existiert **nicht**
  — `star_dmax_probe`/`vlies_density_probe` lesen den Katalog ohne die σ-Felder
  (`spatial.rs` `sigma_plx_mas`/`sigma_pm_ra_masyr`/`sigma_pm_de_masyr` bleiben unkonsumiert);
  minimales neues Bin nötig (Layout `tap_compiler.rs:437-488`, 0.0 = absent).

### Kalibrierte Null (Westfall–Young max-T) — 6-h-Cap
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Rat/Operator-Wort (Sharding der WY-Familie)
- **Lage:** (gemessen 2026-10-02 via GH-API) die drei wy-Jobs `36943967388` wurden bei exakt
  **6 h 00 m** gekappt (`01:10:42→07:10:58` usw.) — die GitHub-Hosted-Runner-Hartgrenze, nicht
  `timeout-minutes: 420`; drei Läufe in Folge `cancelled`, kein Messergebnis.
- **Blockade:** `--n-perm 9999 --block 24` über ein Jahr passt nicht in 6 h (4 Threads)
- **Braucht:** Design-Entscheidung: die WY-Familie sharden (statistisch nicht trivial —
  die Null-Verteilung muss familienweit gelten) oder `n-perm`/Kosten senken. Vorschlag dem
  Rat/Operator vorlegen, nicht solo ändern.

### Weberin — Produzent-Blob-Persistenz
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Operator-Wort (kanonischer Blob-Produzent)
- **Lage:** (gemessen 2026-10-02) `station-convergence 36944080015` `success`, doch
  `station-convergence.yml:10` hat `contents: read` und lädt nur `station-convergence.txt`;
  `data/weberin_verdicts.bin` bleibt auf dem Runner. Kanonischer CDN-Produzent ist
  `weberin_verdicts_compiler` (`ssd.jpl.nasa.gov-weberin`, faltet registrierte Body-Zeilen).
- **Blockade:** zwei Produzenten für dieselbe Datei — der Live-Probe schreibt sie, der
  Compiler schreibt die kanonische
- **Braucht:** Contract-Wort: ist der Live-Probe-Blob lokal oder als Body-Zeile zu registrieren,
  damit der Compiler ihn faltet? Danach `station-convergence.yml`/`main_flow.rs:1036` bauen.

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de liefert `def`; ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-02) OMNI2 H0 1-hr: neuester Echtwert `2026-09-03T00:30Z`, danach nur
  Fill (HAPI: „last … data Sep 02, 2026"). kp.gfz.de: bare Pfad 500, parameterisiert 200 (11/2 d,
  49/Woche), alle `status:"pre"` — `def` noch nicht publiziert. ESOC `ephemeris_juice_recon.bin`:
  CDN 404, lokal absent — **und in `phi` nicht registriert** (Riss: `sgrep recon.bin phi` = 0).
- **Blockade:** externe Datenvorläufe; fehlende ESOC-Registerzeile
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen; ESOC-Asset-Registerzeile (Mountain)
  oder descopen.

### Frühwarnsystem — Präregistrierung (Rat, zerlegt 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** α-Ebene der kalibrierten Null (`wy-max-t`)
- **Lage:** (gemessen 2026-10-02) Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md`
  (`status: unsealed`) committet; α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus (Siegeln gegen 10⁻¹ verboten)
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### ENSO — Desaisonalisierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `enso-probe`
- **Lage:** (gemessen 2026-10-02 via `gh run download 36943976395`) die Positivkontrolle ist
  **bestanden** — die gepflanzte Kopplung wird bei lag 3 detektiert (TE `7.6483e-1` > per-lag-Thr
  `2.7245e-1` und > fam `3.0247e-1`); Bz↔SST bleibt `silent`. Der Jahreszyklus ist als Confound
  benannt (12-Monats-Band), aber die Desaisonalisierung vor der TE ist noch nicht gebaut.
- **Blockade:** keine
- **Braucht:** Desaisonalisierung (SST/Bz/Wnd saisonal bereinigen) vor der TE; dann die Bz↔SST-Zeile
  im GIC-Paper fortschreiben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt — **Commit als Letzter**. Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/harvest/src/bin/tap_compiler.rs` · `.github/workflows/gaia-cdn.yml`
- `docs/paper/gic-causal-driver.md`
- `docs/handover/handover-2026-10-02-river-folge79.md`, und `…-folge78.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`):** `kernel-flatten.yml`, `src/archivar/fit.rs`,
`tools/harvest/src/bin/ceers_spectra_compiler.rs`, die Mycelium-Handover-Moves/Neuanlagen,
`dsn-cdn.yml`/`jades-cdn.yml` — alle unangetastet.

## Burn: open 0.0 · close 0.2400 · cap 0.5 · Grund: tapvizier-Wurzel + gaia-release-tag + 3 Messläufe (ENSO/bz/flyby) + GIC-Kanal-Faltung, ein Pass
