<!--
  title: Axiom-Gate-Survey — h0-lines-register
  class: survey
  date: 2026-09-12
  sha256: a183f39ca6607504c178d9407cecd82ee32e99c140769d1f7216185ffd8d7ea7
  status: live
  see-also: docs/paper/h0-lines-register.md
-->
# Axiom-Gate-Survey — h0-lines-register

Das Paper `docs/paper/h0-lines-register.md` (Titel 50/50, Abstract 144/200,
399 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- **Verletzung gemessen und korrigiert:** Die Erstfassung trug die eigene
  Leiter-H₀ nicht. Das Blatt registriert sie seit 2026-09-08 gewogen
  (`h0_ladder_weigh`, „Die eigene Leiter-H₀"): H₀ = 73.56 ± 1.40 km/s/Mpc.
  Das Handover 2026-09-08 (veraltet) nannte die Leiter als nächstes Atom —
  die Arbeits-Schicht hatte das veraltete Register weitergereicht; das
  Paper folgt jetzt dem Blatt, nicht dem Handover. Getragen sind alle
  Parameter 1:1: 73.56 ± 1.40, Checkpoint 73.53 ± 1.14, M_W1 = −5.914 ± 0.017,
  zp = −13 ± 5 μas, M_B = −19.2469 ± 0.0299 (77 Kalibratoren),
  a_B = 0.7159 ± 0.0018; Reproduktions-Gate PASS (jeder Parameter in 1σ).

## Konsistenz

- Die 2012.08534-Tabelle trägt 74 volle Datenzeilen, die Prosa zitiert 75;
  7 Zeilen sind in π_EDR3 `absent` — im Paper getragen, nicht geglättet.
- Der 75-Quellen-Crossmatch (per-source identity des Cepheiden-Ankers) ist
  als `pending` benannt; die 0.2619-mas-Wiegung überzeichnet sich nicht.

## Pfad

- `tools/measure/src/bin/cepheid_parallax_weigh.rs` (DCEP-Klasse, N = 1606)
  und `tools/measure/src/bin/h0_ladder_weigh.rs` (Leiter end-to-end);
  `GAIA_TAP_SYNC` in `src/archivar/gaia_sso.rs` — alle Pfade stimmen.

## Zuordnung

- see-also → `docs/blatt/blatt-h0-linien-register.md` (lebt).
- Verdikt ohne Paper: keines — das Paper trägt das Register-Verdikt
  (weißes Feld, Asymmetrie, kein Schlichter).

## Externes Register

- 25/25 zitierte arXiv-Kennungen `resolved` (arXiv-API, 2026-09-12).
- DOI 10.1093/mnras/staf700 `resolved` (doi.org).
- NASA-Bindung: HST Key Project, SH0ES (HST/JWST), CCHP/Freedman 2025;
  bibliographische Route über NASA ADS benannt.
