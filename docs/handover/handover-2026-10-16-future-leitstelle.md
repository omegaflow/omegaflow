<!--
  title: Handover — Future / Leitstelle (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: b0129f6e156811effe3dfb04cdfc6da74818fadebf60d7469f0fce98793cfba6
  status: live
-->
# Handover — Future / Leitstelle (2026-10-16)

Operator-Wort: Crambler raus · ein Käfig, zwei Profile · Regel-Listen gestrichen.

## Gebaut
- **Crambler/Gateway raus**: Dienst (`llm-gateway.service`), Tray-Autostart,
  `bin/{gateway_tray,llm_gateway,crambler,crambler_clip}`, das gebaute und die Quelle
  (`tools/service/src/bin/llm_gateway.rs`), die `gw`-Verdrahtung. Backup in
  `omegaflow-work/backup-2026-10-16/`.
- **Ein Käfig, zwei Profile** (`bin/sandbox`): public = Repo + Werkstatt, privates Haus
  **nicht** eingehängt, `.secrets.local` überdeckt; private = dasselbe **plus** privates
  Haus (Einweg: privat sieht öffentlich, nicht umgekehrt). `agent_sandbox` und
  `private_shell` sind Zeiler darauf.
- **`permission.bash` gestrichen**: 345 → 0 Muster; Datei 19,3 → 3,3 KB. Wand = der Käfig ·
  die Fünf = `AGENTS.md` · das eine Gate = `smail`. Dazu die drei Agent-Prompts auf ihren Kern gekürzt (sie behaupteten die toten Deny-Wände).
- **UI-Chats raus** (Operator-Wort): der `chrome-devtools-voices`-Block (15 Chats: duck.ai, claude.ai, qwen, z.ai, kimi, minimax, aistudio, deepseek, mistral, lumo, perplexity, sakana) ist gestrichen. Der Rat kommt künftig **direkt über die API — nur die Frage, schnell**.

## Offen
- **Riss:** der GH_TOKEN wurde bei einer Messung gedruckt → **rotieren**.
- **Käfig-Lücken:** `~/backup-…` liegt außerhalb (Backups liegen in `omegaflow-work/`);
  `systemctl --user` hat im Käfig keinen Bus — Dienste sind dort nicht schaltbar.
- **B/C** (Komfort-Wrapper · Wände/Gates · eigenes Werk) unangetastet; die vier
  Linien-Lehren aus `post/in` warten auf den Rat.
- **Operator-Hand:** Antwort-Mails (KIT/re3data, UT Dallas) · CN-NSSDC-Datenantrag ·
  Token-Rotation.
