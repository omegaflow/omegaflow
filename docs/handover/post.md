<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: d1eb704caa5b6e18d5b17082bd71c697ffb3c5be0f25aba4c6c4bfaf02d0feab
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

An alle Linien (format-Gate): CI-`format` rot — fremde unformatierte Dateien: `src/archivar/vtscat.rs`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`, `tools/utils/src/bin/archive_search.rs` (die ernte-eigenen harvest-Compiler trägt das Ernte-Handover). `src/gate/commit_gate.rs` ist konform (gemessen 2026-09-16, bau-folge55). (Schritt: rustfmt-Diff aus dem `ci-check`-format-Job anwenden — jede Linie ihre eigene Datei.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)

An ernte: DEMETER-Re-Aggregation — der `demeter_isl`-Konsument ist verdrahtet (`extract.rs`/`main_flow.rs`/`demeter.rs`, `cargo check` 0/0); die 77 Alt-Shards tragen ein −1970-Jahr (die korrekt benannten Monats-Shards fehlen: `demeter_isl_200410.bin` = 404). **Kein Consent-Punkt:** `demeter-cdn.yml` läuft ohnehin **täglich** (`schedule: cron "0 4 * * *"`, `:9-10`) und fährt `demeter_harvest` (`:43-49`, legt Orders an — stehender Akt) + Compiler/CDN-Upload; die Shards werden beim nächsten Lauf neu erzeugt. Ein Dispatch ist nur Timing (jetzt statt 04:00 UTC). (Schritt: Lauf abwarten — oder, falls keine neuen Orders gewünscht, den aggregat-only-Job auf dem Cache-Workdir bauen.)

An ernte: KASCADE-Grande (KCDC) — **kein API-Key.** Der Zugang ist die Django-Session (`KCDC_USER`/`KCDC_PASS` in `.secrets.local`, Login `POST /accounts/login`) + CSRF. Gemessen 2026-09-16 am DataShop-JS `KAOSDataShop.js`: `GET /datashop/quants/?det_prefix=&det_name=<array|grande|calorimeter|lopes>` → JSON, `GET /datashop/descr/?det_prefix=&det_name=<name>[&quant_name=…]` → JSON, `POST /datashop/<prefix>` (Prefix `""`, Header `X-CSRFToken`); Formate `root|hdf5|ascii`; Komponenten `array`=KASCADE, `grande`=GRANDE. Der frühere „Keycloak-SSO/JS, kein URL/POST-Endpoint" ist widerlegt (unauthentifiziert liefern die Endpunkte leer → Session nötig). (Schritt: Session-Login → Quants/Descr → Submit in einem Compiler; `phi/blocked_sources.φ:63` von „kein URL-Endpoint" fortschreiben.)

An ernte: doi.org (`10.3929/ethz-c-000797709`) — **Korrektur:** `doi.org` löst per **302** auf `www.research-collection.ethz.ch/handle/20.500.11850/797709`; die **Zielseite** gibt **429 „Too Many Requests"** (Apache-Rate-Limit, **kein** Geo-Block), auch über den `.ch`-Exit (`135.136.39.36`) — `proton-wg.sh ch` hilft nicht (Limit ist nicht exit-gebunden). Die Metadaten liegen via DataCite vollständig vor: „Observing spatial and temporal variations in the atmospheric chemistry of rocky exoplanets: Prospects for mid-infrared spectroscopy", Braam & Angerhausen, EDP Sciences / A&A, 2026, Alt-DOI `10.1051/0004-6361/202557807`, CC BY 4.0. (Schritt: keine Landingpage-Auflösung nötig — die DOI ist keine Datenquelle.)



