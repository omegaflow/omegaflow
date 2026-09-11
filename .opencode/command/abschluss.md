---
description: Emits the closing check — the penultimate prompt, measured not asserted.
---

Hast du nur deine eigene Arbeit committet und gepusht — gemessen, nicht
beteuert: listet `git show --stat HEAD` nur deine Dateien, trägt
`git log origin/main..HEAD --name-only` keinen fremden Pfad, ist `git status`
leer (keine eigene Arbeit uncommitted), und gilt nach dem Push
`git rev-parse HEAD` == `git rev-parse origin/main`? Vor jedem `--amend`:
ist das gestagte Set dein eigenes (amend faltet das alte Commit-Inventar ein)?
Hast du die von dir angenommene Übergabe ins Archiv geschoben und, falls weitere
Sessions folgen, eine neue Übergabe erzeugt?
