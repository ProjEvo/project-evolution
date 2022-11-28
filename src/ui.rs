use std::time::Duration;

use crate::{
    creature::{Creature, Position},
    simulator::{Simulation, MAX_WORLD_X, MAX_WORLD_Y},
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

fn transform_n_from_world_to_pos2(n: f32, available_size: Vec2) -> f32 {
    let x_factor = available_size.x / MAX_WORLD_X;

    n * x_factor
}

fn paint_simulation(simulation: &Simulation, painter: &Painter, available_size: Vec2) {
    let rigid_body_set = simulation.rigid_body_set();
    let impulse_joint_set = simulation.impulse_joint_set();

    for (_, joint) in impulse_joint_set.iter() {
        let body1_position = rigid_body_set.get(joint.body1).unwrap().translation();
        let body2_position = rigid_body_set.get(joint.body2).unwrap().translation();
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

    for (_, body) in rigid_body_set.iter() {
        for collider_handle in body.colliders() {
            let collider = simulation
                .collider_set()
                .get(collider_handle.clone())
                .unwrap();

            let as_ball = collider.shape().as_ball();

            if let None = as_ball {
                continue;
            }

            let as_ball = as_ball.unwrap();

            let circle = CircleShape {
                center: transform_position_from_world_to_pos2(body.translation(), available_size),
                radius: transform_n_from_world_to_pos2(as_ball.radius, available_size),
                fill: Color32::BLUE,
                stroke: Stroke::default(),
            };

            painter.add(circle);
        }
    }
}

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
}

/// Initializes the new interface that will create the objects on the screen
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        App {
            paused: true,
            ..Default::default()
        }
    }

    /// Renders the scene
    fn render(&self, painter: &Painter, available_size: Vec2) {
        paint_simulations(&self.simulations, painter, available_size);
    }

    // Generic test function to step all simulations
    fn step_all(&mut self) {
        for simulation in self.simulations.iter_mut() {
            simulation.step();
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

                    let mut simulation = Simulation::new();

                    simulation.add_creature(&creature);

                    self.simulations.push(simulation);
                }

                let play_button_text = match self.paused {
                    true => "Play",
                    false => "Pause",
                };

                if ui.button(play_button_text).clicked() {
                    self.paused = !self.paused
                }

                if !self.paused {
                    self.step_all();
                }

                self.render(ui.painter(), total_size);
            });

        ctx.request_repaint_after(Duration::from_millis(16))
    }
}
