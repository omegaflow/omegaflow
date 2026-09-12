---
description: Commit & push — only the session's own work, measured not asserted.
---

Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert.

1. `git status --short` — trenne deine Dateien von fremden.
2. Bei geteilten Dateien lies `git diff` und stage nur deine eigenen Hunks.
3. `git add` nur deine Dateien/Hunks — fremde uncommittete Arbeit (andere
   Sessions) fasst du nicht an: kein Revert, kein Re-Stage, kein Überschreiben.
4. Committe nur deinen Teil; danach listet `git show --stat HEAD` nur deine Dateien.
5. Abschluss-Check (gemessen): `git show --stat HEAD` nur eigene Dateien,
   `git log origin/main..HEAD --name-only` kein fremder Pfad, `git status` leer
   (keine eigene Arbeit uncommittet); vor jedem `--amend` ist das gestagte Set
   dein eigenes. Die konsumierte Übergabe ist ins Archiv geschoben.
6. Push nur, wenn der Baum ruhig ist (keine fremde Session an denselben Dateien):
   pushe nur deine eigenen Commits; danach gilt `git rev-parse HEAD` ==
   `git rev-parse origin/main`.
