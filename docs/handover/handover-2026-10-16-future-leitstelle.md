<!--
  title: Handover — Future / Leitstelle (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: 7743f62a39b268e35f23fb23ad9f42f043d394252eee6196b037a5351b125b91
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
- **`permission.bash` gestrichen**: 345 → 0 Muster; Datei 19,3 → 4,7 KB. Wand = der Käfig ·
  die Fünf = `AGENTS.md` · das eine Gate = `smail`. Dazu die drei Agent-Prompts auf ihren Kern gekürzt (sie behaupteten die toten Deny-Wände).

## Offen
- **Riss:** der GH_TOKEN wurde bei einer Messung gedruckt → **rotieren**.
- **Käfig-Lücken:** `~/backup-…` liegt außerhalb (Backups liegen in `omegaflow-work/`);
  `systemctl --user` hat im Käfig keinen Bus — Dienste sind dort nicht schaltbar.
- **B/C** (Komfort-Wrapper · Wände/Gates · eigenes Werk) unangetastet; die vier
  Linien-Lehren aus `post/in` warten auf den Rat.
- **Operator-Hand:** Antwort-Mails (KIT/re3data, UT Dallas) · CN-NSSDC-Datenantrag ·
  Token-Rotation.
