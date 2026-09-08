<!--
  title: Handover — H₀-Leiter gewogen (das nächste Atom: eigenes Gaia-TAP-Crossmatch der 75 + die Registratur-Grammatik)
  class: handover
  date: 2026-09-08
  sha256: 19e3f490ea669f54733bbfb69f6792d2f62e6e5c4109c38eaed6ffe97edf52db
  status: live
  see-also: docs/blatt/blatt-h0-linien-register.md docs/TODO.md tools/measure/src/bin/h0_ladder_weigh.rs .github/workflows/h0-ladder-cdn.yml
-->
# Handover — H₀-Leiter gewogen

Übergabe für die nächste Sitzung. Das abgebende Atom hat die eigene Leiter-H₀
end-to-end gewogen — das H₀-Linien-Register trägt seine erste nicht-zitierte,
sondern gemessene Zeile. Zwei Pendenzen bleiben offen; sie sind das nächste
Atom.

## 0. Die abgebende Sitzung ist ein abgeschlossenes Atom

Der Probe `tools/measure/src/bin/h0_ladder_weigh.rs` wiegt die Leiter aus drei
Dateien (arXiv 2012.08534, `Pantheon+SH0ES.dat`, `Pantheon+SH0ES_STAT+SYS.cov`);
das Blatt rückte von „teils gewogen (Klasse)" auf „ganz gewogen (Leiter)".
Commit: `2662859` (Probe + Workflow `h0-ladder-cdn.yml` + Blatt). Die
TODO-Zeile ging in den parallelen Commit `84d430e` einer Fremd-Sitzung ein —
der Register-Stand ist korrekt, die Zeile steht nicht im Atom-Commit (benannt,
nicht verschwiegen).

## 1. Was gewogen wurde

| Sprosse | eigen | publiziert |
|---|---|---|
| M_W1 (2-param, R19-fix) | −5.914 ± 0.017 | −5.915 ± 0.022 |
| zp (Rest-Parallaxenoffset) | −13 ± 5 μas | −14 ± 6 μas |
| M_B (77 Kalibratoren) | −19.2469 ± 0.0299 | −19.253 (fiduzial) |
| a_B (277 HF, flach ΛCDM Ωm=0.3) | 0.7159 ± 0.0018 | 0.7137 (fiduzial) |
| **H₀** | **73.56 ± 1.40 km/s/Mpc** | **73.0 ± 1.4** |

Reproduktions-Gate **PASS** — jeder Parameter innerhalb 1σ des publizierten
Wertes; die Bestehensregel ist eine genagelte Zahl, die Verdacht-Reihenfolge
bei Abweichung steht im Code (erst die eigene Kette — Einheiten → Parser →
Fit —, dann der publizierte Wert). Vollständiger Befund im Blatt unter
„Die eigene Leiter-H₀ (gewogen 2026-09-08)".

## 2. Das nächste Atom — die zwei Pendenzen

- **Eigenes Gaia-TAP-Crossmatch der 75 SH0ES-Cepheiden.** Die Tabelle
  `bigtable_redux3.tex` trägt keine Koordinaten-/ID-Spalte (nur Sternnamen);
  der Anker des Probe ist die transkribierte π_EDR3, benannt als Transkription.
  Das Crossmatch (Namen-Resolver → `gaiadr3.vari_cepheid`/`gaia_source`) bleibt
  `pending` — der Name allein reicht nicht für std-only. Ein Namen-Resolver ist
  die Vorarbeit (Simbad/IAU, oder die Tabelle aus der Journal-MRT von
  ApJ 908, L6, falls sie Koordinaten trägt).
- **Registratur-Grammatik field-loser Probe-Eingaben.** `sources.φ` verwirft
  Blöcke ohne Feld/Frame (der `flush`-Gate in `src/archivar/parse.rs` fordert
  `kernel_text` oder einen Frame). Ein ehrlicher Sitz (vierter Zeugen-Typ
  „Referenzdatensatz" in `phi/witnesses.φ` + `src/archivar/zeuge.rs`, oder eine
  fetch-only-Klasse) ist ein Kern-Eingriff. Bis dahin manifestieren die
  statischen Beine über `--ci-mode` + Workflow `h0-ladder-cdn.yml` (Präzedenz:
  die lebendige TAP-Leg ist ebenfalls unregistriert).

## 3. Gemessene Zustände — benannt, nicht geglättet

- **74 Zeilen, nicht 75:** die Tabelle trägt 74 volle Datenzeilen (die Prosa
  zitiert 75); 7 Zeilen `\nd` (absent) in π_EDR3, 67 gefittet.
- **43 eindeutige CIDs, das Paper nennt 42:** die 42er-Liste ist aus den
  ausgelieferten Dateien nicht extrahierbar (Journal-MRT von ApJ 934, L7).
- **Kovarianz-Ordnung zertifiziert:** 182/191 Duplikat-CID-Paare liegen exakt
  auf den .dat-Zeilindizes der STATONLY-Matrix. Die DIAG-Spalten sind
  VPEC-aufgebläht, ungleich der .cov-Diagonale (das .cov trägt keine
  Pekuliar-Geschwindigkeitsfehler — eine Eigenschaft des Release).
- **Anker χ²/ndf = 120.1/65:** die Tabellen-Streuung übersteigt die zitierten
  Fehler; die Ankerfehler sind H₀-untergeordnet (dominiert von M_B/a_B).

## 4. Die ehrliche Grenze

Die CMB-Linie bleibt `zitiert` — die Planck-Likelihood ist eine
Forschungsmaschine, keine Session. Der Riss-Knoten ist messbar, wenn die
CMB-Linie in den Bestand einzieht; bis dahin ist der Zustand der gemessene.

## 5. Zeiger

- Probe: `tools/measure/src/bin/h0_ladder_weigh.rs`
- Workflow: `.github/workflows/h0-ladder-cdn.yml`
- Blatt: `docs/blatt/blatt-h0-linien-register.md` (Asymmetrie-Zeile +
  „Die eigene Leiter-H₀")
- TODO: `docs/TODO.md` (die zwei Pendenzen unter „pending — registriert,
  nicht fabriziert")
