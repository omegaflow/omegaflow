<!--
  title: Handover — Ernte-Folge 101 (Stand 2026-09-19)
  session: Ernte-Folge 101
  class: handover
  date: 2026-09-19
  sha256: 82562d150ef11787584967a6da0e190d911d3e9be2450d35b86c568c9ad82571
  status: live
-->
# Handover — Ernte-Folge 101 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-19, Folge 101)

- **HEAD** `f6b10df0` bei Start (== `origin/main`); während der Session von der
  Forschung-Linie auf `bbf3cf3a` → `eb1aa4fd` gezogen (forschung folge99/100,
  gepusht). Safety-Net `refs/safety/1789852033` (Start). Der Baum ist geteilt und
  bewegt sich; eigene Hunks nur auf `ledger.φ` + `external-state.md` (Lasair-Zeile).
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789795811` (Rubin-
  Forum, informativ), kein neuer Ernte-Eingang. `post.md` trägt nur die
  ernte→bau-Zeile (unverändert, bau hat sie nicht gefaltet).
- **CI-Status** — die Zeile `docs/zustand/external-state.md` (CI) wurde in dieser
  Session von der Forschung-Linie auf HEAD `bbf3cf3a` frisch gemessen und
  committet; sie wird zitiert, nicht überschrieben. `ci_manage list` (~21:20Z)
  bestätigt: `openneuro-cdn 35468606830` @`7aa5c23e` **failure** (ds007822,
  parser-gap); `measure-gates 35468941451`/`quake-feeds-cdn 35468653918`/
  `ned-cdn 35468572091`/`placebo-ave-cdn 35468313922` success; pending
  `ci-check 35470085666`, `hyperscanning-te 35468144989`.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt **kein Target**
  (`browser_targets` leer, keine Extension verbunden); der Netzzugriff lief über
  Playwright. Der Playwright-Pfad ist der tragende; der opencode-Browser-Pfad
  bleibt `operator-gebunden` (Extension verbinden).

## Offen

- **OpenNeuro ds007822** `phi/pipeline/ledger.φ:122-124` — `parser-gap`, **wartend
  auf bau**. `extract_eeg` (`openneuro_compiler.rs:300`) lehnt jedes `.set` ab
  („carries no EEG contract"); der `--local`-Pfad (`:422-429`) meldet nur das
  Fehlen, **nicht das Feld**. Der bau-Arm läuft: `openneuro_compiler.rs` ist im
  geteilten Baum bereits modifiziert (`+53`, bau folge95). **Schritt:** bau
  benennt das Feld; dann Ernte-`--probe`. Post an bau steht. `wartend` auf bau.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern). Re-Messung 2026-09-19 (Playwright): dachs sync-QUERY
  HTTP 500, pithia http sync-QUERY 200 — beide „connection to server localhost:5432
  failed: Connection refused", Root je 200; **pithia https-Zertifikat jetzt
  abgelaufen** (`ERR_CERT_DATE_INVALID`, neu). **Schritt:** Re-Check
  (`archive_search --verdict` + sync-QUERY), Trigger Backend-Erholung. `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern). Re-Messung
  2026-09-19: `api.lasair.lsst.ac.uk/api/` **502 über alle 10 Proton-Free-Exits**
  (nl ca ch jp sg us pl no ro mx), Antwort nginx/1.24.0 (Ubuntu) 502 Bad Gateway =
  Upstream down; direct absent; Token-Query (URL-encodiert, Token present) 502;
  Webseite `lasair.lsst.ac.uk/` 200 (Statusseite „ZTF/LSST: Up", status code 200
  @21:18Z), `/query/` 404. **Kein Geo- und kein Token-Gap — Backend server-seitig.**
  **Schritt:** `archive_search --verdict`, Trigger Banner-Wechsel. `blockiert`.
- **ds007471 BrainVision-Arm + ds008192 SNIRF-Arm** `phi/pipeline/ledger.φ:126-132`
  — an bau übergeben; im Baum liegen bereits `src/archivar/brainvision.rs`,
  `snirf.rs` + Compiler (fremd, uncommittet). `wartend` auf bau.
- **opencode-Browser-Bridge** — kein Target (`browser_targets` leer). **Schritt:**
  Extension öffnen/verbinden, dann `browser_open`. `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — 127/304 Spans, Final absent; stündlicher Schedule. Bei
  Abschluss CDN-Pflicht (`sources.φ`). **Schritt:** `ci_manage view <allwise-run>`.
  `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53`
  — Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April.
  `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument →
  `pending`. `wartend`.

## Operator-gebunden

- **Limadou PI-Freigabe** `phi/pipeline/ledger.φ:26-28` — per-act consent.
  `operator-gebunden`.
- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort. `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- **Ernte-Folge 101** (build/flash, Hauptsession): die Netz-/Browser-Messung
  (Playwright + 10 Proton-Exits + Token-Query) lief in der Hauptsession — die
  Browser-Werkzeuge (Playwright, opencode-Browser) sind keinem Sub-Agenten-Profil
  zugänglich, eine flash/pro-Dopplung war strukturell nicht möglich. Kein neuer
  Klassen-Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/pipeline/ledger.φ` (dachs/pithia-Notes →
  Playwright-Re-Messung), `docs/zustand/external-state.md` (Lasair-Zeile),
  `docs/handover/handover-2026-09-19-ernte-folge101.md` (neu), Move
  `handover-2026-09-19-ernte-folge100.md` → `archiv/`.
- **Fremd (nicht anfassen):** `src/archivar/hdf5.rs`, `matfile.rs`, `mod.rs`,
  `src/lib.rs`, `src/mathematikerin/te.rs`, `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `openneuro_compiler.rs`, `tools/utils/src/bin/archive_search/net.rs`,
  `src/archivar/brainvision.rs` + `snirf.rs`, die brainvision-/snirf-Compiler,
  `handover-2026-09-19-bau-folge95.md`, `handover-2026-09-19-forschung-folge100.md`,
  die `handover-2026-09-16-*`-Moves, der bau94-Move. Die CI-Zeile in
  `external-state.md` ist von der Forschung-Linie committet. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
