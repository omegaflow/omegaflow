---
description: Instructs the session to commit only its own uncommitted work.
---

Committe jetzt nur deine eigene uncommittete Arbeit:

1. `git status --short` — trenne deine Dateien von fremden.
2. Bei geteilten Dateien lies `git diff` und stage nur deine eigenen Hunks.
3. `git add` nur deine Dateien/Hunks — fremde uncommittete Arbeit (andere
   Sessions) fasst du nicht an: kein Revert, kein Re-Stage, kein Überschreiben.
4. Committe nur deinen Teil; danach listet `git show --stat HEAD` nur deine
   Dateien.
5. Gepusht wird erst, wenn der Baum ruhig ist und der Operator das Wort gibt.
