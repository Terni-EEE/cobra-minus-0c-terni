use std::path::PathBuf;

use eframe::egui::{Button, Context, Image, Pos2, Rect, Vec2, Window, include_image};

use egui_file_dialog::FileDialog;

#[derive(Default)]
pub struct MainMenu {
    new_project_dialog: FileDialog,
    new_project_path: Option<PathBuf>,
}

impl eframe::App for MainMenu {
    fn logic(&mut self, context: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(context);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let new_project_button_rectangle =
            Rect::from_min_size(Pos2::new(70.0, 250.0), Vec2::new(125.0, 50.0));

        let new_project_button = ui.put(
            new_project_button_rectangle, Button::new("New Project")
        );

        if new_project_button.clicked() {
            self.new_project_dialog.pick_directory();
        }

        if let Some(new_project_path) = self.new_project_dialog.take_picked() {
            self.new_project_path = Some(new_project_path);

            println!("NEW PROJECT PATH: {:?}", self.new_project_path);

            create_new_project(self.new_project_path.as_mut().unwrap(), ui);
        }

        let open_project_button_rectangle =
            Rect::from_min_size(Pos2::new(250.0, 250.0), Vec2::new(125.0, 50.0));

        let open_project_button =
            ui.put(open_project_button_rectangle, Button::new("Open Project"));

        if open_project_button.clicked() {
            println!("OPEN A PROJECT.");
        }

        let cobra_logo_rectangle =
            Rect::from_min_size(Pos2::new(70.0, 60.0), Vec2::new(320.0, 111.0));

        ui.put(
            cobra_logo_rectangle,
            Image::new(include_image!("../assets/images/cobrafull.png")),
        );

        // Add new dialogs below here, always at end.
        self.new_project_dialog.update(ui);
    }
}

fn create_new_project(_new_project_path: &mut PathBuf, context: &Context) {
    Window::new("Dialog")
        .collapsible(false)
        .resizable(false)
        .show(context, |ui| {
            ui.label("Hello!");
            if ui.button("OK").clicked() {
                // close dialog
            }
        });
}
