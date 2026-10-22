// Copyright © 2026 Stephan Kunz
// SPDX-License-Identifier: GPL-3.0-only

//! Test program: all instruments of the library with a slider for every value, a slider for
//! their size and a switch for a demo movement.
//!
//! `cargo run --example basic-panel`
//!
//! The `.slint` files are compiled at start by the Slint interpreter, so changes to them only
//! need a restart, and the library needs no build script (which its users would build too).

use std::error::Error;
use std::path::Path;

use slint_interpreter::{Compiler, ComponentHandle};

fn main() -> Result<(), Box<dyn Error>> {
    let mut compiler = Compiler::default();
    compiler.set_library_paths(aircraft_gauges::library_paths());
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/basic-panel.slint");
    let result = spin_on::spin_on(compiler.build_from_path(&path));
    for diagnostic in result.diagnostics() {
        eprintln!("{diagnostic}");
    }
    if result.has_errors() {
        return Err(format!("{} has errors", path.display()).into());
    }
    let panel = result
        .component("BasicPanel")
        .ok_or("no component BasicPanel")?
        .create()?;
    panel.run()?;
    Ok(())
}
