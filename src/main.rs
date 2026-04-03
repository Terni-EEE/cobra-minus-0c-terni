use eframe::egui::{self, CentralPanel, Context, TopBottomPanel, Vec2, ViewportBuilder};

use egui_code_editor::{self, CodeEditor, ColorTheme, Syntax};

// enum IDETheme {
//     DARK, 
//     LIGHT,
// }

// impl default::Default for IDETheme {
//     fn default() -> Self {
//         IDETheme::DARK
//     }
// }

#[derive(Default)]
struct IDE {
    code: String,
    // theme: IDETheme
}

impl eframe::App for IDE {
    fn update(&mut self, context: &Context, _frame: &mut eframe::Frame) {
        let Self { code } = self;
    
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("NEW BUTTON CLICKED.");
                    }

                    if ui.button("Open").clicked() {
                        println!("OPEN BUTTON CLICKED.");
                    }

                    if ui.button("Save").clicked() {
                        println!("SAVE BUTTON CLICKED.");
                    }

                    if ui.button("Save As").clicked() {
                        println!("SAVE AS BUTTON CLICKED.");
                    }

                    if ui.button("Close").clicked() {
                        context.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        println!("CUT BUTTON CLICKED.");
                    }

                    if ui.button("Copy").clicked() {
                        println!("COPY BUTTON CLICKED.");
                    }

                    if ui.button("Paste").clicked() {
                        println!("PASTE BUTTON CLICKED.");
                    }

                    if ui.button("Select All").clicked() {
                        println!("SELECT ALL BUTTON CLICKED.");
                    }

                    if ui.button("Delete").clicked() {
                        println!("DELETE BUTTON CLICKED.");
                    }
                });
            })
        });

        CentralPanel::default().show(context, |ui: &mut egui::Ui| {
            let mut code_body = CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(Syntax::python())
                .with_numlines(true);

            code_body.show(ui, code);
        });
    }
    
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        println!("This is unneccesary.");
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
