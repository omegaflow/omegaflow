# GPU-Prozess-Tod bei langen Shader-Läufen — Spec-Befund

Recherche 2026-08-14. Quellen: WebGPU-Spec (W3C CRD 2026-08-12),
Chromium-Quellen (`gpu/ipc/service/gpu_watchdog_thread.cc`, `gpu_watchdog_timeout.h`,
`result_codes.h`, `gpu_process_host.cc`), MozillaWiki Platform/GFX/WebGPU.

## Befund aus dieser Maschine

Chrome starb unter der Per-Pixel-Membran-Last (Intel HD 515, ~130 Quellen,
1280×800, ~3–7 s/Frame):

```
GPU process exited unexpectedly: exit_code=512
```

Danach: WebGPU `device.lost` → Sonde liefert 0
(„A valid external Instance reference no longer exists").

## Mechanik (offiziell)

1. **Die WebGPU-Spec setzt keine Laufzeitgrenze**, erlaubt der UA aber einen
   Watchdog (§ 2.1.9 Denial of Service):
   > „For GPU processing time, a WebGPU implementation may set up "watchdog"
   > timer that makes sure an application doesn't cause GPU unresponsiveness
   > for more than a few seconds."
   Und zum Device Loss:
   > „The device may become lost if shader execution does not end in a
   > reasonable amount of time, as determined by the user agent."

2. **Chrome erzwingt die Grenze extern** — `GpuWatchdogThread`: ein separater
   Thread beobachtet die GPU-Main-Threads per Arm/Disarm-Task-Observer.
   Timeouts (`gpu_watchdog_timeout.h`): Mac 25 s, Windows 30 s, **alle anderen
   Plattformen (Linux) 15 s**; Software-Rendering ×2. Auf Nicht-Windows gilt
   `kMaxExtraCyclesBeforeKill = 0` — **der erste verpasste Timeout tötet**, kein
   Gnaden-Zyklus.

3. **Der Kill:** `OnWatchdogTimeout` → `DeliberatelyTerminateToRecoverFromHang`
   → `TerminateCurrentProcessImmediately(RESULT_CODE_HUNG)` mit
   `RESULT_CODE_HUNG = 2` → `_exit(2)`. Der Browser reapt den rohen
   waitpid-Status: `2 << 8 = 512` → exakt unsere Meldung. Kein SIGSEGV — ein
   bewusster Kill bei Fristverzug.

4. **Firefox** hat kein Äquivalent zu `DeliberatelyTerminateToRecoverFromHang`
   für GPU-Tasks — lange Shader blockieren die Queue, ohne dass ein Timer den
   Prozess terminiert. Ein echter GPU-Verlust führt zu Software-WebRender-
   Fallback statt wiederholter Prozess-Zerstörung.

## Warum Nebra damals „schneller als Firefox" war

Der Nebra-Renderer war leicht (Punktwolke/Grid, Frames im Millisekunden-
Bereich) → der 15-s-Watchdog trat nie → Chromes V8 war schneller als Firefox'
JS → Chrome gewann den Tick-Vergleich. Die reine Per-Pixel-Membran dreht das
Verhältnis um: Frames im Sekunden-Bereich liegen jenseits der Frist, Chrome
tötet aktiv, Firefox läuft einfach länger.

## Mitigationen (offiziell konfigurierbar)

- **Chrome:** `--gpu-watchdog-timeout-seconds=<n>` (per Finch konfigurierbar;
  z.B. 60 s hebt die Frist über die Frame-Dauer). Der Verifikations-Start sollte
  den Flag tragen.
- Der Zustand bleibt: Der Tick ist die Kapazitätsmessung des
  Siliziums — der Watchdog-Flag verschiebt nur die Frist, nicht die Physik.
