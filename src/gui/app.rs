// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Paweł Rybak

pub struct App {
    window: Window,
    db_config: DbConfig,
}

enum Window {
    Welcome,
    Main,
}

#[derive(Debug)]
pub struct DbConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: Window::Welcome,
            db_config: DbConfig {
                host: "localhost".into(),
                port: "5432".into(),
                user: "".into(),
                password: "".into(),
                database: "".into(),
            },
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

    fn welcome_view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("Hello");
                ui.add_space(20.0);

                ui.label("Host");
                ui.text_edit_singleline(&mut self.db_config.host);

                ui.label("Port");
                ui.text_edit_singleline(&mut self.db_config.port);

                ui.label("User");
                ui.text_edit_singleline(&mut self.db_config.user);

                ui.label("Password");
                ui.add(egui::TextEdit::singleline(&mut self.db_config.password).password(true));

                ui.label("Database");
                ui.text_edit_singleline(&mut self.db_config.database);

                ui.separator();

                if ui.button("Connect").clicked() {
                    // TODO: trigger db connection here
                    self.window = Window::Main;
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(
                        1024.0, 768.0,
                    )));
                }
            });
        });
    }

    fn main_view(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.label("Connected");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Platypus");
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.window {
            Window::Welcome => self.welcome_view(ctx),
            Window::Main => self.main_view(ctx),
        }
    }
}
