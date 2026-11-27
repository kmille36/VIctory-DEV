# VIctory on an RX 7900 XTX

## Branch 1660s: GTX 1660 6 GB

This branch is `main` (the codec_d Windows build below) plus every patch
from [Impre-visible/victory-gtx1660](https://github.com/Impre-visible/victory-gtx1660)
— 0001, 0002 and the optional 0003, 0004, 0006, 0007 (there is no 0005) —
which took the title from about 360–410 ms a frame to about 100 ms a frame
(about 10 fps) on a GeForce GTX 1660 6 GB under Linux. No game files.

| Patch | What it does | On by default |
|---|---|---|
| 0001 | Shader translation: every loop has one branch back, so vkd3d-proton's DXIL-to-SPIR-V converter stops recursing to a stack overflow on multi-latch loops | yes (always) |
| 0002 | `VICTORY_TEXTURE_MIP_DROP=0..3`: host copies of large placed textures start that many mip levels down, so the title's textures fit in 6 GB | no (`0`) |
| 0003 | Diagnostics: `VICTORY_CARD_SPANS=1` adds a census of the viewports the title sets per frame to the frame-rate line | no |
| 0004 | `VICTORY_CARD_AWAKE=idle`: keep the card awake only while the drawing queue has nothing to run | no |
| 0006 | Diagnostics: a pipeline left out on purpose is reported with its reason instead of "being made" | yes (diagnostics builds) |
| 0007 | `VICTORY_VIEW_INTEGER=0`: leave typed buffer views of integer formats unmarked, to test whether missing heads come from that path | yes (the switch turns it off) |

The patches alone do not give the ~100 ms figure. Most of the gain comes
from three settings (measured on the patch author's machine: GTX 1660 6 GB,
Core i7-13700KF, 62 GB RAM, Arch Linux, NVIDIA driver 620.73.04, GE-Proton11-6
with vkd3d-proton, first playable scene, Series S profile):

```powershell
$env:VICTORY_CARD_BUFFERS_MB = '1024'
$env:VICTORY_CARD_WINDOWS_MB = '256'
$env:VICTORY_TEXTURE_MIP_DROP = '1'
$env:VICTORY_LEVELS_ON_DEMAND = '0'
```

`CARD_BUFFERS_MB=1024` carries the gain: below it the frame time returns to
about 275 ms. The copies go to the card only while a fifth of the card's
memory budget is free; on Windows that budget is what the system grants this
process, so close other programs that use the GPU. If video memory runs
short, raise the mip drop to 2 or 3 (softer textures). Do not lower
`CARD_BUFFERS_MB` below 1024.

Measured on that machine: cool_fox unmodified 360–410 ms (2.4–2.8 fps);
patches + `LEVELS_ON_DEMAND=0` + mip drop 2: 271–291 ms (3.4–3.7 fps);
the settings above: 98–112 ms (9–10 fps, five clean runs). Longer runs at
768x432 internal render, mip drop 3, `CARD_BUFFERS_MB=640`: 56–57 ms
(about 17–18 fps) standing still, 50–60 ms walking through the opening,
67–78 ms on the boat outdoors, 93–120 ms near a wooded shore — but those
longer runs used additional work not in these patches (renderer port 1,
a render-scale switch, skipping ray-tracing-only passes, the wineserver
patch). True 720p at 20 fps is out of reach of this card by these numbers.

Honest limits, from the patch author's own notes:

- All figures are Linux figures (Proton + vkd3d-proton). Windows was not
  tested by the patch author; the texture and memory settings matter most
  on cards with 6–8 GB.
- A tester with a GTX 1660 SUPER on Windows 11 reports the cool_fox fork
  does not get past the start screens with NVIDIA's driver, while it does
  with the software adapter and under Linux (Marvisak/VIctory#2835).
- Only the first scene was measured for the 10 fps figure; later scenes
  are slower.
- Textures start one mip level down (softer); ray-traced lighting is not
  computed (the fork's ambient-light stand-in replaces it).
- Characters' heads were missing in most runs; rebuilding the cache
  folder's pipeline lists from scratch fixed it (move `pipelines` aside
  once and let a run rebuild it).
- `cool_fox` itself no longer answers on GitHub (404 since 2026-11-26);
  this branch keeps its code as it was at `main`.

Build with the diagnostics feature to get the frame statistics line the
figures above come from:

```powershell
cargo build --release --locked -p victory --features diagnostics
```

## Run the codec_d Windows build

Download and extract the Windows x64 ZIP from the [private releases](https://github.com/kmille36/VIctory-DEV/releases/download/pareto-residency-37a410b1/cool_fox-pareto-residency-windows-x64.zip). Open PowerShell in the extracted folder and point the launcher at your own complete Xbox dump.

```powershell
$env:VICTORY_GAME_DIR = 'D:\Games\MyDump'
.\Play-VIctory.cmd pareto-residency
```

The launcher selects Series S and the RTX-tested residency settings. It builds its shader cache locally under `%LOCALAPPDATA%\VIctory\cache-pareto`. Set `VICTORY_CACHE_DIR` to reuse an existing cache outside the checkout. Windows x64, an AVX2 CPU, and a Direct3D 12 GPU with shader model 6.6 and mesh shader support are required.

To build from this checkout, install the pinned Rust toolchain and the MSVC C++ build tools with a Windows SDK, then run `cargo build --release --locked -p victory`. The same launcher finds `target\release\victory.exe`. The sections below describe the upstream AMD fork and its separate launcher.

A fork of [VIctory](https://github.com/Marvisak/VIctory), a compatibility layer
that runs an Xbox Series title natively on Windows from files the user supplies.
This one is about **speed on AMD RDNA 3**: the world at 30 frames a second on
one PC, without giving up the picture. It is not there yet.

**This repository contains no game files, code or assets.** You need your own
copy of the game. It is not affiliated with or endorsed by the game's developer
or publisher.

## Where it stands (2026-11-25)

Tested on one machine only: Windows 11, Intel i9-12900K, 64 GB RAM, Radeon
RX 7900 XTX, 1440p, the Series X profile.

| | |
|---|---|
| Menus, videos, loading screens | 30 fps |
| Prologue, indoors | 50-75 ms a frame (13-20 fps) |
| First mission with the boat | 46-53 ms (19-22 fps) |
| Street chase with many cars and people | 100-165 ms (6-10 fps), single frames far longer |
| Outdoors on foot, the camp | 60-90 ms |
| Flying | 42-57 ms |
| Longest session so far | 41 minutes, no crash |
| Memory | 40-46 GB of private memory: **64 GB of RAM recommended** |

It started at 280 ms a frame in the same scenes two days earlier.

What is known to be wrong:

- Trees, plants and far buildings flicker outdoors in some places.
- Rooms with little direct light are too dark without `-RayTracing`.
- Textures of what just came into view appear late while the title streams.
- The first run with a new driver or a new build stutters while the driver makes
  pipelines (`victory warm` makes them ahead).
- Recording a frame in the world makes the title's own watchdog end it.

## How to run it

You need Windows 10/11 x64, a CPU with AVX2, a Direct3D 12 GPU with shader
model 6.6 and mesh shaders, Rust (the toolchain is pinned), the MSVC build
tools, and your own dump of the title.

```powershell
git clone https://github.com/avatarmanz/VIctory-7900XTX
cd VIctory-7900XTX
copy victory.example.toml victory.toml      # then set your paths, outside this folder
cargo build --locked --profile diag -p victory --features diagnostics
scripts\play.ps1 -Exe <path of your copy's executable>
```

`scripts\play.ps1 -Safe` leaves out the newest, faster ways; try it first when
something is wrong. `-RayTracing` turns the title's ray-traced indirect light on.

Before the first run, ahead of time (both can be stopped and go on later):

```powershell
target\diag\victory.exe shaders <the shader archives>   # every shader translated, minutes
target\diag\victory.exe warm --used                     # pipelines earlier runs drew with, for a new driver or build
```

Commands, debugging switches and upstream's own description:
[docs/README-fxceless.md](docs/README-fxceless.md). Every `VICTORY_...` switch is
described in a comment where it is read.

## What is different here

In short; the reasons and the measurements are in [docs/findings.md](docs/findings.md).

- **List calls are written down, not carried out, on the title's threads**
  (`VICTORY_THIN`): eight threads of ours translate them.
- **Submissions are prepared by a thread of ours and joined**
  (`VICTORY_ASYNC_SUBMIT`, `VICTORY_JOIN_PARTS`), with what can be spread over
  threads spread: refreshes, compares (AVX2), descriptor records, landings.
- **What shaders wrote comes back beside the write guards**, by the dwords that
  changed, on a thread of its own (`VICTORY_RELAY`, `VICTORY_LAND`).
- **Buffer fills that need bytes still on the card are done on the card**
  (`VICTORY_CARD_STEPS`) instead of waiting for it.
- **The title's writes are caught more cheaply**: one view opened at a fault,
  blocks that rest, places in the title's code that look before they write.
- **Textures go up on a copy queue of their own** and a slot shows one only after
  its upload ran, which also ended a device loss that came every 15 minutes or so.
- **Boats float**: textures that shaders write and the title's CPU reads go back
  to title memory (from fxceless and Geckomacabre, on by default here).
- **Frame record and replay** with per-pass times and shader variants compared
  turn by turn ([docs/replay.md](docs/replay.md)).

## Credits

This is a tree of other people's work with more on top. The history is kept, and
commits taken from elsewhere name the commit and its author.

- **[VIctory](https://github.com/Marvisak/VIctory)** by Marvisak and its
  contributors: the loader, the runtime services, the Direct3D 12 back end and
  the shader recompiler.
- **[fxceless/victory-fork](https://github.com/fxceless/victory-fork)**: the fork
  this one started from, and the people whose pull requests it carries.
- **[DexNom/VIctory](https://github.com/DexNom/VIctory)**: ray tracing fixes,
  cube gradients, sparse textures.
- **[Belobug/VIctory-Belobug-AMD](https://github.com/Belobug/VIctory-Belobug-AMD)**:
  the ambient light idea, pipeline workers, and work by romainhedouin.
- **[FQY55kai/ZJOU-VICTORY](https://github.com/FQY55kai/ZJOU-VICTORY)**.
- **[myokex/victory-amd](https://github.com/myokex/victory-amd)**: the guard over
  every view of a page (with Spexivus, joroyt and tightfall), and Joshua Odom's
  reads into title memory.
- **Geckomacabre** and **cxy0072019-sketch**: the texture read-back.
- **[XCL](https://github.com/ivall/xcl)** (GPL-2.0-only) was read for how the same
  title behaves under another layer; no code of it is here.
- This fork's changes were made with [Claude Code](https://claude.com/claude-code).

## License

GPL-3.0-or-later, as VIctory. See [LICENSE](LICENSE); the one file that is not
covered is named in [docs/asset-policy.md](docs/asset-policy.md).
