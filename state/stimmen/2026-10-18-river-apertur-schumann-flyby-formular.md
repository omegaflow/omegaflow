# Formular — River-Triade: Receiver-Apertur Schritt 3 · Schumann `field` · Flyby-`sealed`

**Datum:** 2026-10-18 · **Linie:** future (gefaltet aus `xfer:river…4~e6aa42be`, river-223/224) · **Register:** `R-2026-10-18-l`
**Status:** offen — Schwarm R1 fährt als Riss-Generator (frei, $0); tragende Sitze wählt der Operator.

## Die Fragen (drei, aus einer Deklaration)

River deklarierte: „Rat: Receiver-Apertur 3 · Schumann `field` · Flyby-`sealed`"
(`docs/handover/handover-2026-10-18-river-folge224.md:51`). Die Leitstelle faltet sie zu drei Fragen:

### F1 — Receiver-Apertur Schritt 3: wo wird die Blende gebunden?

- Schritt 1 gebaut: `ReceiverAperture`, deklariertes Receiver-Feld, `declared(extent_m)`, 0/NaN/∞ → `None`
  (river-192, `handover-…folge192.md:58`).
- Schritt 2 gebaut: Schneidung als Faltung/Varianzsumme (σ_eff² = extent² + global_scale²) in
  `field_spatial`/`field_spatial_grad` + CPU-Spiegel + Paritäts-Gate (river-193).
- Schritt 3 offen (river-194:91): (b) die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`)
  an die **Query-Naht** (`vp.surface.w = global_scale`) binden — die deklarierte Blende je Receiver,
  nie in den Record; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).
- Kontrakt: „the query carries its own aperture-derived extent beside the record's, and the two meet
  only in the cut. It is never the record `extent` — a receiver aperture written into the record would
  be Observer-as-vantage, forbidden." (`docs/concepts/archivar-mathematikerin.md:34`)

**Frage:** Ist die Query-Naht-Bindung (b) die richtige Form — und bleibt (c) Operator-Wort, oder
gehört das Fenster-Edit in die Register-Mechanik?

### F2 — Schumann `field`: wie kommt ein weltweites Dichte-Gitter ins Register?

- Gemessener Blocker: WGLC **0** in `phi` (river-219:270); die Quelle ist **täglich**, nicht stündlich;
  Lizenz entschieden (`cc-by-4.0`), `terms`/`ttl 86400` gesetzt — Rest-Blocker ist der **Rat** (`field`-Form).
- Operator-Wort (folge219:48): „Mountain soll das **Rat-Paket zur WGLC-`field`-Form** vorbereiten
  (die Frage, wie ein weltweit gemessenes **Dichte-Gitter** — kein Punkt-Kernel — ins Register kommt).
  Danach der Rat; dann Myceliums `url`/`origin`/`compiler`-Block + CDN-Manifestation, dann die
  bedingte Schumann-TE."
- Gebaut ist der Tages-Deskriptor `schumann_geomag_blitz.te` (`cond wglc_strokes pending`, `bin 86400`);
  der Pair bleibt ungemessen, bis die WGLC-Reihe steht.

**Frage:** (i) Welches `field`-Format trägt ein globales Gitter (Raster vs. Punktwolke vs. eigenes
`grid`-Kind)? (ii) Bleibt die Sequenz Operator-Wort (Paket → Rat → Block → TE), oder darf der Rat
die Form jetzt ohne Mountains Paket entscheiden?

### F3 — Flyby-`sealed`: welcher Träger prüft den Versiegelungs-Vergleich künftig?

- Riss (mycelium-Brief 2026-10-16): `SEALED_SHA256 = "aeb3c82f…"` vs. registrierter Posten
  `ephemeris_juice.bin` (`no-cadence`, **kein `sha256`**), aktueller CDN-Digest `b68842c9…` — der
  `--sealed`-Lauf über den registrierten Posten ist per Konstruktion ein sha-`riss`.
- Die Frage (derselbe Brief): **welcher** Träger prüft künftig —
  1. eine **gepinnte** versiegelte Bahn: Konstante **und** ein eigener gepinnter Register-Posten, oder
  2. die Gate-Mechanik löst sich vom festen `SEALED_SHA256` (Digest aus dem Asset/Manifest)?
- Termin: ESOC-Rekonstruktion + 1σ, Frist **2026-11-01** (`ephemeris_juice_recon.bin` heute absent).

**Frage:** Option 1 (gepinnter Posten) oder 2 (Asset-Digest) — und wer setzt den Posten (River), bis
der Flyby läuft?

## Was der Schwarm nur benennen darf

Der Schwarm ist Riss-Generator, nicht Entscheider (Rat M2, `_rat-anweisung.md` §3b). Kein
Register-Schreibakt aus einem Schwarm-Lauf; tragende Sitze wählt der Operator.

## Belege

- `docs/handover/handover-2026-10-18-river-folge224.md:51` (Deklaration) · `:31-34` (Flyby-Termin)
- `docs/handover/archiv/handover-2026-10-11-river-folge192.md:58` · `…folge193.md:3` · `…folge194.md:91`
- `docs/handover/archiv/handover-2026-10-15-river-folge219.md:48/61/168/270`
- `docs/concepts/archivar-mathematikerin.md:34` · `post/in/2026-10-16-mycelium-an-river-flyby-sealed-sha-riss.md`
