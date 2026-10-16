<!--
  title: Handover — Future / Leitstelle (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: 06aa7bc9ffb2add68b0116a492920532bb28670d0454077f63c4cbe1a9acde5a
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
- **Rat über API (bald):** `bin/rat_dispatch` zeigt noch auf den entfernten Gateway
  (127.0.0.1:1621) → auf direkte APIs umstellen; `ui-seats.md` (259 Zeilen UI-Bedienung)
  wird zur kurzen `seat → modell`-Liste. Operator-Wort: der **Kanal** wird die API —
  das GEFÜTTERT-Wort **bleibt**.
- **B/C** (Komfort-Wrapper · Wände/Gates · eigenes Werk) unangetastet; die vier
  Linien-Lehren aus `post/in` warten auf den Rat.
- **Operator-Hand:** Antwort-Mails (KIT/re3data, UT Dallas) · CN-NSSDC-Datenantrag ·
  Token-Rotation.
