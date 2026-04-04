use eframe::egui;
use egui_dialogs::Dialog;

pub struct NewProjectNameInput {
    name: String,
}

impl Dialog<String> for NewProjectNameInput {
    fn show(&mut self, ctx: &egui::context::Context, dctx: &egui_dialogs::DialogContext) -> Option<String> {
        todo!();
    }
}
