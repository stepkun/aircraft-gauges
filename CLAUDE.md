# CLAUDE.md

Aircraft instruments as a Slint library crate. The crate has no dependencies: the library is
the `ui/` directory, the Rust code only tells the Slint compiler where it is.

## Layout

| Path | What |
|---|---|
| `src/lib.rs` | `library_paths()` for `slint_build::CompilerConfiguration::with_library_paths`, `FONT_SIZES` for `SLINT_FONT_SIZES` |
| `ui/lib.slint` | Exports everything, imported as `@aircraft-gauges/lib.slint` |
| `ui/theme.slint` | Global `Theme`: colours, widths, font sizes |
| `ui/attitude.slint`, `altimeter.slint`, `airspeed.slint`, `heading.slint`, `vertical-speed.slint`, `turn-coordinator.slint` | The instruments |
| `ui/combined.slint` | `Combined`: attitude, vertical speed and altitude tapes, heading tape at the top, airspeed and heading as numbers, 320x240 (4:3, the ResTouch display in landscape); the tick and number slots of the tapes are written out (no `for`). Only for rectangular 4:3 displays (not the round ones) and not for the RP2040, see Constraints |
| `ui/fonts/` | Noto Sans, imported as `@aircraft-gauges/fonts/NotoSans-Regular.ttf` (no system fonts on microcontrollers) |
| `examples/basic-panel.rs`, `examples/basic-panel.slint` | Test program: every instrument, a slider for each value and for the size, a demo movement; compiled at start by the Slint interpreter (dev-dependency), so the library needs no build script |
| `examples/combined.rs`, `examples/combined.slint` | The same for `Combined` |
| `examples/controls.slint` | `Control` (name, slider, value) of both test programs |
| `tools/dial-paths.py` | Generates the static tick and arc paths (between the `BEGIN`/`END generated` markers) and checks how much of their boxes a hand box covers at any angle: nothing, or at most `max_overlap` px for the ticks in quarters of the altimeter and the vertical speed indicator (fewer paths, measured faster on the RP2040), and that every filled outline runs counterclockwise (see Conventions); `--check` fails if a file is not up to date |

## Users

Both check out this repository next to themselves and use it as a `path` build-dependency:

- `../slint-demos`: desktop app, web demo, `tools/host-check` and the firmwares (RP2040,
  RP2350, ESP32-S3), with the patched Slint fork (master, 1.19.0-dev).
- `../x-plane`: `basic-panel` and `combined`, with published Slint from crates.io (1.18.1).

The `.slint` files must therefore compile with both Slint versions: build both after a change.

## Constraints

The RP2040 of `slint-demos` is the target that matters (92 KB heap, no FPU, software renderer
without transforms), except for `Combined`: it needs about 85 KB of heap (the other instruments
up to 58 KB), with the TLSF allocator of the firmwares at least 128 KB (RP2350: 256 KB), and a
rectangular 4:3 display, so of the firmwares only the RP2350 ResTouch-LCD-2.8 (in landscape)
fits; not the round displays. The rules for the RP2040 apply to it all the same, they keep it
fast on the other microcontrollers. **Read "RP2040 constraints for `.slint` code" in `../slint-demos/CLAUDE.md`
before changing a `.slint` file here**, and check every change with its host check:

```sh
cargo run --example basic-panel      # look at every instrument, with any value and size
python3 tools/dial-paths.py          # after changing generated dial geometry or Theme widths
cd ../slint-demos/tools/host-check
cargo run --release                  # heap and partial redraws of every instrument
cargo run --release -- --display 240x240
```

`FONT_SIZES` in `src/lib.rs` and the font sizes of `Theme` must be the same.

## Conventions

- License GPL-3.0-only. New source files start with:
  `// Copyright © 2026 Stephan Kunz` and `// SPDX-License-Identifier: GPL-3.0-only`
- `snake_case` property names, comments explain why, not what.
- Filled `Path` outlines run counterclockwise on the screen (y down), and an outline that moves
  ends with its first point again (`LineTo` before `Close`). Slint tells femtovg (the desktop
  renderer: the examples, `x-plane`, Raspberry Pi) that a clockwise outline is a hole, and femtovg then
  draws it 1 px fatter on each side; Slint takes the direction without the closing edge, which
  can turn it around. Static ticks start at a corner where that doesn't matter instead (a point
  costs about 90 bytes of heap on the RP2040). See `counterclockwise()` in `tools/dial-paths.py`.
- Commits are made by the maintainer.
