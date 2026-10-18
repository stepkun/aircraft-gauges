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
| `ui/fonts/` | Noto Sans, imported as `@aircraft-gauges/fonts/NotoSans-Regular.ttf` (no system fonts on microcontrollers) |
| `examples/gallery.rs`, `examples/gallery.slint` | Test program: every instrument, a slider for each value and for the size, a demo movement; compiled at start by the Slint interpreter (dev-dependency), so the library needs no build script |
| `tools/dial-paths.py` | Generates the static tick and arc paths (between the `BEGIN`/`END generated` markers) and checks they never overlap a hand box; `--check` fails if a file is not up to date |

## Users

Both check out this repository next to themselves and use it as a `path` build-dependency:

- `../slint-demos`: desktop app, web demo, `tools/host-check` and the firmwares (RP2040,
  RP2350, ESP32-S3), with the patched Slint fork (master, 1.19.0-dev).
- `../x-plane`: `basic-instrument-panel`, with published Slint from crates.io (1.18.1).

The `.slint` files must therefore compile with both Slint versions: build both after a change.

## Constraints

The RP2040 of `slint-demos` is the target that matters (92 KB heap, no FPU, software renderer
without transforms). **Read "RP2040 constraints for `.slint` code" in `../slint-demos/CLAUDE.md`
before changing a `.slint` file here**, and check every change with its host check:

```sh
cargo run --example gallery          # look at every instrument, with any value and size
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
- Commits are made by the maintainer.
