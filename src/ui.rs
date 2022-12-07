//! Manages the UI

use std::time::{Duration, Instant};

use crate::{
    creature::{CreatureBuilder, Position},
    simulation::{Simulation, MAX_WORLD_X, MAX_WORLD_Y, STEPS_PER_SECOND},
};
use eframe::{egui, epaint::CircleShape, Theme};
use egui::{Color32, Painter, Pos2, Stroke, Vec2};

use crate::res;

/// Initializes the UI
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

/// Utility method to convert physics coordinates to where they should be on the screen
fn transform_position_from_world_to_pos2(
    position: &rapier::prelude::Vector<f32>,
    available_size: Vec2,
) -> Pos2 {
    let x_factor = available_size.x / MAX_WORLD_X;
    let y_factor = available_size.y / MAX_WORLD_Y;

    Pos2 {
        x: position.x * x_factor,
        y: position.y * y_factor,
    }
}

/// Utility method to convert physics scalars to where they should be on the screen
fn transform_n_from_world_to_pos2(n: f32, available_size: Vec2) -> f32 {
    let x_factor = available_size.x / MAX_WORLD_X;

    n * x_factor
}

/// Paints a [Simulation] using the provided [Painter]
fn paint_simulation(simulation: &Simulation, painter: &Painter, available_size: Vec2) {
    let creature = simulation.creature();

    for muscle in creature.muscles().values() {
        let body1_position = &simulation.get_position_of_node(muscle.from_id);
        let body2_position = &simulation.get_position_of_node(muscle.to_id);
        let point1 = transform_position_from_world_to_pos2(body1_position, available_size);
        let point2 = transform_position_from_world_to_pos2(body2_position, available_size);

        let line = egui::Shape::line(
            vec![point1, point2],
            Stroke::from((
                transform_n_from_world_to_pos2(0.5, available_size),
                Color32::RED,
            )),
        );

        painter.add(line);
    }

    for (id, node) in creature.nodes() {
        let position = simulation.get_position_of_node(*id);

        let circle = CircleShape {
            center: transform_position_from_world_to_pos2(&position, available_size),
            radius: transform_n_from_world_to_pos2(node.size / 2.0, available_size),
            fill: Color32::BLUE,
            stroke: Stroke::default(),
        };

        painter.add(circle);
    }
}

/// Paints a vec of [Simulation]s using the provided [Painter]
fn paint_simulations(simulations: &Vec<Simulation>, painter: &Painter, available_size: Vec2) {
    for simulation in simulations {
        paint_simulation(simulation, painter, available_size)
    }
}

#[derive(Default)]

/// Creates new egui ui struct used to populate objects into new Window
struct App {
    simulations: Vec<Simulation>,
    paused: bool,
    last_frame: Option<Instant>,
}

impl App {
    /// Initializes the egui app
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        App {
            paused: true,
            last_frame: Some(Instant::now()),
            ..Default::default()
        }
    }

    /// Renders the scene
    fn render(&self, painter: &Painter, available_size: Vec2) {
        paint_simulations(&self.simulations, painter, available_size);
    }

    // Steps all simulations
    fn step_all(&mut self) {
        for simulation in self.simulations.iter_mut() {
            simulation.step();
        }
    }
}

impl eframe::App for App {
    /// Called every frame to update the screen
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

                if ui.button("Add Simulation").clicked() {
                    let creature = CreatureBuilder::random()
                        .translate_center_to(Position::new(MAX_WORLD_X / 2.0, MAX_WORLD_Y / 2.0))
                        .build();

                    self.simulations.push(Simulation::new(creature));
                }

                if ui.button("Remove Simulations").clicked() {
                    self.simulations.clear();
                }

                let play_button_text = match self.paused {
                    true => "Play",
                    false => "Pause",
                };

                if ui.button(play_button_text).clicked() {
                    self.paused = !self.paused
                }

                let mut now = Instant::now();

                if !self.paused {
                    if let Some(last_frame) = self.last_frame {
                        let mut passed = now.duration_since(last_frame).as_secs_f32();
                        let delta = 1.0 / STEPS_PER_SECOND as f32;

                        while passed > delta {
                            passed -= delta;
                            self.step_all();
                        }

                        now = now
                            .checked_sub(Duration::from_secs_f32(passed))
                            .unwrap_or(now);
                    }
                }

                self.last_frame = Some(now);

                self.render(ui.painter(), total_size);
            });

        // Logic to continuously re-render the UI
        ctx.request_repaint();
    }
}
