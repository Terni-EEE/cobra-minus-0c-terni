use eframe::egui::{self, CentralPanel, Context, FontFamily, FontId, TextEdit, TextStyle, TopBottomPanel, Vec2, ViewportBuilder};

#[derive(Default)]
struct IDE {
    code: String,
}

impl eframe::App for IDE {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("NEW BUTTON CLICKED.");
                    }
                })
            })
        });

        CentralPanel::default().show(ctx, |ui| {
            let code_body = TextEdit::multiline(&mut self.code)
                .font(TextStyle::Monospace)
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY);

            let code_body_response = ui.add_sized(ui.available_size(), code_body);

            if code_body_response.changed() {
                println!("NEW TEXT ADDED.");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let window_resolution = Vec2::new(1080.0, 720.0);

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(window_resolution),
        ..Default::default()
    };

    eframe::run_native(
        "Cobra Minus 0C.5",
        options,
        Box::new(|_| Ok(Box::<IDE>::default())),
    )
}
