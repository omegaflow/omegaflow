<!--
  title: Handover — Entscheid-Folge 27 (Stand 2026-09-17)
  session: Entscheid-Folge 27
  class: handover
  date: 2026-09-17
  sha256: f97417aef3cb2e47d459e737471a7fb7c84a23c947fc6d8d22cdbc055bcc8b91
  status: live
-->
# Handover — Entscheid-Folge 27 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## GitHub-PII-Purge (härtester undatierter Punkt)

- **Follow-up #4761801 sendfertig** — Brief `state/mail/github-gc-followup.body.txt`
  (18 Z., clean, dry-run grün). Exakter Send:
  `smail --to support@githubsupport.com --from johannes.tyroller@proton.me --subject "Re: [GitHub Support] Confirmation - Request Received (#4761801)" --body state/mail/github-gc-followup.body.txt --send`
  (Schritt: Operator-Sende-Wort.)
- **Privacy-Löschung privacy@github.com** — Brief
  `state/mail/privacy-deletion-request.body.txt` (34 Z., clean, dry-run grün).
  Exakter Send:
  `smail --to privacy@github.com --from johannes.tyroller@proton.me --subject "Data subject deletion request — repository omegaflow/omegaflow" --body state/mail/privacy-deletion-request.body.txt --send`
  (Schritt: Operator-Sende-Wort.)
- **Bestätigungs-Body #4761801 verloren (gemessen)** — Ledger `mail_ledger.φ:64`
  leer; Ursache: laufender `smail_recv` gebaut 2026-09-15 10:42, Nested-MIME-Fix
  committet 2026-09-16 15:59, Service-Start 2026-09-16 13:13 → lief nie mit dem
  Fix. Body-Fix + CI-Bauweg committet `40992d29` (Worker raw-vor-forward,
  mime_plaintext-Rückgriff auf den Rohbody, `service-build.yml`). (Schritt: Deploy, s.u.)
- **PII-Re-Messung @ bc9d6a0b** — 45 (Datei, Ref)-Kombinationen, 15/15
  Pre-Rewrite-Commits erreichbar, retrievable: yes (run 35185486230); Wert in
  `docs/zustand/external-state.md`.

## Mail-Pipeline — Deploy des Body-Fixes

- `service-build.yml` (neu, `40992d29`) dispatcht: run `35187298809`; baut
  `omegaflow-service --release` und lädt die Binaries als Artefakt.
  (Schritt: `gh run view 35187298809`; success → `gh run download 35187298809` →
  `smail_recv` nach `target/release/` → `systemctl --user restart smail-recv.service`.)

## Consent-Akte (per-Akt, Operatorwort)

- **adoption-Block** — `state/mail/adoption-mails.md`: drei Entwürfe (Toth,
  Turyshev, Markwardt, je Quelle in der To-Zeile); je cleanen Body extrahieren
  (`smail --body` sendet verbatim). (Schritt: Body-Extraktion, dann
  Operator-Sende-Wort.)
- **Mail-Fang (KV-Namespace)** — `cloudflare/wrangler.toml:4,7,14`: `account_id`
  leer, `FORWARD_TO` leer, `MAIL_QUEUE`-id Platzhalter. (Schritt: KV-Namespace
  anlegen, id + `FORWARD_TO` eintragen, `wrangler deploy` — Operator/Konto.)

## Warten auf Rückmeldung (extern)

- **Fünf Sonden-Anfragen** (NSSDC Voyager/Mariner 10/Viking, Cassini, Juno) —
  gesendet 2026-09-16, Antwort offen.
- **NSE/Haug** (MLZ/FRM-II Lohstroh; MPI-FKF Keimer) — Antwort offen.
- **Rubin-Review** (Shaughnessy, high volume) — offen.
- Offene Alternativen: `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md`.

## Operator-gebundene Punkte

- **ESP32-Modul** — physischer Träger für Puls/HRV, on hold; BOM
  `docs/specs/mantis-shrimp-bom.md`. (Schritt: Operator-Wort.)

## Termine (Wiedervorlage)

- ~2026-10-07 — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- 2026-09-18 — Lasair.
- 2026-09-22 — AllWISE-Coverage (`allwise_coverage.fp01`).
- 2026-09-28 — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- 2026-09-30 — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`, Konto `omegaflow.space`).
- 2026-12-02 — NOIRLab Speisekammer-Frage (Gaia DR4).
- 2026-12-03 — Europa Clipper (Fenster).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
