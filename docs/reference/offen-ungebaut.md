<!--
  title: Geplant und nicht gebaut — die Köder-/Arm-Session (2026-10-21)
  class: ref
  date: 2026-10-21
  status: live
  see-also: docs/reference/koeder-kette.md docs/reference/koeder-formen.md docs/reference/search-channels.md
-->
# Geplant und nicht gebaut

> **Nachtrag 2026-10-21:** Der Bauauftrag mit Staffel, Abnahmekriterien und
> Key-Prüfung steht in `docs/reference/bauauftrag-2026-10-21.md`. Die Zeile
> „Key-gebunden" unten war falsch: `bin/secrets_keys` listet **190 Key-Namen
> mit Wert**, darunter `BABAMUL_API_TOKEN`, `IEEG_USER/PASS`,
> `PRADAN_USER/PASS`, `TNS_API_KEY`, `IRSA_USER/PASS`, `EARTHDATA_EDL_TOKEN`
> und `LASAIR_LSST_TOKEN`. Die 402/429/400 der Sucharme sind Konto/Quota,
> keine fehlenden Schlüssel.

**Frage:** Was wurde in der Session, die die vielen Such-Arme und die
Köder-Kette gebaut hat, **geplant und nicht zu Ende gebaut** — und was ist
stattdessen passiert? Gemessen an den Dokumenten und Commits, nicht an der
Erinnerung.

## 1. Die Köder-Runde (Datei gelöscht, Inhalt geborgen)

Die Runde `state/rat/2026-10-20-rat-weberin-koeder.md` (1288 Zeilen) wurde mit
`7d6eb96` entfernt (Doppel-ID `R-2026-10-20-a`). Ihr **Verdikt A** (3 A / 2 B)
sagte: *Werkzeug zuerst — `bait values`, `weave` und die Herkunftskarte
ausbauen, **dann die acht Key-Arme und die vier Sonden erneut mit Signatur
absuchen**.*

| Teil | Status |
|---|---|
| `bait values`, `weave`, Herkunftskarte ausbauen | **gebaut** (`df464c33a`, `75f6a51fd`, `110f297d6`) |
| danach die acht Key-Arme + vier Sonden erneut mit Signatur absuchen | **nicht gebaut** |

Dazu das gemessene Offen der Runde:

| Gruppe | Arme | Stand |
|---|---|---|
| Key-gebunden | `babamul`, `demeter`, `ieeg`, `ilrs_npt`, `pradan_ch2`, `tns` | Web + Paper ohne admitierten Token |
| Abgelehnt (B) | `dhm_gauge`, `exposome`, `hamqsl`, `modvolc`, `ogimet`, `wwlln` | Alternative trägt die Messung; `dhm_gauge` kostenpflichtig |
| Ohne Block (C) | `erbq`, `fugin` | JMA-Block existiert; NAOJ-Copyright |
| Request-only | Voyager 1/2, Viking 1/2, Mariner 10, Juno pre-EFB | 5/5 Anfragen nach 34 Tagen ohne Antwort |
| **Neue Arme (belegt)** | `ds004100` (CC0), `10.17632/6mmd96j5bg` (CC-BY-4.0), `edges_raw` | **noch nicht gebaut** |

## 2. Die Kette `bait → find → weave`

| Geplant | Status | Beleg |
|---|---|---|
| `find` → `weave` verkabeln (`--weave`, `--from-find`) | **nicht gebaut** | `koeder-kette.md` |
| maschinenlesbare `find`-Ausgabe (`url/header/value/hex/verdict/terms`) | **nicht gebaut** | ebd. |
| `weave --terms` als Pflicht (statt `unset`) | **nicht gebaut** | ebd. |
| `weave`/`leads_merge` über `bin/` erreichbar | **gebaut 2026-10-21** (`e97c13c0b`) |
| `bait` liest FITS-Header + Redirects | **gebaut 2026-10-21** (`6fd3e9945`) |

## 3. Die Anomalie-Suche (`fuenf-funken-anomalie-suche.md`)

Status im Dokument: **„ungebaut — Ideen, keine Entscheidungen"**.

| Funke | Status |
|---|---|
| 1 Verschwinden statt Erscheinen (Forced Photometry + Coverage) | **ungebaut** |
| 2 TE zwischen Quellen (das „große Tier") | **ungebaut** |
| 3 Broker-Differenz (Fink/Lasair/ALeRCE) | **ungebaut** (billig, aber nicht verkettet) |
| 4 Farbfeld-Baseline mit Staub-Abzug | **ungebaut** (Teile im Regal) |
| 5 Gleichzeitigkeit auf der TDB-Uhr | **ungebaut** (Rechnung auf bestehender Kette) |
| Klasse 3 (Anomalie als Beziehung) | **benannt und ungebaut** |

## 4. Die zwölf Quellen (2026-10-21)

| Gruppe | Status |
|---|---|
| Parkes/ZOA, Kitt Peak, Mount Wilson, COSMIC-1, STEREO/EUVI, TRACE, Yohkoh, Super-K | Endpunkt belegt; **Compiler/Ernte fehlt** (nur Parkes im Register) |
| SARAS, REACH | nur Inferenz-/Forecast-Produkte, **kein Rohdatenarm** |
| PRIZM | nur Code (`github.com/PRIZM-Experiment`), **keine Daten** |
| Baikal NT-200 | **keine öffentliche Ablage** (Wayback 2019) |

## 5. Register und Werkzeuge

| Befund | Zahl | Status |
|---|---:|---|
| Baum-Compiler ohne Register-Direktive | 106 | 53 `workflow`, 28 `variant`, **24 `unregistered`**, 1 register |
| `source_validate` weich | 291 | 290 `elev_zero` (globale/Orbit-Felder), 0 hart |
| `channel_ref` als Pflichtglied | — | **nicht gebaut** (Konsens der Köder-Runde) |

## 6. Such-Arme: geplant, aber extern blockiert

| Arm | Befund |
|---|---|
| `--brave` | **402** (Abrechnung) |
| `--tavily` | **432** (Plan-Limit) |
| `--searchapi` / `--serpapi` / `--consensus` | **429** (Quota) |
| `--googlecse` | **400** (Key ungültig) |
| `--gemini` | **429** (Quota) |
| `--dblp` | **Cloudflare 202** (Anubis); Fallback auf OpenAlex mit `via: openalex (dblp fallback)`; live |
| `--lasair` | **gebaut 2026-10-21**: ZTF-IDs gehen an `lasair-ztf.lsst.ac.uk` mit `LASAIR_TOKEN`, LSST-IDs an `api.lasair.lsst.ac.uk` mit `LASAIR_LSST_TOKEN`; live `objectId ZTF20acigmel` |
| `--arxiv` | API **429** (Edge), nach Retry-Ladder wieder 200; bei anhaltendem 429 fällt der Arm auf die HTML-Suche `arxiv.org/search` zurück (`via: arxiv-html`) |
| `--marginalia` | **gebaut 2026-10-21**: HTML-Route `old-search.marginalia.nu` mit Cookie-Jar und `sst`-Gate; live 200, 3 Treffer |
| Mojeek | **Captcha**, 0 organische Treffer — bewusst verworfen |
| Google Books | Arm existiert **nicht**, Tagesquota 429 — erst bauen, wenn gebraucht |
| `--hn`, `--openverse`, `--nasa-images`, `--artic` | **gebaut 2026-10-21**: keyless, alle live 200 |

## 7. Parameter-Arme, gemessen 2026-10-21

| Arm | Aufruf | Messwert |
|---|---|---|
| `--tap` | `endpoint=https://dc.g-vo.org/tap SELECT TOP 2 * FROM rr.resource` | **200**, Zeilen; `/tap` wird zu `/tap/sync`, `columns` gelesen |
| `--sparql` | `endpoint=https://query.wikidata.org/sparql SELECT ?item WHERE { ?item wdt:P31 wd:Q5 } LIMIT 2` | **200**, Zeilen |
| `--oai-pmh` | `endpoint=https://export.arxiv.org/oai2 set=cs` | **200**, Records |
| `--stac` | `endpoint=https://earth-search.aws.element84.com/v1 collection=sentinel-2-l2a bbox=-180,-90,180,90 datetime=2026-10-01/2026-10-02` | **200**, Items; reine Datumsangaben werden zu RFC3339 normalisiert |
| `--erddap` | `server=https://coastwatch.pfeg.noaa.gov/erddap searchFor=solar` | **200**, Datasets; `/erddap` wird nicht mehr verdoppelt |

`--all` hält sie weiterhin zurück, weil sie ohne `endpoint=`/`server=` nicht
sinnvoll sind. Sie sind gebaut und einzeln gemessen.

**BASE-Gate:** `base_gate()` ist nicht mehr inert, wenn `OMEGAFLOW_TMP` fehlt;
es fällt auf `temp_dir()/omegaflow` zurück. Damit gilt der 1,1-s-Abstand und
der Lock systemweit immer.



## 8. Neben der Session beschlossen, nicht gebaut

- `docs/concepts/ausrede-untersuchung.md`: die Brücke „gemessen + Weg
  vorhanden" ist **nicht gebaut**.
- `docs/concepts/die-akteure-im-boden-und-wasser.md`: **W-Phase-M9** —
  entschieden, **nicht gebaut**.
- `docs/concepts/zeugnis.md`: was nicht gebaut ist, ist `absent` — die
  Nullstände sind registriert, nicht gefüllt.

## Was stattdessen passierte

Gebaut wurden in derselben Session: `sniff` (Bait/Wert/NetCDF/FITS/HEALPix),
`bait` (Text/PDF/Bild/FITS), `find` (MATCH/ZWILLING/NAH), `weave`
(Anker-Merge/Konflikt/DAG/Herkunftskarte), `leads_merge`, `archive_search`
mit `--all`/`--verdict`. Die **Arm-Suche selbst** lief breit, aber die
Runde, die sie hätte abschließen sollen, wurde gelöscht; der zweite Teil
von Verdikt A und die drei belegten neuen Arme blieben liegen.
