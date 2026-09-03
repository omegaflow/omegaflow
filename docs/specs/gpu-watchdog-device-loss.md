<!--
  title: GPU process termination on long shader runs — spec finding
  class: concept
  date: 2026-08-14
  sha256: f18f60b0e8a43bea0ba4d2683b397e214487dad7df925c84c2c01a3d90e933e3
  status: live
-->
# GPU process termination on long shader runs — spec finding

Research 2026-08-14. Sources: WebGPU spec (W3C CRD 2026-08-12),
Chromium sources (`gpu/ipc/service/gpu_watchdog_thread.cc`, `gpu_watchdog_timeout.h`,
`result_codes.h`, `gpu_process_host.cc`), MozillaWiki Platform/GFX/WebGPU.

## Finding from this machine

Chrome terminated under the per-pixel membrane load (Intel HD 515, ~130 sources,
1280×800, ~3–7 s/frame):

```
GPU process exited unexpectedly: exit_code=512
```

After that: WebGPU `device.lost` → the probe delivers 0
(„A valid external Instance reference no longer exists").

## Mechanics (official)

1. **The WebGPU spec sets no runtime limit**, but allows the UA a
   watchdog (§ 2.1.9 Denial of Service):
   > "For GPU processing time, a WebGPU implementation may set up "watchdog"
   > timer that makes sure an application doesn't cause GPU unresponsiveness
   > for more than a few seconds."
   And on device loss:
   > "The device may become lost if shader execution does not end in a
   > reasonable amount of time, as determined by the user agent."

2. **Chrome enforces the limit externally** — `GpuWatchdogThread`: a separate
   thread observes the GPU main threads via an arm/disarm task observer.
   Timeouts (`gpu_watchdog_timeout.h`): Mac 25 s, Windows 30 s, **all other
   platforms (Linux) 15 s**; software rendering ×2. On non-Windows
   `kMaxExtraCyclesBeforeKill = 0` — **the first missed timeout terminates**, no
   grace cycle.

3. **The termination:** `OnWatchdogTimeout` → `DeliberatelyTerminateToRecoverFromHang`
   → `TerminateCurrentProcessImmediately(RESULT_CODE_HUNG)` with
   `RESULT_CODE_HUNG = 2` → `_exit(2)`. The browser reaps the raw
   waitpid status: `2 << 8 = 512` → exactly our message. No SIGSEGV — a
   deliberate termination on deadline miss.

4. **Firefox** has no equivalent to `DeliberatelyTerminateToRecoverFromHang`
   for GPU tasks — long shaders block the queue without a timer terminating the
   process. A real GPU loss leads to a software WebRender
   alternate path instead of repeated process destruction.

## Why Nebra was „faster than Firefox" then

The Nebra renderer was light (point cloud/grid, frames in the millisecond
range) → the 15 s watchdog never fired → Chrome's V8 was faster than Firefox's
JS → Chrome won the tick comparison. The pure per-pixel membrane turns the
ratio around: frames in the seconds range lie beyond the deadline, Chrome
terminates actively, Firefox simply runs longer.

## Mitigations (officially configurable)

- **Chrome:** `--gpu-watchdog-timeout-seconds=<n>` (configurable per Finch;
  e.g. 60 s lifts the deadline above the frame duration). The verification start
  carries the flag.
- The state remains: the tick is the capacity measurement of the
  silicon — the watchdog flag only shifts the deadline, not the physics.
