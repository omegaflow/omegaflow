<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau17)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: b0035094a8356e188c8c203166868b012702e77775b0d6f641b18f29c3abd6bc
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **ESP32-Modul — on hold** (Operator-Wort, 2026-09-13): das Gerät und sein
  Flash kommen zuletzt — zuerst laufen Software und Membranen. Der Binär baut
  (Xtensa-Toolchain in `~/.rustup/toolchains/esp/`, Linker `xtensa-esp32s3-elf-gcc`
  vorhanden; `cargo build --release` mit gesourctem `export-esp.sh`). Offen, wenn
  das Modul an der Reihe ist: `espflash` installieren und das Gerät anstecken
  (heute kein ESP32 enumeriert — `lsusb` ohne Espressif/CP210x/FTDI); dann läuft
  der nn-Strom als `nn=<ms>` am ttyACM. Die kuratierte BOM
  (`docs/specs/mantis-shrimp-bom.md`) und der AliExpress-Warenkorb (45 Artikel)
  stehen bereit.

## Weberin — Bau-Linie

- Neptun-Bau-Linie — der zweite Planet derselben Komposition wie Uranus
  (`--uranus-c-spk`-Stil): Compiler + `sources.φ`-Registrierung +
  kernel-flatten-Schritt; benannt pending im Uranus-Paper
  (`uranus-rift-ephemerides.md`).

## Recherche-Werkzeug — `archive_search`-Ausbau (Operator-Wort, 2026-09-13)

`tools/utils/src/bin/archive_search.rs` wächst zum einen Recherche-Instrument —
intern + extern, ultra-billig, token-frei. Der Rat hielt den Blatt; der Operator
entschied. Gesperrt:

- Name bleibt `archive_search` — auch als Operator-Suchfenster (Rat: keine zweite
  Taufe; der Bin durchsucht in jedem Modus Archive). Selbstversorgung — eigenes
  curl (Muster `sfetch`) und eigener `.secrets.local`-Reader im Bin, Keys lesen
  nie ausgeben, kein Kern-Import.
- Automatische fünfstufige Leiter (direkter curl → proton0-Tunnel → r.jina.ai →
  Wayback/CDX → s.jina.ai) mit gedrucktem Stufen-Protokoll; Proton-VPN ist die
  Netz-Ebene der Eskalation (SOURCE_PORT §13: Tunnel `proton0`, Exit-IP-Re-Route
  gegen Geo-/IP-Block, für Roh-Assets, die der Jina-Reader nicht transportiert).
- OSearch (`~/projects/osearch`) bleibt stehen; seine Fähigkeiten werden
  portiert: MFT (`ntfs.rs`), Index-Cache, `--serve`-Web-UI (Operator-Suchfenster),
  stdin-Loop, `--binary`.
- Register-Weg: der Operator nutzt das Tool auch für eigene Recherchen; eine
  gefundene Zeile übergibt er der Session (die Session trägt sie ins Register) —
  das Werkzeug schreibt nie.
- Lokale Suche: sofort-mit-tippen (120-ms-Takt), mit Toggle ein/aus; Netz bleibt
  Submit-only.
- Grenzen: kein LLM-Call, kein Token, keine Modell-Verdikte; druckt
  Register-fertige Zeilen, schreibt nie; `declined`/`not-published` bleiben der
  Kybernautin; kein Verdict-Cache (jede Zeile trägt ihr Messdatum); kein Python,
  kein neues Crate.
- `--serve` = der eine Operator-Eingang für alle Modi (Rat). Zwänge: Anzeige nie
  Schrift (kein Schreib-Endpunkt, dieselben Register-fertigen Strings wie die
  CLI); die Leiter-Stufen bleiben sichtbar (kein Spinner); Netz nur auf
  ausdrücklichen Abschick; Keys überqueren die Seite nie (Server liest, JS sieht
  nur Ergebnisse); Serve-Disziplin (127.0.0.1, Vordergrund, kein Auto-Öffnen,
  kein Autostart, Tests ephemerer Port + sofort schließen); Modus-Wähler als
  Checkbox-Leiste (eine Bar, nicht ein Regal von Knöpfen).

Modi: lokal `plain` (+ Skip-Zählung, `--binary`, stdin-Loop, Toggle-Takt) ·
`--leads` · `--mft` · `--index` (opt-in, mtime-invalidiert, Inhalt immer frisch)
· `--git` (eigenes Git-Archiv: `git log`/`grep` über alle Refs) · `--serve`
(Operator-Eingang, alle Modi); Netz `--verdict` · `--arxiv` · `--ads`
(`NASA_ADS_TOKEN`) · `--ntrs` (frei) · `--wayback` · `--crossref` · `--wiki`
(frei) · `--github` (Repo-/Code-Suche, `OMEGAFLOW_TOKEN`) · `--crates`
(crates.io) · `--librs` (lib.rs).

EIN Atom, komplett in einer Session — keine Phasen, kein „später" (Sub-Agenten
tragen den eigenen Kontext, die Liste ist keine Last). Am Ende des Atoms stehen
alle Modi: `--verdict` (5-stufig, Register-tauglich) + Skip-Zählung + `--binary`
+ stdin-Loop + Toggle-Takt + OSearch-Port (`--mft`/`--index`/`--serve` mit
Checkbox-Leiste) + `--arxiv` (ATOM-Parser, Muster `tools/science/arxiv.rs`) +
`--ads` + `--ntrs` + `--wayback` + `--crossref` + `--wiki` + `--github` +
`--crates` + `--librs` + `--git`.

Verifikation: `cargo check` 0/0, stille Tests (Fixtures, keine Netz-Pflicht in
der Default-Suite), ein manueller `--verdict`-Lauf als Messung.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit beim Schreiben dieser Übergabe:
`.github/workflows/hinet-cdn.yml`, `docs/concepts/die-akteure-im-boden-und-wasser.md`,
`tools/harvest/src/bin/hinet_win32_compiler.rs` — unberührt.
