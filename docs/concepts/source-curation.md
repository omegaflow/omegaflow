<!--
  title: Source Curation — Der eine Pfad
  class: concept
  date: 2026-10-14
  sha256: 92c83002695cd636c1f699f52652dd315d5efcab4bc2bf5d2d5fff24d32f757f
  status: live
  see-also: AGENTS.md docs/SOURCE_PORT.md
-->
## Source Curation — Der eine Pfad

All source work (grind, port, curation) runs exclusively through
`docs/SOURCE_PORT.md` — the self-carrying protocol with state machine,
workflow procedure, reference map and path map. Die Grenze declined/blocked
ist die Frage, nicht die Quelle: `blocked` = Zugang (`key`/`account`/
`ip-blocked`/`parser-def`), `declined` = Verdikt (unphysikalisch/Modell/
Registry/**kommerziell**/superseded). **Der Konsument ist kein Kriterium** — die
Presence bewegt sich frei durch den 4D-Block; „kein gebauter Konsument" ist eine
Bau-Reihenfolge, kein Quellen-Verdikt. **`blocked key-needed` ist eine Messung, keine
Vermutung:** der Eintrag verlangt den gemessenen **401 MIT dem vorhandenen Token**
(`.secrets.local`) — ein 401 ohne Token ist kein Key-Gap (der IONEX-Eintrag
2026-09-16 widerlegt: mit `EARTHDATA_EDL_TOKEN` → HTTP 200). Erst messen, dann fordern.
**Authentifizierung ist kein Ausschlusskriterium** (Operator-Wort 2026-10-08): ein `blocked account`/`blocked key` ist **kein Quellen-Verdikt**, sondern eine `pending`-Operator-Aufgabe — wo der Operator zugangsberechtigt ist oder ein Key bereits besteht, wird die Quelle über die **Auth-Route** geführt (Registrierung/Key als Operator-Hand), nie als `declined` gestrichen. Gilt generell für Quellen.
Work surface: the flat `phi/` registers. The source-port backlog (`queue/`/`stage/`/`probe_*`/`ledger.φ`/`index.φ`) and the static catalog dispositions are `descoped` (2026-10-11) — live arms and `archive_search` supersede; the 12 real pending compilers stand in `state/wartend.φ`. Register: `phi/sources.φ` — the one data register; the `witness`/`record` blocks (the non-radiating witnesses) live inside it, the `field`/`force_type` blocks carry the force sources. The
registers are the queue: `register_lookup --open` surfaces the state registers
owner-tagged (`phi/sources.φ` + `nrs_stations.φ`;
offene Marker → mycelium, `asset fehlt` → mountain); every known open state maps to an owner (unit-tested: `tools/register/src/bin/register_lookup.rs`, `disposition_owner_maps_every_known_state` / `state_class_maps_every_register_state`).
Harvested data that stays belongs on the local machine under
`$OMEGAFLOW_WORK/sources/<netloc>/` (raw) and
`$OMEGAFLOW_WORK/sources/<netloc>/compiled/` (compiled assets) — one canonical
root, outside the code repo. `/tmp` carries only intermediates that may vanish
after the run; `data/`, `cache/` and `tmp/` inside the code repo are not data
homes and stay ignored. A kept dataset is never an unmanifested local file; its
durable home is the CDN asset registered as a `url`-line in `phi/sources.φ`
(the CDN-Manifestation duty below). CDN upload is allowlisted: PD, CC0, CC-BY,
CC-BY-NC, CC-BY-SA, OGL, Etalab, PDL, JP, own-work; every other token needs a
documented review (source, date, quote) before it leaves the machine. The
external archive root `archive-root` (physisch
heute `$HOME/archive/archive-root/`) holds only legacy material:
`handover/` (ein Altfund, keine laufende Praxis), `bundles/`, `concept-history/`,
`omegaflow-legacy/`, `omegaflow-legacy-backup-2026-09-02/`, `vanilla-dateidocs/`,
`commit_rewrite-2026-09-06/`; it is not the home of current harvests. The one
physical address lives here only; every
other document refers to `archive-root`. A
new session reads exactly that one document.

The tracked `phi/*.φ` set is the canon, declared in `REGISTER_CLASSES` (`src/gate/commit_gate.rs`). A new tracked `phi/*.φ` file is an architecture act — it needs the operator/council word and a `REGISTER_CLASSES` entry in the same commit; the canon gate blocks silent creation.

A register `note` carries the measured line only — at most 256 characters, no narration: the measurement tokens (code, hash, timestamp, host) stay, the story falls; the `note` is the measured evidence of the verdict, never documentation. `#` comments do not live in tracked registers (`prompt.φ` rule 1). The prose gate blocks new `note`-essays and register comments; the existing mass is relaxed register by register, never carried as a reason to inflate a new line. In `phi/sources.φ` the `note` directive is forbidden outright — it has no parser arm (writing it produces nothing); the Maschinen-Register carries directives only. The one data register carries directives only: since 2026-10-11 the witness blocks folded from the former `phi/witnesses.φ` into `phi/sources.φ` carry no `note` either (operator word — prose has no place in a data register; their blocks are `witness`/`url`/`record`/`force`). The gate blocks a new `note` in `phi/sources.φ` (`phi-sources-note`); over both stands the length/comment gate (`phi-note-length`, `phi-register-comment`).

### PII und Mail-Inhalte — nie getrackt

Personenbezogene Daten (private Adresse, private E-Mail, Login-Konstrukte,
Kontakt-Kombinationen) und Mail-Inhalte (Entwürfe, gesendete Briefe, Anträge mit
privater Korrespondenz) sind **nie getrackt**. Heimat: `state/mail/` (gitignored)
für Entwürfe/Briefe, `.secrets.local` für Zugangsdaten und Infrastruktur-IDs
(Cloudflare `account_id`). Getrackte Aufträge sind redigiert: Rolle/Institution
statt Name, Platzhalter statt Adresse, Verweis (`state/mail/<datei>`) statt
Brieftext. Öffentliche Attribution bleibt: Name im Copyright, Papier-Seal,
zitierte Autoren. Die Entscheidungsregel: **öffentliche Rolle + öffentliches Werk
= bleibt; privater Kanal = geht.** Finanz-/Funding-Inhalte des Operators
(Förderstrategie, Bewerbungen, finanzielle Lage, persönliche Nutzungs-/
Kostenmuster) sind ebenso **nie getrackt**; Heimat das private Repo `omegaflow/private`,
auf `state/` verwurzelt: `future/` (Future-Linie: Förder-Akte, `handover/`, Register) und `mail/`
= `state/mail/`, der kanonische Postkorb (gelesen in
`tools/service/src/bin/smail_recv.rs`, `mail_watchdog.rs`, `mail_digest.rs`,
`tools/register/src/bin/open_points_check.rs`). Ein zweites Mail-Heim existiert
nicht: `state/future/mail/` ist entfernt; Mail-Dateien zwischen zwei Verzeichnissen
zu kopieren ist verboten.
Die Sache bleibt: Programm, Frist, Eligibility; die Kosten der Maschine
(Hardware-BOM, Modell-Benchmark) bleiben Engineering-Daten. Jede neu gefundene Verletzung wird
Gate-Fixture in `src/gate/commit_gate_vocab.json` und Gate-Test im selben Atom;
der Befund wird gemessen (`pii_scan`), nicht nur beschrieben.

**Die Future-Übergabe ist vollständig privat (Rat + Operator-Wort, 2026-09-22).**
`docs/handover/` trägt keine neue `future`-Folge — Futures Register (Folge-N,
Operator-Queue, Archiv) liegt im privaten Repo `omegaflow/private` unter
`handover/` (gleiche Benennung, gleicher Header, gleicher `archiv/`-Move; lokal
unter `state/`, gitignored). Öffentlich ist von Future nur die Adresse, nie der
Inhalt. Die Alt-Übergaben folge83–90 sind aus der öffentlichen Historie getilgt
(History-Rewrite 2026-09-22) und liegen privat im `handover/archiv/`.

### CDN-Manifestation — eine Session-Duty (Sitzung nicht mehr nur lokal)

The CI manifestator (`kernel-flatten.yml`, `--ci-mode`) is the only writer of
the canonical CDN assets. A session that harvests a **new or changed dataset**
does not close that work while the dataset exists only on the local machine:
it is a register duty to add/update the source in `phi/sources.φ` so the CI
manifestator brings the asset to the shared CDN (the durable home every later
session reads). A finished-but-unmanifested harvest is a register debt, not a
checkmark. Consequence of the code as built: every compiler gates the CDN
upload behind an explicit `--ci-mode` — a local run alone never feeds the
shared memory. The session closes the harvest only when the asset is
registered for manifestation (or the operator names the pending explicitly).

**Lokal-only — nicht offene Lizenz, nur für Experimente (Operator-Wort, 2026-10-13).**
A source whose data carry **no open licence** may be kept and used **locally** —
a working copy under `$OMEGAFLOW_WORK/data/` (outside the repo) for calibration, experiments and
validation — but it is **never manifested**: no `phi/sources.φ` line, no CDN
release, no public `origin`/`url`/`compiler` surface. The disposition is
`local-only`, a state beside `descoped`/`declined` (the source is used, not
released). The operator's word is the gate: *"so wie mit allem lokal für
Experimente sind nicht offene Lizenzen ok, aber nicht veröffentlichen"*
(OCNS2019, Neptun-Astrometrie-Kalibrierung). A local-only dataset is named in the
owning line's handover (path · purpose · licence state), never registered as a
CDN asset; a compiler must refuse it (`--ci-mode` upload only for a registered
source). The decisive line is **use vs. release**, not licence openness: non-open
data may inform locally, it may never leave the machine.

