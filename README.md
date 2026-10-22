# aircraft-gauges

Aircraft instruments as a [Slint](https://slint.dev) library: attitude indicator, altimeter,
airspeed indicator, heading indicator, vertical speed indicator and turn coordinator, with a
shared `Theme` and the embedded Noto Sans font. They are drawn without transforms, so they
also run with the software renderer on microcontrollers (a Raspberry Pi Pico with 264 KB RAM).

`Combined` shows attitude, airspeed, altitude, vertical speed and heading in one 320x240
display (4:3), with tapes for vertical speed, altitude and heading. It needs a rectangular
4:3 display, not a round one, and about 85 KB of heap, too much for the Pico (RP2040): it runs
on a Pico 2 (RP2350) with a 320x240 display and on the desktop.

Used by [slint-demos](https://github.com/stepkun/slint-demos) (desktop, web and firmwares)
and [x-plane](https://github.com/stepkun/x-plane) (instrument panel for X-Plane).

## Use

`Cargo.toml`:

```toml
[build-dependencies]
aircraft-gauges = { path = "../aircraft-gauges" }
slint-build = "1.18"
```

`build.rs`:

```rust
fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(aircraft_gauges::library_paths());
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
}
```

`ui/app.slint`:

```slint
import { Attitude, Theme } from "@aircraft-gauges/lib.slint";
import "@aircraft-gauges/fonts/NotoSans-Regular.ttf";

export component App inherits Window {
    default-font-family: "Noto Sans";
    background: Theme.background;
    Attitude { roll: 10deg; pitch: 5deg; }
}
```

With the software renderer and embedded resources, also set
`SLINT_FONT_SIZES` to `aircraft_gauges::FONT_SIZES` in `build.rs` before compiling.

## Test program

```sh
cargo run --example basic-panel
```

shows every instrument with a slider for each value and for the size, and a demo movement,
and

```sh
cargo run --example combined
```

the same for the combined display.
The `.slint` files are loaded at start by the Slint interpreter, so a change only needs a
restart. `SLINT_BACKEND=winit-software` uses the software renderer of the microcontrollers
(the published one, without the partial redraw fix of the Slint fork of slint-demos, so moving
paths can leave traces).

## License

GPL-3.0-only, the font is under the SIL Open Font License (`ui/fonts/NotoSans-Regular.ttf.license`).
