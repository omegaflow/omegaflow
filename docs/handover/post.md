<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: c5d586fb0d8a105e0922fc028018d8a96860b228206a0ab86fa15e8bf1fe6c09
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

An bau: `text_review` (measure-Bin, `tools/measure/src/bin/text_review.rs`, committet `197bccf1`) erreicht `tools-latest` nicht — `tools-build.yml` baut/publiziert nur register/utils/gate/omegaflow, keinen measure-Bin; `bin/.tools_ensure text_review` liest `pending — absent from the tools-latest manifest`. Die folge77-Annahme „Release kommt mit `tools-build`" ist falsifiziert. Lage: der Lauf auf den Förder-Entwürfen ist dadurch blockiert. Braucht: `text_review` in `tools-build.yml` eintragen (build-Step + `tools.manifest`-Zeile + `gh release upload`), oder eine measure-build-Route nennen. (Schritt: Workflow-Hunk.)

An entscheid: DEMETER ISL (CDPP) ist operator-gebunden — Lage: `phi/blocked_sources.φ:70`, Re-Messung 2026-09-21: rs-order/rs-catalog GET+POST 403 ohne Token (Jetty), CDPP nennt „valid email necessary", ISL-Route account-gated; Braucht: Konto/Zugang bei CDPP. (Schritt: `state/mail/`-Entwurf vorbereiten + `smail --dry-run` verifizieren, dann Operator-Wort.)

