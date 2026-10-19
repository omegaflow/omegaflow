# Mycelium — Pass 2026-10-18: SearXNG verifiziert, Register grün

**Linie:** mycelium · **Datum:** 2026-10-18 · **HEAD vor dem Pass:** `a798ef376`

Gemessen, nicht bewertet. Grundgesamtheit: die vier vorgegebenen Zeilen des Linien-Kommandos.

## 0. Die vier Zeilen (zahlentreu)

| Zeile | Ergebnis | exit |
|---|---|---|
| `register_lookup --addressed mycelium` | 2 Blöcke (river-folge223, sensory-folge259), 6 Tokens; `--fail` → **0 unbeglichen** | 0 |
| `register_lookup --fired mycelium` | `0 fired points` | 0 |
| `xfer list --line mycelium --open` | `.\1~ed07ad98 SENT`, `.\3~7a1c533b SENT` | 0 |
| `ls -1 post/in/*-an-mycelium*.md` | `Datei oder Verzeichnis nicht gefunden` — **0 Treffer** | 2 |
| `xfer coverage --line mycelium` | emitted 7 · disposed 5 · **open 2** | 2 |

Die Zahl ist im Fenster dieses Passes von **1 unbeglichen → 0** gefallen: der Emitter hat `.13` geschlossen (siehe §3).

## 1. Post · Trigger · Transfer

- **Post = 0 im Baum.** `post/` wird von diesem Repo nicht getragen (`git ls-files 'post/*'` leer). Die zitierten Briefe liegen im Nachbar-Repo `omegaflow-work/post/in/` (`ls -1` dort: `2026-10-17-sensory-haertefaelle-survey-an-mycelium.md`, `2026-10-18-sensory-an-mycelium-searxng-instanz.md`) — Pfad-Riss, bereits benannt (`2026-10-18-mycelium-netz-messung.md` §1).
- **Trigger = 0.**
- Also entscheidet der **Transfer**: eingehend 0 unbeglichen; die zwei offenen Token sind **eigene** (→ future).

## 2. Delegation und Prüfung

Zwei freie `general`-Subagenten (`opencode/deepseek-v4.1-flash-free`), done-when vom Pass:

- **SearXNG-Sweep (12 Instanzen × 2 Abfragen).** Nur `https://search.lumy.live` liefert gültiges JSON mit relevanten Treffern (HTTP 200 `application/json`, 25 Treffer, Top `https://icgem.gfz.de/home` „ICGEM International Center for Global Gravity Field Models"); `search.mectov.my.id` liefert 0/0 (abfrageunabhängig leer). Die übrigen: 5×429, 3×Anti-Bot-HTML, 1×403, 1×TLS-Fehler. **Verdikt: die Voreinstellung ist korrekt.**
- **Status-Aufnahme (6 Kommandos).** Alle Zahlen wörtlich übertragen.

Eigene Gegenmessung (dieser Pass): `bin/archive_search --searxng "ICGEM gravity field model"` → exit 0, 18 `[searxng]`-Zeilen (Engines yandex/mwmbl); `curl` auf `https://search.lumy.live/search?q=test&format=json` → `HTTP 200 application/json` (72007 B).

## 3. Geschlossen: SearXNG `.13~99e00470` (eingehend) — verifiziert

Der Post (`2026-10-18-sensory-an-mycelium-searxng-instanz.md`) wies mycelium die SearXNG-Infra zu. Der Emitter hat den Token geschlossen:

- Ledger `state/zustand/ledger/sensory-259/ses_ec618129fffeLFawr4V3pQKpOl.log:31`:
  `2026-10-18T10:49:01Z CLOSED xfer:sensory-259.ses_ec618129fffeLFawr4V3pQKpOl.13~99e00470 by=sensory-259.ses_ec618129fffeLFawr4V3pQKpOl ref=3e27dff0f`
- Emitter-Commit `a798ef376` (sensory Nachtrag-Pass).
- Code-Beleg `tools/utils/src/bin/archive_search/net.rs:1802`:
  `const SEARXNG_INSTANCES: &[&str] = &["https://search.lumy.live"];` (Commit `3e27dff0f`).
- Register-Prüfung: `register_lookup --addressed mycelium --fail` → **0 unbeglichen**.

**Beleg:** die Ledger-Zeile (oben) + `a798ef376` + `3e27dff0f` + die Messung §2.

## 4. Geschlossen: der eigene Handover-Kopf trug einen veralteten Selbsthash

`docs/handover/handover-2026-10-16-mycelium-folge361.md` trug auf HEAD (`a798ef376`)
`sha256: ad194d51762a3640221d6909da91372336691d8097f66a6a85ce315bd468777e`. `bin/omega_sh sha`
über den header-freien Rumpf misst `c837fb294df7f2d469636cbccd46a0d029692fc762d771b137dcf34fbdc9cd75`.
Der Arbeitsbaum trug bereits den richtigen Wert; dieser Pass verifiziert und committet ihn.

**Beleg:** `bin/omega_sh sha <f>` → `c837fb29…` + dieser Commit.

## 5. Offen — nicht von mycelium zu schließen

- `.1~ed07ad98` **ICGEM Grad-Cap** — Rat `R-2026-10-18-f` **offen** (Operator wählt Cap/Epoche/`ttl`/Bootstrap).
- `.3~7a1c533b` **Offen-Zählung/Kappe** — Rat `R-2026-10-17-i` 4/4 A **an mycelium**, aber der Bau (`count_open_points`) ist Gate-Code: die Verdikt-Grenze `2026-10-17-rat-offen-stand.md:62` sagt „commit_check ist Gate-Code und wird nicht still umgebaut"; `opencode.json` setzt `edit:deny` auf `tools/gate/**`. → bleibt offen.
- **Pfad-Riss:** die `post/in/…`-Briefe liegen in `omegaflow-work/post/in/`, nicht im Register-Repo.

## 6. Bilanz

Post 0 · Trigger 0 · Transfer: eingehend 0 unbeglichen, eigene 2 offen. Zwei belegte Punkte geschlossen (SearXNG `.13` verifiziert; der stale Kopf-Selbsthash). Kein Ledger-Schluss von mycelium gesetzt — `.13` schließt der Emitter; die zwei eigenen Token warten auf Operator/Rat.
