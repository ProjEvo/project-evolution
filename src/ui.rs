// File creates User Interface (Window, Button, [Creature] Creation, and [Creature] Evolution)
use eframe::egui;

use crate::res;

pub fn init() {
    let native_options = eframe::NativeOptions {
        icon_data: Some(res::load_icon_data()),
        ..Default::default()
    };

    eframe::run_native(
        "Project Evolution",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

#[derive(Default)]

//Creates New EGUI User Interface Struct used to populate objects into new Window
struct App {}

//Initializes the New Interface that will create the objects on the screen
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    //Function Updates the screen that is to be blitted (currently very underdeveloped, needs to be fully realized soon)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // initializes a central panel of the UI with contents to be added

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is should be a blank UI with a couple of buttons");
            if ui.add(egui::Button::new("Click")).clicked() {}
        });
    }
}
