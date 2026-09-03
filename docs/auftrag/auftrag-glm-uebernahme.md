<!--
  title: Auftrag — GLM-Verifikation registrieren + Paper auf main führen
  class: auftrag
  date: 2026-08-30
  status: pending
  see-also: docs/auftrag/auftrag-main-reinigung-kanonische-ordnung.md
  sha256: de7d4f74c046ad4b05d33327c19ca4541828b8e215ac1dffd2c5c941b9eb9ff6
-->

# Auftrag — GLM-Verifikation registrieren + Paper auf main führen

Session: Registrierung im Repo
Status: ausstehend

## Ziel

Die GLM-Verifikation registrieren, die drei offenen Punkte des Gegen-Audits schließen,
die Paper als Ganzes auf `main` führen — ohne Merge der Review-Infrastruktur.

## Ist-Zustand (gemessen am 2026-08-30)

- Der Arbeitsbaum ist sauber, `docs/audit/` existiert NICHT.
- Die Verifikation liegt ausschließlich im Repo, untracked:
  `docs/audit/glm-verifikation-2026-08-28.md`.
- Die Commits des Gegen-Audits existieren: `e2101d6`, `9d25fe4`, `8e1639a`,
  `ec6cca6`, `cd24606`, `66deafa`, `cf1155f`.

## 1. Verifikation registrieren (Doktrin: Wahrheit in git, nie im Verlauf)

- Die Datei liegt untracked im Repo.
- Pfad übernehmen: `docs/audit/glm-verifikation-2026-08-28.md`.
- Die Verifikation committen (docs/audit/).
- Prüfen: `git log --all -- docs/audit/` muss danach die Datei zeigen.

## 2. Die drei offenen Punkte des Gegen-Audits schließen

- **GIC „1321":** Formulierung ersetzen durch die interne, im Blatt verankerte
  Diskrepanz **1378 (Bz-Records) ↔ 1260 (paired)**. Auftrag 5 soll diese Version
  übernehmen — nicht „1321".
- **solar-cycle „Phasenraum-Re-Run":** Cross-Paper-Folgerung, zwischenzeitlich durch
  `8e1639a` + `ec6cca6` aufgelöst (alle vier Instrumente post-fix, kein Verdikt kippt).
  War wahr, ist erledigt — **kein neuer Fix nötig**, nur im Register als erledigt markieren.
- **Patient 201/202:** Externes Domänenwissen (MIT-BIH-Dokumentation führt 201/202 als
  zwei Hälften derselben Aufnahme). Einen Satz im Blatt `lead-geometry-direction.md`
  mit diesem Anker (PhysioNet-Doku) ergänzen, damit der Befund prüfbar wird.

## 3. Paper als Ganzes auf `main` führen — Pfad-Übernahme, kein Merge

- Vorprüfung: `git diff --stat origin/main origin/paper-reviewen -- docs/paper/` — muss
  zeigen, dass `docs/paper/` auf `paper-reviewen` eine Obermenge von `main` ist.
- **In der Leitstellen-Session** (nie Arbeits-Session):
  - `git checkout main && git pull origin main`
  - `git checkout origin/paper-reviewen -- docs/paper/ docs/surveys/ docs/concepts/`
  - Stichprobe: `grep -c "79-year window" docs/paper/signal-cone-audit-sheet.md` muss **0**
    sein (enthält jetzt „~10 years").
  - `git add docs/ && git commit` — Commit-Message nennt Quellspanne `66deafa..cd24606`.
  - `git push origin main`
- Review-Werkzeuge (`export_latex`, `reference_verify`, `paper_new`, arxiv-Reader) bleiben
  **auf `paper-reviewen`** — nicht auf `main` (Doktrin `cf1155f`).

## 4. Offen, unabhängig von der Übernahme

- flyby-2-Metrik, signalkegel 492,0 s + „holds" (noch nicht gefixt — Auftrag 7),
  corona 5 Punkte, gic (mit 1378↔1260), lead-geometry (100/99 + Atom-10),
  urknall/dunkler-fluss Präzision+Kommata, flyby-1 Spanne, laic 2 Sätze,
  boden-quellen 1 Satz.
