<!--
  title: Freie Alternativen für die 17 lizenzgesperrten Arme (2026-10-19)
  class: survey
  date: 2026-10-19
  sha256: 67207dbe22249bbb870880f68c6acad9772f73070e8267812c56f6831a7c93f3
  status: live
  see-also: docs/blatt/blatt-2026-10-19-entscheidung-17-lizenzarme.md docs/reference/search-channels.md
-->
# Freie Alternativen für die 17 lizenzgesperrten Arme

**Gemessen 2026-10-19**, Kanäle: Zenodo, DataCite, OSF, figshare, Dryad, PANGAEA,
Harvard Dataverse, OpenAlex, Internet Archive, GitHub, institutionelle Spiegel.
Je Arm: freie Route (HTTP), Lizenz **auf der Seite** belegt, vorhandener Key (Name, nie Wert).

## Ergebnis

| Klasse | Anzahl | Arme |
|---|---:|---|
| freie Kopie gefunden | 6 | `erbq`, `hamqsl`, `lro_trk`, `modvolc`, `ogimet`, `wwlln` |
| nur mit Key messbar (off-CDN) | 8 | `babamul`, `demeter`, `icrf3`, `ieeg`, `ilrs_npt`, `pradan_ch2`, `substorm`, `tns` |
| keine Route gefunden | 3 | `dhm_gauge`, `exposome`, `fugin` |

**Wichtig:** Bei `modvolc` (FIRMS), `wwlln` (GOES GLM) und `ogimet` (NCEI ISD) ist die
Alternative **dieselbe Messung, nicht dieselbe Datei**. Das ist kein Mirror und kein
Ersatz für den alten Block — es wäre ein **neuer Arm** mit neuem `origin`, neuer Lizenz,
neuem Compiler. `hamqsl` (SWPC) und `erbq` (JMA) sind ebenfalls andere Dateien desselben
Phänomens; `lro_trk` (LOLA-PDS) ist die nächste echte Kopie (NASA-CC0).

## Korrektur am Blatt: `demeter`

Die CDPP-Lizenzseite (`https://www.cdpp.eu/index.php/licence-sipad`, gelesen 2026-10-19)
sagt ausdrücklich: **Weitergabe archivierter Produkte an Dritte bedarf der schriftlichen
Zustimmung.** Die im Blatt vermerkte „Etalab-2.0"-Messung vom 2026-10-11 ließ sich dort
**nicht bestätigen**. `demeter` bleibt damit Key-gebunden (`auth-required`), **nicht**
CDN-fähig.

## `icrf3`

CC0 ist nur über den **NASA-CDIS-CRF-Spiegel** belegt (Earthdata-Login); das
IERS-Original trägt keinen Lizenztext. Bindung also nur über CDDIS mit
`EARTHDATA_EDL_TOKEN`, Lizenz NASA-CC0.

## Keys ≠ Lizenz

`BABAMUL_API_TOKEN`, `CDPP_*`, `IEEG_*`, `PRADAN_*`, `SUPERMAG_USER`, `TNS_*` öffnen
**nur den Zugang**. Einzige Ausnahme: `EARTHDATA_EDL_TOKEN` öffnet CDDIS, wo NASA-CC0
belegt ist. `FIRMS_MAP_KEY` ist optional (das CSV-Archiv ist keyless).

## Was diese Messung **nicht** ist

- **Kein Beweis der Nichtexistenz.** Ein Null-Treffer heißt „in diesen Kanälen an diesem
  Tag nicht gefunden". Die drei „keine Route" sind `absent`, nicht `nein`.
- **Nicht mit dem Sniffer geprüft.** `sniff` prüft **Dateien** (Kopf, Spalten, Hash) —
  es findet keine Alternativen. Es kann einen gefundenen Kandidaten verifizieren, nicht
  einen fehlenden ersetzen.
- **Nicht abschließend.** Eine zweite Runde mit anderen Suchbegriffen (Sprache,
  Akronym, Datenhalter) könnte weitere Kopien finden.

## Folge

1. Die 8 Key-Arme als `terms auth-required` mit Origin-Endpunkt binden (off-CDN).
2. Die 4 „andere Messung" als **neue Arme** vorschlagen, nicht als Ersatz.
3. `demeter` im Blatt korrigieren.

## Browser-Belege (2026-10-19)

Die acht Off-CDN-Arme wurden mit dem Browser auf ihren Lizenzseiten geprüft (JS, PDF,
Consent). **Ergebnis: keine Seite trägt einen im Haus admitierten Lizenz-Token.**
`auth-required` bleibt überall — jetzt gemessen, nicht ungemessen.

| Arm | Seite | wörtlicher Befund | Klasse |
|---|---|---|---|
| `babamul` | `babamul.caltech.edu` | kein Lizenztext, nur Login/API-Token | `auth-required` |
| `demeter` | `cdpp.eu/index.php/licence-sipad` | Custom-Lizenz CNES/CNRS V1.1: Weitergabe nur mit schriftlicher Zustimmung | `auth-required` |
| `icrf3` | `cddis.nasa.gov` + `iers.org` | Earthdata „full and open sharing" **ohne Token**; IERS-Impressum ohne Lizenz | `auth-required` |
| `ieeg` | `ieeg.org` | Registrierungs-Terms mit ausdrücklichem Kopier-/Verteilverbot | `auth-required` |
| `ilrs_npt` | `edc.dgfi.tum.de/en/data/npt/` | kein Lizenztext, nur Register/Login | `auth-required` |
| `pradan_ch2` | `pradan.issdc.gov.in` | zwei Fassungen: „open and free" vs. Kopierverbot/ISRO-Copyright | `auth-required` |
| `substorm` | `supermag.jhuapl.edu/lib/services/` | „fair use", keine Weiterverbreitung, Registrierung | `auth-required` |
| `tns` | `wis-tns.org` | kein Lizenztext, nur „Copyright © 2015" | `auth-required` |

Zwei Zugriffsrisse: `supermag` blockte den Browser (F5/TSPD; Text über den gerenderten
Reader-Abruf), `tns` gab dem Browser **403** (mit Browser-UA per curl gelesen, als
„JS nicht geprüft" markiert).

Die zwei stärksten Sätze: `ieeg` verbietet Kopieren/Verteilen ausdrücklich; `pradan_ch2`
widerspricht sich selbst („open and free" vs. „copying prohibited") — beide Fassungen
zitiert, keine gewählt.
