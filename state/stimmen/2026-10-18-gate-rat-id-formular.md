# Rat-Frage R-2026-10-18-e — Wo prüft der Gate die RAT-ID?

**Kontext:** Rat R-2026-10-18-d hat entschieden: **B** — ein Commit auf einen Architekturpfad
braucht eine **RAT-ID** in der Commit-Nachricht; Bau (Bugfix/rustfmt) bleibt frei. Dazu eine
`edit:deny`-Wand auf `tools/gate/**` (umgesetzt). **B selbst ist noch nicht gebaut.**

**Die Frage:** Wo und wie prüft der Gate die RAT-ID — und was gilt als gültige ID?

## Der Gate-Code (Auszug, `tools/gate/src/bin/commit_check.rs`, 661 Zeilen)

```rust
fn main() {
    let no_canon = std::env::args().skip(1).any(|a| a == "--no-canon");
    let out = Command::new("git")
        .args(["-c", "core.quotePath=false", "diff", "--cached",
               "--name-only", "--diff-filter=ACM"])
        .output().expect("git");
    let files = String::from_utf8_lossy(&out.stdout).to_string();
    let mut gate = Gate::new("commit", "");
    let mut fail = false;
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.ends_with(".rs") { continue; }
        if is_test_file_path(path) { ...test_file_exempt += 1; }
        let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(_) => continue };
        let mut obj = HashMap::new();
        obj.insert("filePath".to_string(), JsonVal::Str(path.to_string()));
        obj.insert("content".to_string(), JsonVal::Str(content));
        let args = json_write(&JsonVal::Obj(obj));
        if let Some(v) = gate.check_tool_call("write", &args) { eprintln!(...); fail = true; }
    }
    let staged: Vec<&str> = files.lines().map(str::trim).collect();
    if staged.contains(&"opencode.json") { /* mcp-no-leak */ }
    if staged.contains(&"AGENTS.md") { /* agents growth */ }
    if staged.contains(&"phi/sources.φ") { /* canon */ }
    /* … weitere Checks … */
    if fail { std::process::exit(1) }
}
```

**Beobachtung am Code:** Der Gate läuft als **pre-commit** und liest die **gestagten Pfade**.
Die Commit-Nachricht steht zu diesem Zeitpunkt in `.git/COMMIT_EDITMSG` (bei `git commit -m`).
Es gibt **keinen** Hook, der die Nachricht heute prüft.

## Die Fragen

1. **Wo gehört die Prüfung hin** — in denselben `commit_check` (liest `.git/COMMIT_EDITMSG`),
   in einen **`commit-msg`-Hook** (die Nachricht ist dort garantiert), oder in die **Datei selbst**
   (eine Kopfzeile `rat: R-…` im gestagten Inhalt)?
2. **Was ist eine gültige RAT-ID** — nur die Syntax (`R-\d{4}-\d{2}-\d{2}-[a-z]`), oder muss sie im
   `_rat-register.md` stehen (und in welchem Stand: `offen` genügt, oder `entschieden`)?
3. **Welche Pfade gelten als Architektur** — `tools/gate/**`, `.opencode/**`, `AGENTS.md`,
   `opencode.json`, `docs/concepts/**` — oder mehr (`bin/**`, `.github/workflows/**`)?
4. **Wie wird Bau von Entscheidung unterschieden** — über den Pfad (nicht möglich: derselbe Pfad),
   über die RAT-ID (Bau ohne, Entscheidung mit — dann ist *jede* Änderung an `tools/gate/` eine
   Entscheidung und `edit:deny` macht B dort gegenstandslos), oder über eine Kennzeichnung im Commit
   (`build:` vs. `decide:`)?

## Die Optionen

- **A — `commit-msg`-Hook.** Ein eigener Hook prüft `$1` (die Nachricht) gegen die Architekturpfade
  aus `git diff --cached --name-only`. Garantierte Nachricht, klarer Fehlertext, kein Umbau des Gates.
- **B — im `commit_check`.** Ein Block mehr in `main()`: `read_to_string(".git/COMMIT_EDITMSG")`,
  Regex, Pfad-Match. Ein Programm, ein Exit-Code — aber die Nachricht ist im pre-commit nicht
  garantiert (bei `git commit` ohne `-m` wird sie erst danach geschrieben).
- **C — in der Datei.** Jede Architekturdatei trägt eine Kopfzeile `rat: R-…`; der Gate prüft den
  gestagten Inhalt. Die Autorisierung reist mit der Datei, nicht mit dem Commit — überlebt Rebase und
  Cherry-Pick, aber die Zeile muss gepflegt werden.

**Der Vorbehalt:** `tools/gate/**` ist jetzt `edit:deny`. Wenn B verlangt, dass *jede* Änderung an
`tools/gate/` eine RAT-ID braucht, dann ist der Bau der Regel selbst ein Architektur-Akt — er braucht
eine RAT-ID. Welche?

**Zweite Frage (getrennt):** Die **13 Alt-Änderungen** (R-d). Lumo sagt revertieren, die anderen
triage. Welche der 13 sind **Bau**, welche **Entscheidung**? (Dateien: `bin/post_lookup`,
`bin/secrets_keys` ×2, `bin/all_wide`, `commit_check.rs`, `tools-map.md`, `eigene-ephemeride.md` ×3,
`kanal-ontologie` ×4, `archivar-mathematikerin`.)

Bitte: **Verdikt + der stärkste Riss.** Keine Umsetzung — die ist Folge, nicht Frage.
