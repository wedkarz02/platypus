// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Paweł Rybak

use platypus::gui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (run with RUST_LOG=debug)

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 640.0]),
        // .with_min_inner_size([480.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Platypus",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::app::App::new(cc)))),
    )
}
