# Mycelium — Pass 2026-10-18: RBN-Manifestation geschlossen

**Linie:** mycelium · **Datum:** 2026-10-18 · **HEAD vor dem Pass:** `af10ee803`

Gemessen, nicht bewertet. Grundgesamtheit: die vier vorgegebenen Zeilen des Linien-Kommandos.

## 0. Die vier Zeilen (zahlentreu)

| Zeile | Ergebnis | exit |
|---|---|---|
| `register_lookup --addressed mycelium` | 2 Blöcke (river-folge223, sensory-folge259), 6 Tokens | 0 |
| `register_lookup --fired mycelium` | `0 fired points` | 0 |
| `xfer list --line mycelium --open` | `.\1~ed07ad98 SENT`, `.\3~7a1c533b SENT` | 0 |
| `ls -1 post/in/*-an-mycelium*.md` | `Datei oder Verzeichnis nicht gefunden` — **0 Treffer** | 2 |

`register_lookup --addressed mycelium --fail` → exit 2, **1 unbeglichen** (`.13~99e00470`, sensory-259 → mycelium).

## 1. Post · Trigger · Transfer

- **Post = 0:** `post/` existiert im Baum nicht (`git ls-files post/*` leer). Der in sensory-folge259:37 zitierte Brief `post/in/2026-10-18-sensory-an-mycelium-searxng-instanz.md` ist ein Phantom (vgl. Netz-Messung 2026-10-18 §1).
- **Trigger = 0:** `--fired` leer.
- Also entscheidet der **Transfer**.

## 2. Transfer-Lage

| Token | Punkt | Ledger | Zustand |
|---|---|---|---|
| eigene `.1~ed07ad98` | ICGEM Grad-Cap (Rat) | SENT → future | offen, nicht von mycelium schließbar |
| eigene `.3~7a1c533b` | Offen-Zählung/Kappe (Rat) | SENT → future | offen, Bau liegt hinter der Wand |
| sensory `.13~99e00470` | SearXNG-Instanz | SENT → mycelium, kein CLOSE | **unbeglichen** (Emitter schließt) |

Die adressierten river-Tokens (`.2`/`.3`), sensory `.6`/`.7`/`.8` sind in ihren Emitter-Ledgern `CLOSED`. `--addressed` listet geschlossene Blöcke weiter — Echo ≠ Erledigung (river-folge223:31).

## 3. Geschlossen: die RBN-Manifestation (Residual zu sensory `.7~15f0a130`)

Netz-Messung 2026-10-18 §3 hat benannt: RBN-Route steht (7/7 206), aber ohne `format` — „eine Manifestation im Sinn der Quellen-Netze ist es nicht". Dieser Pass schließt das Residual.

**Messung (dieser Pass):**
- Range-GET `0-0` auf alle 7 Release-URLs → **7/7 HTTP 206**.
- `curl -r 0-3000000 …/2019-05_RBN.gz` → `file`: `gzip compressed data, was "2019-05-01_RBN.csv"`.
- `gzip -dc … | head -3` (roh): `2019-05-01 00:00:00,DL1NKS,JN49qp,50.2000,11.3167,H,EA5WU,IM99XV,39.9000,-0.0667,H,7001000,31,1,,,CQ,31,,,,,1449.6,45.1904,5.1092` — Komma-Trenner, CRLF, **keine Kopfzeile, 25 Spalten**.

**Bau:** `format csv_gz` in die 7 RBN-Blöcke (`phi/sources.φ:30596–30657`) eingetragen. Der Token ist im Repo belegt: `src/archivar/extract.rs:5282` (`"csv_gz" => read(body) -> gunzip`), Präzedenz `phi/sources.φ:31914` (Kellerman) und 1087 weitere Format-ohne-Feld-Blöcke.

**Nicht getan (ehrlich):** keine `field`/`quantity`-Zeilen. Deren Syntax `field <name> <name> <kernel> <kraft> <einheit>` verlangt `<kernel>`/`<kraft>`; das sind Modellierungs-Token, die kein Byte der CSV nennt (Station-zu-Station-Spots). Sie wären erfunden — das bleibt ein benannter Riss.

**Beleg:** Commit `9309246b6` (nur `phi/sources.φ`, +7 Zeilen) + die Rohmessung oben + `phi/sources.φ:31914` + `src/archivar/extract.rs:5282`. Ledger: kein offener mycelium-Token (sensory `.7` ist bereits `CLOSED`).

## 4. Gemessen, nicht geschlossen: SearXNG `.13~99e00470` (unbeglichen)

- HEAD-Voreinstellung `tools/utils/src/bin/archive_search/net.rs:1802` = `["https://search.lumy.live"]` (Commit `3e27dff0f`).
- **lumy.live intermittent:** 3/3 curl `--max-time 20` → `HTTP 000` (timeout), auch `/` → timeout (12:2x). Ein paralleler Lauf: `HTTP 200`, 24–25 relevante Treffer für „ICGEM gravity field model" (Top: *ICGEM International Center for Global Gravity Field Models*). Also erreichbar, aber nicht stabil.
- **mectov my.id:** `HTTP 200`, aber für „ICGEM gravity field model" und „zzzqqq12345xyz" **0 Treffer**; „test" liefert generische Bing-Treffer (Speedtest). Nicht als alleinige Quelle brauchbar.
- **Der gebaute Arm war stale:** `strings target/release/archive_search` trägt noch `https://sx.xo.st` und `https://search.mectov.my.id` (vor `3e27dff0f`) — die `--searxng`-Läufe dieses Passes trafen mectov, nicht lumy.
- handler: der **Emitter (sensory)** setzt den `CLOSED`-Eintrag (Empfänger kann seinen Eingang nicht quittieren, river-folge223:30).

## 5. Offen — nicht von mycelium zu schließen

- `.1~ed07ad98` **ICGEM Grad-Cap** — Rat `R-2026-10-18-f` (gfc-Block) ist **offen — Operator wählt**.
- `.3~7a1c533b` **Offen-Zählung/Kappe** — Rat `R-2026-10-17-i` 4/4 A **an mycelium**, aber der Bau (`count_open_points` liest `- `-Bullets **und** `### `) lebt in `tools/gate/src/bin/commit_check.rs`, das laut `opencode.json:42` `edit:deny` ist. Der Zähler ist Gate-Code; mycelium kann ihn nicht still umbauen (Verdikt-Grenze). → bleibt offen.
