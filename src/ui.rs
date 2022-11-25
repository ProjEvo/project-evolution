use std::time::Duration;

use crate::{
    creature::{Creature, Position},
    simulator::{MAX_WORLD_X, MAX_WORLD_Y},
};
/// Manages User Interface (UI)
use eframe::{egui, epaint::CircleShape, Theme};
use egui::{Color32, Painter, Pos2, Stroke, Vec2};

use crate::res;

pub fn init() {
    let native_options = eframe::NativeOptions {
        icon_data: Some(res::load_icon_data()),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        maximized: true,
        initial_window_size: Some(Vec2::new(500.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Project Evolution",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

fn transform_position_from_world_to_pos2(position: &Position, available_size: Vec2) -> Pos2 {
    let x_factor = available_size.x / MAX_WORLD_X;
    let y_factor = available_size.y / MAX_WORLD_Y;

    Pos2 {
        x: position.x * x_factor,
        y: position.y * y_factor,
    }
}

fn transform_n_from_world_to_pos2(n: f32, available_size: Vec2) -> f32 {
    let x_factor = available_size.x / MAX_WORLD_X;

    n * x_factor
}

fn paint_creature_muscles(creature: &Creature, painter: &Painter, available_size: Vec2) {
    for muscle in creature.muscles().values() {
        let from_node = creature.nodes().get(&muscle.from_id).unwrap();
        let to_node = creature.nodes().get(&muscle.to_id).unwrap();

        let from_point = transform_position_from_world_to_pos2(&from_node.position, available_size);
        let to_point = transform_position_from_world_to_pos2(&to_node.position, available_size);
        let points = vec![from_point, to_point];

        let line = egui::Shape::line(
            points,
            Stroke::from((
                transform_n_from_world_to_pos2(0.5, available_size),
                Color32::RED,
            )),
        );

        painter.add(line);
    }
}

fn paint_creature_nodes(creature: &Creature, painter: &Painter, available_size: Vec2) {
    for node in creature.nodes().values() {
        let circle = CircleShape {
            center: transform_position_from_world_to_pos2(&node.position, available_size),
            radius: transform_n_from_world_to_pos2(node.size / 2.0, available_size),
            fill: Color32::BLUE,
            stroke: Stroke::default(),
        };

        painter.add(circle);
    }
}

fn paint_creature(creature: &Creature, painter: &Painter, available_size: Vec2) {
    paint_creature_muscles(creature, painter, available_size);
    paint_creature_nodes(creature, painter, available_size);
}

fn paint_creatures(creatures: &Vec<Creature>, painter: &Painter, available_size: Vec2) {
    for creature in creatures {
        paint_creature(creature, painter, available_size)
    }
}

#[derive(Default)]

/// Creates new egui ui struct used to populate objects into new Window
struct App {
    creatures: Vec<Creature>,
}

/// Initializes the new interface that will create the objects on the screen
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    /// Renders the scene
    fn render(&self, painter: &Painter, available_size: Vec2) {
        paint_creatures(&self.creatures, painter, available_size)
    }

    // Generic test function to translate all the creatures to the right
    fn translate_all(&mut self) {
        for creature in self.creatures.iter_mut() {
            creature.translate(5.0, 0.0)
        }
    }
}

impl eframe::App for App {
    /// Updates the screen that is to be blitted (currently very underdeveloped, needs to be fully realized soon)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let custom_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            ..Default::default()
        };

        // initializes a central panel of the UI with contents to be added
        egui::CentralPanel::default()
            .frame(custom_frame)
            .show(ctx, |ui| {
                let total_size = ui.available_size();

                ui.heading("Hello World!");
                ui.label("This is should be a blank UI with a couple of buttons");

                if ui.button("Add Creature").clicked() {
                    let mut creature = Creature::random();

                    creature
                        .translate_center_to(Position::new(MAX_WORLD_X / 2.0, MAX_WORLD_Y / 2.0));

                    self.creatures.push(creature);
                }

                if ui.button("Translate All").clicked() {
                    self.translate_all();
                }

                self.render(ui.painter(), total_size);
            });

        ctx.request_repaint_after(Duration::from_millis(16))
    }
}
