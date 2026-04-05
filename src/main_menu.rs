use std::path::PathBuf;

use eframe::egui::{Button, Context, Image, Panel, Pos2, Rect, TextEdit, Vec2, Window, include_image};

use egui_file_dialog::FileDialog;

#[derive(Default)]
/// The main menu of the application.
///
/// When creating a new project, the user is prompted with a file dialog, then, a check is ran
/// to see if the output isn't `None` to proceed with the input dialog to name the project.
pub struct MainMenu {
    new_project_dialog: FileDialog,
    new_project_path: Option<PathBuf>,
    new_project_name: String, 
}

fn get_name_of_new_project(context: &Context, new_project_name: &mut String) {
    let new_project_name_dialog = Window::new("Project Name")
        .collapsible(false)
        .resizable(false)
        .max_width(250.0)
        .max_height(230.0);

    new_project_name_dialog.show(context, |ui| {
        let new_project_name_input = TextEdit::singleline(new_project_name);
        new_project_name_input.show(ui);

        let _ok_button = ui.button("Ok");
        let _close_button = ui.button("Close");

        // Panel::bottom("ok_close_options_new_project_name_input").show_inside(ui, |ui| {
        //     let _ok_button = ui.button("Ok");
        //     let _close_button = ui.button("Close");
        // });
    });
}

impl eframe::App for MainMenu {
    fn logic(&mut self, context: &Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(context);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let new_project_button_rectangle =
            Rect::from_min_size(Pos2::new(70.0, 250.0), Vec2::new(125.0, 50.0));

        let new_project_button = ui.put(new_project_button_rectangle, Button::new("New Project"));

        if new_project_button.clicked() {
            self.new_project_dialog.pick_directory();
        }

        if let Some(new_project_path) = self.new_project_dialog.take_picked() {
            self.new_project_path = Some(new_project_path);

            println!("NEW PROJECT PATH: {:?}", self.new_project_path);
        }

        if self.new_project_path != None {
            get_name_of_new_project(ui, &mut self.new_project_name);
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
