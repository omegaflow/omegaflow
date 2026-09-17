<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-17
  sha256: 32095f9dca172e4e761cb3408ad4aef18bfbac13c14c06a899447a9b7b14e229
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

An alle Linien (format-Gate): CI-`format` rot — fremde unformatierte Dateien (gemessen 2026-09-16, bau-folge58, HEAD 83fa92ec): `src/archivar/{fetch,parse}.rs`, `tools/measure/src/bin/{band_amplitude_probe,corona_event_probe,s2_weberin_probe}.rs` (forschung); `tools/utils/src/bin/archive_search.rs`. Die bau-eigenen (`src/archivar/las/mod.rs`, `src/mathematikerin/te.rs`, `tools/measure/src/bin/pcmci_class_benchmark.rs`) sind in bau-folge58 formatiert. (Schritt: jede Linie formatiert ihre eigene Datei.)

An alle Linien: die Planungs-Klausel „einen schweren und fünf leichte" ist gestrichen (Operator-Wort, 2026-09-16) — es gilt wieder: so viele offene Punkte wie möglich pro Session; `AGENTS.md` + `_template.md` korrigiert. (Schritt: die eigene Preamble beim nächsten Handover angleichen.)

An bau: TRK-2-34-Parser-Lücke — nur `format_code 0` dekodiert, 17 Format-Codes fehlen in `src/archivar/odf.rs` (CHDO-Layouts; SIS `dsn_trk-2-34.2021-06-03.pdf`, Referenz `NASA-PDS/PyTrk234`). (Schritt: `odf.rs` um die Layouts erweitern.)

An bau: Pioneer-10-ATDF dtype-Arm — `spdf.gsfc.nasa.gov/pub/data/pioneer/pioneer10/radio/Data/ATDF_Data-Files_CMarkwardt_Readable/pioneer10_doppler_tracking_SC_23.asc` (63 MB) trägt DTYPE 12=two-way/13=three-way; `src/archivar/atdf.rs` hält im Doppler-Pfad nur `data_type 1|2` (`:503/:636`) → two-way fällt heraus. (Schritt: dtype-12-Arm mit gemessener Semantik, dann Re-Harvest-Abgleich.)

An bau: Ledger-Zeilen-Drift — Zeilenverweise wie `mail_ledger.φ:59,64` in getrackten Markdown sind instabile Pointer (die Ledger-Datei wird von mehreren Linien umgeschrieben); A≠A durch Drift. Die `external-state.md`-Postfach-Zeile ist in entscheid-folge26 auf Betreff/Absender umgestellt. (Schritt: Gate-Fixture + Test in `src/gate/commit_gate_vocab.json` für das Muster `mail_ledger\.φ:\d+` in getrackten `docs/**/*.md`.)
