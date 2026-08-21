# Handover: Berkeley-PSP-VSC + Wind/WAVES wav_h1

Registriert 2026-08-20. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # muss 0/0 sein
cargo run --bin bia_efield_compiler -- --probe <datei>.cdf   # CDF-Struktur zeigen
```

Referenzen (stehend): `src/cdf.rs` (der Parser), `src/bin/bia_efield_compiler.rs`
(die Ernte), `phi/pipeline/ledger.φ` (kraft-kanal electric), `TODO.md`
(Solar-Akteure-Folgen), `phi/pipeline/research/agent_output/solar_akteure_probe.φ`
(Befunde). Der cdf_reader-Atom liegt archiviert unter
`/home/johannes/projects/archive/handover/handover-2026-08-20-cdf-reader.md`.

## Auftrag

Zwei Payloads des solar-electric-Kanals klären:

A. **Berkeley-PSP-VSC** — die Behauptung „kalibriertes mV/m-Feld existiert
   nur als Berkeley-CDF" (Recherche-Agentin) verifizieren oder benannt als
   unverifizierbar schließen. Fund → Route dokumentieren (Ernte = Folge-Atom).
B. **Wind/WAVES wav_h1** — E_VOLTAGE_RAD2 (kalibrierte Antennen-Spannung,
   1994–2021). Layout-Probe + Ernte-Prototyp. Der cdf_reader steht schon.

## Verifizierter Kontext (2026-08-20)

- `src/cdf.rs` parst NASA-CDF **3.x**: CDR/GDR/VDR/VXR/VVR/CVVR, gzip via
  inflate, Encoding 1 (BE) + 6 (LE), EPOCH/EPOCH16/TT2000. **Nicht** CDF 2.x
  (Magic cdf26002 → Version-2-Note), kein Huffman/Adaptive, kein Sparse.
- Berkeley-Befund: research.ssl.berkeley.edu ist **lokal DNS-tot**, via
  Jina-Reader 200:
  `curl -sS https://r.jina.ai/https://research.ssl.berkeley.edu/...`
  Baum-Walk l2/l3: KEIN vsc-Verzeichnis (aeb, dfb_*, f2_100bps, mag_*,
  rfs_*, sc_pse, tds_wf, dust, merged_scam_wf, sqtn_*). CDAWeb-HAPI-Katalog:
  kein PSP_*VSC* (PSP_FLD_L2_VSC → 1406; AEB@0/@1 = HK). SPDF spiegelt
  dieselbe Menge.
- Wind-Befund: `spdf.gsfc.nasa.gov/pub/data/wind/waves/wav_h1/` lebt,
  Jahresverzeichnisse 1994–2021. E_VOLTAGE_RAD2 in wi_h1_wav; WI_L2_WAV
  RAD1/RAD2 tragen nur PSD. (Via Jina-Reader falls direkter Zugriff stockt.)
- Operator-Limit: **keine Membrane-Fenster** — alle Tests kopf-los.

## Session-Fragen

A1. Existiert das VSC-Produkt unter anderem Namen? Kandidaten prüfen:
    aeb/ („AC electric"?), tds_wf/ (Wellenformen = Spannungen?),
    dfb_wf_* (differential voltage), l1b/l1_dat (Kalibrationsbereich),
    die Release-Notes auf fields.ssl.berkeley.edu (Jina). Ein .cdf je
    Kandidat laden und mit `--probe` die Variablen benennen.
A2. Woher stammt die VSC-Behauptung? Rückverfolgen (SPDF-Datashop, alte
    CDAWeb-Listen, Papers). Ohne Fund: das pending im Register SCHLIEßEN
    mit dem Verdikt „vom live-Baum verschwunden/unveröffentlicht" —
    benannt, nicht verschwiegen. (TS1/TS2-EFI_EAC ist der Nachfolger,
    eigenes Atom — nicht vermischen.)

B1. Datei-Layout wav_h1: **Magic prüfen (CDF 2.6 vs 3.x — 1994er-Datei!)**,
    Encoding, Kompression (alte rVDR/RLE?), Epoch-Typ (CDF_EPOCH
    erwartet), Variablen (E_VOLTAGE_RAD1/RAD2 je Antenne? num_elements?),
    Kadenz, Dateigröße/Tag → Erntemasse für ~9855 Tage abschätzen.
    Ein 1994er + ein 2021er File laden (SPDF, via Jina-Reader falls nötig).
B2. Semantik: V oder V/m? Force-Gate-Urteil dokumentieren (em;
    die Spannung ist der Messwert selbst).
B3. Ernte-Prototyp: ein Monat → Mediane → eigener bin (Magic-Klasse WAV1?)
    oder Verallgemeinerung des bia_efield_compiler — Session-Entscheid.
    Frame `at wind`: BODY_REGISTRY + CDN-Asset ephemeris_wind.bin prüfen.
    Falls CDF 2.6: cdf.rs erweitern (Layout in der cdflib-Quelle
    dokumentiert — 32-bit block_size, verschobene Offsets; begrenzt).

## Gates

- cargo check 0/0, cargo test (Tests gegen die echten wav_h1-Dateien,
  1994 + 2021, kopf-los).
- Ernte-Prototyp roundtrip-geprüft; Befunde in
  phi/pipeline/research/agent_output/ (Fortschreibung oder neues Doc).
- Register: ledger.φ (kraft-kanal electric + wind-waves), TODO.md.
- Ein Commit je Einheit; das letzte schließt die offenen Posten.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

TRACERS-SOC, TS1/TS2-EFI_EAC, GONG L 31..200, die LIRA-Ernte (steht).
