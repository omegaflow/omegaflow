<!--
  title: Auftrag — Roh-Tracking-Anfragen (request-only) — fünf offene Sonden-Daten
  class: auftrag
  date: 2026-09-16
  sha256: ba23086e79987af1d6717591881051bf855ab05558019d17b08c62d6960fd7d9
  status: live
  see-also: phi/blocked_sources.φ
-->
# Auftrag: die request-only-Rohdaten der fünf offenen Sonden — Anfrage-Vorlagen

## Zweck

Neun Sonden tragen offenes closed-loop-ODF (TRK-2-34) und sind als Ernte-Duty in
`phi/blocked_sources.φ` registriert. Fünf Sonden-Rohdaten bleiben **request-only** —
kein anonymer Pfad; die formelle Anfrage ist der einzige Weg. Dieser Auftrag legt
die Anfrage-Vorlagen an (einreichbar durch einen Menschen). Keine Zahl wird hier
erfunden; jede Anfrage trägt ihren gemessenen Ort (2026-09-15/16, curl +
archive_search + NSSDCA Master Catalog).

## Die fünf offenen Posten (gemessen)

| Sonde | Produkt | Ort (gemessen 2026-09-16) | Status |
|---|---|---|---|
| Voyager 1/2 closed-loop | ODF/TDF/TRK-2-34 | NSSDC `PSNO-00007` (SDDPT 77-084A-0071, VGR_8201–8245; „Ready for Offline Distribution") | request-only |
| Mariner 10 | 7-Track-Bänder (762 Tapes, 6→8-Bit gepolstert) | NSSDC `PSCM-00009` (SDDPT 73-085A-0008, MVM_1001–1007; „Ready for Offline Distribution") | request-only |
| Viking 1/2 | Doppler+Range (Tracking) | NSSDC `PSPG-00011` („Data Identified but not Received") + `PSPG-00457` (SDDPT 75_075A_0008, VO_6101–6110; „Ready for Offline Distribution") | request-only/absent |
| Cassini closed-loop | TNF/ODF (TRK-2-34/2-18) | JPL-NAV (kein öffentlicher Kanal; PDS trägt nur open-loop + derived) | request-only |
| Juno Earth-Flyby 2013-10-09 | pre-EFB Merged-ODF | JPL-NAV — Vorlage in `docs/auftrag/archiv/auftrag-flyby-doppler-rohdaten.md` | request-only |

## Ansprechpartner (gemessen 2026-09-16)

Quelle: NSSDCA Master Catalog, je Datensatzseite (`nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=<ID>`)
und `nssdc.gsfc.nasa.gov/about/about_cruso.html` (beide 200).

- **Bestellkanal (CRUSO):** Coordinated Request User Support Office, Code 690.1,
  NASA Space Science Data Coordinated Archive, NASA Goddard Space Flight Center,
  Greenbelt, Maryland 20771 USA — `gsfc-dl-nssdca-request@mail.nasa.gov`.
- **`PSNO-00007` (Voyager):** General Contact (der Katalogkontakt für `PSNO-00007`),
  NASA JPL; Kommentare an den Katalog-Generalkontakt.
- **`PSCM-00009` (Mariner 10):** Data Provider (der Data Provider, `PSCM-00009`),
  NASA JPL; General Contact (der Katalogkontakt für `PSCM-00009`), NASA JPL.
- **`PSPG-00011` (Viking Tracking):** General Contact (keine E-Mail im Katalog),
  NASA Langley Research Center; Kommentare an CRUSO.
- **`PSPG-00457` (Viking SDDPT):** Data Provider (der Data Provider, `PSPG-00457`),
  NASA JPL; General Contact (der Katalogkontakt für `PSPG-00457`), NASA JPL.

Der Katalogtext verlangt je Datensatz den „NSSDC contact person listed"; der
formale Bestellweg ist CRUSO, der benannte Kontakt ist der Data Provider.

## Versand (2026-09-16)

Die drei NSSDC-Anfragen und die zwei JPL-NAV-Anfragen wurden am 2026-09-16 vom
Operator aus Proton (`<operator-adresse>`) gesendet; Kopien liegen im
`state/mail/mail_ledger.φ` (2026-09-16 13:47–13:55Z). Antworten offen.

## Vorlage 1a — NSSDC-SDDPT-Antrag: Voyager 1/2 (closed-loop)

> **To:** CRUSO `gsfc-dl-nssdca-request@mail.nasa.gov` (cc der Katalog-Generalkontakt,
> NASA JPL — der Katalogkontakt für `PSNO-00007`)
>
> **Subject: Data request — Voyager 1/2 raw closed-loop two-way Doppler (NSSDC PSNO-00007)**
>
> We request the raw closed-loop radio-science tracking data held at NSSDC under
> `PSNO-00007`, for independent re-derivation of the residual against an N-body
> model (academic, A=A — no fabricated value; the publication credits the source).
>
> Mission: Voyager 1/2. Product: closed-loop two-way DSN Doppler, recorded as
> ATDF/ODF and never released to PDS; the cruise/post-Saturn window is the
> request-only gap (the open-loop ODR occultation and the Saturn-encounter data
> survive elsewhere). Format: native ATDF/ODF (TRK-2-34). Availability:
> "Ready for Offline Distribution" (SDDPT 77-084A-0071). A caveat, honestly
> named: the catalog description names mixed tape families (ODR/PWS/mosaics) —
> the closed-loop Doppler content is not explicitly confirmed by the catalog.
> First measurement: the tape inventory `B_INFO.TXT`. This request is the formal
> SDDPT (Science Digital Data Preservation Task) channel.

## Vorlage 1b — NSSDC-SDDPT-Antrag: Mariner 10 (7-Track)

> **To:** CRUSO `gsfc-dl-nssdca-request@mail.nasa.gov` (cc der Katalog-Generalkontakt,
> NASA JPL — der Katalogkontakt für `PSCM-00009`)
>
> **Subject: Data request — Mariner 10 raw radio-science/tracking (NSSDC PSCM-00009)**
>
> We request the raw radio-science tracking data held at NSSDC under `PSCM-00009`,
> for independent re-derivation of the residual against an N-body model (academic,
> A=A — no fabricated value; the publication credits the source).
>
> Mission: Mariner 10. Product: Celestial Mechanics / Radio Science Engineering
> Data Records — X- (8400 MHz) and S- (2113 MHz) band radio transmissions for
> tracking and occultation studies, the 762 7-track tapes copied to CD-WO
> (SDDPT 73-085A-0008, volume IDs MVM_1001–MVM_1007). Format: 7-track magnetic
> tape, 6→8-bit padded; ISO 9660 level 1 CD-WO. Availability: "Ready for Offline
> Distribution". A new 7-track parser is built only after a successful request.

## Vorlage 1c — NSSDC-SDDPT-Antrag: Viking 1/2 (Tracking)

> **To:** CRUSO `gsfc-dl-nssdca-request@mail.nasa.gov` (cc der Katalog-Generalkontakt,
> NASA JPL — der Katalogkontakt für `PSPG-00457`; der Katalog-Generalkontakt,
> NASA LaRC — für `PSPG-00011`)
>
> **Subject: Data request — Viking 1/2 raw tracking (Doppler+Range) (NSSDC PSPG-00011 / PSPG-00457)**
>
> We request the raw radio-science tracking data held at NSSDC under `PSPG-00011`
> (and `PSPG-00457`), for independent re-derivation of the residual against an
> N-body model (academic, A=A — no fabricated value; the publication credits the
> source).
>
> Mission: Viking 1/2. Product: Doppler + Range (tracking). Format (measured
> 2026-09-16, `PSPG-00011` catalog): merged/reformatted project tracking tapes;
> each record carries time, S- and X-band Doppler frequency, S- and X-band range
> (light time in nanoseconds) and station information; Doppler spacing ≤1 min,
> ranging 5–20 min; state vectors and a maneuver calendar included. `PSPG-00457`
> is the 7-track-tape SDDPT copy (6→8-bit padded, ISO 9660 level 1 CD-WO,
> 75_075A_0008, VO_6101–6110). Availability: `PSPG-00011` "Data Identified but
> not Received"; `PSPG-00457` "Ready for Offline Distribution".

## Vorlage 2 — JPL-NAV-Datenanfrage (Cassini closed-loop)

> **Subject: Data request — Cassini closed-loop two-way Doppler (TRK-2-34/2-18)**
>
> We request the raw closed-loop two-way X/Ka/S-band Doppler range-rate for the
> Cassini mission, for independent re-derivation against an N-body model. The PDS
> radio-science archive carries open-loop RSR and derived products only; the
> closed-loop TNF/ODF stayed with the navigation team. Academic, A=A; the
> publication credits the source. Contacts: DSN Commitments Office /
> Interplanetary Network Directorate, deepspace.jpl.nasa.gov.

## Kernregel (0 honored)

Kein Wert wird ohne geladene Messung re-deriviert. Bleibt eine Anfrage ohne
Antwort, bleibt der Posten `pending` mit Ort — nie eine erfundene Zahl.
