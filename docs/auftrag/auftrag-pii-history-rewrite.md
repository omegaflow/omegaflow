<!--
  title: Auftrag — PII-History-Rewrite (private Operator-Adresse)
  class: auftrag
  date: 2026-09-28
  sha256: c86f251f20a6ce94ac25cefa328a737f69552238c528a1dbdaa91e15f4985990
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
(<operator-adresse>) im Baum, keine Bank-/Gesundheitsdaten.

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

## Nachtrag (2026-09-25, Sensory-Folge 163) — [redacted]-Geräte-MAC

Neues PII gefunden: die MAC des persönlichen Garmin [redacted]. Der Wert steht lokal
in `.secrets.local` als `[redacted]_MAC` und in **keinem Dokument**. Sie stand in 10
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
export [redacted]_MAC='<aus .secrets.local>'
[redacted]_MAC_US=$(printf '%s' "$[redacted]_MAC" | tr ':' '_')
git grep -lI "$[redacted]_MAC"    | xargs -r sed -i "s/$[redacted]_MAC/AA:BB:CC:DD:EE:FF/g"
git grep -lI "$[redacted]_MAC_US" | xargs -r sed -i "s/$[redacted]_MAC_US/AA_BB_CC_DD_EE_FF/g"
```

Bis der Lauf steht, ist die archivierte MAC in HEAD weiter öffentlich — gemessen
und benannt, nicht verdeckt.

Gelöst (2026-09-25, Operator-Wort „platzhalter ausschiessen"): eine Geräte-MAC ist
eine PII-Klasse. Das Gate (`src/gate/commit_gate.rs` `mac_address_hit`) flaggt
MAC-Muster (`xx:xx:xx:xx:xx:xx`, Trenner `:` oder `_`) als `pii`; der Platzhalter
`AA:BB:CC:DD:EE:FF` / `AA_BB_CC_DD_EE_FF` ist ausgenommen. Fixtures
`pii_device_mac`/`pii_device_mac_path` (blockiert) und
`pii_device_mac_placeholder`/`pii_device_mac_underscore_placeholder` (passieren),
Tests im selben Atom; `cargo check --tests` lokal 0/0, Lauf über CI.
