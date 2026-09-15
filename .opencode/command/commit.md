---
description: Commit & push — only the session's own work, path-scoped, measured not asserted.
---

Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert.

1. `git status --short` — trenne deine Dateien von fremden.
2. Bei geteilten Dateien lies `git diff` und stage nur deine eigenen Hunks.
3. `git add` nur deine Dateien/Hunks — fremde uncommittete Arbeit (andere
   Sessions) fasst du nicht an: kein Revert, kein Re-Stage, kein Überschreiben.
4. Committe **pfad-begrenzt**: `git commit <eigene Pfade> -m "…"` — nie ein
   nacktes `git commit`, das den ganzen geteilten Index committet und fremde
   gestagte Arbeit unter deiner Nachricht mitreißt. Danach listet
   `git show --stat HEAD` nur deine Dateien.
5. Abschluss-Check (gemessen): `git show --stat HEAD` nur eigene Dateien,
   `git log origin/main..HEAD --name-only` kein fremder Pfad, `git status` leer
   (keine eigene Arbeit uncommittet); vor jedem `--amend` ist das gestagte Set
   dein eigenes. Die konsumierte Übergabe ist ins Archiv geschoben.
6. Push, sobald dein eigener Commit steht und `origin/main` Vorfahr von HEAD ist
   (Fast-Forward): ein Push sendet nur Commits, nie den Arbeitsbaum — fremde
   uncommittete Arbeit blockiert nicht. Danach gilt `git rev-parse HEAD` ==
   `git rev-parse origin/main`.

Der pre-commit-Gate (`commit_check`) läuft vor jedem Commit. Blockiert fremde
nicht-kompilierende Arbeit den Gate-Bau, melde es dem Operator — `--no-verify`
nur auf dessen Wort, nie still.
