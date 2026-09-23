<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 7be218d24080cfef0d4f295726f474105763faf22f3108006bf68d68ca5b814f
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An future: Aufnahme des 13. Korpus (5206 Blöcke, `phi/pipeline/index.φ:35`) ins Register — gemessen: Träger vorhanden (`archive-root/pipeline-auslese-2026-09-17/stage/master_converted.φ`), Duplikat-Quote nur 1,2 % exakt (57/4603), pfadgenau 2,1 %; echte Neuzugänge (overpass.openstreetmap.fr, api.worldbank.org, api.gbif.org …). Frage an Operator/Rat: registrieren? (Schritt: Operator-Wort.)

An future: CNES-Order 18387 ist nicht brauchbar (44,5 % Dateien in Fehler, Fortschritt 16 % seit 2026-09-21, Ablauf 2026-09-28) — eine Pause kauft nichts. Einzig wertvoll: Neuordnung nur über `DMT_N1_1144` in 100er-Batches (Schreibakt bei CNES). (Schritt: Operator-Wort für die Neuordnung.)

An future: Globus-Transfer `af68c4f1` (SuperDARN MAP) braucht Konto/Token — kein anonymer Mirror (VT-Hosts DNS-tot, USask/VT login-gated); lokal 2932 `.map` (nur 1993–2002, ~1562 Dateien fehlen), 9 Zero-Byte. Frage: Globus-Konto/Token bereitstellen oder Transfer per Web-UI bestätigen? (Schritt: Globus-Login.)

An mycelium: Beat-Paar-Akquisition (Atom-D-Fortsetzung) — eine Recherche fand für die Zweistationen-Open-Loop-Variante nur gemessene Absenz (kein Roh-Asset in `--mwmbl`/`--ads`/`--zenodo`/`--github`/`--heasarc`), aber eine konkrete Kandidatenklasse für „zwei kohärente Töne in einem Band": **Dual-Comb** — zwei phasenstarre Frequenzkämme, der Detektorstrom ist ihr Schwebungssignal. Assets (Zenodo, open Dataset): `zenodo.org/records/6413816` (HDF5 `DCS 625MSa.h5` verifiziert 200, 93 581 524 B), `2542265` (`RF.zip` = RF-Beatnote), `6451876`, `10709448`, `15095848`. Einschränkung: optisches Band, nicht DSN-RF — Force-Gate-Klassifikation nötig. (Schritt: Kandidaten gegen Force-Gate/Registry prüfen und als Quelle registrieren oder verwerfen.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

