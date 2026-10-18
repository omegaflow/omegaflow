# Drei Härtefall-Anfragen — vorbereitet, nicht gesendet (2026-10-18)

**Status:** `vorbereitet, Operator-Sendewort nötig` · **Nicht gesendet. Kein Commit.**
**Bezug:** `docs/surveys/survey-2026-10-17-haertefaelle-offene-routen.md` (RBN-Lizenz · CEIC D18000 · DOSIS-3D/EuTEF DOSTEL)
**Regel:** Jede Antwort ist eine Zulieferung und wird am Baum gemessen (Mountain), nie roh geglaubt. Keine Zusage über Datenrechte in diesen Texten.

---

## 0 — Nachmessung 2026-10-18 (die tragenden Fakten)

| Punkt | Messung | Ergebnis |
|---|---|---|
| RBN Lizenzseiten | `GET` `https://www.reversebeacon.net/{about,terms,privacy,legal,copyright}` | **5/5 HTTP 404** |
| RBN einziger Lizenzsatz | `GET https://www.reversebeacon.net/raw_data/` | **200**, wortgleich: „Data from the RBN are freely available for study and analysis." + Bitte, Ideen/Ergebnisse zu teilen; **kein** Redistributions-Grant, **keine** Lizenz genannt |
| RBN Kontaktkanal | `GET https://www.reversebeacon.net/contact` | **200**; Webformular (kein E-Mail-Link veröffentlicht) |
| RBN Release-Route | `Range 0-0` auf 7 Release-URLs (`2018-11` … `2019-05`) | **7/7 HTTP 206** |
| CEIC DOIs | `GET https://doi.org/10.12080/nedc.gc_zl.{ds0005.2026,ds0006.2026,ds00001.2024,ds0001.2026}` | **4/4 HTTP 404**, Body: „This DOI cannot be found in the DOI System" |
| CEIC DataCite | `GET https://api.datacite.org/dois?query=prefix:10.12080` | **0** registrierte DOIs |
| CEIC Katalogseite | `GET https://data.earthquake.cn/D18000/index.html` | **200** (Katalog sichtbar) |
| CEIC Order | `GET …/datashare/report.shtml?PAGEID=normalordercheck&dtid=` | **302 → `login.jsp`** (final 200): Auth-Gate, kein offener Download |
| DOSIS 3D | HREDA TAP `v_doi_investigation`, `investigation_doi='10.57780/esa-znu2ukt'` | `data_available=false`, `ancillary_data_available=false` |
| EuTEF DOSTEL | HREDA TAP `v_doi_investigation`, `investigation_doi='10.57780/esa-8hfqjak'` | `data_available=false`, `ancillary_data_available=false` |
| DOSIS-Familie | HREDA TAP, `investigation_name LIKE '%DOSIS%'` | auch `EXPOSE-E DOSIS/DOBIES`, `EXPOSE-R DOSIS`, `DOSIS/DOBIES` = **false** |
| HREDA Kontakt | `GET https://www.cosmos.esa.int/web/hreda/register` | `mailto:hreda.info@sdc.upm.es` („registration support") |

**TAP-Endpunkt (für die Nachprüfung):** `https://hreda.esac.esa.int/hreda-sl-tap/tap/sync`
`REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT investigation_doi,data_available,ancillary_data_available FROM v_doi_investigation WHERE investigation_doi='10.57780/esa-znu2ukt'`
(Die DOI-Spalte liegt in `v_doi_investigation`; `v_dataset.dataset_doi` ist leer, `v_dataset` hat 163 Zeilen.)

---

## 1 — RBN: Lizenz / Redistribution

**Kanal:** Webformular, keine E-Mail-Adresse veröffentlicht.
**URL:** `https://www.reversebeacon.net/contact`
**Betreff:** kein Betrefffeld — erste Zeile der Nachricht trägt den Betreff.
**Erste Zeile:** `License and redistribution terms for RBN spot data`

**Formularfelder (exakt):**
- `name` (Text, max. 40)
- `callsign` (Text, max. 10) — **Operator muss Rufzeichen liefern oder `N/A` freigeben**
- `email` (Text, max. 254)
- `message` (Textarea, min. 10)
- `recaptcha_response` (hidden, reCAPTCHA v3 — automatisch)
- Submit: `send it` (Button-Name `send`)

**Nachricht (exakt):**

```text
License and redistribution terms for RBN spot data

Hello RBN team,

I use the Reverse Beacon Network daily spot archives (for example the
rbn_history/YYYYMMDD.zip files and the 2018-11 to 2019-05 CSV.GZ extracts)
for a small, independent, non-commercial research project on ionospheric
and propagation signals.

I have read https://www.reversebeacon.net/raw_data/ — "Data from the RBN are
freely available for study and analysis." I could not find a licence or terms
page: /about, /terms, /privacy, /legal and /copyright all return HTTP 404
(measured 2026-10-18).

Two questions:

1. Is there a licence for the spot data (for example CC BY, CC0, or public
   domain), and may I redistribute the raw CSV files or derived extracts
   non-commercially, with attribution to RBN and its skimmer operators?

2. If redistribution is not granted, may I publish only aggregated or derived
   results (for example monthly statistics) with attribution, and keep the raw
   files internal?

If the answer differs by data source (RBN vs. PSKReporter/WSPRNet), please say
so. I will follow your terms and cite RBN as you require. A short reply is
enough, and I am happy for the answer to be posted publicly.

Kind regards,
[NAME]
[EMAIL]
[CALLSIGN or "N/A"]
```

**Operator-Verdikt (exakt, vor dem Senden einsetzen):**

> Sendefreigabe RBN: Ich genehmige die Übermittlung des unter §1 stehenden Texts über `https://www.reversebeacon.net/contact` mit `name=«[NAME]»`, `callsign=«[CALLSIGN oder N/A]»`, `email=«[EMAIL]»`. Ich bestätige, dass die Angaben zu meiner Person und der nicht-kommerzielle Zweck wahr sind. Ich erwarte keine Zusage über Redistributionsrechte; die Antwort wird gemessen.

---

## 2 — CEIC: D18000 (Schwere), Auslandszugang + DOI-Status

**An:** `datashare@seis.ac.cn`
**Cc:** `dcogcd@163.com`
**Betreff:** `D18000 gravity data — foreign access, DOI status and redistribution terms`

**Hinweis zum Versand:** Falls der Cc an `dcogcd@163.com` abgewiesen wird (Consumer-Mailbox/Spamfilter), denselben Text separat an `dcogcd@163.com` senden. Keine zweite Fassung mit anderen Zusagen.

**Body (exakt):**

```text
D18000 gravity data — foreign access, DOI status and redistribution terms

Hello,

I am an independent, non-commercial foreign researcher. I have read the
D18000 pages for the gravity datasets (continuous 1998–2015, about 300 GB;
absolute 2009–2015, about 22 GB) at
https://data.earthquake.cn/D18000/index.html. Three questions decide whether
I can apply at all, and I would like to ask them before registering.

1. Real-name verification (实名认证): can an independent, non-commercial
   foreign researcher complete it with a passport, or is a Chinese ID document
   required? Which fields and documents exactly, and is there a nationality
   restriction on the application?

2. Access and DOI: the order endpoint
   https://data.earthquake.cn/datashare/report.shtml?PAGEID=normalordercheck&dtid=
   redirects to login.jsp. After verification, are the D18000 datasets
   (continuous and absolute) available to a foreign applicant? The DOI suffixes
   we have seen — 10.12080/nedc.gc_zl.ds0005.2026, ds0006.2026, ds00001.2024
   and ds0001.2026 — currently resolve to "This DOI cannot be found in the DOI
   System" (measured 2026-10-18), and DataCite lists no records under prefix
   10.12080. Which DOI should be cited for D18000, and is a 2026 DOI still to
   be minted?

3. Terms: are non-commercial research use and publication of derived results
   permitted? May we redistribute the raw data, or only derived results? Which
   citation and acknowledgement do you require?

My purpose: a small, independent, non-commercial project on the time-varying
gravity field. I use multiple stations over the full available span, compute
the gravity residual after tide and drift correction, and test whether it
carries information about other geophysical series. I will follow your terms
and cite the data as you require.

您好，我是一名独立的外国研究者，希望申请 D18000 重力测量数据。
请问实名认证是否接受护照？外国申请者是否受限？另外，D18000 当前的
正式 DOI 是哪一个？谢谢。

Kind regards,
[NAME]
[EMAIL]
```

**Operator-Verdikt (exakt, vor dem Senden einsetzen):**

> Sendefreigabe CEIC: Ich genehmige den Versand des unter §2 stehenden Texts von meiner Adresse an `datashare@seis.ac.cn` mit Cc an `dcogcd@163.com`. Ich bestätige, dass ich unabhängiger, nicht-kommerzieller ausländischer Forscher bin und dass die Angaben zu meiner Person wahr sind. Ich erwarte keine Zusage über Datenrechte; die Antwort wird gemessen.

---

## 3 — DOSIS-3D / EuTEF DOSTEL: ESA-HREDA (LDAP)

**An:** `hreda.info@sdc.upm.es`
**Betreff:** `Data access request — DOSIS 3D and EuTEF DOSTEL (10.57780/esa-znu2ukt, 10.57780/esa-8hfqjak)`

**Alternativer Kanal (nur falls E-Mail nicht ankommt):** ESA SITU Service Desk
`https://support.cosmos.esa.int/situ-service-desk/servicedesk/customer/portal/1`
(HREDA-Kategorie wählen; derselbe Text).

**Body (exakt):**

```text
Data access request — DOSIS 3D and EuTEF DOSTEL
(10.57780/esa-znu2ukt, 10.57780/esa-8hfqjak)

Dear HREDA support,

I am an independent, non-commercial researcher. I would like to request access
to the DOSIS 3D and EuTEF DOSTEL data in HREDA.

The public metadata currently reports data_available=false and
ancillary_data_available=false for both records:

- DOSIS 3D — DOI 10.57780/esa-znu2ukt, investigation DOSIS_3D_ISS,
  2012-04-28 to 2024-03-30
- EuTEF DOSTEL — DOI 10.57780/esa-8hfqjak, investigation EuTEF_DOSTEL_ISS,
  2007-10-13 to 2009-10-11

Measured via the HREDA TAP service (v_doi_investigation) on 2026-10-18. The
download route also returns HTTP 403 for anonymous users.

Three questions:

1. Are data files (for example the DOSIS-3D dose tables/XLSX and DOSTEL
   spectra) available on request, or does data_available=false mean that no
   files exist in HREDA?

2. If files exist: how can an independent researcher obtain them — HREDA
   account plus data-owner approval, ESA LDAP, or a direct request to the PI?
   Please name the responsible data owner/PI and the exact access procedure.

3. Which terms apply to non-commercial research use, publication of derived
   results, and (if applicable) redistribution? If redistribution is not
   permitted, I will keep the raw files internal and publish only derived
   results with the required acknowledgement.

My use is small and specific: ISS radiation dose and dose-equivalent data as an
independent cross-check against open dosimetry records (for example CDAWeb
TEPC). I will follow your terms and cite the data as you require.

Kind regards,
[NAME]
[EMAIL]
[affiliation, if required]
```

**Operator-Verdikt (exakt, vor dem Senden einsetzen):**

> Sendefreigabe DOSIS: Ich genehmige den Versand des unter §3 stehenden Texts von meiner Adresse an `hreda.info@sdc.upm.es`. Ich bestätige, dass ich unabhängiger, nicht-kommerzieller Forscher bin und dass die Angaben zu meiner Person wahr sind. Ich erwarte keine Zusage über Datenrechte; die Antwort wird gemessen.

---

## 4 — Was der Operator liefern muss (eine Liste)

1. **`[NAME]`** — wahrer Name (in allen drei Texten).
2. **`[EMAIL]`** — Absenderadresse (in allen drei Texten; RBN-Formularfeld).
3. **`[CALLSIGN oder N/A]`** — RBN-Formularfeld `callsign`, max. 10 Zeichen. Kein Rufzeichen im Baum gemessen; kein Feld ohne wahren Wert.
4. **`[affiliation, if required]`** — nur DOSIS, nur falls HREDA eine Einrichtung verlangt; sonst leer lassen.
5. **Drei Sendeworte** (die exakten Sätze aus §1–§3) — oder ein Sammelwort, das alle drei Sätze ausdrücklich nennt.

**Nicht nötig / nicht behauptet:** eine Registrierung, ein Konto, eine Unterschrift oder eine Zahlung. Keine der drei Anfragen ist ein Antrag mit Zusage; alle drei sind Fragen. Die Antworten sind Zulieferungen, keine Freigaben.

---

## 4 — Alternativrouten (`archive_search --all`, 2026-10-18)

- **RBN — Anfrage optional.** Keyless **CC-BY-4.0** auf Zenodo: `10.5281/zenodo.13362701`
  (Amateur Radio Spot Data 2018-11…2019-05, Monats-TARs RBN/WSPRNet/PSKReporter),
  `2483022` (HamSCI 2017 SEQP, aggregiertes RBN-CSV), `5799649`, `1067790`. Der volle
  RBN-Stream bleibt ohne Grant; ein Korpus ist ohne Anfrage erreichbar.
- **CEIC D18000 — Alternativen, nicht das Produkt.** CEA/CENC-abgeleitete
  **CC-BY-4.0**-Wellenformen: `23103204` (Weiyuan SAC), `13978646`
  (CENC-Katalog+Wellenformen), `10499328` (CNSN). Das exakte D18000 bleibt auth-gated.
- **DOSIS — keine offene Route gefunden.** `--all` liefert nur Paper/ORCID
  (DLR-Dosimetrie), keine Daten. Anfrage bleibt der Weg.
