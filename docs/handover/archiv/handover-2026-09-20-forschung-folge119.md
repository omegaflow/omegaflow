<!--
  title: Handover — Forschung-Folge 119 (Stand 2026-09-20)
  session: Forschung-Folge 119
  class: handover
  date: 2026-09-20
  sha256: e470506d107fdbe2de47d5ab838d261f0714f51dbda44fd674e34cd67f0a1150
  status: live
-->
# Handover — Forschung-Folge 119 (2026-09-20)

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
Der Planungs-Pass nennt die offenen Punkte als Tafel (`Punkt | Status | Bindung |
Schritt`) — der erste ist der härteste undatierte; die Session arbeitet so viele
ab wie möglich. Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen
nur ihren Auslöser und werden nie als Handlungsschritt geführt; gibt es keinen
abarbeitbaren undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Forschung-Folge 119)

- **HEAD** bei Session-Beginn `84aa5147` (== `origin/main`); während der Session
  zog die bau-Linie auf `0cbc6190` nach (fremder Commit, unangetastet). Fremde
  uncommittete Arbeit im Baum — **nicht angefasst**: `docs/handover/post.md` (M),
  `docs/zustand/external-state.md` (M, entscheid-folge64-Hunks),
  `phi/pipeline/probe_hapi_proposed.φ` (M), `phi/pipeline/probe_wave.φ` (M),
  `docs/handover/handover-2026-09-20-entscheid-folge64.md` (??), gestagter Move
  `entscheid-folge63 → archiv/`.
- **Postfach** — kein neuer Ledger-Eingang seit `1789922257` (`sales@pine64.org`:
  Verweis auf `info@pine64.org`); letzter Eingang bleibt `1789922257`. `post.md`
  trägt fremde Hunks — nicht angefasst.
- **CI** — `ci_manage view`: **`ci-check` `35526340084` pending @HEAD `84aa5147`**
  (der gesuchte HEAD-Verdikt-Lauf, noch kein Verdikt); `ci-check` `35526010713`
  in_progress @`b9112e5d`; **`te-gate` `35513982359` @`7d0a1272` weiter in_progress**
  (kein Update seit Start 13:36Z); `tools-build` `35525924525` @`b9112e5d` success.
  Jüngere `ci-check`-Läufe cancelled (Concurrency-Gruppe).
- **Zustand-Ledger fortgeschrieben** — `external-state.md` CI- und Postfach-Zeile
  auf den gemessenen Stand gesetzt (CI @`0cbc6190`, Postfach `1789922257`); die
  Datei trug fremde entscheid-Hunks (Free-Model-Katalog-Zeile) — mitgefaltet, nicht
  überschrieben. `post.md`: zwei Zeilen gesetzt (`An alle Linien:` Tafel-Regel,
  `An entscheid:` Hardware-Sponsoring). Beide Register committet (geteilter Baum —
  der Silo-Schutz hätte das Register eingefroren).

## Planungs-Tafel (offene Punkte)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| 1. `ci-check` HEAD-Verdikt `35526340084` @`84aa5147` | wartend | eigen | `ci_manage view 35526340084` (Lauf pending; success → #1 zu, failure Format → `ci_manage log`-Zeile übernehmen) |
| 2. TE-Gate-Verdikt `35513982359` @`7d0a1272` | wartend | eigen | `ci_manage view 35513982359`; success → vier `fpr_rise_sigma_test`-Zeilen via `ci_manage log`; cancelled → `gh workflow run te-gate.yml` @HEAD (`5b406e16`) |
| 3. Chrome-DevTools MCP Membran-Lauf | operator-gebunden | operator | Operator-Wort für sichtbaren Vordergrund-Lauf; danach CDP-Lesen |
| 4. Hardware-Sponsoring (Pine64/Framework/Tuxedo) | operator-gebunden | operator | Antwort an Dritte = consent-pflichtig; Post-Zeile `An entscheid:` gesetzt 2026-09-20 — Operator-Antwort offen |
| 5. Browser-Anbindung Rest | operator-gebunden | operator | Cookie-Editor-Transfer Operator-Profil ↔ Playwright; Playwright-Extension befund-gated |
| 6. Flyby-Path-2-Kette | termin | termin:2026-09-28 | Zellen ab Perigäum füllen (`ernte`/`research-max`) |
| 7. NSE/Haug | wartend | dritter | Trigger Dateieingang |
| 8. BepiColombo MORE | termin | termin:2027-04 | Freigabe Wissenschaftsphase |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
