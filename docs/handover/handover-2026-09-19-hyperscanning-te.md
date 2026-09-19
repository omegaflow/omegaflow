<!--
  title: Handover — Hyperscanning-TE (Stand 2026-09-19)
  session: Hyperscanning-TE (OpenNeuro ds007822) — family-wise TE screen
  class: handover
  date: 2026-09-19
  sha256: d9598b80581c786fe8b04f2c441ca97b1375b80c9b8a8fd8eb5c0c6515278f50
  status: live
-->
# Handover — Hyperscanning-TE (2026-09-19)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — zuerst den Watchdog-Snapshot
  `/tmp/opencode/ci_status.md` lesen (kein API-Aufruf); bei Lücke/Detail
  `ci_manage list` / `ci_manage view <id>`, Fehllog `ci_manage log <id>`.
  **Nie** `gh run list`/`gh run view`; `gh` nur für `workflow run`/`run download`.

## Werkzeuge (gebaut — nutzt sie)

- `archive_search` — Inhalt (`--root`)/Pfade (`--index`)/NTFS/19 Netz-Modi/`--playwright`/`--verdict`/`--sniff`/`--all`; ersetzt bash-`grep`, `curl`, webfetch.
- `sgrep [-i]` — Zeilensuche über `git ls-files`.
- `sfetch` — fetch; ersetzt `curl -s`.
- `omega_sh` — `reports|status|search|fetch|jwst`.
- `smail` — Mail (Resend), `--dry-run`; Inhalte nie getrackt.
- `register_lookup` — `--live`/`--open`/`--history`.
- `git_safety` — `--snapshot`/`--restore`/`--list`.
- `ci_manage` — `list`/`view`/`log`/`cancel`/`rerun`; statt `gh run list`/`gh run view`.
- `sread [--offset --limit]` — Datei lesen.
- `session_burn` — Burn je Session.
- `gh` — nur `workflow run`/`run download`.

## Hyperscanning-TE (offen)

Gebaut (siehe git `778c68db`): `hyperscanning_group_te` (familiäre Max-Statistik,
`--channel Fz --lags 128 --surrogates 200`, Schwelle = empirisches Perzentil der
Surrogat-Familien-Maxima), `hyperscanning_te_matrix` (Per-Triade-Matrix),
`eeglab.rs` liest Double-MAT, Workflow `.github/workflows/hyperscanning-te.yml`.

- **CI-Lauf `35456288477` (hyperscanning-te, queued 2026-09-19T16:51:48Z) — Ergebnis ungelesen.**
  Der Lauf ist zugleich der erste Parser-Test auf echten ds007822-`.set` (MAT v5,
  Fz-Label). Meldet das Werkzeug `absent`, fehlt Fz im Montage-Label oder das
  Format weicht ab — dann Kanal/Label nach dem gemessenen Log justieren.
  (Schritt: `ci_manage view 35456288477` einmalig; kein Poll.)
- **Bestätigungsstufe** — der Screening-Lauf fährt p95/200; die Überlebenden brauchen
  p99/1000 (Gremium-Verdikt: zweistufig).
  (Schritt: `hyperscanning_group_te --percentile 99 --surrogates 1000` auf den
  Survivor-Zellen; eigener Workflow-Dispatch.)
- **Kohärente Phasen-Null** — gemeinsame Rotation beider Serien (der eigentliche
  Paar-Null; heute trägt die Einzel-Phasen-Null nur als konservative Obernull).
  (Schritt: neues `TeNull`-Modell in `src/mathematikerin/te.rs` + Kalibrier-Gate-Test
  im selben Atom.)
- **Eigen-Historie als Konditionierer** — die eigene Vergangenheit des Ziels ist der
  nächstliegende Konfundierer; der Apparat existiert (`conditional_te_*_n` mit `conds`).
  (Schritt: `hyperscanning_group_te` ruft `&[]` — `LaggedCond` auf die Zielserie setzen.)
- **Mehrere Frontalkanäle** — Fz/F3/F4 als getrennte Matrizen.
  (Schritt: `--channel F3` / `--channel F4` je Lauf.)
- **Topologische TE** (Takens, `te_compute`) als Upgrade der binned 4-Bin-Schätzung.
  (Schritt: `topological_te_phase` statt `transfer_entropy_binned` im Gruppen-Werkzeug.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
