use std::collections::BTreeMap;

use eframe::egui::{self, CentralPanel, FontData, FontDefinitions, FontFamily, MenuBar, Panel, Style, Vec2, ViewportBuilder, Visuals};

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
    fn logic(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        add_anonymous_pro_font(context);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let Self { code } = self;

        ui.set_theme(egui::Theme::Dark);

        Panel::top("menu_bar").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        println!("NEW BUTTON CLICKED.");
                    }

                    if ui.button("Open").clicked() {
                        println!("OPEN BUTTON CLICKED.");
                    }

                    ui.separator();

                    if ui.button("Save").clicked() {
                        println!("SAVE BUTTON CLICKED.");
                    }

                    if ui.button("Save As").clicked() {
                        println!("SAVE AS BUTTON CLICKED.");
                    }

                    ui.separator();

                    if ui.button("Close").clicked() {
                        todo!();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        println!("CUT BUTTON CLICKED.");
                    }

                    ui.separator();

                    if ui.button("Copy").clicked() {
                        println!("COPY BUTTON CLICKED.");
                    }

                    if ui.button("Paste").clicked() {
                        println!("PASTE BUTTON CLICKED.");
                    }

                    ui.separator();

                    if ui.button("Select All").clicked() {
                        println!("SELECT ALL BUTTON CLICKED.");
                    }

                    if ui.button("Delete").clicked() {
                        println!("DELETE BUTTON CLICKED.");
                    }
                });
            })
        });

        CentralPanel::default().show_inside(ui, |ui: &mut egui::Ui| {
            let mut code_body = CodeEditor::default()
                .id_source("code editor")
                .with_rows(1)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GITHUB_DARK)
                .with_syntax(Syntax::python())
                .with_numlines(true);

            code_body.show(ui, code);
        });
    }
}

fn add_anonymous_pro_font(context: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "AnonymousPro".to_owned(), 
        std::sync::Arc::new(FontData::from_static(include_bytes!("../assets/fonts/Anonymous_Pro/AnonymousPro-Regular.ttf"))),
    );

    let mut anonymous_pro_font_family = BTreeMap::new();
    anonymous_pro_font_family.insert(FontFamily::Name("AnonymousPro".into()), vec!["AnonymousPro".to_owned()]);

    fonts.families.append(&mut anonymous_pro_font_family);

    context.set_fonts(fonts);
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
        Box::new(|creation_context| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };

            let theme = egui::Theme::Dark;

            creation_context.egui_ctx.set_style_of(theme, style);

            Ok(Box::<IDE>::default())
        }),
    )
}
