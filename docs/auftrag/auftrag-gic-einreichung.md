<!--
  title: Auftrag — GIC-Paper einreichen (Ziel: Space Weather)
  class: auftrag
  date: 2026-09-27
  sha256: d2ecfbac3f16eb87a2a67286e7115101a25911cc6d03f7143a5108c9ea28b838
  status: live
  see-also: docs/paper/gic-causal-driver.md
-->
# Auftrag: die Einreichung des GIC-Papers

## Zweck

Der Bau-Auftrag aus `future-folge112` (7 Lücken + Prior-Art-Framing) ist
abgeschlossen; `docs/paper/gic-causal-driver.md` ist reif. Dieser Auftrag trägt
die **Einreichkante**: Venue, Cover Letter, Autoren-/Framing-Klärung, der eine
verbleibende Vorbereitungsschritt. Der **Akt** (Absenden über GEMS / ESSOAr) ist
die Operator-Hand — das Gegenüber ist ein Dritter (Verlag).

Das Papier ist **höchste Priorität** (Operator-Wort, 2026-09-27).

## Venue (gemessen 2026-09-27 via `archive_search`)

| Venue | Scope-Treffer | OA/APC | Einreichung | Preprint |
|---|---|---|---|---|
| **Space Weather (AGU/Wiley)** | „understanding, forecasting and the mitigation of space weather and its impacts" — das Ground-dB/dt ist der Impact-Kanal | OA, **$3,240** | GEMS | ESSOAr |
| Annales Geophysicae (EGU/Copernicus) | solar-terrestrial + space weather + geomagnetism; offenes interaktives Review | OA, **€1,350** | Copernicus-Editor | EGUsphere (frei) |
| JSWSC (EDP) | „new methods … data analysis techniques" (Technical Article) | OA, **€1,250** | Editorial Manager | arXiv |

**#1 = Space Weather.** Die Scope-Zeile nennt genau die gemessene Kette; die
direkten Nachbarn liegen dort (Pulkkinen et al. 2017, `10.1002/2016SW001501`;
Ground-dB/dt-Statistik 2024, `10.1029/2023sw003767`; GIC-Forecast-Methodenpapier
2026-09-26). ≤25 publication units passen; kein Grund-/Seitenpreis; 54 %
Annahme, 42 Tage erste Entscheidung. **Fallback:** AnGeo (offenes Review, Venue
der TE→Dst-Linie Johnson, Wing & Camporeale 2018), dann JSWSC (Technical
Article). GRL fällt (Letters, 12 PU, „research articles not accepted").

## Prior-Art (gemessen 2026-09-27)

Kein Papier gefunden, das TE von L1-Solarwind-Treibern auf **Ground-dB/dt-Maxima
an INTERMAGNET-Stationen** unter einer phasen-randomisierten Null **mit
family-wise Schranke** misst. Nächste Nachbarn: Manshour et al. 2021
(`10.3390/e23040390`, Ziel AE/SYM-H); Boutsi et al. 2025 (`10.3390/e27020172`,
GIC-Indices, kein TE); Johnson, Wing & Camporeale 2018
(`10.5194/angeo-36-945-2018`, Ziel Dst). **Novelty = Kanal (TE → Ground-dB/dt) +
Schranke (family-wise round-maximum)**, nicht die phasen-randomisierte Null allein.
**CJSS 2022 gelesen — Volltext, gemessen 2026-09-27:** Yu, Tong, Fang & Hu
(`10.11728/cjss2022.03.210406045`) ranken Solarwind-Treiber zum Sym-H-Index per TE
(93 Stürme, 2010–2018); E und Bz dominieren bei 60 min (E 0.200, Bz 0.196 nats); der
Null ist ein Quell-Shuffle (100 Resamples, 95 %), **keine** family-wise Korrektur. Ziel
ist der Sturm-Index, nicht Ground-dB/dt → unsere Novelty-Zeile wird **nicht
vorweggenommen**, in der Richtung **gestützt**; als Zitat im Papier aufgenommen
(Related Work + Referenz). Volltext/Referenzen:
`docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md`; PDF
`data/cjss.ac.cn/210406045.pdf`.

### Prior-Art-Zeile (für den Cover Letter, kopierbar)

> No prior study measures transfer entropy from L1 solar-wind drivers to the
> ground-dB/dt maxima at INTERMAGNET stations under a phase-randomized null with a
> family-wise multiple-comparison bound; the nearest neighbors are Manshour et al.
> 2021 (10.3390/e23040390, target AE/SYM-H), Boutsi et al. 2025 (10.3390/e27020172,
> GIC indices, no TE), Johnson, Wing & Camporeale 2018 (10.5194/angeo-36-945-2018,
> target Dst) and Yu et al. 2022 (10.11728/cjss2022.03.210406045, target Sym-H,
> source-shuffled null, no family-wise correction).

Quellen: `docs/paper/gic-causal-driver.md:35`–`:39` (Verweise) · `:336`–`:354`
(Relation to the literature) · `:446`–`:456` (Referenzen) · dieser Auftrag,
Abschnitt Prior-Art (gemessen 2026-09-27).

## Cover Letter (Entwurf, 8 Sätze)

1. We submit "The directional driver of geomagnetically induced currents" as a Research Article in *Space Weather*.
2. Which L1 quantity drives the ground excitation in the directional information-flow sense is open sub-daily; prior transfer-entropy work ends at magnetospheric indices, and this manuscript measures the ground end of the chain.
3. We measure KDE transfer entropy (Silverman bandwidths, phase-randomized surrogates, fixed seeds) from L1 Bz, speed and density to minute, hourly and daily maxima of dB/dt at INTERMAGNET Abisko and Sodankylä, with a family-wise bound — the round-maximum surrogate TE — as the multiple-comparison control.
4. The result is reported exactly as measured: no family-clearing hourly driver is established; the yearly-round Bz→dB/dt arrow (2024/2025 ABK, 2024 SOD) does not survive the hardened quarterly null (24/24 directed rows family bound, n_surr = 100, lag sweep 0–6 h) and PCMCI removes the conditioned edge in 13 of 16 shards; the two witnesses are carried as a riss, never averaged.
5. What survives the riss: the density control stays silent throughout, the forward direction dominates the reverse in every yearly round, and the 32-year daily grain is empty because daily means wash the storm signal out — Bz remains the leading sub-daily candidate via the yearly arrow and the asymmetry.
6. The contribution is the instrument and its honesty: a family-wise bound that gives the GIC-driver literature a conservative standard for reading single-lag driver claims, applied at the ground end of the chain where it has not been applied before.
7. All values are machine-measured; estimator, probes and data routes are published in the omegaflow repository, and the estimator is validated against the Schreiber (2000) coupled-Hénon benchmark in the manuscript.
8. The manuscript has not been submitted elsewhere; we will post it to ESSOAr at submission.

### QUELLEN (jede Zustandsbehauptung → gemessene Quelle; der Operator liest den Block vor dem Wort)

- Satz 1 — „Research Article" ist ein SWE-Typ → `claim → browser-bridge@2026-09-27 →
  https://www.agu.org/publications/authors/journals/text-graphics-requirements`
  („Research Articles … up to 25 publication units").
- Satz 2 — „prior transfer-entropy work ends at magnetospheric indices" →
  `docs/paper/gic-causal-driver.md:36` (Johnson & Wing 2005; Wing et al. 2016;
  Yu et al. 2022) · `:341` (Yu et al., Ziel Sym-H) · `:456` (Yu-Referenz).
- Satz 3 — Methode (KDE TE, Silverman-Bandbreiten, phasen-randomisierte Surrogate,
  fester Seed, Bz/Speed/Density → Min-/Std-/Tages-Maxima von dB/dt, family bound) →
  `gic-causal-driver.md:85` (Silverman) · `:94` (Surrogate) · `:96` (fam-Definition) ·
  `:52`–`:61` (Daten) · `:67`–`:69` (dB/dt).
- Satz 4 — Ergebnis (kein family-clearing Std.-Treiber; Jahres-Runde Bz→dB/dt
  2024/2025 ABK + 2024 SOD; gehärtete Quartals-Null 24/24, n_surr=100, Lag-Sweep
  0–6 h; PCMCI entfernt die Kante in 13/16 Shards) →
  `gic-causal-driver.md:203` (2024) · `:220` (2025) · `:237` (SOD) · `:124`–`:130`
  (n_surr=100, Lag-Sweep 0–6 h, 24/24 family bound) · `:374`–`:382` (PCMCI 13/16).
- Satz 5 — „density control stays silent … daily grain is empty" →
  `gic-causal-driver.md:212` (Dichte stumm 2024) · `:254`–`:270` (Tageskorn, n≈3900).
- Satz 6 — family-wise bound als konservative Schranke →
  `gic-causal-driver.md:415` („fam is conservative but not exhaustive").
- Satz 7 — Schreiber-2000-Benchmark-Validierung → `gic-causal-driver.md:142`–`:172` (§3.5).
- Satz 8 — „not submitted elsewhere" ist die Dual-Publication-Versicherung des
  Operators (AGU-Vorgabe → `claim → browser-bridge@2026-09-27 →
  https://www.agu.org/publications/authors/journals/text-graphics-requirements`,
  „an original submission and not under active consideration elsewhere") — keine
  maschinelle Messung; „post to ESSOAr" → Abschnitt ESS Open Archive unten.

## GEMS — Einreichmechanik (gemessen 2026-09-27, Browser-Bridge/Operator-Profil)

GEMS ist AGUs Einreichsystem (eJournalPress). Portal
`https://spaceweather-submit.agu.org/` (Journal-ID `j_id=281`); Editor in Chief
Steven Morley. Universal-Login (AGU + GEMS geteilt); **ORCID Pflicht** für den
korrespondierenden Autor, empfohlen für alle Ko-Autoren. Neues Konto: „New
authors should register for an account" (`form_type=display_smart_reg`).

**GEMS-Formularfelder (Reihenfolge der Einreichung):**

1. **Article type** — „Research Article" (≤ 25 Publication Units).
2. **Title + Abstract** — Abstract < 250 Wörter, ein Absatz (unser 199, Gate grün).
   Bei Overleaf/Curvenote/Authorea-Upload werden Titel+Abstract automatisch aus
   der Datei geladen; sonst von Hand.
3. **Authors** — Namen, E-Mails, Affiliationen; CRediT-Beiträge (Checkboxen im
   „Author"-Tab); ORCID je Autor (Pflicht korrespondierend). Autorenblock privat:
   `state/paper/gic-autoren-2026-09-27.md`.
4. **Key Points** — 1–3, je ≤ 140 Zeichen, ganze Sätze, keine Abkürzungen.
   FEHLT im Manuskript (Lücke, unten).
5. **Plain Language Summary** — ≤ 200 Wörter, jargonfrei; **Pflicht für Space
   Weather**. FEHLT im Manuskript (Lücke).
6. **Index Terms** (bis 5, AGU Index Set) + **Keywords** (bis 6, frei).
7. **Funding** — Förderquellen + Grant-IDs im GEMS-Formular (CHORUS); derzeit
   unabhängig/ohne Förderung → leer.
8. **Financial Information** — APC-Option wählen (APC-Pfad unten).
9. **Cover Letter** — optional, **ins Formular eingetragen, nicht als Datei
   hochgeladen**; der 8-Satz-Entwurf (oben) kommt in dieses Textfeld.
10. **Suggested Reviewers** — **3 oder mehr** (Namen/E-Mail/Affiliation). FEHLT
    (Vorbereitung, unten).
11. **Open Research** — Data- + Software-Availability-Statement im Manuskript
    (Abschnitt „Open Research"), Daten in vertrauenswürdigem Repositorium deponiert
    und zitiert; der „Data and code"-Absatz (`gic-causal-driver.md:460`) ist noch
    kein AGU-konformer Abschnitt (Lücke).
12. **Conflict of Interest** — Erklärung im Artikel-File: „The authors declare
    there are no conflicts of interest for this manuscript." FEHLT (Lücke).
13. **Uploads** — **eine** vollständige PDF (Text + Figuren + Tabellen; LaTeX →
    PDF konvertieren) für die Erst-Einreichung; Supporting Information separat.
14. **Verify & Submit** — der finale Akt (Operator-Hand).

Quellen (gemessen 2026-09-27, Browser-Bridge/Operator-Profil): AGU „Journals
Submission Checklists"
(`https://www.agu.org/publications/authors/journals/submission-checklists`),
„In-Depth Text and Graphics Requirements"
(`https://www.agu.org/publications/authors/journals/text-graphics-requirements`),
GEMS-Portal `https://spaceweather-submit.agu.org/`.

## APC-Pfad (gemessen 2026-09-27)

Space Weather ist **fully gold Open Access**: APC **$3,240**, keine
Base-/Excess-Page-Fees (gemessen: AGU „Publication Fees and Funding Options",
`https://www.agu.org/publications/authors/journals/publication-fees`, Tabelle
„Fully Gold Open Access Journals" → Space Weather $3,240). Die Einreichung selbst
kostet nichts; die APC fällt erst **bei Annahme** an (Kreditkarte/Rechnung/Pro-forma).

4 Funding-Optionen (Auswahl im GEMS-Abschnitt „Financial Information"):

1. Institutionelle/Förder-WOAA (Wiley Open Access Account) — entfällt (unabhängig,
   keine Institution).
2. LMIC-Waiver/Discount (Research4Life / Pricing-Power-Parity) — entfällt
   (Deutschland, nicht LMIC-Liste).
3. Corresponding Author zahlt die APC.
4. **AGU-Waiver** — 50 % Discount oder voller Erlass: in GEMS „I request a waiver
   of the article publication charge" wählen, 50 %/100 % angeben, kurze Begründung
   im Textfeld.

→ Operator-Entscheidung **bei Annahme**: Option 3 (zahlen) oder Option 4
(Waiver-Antrag). Kein Schritt bei der Einreichung selbst. Vor einem Waiver-Antrag
prüfen, dass Option 1/2 nicht greifen (AGU-Vorgabe, ebenda: „Before requesting a
waiver, please check that you are not eligible for the other 3 funding options").

## ESSOAr / ESS Open Archive — Preprint-Schritte (gemessen 2026-09-27)

AGU-Preprint-Policy: Drafts (vor Peer Review oder akzeptierte Fassung) dürfen auf
dem Earth and Space Science Open Archive (ESSOAr) oder einem anderen
Preprint-Server posten (gemessen: AGU Fees-Seite, „As part of our Preprint Policy,
AGU also encourages authors to post draft manuscripts … on Earth and Space Science
Open Archive or another preprint server"). ESSOAr heißt jetzt **ESS Open Archive**:
`https://essopenarchive.org/` (leitet von `essoar.org` um).

Schritte (direkte Einreichung; gemessen: `essopenarchive.org/submission-guide`):

1. Registrieren über **ORCID** oder **CONNECT**; kostenlos.
2. „Submit Document" → **eine** einzige primäre Datei hochladen (`.docx` oder PDF;
   Titel < 300 Zeichen, Abstract < 3000 Zeichen; PDF ohne eingebettete
   Kommentare/digitale Signaturen). Titel+Abstract werden automatisch erkannt.
3. Ko-Autoren eintragen — **vollständig und in der Reihenfolge der Hauptdatei**,
   sonst Rückgabe zur Korrektur. Autorenblock: `state/paper/gic-autoren-2026-09-27.md`.
4. **Collection** wählen, **Fields of Interest** wählen, **Lizenz** wählen (Autoren
   behalten Copyright; 4 Lizenztypen).
5. Einreichen → nach Freigabe **DOI**, öffentlich, zitierbar; nach Freigabe nicht
   mehr entfernbar. Versionsverlauf über „My Documents" → „Create New Version".
6. Preprint↔Paper-Verknüpfung nach Annahme: `support@essoar.org`.

Posting-Zeitpunkt: **zum Einreichzeitpunkt** (im Cover Letter Satz 8 angekündigt).
Gesamtgröße aller Dateien ≤ 1 GB.

## Offene Vorbereitung (autonom bis zur Kante)

- **Titel (entschieden 2026-09-27):** englisch, „The directional driver of
  geomagnetically induced currents" (58 Zeichen, Gate title ≤ 75). Der Titel nennt
  den Impact-Kanal (GIC); gemessen ist dB/dt, die GIC-Anregung — §6 benennt es. Kein
  Untertitel (Gate-Länge).
- **Autorenblock (angelegt 2026-09-27):** nicht-anonyme Form privat in
  `state/paper/gic-autoren-2026-09-27.md` (Johannes Tyroller, ORCID
  `0009-0007-5565-6348`; getrackt bleibt `*Omegaflow Working Group*`).
- **ESSOAr-Preprint** zum Einreichzeitpunkt posten (Schritte im Abschnitt ESS Open
  Archive oben).
- **Submission-Lücken gegen die AGU-Checkliste (nachgemessen 2026-09-27 am
  Manuskript `5ab33d20…`, `sha` via `omega_sh sha`):** vorhanden sind (a) **Key
  Points** (`gic-causal-driver.md:19`, 3 Punkte), (b) **Plain Language Summary**
  (`:25`), (c) **Open Research** mit Data- + Software-Availability-Statement
  (`:485`), (d) **Conflict-of-Interest**-Erklärung (`:505`), (e) **Acknowledgements**
  (`:509`). Die frühere „FEHLT"-Liste war ungemessen und ist widerlegt. Offen:
  (c-rest) die Provider-URLs (SWPC/CDAWeb/INTERMAGNET, je HTTP 200 gemessen) und die
  Software-Repo-URL (`https://github.com/omegaflow/omegaflow`) sind eingetragen; ein
  zitierbarer **DOI** (Zenodo o. ä.) fehlt noch — `pending`, nicht Sende-Blocker
  (AGU akzeptiert DOI *oder* URL); (f) **3+ Suggested Reviewers** — 6 Kandidaten mit
  Affiliation/E-Mail/ORCID vorbereitet in `state/paper/gic-gutachter-2026-09-27.md`
  (privat), ins GEMS-Formular zu übertragen. Manuskriptlücken autonom gefüllt; der
  Sende-Akt bleibt Operator-Hand.

## Kante (Operator-Hand)

- **Artefakt:** dieses Doc + `docs/paper/gic-causal-driver.md` (Gate grün,
  title 58 / abstract 199 / nums 712 / sha `5ab33d20…`).
- **Ausführbefehl (nur Operator-Hand):** GEMS-Portal
  `https://spaceweather-submit.agu.org/` → Universal-Login → „Submit" →
  Formularfelder 1–14 (Abschnitt GEMS oben) ausfüllen → „Verify & Submit".
  ESS Open Archive (`https://essopenarchive.org/`) parallel: „Submit Document".
  Kein API-POST, kein Mailversand durch die Maschine.
- **Wort erwartet:** „gic einreichen".
