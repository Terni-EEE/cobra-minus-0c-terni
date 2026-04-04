mod ide;
mod main_menu;

use eframe::egui::{self, Style, Vec2, ViewportBuilder, Visuals};

use main_menu::MainMenu;

/// Launches the main menu with `window_resolution`, `options` and `style`.
/// The `style` has the dark theme.
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

            Ok(Box::<MainMenu>::default())
        }),
    )
}
