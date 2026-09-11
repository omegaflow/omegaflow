<!--
  title: Handover — Nicht-autonom (Stand 2026-09-11)
  class: handover
  date: 2026-09-11
  sha256: 1e7f138561255a0ab1560e6946c600e972133cb9df6488ec249b019f6e7b2f55
  status: live
-->
# Handover — Nicht-autonom (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Keine Session-Arbeit — jedes Item wartet auf Operator-Wort, CI oder Kalender.

## Operator-Wort (extern)

- Desktop-Fork (GTX 970)
- sicherung-risiko-heime — frische Kopie 2026-09-11: DB 1,21 G
  (`~/backup/sessions/opencode/snapshot-2026-09-11/opencode.db`, sha256
  `95892c01…`, integrity ok) + undo-snapshot 1,3 G
  (`…/undo-snapshot-2026-09-11/`); wiederkehrender Rhythmus/Registeranker offen
  (der Mirror altert sonst erneut).
- adoption — Repo `omegaflow/omegaflow` ist public (gemessen 2026-09-11); der
  Drei-Mail-Block (Toth/Turyshev/Markwardt) wartet auf den two-/three-way-Split
  (`docs/paper/twenty-second-band-ground-chain.md` §4 offen).
- LISA Pathfinder (Selbstregistrierung ab 15.09.)
- NOIRLab Data Lab — Altkonto `omegaflow` existiert, Passwort-Reset scheitert an
  der E-Mail-Zuordnung; Neuregistrierung `jtyroller` eingereicht (2026-09-11),
  wartet auf menschliche Freigabe.
- TOAR-Vollzugang — Projektvorstellung an Sabine Schröder (Jülich) am 2026-08-25
  raus, wartet auf Freigabe; kein Helmholtz-Konto vorhanden, anonym lädt das
  Dashboard (24 385 Stationen).
- Rubin RSP-Datenrechte — MelissaGraham (LSST) hat das unvollständige Konto
  expungt (Forum-Topic 12665, 2026-09-11); Signup neu durchlaufen (Google-IdP,
  E-Mail verifiziert, Username `omegaflow`), Antrag in Prüfung — wartet auf
  Freigabe bzw. „Confirming your data rights for the Rubin Science Platform".
- TNO-Kette

## CI/Merge

- papier-kleinpass (nach Merge)
- de441 Re-Verifikation (grüner bodies-Job)

## Kalender

- Nadel Ⅰ — Jeans-Residuum bis Gaia DR4
- Nadel Ⅱ — JUICE 28./29.9. + Europa Clipper 3.12.
- gaia-dr4-iapetus
- LISA Pathfinder

## Ernten (Check-back)

- AllWISE
- ned-Crawl — hing bei 1/40: Kegel 52 (RA 245.45/DEC −72.18) lief in den
  170-s-Timeout und brach die ganze Slice ab; Fix im Workflow (`ned-cdn.yml`:
  Void-Kegel überspringen, in `ned_void.json` protokollieren, Slice läuft
  weiter), Verifikation im nächsten Lauf offen.
- ned-objdir — Bulk-Redshift-Zugang für den NEDTAP.objdir-Harvest angefragt;
  IPAC-Auto-Bestätigung 2026-09-09 („we'll get back to you"), inhaltliche
  Antwort ausstehend.
