<!--
  title: Auftrag — PII-History-Rewrite (private Operator-Adresse)
  class: auftrag
  date: 2026-09-20
  sha256: a13c476bcb66cae22d896b63c5a79f9d22520a0bb2439c2695236d35704e061c
  status: live
  see-also: docs/zustand/external-state.md .gitignore
-->
# Auftrag: PII-History-Rewrite — die private Operator-Adresse aus der Historie

## Befund (gemessen 2026-09-20)

Die private Operator-Adresse steht in **1.472 Commits** (Autor/Committer,
2026-09-03 … 2026-09-15) und in älteren Blobs von fünf Docs. Commit `99c6500d`
redigierte sie in HEAD (fünf Dateien → `<operator-adresse>`, Header-`sha256`
neu); die **Historie trägt sie weiter**. Das GitHub-GC-Ticket #4761801 hängt
daran. Kein anderes PII gefunden: kein IBAN-Muster, keine Adresse
(Corneliweg/Dachsberg/79875) im Baum, keine Bank-/Gesundheitsdaten.

## Blocker (gemessen)

- `git filter-repo` ist nicht installiert; Python ist im Projekt verboten (filter-repo ist Python).
- `git push --force`/`-f` ist in `opencode.json` verweigert.
- Geteilter Arbeitsbaum (fremde uncommittete Arbeit) → Rewrite nur im frischen Klon.

## Prozedur (frischer Klon — destruktiv, Operator-Wort nötig)

```bash
git clone --no-local --mirror https://github.com/omegaflow/omegaflow.git /tmp/of-rewrite
cd /tmp/of-rewrite
export PRIVATE_ADDR='<die private Operator-Adresse>'
git filter-branch --env-filter "
  [ \"\$GIT_AUTHOR_EMAIL\"    = \"\$PRIVATE_ADDR\" ] && export GIT_AUTHOR_EMAIL=code@omegaflow.space
  [ \"\$GIT_COMMITTER_EMAIL\" = \"\$PRIVATE_ADDR\" ] && export GIT_COMMITTER_EMAIL=code@omegaflow.space
" -- --all
git filter-branch --tree-filter \
  "git grep -lI \"\$PRIVATE_ADDR\" | xargs -r sed -i \"s/\$PRIVATE_ADDR/<operator-adresse>/g\"" -- --all
git for-each-ref --format='%(refname)' refs/original/ | xargs -n1 git update-ref -d
git reflog expire --expire=now --all && git gc --prune=now --aggressive
git push --force --all && git push --force --tags
```

Hinweis: `--tree-filter` ist über ~5.700 Commits sehr langsam. Schneller, falls
erlaubt: `git filter-repo` **außerhalb** des Repos installieren.

## Schritt

Operator-Wort für den destruktiven Lauf; danach ziehen alle Klone/Linien neu
(`git fetch --all` + `git reset --hard origin/main`). Der lokale `.mailmap`
(gitignored) ist nur Anzeige-Mapping, kein Ersatz für den Rewrite.
