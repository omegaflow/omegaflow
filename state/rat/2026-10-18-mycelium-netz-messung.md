# Mycelium — Netz-Messung 2026-10-18

**Repo:** `/home/johannes/projects/omegaflow` · **HEAD vor diesem Pass:** `73538f3d9` · **Messzeitpunkt:** 2026-10-18 · **Linie:** mycelium

Gemessen, nicht bewertet. Grundgesamtheit: die vier vorgegebenen Zeilen (`register_lookup --addressed`, `--fired`, `xfer list --line mycelium --open`, `ls post/in/*-an-mycelium*.md`).

## 0. Die vier Zeilen

| Zeile | Ergebnis | exit |
|---|---|---|
| `register_lookup --addressed mycelium` | 2 Blöcke (river-folge223, sensory-folge259), 6 Tokens | 0 |
| `register_lookup --fired mycelium` | `0 fired points` | 0 |
| `xfer list --line mycelium --open` | `.\1~ed07ad98 SENT`, `.\3~7a1c533b SENT` | 0 |
| `ls -1 post/in/*-an-mycelium*.md` | `Datei oder Verzeichnis nicht gefunden` — **0 Treffer** | 2 |

`register_lookup --addressed mycelium --fail` → exit 2, `1 unbeglichen` (`.13~99e00470`), `XFER-NOECHO .13~99e00470 <- sensory-259 to mycelium`.

## 1. Post: der Pfad ist ein Phantom

`post/in/` existiert im Baum nicht. `git ls-files post/*` → leer. `git log --all --diff-filter=D --name-only -- 'post/in/*'` → leer; `post/` kommt in keinem Commit vor.

Damit zeigt der in sensory-folge259:37 und im Register zitierte Brief `post/in/2026-10-18-sensory-an-mycelium-searxng-instanz.md` auf einen Pfad, den dieses Repo nie getragen hat. Post schlägt Trigger schlägt Transfer — hier ist Post **0**, also entscheidet der Transfer.

## 2. Transfer: was offen ist und was nicht

| Token | Punkt | Register-Status | CDN/Messung |
|---|---|---|---|
| river `.2~aa2b1a08` | Weberin-Manifest | echoed | geschlossen, `xfer:river-224-4` → `phi/sources.φ:11842`, 206, `weberin_verdicts.bin`, sha `47defdb3` |
| river `.3~857b157e` | FODG erledigt | echoed | geschlossen, `xfer:river-224-5` → `phi/sources.φ:10389`, 206, `themis_asi_fodg_…bin`, sha `3ad742b3` |
| sensory `.6~1a57ab58` | Voyager-Saturn-Korpus | echoed | geschlossen, `xfer:sensory-259-3` → `phi/sources.φ:11738`, 206, 88 438 190 B |
| sensory `.7~15f0a130` | RBN-Teilkorpus | echoed | **Route ja, Manifestation nein** (siehe 3) |
| sensory `.8~e2c9ef6e` | GRBR C/NOFS | echoed | geschlossen (`9514f4b6c`), `phi/sources.φ:31235`, 206, sha `e606ed21` |
| sensory `.13~99e00470` | SearXNG-Instanz | **unbeglichen / NOECHO** | **widerlegt** (siehe 4) |

`--addressed` listet geschlossene Blöcke weiter — die Beobachtung aus river-folge223:31 (`.11~04ec4cc1`) gilt unverändert. Echo im Register ist deshalb kein Beleg für Erledigung; der Beleg ist der Ledger-Eintrag bzw. der Commit.

## 3. CDN: 206 für alle drei, aber RBN ohne `format`

Range-GET `0-0` auf die Release-URLs (kein Body):

```
206 voyager_saturn.bin          phi/sources.φ:11738
206 2019-03_RBN.gz              phi/sources.φ:30632
206 grbr_cnofs_2012_2014.zip    phi/sources.φ:31235
```

RBN liegt als **7 Monatsblöcke** vor (`phi/sources.φ:30596-30657`: 2018-11 … 2019-05), jeder mit `terms CC-BY-4.0 https://zenodo.org/records/10673982`, `origin …/api/records/10673982/files/<monat>_RBN.gz/content`, `harvest raw_zenodo`, `sha256` und `compiler tools/harvest/src/bin/raw_mirror.rs` — aber **ohne `format`-Zeile und mit 0 `field`/`quantity`-Zeilen**. Die CDN-Route ist da (7/7 206); eine Manifestation im Sinn der Quellen-Netze ist es nicht. Der Transfer verlangte „CDN-Route prüfen" — geprüft, Route vorhanden, Rest ist `format`/Felder.

Nebenriss: sensory-folge259:36 nennt `zenodo.21380427`, `phi/sources.φ:31237` und Commit `9514f4b6c` nennen `21380428`.

## 4. SearXNG: die Lage stimmt nicht — der Punkt ist geschlossen

Der Arm ist gebaut (`tools/utils/src/bin/archive_search.rs:457`, `net.rs:4177`), und `format=json` wird verlangt (`net.rs:1821`). Die Voreinstellung trug zwei Instanzen (`net.rs:1802`):

```
sx.xo.st                  → 000  curl (35) TLS: tlsv1 unrecognized name
search.mectov.my.id       → 200  application/json, 5 600 B, {"query":"test","results":[…]
```

„Keine öffentliche JSON-Instanz" ist damit **widerlegt**: die zweite Voreinstellung antwortet mit echtem JSON. Die erste ist TLS-tot. Erledigt: tote Voreinstellung entfernt (`net.rs:1802`), `cargo check --manifest-path tools/utils/Cargo.toml --bin archive_search` grün.

## 5. Offen nach diesem Pass

- sensory `.7`: `format`/Felder für die 7 RBN-Blöcke (Route steht).
- sensory `.13`: mit diesem Commit beantwortet; der Emitter (sensory) setzt den `CLOSED`-Eintrag — der Empfänger kann seinen Eingang nicht quittieren (river-folge223:30).
- eigene `.1~ed07ad98` / `.3~7a1c533b`: `SENT` an future, `coverage` exit 2; `.1` ist bei future unbeglichen.
