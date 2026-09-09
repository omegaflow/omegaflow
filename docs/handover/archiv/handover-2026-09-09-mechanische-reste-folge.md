<!--
  title: Übergabe — mechanische Reste (Folge): CDN-Dispatch + Matrix-Re-Verifikation geschlossen, HILTON-Report gemessen, ned/AllWISE als Checkpunkte
  class: handover
  date: 2026-09-09
  sha256: 0099b4407afc931363b56a84882d825419ab56a724ac1b8ef6341765aa968245
  status: archived
  see-also: docs/handover/handover-thematisch-mechanische-reste.md docs/handover/archiv/
-->

# Übergabe — mechanische Reste (Folge)

Übergabe der Sitzung, die die sechs Rest-Übergaben konsumiert hat —
`neptun-scheinbar-orts-kette`, `uranus-push-cdn`, `matrix-fix-nachtrag`,
`te-atome-blocknull-ksg-pcmci`, `allwise-2mass-ernte`,
`weberin-folge-cdn-ned-twomrs` — nur deren Restpunkte. Alle sechs liegen jetzt
in `docs/handover/archiv/` (von der Registratur-Sitzung verschoben, nicht von
dieser).

## Geschlossen (trägt Git)

- **CDN-Dispatch Neptun** (neptun §2a + uranus §2): `neptune-c-spk-cdn.yml`
  dispatched — Run 34359296647, success. `ephemeris_neptune_c.bin` jetzt
  **200 / 17 181 776 B** (breit 1802–2030), ersetzt das alte 4.9-MB-±30-Jahre-
  Exemplar.
- **Matrix-Fix Re-Verifikation** (matrix-fix §5.1): earth-bin (18 MB,
  Matrix-Fix) vom CDN gezogen, stale Membran-Cache
  `omegaflow_series_ephemeris_earth.bin` gelöscht; `galileo_elevation_match`
  full + sanity: Haupttabellen exakt unverändert gegen die Anker (replication
  1.828/18.827, interior 1.162/4.757, Geometrie +1.0/−22.5/−7.9), Sanity-Peak
  DSS43 jetzt **~04:00** statt 12:00. Hypothese bestätigt.
- **Shift-Null-Zug-5** (te-atome §6.1): steht am HEAD als Dauer-Tor (Atom-4,
  dbbaa0c); Probe `point-16-shift` success (Run 34365462328, FPR 4,11 % ≤ 8 %).
  Kein eigener Lauf nötig.
- **HILTON-Report-Modus** (neptun §2b): gebaut —
  `neptune_apparent_chain_probe --report <N>` listet je Serie Quellzeile, Datum,
  ΔRA/ΔDec (″) + Rohzeile für |Δ| > N·σ (σ = 1.4826·MAD). In bd4d123 eingefaltet
  (Blob 7a24291). Datencheck: **CAMB 1862 (−1.6°) und 1868 (−14.8°) sind
  RA-Platzhalter-Zeilen** — die Quelle trägt RA `0 0 0.00000`, ΔRA = −model_RA;
  alle 56 CAMB-Platzhalter geflaggt. Zweite Konvention: GREN-dec `90 0 0.0000`
  als Absent-Marker.
- **CI-Optimierung** (Commit 5503758): `ci-check` test → `cargo test --release`
  + `cancel-in-progress: false`; Release ~17× schneller (gemessen 377 s →
  22,6 s je Gate). Die debug-Vollsuite (~20 schwere TE-Tests) war Stunden und
  wurde bei jedem Push abgebrochen.

## Offen (Checkpunkte — warten extern/Myzel)

- **ned-Landung** (weberin): 1/40 Slices (`ned_part_00000.json` liegt),
  `ned.json` erst bei 40/40 (~40 h, stündlicher Cron); danach Roundtrip + Größe.
- **AllWISE-Ernte** ~13 d (Run 34343245265); danach Roundtrip + Größe,
  `--survey allwise`.
- **IPAC-Antwort** ausstehend; `auftrag-ned-objdir-zugang.md` bleibt pending.
- **HILTON-Absent-Fix** pending: Parser soll RA `0 0 0.00000` (und GREN-dec
  `90 0 0.0000`) als absent lesen → Zeile skipped, 0 honored. Ursache gemessen,
  Fix nicht committet (Operator-Wort „Beides erst später committen").
- **CI-Release-Suite**: erster voller grüner Release-Lauf noch nicht beobachtet
  (geschätzt ~10–15 min, nicht gemessen).

## Baum

- Geteilter Baum: mein einziger Commit ist 5503758 (nur ci-check.yml).
  HEAD == origin/main == 8d2d40b.
