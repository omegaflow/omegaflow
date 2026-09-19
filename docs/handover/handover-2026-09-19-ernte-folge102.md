<!--
  title: Handover — Ernte-Folge 102 (Stand 2026-09-19)
  session: Ernte-Folge 102
  class: handover
  date: 2026-09-19
  sha256: bafd111df0a8be1c95167a583848333a17bda4b38bc30be921706bb9f0f251a3
  status: live
-->
# Handover — Ernte-Folge 102 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Folge 102)

- **HEAD** `3403dce7` bei Start (== `origin/main`, „ernte folge101"); Safety-Net
  `refs/safety/1789853267` (Start). Der Baum ist geteilt und trägt fremde
  uncommittete bau-Arbeit: `src/archivar/{hdf5,matfile,mod}.rs`, `src/lib.rs`,
  `tools/harvest/src/bin/{icesat2_atl03,openneuro,brainvision,snirf}_compiler.rs`,
  `tools/utils/src/bin/archive_search/net.rs`, `src/archivar/{brainvision,snirf}.rs`
  — nicht angefasst. Eigener Pfad-Satz: `docs/zustand/external-state.md` (CI-Zeile),
  neues Handover.
- **Postfach** — `state/mail/mail_ledger.φ`: jüngster Eingang `1789795811`
  (Rubin-Forum „AGN DP2 light curves", informativ), **kein neuer Ernte-Eingang** seit
  Folge 101. Zustand-Eintrag `external-state.md:20` zitiert.
- **CI-Status** — `ci_manage list` (2026-09-19 ~21:30Z): **failure**
  `openneuro-cdn` `35468606830` @`7aa5c23e` (ds007822 `.set`, parser-gap —
  unverändert); **success** `measure-gates` `35468941451`, `quake-feeds-cdn`
  `35468653918`, `ned-cdn` `35468572091`, `placebo-ave-cdn` `35468313922`,
  `openneuro-cdn` `35468312819`, `auto-dispatch` `35468305429`, `harvest-dispatch`
  `35468305339`; **pending** `ci-check` `35470468101`; **in_progress**
  `silence-map-probe` `35468942740`, `ci-check` `35468441157`; cancelled
  `ci-check`-Kette. Der Watchdog-Snapshot (22:29 +02:00) nennt zusätzlich
  `in_progress` `harvest` `35467466676`, `hyperscanning-te` `35465589119`,
  `ci-check` `35465331299`, `measure-gates` `35457737916` (außerhalb des
  20er-Fensters). Zustand-Eintrag `external-state.md:22` auf HEAD `3403dce7`
  fortgeschrieben.
- **Werkzeug-Grenze** — die opencode-Browser-Bridge trägt **kein Target**
  (`browser_targets` leer, keine Extension verbunden); der Netzzugriff läuft über
  Playwright. Der opencode-Browser-Pfad bleibt `operator-gebunden`.

## Offen

- **OpenNeuro ds007822** `phi/pipeline/ledger.φ:122-124` — `parser-gap`, **wartend
  auf bau**. Baum-Messung: bau's uncommittete Änderung an `openneuro_compiler.rs`
  ist **+53, ausschließlich `mod tests`**
  (`a_flattened_mat_v5_eeg_with_utf8_labels_extracts`); die
  Produktions-`extract_eeg`-Lücke (`:300`, „carries no EEG contract") ist
  unberührt. **Schritt:** bau benennt das Feld; dann Ernte-`--probe`. Post an bau
  steht. `wartend` auf bau.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20` —
  `blockiert` (extern). Re-Messung 2026-09-19 (Playwright): dachs sync-QUERY HTTP 500,
  pithia http sync-QUERY 200 — beide „connection to server localhost:5432 failed:
  Connection refused", Root je 200; pithia https-Zertifikat abgelaufen
  (`ERR_CERT_DATE_INVALID`). **Schritt:** Re-Check (`archive_search --verdict` +
  sync-QUERY), Trigger Backend-Erholung. `blockiert`.
- **Lasair-LSST API** `external-state.md:23` — `blockiert` (extern). Re-Messung
  2026-09-19: `api.lasair.lsst.ac.uk/api/` 502 über alle 10 Proton-Free-Exits,
  Antwort nginx/1.24.0 (Ubuntu) 502 Bad Gateway = Upstream down; direct absent;
  Token-Query 502; Webseite `lasair.lsst.ac.uk/` 200 („ZTF/LSST: Up"); `/query/` 404.
  Kein Geo- und kein Token-Gap — Backend server-seitig. **Schritt:**
  `archive_search --verdict`, Trigger Banner-Wechsel. `blockiert`.
- **ds007471 BrainVision-Arm + ds008192 SNIRF-Arm** `phi/pipeline/ledger.φ:126-132`
  — an bau übergeben; im Baum liegen `src/archivar/brainvision.rs`, `snirf.rs` +
  Compiler (fremd, uncommittet). `wartend` auf bau.
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

- **Ernte-Folge 102** (build/flash, Hauptsession): kein bearbeitbarer Auswahlpunkt
  — alle offenen Ernte-Punkte sind `wartend` auf bau, extern `blockiert`
  (Trigger ungefeuert, 2026-09-19 re-gemessen), `operator-gebunden` oder datiert.
  Keine Delegation, kein Doppellauf; keine Benchmark-Klasse.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `docs/zustand/external-state.md` (CI-Zeile auf HEAD
  `3403dce7`), `docs/handover/handover-2026-09-19-ernte-folge102.md` (neu), Move
  `handover-2026-09-19-ernte-folge101.md` → `archiv/`.
- **Fremd (nicht anfassen):** die bau-Arme (`hdf5.rs`, `matfile.rs`, `mod.rs`,
  `lib.rs`, `icesat2_atl03_compiler.rs`, `openneuro_compiler.rs`,
  `archive_search/net.rs`, `brainvision.rs` + `snirf.rs`, die brainvision-/snirf-
  Compiler), `handover-2026-09-19-bau-folge95.md`, die `handover-2026-09-16-*`-Moves,
  der bau94-Move. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
