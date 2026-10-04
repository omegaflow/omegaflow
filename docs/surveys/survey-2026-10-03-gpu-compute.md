<!--
  title: Survey — GPU-Rechenzeit: freie Wege und GitHub-Runner (2026-10-03)
  class: survey
  date: 2026-10-03
  sha256: a39fae365a2145f03b88fa1f6ab935eec3df4fd5496974e2ec0b7b210753035d
  status: live
  see-also: docs/concepts/github-pipeline.md state/future/bewerbungen-vs-zai-export.md state/future/survey-funding-pflichtfrei.md
-->
# Survey — GPU-Rechenzeit: freie Wege und GitHub-Runner

Antwort auf die Operator-Frage (2026-10-03): *„brauche ich einen Monat mehr runner und
kann ich github upgraden um auch gpu läufe machen zu können?"* Zuvor lag nur ein Stub
(`state/future/survey-funding-pflichtfrei.md:25-27`) und die GPU-Zeile in
`docs/concepts/github-pipeline.md:38,102-105,125`. Jede Zahl hier ist am 2026-10-03 über
`archive_search` gemessen; Unbelegtes steht als `unbelegt`.

## 0. Antwort vorweg

- **Mehr CPU-Runner-Minuten: nein.** Öffentliche Repos fahren auf GitHub-*Standard*-Runnern,
  die frei und unbegrenzt sind. Der private Baum (`state/`) hat **keine** Workflows, also keine
  privaten Minuten. Der Engpass ist lokal, nicht die CI — der Operator lädt Jobs aus, um am
  Rechner arbeiten zu können (`state/future/bewerbungen-vs-zai-export.md:215-221`).
- **GitHub für GPU: nur bezahlt.** GPU gibt es ausschließlich als *larger runner*, nur für
  **Org/Enterprise auf Team oder Enterprise Cloud**, und **immer kostenpflichtig — auch in
  öffentlichen Repos**.
- **Kostenloser GitHub-GPU-Weg existiert nicht.** Der einzige GitHub-gebührenfreie Pfad ist
  **self-hosted** auf eigener Hardware (Actions-Gebühr entfällt; GPU stellt der Host).

## A. GitHub Actions — GPU-Runner (gemessen)

- **Eligibility:** larger runners für Organisationen/Enterprises auf **GitHub Team** oder
  **GitHub Enterprise Cloud**; personal Free/Pro reicht nicht. Kreditkarte + Spending-Limit > 0
  nötig. — `docs.github.com/en/actions/concepts/runners/larger-runners` (HTTP 206);
  `…/actions/reference/runners/github-hosted-runners` (HTTP 206), 2026-10-03.
- **Hardware:** der einzige geführte GPU-Runner ist **4 vCPU · 1× Tesla T4 · 28 GB RAM ·
  16 GB VRAM · 176 GB SSD · Ubuntu/Windows**. — `raw.githubusercontent.com/github/docs/main/content/actions/reference/runners/larger-runners.md`
  (HTTP 206, Z. 45-49). GA: `github.blog/changelog/2024-07-08-github-actions-gpu-hosted-runners-are-now-generally-available/` (HTTP 200).
- **Preise (per Minute):** `linux_4_core_gpu` **$0.052/min**, `windows_4_core_gpu`
  **$0.102/min**; Aufrundung auf die volle Minute, keine Idle-Kosten. —
  `docs.github.com/en/billing/reference/actions-runner-pricing` (HTTP 206), 2026-10-03.
- **Public-Repo-Regel:** „Included minutes cannot be used for larger runners"; „The larger
  runners are not free for public repositories"; „Larger runners are always charged for, even
  when used by public repositories or when you have quota available". — ebenda.
- **`runs-on`:** bei Linux/Windows der **selbst vergebene Runner-Name** (kein festes
  GitHub-Label); Targeting per Runner-Group, Label oder beidem. Feste Labels `gpu_1x_a100` /
  `gpu_1x_h100` sind in der aktuellen Doku **nicht belegt → `unbelegt`**.
- **Limits:** larger runner bis 1000 gleichzeitige Jobs, davon **max. 100 gleichzeitige
  GPU-Jobs**. — `raw.githubusercontent.com/github/docs/main/content/actions/reference/limits.md` (HTTP 206).

## B. Kostenlose / günstige GPU-Anbieter (gemessen)

| Anbieter | freies Kontingent | GPU | headless/Batch? | Grenze | Quelle (HTTP, 2026-10-03) |
|---|---|---|---|---|---|
| **Kaggle Notebooks** | **30 h/Woche** (Reset wöchentlich) | Tesla **P100**, zusätzlich T4×2 | **ja** — offiziell „use the Kaggle API to avoid interactive sessions"; `kaggle kernels push` | 60 min idle; 20 GB Work-Dir | `kaggle.com/docs/efficient-gpu-usage` (200) |
| **Google Colab** | „free of charge"-Tier; GPU-Zugang „heavily restricted" | T4 (typisch) | **nein** — kein offizieller API/CLI-Weg | Limits **unveröffentlicht**, idle/max-Lifetime, GPU „vary over time" | `research.google.com/colaboratory/faq.html` (206) |
| **Hugging Face ZeroGPU** | **5 min/Tag** Free (unauth 2, PRO 40) | RTX Pro 6000 Blackwell | **nein** — nur gehostete Gradio-Spaces | quota-gebunden | `huggingface.co/docs/hub/en/spaces-zerogpu` (200) |
| **Hugging Face Jobs** | **kein** Gratis-GPU (pay-as-you-go) | T4/L4/L40S/A100 | ja (`hf jobs run`), kostenpflichtig | Guthaben nötig | `huggingface.co/docs/hub/jobs-pricing` (200) |
| **Modal** | **$30/Monat Gratis-Compute** | serverless T4…B200, per-second | **ja** — Python-SDK/CLI, Container | $30/Monat-Credit | `modal.com/pricing` (206) |
| **Lightning AI** | ~**80 GPU-h** einmalige Credits (interruptible) | T4/L4/L40S | **ja** — CLI/Studio, CI-tauglich | Credits **verfallen 12 Mon.** | `lightning.ai/pricing` (206) |
| **Paperspace Gradient** | Free-GPU+ (M4000) | M4000 | teilweise (API existiert) | **6 h/Session**, Notebooks **public**, „limited pool" | Paperspace-Docs (206) |
| **RunPod** | kein Free-Tier | RTX 4090/A100/H100 | **ja** — API/CLI/serverless | pay-per-use | `runpod.io/pricing` (200) |
| **Vast.ai** | kein Free-Tier („Start with $5") | Consumer/Datacenter | **ja** — REST/CLI | Mindestguthaben $5 | HTTP **unbelegt** (`--verdict` Timeout) |
| **Lambda** | kein Free-Tier | V100 $0.79/h … H100 $3.99/h | **ja** — API/CLI | pay-per-use | `lambda.ai/pricing` (200) |
| **SageMaker Studio Lab** | kostenlos (E-Mail, Genehmigung) | G4dn (T4) lt. Drittquelle | **nein** — JupyterLab/VSCode, kein API | Warteliste; GPU teils „nicht verfügbar" | `aws.amazon.com/sagemaker/ai/pricing` (200) |
| **GitHub Codespaces** | 120 Core-h/Monat | **keine GPU** (kein Machine-Type) | ja für CPU | GPU nur via larger runner (Team, bezahlt) | `docs.github.com/en/codespaces/overview` (206) |
| **Binder / Deepnote** | kostenlos | **keine GPU** / GPU unbelegt | nein / teilweise | ephemer / Free-GPU unbelegt | `mybinder.readthedocs.io` (Wayback 200); `deepnote.com` (206) |

**Fazit:** Der einzige **echte Gratis-Batch-Pfad** ist **Kaggle** (30 h/Woche P100, offizielle
API). Günstig headless sind **Modal** ($30/Monat gratis) und **Lightning** (~80 GPU-h, verfallen).
Rein interaktiv und für Automatisierung untauglich: Colab, HF ZeroGPU, SageMaker Studio Lab,
Codespaces (keine GPU), Binder/Deepnote. Paid-und-headless: RunPod, Vast.ai, Lambda.

## C. Der tatsächliche GPU-Bedarf von omegaflow (am Baum gemessen)

- **Korrektheits-Bedarf an Hardware-GPU: null.** Jede GPU-Anforderung läuft **ohne Surface**
  (`compatible_surface: None`) und **skippt** ohne Adapter — kein Hard-Fail:
  `src/mathematikerin/omega.rs:1256-1270` (Feldevaluation/Membrane), `src/mathematikerin/scalar_te_gpu.rs:22-26`
  (KDE-TE), Parity-Tests `src/mathematikerin/tests.rs:106,1006,2031` und `src/mathematikerin/te.rs:7469`.
  Die CI fährt die GPU-Pfade auf dem **Software-Vulkan-Adapter lavapipe**:
  `.github/workflows/ci-check.yml:42-43` (`mesa-vulkan-drivers`), `.github/workflows/matrix-rotor.yml:49-50`.
  **Kein Workflow nutzt GPU-Hardware oder self-hosted Runner.**
- **Der reale Bedarf ist Performance/Real-Time, lokal.** `docs/concepts/archivar-mathematikerin.md:40`:
  die GPU ist die **Membran**, nicht das Rückgrat („the compute strategy never rests on a free
  cloud GPU"). Gepackt vom lokalen XPS 13 (2C/4T, Intel HD 520, iGPU,
  `state/future/bewerbungen-vs-zai-export.md:217`). Eine dedizierte GPU war nie im Einsatz; die
  GTX 970 des Operators ist im Export nur als Besitz/Frage belegt (`:213`).
- **Was heißt das:** Für die **Richtigkeit** der TE-/Feld-Codes braucht die CI keine GPU — lavapipe
  genügt. GPU-Automation zahlt sich nur für **schnelle Messläufe** aus, die sonst lokal laufen.

## D. CI-Integrationswege für GPU (gemessen)

| Weg | wie | Kosten | Eligibility | Quelle (HTTP, 2026-10-03) |
|---|---|---|---|---|
| **Self-hosted Runner (GTX 970)** | Runner-App auf eigener Maschine; `runs-on: [self-hosted]`; wgpu nutzt Vulkan (kein CUDA nötig) | **$0** GitHub-Gebühr; Strom/Betrieb trägt man | jedes Konto — **harte Kante:** GitHub rät bei **öffentlichen** Repos ausdrücklich ab (Fork-PRs können die Umgebung kompromittieren) | `docs.github.com/…/self-hosted-runners` (206); `…/security/secure-use` (206) |
| **GitHub larger runner (T4)** | Org-Settings → Runner-Group → `runs-on` | Linux **$0.052/min**, Windows **$0.102/min** (immer) | **nur Team/GHEC** | `…/actions-runner-pricing` (206) |
| **Cirun** | managed self-hosted platform, Runner in **eigener** Cloud/on-prem (AWS/GCP/Hetzner/Vast/on-prem) | **„Free for Open Source $0"**; man zahlt die eigene Cloud | jedes Konto | `docs.cirun.io` (206) |
| **Depot** | GitHub-Runner; GPUs via **eigenes AWS-Konto** (Managed) | eigener AWS-GPU-Preis + Depot-Gebühr (Tarif **unbelegt**) | eigenes AWS | `depot.dev/docs/github-actions/runner-types` (200) |
| **RunPod / Modal** | Compute-Offload aus einem CI-Step (kein Runner-Ersatz) | RunPod A100 SXM $1.59/h, RTX 4090 $0.74/h; Modal T4 ~$0.59/h | Account + API-Key | `docs.runpod.io/serverless/workers/github-integration` (200); `modal.com/docs/guide/gpu` (206) |
| **BuildJet** | (historisch) managed Runner | — | — | **eingestellt 2026-03-31** → tot. `buildjet.com/…/we-are-shutting-down` (200) |
| **Namespace** | GitHub-Runner-Ersatz | — | **kein GPU-Angebot belegt** | `namespace.so/github-actions` (206) |

## E. Risse und Unbelegtes (nicht geglättet)

- **Feste GPU-Labels** `gpu_1x_a100`/`gpu_1x_h100`: in der aktuellen GitHub-Doku nicht belegt → `unbelegt`.
- **GTX 970 + wgpu:** die konkrete Lauffähigkeit dieser Maxwell-Karte unter wgpu/Vulkan ist **am Baum ungemessen** → `unbelegt`. (Nvidia stellte den Game-Ready-Treiber-Support für Maxwell/Pascal/Volta im Dez. 2025 ein; Security-Updates laufen weiter.)
- **Vast.ai** HTTP-Code unbelegt (Timeout); **Depot-GPU-Preis** unbelegt; **Namespace-GPU** unbelegt.
- **Colab** veröffentlicht seine Free-Limits nicht — jede Zahl dazu ist Drittquelle.

## F. Empfehlung (mit Begründung)

1. **Nichts upgraden für Richtigkeit.** Die CI braucht keine GPU; lavapipe trägt die Tests. Also **keine** zusätzlichen Runner-Minuten.
2. **Wenn automatisierte GPU-Messläufe gewünscht sind:** zuerst **Kaggle** (einziger echter Gratis-Batch, 30 h/Woche), dann **Modal** ($30/Monat gratis, headless) — beide ohne GitHub-Planwechsel.
3. **GitHub-GPU nur**, wenn Team/GHEC ohnehin gewollt ist: dann T4-larger-runner zu $0.052/min.
4. **Self-hosted GTX 970** ist $0, aber die public-Repo-Sicherheitskante und die ungemessene wgpu-Lauffähigkeit stehen dagegen; über **Cirun** (free for OSS) ließe sich die Karte kontrollierter poolen.

## G. Stimmen-Lauf (freie LLM-Flotte, 2026-10-03)

Aufruf: `opencode run --agent voice` (public-only, `archive_search_public`), **6 Modelle**, eine
**4-teilige** Frage; Rohströme in `state/stimmen/2026-10-03_gpu-compute-*.md`.

- **Ergebnis (gemessen am Lauf-Log):** nur `moonshotai/kimi-k3` endete `rc=0` — mit **leerem**
  40-Byte-Body; die fünf übrigen (nemotron-ultra/super, gemini-3.7-flash, qwen3.8-27b,
  nemotron-super openrouter) liefen in die 300-s-Kappe (`rc=124`) mit **rohen Suchströmen, ohne
  fertige Antwort**.
- **Kein neuer Primärquellen-Fakt.** Ihr Material **bestätigt** den Befund (GitHub-Pricing und
  `…/security/secure-use`; Colab-FAQ „dynamic usage limits … vary over time") und liefert nur
  Drittquellen: `gmicloud.ai/blog/…gpu-cloud-trials-2026` (Aggregator, HTTP 200), `reddit.com/r/kaggle/…`,
  `davesnider.com/gputests` („a GPU runner … doesn't automatically give you GPU acceleration — the
  hosted image comes with mesa and llvmpipe"), `dvc.org/blog/cml-self-hosted-runners-on-demand-with-gpus`.
- **Lehre:** eine **vierteilige** Frage überlastet die Stimmen — der tragfähige Modus ist **eine
  Frage je Lauf** (`docs/concepts/free-voices.md:43`). Die Stimme bleibt Zeuge (Claim), nie die
  Messung; kein Drittquellen-Fakt ging ungeprüft ins Blatt.

## H. Der lokale GPU-Weg: nur das XPS 13 — kein dedizierter GPU

**Korrektur (Operator-Wort 2026-10-03 „der rechner ist nicht verfügbar, mein rechner ist der
xps13"):** Der Rechner mit der **GTX 970 ist nicht verfügbar**; die Maschine ist das **XPS 13**
(Intel HD 520, iGPU). Der in `docs/concepts/github-pipeline.md:38` und in ~10 archivierten
Übergaben getragene „Desktop-Fork (GTX 970)" ist damit **descoped** — **kein** Self-hosted-/
Rechner-GPU-Worker.

Gemessen zum verbleibenden lokalen Weg:

- Die GPU-Anforderungen laufen `compatible_surface: None` (kein Fenster) und **skippten** ohne
  Adapter; die CI nutzt nur **lavapipe** (`ci-check.yml:42-43`, `matrix-rotor.yml:49-50`).
- Der reale GPU-Bedarf ist Performance/Real-Time und läuft auf der **XPS-iGPU** (Intel HD 520) —
  begrenzt: der Linux-GPU-Watchdog tötet lange Shader nach 15 s
  (`docs/specs/gpu-watchdog-device-loss.md:39-42`).
- `docs/concepts/archivar-mathematikerin.md:40`: die Compute-Strategie ruht nie auf einer fremden
  Cloud-GPU.

**Konsequenz:** Will man mehr GPU-Leistung als die XPS-iGPU, geht es **nur remote** — und der freie
**Kaggle-Weg trägt unsere GPU nicht** (CUDA-only, kein Vulkan — §I); es bleibt ein
**Vulkan-fähiger Container-Provider** (RunPod/Vast, bezahlt, `NVIDIA_DRIVER_CAPABILITIES=graphics`)
oder die lokale iGPU. Die „GTX 970"-Hoffnung ist aus dem Weg; der seit 2026-09-09 getragene Worker
wird **nicht** gebaut.

## I. Kaggle-Praxisprobe (2026-10-03): der freie Weg trägt unsere GPU nicht

Gebaut: `tools/utils/src/bin/kaggle_kernel.rs` (Rust std + curl, Legacy-REST `push`/`status`/`output`)
+ R-Probe-Kernel (`state/future/kaggle-gpu-probe/`, **kein Python**). Konto registriert, Legacy-API-Key
in `~/.kaggle/kaggle.json`, API-Antwort **200**.

Gemessen im Lauf (Kernel `johannestyroller/omegaflow-gpu-probe`, laut Kaggle-Seite auf **„GPU T4 x2"**):

- Ubuntu 22.04.4; `KAGGLE_DOCKER_IMAGE=gcr.io/kaggle-gpu-images/rstats` (R-Image).
- `/proc/driver/nvidia/` **vorhanden** (GPU ist angehängt), **aber**: `nvidia-smi` fehlt, keine
  `libnvidia*` im Userspace, keine Vulkan-ICD (`/usr/share/vulkan/icd.d/` fehlt).
- `NVIDIA_DRIVER_CAPABILITIES=compute,utility` — **kein `graphics`**; der Vulkan-/GL-Userspace wird
  vom NVIDIA-Container-Runtime also **nicht injiziert**.
- Nach `apt install libvulkan1 mesa-vulkan-drivers vulkan-tools`: Vulkan-Loader 1.3.204 da, Gerät ist
  nur **llvmpipe** (CPU, `DEVICE_TYPE_CPU`); `vulkaninfo` mit `nvidia_icd.json` scheitert
  (`ERROR_INCOMPATIBLE_DRIVER`).

**Verdikt:** Kaggles freie GPU ist **CUDA-compute-only** und stellt **kein Vulkan**. wgpu hat keinen
CUDA-Backend → unser WGSL-Pfad (`te_compute`, Membran) läuft dort **nicht**. Der freie GPU-Weg ist für
die GPU-Arbeit **geschlossen**; es bleibt llvmpipe (CPU, wie die CI — kein Gewinn) oder ein
**Container-Provider mit Vulkan** (`NVIDIA_DRIVER_CAPABILITIES=graphics` + NVIDIA-ICD, RunPod/Vast;
bezahlt, §D).

## I2. Colab-Praxisprobe (2026-10-04): dieselbe Klasse — kein Vulkan

Freie **Tesla T4** mit vollem Treiber (**580.82.07, CUDA 13**), `nvidia-smi` läuft. Aber Vulkan:

- eine selbst geschriebene `nvidia_icd.json` auf `libGLX_nvidia.so.0` → der Loader lädt die Lib,
  bekommt aber **kein `vkCreateInstance` via `vk_icdGetInstanceProcAddr`** — die GL-/Vulkan-Komponente
  ist ein **Stub** (compute-only-Injektion, wie bei Kaggle).
- `vulkaninfo` endet `ERROR_INCOMPATIBLE_DRIVER`, „Found no drivers".

**Verdikt:** **Colab trägt ebenfalls kein Vulkan.** Die FAQ bestätigt den Charakter der Ressource:
GPU-Typen „vary over time", Ressourcen „not guaranteed and not unlimited", Limits unveröffentlicht
(`research.google.com/colaboratory/faq.html#gpu-availability`, HTTP 206, 2026-10-04).

**Gesamtbefund der freien Wege (gemessen):** Freie Cloud-GPUs sind **compute-only** (CUDA/OpenCL).
**Kein freier Anbieter liefert Vulkan** — weder Kaggle noch Colab. wgpu hat keinen CUDA-Backend;
die graphics-Capability gibt es nur dort, wo der Anbieter sie injiziert (bezahlter Container mit
`NVIDIA_DRIVER_CAPABILITIES=graphics`) oder lokal (XPS-iGPU, Intel/Mesa). Für unseren WGSL-Pfad ist
der freie Weg damit **erschöpft**.

## J. Vulkan-Karte (2026-10-04) — wo unser WGSL läuft

Entscheidend ist **nicht** „GPU", sondern **Vulkan** (Treiber-Userspace mit `graphics`-Capability).
Gemessen über die Flotte (`archive_search`) + lokal (`vulkaninfo`):

| Weg | Typ | Vulkan | Beleg |
|---|---|---|---|
| **XPS 13 (Intel HD 520, SKL GT2)** | iGPU, Mesa ANV 25.2.8 | **ja** (Vulkan 1.4.318) | lokal `vulkaninfo --summary` 2026-10-04 |
| **Vast.ai** | Container **oder Linux-VM**, eigenes Image | **ja, verifiziert** | `nvidia/cudagl` + GTX 1070 Ti, `vulkaninfo` listet Gerät — `github.com/brianonbased-dev/HoloScript` (206), 2026-05-29 |
| **RunPod** | Container, eigenes Image | möglich (`NVIDIA_DRIVER_CAPABILITIES=all`), per-Host | `docs.runpod.io/pods/templates/environment-variables` (200) |
| **Google Cloud (Trial $300/90 T)** | volle VM, eigener Treiber | ja (installierbar) | `download.nvidia.com/.../installedcomponents.html` (200) |
| **Lambda (V100 $0.79/h)** | volle Instanz | ja (eigener Treiber) | `lambda.ai/pricing` (200) |
| **Oracle OCI (A10 $2/h, Trial $300/30 T)** | volle VM | ja (eigener Treiber) | `docs.oracle.com/.../computeshapes.htm` (200) |
| Kaggle | Container `compute,utility` | **nein** | §I |
| Colab | Container, Stub | **nein** | §I2 |
| Modal | gVisor/`nvproxy`, compute-only | **nein** | `gvisor.dev/docs/user_guide/gpu` (206) |
| SageMaker Studio Lab | Container, **kein sudo**, geschlossen | nein | `github.com/aws/studio-lab-examples/issues/118` (200) |
| Lightning AI (~80 GPU-h) | Container, fester Treiber | unbelegt | `lightning.ai/pricing` (200) |
| Paperspace Gradient | Container (root) | unbelegt | `docs.digitalocean.com/.../use-terminal/` (200) |

**Regel aus der Messung:** Vulkan gibt es dort, wo wir **Treiber/Capability kontrollieren** — oder
wo der Anbieter `graphics` injiziert. Freie Notebook-Container (Kaggle, Colab, Modal, SageMaker)
sind **compute-only** und fallen aus. Wirklich frei mit Vulkan ist nur die **lokale XPS-iGPU**;
sonst **volle VM** (GCP-Trial, Lambda, Oracle) oder **Vast.ai/RunPod** (bezahlt, Container mit
`graphics`).
