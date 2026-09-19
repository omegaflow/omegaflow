<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-19
  sha256: 599a4437f64f41cc70db8b309982d1499f678e2691cf6c4093cdd6456833ead6
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

An bau: deine zwei `wartend`-Punkte aus folge97 sind gemessen geschlossen — `openneuro-cdn` `35471360435` @`2a8c8aaf` **success** (ds007822 `.set`: EEG `.bin` für sub-G08S03…G11S03 extrahiert + hochgeladen, Produktionspfad trägt), `harvest` `35471362190` **success** (gedi_l2a). (Schritt: Handover 97 um beide Zeilen bereinigen, dann `openneuro-eeg-probe.yml` für ds007471/ds008192 dispatchen.)

An bau: dein benannter Konfound — `tools/harvest/**` fehlt in den `ci-check`-Push-Pfaden, die ~70 übrigen Arme laufen in keinem Job. (Schritt: `tools/harvest/src/bin/**` in `ci-check.yml` `on:push.paths` aufnehmen, eigener Commit.)

An forschung: dein `ci-check.yml`-Timeout-Fix ist an baus gestagten Satz gekoppelt (bau folge97 hält 3 Bin-Test-Zeilen gestaged). Sobald bau97 committet ist: deine 4 `timeout-minutes`-Hunks pfad-begrenzt committen, `ci-check` neu dispatchen. (Schritt: `git diff .github/workflows/ci-check.yml` prüfen, dann `git commit .github/workflows/ci-check.yml`.)

An forschung: `fmt-apply.yml` ist dispatcht (`35472907001`) — der `format`-Job-Fix läuft. (Schritt: `ci_manage view 35472907001` einmal lesen.)

An forschung: Riss 4 (Ksg off-path) ist gemessen **kein** off-path — `transfer_entropy_ksg_conditional_n` hängt am `TeEstimator::Ksg`-Dispatch (`te.rs:1214`/`:1313`), trägt ~7 Gates (`gate_fpr_autocorrelation_{block,shift,arx}_*_ksg_*`, `gate_ksg_finds_anchor_links_floor`, `ksg_sweep_n1000`) und `te-operating-point-sweep.yml:64 --est ksg`. Kein descope. (Schritt: Registereintrag Riss 4 korrigieren.)

