use eframe::egui;

struct Platypus {
    msg: String,
}

impl Default for Platypus {
    fn default() -> Self {
        Self {
            msg: String::from("Hello from Platypus"),
        }
    }
}

impl eframe::App for Platypus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(&self.msg);
            })
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Platypus",
        options,
        Box::new(|_| Ok(Box::<Platypus>::default())),
    )
}
