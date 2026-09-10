<!--
  title: Handover — NAIF 501–504: jup365.bsp geerntet (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: bef218ae4fd07bf10027d4c5dba15b850c09cf73bad1fa58b375717b1836e1a9
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom-vier-atome.md
-->
# Handover — NAIF 501–504: jup365.bsp geerntet (2026-09-10)

## Geleistet (trägt Git)

- `7182935` — register the jup365.bsp NAIF satellite kernel in sources.φ: nur der Schwanz-Hunk (Leerzeile 7552 + 7553–7556 url/format spk/at sun/ttl 86400), 5 Zeilen, eine Datei.

## Ernte (gemessen)

- jup365.bsp: 1136581632 Byte, Last-Modified Sun 14 Mar 2021 15:29:22 GMT (1615735762), sha256 dbf016c01ba4d022154838000cf3f06962cf958ddc503a366f7fe8f81495c5cb (lokal gemessen; NAIF publiziert keinen Prüfsummen-Träger — kein .md5/.sha256, jup365.cmt ohne Digest, jup365.mrg ist Merge-Manifest). Asset: cache/naif.jpl.nasa.gov/pub-naif-generic_kernels-spk-satellites-jup365.bsp.json (Netloc-Verzeichnis der ura111/nep097-Geschwister, gitignored).
- Ernte-Verlauf: 1,1-GiB-Zug überschritt das erste 15-min-Fenster bei 875708416 Byte; curl -C - setzte auf exakt 1136581632 Byte fort (Exit 0, Größe = Content-Length).

## Register (Verdikte wörtlich)

- Q1: Nur die 5 Schwanz-Zeilen committet (Leerzeile 7552 + 7553–7556 url/format/at sun/ttl 86400): `restore --staged` löst die drei fremden Staging-Pfade (.gitignore, MANIFEST.φ, copernicus_disposition.φ — deren Arbeit bleibt im Arbeitsbaum), Staging nur des Schwanz-Hunks, `git diff --cached` = nur der Schwanz, dann `git commit` ohne Pathspec. `git commit -- phi/sources.φ` verboten — gemessen: es verschluckt 40 Zeilen, davon 35 fremde (noaa +33 @5563, supermag-Rename ±2 @6010).
- Q2: CDN-Manifestation pending, gemessen: kernel-flatten 5 Läufe mit conclusion failure (letzter 2026-09-09T19:57Z); in den beiden letzten Läufen bodies mit abgebrochenem Flatten-Schritt, jwst-spectra mit fehlgeschlagenem Compile; kein Lauf seither, keiner in Flug. Dispatch notiert: `gh workflow run kernel-flatten.yml --repo omegaflow/omegaflow`; Erfolgskriterium: bodies-Job grün + jup365-Release am CDN — das nächste Atom dispatcht mit Wächter und schließt die Zeile mit dem gemessenen Ausgang.
- Q3: Atom geschlossen: NAIF 501–504 geerntet — jup365.bsp im Cache (1136581632 B, Last-Modified 1615735762, sha256 dbf016c…, gemessen; NAIF veröffentlicht keine Prüfsumme), sources.φ 7553–7556 als einziger Hunk committet; CDN-Manifestation pending (CI-Anatomie benannt, Dispatch notiert). Kollision: handover-2026-09-10-supermag-koernung.md steht untracked — benannt, nicht konsumiert; handover-2026-09-10-autonom-vier-atome.md bleibt live (Kollisionskarte noch von Parallel-Sessionen gelesen, seine NAIF-Zeile ist mit diesem Atom abgearbeitet); geo.rs bleibt Dreier-Eigentum — ANR-Paarung ist das nächste Atom, antares_loci_compiler wartet gebaut und verifiziert.

## Offen (benannt, nie 0)

- kernel-flatten-Dispatch mit Wächter (nächstes Atom, Kommando + Kriterium oben).
- ANR-Wiederanlage + antares_loci_compiler-Commit — nächstes Atom, wenn geo.rs freigegeben ist (Compiler gebaut + verifiziert, nur der geo-Support fehlt).
- pack_iaga für 163 Ziffer-Codes — eigenes Atom.
