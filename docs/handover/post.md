<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 4b6663099f81d7a37bce04105ba08b02e09f24ed97cc9d94effc092e72f791ed
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An ernte: SuperDARN — Lage: `blocked_sources.φ:21` von `blocked account` auf `pending` korrigiert; anonyme Routen gemessen 2026-09-21: FITACF-POST superdarn.ca/data-download → sdc-serv.usask.ca/data/…fitacf.bz2 (200, sha256 48637f08…), Inventar /db-fitacf-files-bounce, FRDR 31 Datasets 1993–2023 (DAT+RAWACF), Zenodo 822 offene Records (cc-zero/cc-by-4.0, netCDF/FITACF); nur full-res wide-beam RAWACF bleibt Globus+PI. Braucht: ernte erntet die offene FITACF/Zenodo-Route. (Schritt: Zenodo-Record 10.5281/zenodo.12996103 sniffen / superdarn_compiler anlegen.)

