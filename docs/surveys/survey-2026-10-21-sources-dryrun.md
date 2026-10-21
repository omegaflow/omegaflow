<!--
  title: Sources-Dryrun — v3 auf phi/sources.φ, ohne Schreiben (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 3897f8e251abf34cecc76e8ad437624451f385a2c8d1ee70aed36dfd314ede76
  status: live
  owner: future
  ack: future 2026-10-21T15:20Z
  next: die 113 unbekannten terms kuratieren; dann Chargen-Edit mit source_validate
  orphan-after: none
  see-also: docs/concepts/messzustaende-v3.md state/wartend-zustand.tsv
-->
# Sources-Dryrun — v3 auf `phi/sources.φ`

**Frage.** Wie viele der 2.860 Quellenblöcke lassen sich mit v3 eindeutig
klassifizieren — und wo bleibt eine Lücke? **Dry Run: `phi/sources.φ` wird
nicht verändert.**

**Methode.** Blöcke durch Leerzeile getrennt; je Block `terms` (erster Wert),
`status`, `sha256`. Achse 1: `sha256` vorhanden → `gemessen`, sonst `pending`.
Achse 2: `auth-required` → `wartend`, `status vorschlag` → `offen`, sonst
`offen`. Achse 3: bekannte freie Terme → `frei`; bekannte restriktive →
`LOCK(rechtlich)`; Rest → **unbekannt** (nicht als LOCK gezählt).

## Ergebnis

| Achse | Zelle | Zahl |
|---|---|---:|
| — | Blöcke | **2.860** |
| Zustand | `gemessen` (sha256) | **1.186** |
| Zustand | `pending` | **1.674** |
| Lebenszyklus | `offen` | **2.853** |
| Lebenszyklus | `wartend` (`auth-required`) | **7** |
| Lebenszyklus | `declined` | **0** |
| Zugang | `frei` | **2.684** |
| Zugang | `LOCK(rechtlich)` | **63** |
| Zugang | **unbekannt** | **113** |

**Die 63 LOCK** sind `per-service-terms` (36), `vires` (12), `wdcb` (8) und
`auth-required` (7). Die 398 `CC-BY-NC` sind **frei** — wir sind NC; NC ist eine
Nutzungsbedingung (Attribution, nicht-kommerziell), die wir erfüllen, **kein**
Zugangslock. Der Lock ist **Weitergabe/Consent**, nicht Kommerz.

**Die 7 `auth-required`** sind der einzige echte `wartend`-Fall im Register
(babamul, ieeg, pradan, tns, …). `declined` kommt **0×** vor — die Ablehnungen
liegen offenbar in einer separaten Ablage, nicht in `sources.φ`.

**Die 113 Unbekannten** sind der nächste Kuratierungsschritt. Die häufigsten:

- frei/offen: `CC-BY-3.0`, `MIT`, `ODbL-1.0`, `OGL-UK-3.0`, `OGL-Canada-2.0`,
  `ODC-DbCL-1.0`, `PDDL-1.0`, `Copernicus`, `cadc-open`, `onc-open`,
  `noaa-nodd-open`, `USGov-disclaimer`.
- restriktiv: `CC-BY-NC-SA-4.0` (Non-Commercial + Share-Alike),
  `erg-sc-rules-of-the-road` (Vorabprüfung nötig).
- Sonderfall: `reviewed-2026-10-20` (8×) ist **kein Lizenzterm**, sondern ein
  Prüfdatum — gehört nach `stand`, nicht nach `terms`.

## Was der Dryrun zeigt

0. **Korrektur: NC ist kein Lock.** Der erste Lauf zählte `CC-BY-NC` als
   restriktiv. Das war falsch — wir sind NC. NC-kompatible Terme sind `frei`
   (mit Attribution); der Lock beginnt bei `auth-required`, `no-redistribution`,
   `per-service` oder fehlender Lizenz.
1. **Achse 3 ist die große Achse:** 63 LOCK + 113 unbekannt gegen 2.684 frei.
   Genau hier hat v2 `auth-required` mit `declined` vermischt.
2. **Achse 1 ist überwiegend `pending`:** 1.674 Blöcke ohne `sha256` — der
   Register steht, die Messung fehlt.
3. **`declined` fehlt im Register:** 0 Fälle. Die Ablehnungen sind nicht hier;
   das ist eine Lücke der Ablage, kein v3-Fehler.
4. **`reviewed-…` ist Provenienz**, kein Term — derselbe Fehler wie
   `gemessen <datum>` in den Warten.

## Nächster Schritt

1. Die 113 Unbekannten in eine kuratierte Term-Map (frei | LOCK | Sonderfall).
2. Dann **Chargen-Edit**: `terms`/`status` je Block um die drei Felder ergänzen,
   `source_validate` + `register_sort` nach jeder Charge.
3. Kein Big Bang; die 2.860 Blöcke bleiben bis dahin unverändert.
