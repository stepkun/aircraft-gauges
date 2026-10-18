// Copyright © 2026 Stephan Kunz
// SPDX-License-Identifier: GPL-3.0-only

//! Aircraft instruments as a Slint library: `Attitude`, `Altimeter`, `Airspeed`, `Heading`,
//! `VerticalSpeed`, `TurnCoordinator` and the global `Theme`, with the embedded font they are
//! designed for.
//!
//! Add the crate as a build-dependency and give its library path to the Slint compiler in
//! `build.rs`:
//!
//! ```ignore
//! let config = slint_build::CompilerConfiguration::new()
//!     .with_library_paths(aircraft_gauges::library_paths());
//! slint_build::compile_with_config("ui/app.slint", config).unwrap();
//! ```
//!
//! and import from the library in the `.slint` files:
//!
//! ```slint,ignore
//! import { Attitude, Theme } from "@aircraft-gauges/lib.slint";
//! import "@aircraft-gauges/fonts/NotoSans-Regular.ttf";
//! ```

use std::collections::HashMap;
use std::path::PathBuf;

/// Name of the library in the imports, `@aircraft-gauges/...`
pub const LIBRARY_NAME: &str = "aircraft-gauges";

/// The font sizes of `Theme` (`menu_font_size`, `font_size`, `large_font_size`) for
/// `SLINT_FONT_SIZES`: the instruments scale their font sizes with their size, so the compiler
/// can't collect them, and only these glyphs are embedded for the software renderer.
pub const FONT_SIZES: &str = "12,14,20";

/// The directory with the `.slint` files and the font
pub fn library_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui")
}

/// The library paths for `slint_build::CompilerConfiguration::with_library_paths`
pub fn library_paths() -> HashMap<String, PathBuf> {
    HashMap::from([(LIBRARY_NAME.to_string(), library_path())])
}
