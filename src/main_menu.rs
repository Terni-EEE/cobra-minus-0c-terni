use eframe::egui::{Button, Image, Pos2, Rect, Vec2, include_image};

#[derive(Default)]
pub struct MainMenu {}

impl eframe::App for MainMenu {
    fn logic(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(context);
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let new_project_button_rectangle =
            Rect::from_min_size(Pos2::new(70.0, 250.0), Vec2::new(125.0, 50.0));

        let new_project_button = ui.put(new_project_button_rectangle, Button::new("New Project"));

        if new_project_button.clicked() {
            println!("CREATE NEW PROJECT.");
        }

        let open_project_button_rectangle =
            Rect::from_min_size(Pos2::new(250.0, 250.0), Vec2::new(125.0, 50.0));

        let open_project_button =
            ui.put(open_project_button_rectangle, Button::new("Open Project"));

        if open_project_button.clicked() {
            println!("OPEN A PROJECT.");
        }

        let cobra_logo_rectangle = Rect::from_min_size(Pos2::new(70.0, 60.0), Vec2::new(320.0, 111.0));

        ui.put(cobra_logo_rectangle, Image::new(include_image!("../assets/images/cobrafull.png")));
    }
}
