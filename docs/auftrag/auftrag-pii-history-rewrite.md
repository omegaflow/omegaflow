<!--
  title: Auftrag — PII-History-Rewrite (private Operator-Adresse)
  class: auftrag
  date: 2026-09-20
  sha256: 72bc6f5659a4f0d533525af90415859b637df09b4bdc8e423b183e52d304220f
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

## Nachtrag (2026-09-25, Sensory-Folge 163) — FR945-Geräte-MAC

Neues PII gefunden: die MAC des persönlichen Garmin FR945. Der Wert steht lokal
in `.secrets.local` als `FR945_MAC` und in **keinem Dokument**. Sie stand in 10
archivierten Sensory-Übergaben (`docs/handover/archiv/handover-2026-09-23-sensory-folge153.md`
… `handover-2026-09-25-sensory-folge162.md`, 15 Stellen) und in der Test-Fixture
`src/archivar/ble.rs` (8 Stellen). Der laufende Baum ist seit 2026-09-25
MAC-frei: Fixture auf den Platzhalter `AA:BB:CC:DD:EE:FF` umgestellt,
`handover-2026-09-25-sensory-folge163.md` trägt die Kennung nicht mehr; die
**Historie und die 10 archivierten Blobs tragen sie weiter** — sie fallen in
denselben Rewrite.

Erweiterung des `--tree-filter` (Wert aus `.secrets.local` exportieren, nie ins
Skript schreiben):

```bash
export FR945_MAC='<aus .secrets.local>'
FR945_MAC_US=$(printf '%s' "$FR945_MAC" | tr ':' '_')
git grep -lI "$FR945_MAC"    | xargs -r sed -i "s/$FR945_MAC/AA:BB:CC:DD:EE:FF/g"
git grep -lI "$FR945_MAC_US" | xargs -r sed -i "s/$FR945_MAC_US/AA_BB_CC_DD_EE_FF/g"
```

Bis der Lauf steht, ist die archivierte MAC in HEAD weiter öffentlich — gemessen
und benannt, nicht verdeckt.

Offen (Architektur): eine Geräte-MAC ist eine neue PII-Klasse — das Gate-Vokabular
(`src/gate/commit_gate_vocab.json` `pii`) kennt nur Mail-Domains und Home-Pfade.
Ob ein MAC-Muster Gate-Fixture wird (und wie der Platzhalter `AA:BB:CC:DD:EE:FF`
davon ausgenommen bleibt), braucht das Architektur-Wort.
