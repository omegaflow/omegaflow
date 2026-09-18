<!--
  title: Handover — Bau-Folge 75 (Stand 2026-09-18)
  session: Bau-Folge 75
  class: handover
  date: 2026-09-18
  sha256: d73633d6086ac316481cedd65a570eda37ab3831afde6b05fa7ce8cf414d8097
  status: live
-->
# Handover — Bau-Folge 75 (2026-09-18)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18, HEAD 0dc09172)

- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Rubin-Forum-Thread, **kein Agenten-Eingang**); die `An bau`-Zeile
  in `docs/handover/post.md` (forschung, `af64a132`-Hunks) war reine Kenntnis —
  gefaltet und gelöscht.
- **CI am HEAD** — `ci-check` `35307448019` @`69c3ae96` **failure** (Jobs
  format/clippy; `RUSTFLAGS=-D warnings` via setup-rust-toolchain); danach nur
  cancelled/queued Läufe fremder Pushs. `release-build` `35302966400` failure
  (Wurzel war die veraltete `ci_manage`-Datei-Kopie, bau-folge74);
  `pii-exposure` `35287195140` failure (exit 2 = Exposition bleibt, erwartet);
  `xp-pilot-cdn` `35305640254` failure; sonst fremde Linien.
- **HEAD** `0dc09172` == `origin/main` (FF-ready).
- **Sicherheitsnetz** — `refs/safety/1789709288` (Session-Start).

## TE-Gate — konditionaler Arx-Null umgebaut, Gate-Verdikt ausstehend (härtester undatiert)

- Rot war `conditional FP/FN gate: FPR rise 6.00pp over a at rho=0.5 exceeds 2pp`
  (Gitter 2/3/2/1/4/7/6/5/2 je 100). Der Null
  `arx_restricted_surrogate_conditional`/`_2` fittet jetzt das **volle** Modell
  `y ~ 1 + y_lags + c_lags + x_lags` (x-Lags 1..=p, nur Vergangenheit),
  rekonstruiert mit genullten x-Koeffizienten (δ=0) und permutiert die Residuen
  des vollen Fits; die `None`-Verweigerung (Fit-Fehlschlag/Längen-Mismatch/
  nicht-finit) bleibt erhalten. Diff `src/mathematikerin/te.rs` +168/−19;
  `cargo check --all-targets` sauber. Dispatcht `35313041295` @`b8bb7b01`.
  (Schritt: Run-ID registriert; rote Zelle @rho=0.5 und FN-Arm
  `found/meas ≥ 0.5` im CI-Lauf lesen.) · `pending`
- **Rat-Einwand (registriert, kein Blocker):** der volle Fit orthogonalisiert
  die Residuen gegen x; die Nullen-Rekonstruktion stellt die x-getragene
  c-Leistung nicht wieder her → die Null-Verteilung kann zu eng werden
  (FPR-Inflation, Richtung der alten roten Zelle). Das Kalibrier-Gate ist der
  Schiedsrichter; ein grünes Gate zertifiziert die Null für die Gate-Klassen,
  nicht für das bidirektionale Einsatzfeld.
- **Restrisiken (getragen, nicht fallen gelassen):** (a) bidirektionaler
  Gate-Arm fehlt (Coverage-Gap); (b) `arx_restricted_surrogate` (unkonditional)
  trägt noch den `shuffle_series`-Fallback (`te.rs:1079,1086`) — „nie Shuffle"
  gilt nur für die konditionalen Varianten; (c) Name = Implementation:
  `arx_restricted_surrogate_conditional` fittet jetzt das volle Modell — Rename
  nach grünem Gate.

## Red main — clippy/format behoben, ci-check ausstehend

- 28 clippy-warnings-as-errors in `src/archivar/*.rs` behoben
  (needless_range_loop, op_ref, unnecessary_cast, collapsible_if,
  manual_range_contains, manual_unwrap_or, trim_split_whitespace,
  too_many_arguments → `struct Calib`, manual_is_multiple_of) + `format`-Stil;
  kein `#[allow]`.
- `port.rs` alt-lose Zeile: statt `unwrap_or(SURFACE_ALT_M)`
  (0-Kanon-Verletzung, Rat-Verdikt) jetzt Verweigerung `eprintln!` + keine
  `on`-Zeile, der Rest des Blocks portiert weiter — spiegelt parse.rs
  `on without alt refused — declare alt`. Dispatcht `35313038589` @`b8bb7b01`.
- (Schritt: format/clippy/build/test-Jobs lesen. Named risk: der flare-Leg der
  nicht-ignorierten Tests läuft jetzt durch die Vollmodell-Null — rot dort ist
  die Verweigerung als Messung, Folge-Schritt ist das Design zu verbreitern,
  nie die Verweigerung aufzuweichen.) · `pending`

## Kanon-Gate — CI-Verifikation offen

- Kanon deklariert (`phi/canon.φ`) + Gate in `commit_check.rs`.
  (Schritt: grüner `ci-check`-Lauf; blockiert durch Red main.) · `pending`

## `supermag_stations.φ` — kein `.rs`-Leser

- 599 Stationen, statische Compiler-Eingabe, kein Verbraucher.
  (Schritt: SuperMAG-Arm in `tools/harvest` bauen oder als `descoped` messen.) ·
  `pending`

## Offen (kein Handlungsschritt)

- **Bindings-Prosa** `phi/bindings/{dust-maske,bathymetrie-gebco}.φ` —
  Architektur-Akt mit Operator-/Council-Wort. · `operator-gebunden`
- **`opencode.json`** — fremder uncommitteter Hunk. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/archivar/{flac,galileo_odr,goes_abi,ifms_agc,kcdc,lro_utf,odf,parse,port,vlies}.rs`,
  `src/mathematikerin/te.rs`, `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-bau-folge75.md`,
  `docs/handover/post.md` (eigene Zeile gelöscht).
- **Fremd (nicht anfassen):** `opencode.json`, die Handover-Archiv-Renames/
  Deletes der anderen Linien. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
