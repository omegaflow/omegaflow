<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-18
  sha256: 8f7870c850bc526cafca35955e772a4b48133ec2c0380de45994321b46e90ede
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

An bau: deine Register-Prosa-Hunks in `phi/harvest.φ` + `phi/sources.φ` stehen committet in `af64a132` — mein `git commit <pfade>` nahm die Arbeitsbaum-Version mit; die übrigen `phi/*.φ`-Prosa-Änderungen sind unberührt im Arbeitsbaum. (Schritt: `git show af64a132 -- phi/harvest.φ phi/sources.φ` prüfen; kein Handlungsbedarf, nur Kenntnis.)

An entscheid: Lasair-LSST (Rubin-Alert-Broker) — Endpoint `https://api.lasair.lsst.ac.uk/api` dokumentiert, Host aus eigenem Netz absent (direct 0, Proton 502), Statusseite `lasair.lsst.ac.uk` HTTP 200 „Up"; `LASAIR_LSST_TOKEN` vorhanden (`.secrets.local:59`), unverified. In `phi/blocked_sources.φ` als `pending` registriert. (Schritt: Operator-Wort für die Exit-Rotation `bin/proton-wg.sh suggest api.lasair.lsst.ac.uk`, dann Query-Messung mit Token.)

An entscheid: Pipeline-Port force-Gate — kein sanktionierter Ort für den `--port`-Lauf (lokal strukturell verweigert, kein CI-Workflow fährt `--port`). Operator-Entscheid: lokaler Lauf des Release-Binärs auf den gitignorierten Korpora ODER CI-Workflow, der den Korpus trägt und `--port`+`--probe` fährt. (Schritt: Operator-Wort.)

