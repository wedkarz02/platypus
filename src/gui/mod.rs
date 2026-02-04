// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Paweł Rybak

pub struct App {
    message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            message: String::from("Platypus"),
        }
    }
}

impl App {
    // Called once before the first frame
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(&self.message))
        });
    }
}
